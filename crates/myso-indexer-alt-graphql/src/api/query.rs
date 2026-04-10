// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context as _;
use anyhow::anyhow;
use async_graphql::Context;
use async_graphql::Object;
use async_graphql::Result;
use async_graphql::connection::Connection;
use fastcrypto::encoding::Base58;
use fastcrypto::encoding::Encoding;
use futures::future::try_join_all;
use myso_indexer_alt_reader::fullnode_client::Error::GrpcExecutionError;
use myso_indexer_alt_reader::fullnode_client::FullnodeClient;
use myso_rpc::proto::myso::rpc::v2 as proto;

use crate::api::mutation::TransactionInputError;
use crate::api::scalars::base64::Base64;
use crate::api::scalars::digest::Digest;
use crate::api::scalars::id::Id;
use crate::api::scalars::json::Json;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::scalars::type_filter::TypeInput;
use crate::api::scalars::uint53::UInt53;
use crate::api::types::address;
use crate::api::types::address::Address;
use crate::api::types::address::AddressKey;
use crate::api::types::checkpoint::CCheckpoint;
use crate::api::types::checkpoint::Checkpoint;
use crate::api::types::checkpoint::filter::CheckpointFilter;
use crate::api::types::coin_metadata::CoinMetadata;
use crate::api::types::dynamic_field::DynamicField;
use crate::api::types::epoch::CEpoch;
use crate::api::types::epoch::Epoch;
use crate::api::types::event::CEvent;
use crate::api::types::event::Event;
use crate::api::types::event::filter::EventFilter;
use crate::api::types::governance::{
    AnonymousVotingTrend, Delegate, GovernanceEvent, GovernanceRegistry, NominatedDelegate,
    Proposal,
};
use crate::api::types::insurance::{InsurancePolicy, InsuranceVault};
use crate::api::types::move_object::MoveObject;
use crate::api::types::move_package;
use crate::api::types::move_package::MovePackage;
use crate::api::types::move_package::PackageCheckpointFilter;
use crate::api::types::move_package::PackageKey;
use crate::api::types::move_type;
use crate::api::types::move_type::MoveType;
use crate::api::types::mydata::{
    MyDataPurchase, MyDataQueryBroadPool, MyDataQueryClaim, MyDataQueryDistributionRound,
    MyDataQueryListingSubPool, MyDataQueryMerkleRoot, MyDataQuerySnapshotAnchor, MyDataQuerySubPool,
    MyDataRecord,
};
use crate::api::types::node::Node;
use crate::api::types::object;
use crate::api::types::object::Object;
use crate::api::types::object::ObjectKey;
use crate::api::types::object::VersionFilter;
use crate::api::types::object_filter::ObjectFilter;
use crate::api::types::object_filter::ObjectFilterValidator as OFValidator;
use crate::api::types::platform::{Platform, PlatformUserAccess};
use crate::api::types::post::{CommentSummary, Post, ReactionSummary, RepostSummary, TipSummary};
use crate::api::types::profile::Profile;
use crate::api::types::promotion::{Promotion, PromotionTimeSeries};
use crate::api::types::protocol_configs::ProtocolConfigs;
use crate::api::types::service_config::ServiceConfig;
use crate::api::types::simulation_result::SimulationResult;
use crate::api::types::social_config::{
    InsuranceConfig, MyDataConfig, PocConfig, PostConfig, SpotConfig, SptExchangeConfig,
};
use crate::api::types::spot::{
    SpotBet, SpotBetWithdrawal, SpotPayout, SpotRecord, SpotRefund, SpotResolution,
};
use crate::api::types::spt::{
    SptHolding, SptOrder, SptPool, SptPriceHistory, SptReservationHolding,
    SptReservationVolumeBucket, SptReservationVolumeInterval, SptSortBy,
};
use crate::api::types::transaction::CTransaction;
use crate::api::types::transaction::Transaction;
use crate::api::types::transaction::filter::TransactionFilter;
use crate::api::types::transaction::filter::TransactionFilterValidator as TFValidator;
use crate::api::types::transaction_effects::TransactionEffects;
use crate::api::types::vesting::{
    VestingLeaderboardEntry, VestingLeaderboardResponse, VestingWallet,
};
use crate::api::types::zklogin;
use crate::api::types::zklogin::ZkLoginIntentScope;
use crate::api::types::zklogin::ZkLoginVerifyResult;
use crate::error::RpcError;
use crate::error::bad_user_input;
use crate::error::upcast;
use crate::pagination::Page;
use crate::pagination::PaginationConfig;
use crate::scope::Scope;
use crate::task::chain_identifier::ChainIdentifier;

#[derive(Default)]
pub struct Query {
    /// Queries will use this scope if it is populated, instead of creating a fresh scope from
    /// information in the request-wide [Context].
    pub(crate) scope: Option<Scope>,
}

#[Object]
impl Query {
    /// Fetch a `Node` by its globally unique `ID`. Returns `null` if the node cannot be found (e.g., the underlying data was pruned or never existed).
    async fn node(&self, ctx: &Context<'_>, id: Id) -> Option<Result<Node, RpcError>> {
        async {
            let scope = self.scope(ctx)?;
            Ok(match id {
                Id::Address(a) => Some(Node::Address(Box::new(Address::with_address(scope, a)))),

                Id::Checkpoint(s) => Checkpoint::with_sequence_number(scope, Some(s))
                    .map(Box::new)
                    .map(Node::Checkpoint),

                Id::DynamicFieldByAddress(a) => {
                    let object = Object::with_address(scope, a);
                    DynamicField::from_object(&object, ctx)
                        .await?
                        .map(Box::new)
                        .map(Node::DynamicField)
                }

                Id::DynamicFieldByRef(a, v, d) => {
                    let object = Object::with_ref(&scope, a, v, d);
                    DynamicField::from_object(&object, ctx)
                        .await?
                        .map(Box::new)
                        .map(Node::DynamicField)
                }

                Id::Epoch(e) => Some(Node::Epoch(Box::new(Epoch::with_id(scope, e)))),

                Id::MoveObjectByAddress(a) => {
                    let object = Object::with_address(scope, a);
                    MoveObject::from_object(&object, ctx)
                        .await?
                        .map(Box::new)
                        .map(Node::MoveObject)
                }

                Id::MoveObjectByRef(a, v, d) => {
                    let object = Object::with_ref(&scope, a, v, d);
                    MoveObject::from_object(&object, ctx)
                        .await?
                        .map(Box::new)
                        .map(Node::MoveObject)
                }

                Id::MovePackage(a) => Some(Node::MovePackage(Box::new(MovePackage::with_address(
                    scope, a,
                )))),

                Id::ObjectByAddress(a) => {
                    Some(Node::Object(Box::new(Object::with_address(scope, a))))
                }

                Id::ObjectByRef(a, v, d) => {
                    Some(Node::Object(Box::new(Object::with_ref(&scope, a, v, d))))
                }

                Id::Transaction(d) => Some(Node::Transaction(Box::new(Transaction::with_digest(
                    scope, d,
                )))),

                Id::Profile(addr) => {
                    let reader_opt = match ctx.data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>() {
                        Some(r) => r,
                        None => return Ok(None),
                    };
                    if let Some(reader) = reader_opt.as_ref() {
                        if let Ok(response) =
                            reader.get_profile_or_wallet_by_address(&addr.to_string()).await
                        {
                            Profile::from_response(response)
                                .map(|p| Node::Profile(Box::new(p)))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }

                Id::Post(post_id) => {
                    let reader_opt = match ctx.data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>() {
                        Some(r) => r,
                        None => return Ok(None),
                    };
                    if let Some(reader) = reader_opt.as_ref() {
                        if let Ok(Some(post)) = reader.get_post_by_id(&post_id).await {
                            Some(Node::Post(Box::new(Post::from_db(post))))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }

                Id::Platform(platform_id) => {
                    let reader_opt = match ctx.data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>() {
                        Some(r) => r,
                        None => return Ok(None),
                    };
                    if let Some(reader) = reader_opt.as_ref() {
                        if let Ok(Some(platform)) =
                            reader.get_platform_by_id(&platform_id).await
                        {
                            Some(Node::Platform(Box::new(Platform::from_db(platform))))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            })
        }
        .await
        .transpose()
    }

    /// Fetch a social profile by owner address. Returns null if social DB not configured or not found.
    async fn profile(
        &self,
        ctx: &Context<'_>,
        address: MySoAddress,
    ) -> Option<Result<Option<Profile>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let addr_str = address.to_string();
        Some(
            reader
                .get_profile_or_wallet_by_address(&addr_str)
                .await
                .map_err(Into::into)
                .map(|r| Profile::from_response(r)),
        )
    }

    /// List social profiles with pagination. Returns empty when social DB not configured.
    async fn profiles(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<Profile>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .get_profiles(limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(Profile::from_db).collect()),
        )
    }

    /// Fetch a post by ID. Returns null if social DB not configured or not found.
    async fn post(
        &self,
        ctx: &Context<'_>,
        id: async_graphql::ID,
    ) -> Option<Result<Option<Post>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_post_by_id(id.as_str())
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(Post::from_db)),
        )
    }

    /// List posts with optional filters. Returns empty when social DB not configured.
    async fn posts(
        &self,
        ctx: &Context<'_>,
        owner: Option<String>,
        post_type: Option<String>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<Post>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_posts(owner.as_deref(), post_type.as_deref(), limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(Post::from_db).collect()),
        )
    }

    /// Fetch a comment by ID. Returns null if social DB not configured or not found.
    async fn comment(
        &self,
        ctx: &Context<'_>,
        id: async_graphql::ID,
    ) -> Option<Result<Option<CommentSummary>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_comment_by_id(id.as_str())
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(CommentSummary::from_row)),
        )
    }

    /// Reactions for a post (paginated). Returns empty when social DB not configured.
    async fn reactions(
        &self,
        ctx: &Context<'_>,
        post_id: async_graphql::ID,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<ReactionSummary>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .get_post_reactions(post_id.as_str(), limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(ReactionSummary::from_row).collect()),
        )
    }

    /// Reposts of a post (paginated). Returns empty when social DB not configured.
    async fn reposts(
        &self,
        ctx: &Context<'_>,
        post_id: async_graphql::ID,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<RepostSummary>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .get_post_reposts(post_id.as_str(), limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(RepostSummary::from_row).collect()),
        )
    }

    /// Tips received for a post (paginated). Returns empty when social DB not configured.
    async fn tips(
        &self,
        ctx: &Context<'_>,
        post_id: async_graphql::ID,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<TipSummary>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .get_post_tips(post_id.as_str(), limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(TipSummary::from_row).collect()),
        )
    }

    /// Fetch a promotion by ID. Returns null if social DB not configured or not found.
    async fn promotion(
        &self,
        ctx: &Context<'_>,
        id: async_graphql::ID,
    ) -> Option<Result<Option<Promotion>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            async {
                let row = reader
                    .get_promotion(id.as_str())
                    .await
                    .map_err(RpcError::from)?;
                let row = match row {
                    Some(r) => r,
                    None => return Ok::<Option<Promotion>, RpcError>(None),
                };
                let views = reader
                    .get_promotion_views_count(&row.promotion_id)
                    .await
                    .map_err(RpcError::from)?;
                Ok(Some(Promotion::from_row(row, views)))
            }
            .await,
        )
    }

    /// List promoted posts (paginated, optionally filtered by platform). Returns empty when social DB not configured.
    async fn promoted_posts(
        &self,
        ctx: &Context<'_>,
        platform_id: Option<String>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<Promotion>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            async {
                let rows = reader
                    .list_promoted_posts(platform_id.as_deref(), limit, offset)
                    .await
                    .map_err(RpcError::from)?;
                let mut out = Vec::with_capacity(rows.len());
                for row in rows {
                    let views = reader
                        .get_promotion_views_count(&row.promotion_id)
                        .await
                        .map_err(RpcError::from)?;
                    out.push(Promotion::from_row(row, views));
                }
                Ok::<Vec<Promotion>, RpcError>(out)
            }
            .await,
        )
    }

    /// Top performing promotions by view count.
    async fn top_performing_promotions(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
    ) -> Option<Result<Vec<Promotion>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        Some(
            async {
                let rows = reader
                    .get_top_performing_promotions(limit)
                    .await
                    .map_err(RpcError::from)?;
                let mut out = Vec::with_capacity(rows.len());
                for row in rows {
                    let views = reader
                        .get_promotion_views_count(&row.promotion_id)
                        .await
                        .map_err(RpcError::from)?;
                    out.push(Promotion::from_row(row, views));
                }
                Ok::<Vec<Promotion>, RpcError>(out)
            }
            .await,
        )
    }

    /// Global promotion spending trends (last 30 days).
    async fn promotion_spending_trends(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
    ) -> Option<Result<Vec<PromotionTimeSeries>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(30).min(90) as i64;
        Some(
            reader
                .get_spending_trends(limit)
                .await
                .map_err(RpcError::from)
                .map(|rows| {
                    rows.into_iter()
                        .map(PromotionTimeSeries::from_row)
                        .collect()
                }),
        )
    }

    /// Fetch a platform by ID. Returns null if social DB not configured or not found.
    async fn platform(
        &self,
        ctx: &Context<'_>,
        id: async_graphql::ID,
    ) -> Option<Result<Option<Platform>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_platform_by_id(id.as_str())
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(Platform::from_db)),
        )
    }

    /// List platforms with optional approved filter. Returns empty when social DB not configured.
    async fn platforms(
        &self,
        ctx: &Context<'_>,
        approved_only: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<Platform>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_platforms(approved_only.unwrap_or(false), limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(Platform::from_db).collect()),
        )
    }

    /// Fetch a vesting wallet by ID. Returns null if social DB not configured or not found.
    async fn vesting_wallet(
        &self,
        ctx: &Context<'_>,
        id: async_graphql::ID,
    ) -> Option<Result<Option<VestingWallet>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_vesting_wallet(id.as_str())
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(VestingWallet::from_row)),
        )
    }

    /// List vesting wallets with optional owner and active-only filters. Returns empty when social DB not configured.
    async fn vesting_wallets(
        &self,
        ctx: &Context<'_>,
        owner: Option<MySoAddress>,
        active_only: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<VestingWallet>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let owner_str = owner.as_ref().map(|a| a.to_string());
        Some(
            reader
                .list_vesting_wallets(
                    owner_str.as_deref(),
                    active_only.unwrap_or(false),
                    limit,
                    offset,
                )
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(VestingWallet::from_row).collect()),
        )
    }

    /// Vesting leaderboard by total vested amount. Returns empty when social DB not configured.
    async fn vesting_leaderboard(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<VestingLeaderboardResponse, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .get_vesting_leaderboard(limit, offset)
                .await
                .map_err(Into::into)
                .map(|r| VestingLeaderboardResponse {
                    entries: r
                        .entries
                        .into_iter()
                        .map(VestingLeaderboardEntry::from_row)
                        .collect(),
                    total: r.total,
                }),
        )
    }

    /// SPT holdings for a profile (holder address). Returns empty when social DB not configured.
    async fn spt_holdings(
        &self,
        ctx: &Context<'_>,
        profile: MySoAddress,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<SptHolding>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .get_spt_holdings_by_holder(&profile.to_string(), limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(SptHolding::from_row).collect()),
        )
    }

    /// SPT pool by ID. Returns null if social DB not configured or not found.
    async fn spt_pool(
        &self,
        ctx: &Context<'_>,
        id: async_graphql::ID,
    ) -> Option<Result<Option<SptPool>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_spt_pool(id.as_str())
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(SptPool::from_row)),
        )
    }

    /// List SPT pools with optional token type filter and sorting. Returns empty when social DB not configured.
    async fn spt_pools(
        &self,
        ctx: &Context<'_>,
        token_type: Option<i32>,
        sort_by: Option<SptSortBy>,
        order: Option<SptOrder>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<SptPool>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let sort_by = sort_by.unwrap_or_default();
        let ascending = matches!(order, Some(SptOrder::Asc));
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let token_type = token_type.map(|t| t as i16);
        Some(
            reader
                .list_spt_pools(token_type, sort_by.into(), ascending, limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(SptPool::from_row).collect()),
        )
    }

    /// Current reservation holders for a reservation pool id. Returns null when social DB not configured.
    async fn spt_reservation_holders(
        &self,
        ctx: &Context<'_>,
        pool_id: async_graphql::ID,
        viewer: Option<MySoAddress>,
        prioritize_followed: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<SptReservationHolding>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        let prioritize = prioritize_followed.unwrap_or(false);
        Some(
            reader
                .get_reservation_holdings_for_pool(
                    pool_id.as_str(),
                    limit,
                    offset,
                    viewer_s.as_deref(),
                    prioritize,
                )
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(SptReservationHolding::from_row).collect()),
        )
    }

    /// Former reservation holders for a reservation pool id (withdrawn). Returns null when social DB not configured.
    async fn spt_former_reservation_holders(
        &self,
        ctx: &Context<'_>,
        pool_id: async_graphql::ID,
        viewer: Option<MySoAddress>,
        prioritize_followed: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<SptReservationHolding>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        let prioritize = prioritize_followed.unwrap_or(false);
        Some(
            reader
                .get_former_reservation_holdings_for_pool(
                    pool_id.as_str(),
                    limit,
                    offset,
                    viewer_s.as_deref(),
                    prioritize,
                )
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(SptReservationHolding::from_row).collect()),
        )
    }

    /// SPT price history. Provide profileAddress or poolId (at least one required). Returns empty when social DB not configured.
    async fn spt_price_history(
        &self,
        ctx: &Context<'_>,
        profile_address: Option<MySoAddress>,
        pool_id: Option<async_graphql::ID>,
        limit: Option<u64>,
    ) -> Option<Result<Vec<SptPriceHistory>, RpcError<std::io::Error>>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;

        let pool_id_resolved = if let Some(id) = &pool_id {
            Some(id.as_str().to_string())
        } else if let Some(addr) = &profile_address {
            reader
                .get_spt_pool_id_for_profile(&addr.to_string())
                .await
                .ok()
                .flatten()
        } else {
            return Some(Err(bad_user_input(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Either profileAddress or poolId is required",
            ))));
        };

        let Some(pool_id_str) = pool_id_resolved else {
            return Some(Ok(vec![]));
        };

        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = 0i64;
        Some(
            reader
                .get_spt_price_history(&pool_id_str, limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(SptPriceHistory::from_row).collect()),
        )
    }

    /// Time-bucketed reservation deposit / withdrawal volume for a reservation pool (MYSO base units). Returns empty when social DB not configured.
    async fn spt_reservation_volume_history(
        &self,
        ctx: &Context<'_>,
        pool_id: async_graphql::ID,
        interval: SptReservationVolumeInterval,
        limit: Option<u64>,
        from: Option<String>,
        to: Option<String>,
    ) -> Option<Result<Vec<SptReservationVolumeBucket>, RpcError<std::io::Error>>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;

        let parse_bound = |label: &str, s: &str| {
            chrono::DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .map_err(|e| {
                    bad_user_input(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("invalid {label} (expected RFC3339): {e}"),
                    ))
                })
        };

        let from_dt = match from.as_deref() {
            None => None,
            Some(s) => match parse_bound("from", s) {
                Ok(dt) => Some(dt),
                Err(e) => return Some(Err(e)),
            },
        };
        let to_dt = match to.as_deref() {
            None => None,
            Some(s) => match parse_bound("to", s) {
                Ok(dt) => Some(dt),
                Err(e) => return Some(Err(e)),
            },
        };

        let limit = limit.unwrap_or(168).min(500) as i64;
        let interval_reader =
            myso_indexer_alt_social_reader::SptReservationVolumeInterval::from(interval);
        Some(
            reader
                .get_spt_reservation_volume_history(
                    pool_id.as_str(),
                    interval_reader,
                    limit,
                    from_dt,
                    to_dt,
                )
                .await
                .map_err(Into::into)
                .map(|v| {
                    v.into_iter()
                        .map(SptReservationVolumeBucket::from_row)
                        .collect()
                }),
        )
    }

    /// Spot bets for a post. Returns empty when social DB not configured.
    async fn spot_bets(
        &self,
        ctx: &Context<'_>,
        post_id: async_graphql::ID,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<SpotBet>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_spot_bets(post_id.as_str(), limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(SpotBet::from_row).collect()),
        )
    }

    /// Spot record for a post (1:1). Returns null when social DB not configured or no record.
    async fn spot_record(
        &self,
        ctx: &Context<'_>,
        post_id: async_graphql::ID,
    ) -> Option<Result<Option<SpotRecord>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_spot_record(post_id.as_str())
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(SpotRecord::from_row)),
        )
    }

    /// Spot payouts for a post. Returns empty when social DB not configured.
    async fn spot_payouts(
        &self,
        ctx: &Context<'_>,
        post_id: async_graphql::ID,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<SpotPayout>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_spot_payouts(post_id.as_str(), limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(SpotPayout::from_row).collect()),
        )
    }

    /// Spot refunds for a post. Returns empty when social DB not configured.
    async fn spot_refunds(
        &self,
        ctx: &Context<'_>,
        post_id: async_graphql::ID,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<SpotRefund>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_spot_refunds(post_id.as_str(), limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(SpotRefund::from_row).collect()),
        )
    }

    /// Spot resolution for a post (1:1). Returns null when social DB not configured or not resolved.
    async fn spot_resolution(
        &self,
        ctx: &Context<'_>,
        post_id: async_graphql::ID,
    ) -> Option<Result<Option<SpotResolution>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_spot_resolution(post_id.as_str())
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(SpotResolution::from_row)),
        )
    }

    /// Spot bet withdrawals for a post. Returns empty when social DB not configured.
    async fn spot_bet_withdrawals(
        &self,
        ctx: &Context<'_>,
        post_id: async_graphql::ID,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<SpotBetWithdrawal>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_spot_bet_withdrawals(post_id.as_str(), limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(SpotBetWithdrawal::from_row).collect()),
        )
    }

    /// MyData record by ID. Returns null when social DB not configured or not found.
    async fn mydata_record(
        &self,
        ctx: &Context<'_>,
        id: async_graphql::ID,
    ) -> Option<Result<Option<MyDataRecord>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_mydata_record(id.as_str())
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(MyDataRecord::from_row)),
        )
    }

    /// MyData records by owner. Returns empty when social DB not configured.
    async fn mydata_records(
        &self,
        ctx: &Context<'_>,
        owner: MySoAddress,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<MyDataRecord>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_mydata_records_by_owner(&owner.to_string(), limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(MyDataRecord::from_row).collect()),
        )
    }

    /// MyData purchases by buyer. Returns empty when social DB not configured.
    async fn mydata_purchases(
        &self,
        ctx: &Context<'_>,
        buyer: MySoAddress,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<MyDataPurchase>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_mydata_purchases_by_buyer(&buyer.to_string(), limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(MyDataPurchase::from_row).collect()),
        )
    }

    /// List MyData records (paginated, optionally filtered by creator, media_type, platform_id). Returns empty when social DB not configured.
    async fn list_mydata(
        &self,
        ctx: &Context<'_>,
        creator: Option<MySoAddress>,
        media_type: Option<String>,
        platform_id: Option<String>,
        sort_by: Option<String>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<MyDataRecord>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let creator_str = creator.as_ref().map(|a| a.to_string());
        Some(
            reader
                .list_mydata(
                    creator_str.as_deref(),
                    media_type.as_deref(),
                    platform_id.as_deref(),
                    sort_by.as_deref(),
                    limit,
                    offset,
                )
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(MyDataRecord::from_row).collect()),
        )
    }

    /// Popular MyData records (ordered by purchase + revenue + access counts). Returns empty when social DB not configured.
    async fn popular_mydata(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<MyDataRecord>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .get_popular_mydata(limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(MyDataRecord::from_row).collect()),
        )
    }

    /// MyData query marketplace broad pools (indexed `BroadPoolCreatedEvent`). Empty when social DB not configured.
    async fn mydata_query_broad_pools(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<MyDataQueryBroadPool>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_mydata_query_broad_pools(limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(MyDataQueryBroadPool::from_row).collect()),
        )
    }

    /// Sub-pools under a broad pool.
    async fn mydata_query_sub_pools_for_broad_pool(
        &self,
        ctx: &Context<'_>,
        broad_pool_id: String,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<MyDataQuerySubPool>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_mydata_query_sub_pools_for_broad_pool(&broad_pool_id, limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(MyDataQuerySubPool::from_row).collect()),
        )
    }

    /// Listings assigned to a sub-pool (junction).
    async fn mydata_query_listings_for_sub_pool(
        &self,
        ctx: &Context<'_>,
        sub_pool_id: String,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<MyDataQueryListingSubPool>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_mydata_query_listings_for_sub_pool(&sub_pool_id, limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(MyDataQueryListingSubPool::from_row).collect()),
        )
    }

    /// Latest snapshot anchor for a snapshot ID (includes manifest hash and payment reference when indexed from upgraded packages).
    async fn mydata_query_snapshot_anchor(
        &self,
        ctx: &Context<'_>,
        snapshot_id: String,
    ) -> Option<Result<Option<MyDataQuerySnapshotAnchor>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_mydata_query_snapshot_anchor(&snapshot_id)
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(MyDataQuerySnapshotAnchor::from_row)),
        )
    }

    /// Recent snapshot anchors (paginated).
    async fn mydata_query_snapshot_anchors(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<MyDataQuerySnapshotAnchor>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_mydata_query_snapshot_anchors(limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(MyDataQuerySnapshotAnchor::from_row).collect()),
        )
    }

    /// Distribution round for a snapshot (from `DistributionRecordedEvent` when indexed).
    async fn mydata_query_distribution_round(
        &self,
        ctx: &Context<'_>,
        snapshot_id: String,
    ) -> Option<Result<Option<MyDataQueryDistributionRound>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_mydata_query_distribution_round(&snapshot_id)
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(MyDataQueryDistributionRound::from_row)),
        )
    }

    /// Recent distribution rounds (paginated).
    async fn mydata_query_distribution_rounds(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<MyDataQueryDistributionRound>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_mydata_query_distribution_rounds(limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| {
                    v.into_iter()
                        .map(MyDataQueryDistributionRound::from_row)
                        .collect()
                }),
        )
    }

    /// Published Merkle root for a snapshot.
    async fn mydata_query_merkle_root(
        &self,
        ctx: &Context<'_>,
        snapshot_id: String,
    ) -> Option<Result<Option<MyDataQueryMerkleRoot>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_mydata_query_merkle_root(&snapshot_id)
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(MyDataQueryMerkleRoot::from_row)),
        )
    }

    /// Claim events for a snapshot.
    async fn mydata_query_claims_for_snapshot(
        &self,
        ctx: &Context<'_>,
        snapshot_id: String,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<MyDataQueryClaim>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(200) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_mydata_query_claims_for_snapshot(&snapshot_id, limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(MyDataQueryClaim::from_row).collect()),
        )
    }

    /// List governance proposals (paginated, optionally filtered by platform, registry type, status, submitter). Returns empty when social DB not configured.
    ///
    /// `registryType` matches `Proposal.registryType` (0=ecosystem, 1=proof of creativity, 2=platform).
    /// If `platformId` is set, the effective registry type comes from that platform's governance registry and `registryType` is ignored.
    /// New on-chain proposals start in delegate review (`status` 1), not submitted (`status` 0).
    async fn proposals(
        &self,
        ctx: &Context<'_>,
        platform_id: Option<String>,
        #[graphql(name = "registryType")] registry_type: Option<i16>,
        status: Option<i16>,
        submitter: Option<String>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<Proposal>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_proposals(
                    platform_id.as_deref(),
                    status,
                    registry_type,
                    submitter.as_deref(),
                    limit,
                    offset,
                )
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(Proposal::from_row).collect()),
        )
    }

    /// Fetch a proposal by ID. Returns null when social DB not configured or not found.
    async fn proposal(
        &self,
        ctx: &Context<'_>,
        id: async_graphql::ID,
    ) -> Option<Result<Option<Proposal>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_proposal_by_id(id.as_str())
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(Proposal::from_row)),
        )
    }

    /// List delegates (paginated, optionally filtered by registry type and active status). Returns empty when social DB not configured.
    async fn delegates(
        &self,
        ctx: &Context<'_>,
        registry_type: Option<i16>,
        is_active: Option<bool>,
        viewer: Option<MySoAddress>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<Delegate>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        Some(
            async {
                let rows = reader
                    .list_delegates(registry_type, is_active, limit, offset)
                    .await
                    .map_err(RpcError::from)?;
                let ctx_map = if let Some(ref vs) = viewer_s {
                    let addrs: Vec<String> = rows.iter().map(|r| r.address.clone()).collect();
                    reader
                        .batch_viewer_social_context_for_addresses(&addrs, vs)
                        .await
                        .ok()
                } else {
                    None
                };
                Ok(rows
                    .into_iter()
                    .map(|row| {
                        let c = ctx_map.as_ref().and_then(|m| m.get(&row.address)).copied();
                        Delegate::with_viewer(row, c)
                    })
                    .collect())
            }
            .await,
        )
    }

    /// Fetch a delegate by address. Returns null when social DB not configured or not found.
    async fn delegate(
        &self,
        ctx: &Context<'_>,
        address: MySoAddress,
        viewer: Option<MySoAddress>,
    ) -> Option<Result<Option<Delegate>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let viewer_s = viewer.map(|a| a.to_string());
        Some(
            async {
                let opt = reader
                    .get_delegate_by_address(&address.to_string())
                    .await
                    .map_err(RpcError::from)?;
                Ok(match opt {
                    None => None,
                    Some(row) => {
                        let c = if let Some(ref vs) = viewer_s {
                            reader
                                .batch_viewer_social_context_for_addresses(
                                    &[row.address.clone()],
                                    vs,
                                )
                                .await
                                .ok()
                                .and_then(|m| m.get(&row.address).copied())
                        } else {
                            None
                        };
                        Some(Delegate::with_viewer(row, c))
                    }
                })
            }
            .await,
        )
    }

    /// List nominated delegates (paginated, optionally filtered by platform, registry type, and status).
    /// Returns empty when social DB not configured.
    ///
    /// `registryType`: 0=ecosystem, 1=proof of creativity, 2=platform DAO.
    /// If `platformId` is set, results are scoped to that platform's governance registry (same resolution
    /// as `proposals(platformId:)` / `governanceRegistry(platformId:)`), and `registryType` is ignored.
    /// **Platform DAO (`registryType` 2) requires `platformId`:** when `registryType` is 2 and `platformId`
    /// is omitted, returns an empty list (no cross-platform aggregation).
    ///
    /// Without `platformId`, only ecosystem/PoC nominee rows are returned (`governance_registry_id` unset).
    /// Unfiltered lists omit `registryType` 2.
    async fn nominated_delegates(
        &self,
        ctx: &Context<'_>,
        platform_id: Option<String>,
        #[graphql(name = "registryType")] registry_type: Option<i16>,
        status: Option<i16>,
        viewer: Option<MySoAddress>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<NominatedDelegate>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        if platform_id.is_none() && registry_type == Some(2) {
            return Some(Ok(vec![]));
        }
        let mut limit = limit.unwrap_or(50).min(100) as i64;
        if limit == 0 {
            limit = 50;
        }
        let offset = offset.unwrap_or(0) as i64;
        let viewer_s = viewer.map(|a| a.to_string());
        let effective_registry_type = if platform_id.is_some() {
            None
        } else {
            registry_type
        };
        Some(
            async {
                let rows = reader
                    .list_nominated_delegates(
                        platform_id.as_deref(),
                        effective_registry_type,
                        status,
                        limit,
                        offset,
                    )
                    .await
                    .map_err(RpcError::from)?;
                let ctx_map = if let Some(ref vs) = viewer_s {
                    let addrs: Vec<String> = rows.iter().map(|r| r.address.clone()).collect();
                    reader
                        .batch_viewer_social_context_for_addresses(&addrs, vs)
                        .await
                        .ok()
                } else {
                    None
                };
                Ok(rows
                    .into_iter()
                    .map(|row| {
                        let c = ctx_map.as_ref().and_then(|m| m.get(&row.address)).copied();
                        NominatedDelegate::with_viewer(row, c)
                    })
                    .collect())
            }
            .await,
        )
    }

    /// List governance registries, optionally filtered by registry type. Returns empty when social DB not configured.
    async fn governance_registries(
        &self,
        ctx: &Context<'_>,
        registry_type: Option<i16>,
    ) -> Option<Result<Vec<GovernanceRegistry>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .list_governance_registries(registry_type)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(GovernanceRegistry::from_row).collect()),
        )
    }

    /// List governance events (paginated). Returns empty when social DB not configured.
    async fn governance_events(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<GovernanceEvent>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_governance_events(limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(GovernanceEvent::from_row).collect()),
        )
    }

    /// Anonymous voting trends (daily aggregates). Returns empty when social DB not configured.
    async fn anonymous_voting_trends(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
    ) -> Option<Result<Vec<AnonymousVotingTrend>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(30).min(100) as i64;
        Some(
            reader
                .get_anonymous_voting_trends(limit)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(AnonymousVotingTrend::from_row).collect()),
        )
    }

    /// Fetch governance registry for a platform (by platform ID). Returns null when social DB not configured or platform has no registry.
    async fn governance_registry(
        &self,
        ctx: &Context<'_>,
        platform_id: String,
    ) -> Option<Result<Option<GovernanceRegistry>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_governance_registry_by_platform_id(&platform_id)
                .await
                .map_err(Into::into)
                .map(|opt| {
                    opt.map(|row| {
                        GovernanceRegistry::from_row_with_platform(row, Some(platform_id))
                    })
                }),
        )
    }

    /// Insurance policy by ID. Returns null when social DB not configured or not found.
    async fn insurance_policy(
        &self,
        ctx: &Context<'_>,
        id: async_graphql::ID,
    ) -> Option<Result<Option<InsurancePolicy>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_insurance_policy(id.as_str())
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(InsurancePolicy::from_row)),
        )
    }

    /// Insurance policies with optional filters. Returns empty when social DB not configured.
    async fn insurance_policies(
        &self,
        ctx: &Context<'_>,
        insured: Option<MySoAddress>,
        market_id: Option<String>,
        vault_id: Option<String>,
        status: Option<i16>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<InsurancePolicy>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let insured_str = insured.map(|a| a.to_string());
        Some(
            reader
                .list_insurance_policies(
                    insured_str.as_deref(),
                    market_id.as_deref(),
                    vault_id.as_deref(),
                    status,
                    limit,
                    offset,
                )
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(InsurancePolicy::from_row).collect()),
        )
    }

    /// Insurance vault by ID. Returns null when social DB not configured or not found.
    async fn insurance_vault(
        &self,
        ctx: &Context<'_>,
        vault_id: async_graphql::ID,
    ) -> Option<Result<Option<InsuranceVault>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_insurance_vault(vault_id.as_str())
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(InsuranceVault::from_row)),
        )
    }

    /// Insurance vaults. Returns empty when social DB not configured.
    async fn insurance_vaults(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Result<Vec<InsuranceVault>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        Some(
            reader
                .list_insurance_vaults(limit, offset)
                .await
                .map_err(Into::into)
                .map(|v| v.into_iter().map(InsuranceVault::from_row).collect()),
        )
    }

    /// SPT exchange configuration. Returns null when social DB not configured or no config.
    async fn spt_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Option<Result<Option<SptExchangeConfig>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_spt_exchange_config()
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(SptExchangeConfig::from_row)),
        )
    }

    /// Post configuration. Returns null when social DB not configured or no config.
    async fn post_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Option<Result<Option<PostConfig>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_post_config()
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(PostConfig::from_row)),
        )
    }

    /// Proof of Creativity configuration. Returns null when social DB not configured or no config.
    async fn poc_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Option<Result<Option<PocConfig>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_poc_configuration()
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(PocConfig::from_row)),
        )
    }

    /// SPoT (Social Proof of Truth) configuration. Returns null when social DB not configured or no config.
    async fn spot_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Option<Result<Option<SpotConfig>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_spot_config()
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(SpotConfig::from_row)),
        )
    }

    /// MyData configuration. Returns null when social DB not configured or no config.
    async fn mydata_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Option<Result<Option<MyDataConfig>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_mydata_config()
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(MyDataConfig::from_row)),
        )
    }

    /// Insurance configuration. Returns null when social DB not configured or no config.
    async fn insurance_configuration(
        &self,
        ctx: &Context<'_>,
    ) -> Option<Result<Option<InsuranceConfig>, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_insurance_config()
                .await
                .map_err(Into::into)
                .map(|opt| opt.map(InsuranceConfig::from_row)),
        )
    }

    /// Check if follower follows following. Returns null when social DB not configured.
    async fn social_graph_following(
        &self,
        ctx: &Context<'_>,
        follower: MySoAddress,
        following: MySoAddress,
    ) -> Option<Result<bool, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .check_following(&follower.to_string(), &following.to_string())
                .await
                .map_err(Into::into),
        )
    }

    /// Check if blocker has blocked blocked. Returns null when social DB not configured.
    async fn check_profile_blocked(
        &self,
        ctx: &Context<'_>,
        blocker: MySoAddress,
        blocked: MySoAddress,
    ) -> Option<Result<bool, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .check_profile_blocked(&blocker.to_string(), &blocked.to_string())
                .await
                .map_err(Into::into),
        )
    }

    /// Check if platform has blocked this profile. Returns null when social DB not configured.
    async fn check_platform_blocked(
        &self,
        ctx: &Context<'_>,
        profile: MySoAddress,
        platform: async_graphql::ID,
    ) -> Option<Result<bool, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .check_platform_blocked(&profile.to_string(), platform.as_str())
                .await
                .map_err(Into::into),
        )
    }

    /// Member, platform-block, and moderator flags for a wallet (single DB round-trip).
    /// Returns null when social DB not configured.
    async fn platform_user_access(
        &self,
        ctx: &Context<'_>,
        platform: async_graphql::ID,
        user: MySoAddress,
    ) -> Option<Result<PlatformUserAccess, RpcError>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        Some(
            reader
                .get_platform_user_access(platform.as_str(), &user.to_string())
                .await
                .map_err(Into::into)
                .map(PlatformUserAccess::from_row),
        )
    }

    /// Look-up an account by its MySoAddress.
    ///
    /// If `rootVersion` is specified, nested dynamic field accesses will be fetched at or before this version. This can be used to fetch a child or descendant object bounded by its root object's version, when its immediate parent is wrapped, or a value in a dynamic object field. For any wrapped or child (object-owned) object, its root object can be defined recursively as:
    ///
    /// - The root object of the object it is wrapped in, if it is wrapped.
    /// - The root object of its owner, if it is owned by another object.
    /// - The object itself, if it is not object-owned or wrapped.
    ///
    /// Specifying a `rootVersion` disables nested queries for paginating owned objects or dynamic fields (these queries are only supported at checkpoint boundaries).
    ///
    /// If `atCheckpoint` is specified, the address will be fetched at the latest version as of this checkpoint. This will fail if the provided checkpoint is after the RPC's latest checkpoint.
    ///
    /// If none of the above are specified, the address is fetched at the checkpoint being viewed.
    ///
    /// Returns `null` if the address does not exist.
    async fn address(
        &self,
        ctx: &Context<'_>,
        address: Option<MySoAddress>,
        root_version: Option<UInt53>,
        at_checkpoint: Option<UInt53>,
    ) -> Option<Result<Address, RpcError<address::Error>>> {
        async {
            Address::by_key(
                ctx,
                self.scope(ctx)?,
                AddressKey {
                    address,
                    root_version,
                    at_checkpoint,
                },
            )
            .await
        }
        .await
        .transpose()
    }

    /// The network's genesis checkpoint digest (uniquely identifies the network), Base58-encoded.
    async fn chain_identifier(&self, ctx: &Context<'_>) -> Result<String, RpcError> {
        let chain_id: &ChainIdentifier = ctx.data()?;
        Ok(Base58::encode(chain_id.wait().await.as_bytes()))
    }

    /// Fetch a checkpoint by its sequence number, or the latest checkpoint if no sequence number is provided.
    ///
    /// Returns `null` if the checkpoint does not exist in the store, either because it never existed or because it was pruned.
    async fn checkpoint(
        &self,
        ctx: &Context<'_>,
        sequence_number: Option<UInt53>,
    ) -> Option<Result<Checkpoint, RpcError>> {
        async {
            let scope = self.scope(ctx)?;
            Ok(Checkpoint::with_sequence_number(
                scope,
                sequence_number.map(|s| s.into()),
            ))
        }
        .await
        .transpose()
    }

    /// Paginate checkpoints in the network, optionally bounded to checkpoints in the given epoch.
    async fn checkpoints(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<CCheckpoint>,
        last: Option<u64>,
        before: Option<CCheckpoint>,
        filter: Option<CheckpointFilter>,
    ) -> Option<Result<Connection<String, Checkpoint>, RpcError>> {
        Some(
            async {
                let scope = self.scope(ctx)?;
                let pagination: &PaginationConfig = ctx.data()?;
                let limits = pagination.limits("Query", "checkpoints");
                let page = Page::from_params(limits, first, after, last, before)?;

                let filter = filter.unwrap_or_default();
                Checkpoint::paginate(ctx, scope, page, filter).await
            }
            .await,
        )
    }

    /// Fetch the CoinMetadata for a given coin type.
    ///
    /// Returns `null` if no CoinMetadata object exists for the given coin type.
    async fn coin_metadata(
        &self,
        ctx: &Context<'_>,
        coin_type: TypeInput,
    ) -> Option<Result<CoinMetadata, RpcError<object::Error>>> {
        async { CoinMetadata::by_coin_type(ctx, self.scope(ctx)?, coin_type.into()).await }
            .await
            .transpose()
    }

    /// Fetch an epoch by its ID, or fetch the latest epoch if no ID is provided.
    ///
    /// Returns `null` if the epoch does not exist yet, or was pruned.
    async fn epoch(
        &self,
        ctx: &Context<'_>,
        epoch_id: Option<UInt53>,
    ) -> Option<Result<Epoch, RpcError>> {
        async {
            let scope = self.scope(ctx)?;
            Epoch::fetch(ctx, scope, epoch_id).await
        }
        .await
        .transpose()
    }

    /// Paginate epochs that are in the network.
    async fn epochs(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<CEpoch>,
        last: Option<u64>,
        before: Option<CEpoch>,
    ) -> Option<Result<Connection<String, Epoch>, RpcError>> {
        async {
            let scope = self.scope(ctx)?;
            let pagination: &PaginationConfig = ctx.data()?;
            let limits = pagination.limits("Query", "epochs");
            let page = Page::from_params(limits, first, after, last, before)?;

            Epoch::paginate(ctx, &scope, page).await
        }
        .await
        .transpose()
    }

    /// Paginate events that are emitted in the network, optionally filtered by event filters.
    async fn events(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<CEvent>,
        last: Option<u64>,
        before: Option<CEvent>,
        filter: Option<EventFilter>,
    ) -> Option<Result<Connection<String, Event>, RpcError>> {
        Some(
            async {
                let scope = self.scope(ctx)?;
                let pagination: &PaginationConfig = ctx.data()?;
                let limits = pagination.limits("Query", "events");
                let page = Page::from_params(limits, first, after, last, before)?;

                Event::paginate(ctx, scope, page, filter.unwrap_or_default()).await
            }
            .await,
        )
    }

    /// Fetch addresses by their keys.
    ///
    /// Returns a list of addresses that is guaranteed to be the same length as `keys`. If an address in `keys` does not exist, its corresponding entry in the result will be `null`.
    async fn multi_get_addresses(
        &self,
        ctx: &Context<'_>,
        keys: Vec<AddressKey>,
    ) -> Result<Vec<Option<Address>>, RpcError<address::Error>> {
        let scope = self.scope(ctx)?;
        try_join_all(
            keys.into_iter()
                .map(|k| Address::by_key(ctx, scope.clone(), k)),
        )
        .await
    }

    /// Fetch checkpoints by their sequence numbers.
    ///
    /// Returns a list of checkpoints that is guaranteed to be the same length as `keys`. If a checkpoint in `keys` could not be found in the store, its corresponding entry in the result will be `null`. This could be because the checkpoint does not exist yet, or because it was pruned.
    async fn multi_get_checkpoints(
        &self,
        ctx: &Context<'_>,
        keys: Vec<UInt53>,
    ) -> Result<Vec<Option<Checkpoint>>, RpcError> {
        let scope = self.scope(ctx)?;
        Ok(keys
            .into_iter()
            .map(|k| Checkpoint::with_sequence_number(scope.clone(), Some(k.into())))
            .collect())
    }

    /// Fetch epochs by their IDs.
    ///
    /// Returns a list of epochs that is guaranteed to be the same length as `keys`. If an epoch in `keys` could not be found in the store, its corresponding entry in the result will be `null`. This could be because the epoch does not exist yet, or because it was pruned.
    async fn multi_get_epochs(
        &self,
        ctx: &Context<'_>,
        keys: Vec<UInt53>,
    ) -> Result<Vec<Option<Epoch>>, RpcError> {
        let scope = self.scope(ctx)?;
        let epochs = keys
            .into_iter()
            .map(|k| Epoch::fetch(ctx, scope.clone(), Some(k)));

        try_join_all(epochs).await
    }

    /// Fetch objects by their keys.
    ///
    /// Returns a list of objects that is guaranteed to be the same length as `keys`. If an object in `keys` could not be found in the store, its corresponding entry in the result will be `null`. This could be because the object never existed, or because it was pruned.
    async fn multi_get_objects(
        &self,
        ctx: &Context<'_>,
        keys: Vec<ObjectKey>,
    ) -> Result<Vec<Option<Object>>, RpcError<object::Error>> {
        let scope = self.scope(ctx)?;
        let objects = keys
            .into_iter()
            .map(|k| Object::by_key(ctx, scope.clone(), k));

        try_join_all(objects).await
    }

    /// Fetch packages by their keys.
    ///
    /// Returns a list of packages that is guaranteed to be the same length as `keys`. If a package in `keys` could not be found in the store, its corresponding entry in the result will be `null`. This could be because that address never pointed to a package, or because the package was pruned.
    async fn multi_get_packages(
        &self,
        ctx: &Context<'_>,
        keys: Vec<PackageKey>,
    ) -> Result<Vec<Option<MovePackage>>, RpcError<move_package::Error>> {
        let scope = self.scope(ctx)?;
        let packages = keys
            .into_iter()
            .map(|k| MovePackage::by_key(ctx, scope.clone(), k));

        try_join_all(packages).await
    }

    /// Fetch transactions by their digests.
    ///
    /// Returns a list of transactions that is guaranteed to be the same length as `keys`. If a digest in `keys` could not be found in the store, its corresponding entry in the result will be `null`. This could be because the transaction never existed, or because it was pruned.
    async fn multi_get_transactions(
        &self,
        ctx: &Context<'_>,
        keys: Vec<Digest>,
    ) -> Result<Vec<Option<Transaction>>, RpcError> {
        let scope = self.scope(ctx)?;
        let transactions = keys
            .into_iter()
            .map(|d| Transaction::fetch(ctx, scope.clone(), d));

        try_join_all(transactions).await
    }

    /// Fetch transaction effects by their transactions' digests.
    ///
    /// Returns a list of transaction effects that is guaranteed to be the same length as `keys`. If a digest in `keys` could not be found in the store, its corresponding entry in the result will be `null`. This could be because the transaction effects never existed, or because it was pruned.
    async fn multi_get_transaction_effects(
        &self,
        ctx: &Context<'_>,
        keys: Vec<Digest>,
    ) -> Result<Vec<Option<TransactionEffects>>, RpcError> {
        let scope = self.scope(ctx)?;
        let effects = keys
            .into_iter()
            .map(|d| TransactionEffects::fetch(ctx, scope.clone(), d));

        try_join_all(effects).await
    }

    /// Fetch types by their string representations.
    ///
    /// Types are canonicalized: In the input they can be at any package address at or after the package that first defines them, and in the output they will be relocated to the package that first defines them.
    ///
    /// Returns a list of types that is guaranteed to be the same length as `keys`. If a type in `keys` could not be found, its corresponding entry in the result will be `null`.
    async fn multi_get_types(
        &self,
        ctx: &Context<'_>,
        keys: Vec<TypeInput>,
    ) -> Result<Vec<Option<MoveType>>, RpcError<move_type::Error>> {
        let types = keys
            .into_iter()
            .map(|t| async move { MoveType::canonicalize(t.into(), self.scope(ctx)?).await });

        try_join_all(types).await
    }

    /// Fetch an object by its address.
    ///
    /// If `version` is specified, the object will be fetched at that exact version.
    ///
    /// If `rootVersion` is specified, the object will be fetched at the latest version at or before this version. Nested dynamic field accesses will also be subject to this bound. This can be used to fetch a child or ancestor object bounded by its root object's version. For any wrapped or child (object-owned) object, its root object can be defined recursively as:
    ///
    /// - The root object of the object it is wrapped in, if it is wrapped.
    /// - The root object of its owner, if it is owned by another object.
    /// - The object itself, if it is not object-owned or wrapped.
    ///
    /// Specifying a `version` or a `rootVersion` disables nested queries for paginating owned objects or dynamic fields (these queries are only supported at checkpoint boundaries).
    ///
    /// If `atCheckpoint` is specified, the object will be fetched at the latest version as of this checkpoint. This will fail if the provided checkpoint is after the RPC's latest checkpoint.
    ///
    /// If none of the above are specified, the object is fetched at the checkpoint being viewed.
    ///
    /// It is an error to specify more than one of `version`, `rootVersion`, or `atCheckpoint`.
    ///
    /// Returns `null` if an object cannot be found that meets this criteria.
    async fn object(
        &self,
        ctx: &Context<'_>,
        address: MySoAddress,
        version: Option<UInt53>,
        root_version: Option<UInt53>,
        at_checkpoint: Option<UInt53>,
    ) -> Option<Result<Object, RpcError<object::Error>>> {
        async {
            Object::by_key(
                ctx,
                self.scope(ctx)?,
                ObjectKey {
                    address,
                    version,
                    root_version,
                    at_checkpoint,
                },
            )
            .await
        }
        .await
        .transpose()
    }

    /// Paginate objects in the live object set, optionally filtered by owner and/or type. `filter` can be one of:
    ///
    /// - A filter on type (all live objects whose type matches that filter).
    /// - Fetching all objects owned by an address or object, optionally filtered by type.
    /// - Fetching all shared or immutable objects, filtered by type.
    async fn objects(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::CLive>,
        last: Option<u64>,
        before: Option<object::CLive>,
        #[graphql(validator(custom = "OFValidator::default()"))] filter: ObjectFilter,
    ) -> Option<Result<Connection<String, Object>, RpcError<object::Error>>> {
        Some(
            async {
                let pagination: &PaginationConfig = ctx.data()?;
                let limits = pagination.limits("Query", "objects");
                let page = Page::from_params(limits, first, after, last, before)?;

                Object::paginate_live(ctx, self.scope(ctx)?, page, filter).await
            }
            .await,
        )
    }

    /// Paginate all versions of an object at `address`, optionally bounding the versions exclusively from below with `filter.afterVersion` or from above with `filter.beforeVersion`.
    async fn object_versions(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::CVersion>,
        last: Option<u64>,
        before: Option<object::CVersion>,
        address: MySoAddress,
        filter: Option<VersionFilter>,
    ) -> Option<Result<Connection<String, Object>, RpcError>> {
        Some(
            async {
                let pagination: &PaginationConfig = ctx.data()?;
                let limits = pagination.limits("Query", "objectVersions");
                let page = Page::from_params(limits, first, after, last, before)?;

                Object::paginate_by_version(
                    ctx,
                    self.scope(ctx)?,
                    page,
                    address.into(),
                    filter.unwrap_or_default(),
                )
                .await
            }
            .await,
        )
    }

    /// Fetch a package by its address.
    ///
    /// If `version` is specified, the package loaded is the one that shares its original ID with the package at `address`, but whose version is `version`.
    ///
    /// If `atCheckpoint` is specified, the package loaded is the one with the largest version among all packages sharing an original ID with the package at `address` and was published at or before `atCheckpoint`.
    ///
    /// If neither are specified, the package is fetched at the checkpoint being viewed.
    ///
    /// It is an error to specify both `version` and `atCheckpoint`, and `null` will be returned if the package cannot be found as of the latest checkpoint, or the address points to an object that is not a package.
    ///
    /// Note that this interpretation of `version` and "latest" differs from the one used by `Query.object`, because non-system package upgrades generate objects with different IDs. To fetch a package using the versioning semantics of objects, use `Object.asMovePackage` nested under `Query.object`.
    async fn package(
        &self,
        ctx: &Context<'_>,
        address: MySoAddress,
        version: Option<UInt53>,
        at_checkpoint: Option<UInt53>,
    ) -> Option<Result<MovePackage, RpcError<move_package::Error>>> {
        async {
            MovePackage::by_key(
                ctx,
                self.scope(ctx)?,
                PackageKey {
                    address,
                    version,
                    at_checkpoint,
                },
            )
            .await
        }
        .await
        .transpose()
    }

    /// Paginate all packages published on-chain, optionally bounded to packages published strictly after `filter.afterCheckpoint` and/or strictly before `filter.beforeCheckpoint`.
    async fn packages(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<move_package::CPackage>,
        last: Option<u64>,
        before: Option<move_package::CPackage>,
        filter: Option<PackageCheckpointFilter>,
    ) -> Option<Result<Connection<String, MovePackage>, RpcError>> {
        Some(
            async {
                let pagination: &PaginationConfig = ctx.data()?;
                let limits = pagination.limits("Query", "packages");
                let page = Page::from_params(limits, first, after, last, before)?;

                MovePackage::paginate_by_checkpoint(
                    ctx,
                    self.scope(ctx)?,
                    page,
                    filter.unwrap_or_default(),
                )
                .await
            }
            .await,
        )
    }

    /// Paginate all versions of a package at `address`, optionally bounding the versions exclusively from below with `filter.afterVersion` or from above with `filter.beforeVersion`.
    ///
    /// Different versions of a package will have different object IDs, unless they are system packages, but will share the same original ID.
    async fn package_versions(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<object::CVersion>,
        last: Option<u64>,
        before: Option<object::CVersion>,
        address: MySoAddress,
        filter: Option<VersionFilter>,
    ) -> Option<Result<Connection<String, MovePackage>, RpcError>> {
        Some(
            async {
                let pagination: &PaginationConfig = ctx.data()?;
                let limits = pagination.limits("Query", "packageVersions");
                let page = Page::from_params(limits, first, after, last, before)?;

                MovePackage::paginate_by_version(
                    ctx,
                    self.scope(ctx)?,
                    page,
                    address.into(),
                    filter.unwrap_or_default(),
                )
                .await
            }
            .await,
        )
    }

    /// Fetch the protocol config by protocol version, or the latest protocol config used on chain if no version is provided.
    async fn protocol_configs(
        &self,
        ctx: &Context<'_>,
        version: Option<UInt53>,
    ) -> Option<Result<ProtocolConfigs, RpcError>> {
        async {
            if let Some(version) = version {
                Ok(Some(ProtocolConfigs::with_protocol_version(version.into())))
            } else {
                let scope = self.scope(ctx)?;
                ProtocolConfigs::latest(ctx, &scope).await
            }
        }
        .await
        .transpose()
    }

    /// Configuration for this RPC service.
    async fn service_config(&self, ctx: &Context<'_>) -> Result<ServiceConfig, RpcError> {
        let scope = self.scope(ctx)?;
        Ok(ServiceConfig { scope })
    }

    /// Fetch a transaction by its digest.
    ///
    /// Returns `null` if the transaction does not exist in the store, either because it never existed or because it was pruned.
    async fn transaction(
        &self,
        ctx: &Context<'_>,
        digest: Digest,
    ) -> Option<Result<Transaction, RpcError>> {
        async { Transaction::fetch(ctx, self.scope(ctx)?, digest).await }
            .await
            .transpose()
    }

    /// Fetch transaction effects by its transaction's digest.
    ///
    /// Returns `null` if the transaction effects do not exist in the store, either because that transaction was not executed, or it was pruned.
    async fn transaction_effects(
        &self,
        ctx: &Context<'_>,
        digest: Digest,
    ) -> Option<Result<TransactionEffects, RpcError>> {
        async { TransactionEffects::fetch(ctx, self.scope(ctx)?, digest).await }
            .await
            .transpose()
    }

    /// The transactions that exist in the network, optionally filtered by transaction filters.
    async fn transactions(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<CTransaction>,
        last: Option<u64>,
        before: Option<CTransaction>,
        #[graphql(validator(custom = "TFValidator"))] filter: Option<TransactionFilter>,
    ) -> Option<Result<Connection<String, Transaction>, RpcError>> {
        Some(
            async {
                let scope = self.scope(ctx)?;
                let pagination: &PaginationConfig = ctx.data()?;
                let limits = pagination.limits("Query", "transactions");
                let page = Page::from_params(limits, first, after, last, before)?;

                // Use the filter if provided, otherwise use default (unfiltered)
                let filter = filter.unwrap_or_default();
                Transaction::paginate(ctx, scope, page, filter).await
            }
            .await,
        )
    }

    /// Fetch a structured representation of a concrete type, including its layout information.
    ///
    /// Types are canonicalized: In the input they can be at any package address at or after the package that first defines them, and in the output they will be relocated to the package that first defines them.
    ///
    /// Fails if the type is malformed, returns `null` if a type mentioned does not exist.
    async fn type_(
        &self,
        ctx: &Context<'_>,
        type_: TypeInput,
    ) -> Option<Result<MoveType, RpcError<move_type::Error>>> {
        async { MoveType::canonicalize(type_.into(), self.scope(ctx)?).await }
            .await
            .transpose()
    }

    /// Simulate a transaction to preview its effects without executing it on chain.
    ///
    /// Accepts a JSON transaction matching the [MySo gRPC API schema](https://docs.mysocial.network/references/fullnode-protocol#myso-rpc-v2-Transaction).
    /// The JSON format allows for partial transaction specification where certain fields can be automatically resolved by the server.
    ///
    /// Alternatively, for already serialized transactions, you can pass BCS-encoded data:
    /// `{"bcs": {"value": "<base64>"}}`
    ///
    /// Unlike `executeTransaction`, this does not require signatures since the transaction is not committed to the blockchain. This allows for previewing transaction effects, estimating gas costs, and testing transaction logic without spending gas or requiring valid signatures.
    ///
    /// - `checksEnabled`: If true, enables transaction validation checks during simulation. Defaults to true.
    /// - `doGasSelection`: If true, enables automatic gas coin selection and budget estimation. Defaults to false.
    async fn simulate_transaction(
        &self,
        ctx: &Context<'_>,
        transaction: Json,
        checks_enabled: Option<bool>,
        do_gas_selection: Option<bool>,
    ) -> Result<SimulationResult, RpcError<TransactionInputError>> {
        let fullnode_client: &FullnodeClient = ctx.data()?;

        // Convert Json to serde_json::Value and parse as proto::Transaction
        let json_value: serde_json::Value = transaction
            .try_into()
            .map_err(|err| bad_user_input(TransactionInputError::InvalidTransactionJson(err)))?;
        let proto_tx: proto::Transaction = serde_json::from_value(json_value)
            .map_err(|err| bad_user_input(TransactionInputError::InvalidTransactionJson(err)))?;

        // Simulate transaction using proto
        match fullnode_client
            .simulate_transaction(
                proto_tx,
                checks_enabled.unwrap_or(true),
                do_gas_selection.unwrap_or(false),
            )
            .await
        {
            Ok(response) => {
                let scope = self.scope(ctx)?;
                let tx_data = response
                    .transaction
                    .as_ref()
                    .and_then(|executed_tx| executed_tx.transaction.as_ref())
                    .and_then(|tx| tx.bcs.as_ref())
                    .ok_or_else(|| anyhow!("Missing transaction or BCS in simulation response"))?
                    .deserialize()
                    .context("Failed to deserialize transaction from response")?;

                SimulationResult::from_simulation_response(scope, response, tx_data).map_err(upcast)
            }
            Err(GrpcExecutionError(status)) => Ok(SimulationResult {
                effects: None,
                outputs: None,
                error: Some(status.to_string()),
            }),
            Err(other_error) => Err(anyhow!(other_error)
                .context("Failed to simulate transaction")
                .into()),
        }
    }

    /// Verify a zkLogin signature os from the given `author`.
    ///
    /// Returns a `ZkLoginVerifyResult` where `success` is `true` and `error` is empty if the signature is valid. If the signature is invalid, `success` is `false` and `error` contains the relevant error message.
    ///
    /// - `bytes` are either the bytes of a serialized personal message, or `TransactionData`, Base64-encoded.
    /// - `signature` is a serialized zkLogin signature, also Base64-encoded.
    /// - `intentScope` indicates whether `bytes` are to be parsed as a personal message or `TransactionData`.
    /// - `author` is the signer's address.
    async fn verify_zk_login_signature(
        &self,
        ctx: &Context<'_>,
        bytes: Base64,
        signature: Base64,
        intent_scope: ZkLoginIntentScope,
        author: MySoAddress,
    ) -> Result<ZkLoginVerifyResult, RpcError<zklogin::Error>> {
        zklogin::verify_signature(
            ctx,
            self.scope(ctx)?,
            bytes,
            signature,
            intent_scope,
            author,
        )
        .await
    }
}

impl Query {
    /// The scope under which all queries are supposed to be queried.
    fn scope<E: std::error::Error>(&self, ctx: &Context<'_>) -> Result<Scope, RpcError<E>> {
        self.scope.clone().map_or_else(|| Scope::new(ctx), Ok)
    }
}
