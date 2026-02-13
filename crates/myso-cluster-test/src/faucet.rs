// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::cluster::{Cluster, new_wallet_context_from_cluster};
use async_trait::async_trait;
use fastcrypto::encoding::{Encoding, Hex};
use myso_faucet::{FaucetConfig, FaucetResponse, SimpleFaucet};
use myso_types::base_types::MySoAddress;
use myso_types::crypto::KeypairTraits;
use prometheus::Registry;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tracing::{Instrument, debug, info, info_span};

pub struct FaucetClientFactory;

impl FaucetClientFactory {
    pub async fn new_from_cluster(
        cluster: &(dyn Cluster + Sync + Send),
    ) -> Arc<dyn FaucetClient + Sync + Send> {
        match cluster.remote_faucet_url() {
            Some(url) => Arc::new(RemoteFaucetClient::new(url.into())),
            // If faucet_url is none, it's a local cluster
            None => {
                let key = cluster
                    .local_faucet_key()
                    .expect("Expect local faucet key for local cluster")
                    .copy();
                let wallet_context = new_wallet_context_from_cluster(cluster, key)
                    .await
                    .instrument(info_span!("init_wallet_context_for_faucet"));

                let tmp = tempfile::tempdir().unwrap();
                let wal_path = tmp.path().join("faucet.wal");
                let config = FaucetConfig::default();
                let prom_registry = Registry::new();
                let simple_faucet = SimpleFaucet::new(
                    wallet_context.into_inner(),
                    &prom_registry,
                    &wal_path,
                    config,
                )
                .await
                .unwrap();

                Arc::new(LocalFaucetClient::new(simple_faucet, tmp))
            }
        }
    }
}

/// Faucet Client abstraction
#[async_trait]
pub trait FaucetClient {
    async fn request_myso_coins(&self, request_address: MySoAddress) -> FaucetResponse;
}

/// Client for a remote faucet that is accessible by POST requests
pub struct RemoteFaucetClient {
    remote_url: String,
}

impl RemoteFaucetClient {
    fn new(url: String) -> Self {
        info!("Use remote faucet: {}", url);
        Self { remote_url: url }
    }
}

#[async_trait]
impl FaucetClient for RemoteFaucetClient {
    /// Request test MYSO coins from faucet.
    /// It also verifies the effects are observed by fullnode.
    async fn request_myso_coins(&self, request_address: MySoAddress) -> FaucetResponse {
        let gas_url = format!("{}/v2/gas", self.remote_url);
        debug!("Getting coin from remote faucet {}", gas_url);
        let data = HashMap::from([("recipient", Hex::encode(request_address))]);
        let map = HashMap::from([("FixedAmountRequest", data)]);

        let auth_header = match env::var("FAUCET_AUTH_HEADER") {
            Ok(val) => val,
            _ => "".to_string(),
        };

        let response = reqwest::Client::new()
            .post(&gas_url)
            .header("Authorization", auth_header)
            .json(&map)
            .send()
            .await
            .unwrap_or_else(|e| panic!("Failed to talk to remote faucet {:?}: {:?}", gas_url, e));
        let full_bytes = response.bytes().await.unwrap();
        let faucet_response: FaucetResponse = serde_json::from_slice(&full_bytes)
            .map_err(|e| anyhow::anyhow!("json deser failed with bytes {:?}: {e}", full_bytes))
            .unwrap();

        if let Some(ref error) = faucet_response.error {
            panic!("Failed to get gas tokens with error: {}", error)
        };

        faucet_response
    }
}

/// A local faucet that holds some coins since genesis
pub struct LocalFaucetClient {
    simple_faucet: Arc<SimpleFaucet>,
    _tempdir: tempfile::TempDir,
}

impl LocalFaucetClient {
    fn new(simple_faucet: Arc<SimpleFaucet>, tempdir: tempfile::TempDir) -> Self {
        info!("Use local faucet");
        Self {
            simple_faucet,
            _tempdir: tempdir,
        }
    }
}
#[async_trait]
impl FaucetClient for LocalFaucetClient {
    async fn request_myso_coins(&self, request_address: MySoAddress) -> FaucetResponse {
        use myso_faucet::{Faucet, FaucetConfig};
        use uuid::Uuid;

        let config = FaucetConfig::default();
        let amounts = vec![config.amount; config.num_coins];
        match self
            .simple_faucet
            .send(Uuid::new_v4(), request_address, &amounts)
            .await
        {
            Ok(receipt) => FaucetResponse {
                transferred_gas_objects: receipt.sent,
                error: None,
            },
            Err(e) => FaucetResponse {
                transferred_gas_objects: vec![],
                error: Some(e.to_string()),
            },
        }
    }
}
