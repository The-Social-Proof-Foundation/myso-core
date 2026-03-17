// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::Context;
use anyhow::bail;
use async_graphql::dataloader::DataLoader;
use myso_indexer_alt_metrics::db::DbConnectionStatsCollector;
use prometheus::Registry;
use url::Url;

use myso_pg_db as db;

use crate::metrics::DbReaderMetrics;
use crate::platform::PlatformRow;
use crate::post::PostRow;
use crate::profile::get_profile_by_address;
use crate::profile::get_profile_or_wallet_by_address;
use crate::profile::get_profiles;
use crate::social_graph::check_following;

pub use myso_indexer_alt_social_schema::models::Profile;

/// Reader for the social postgres database. Connects to the database populated by
/// myso-indexer-alt-social and provides query methods for profiles, posts, platforms,
/// and social graph data.
#[derive(Clone)]
pub struct SocialPgReader {
    db: Option<db::Db>,
    metrics: Arc<DbReaderMetrics>,
}

impl SocialPgReader {
    /// Create a new social database reader. If `database_url` is `None`, the reader
    /// will not accept any connection requests (they will all fail).
    ///
    /// `prefix` is used to prefix the metrics collected by this reader.
    pub async fn new(
        prefix: Option<&str>,
        database_url: Option<Url>,
        db_args: db::DbArgs,
        registry: &Registry,
    ) -> anyhow::Result<Self> {
        let db = if let Some(database_url) = database_url {
            let db = db::Db::for_read(database_url, db_args)
                .await
                .context("Failed to create social database for reading")?;

            registry
                .register(Box::new(DbConnectionStatsCollector::new(
                    prefix,
                    db.clone(),
                )))
                .context("Failed to register social database connection stats collector")?;

            Some(db)
        } else {
            None
        };

        let metrics = DbReaderMetrics::new(prefix, registry);

        Ok(Self { db, metrics })
    }

    /// Create a data loader backed by this reader.
    pub fn as_data_loader(&self) -> DataLoader<Self> {
        DataLoader::new(self.clone(), tokio::spawn)
    }

    /// Check if this reader has a database available.
    pub fn has_database(&self) -> bool {
        self.db.is_some()
    }

    /// Acquire a connection to the database.
    pub async fn connect(&self) -> anyhow::Result<db::Connection<'_>> {
        let Some(db) = &self.db else {
            bail!("No social database to connect to");
        };

        db.connect()
            .await
            .context("Failed to connect to social database")
    }

    /// Get a profile by owner address.
    pub async fn get_profile_by_address(&self, address: &str) -> anyhow::Result<Option<Profile>> {
        let mut conn = self.connect().await?;
        get_profile_by_address(&mut conn, address, &self.metrics).await
    }

    /// Get profile by address, or wallet-only data when no profile exists.
    pub async fn get_profile_or_wallet_by_address(
        &self,
        address: &str,
    ) -> anyhow::Result<crate::profile::ProfileByAddressResponse> {
        let mut conn = self.connect().await?;
        get_profile_or_wallet_by_address(&mut conn, address, &self.metrics).await
    }

    /// Get profiles with pagination.
    pub async fn get_profiles(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<Profile>> {
        let mut conn = self.connect().await?;
        get_profiles(&mut conn, limit, offset, &self.metrics).await
    }

    /// Get a post by ID.
    pub async fn get_post_by_id(&self, post_id: &str) -> anyhow::Result<Option<PostRow>> {
        let mut conn = self.connect().await?;
        crate::post::get_post_by_id(&mut conn, post_id, &self.metrics).await
    }

    /// List posts with optional filters.
    pub async fn list_posts(
        &self,
        owner: Option<&str>,
        post_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<PostRow>> {
        let mut conn = self.connect().await?;
        crate::post::list_posts(&mut conn, owner, post_type, limit, offset, &self.metrics).await
    }

    /// Get a platform by ID.
    pub async fn get_platform_by_id(
        &self,
        platform_id: &str,
    ) -> anyhow::Result<Option<PlatformRow>> {
        let mut conn = self.connect().await?;
        crate::platform::get_platform_by_id(&mut conn, platform_id, &self.metrics).await
    }

    /// List platforms with optional approved filter.
    pub async fn list_platforms(
        &self,
        approved_only: bool,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<Vec<PlatformRow>> {
        let mut conn = self.connect().await?;
        crate::platform::list_platforms(&mut conn, approved_only, limit, offset, &self.metrics)
            .await
    }

    /// Check if follower follows following.
    pub async fn check_following(
        &self,
        follower_address: &str,
        following_address: &str,
    ) -> anyhow::Result<bool> {
        let mut conn = self.connect().await?;
        check_following(
            &mut conn,
            follower_address,
            following_address,
            &self.metrics,
        )
        .await
    }
}
