// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Analytics indexer builder.

use std::sync::Arc;
use std::time::Duration;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use object_store::ClientOptions;
use object_store::aws::AmazonS3Builder;
use object_store::azure::MicrosoftAzureBuilder;
use object_store::gcp::GoogleCloudStorageBuilder;
use object_store::local::LocalFileSystem;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;
use tokio_util::sync::CancellationToken;
use tracing::info;

use myso_indexer_alt_framework::Indexer;
use myso_indexer_alt_framework::service::Service;

use crate::config::IndexerConfig;
use crate::config::OutputStoreConfig;
use crate::metrics::Metrics;
use crate::package_store::PackageCache;
use crate::progress_monitoring::spawn_snowflake_monitors;
use crate::store::AnalyticsStore;

/// Build and run an analytics indexer, returning a Service handle.
pub async fn build_analytics_indexer(
    config: IndexerConfig,
    metrics: Metrics,
    registry: prometheus::Registry,
) -> Result<Service> {
    config.validate()?;

    let store = match &config.output_store {
        crate::config::OutputStoreConfig::ClickHouse {
            host,
            port,
            user,
            password,
        } => {
            let ch_store =
                crate::store::ClickHouseStore::new(host, *port, user, password.as_deref())?;
            ch_store
                .bridge
                .execute(
                    "CREATE TABLE IF NOT EXISTS transactions (\
                    checkpoint_sequence_number UInt64, transaction_digest String, sender String, \
                    timestamp_ms Int64, tx_kind LowCardinality(String), gas_computation_cost UInt64, \
                    gas_storage_cost UInt64, gas_storage_rebate UInt64, status UInt8, epoch UInt64, \
                    gas_price UInt64, gas_budget UInt64, gas_owner String, is_sponsored UInt8, \
                    created_objects UInt32, mutated_objects UInt32, execution_error Nullable(String), \
                    indexed_at DateTime64(3, 'UTC') DEFAULT now()) \
                    ENGINE = MergeTree() ORDER BY (checkpoint_sequence_number, transaction_digest)",
                )
                .await
                .map_err(|e| anyhow::anyhow!("Create ClickHouse table: {}", e))?;
            ch_store
                .bridge
                .execute(
                    "CREATE TABLE IF NOT EXISTS events (\
                    checkpoint_sequence_number UInt64, transaction_digest String, event_index UInt64, \
                    epoch UInt64, timestamp_ms Int64, sender String, package String, module String, \
                    event_type LowCardinality(String), event_json String, bcs_length UInt64, \
                    indexed_at DateTime64(3, 'UTC') DEFAULT now()) \
                    ENGINE = MergeTree() ORDER BY (checkpoint_sequence_number, transaction_digest, event_index)",
                )
                .await
                .map_err(|e| anyhow::anyhow!("Create ClickHouse events table: {}", e))?;
            ch_store
                .bridge
                .execute(
                    "CREATE TABLE IF NOT EXISTS move_calls (\
                    checkpoint_sequence_number UInt64, transaction_digest String, cmd_idx UInt64, \
                    epoch UInt64, timestamp_ms Int64, package String, module LowCardinality(String), \
                    function LowCardinality(String), indexed_at DateTime64(3, 'UTC') DEFAULT now()) \
                    ENGINE = MergeTree() ORDER BY (checkpoint_sequence_number, transaction_digest, cmd_idx)",
                )
                .await
                .map_err(|e| anyhow::anyhow!("Create ClickHouse move_calls table: {}", e))?;
            ch_store
                .bridge
                .execute(
                    "CREATE TABLE IF NOT EXISTS objects (\
                    object_id String, version UInt64, digest String, type_ Nullable(String), \
                    checkpoint_sequence_number UInt64, epoch UInt64, timestamp_ms Int64, \
                    owner_type Nullable(String), owner_address Nullable(String), object_status String, \
                    initial_shared_version Nullable(UInt64), previous_transaction String, \
                    has_public_transfer UInt8, is_consensus UInt8, storage_rebate Nullable(UInt64), \
                    bcs String, coin_type Nullable(String), coin_balance Nullable(UInt64), \
                    struct_tag Nullable(String), object_json Nullable(String), bcs_length UInt64, \
                    indexed_at DateTime64(3, 'UTC') DEFAULT now()) \
                    ENGINE = MergeTree() ORDER BY (checkpoint_sequence_number, object_id, version)",
                )
                .await
                .map_err(|e| anyhow::anyhow!("Create ClickHouse objects table: {}", e))?;
            ch_store
                .bridge
                .execute(
                    "CREATE TABLE IF NOT EXISTS balance_changes (\
                    checkpoint_sequence_number UInt64, transaction_digest String, epoch UInt64, \
                    timestamp_ms Int64, owner String, coin_type String, amount Int64, \
                    indexed_at DateTime64(3, 'UTC') DEFAULT now()) \
                    ENGINE = MergeTree() ORDER BY (checkpoint_sequence_number, transaction_digest, owner, coin_type)",
                )
                .await
                .map_err(|e| anyhow::anyhow!("Create ClickHouse balance_changes table: {}", e))?;
            AnalyticsStore::new_clickhouse(ch_store, config.clone(), metrics.clone())
        }
        _ => {
            let object_store = create_object_store(&config.output_store)?;
            AnalyticsStore::new(object_store.clone(), config.clone(), metrics.clone())
        }
    };

    // Find checkpoint range (snaps to file boundaries in migration mode)
    let (adjusted_first_checkpoint, adjusted_last_checkpoint) = store
        .find_checkpoint_range(config.first_checkpoint, config.last_checkpoint)
        .await?;

    let work_dir = if let Some(ref work_dir) = config.work_dir {
        tempfile::Builder::new()
            .prefix("myso-analytics-indexer-")
            .tempdir_in(work_dir)?
            .keep()
    } else {
        tempfile::Builder::new()
            .prefix("myso-analytics-indexer-")
            .tempdir()?
            .keep()
    };

    let package_cache_path = work_dir.join("package_cache");
    let package_cache = Arc::new(PackageCache::new(&package_cache_path, &config.rpc_api_url));

    let indexer_args = myso_indexer_alt_framework::IndexerArgs {
        first_checkpoint: adjusted_first_checkpoint,
        last_checkpoint: adjusted_last_checkpoint,
        pipeline: vec![],
        task: Default::default(),
    };

    // When streaming is configured, use gRPC GetCheckpoint for ingestion fallback (when outside
    // buffer). This avoids depending on checkpoint-blob-indexer/remote store.
    let ingestion_args = if config.streaming_url.is_some() {
        myso_indexer_alt_framework::ingestion::ingestion_client::IngestionClientArgs {
            rpc_api_url: Some(config.rpc_api_url.parse()?),
            rpc_username: config.rpc_username.clone(),
            rpc_password: config.rpc_password.clone(),
            ..Default::default()
        }
    } else {
        myso_indexer_alt_framework::ingestion::ingestion_client::IngestionClientArgs {
            remote_store_url: config
                .remote_store_url
                .as_ref()
                .map(|u| url::Url::parse(u))
                .transpose()?,
            local_ingestion_path: config.local_ingestion_path.clone(),
            rpc_api_url: Some(config.rpc_api_url.parse()?),
            rpc_username: config.rpc_username.clone(),
            rpc_password: config.rpc_password.clone(),
            ..Default::default()
        }
    };

    let client_args = myso_indexer_alt_framework::ingestion::ClientArgs {
        ingestion: ingestion_args,
        streaming: myso_indexer_alt_framework::ingestion::streaming_client::StreamingClientArgs {
            streaming_url: config
                .streaming_url
                .as_ref()
                .map(|url| url.parse())
                .transpose()?,
        },
    };

    let ingestion_config = config.ingestion.clone();

    let mut indexer = Indexer::new(
        store.clone(),
        indexer_args,
        client_args,
        ingestion_config,
        None,
        &registry,
    )
    .await?;

    for pipeline_config in config.pipeline_configs() {
        info!("Registering pipeline: {}", pipeline_config.pipeline);
        pipeline_config
            .pipeline
            .register(
                &mut indexer,
                pipeline_config,
                package_cache.clone(),
                metrics.clone(),
                config.sequential.clone(),
            )
            .await?;
    }

    // Spawn Snowflake monitors (if configured)
    let cancel = CancellationToken::new();
    let sf_handles = spawn_snowflake_monitors(&config, metrics, cancel.clone())?;

    // Run the indexer and register shutdown signals
    let service = indexer.run().await?;
    Ok(service
        .with_shutdown_signal(async move {
            store.shutdown().await;
        })
        .with_shutdown_signal(async move {
            cancel.cancel();
            for handle in sf_handles {
                let _ = handle.await;
            }
        }))
}

fn create_object_store(config: &OutputStoreConfig) -> Result<Arc<dyn object_store::ObjectStore>> {
    match config {
        OutputStoreConfig::ClickHouse { .. } => {
            bail!("Use build_analytics_indexer_clickhouse for ClickHouse output")
        }
        OutputStoreConfig::Gcs {
            bucket,
            service_account_path,
            custom_headers,
            request_timeout_secs,
        } => {
            let mut client_options =
                ClientOptions::default().with_timeout(Duration::from_secs(*request_timeout_secs));

            // Apply custom headers (e.g., for requester-pays buckets)
            if let Some(headers_map) = custom_headers {
                let mut headers = HeaderMap::new();
                for (key, value) in headers_map {
                    headers.insert(
                        HeaderName::try_from(key.as_str())?,
                        HeaderValue::from_str(value)?,
                    );
                }
                client_options = client_options.with_default_headers(headers);
            }

            GoogleCloudStorageBuilder::new()
                .with_client_options(client_options)
                .with_bucket_name(bucket)
                .with_service_account_path(service_account_path.to_string_lossy())
                .build()
                .map(|s| Arc::new(s) as Arc<dyn object_store::ObjectStore>)
                .context("Failed to create GCS store")
        }
        OutputStoreConfig::S3 {
            bucket,
            region,
            access_key_id,
            secret_access_key,
            endpoint,
            request_timeout_secs,
        } => {
            let client_options =
                ClientOptions::default().with_timeout(Duration::from_secs(*request_timeout_secs));
            let mut builder = AmazonS3Builder::new()
                .with_client_options(client_options)
                .with_bucket_name(bucket)
                .with_region(region);
            if let Some(key) = access_key_id {
                builder = builder.with_access_key_id(key);
            }
            if let Some(secret) = secret_access_key {
                builder = builder.with_secret_access_key(secret);
            }
            if let Some(ep) = endpoint {
                builder = builder.with_endpoint(ep);
            }
            builder
                .build()
                .map(|s| Arc::new(s) as Arc<dyn object_store::ObjectStore>)
                .context("Failed to create S3 store")
        }
        OutputStoreConfig::Azure {
            container,
            account,
            access_key,
            request_timeout_secs,
        } => {
            let client_options =
                ClientOptions::default().with_timeout(Duration::from_secs(*request_timeout_secs));
            MicrosoftAzureBuilder::new()
                .with_client_options(client_options)
                .with_container_name(container)
                .with_account(account)
                .with_access_key(access_key)
                .build()
                .map(|s| Arc::new(s) as Arc<dyn object_store::ObjectStore>)
                .context("Failed to create Azure store")
        }
        OutputStoreConfig::File { path } => LocalFileSystem::new_with_prefix(path)
            .map(|s| Arc::new(s) as Arc<dyn object_store::ObjectStore>)
            .context("Failed to create file store"),
        OutputStoreConfig::Custom(store) => Ok(store.clone()),
    }
}
