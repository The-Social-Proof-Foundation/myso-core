// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use chrono::Utc;
use tracing::{debug, info};

use crate::api::AppState;
use crate::social_client::SocialClient;
use crate::store::OracleStore;

/// Polls social-server for pending SPoT posts and enqueues `ReviewPost` jobs.
pub struct PostPoller {
    store: Arc<OracleStore>,
    social: SocialClient,
}

impl PostPoller {
    pub fn new(store: Arc<OracleStore>, social: SocialClient) -> Self {
        Self { store, social }
    }

    pub async fn poll_once(&self) -> anyhow::Result<usize> {
        let cursor = crate::store::markets::max_ingested_created_at_ms(self.store.pool()).await?;
        let posts = self.social.list_pending_spot_posts(50, cursor).await?;
        if posts.is_empty() {
            return Ok(0);
        }
        let mut enqueued = 0usize;
        for post in posts {
            if crate::store::markets::market_exists(self.store.pool(), &post.post_id).await? {
                debug!(post_id = %post.post_id, "skip already-ingested post");
                continue;
            }
            let market_id = crate::store::markets::insert_market(
                self.store.pool(),
                &post.post_id,
                &post.owner,
                &post.content,
            )
            .await?;
            crate::store::jobs::enqueue_job(
                self.store.pool(),
                "ReviewPost",
                Some(market_id),
                None,
                post.created_at,
                Utc::now(),
                serde_json::json!({
                    "post_id": post.post_id,
                    "owner": post.owner,
                    "content": post.content,
                    "created_at_ms": post.created_at,
                    "post_type": post.post_type,
                }),
            )
            .await?;
            enqueued += 1;
        }
        if enqueued > 0 {
            info!(enqueued, "ingested pending SPoT posts");
        }
        Ok(enqueued)
    }
}

pub async fn run_post_poller(state: Arc<AppState>) -> anyhow::Result<usize> {
    let social = SocialClient::new(
        state.args.social_server_url.clone(),
        state.args.social_sync_secret.clone(),
    );
    let poller = PostPoller::new(state.store.clone(), social);
    poller.poll_once().await
}
