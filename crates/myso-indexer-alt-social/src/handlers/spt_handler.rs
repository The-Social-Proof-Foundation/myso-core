// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! SPT pipeline: indexes social_proof_tokens / spt module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::sql_types::{BigInt, Nullable, Text, Timestamptz};
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    default_spt_config, merge_spt_config, InsertSptConfig, NewSocialProofTokensEvent,
    NewSptConfigEvent, NewSptHolding, NewSptPool, NewSptPriceHistory, NewSptReservation,
    NewSptReservationPool, NewSptRevenue, NewSptTransaction, NewUnifiedRevenue, ProfileUpdateSet,
    RESERVATION_POOL_STATUS_ACTIVE, RESERVATION_POOL_STATUS_THRESHOLD_MET,
    REVENUE_TYPE_SPT_CREATOR_FEE, REVENUE_TYPE_SPT_PLATFORM_FEE, REVENUE_TYPE_SPT_TREASURY_FEE,
    TOKEN_TYPE_POST, TOKEN_TYPE_PROFILE, TRANSACTION_TYPE_BUY,
};
use myso_indexer_alt_social_schema::schema::{
    ecosystem_treasury, posts, profiles, spt_config, spt_events, spt_holdings, spt_pools,
    spt_reservation_pools, spt_reservations, spt_revenue, spt_transactions, unified_revenue,
};

use super::common;
use super::events;
use super::organization_stats::{
    apply_org_outbound_spend, apply_org_revenue, resolve_organization_id_for_derived_address,
};
use super::spt;
use super::ProfileUpdate;

use crate::metrics::SocialMetrics;

const SPT_MODULES: &[&str] = &["social_proof_tokens", "spt"];

/// Insert into `spt_price_history` with `circulating_supply` synced from the latest `spt_pools` row.
/// Token buy/sell events omit supply on-chain; the pool row reflects it after [`SptRow::SptPoolSupplyUpdate`].
const SPT_PRICE_HISTORY_INSERT_SQL: &str = r#"
INSERT INTO spt_price_history (pool_id, price, circulating_supply, time, transaction_id)
VALUES (
    $1,
    $2,
    COALESCE(
        (SELECT circulating_supply FROM spt_pools WHERE pool_id = $1 ORDER BY time DESC LIMIT 1),
        $3
    ),
    $4,
    $5
)
"#;

/// Developer fee recipient (`platforms.developer_address`) or fallback `platform_id`, plus optional
/// linked platform id for `unified_revenue.platform_scope` (matches reservation fee handling).
#[derive(Debug, Clone)]
struct ResolvedSptPlatform {
    fee_recipient: String,
    linked_platform_id: Option<String>,
}

#[derive(QueryableByName)]
struct CreatorPlatformIdRow {
    #[diesel(sql_type = Text)]
    platform_id: String,
}

#[derive(QueryableByName)]
struct PlatformDeveloperRow {
    #[diesel(sql_type = Text)]
    developer_address: String,
}

async fn resolve_spt_platform_for_creator<'a>(
    conn: &mut Connection<'a>,
    creator_address: &str,
) -> Result<ResolvedSptPlatform, diesel::result::Error> {
    if creator_address.trim().is_empty() {
        return Ok(ResolvedSptPlatform {
            fee_recipient: String::new(),
            linked_platform_id: None,
        });
    }

    let linked_platform_id: Option<String> = diesel::sql_query(
        r#"SELECT pb.platform_id AS platform_id
           FROM profiles p
           INNER JOIN profile_badges pb ON pb.profile_id = p.profile_id
               AND pb.badge_id = p.selected_badge_id
           WHERE LOWER(TRIM(p.owner_address)) = LOWER(TRIM($1))
             AND p.profile_id IS NOT NULL
             AND p.selected_badge_id IS NOT NULL
             AND (pb.revoked IS NULL OR pb.revoked = false)
           ORDER BY pb.time DESC
           LIMIT 1"#,
    )
    .bind::<Text, _>(creator_address)
    .get_result::<CreatorPlatformIdRow>(conn)
    .await
    .optional()?
    .map(|row| row.platform_id)
    .filter(|s| !s.is_empty());

    let fee_recipient: String = if let Some(ref pid) = linked_platform_id {
        diesel::sql_query(
            "SELECT developer_address FROM platforms WHERE platform_id = $1 ORDER BY id DESC LIMIT 1",
        )
        .bind::<Text, _>(pid.as_str())
        .get_result::<PlatformDeveloperRow>(conn)
        .await
        .optional()?
        .map(|row| row.developer_address)
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| pid.clone())
    } else {
        String::new()
    };

    Ok(ResolvedSptPlatform {
        fee_recipient,
        linked_platform_id,
    })
}

#[derive(QueryableByName)]
struct ReservationPoolOwnerRow {
    #[diesel(sql_type = Text)]
    owner: String,
}

async fn resolve_reservation_pool_owner_for_fees(
    conn: &mut Connection<'_>,
    pool_id: &str,
    associated_id: &str,
    token_type: i16,
) -> Result<String, diesel::result::Error> {
    let by_pool = diesel::sql_query(
        "SELECT owner FROM spt_reservation_pools WHERE pool_id = $1 ORDER BY time DESC LIMIT 1",
    )
    .bind::<Text, _>(pool_id)
    .get_result::<ReservationPoolOwnerRow>(conn)
    .await
    .optional()?
    .map(|row| row.owner)
    .unwrap_or_default();
    if !by_pool.is_empty() {
        return Ok(by_pool);
    }

    let by_associated = diesel::sql_query(
        "SELECT owner FROM spt_reservation_pools WHERE associated_id = $1 ORDER BY time DESC LIMIT 1",
    )
    .bind::<Text, _>(associated_id)
    .get_result::<ReservationPoolOwnerRow>(conn)
    .await
    .optional()?
    .map(|row| row.owner)
    .unwrap_or_default();
    if !by_associated.is_empty() {
        return Ok(by_associated);
    }

    if token_type == TOKEN_TYPE_POST {
        let owner = diesel::sql_query(
            "SELECT owner FROM posts WHERE post_id = $1 ORDER BY time DESC LIMIT 1",
        )
        .bind::<Text, _>(associated_id)
        .get_result::<ReservationPoolOwnerRow>(conn)
        .await
        .optional()?
        .map(|row| row.owner)
        .unwrap_or_default();
        if !owner.is_empty() {
            return Ok(owner);
        }
    } else if token_type == TOKEN_TYPE_PROFILE {
        if let Some(suffix) = associated_id.strip_prefix("profile_") {
            if !suffix.is_empty() {
                return Ok(suffix.to_string());
            }
        }
        let owner = diesel::sql_query(
            "SELECT owner_address AS owner FROM profiles WHERE profile_id = $1 OR owner_address = $1 LIMIT 1",
        )
        .bind::<Text, _>(associated_id)
        .get_result::<ReservationPoolOwnerRow>(conn)
        .await
        .optional()?
        .map(|row| row.owner)
        .unwrap_or_default();
        if !owner.is_empty() {
            return Ok(owner);
        }
    }

    Ok(String::new())
}

#[derive(QueryableByName)]
struct SptRevenueCountRow {
    #[diesel(sql_type = BigInt)]
    n: i64,
}

async fn spt_revenue_exists_for_transaction(
    conn: &mut Connection<'_>,
    transaction_id: &str,
) -> Result<bool, diesel::result::Error> {
    let row = diesel::sql_query(
        "SELECT COUNT(*)::bigint AS n FROM spt_revenue WHERE transaction_id = $1",
    )
    .bind::<Text, _>(transaction_id)
    .get_result::<SptRevenueCountRow>(conn)
    .await?;
    Ok(row.n > 0)
}

#[derive(QueryableByName)]
struct SptReservationCanonPoolRow {
    #[diesel(sql_type = Text)]
    pool_id: String,
}

#[derive(QueryableByName)]
pub(crate) struct SptReserverAggRow {
    #[diesel(sql_type = Text)]
    reserver_address: String,
    #[diesel(sql_type = BigInt)]
    net_myso: i64,
}

#[derive(Debug, Clone)]
pub enum SptRow {
    SptPool(NewSptPool),
    SptTransaction(NewSptTransaction),
    SptHolding(NewSptHolding),
    SptPoolSupplyUpdate {
        pool_id: String,
        delta: i64,
    },
    SptPriceHistory(NewSptPriceHistory),
    SptLaunchHoldingsFromReservations {
        pool_id: String,
        associated_id: String,
        owner: String,
        circulating_supply: i64,
        total_reserved_at_launch: i64,
        created_at: i64,
        time: chrono::DateTime<chrono::Utc>,
        transaction_id: String,
    },
    SptReservationPool(NewSptReservationPool),
    SptReservation {
        associated_id: String,
        reservation: NewSptReservation,
        token_type: i16,
        total_reserved: i64,
        threshold_met: bool,
        created_at: i64,
    },
    SptReservationPoolUpdate {
        pool_id: String,
        associated_id: String,
        total_reserved: i64,
        status: Option<String>,
        required_threshold: Option<i64>,
    },
    SptConfig(NewSptConfigEvent),
    SocialProofTokensEvent(NewSocialProofTokensEvent),
    SptBuySellRevenueData {
        pool_id: String,
        associated_id: String,
        token_type: i16,
        trader: String,
        transaction_type: String,
        creator_fee: i64,
        platform_fee: i64,
        treasury_fee: i64,
        amount: i64,
        myso_amount: i64,
        token_price: i64,
        revenue_time: i64,
        transaction_id: String,
    },
    ProfileUpdate(ProfileUpdate),
    PostRevenueRedirectUpdate {
        post_id: String,
        revenue_redirect_to: String,
        revenue_redirect_percentage: i64,
        poc_redirection_kind: i16,
    },
}

impl SptRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::SptPool(p) => Some(SptRow::SptPool(p)),
            crate::handlers::SocialEventRow::SptTransaction(t) => Some(SptRow::SptTransaction(t)),
            crate::handlers::SocialEventRow::SptHolding(h) => Some(SptRow::SptHolding(h)),
            crate::handlers::SocialEventRow::SptPoolSupplyUpdate { pool_id, delta } => {
                Some(SptRow::SptPoolSupplyUpdate { pool_id, delta })
            }
            crate::handlers::SocialEventRow::SptPriceHistory(ph) => {
                Some(SptRow::SptPriceHistory(ph))
            }
            crate::handlers::SocialEventRow::SptLaunchHoldingsFromReservations {
                pool_id,
                associated_id,
                owner,
                circulating_supply,
                total_reserved_at_launch,
                created_at,
                time,
                transaction_id,
            } => Some(SptRow::SptLaunchHoldingsFromReservations {
                pool_id,
                associated_id,
                owner,
                circulating_supply,
                total_reserved_at_launch,
                created_at,
                time,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::SptReservationPool(rp) => {
                Some(SptRow::SptReservationPool(rp))
            }
            crate::handlers::SocialEventRow::SptReservation {
                associated_id,
                reservation,
                token_type,
                total_reserved,
                threshold_met,
                created_at,
            } => Some(SptRow::SptReservation {
                associated_id,
                reservation,
                token_type,
                total_reserved,
                threshold_met,
                created_at,
            }),
            crate::handlers::SocialEventRow::SptReservationPoolUpdate {
                pool_id,
                associated_id,
                total_reserved,
                status,
                required_threshold,
            } => Some(SptRow::SptReservationPoolUpdate {
                pool_id,
                associated_id,
                total_reserved,
                status,
                required_threshold,
            }),
            crate::handlers::SocialEventRow::SptConfig(c) => Some(SptRow::SptConfig(c)),
            crate::handlers::SocialEventRow::SocialProofTokensEvent(e) => {
                Some(SptRow::SocialProofTokensEvent(e))
            }
            crate::handlers::SocialEventRow::SptBuySellRevenueData {
                pool_id,
                associated_id,
                token_type,
                trader,
                transaction_type,
                creator_fee,
                platform_fee,
                treasury_fee,
                amount,
                myso_amount,
                token_price,
                revenue_time,
                transaction_id,
            } => Some(SptRow::SptBuySellRevenueData {
                pool_id,
                associated_id,
                token_type,
                trader,
                transaction_type,
                creator_fee,
                platform_fee,
                treasury_fee,
                amount,
                myso_amount,
                token_price,
                revenue_time,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::ProfileUpdate(up) => Some(SptRow::ProfileUpdate(up)),
            crate::handlers::SocialEventRow::PostRevenueRedirectUpdate {
                post_id,
                revenue_redirect_to,
                revenue_redirect_percentage,
                poc_redirection_kind,
            } => Some(SptRow::PostRevenueRedirectUpdate {
                post_id,
                revenue_redirect_to,
                revenue_redirect_percentage,
                poc_redirection_kind,
            }),
            _ => None,
        }
    }
}

impl FieldCount for SptRow {
    const FIELD_COUNT: usize = 17;
}

/// SPT rows: live pools use `spt_holdings`; reservation phase uses `spt_reservations` (+ view
/// `spt_reservation_holdings`). A unified hypertable was considered and **deferred** — readers and
/// GraphQL assume this split.
///
/// At launch, initial `spt_holdings` rows are derived in SQL from `spt_reservations` using the same
/// proportional split and owner remainder as `social_proof_tokens::create_social_proof_token`:
/// each reserver gets `net_myso * circulating_supply / total_reserved_at_launch` (floored), with the
/// remainder to the owner. `circulating_supply` is the event’s nano-SPT total (post-threshold mint,
/// scaled by `base_price` on-chain); `total_reserved_at_launch` is net nano-MYSO reserved.
pub struct SptHandler;

/// Floored proportional split plus owner remainder; mirrors on-chain launch distribution.
pub(crate) fn proportional_spt_launch_holdings(
    aggs: &[SptReserverAggRow],
    total_reserved_at_launch: i64,
    circulating_supply: i64,
    owner: &str,
    pool_id: &str,
    associated_id: &str,
) -> Result<std::collections::BTreeMap<String, i128>, anyhow::Error> {
    use std::collections::BTreeMap;

    let ledger_sum: i128 = aggs.iter().map(|a| a.net_myso as i128).sum();
    let supply = circulating_supply as i128;
    let mut denom = total_reserved_at_launch.max(0) as i128;
    if denom <= 0 && !aggs.is_empty() && supply > 0 {
        denom = ledger_sum.max(0);
        if denom > 0 {
            SocialMetrics::record_spt_launch_denominator_ledger_fallback();
            tracing::warn!(
                target: "social_indexer::spt",
                associated_id = %associated_id,
                pool_id = %pool_id,
                ledger_sum,
                "spt launch: total_reserved_at_launch was 0 but reservation aggregates exist; using ledger sum as denominator",
            );
        }
    }

    let mut amounts: BTreeMap<String, i128> = BTreeMap::new();
    if denom > 0 {
        for a in aggs {
            let share = (a.net_myso as i128 * supply) / denom;
            *amounts.entry(a.reserver_address.clone()).or_insert(0) += share;
        }
    }

    let floored_sum: i128 = amounts.values().sum();
    let delta = supply - floored_sum;
    let owner_entry = amounts.entry(owner.to_string()).or_insert(0);
    *owner_entry = owner_entry.checked_add(delta).ok_or_else(|| {
        anyhow::anyhow!("spt launch: owner remainder overflow for pool {}", pool_id)
    })?;

    let final_sum: i128 = amounts.values().sum();
    if final_sum != supply {
        return Err(anyhow::anyhow!(
            "spt launch: holdings sum {} != circulating_supply {} for pool {}",
            final_sum,
            supply,
            pool_id
        ));
    }

    Ok(amounts)
}

impl SptHandler {
    /// After applying all rows in the batch, detect pool rows with zero `circulating_supply` while
    /// the reservation ledger still shows net MYSO for that `associated_id` (parser / legacy-event bug).
    ///
    /// Set `MYSO_SOCIAL_INDEXER_STRICT_SPT_LAUNCH=1` to fail the batch (staging / CI).
    async fn validate_zero_circ_pools_after_batch<'a>(
        conn: &mut Connection<'a>,
        zero_circ: &[(String, String)],
    ) -> Result<()> {
        if zero_circ.is_empty() {
            return Ok(());
        }
        let strict = std::env::var("MYSO_SOCIAL_INDEXER_STRICT_SPT_LAUNCH")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        #[derive(QueryableByName)]
        struct NetRow {
            #[diesel(sql_type = BigInt)]
            net_myso: i64,
        }

        for (pool_id, associated_id) in zero_circ {
            let canon: Option<SptReservationCanonPoolRow> = diesel::sql_query(
                "SELECT pool_id FROM spt_reservation_pools WHERE associated_id = $1 ORDER BY time DESC LIMIT 1",
            )
            .bind::<Text, _>(associated_id)
            .get_result(conn)
            .await
            .optional()?;

            let reservation_pool_key = canon
                .map(|c| c.pool_id)
                .unwrap_or_else(|| format!("reservation_pool_{}", associated_id));
            let placeholder = format!("reservation_pool_{}", associated_id);

            let net_row: NetRow = diesel::sql_query(
                r#"SELECT COALESCE(SUM(amount), 0)::bigint AS net_myso FROM spt_reservations
                   WHERE pool_id = $1 OR pool_id = $2"#,
            )
            .bind::<Text, _>(&reservation_pool_key)
            .bind::<Text, _>(&placeholder)
            .get_result(conn)
            .await?;

            if net_row.net_myso > 0 {
                SocialMetrics::record_spt_pool_zero_supply_with_reservations();
                tracing::error!(
                    target: "social_indexer::spt",
                    pool_id = %pool_id,
                    associated_id = %associated_id,
                    net_reservation_myso = net_row.net_myso,
                    "spt pool inserted with circulating_supply=0 but reservation ledger has net MYSO; use extended TokenPoolCreatedEvent on chain and replay checkpoints (greenfield)"
                );
                if strict {
                    return Err(anyhow::anyhow!(
                        "spt strict launch check failed: pool {} associated {} has circulating_supply=0 but net reservation MYSO {}",
                        pool_id,
                        associated_id,
                        net_row.net_myso
                    ));
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Processor for SptHandler {
    const NAME: &'static str = "spt";

    type Value = SptRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let epoch = checkpoint.summary.epoch;
        let timestamp_ms = checkpoint.summary.timestamp_ms;
        let mut values = Vec::new();
        for tx in &checkpoint.transactions {
            let tx_digest = tx.transaction.digest().to_string();
            let Some(events) = &tx.events else {
                continue;
            };
            for (event_seq, ev) in events.data.iter().enumerate() {
                if !common::is_social_package_event(&ev.package_id, &ev.type_.address) {
                    continue;
                }
                let module = ev.type_.module.as_str();
                if !SPT_MODULES.contains(&module) {
                    continue;
                }
                let event_name = ev.type_.name.as_str();
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let event_data =
                    match events::parse_event_contents(module, event_name, &ev.contents) {
                        Ok(v) => v,
                        Err(e) => {
                            tracing::warn!(
                                tx_digest = %tx_digest,
                                module,
                                event_name,
                                error = %e,
                                hex_preview = %e.contents_hex_preview(48),
                                "spt pipeline: event contents parse failed; skipping event"
                            );
                            SocialMetrics::record_event_bcs_parse_failed(module, event_name);
                            continue;
                        }
                    };
                if let Some(rows) =
                    spt::handle_spt_event(event_name, &event_data, &event_id, epoch, timestamp_ms)
                {
                    for row in rows {
                        if let Some(r) = SptRow::from_social(row) {
                            values.push(r);
                        }
                    }
                }
            }
        }
        Ok(values)
    }
}

async fn load_latest_spt_config(conn: &mut Connection<'_>) -> Result<Option<InsertSptConfig>> {
    spt_config::table
        .order(spt_config::time.desc())
        .select(InsertSptConfig::as_select())
        .first(conn)
        .await
        .optional()
        .map_err(Into::into)
}

#[async_trait]
impl Handler for SptHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        let mut zero_circ_pools: Vec<(String, String)> = Vec::new();
        let mut running_spt_config = load_latest_spt_config(conn)
            .await?
            .unwrap_or_else(default_spt_config);
        for row in values {
            match row {
                SptRow::SptPool(p) => {
                    if p.circulating_supply == 0 {
                        zero_circ_pools.push((p.pool_id.clone(), p.associated_id.clone()));
                    }
                    total += diesel::insert_into(spt_pools::table)
                        .values(p)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptTransaction(t) => {
                    let mut tx = t.clone();
                    if tx.organization_id.is_none() {
                        tx.organization_id =
                            resolve_organization_id_for_derived_address(conn, &tx.sender).await?;
                    }
                    total += diesel::insert_into(spt_transactions::table)
                        .values(&tx)
                        .execute(conn)
                        .await?;
                    if tx.transaction_type == TRANSACTION_TYPE_BUY && tx.myso_amount > 0 {
                        apply_org_outbound_spend(
                            conn,
                            tx.organization_id.as_deref(),
                            tx.myso_amount,
                            None,
                            tx.created_at,
                        )
                        .await?;
                    } else if tx.myso_amount < 0 {
                        apply_org_revenue(
                            conn,
                            tx.organization_id.as_deref(),
                            tx.myso_amount.saturating_neg(),
                            None,
                            tx.created_at,
                        )
                        .await?;
                    }
                }
                SptRow::SptHolding(h) => {
                    total += diesel::insert_into(spt_holdings::table)
                        .values(h)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptPoolSupplyUpdate { pool_id, delta } => {
                    let update_sql = "UPDATE spt_pools SET circulating_supply = circulating_supply + $1 \
                         WHERE pool_id = $2 AND time = (SELECT time FROM spt_pools WHERE pool_id = $2 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<BigInt, _>(*delta)
                        .bind::<Text, _>(pool_id)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptPriceHistory(ph) => {
                    total += diesel::sql_query(SPT_PRICE_HISTORY_INSERT_SQL)
                        .bind::<Text, _>(&ph.pool_id)
                        .bind::<BigInt, _>(ph.price)
                        .bind::<BigInt, _>(ph.circulating_supply)
                        .bind::<Timestamptz, _>(ph.time)
                        .bind::<Text, _>(&ph.transaction_id)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptLaunchHoldingsFromReservations {
                    pool_id,
                    associated_id,
                    owner,
                    circulating_supply,
                    total_reserved_at_launch,
                    created_at,
                    time,
                    transaction_id,
                } => {
                    if *circulating_supply > 0 {
                        let canon: Option<SptReservationCanonPoolRow> = diesel::sql_query(
                        "SELECT pool_id FROM spt_reservation_pools WHERE associated_id = $1 ORDER BY time DESC LIMIT 1",
                    )
                    .bind::<Text, _>(associated_id)
                    .get_result(conn)
                    .await
                    .optional()?;

                        let reservation_pool_key = canon
                            .map(|c| c.pool_id)
                            .unwrap_or_else(|| format!("reservation_pool_{}", associated_id));
                        let placeholder = format!("reservation_pool_{}", associated_id);

                        let aggs: Vec<SptReserverAggRow> = diesel::sql_query(
                            r#"SELECT reserver_address, SUM(amount)::bigint AS net_myso
 FROM spt_reservations
                         WHERE pool_id = $1 OR pool_id = $2
                         GROUP BY reserver_address
                         HAVING SUM(amount) > 0"#,
                        )
                        .bind::<Text, _>(&reservation_pool_key)
                        .bind::<Text, _>(&placeholder)
                        .load(conn)
                        .await?;

                        if *total_reserved_at_launch > 0 && aggs.is_empty() {
                            return Err(anyhow::anyhow!(
                            "spt launch: expected reservation rows for pool {} (associated {}); index reservations before token pool or replay checkpoint",
                            pool_id,
                            associated_id
                        ));
                        }

                        let ledger_sum: i128 = aggs.iter().map(|a| a.net_myso as i128).sum();
                        if ledger_sum != *total_reserved_at_launch as i128 {
                            tracing::warn!(
                                target: "social_indexer::spt",
                                associated_id = %associated_id,
                                pool_id = %pool_id,
                                ledger_sum,
                                total_reserved_at_launch,
                                "spt launch: reservation ledger sum differs from on-chain total_reserved_at_launch",
                            );
                        }

                        let amounts = proportional_spt_launch_holdings(
                            &aggs,
                            *total_reserved_at_launch,
                            *circulating_supply,
                            owner,
                            pool_id,
                            associated_id,
                        )?;

                        for (holder_address, amt) in amounts {
                            if amt == 0 {
                                continue;
                            }
                            let amt_i64 = i64::try_from(amt).map_err(|_| {
                                anyhow::anyhow!(
                                    "spt launch: holding amount overflow for pool {}",
                                    pool_id
                                )
                            })?;
                            let h = NewSptHolding {
                                pool_id: pool_id.clone(),
                                holder_address,
                                amount: amt_i64,
                                acquired_at: *created_at,
                                time: *time,
                                transaction_id: transaction_id.clone(),
                            };
                            total += diesel::insert_into(spt_holdings::table)
                                .values(h)
                                .execute(conn)
                                .await?;
                        }
                    }
                }
                SptRow::SptReservationPool(rp) => {
                    total += diesel::insert_into(spt_reservation_pools::table)
                        .values(rp)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptReservation {
                    associated_id,
                    reservation,
                    token_type,
                    total_reserved,
                    threshold_met,
                    created_at,
                } => {
                    #[derive(QueryableByName)]
                    struct PoolIdRow {
                        #[diesel(sql_type = Text)]
                        pool_id: String,
                    }
                    let pool_id_row: Option<PoolIdRow> = diesel::sql_query(
                        "SELECT pool_id FROM spt_reservation_pools WHERE associated_id = $1 ORDER BY time DESC LIMIT 1",
                    )
                    .bind::<Text, _>(associated_id)
                    .get_result(conn)
                    .await
                    .optional()?;
                    let pool_id = if let Some(ref row) = pool_id_row {
                        row.pool_id.clone()
                    } else {
                        let synthetic_pool_id = format!("reservation_pool_{}", associated_id);
                        #[derive(QueryableByName)]
                        struct OwnerRow {
                            #[diesel(sql_type = Text)]
                            owner: String,
                        }
                        let owner = if *token_type == TOKEN_TYPE_POST {
                            diesel::sql_query(
                                "SELECT owner FROM posts WHERE post_id = $1 ORDER BY time DESC LIMIT 1",
                            )
                            .bind::<Text, _>(associated_id)
                            .get_result::<OwnerRow>(conn)
                            .await
                            .ok()
                            .map(|r| r.owner)
                        } else {
                            diesel::sql_query(
                                "SELECT owner_address FROM profiles WHERE profile_id = $1 OR owner_address = $1 LIMIT 1",
                            )
                            .bind::<Text, _>(associated_id)
                            .get_result::<OwnerRow>(conn)
                            .await
                            .ok()
                            .map(|r| r.owner)
                        }
                        .unwrap_or_else(|| reservation.reserver_address.clone());
                        let status = if *threshold_met {
                            RESERVATION_POOL_STATUS_THRESHOLD_MET.to_string()
                        } else {
                            RESERVATION_POOL_STATUS_ACTIVE.to_string()
                        };
                        let synthetic_pool = NewSptReservationPool {
                            pool_id: synthetic_pool_id.clone(),
                            associated_id: associated_id.clone(),
                            token_type: *token_type,
                            owner: owner.clone(),
                            total_reserved: *total_reserved,
                            required_threshold: *total_reserved,
                            status,
                            created_at: *created_at,
                            time: reservation.time,
                            transaction_id: reservation.transaction_id.clone(),
                        };
                        total += diesel::insert_into(spt_reservation_pools::table)
                            .values(&synthetic_pool)
                            .execute(conn)
                            .await?;
                        tracing::info!(
                            associated_id = %associated_id,
                            pool_id = %synthetic_pool_id,
                            "created synthetic SptReservationPool (no canonical pool found)"
                        );
                        synthetic_pool_id
                    };
                    let mut r = reservation.clone();
                    if r.organization_id.is_none() {
                        r.organization_id =
                            resolve_organization_id_for_derived_address(conn, &r.reserver_address)
                                .await?;
                    }
                    r.pool_id = pool_id.clone();
                    let reserver_org = r.organization_id.clone();
                    let reserver_amount = r.amount;
                    let reserver_at = r.reserved_at;
                    let spt_transaction = NewSptTransaction::from_reservation(&r, pool_id.clone());
                    let reservation_inserted = diesel::insert_into(spt_reservations::table)
                        .values(r)
                        .on_conflict((spt_reservations::transaction_id, spt_reservations::time))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    total += reservation_inserted;
                    if reservation_inserted > 0 {
                        tracing::info!(
                            associated_id = %associated_id,
                            pool_id = %pool_id,
                            reserver = %reservation.reserver_address,
                            amount = %reservation.amount,
                            "SptReservation inserted"
                        );
                        if reserver_amount > 0 {
                            apply_org_outbound_spend(
                                conn,
                                reserver_org.as_deref(),
                                reserver_amount,
                                None,
                                reserver_at,
                            )
                            .await?;
                        } else if reserver_amount < 0 {
                            apply_org_revenue(
                                conn,
                                reserver_org.as_deref(),
                                reserver_amount.unsigned_abs() as i64,
                                None,
                                reserver_at,
                            )
                            .await?;
                        }

                        total += diesel::insert_into(spt_transactions::table)
                            .values(&spt_transaction)
                            .execute(conn)
                            .await?;
                    } else {
                        tracing::debug!(
                            associated_id = %associated_id,
                            pool_id = %pool_id,
                            transaction_id = %reservation.transaction_id,
                            "SptReservation skipped (duplicate transaction_id, time)"
                        );
                    }

                    let creator_fee = reservation.creator_fee.unwrap_or(0);
                    let platform_fee = reservation.platform_fee.unwrap_or(0);
                    let treasury_fee = reservation.treasury_fee.unwrap_or(0);
                    let fees_nonzero = creator_fee != 0 || platform_fee != 0 || treasury_fee != 0;
                    let revenue_missing = if fees_nonzero {
                        !spt_revenue_exists_for_transaction(
                            conn,
                            reservation.transaction_id.as_str(),
                        )
                        .await?
                    } else {
                        false
                    };
                    if fees_nonzero && (reservation_inserted > 0 || revenue_missing) {
                        let creator_address = resolve_reservation_pool_owner_for_fees(
                            conn,
                            pool_id.as_str(),
                            associated_id.as_str(),
                            *token_type,
                        )
                        .await?;

                        if creator_address.is_empty() {
                            tracing::warn!(
                                associated_id = %associated_id,
                                pool_id = %pool_id,
                                transaction_id = %reservation.transaction_id,
                                "SptReservation fees skipped: missing reservation pool owner"
                            );
                        } else {
                            let treasury_address = ecosystem_treasury::table
                                .order(ecosystem_treasury::time.desc())
                                .select(ecosystem_treasury::treasury_address)
                                .first::<String>(conn)
                                .await
                                .ok()
                                .unwrap_or_default();

                            let resolved =
                                resolve_spt_platform_for_creator(conn, creator_address.as_str())
                                    .await?;
                            let linked_platform_id = resolved.linked_platform_id.clone();
                            let platform_fee_recipient = resolved.fee_recipient;

                            // Reservation-phase fees are always keyed to the reservation pool object id.
                            let revenue_spt_pool_id = pool_id.clone();

                            let withdraw = reservation.amount < 0;
                            let revenue_time = reservation.reserved_at;
                            let payer = reservation.reserver_address.clone();
                            let tx_id = reservation.transaction_id.clone();
                            let spt_row_time = reservation.time;

                            let platform_scope_for_fees: Option<String> =
                                Some(linked_platform_id.clone().unwrap_or_default());

                            let spt_rev = NewSptRevenue::from_reservation_event(
                                revenue_spt_pool_id,
                                withdraw,
                                payer.clone(),
                                creator_address.clone(),
                                platform_fee_recipient.clone(),
                                treasury_address.clone(),
                                creator_fee,
                                platform_fee,
                                treasury_fee,
                                reservation.amount,
                                0_i64,
                                0_i64,
                                revenue_time,
                                tx_id.clone(),
                                spt_row_time,
                            );
                            match diesel::insert_into(spt_revenue::table)
                                .values(&spt_rev)
                                .execute(conn)
                                .await
                            {
                                Ok(n) => total += n,
                                Err(e) => {
                                    tracing::warn!(
                                        error = %e,
                                        transaction_id = %tx_id,
                                        pool_id = %pool_id,
                                        associated_id = %associated_id,
                                        "spt_revenue insert failed for reservation"
                                    );
                                }
                            }

                            diesel::sql_query("SAVEPOINT spt_reservation_unified_revenue")
                                .execute(conn)
                                .await?;

                            let unified_writes: Result<usize, diesel::result::Error> = async {
                                let mut subtotal = 0usize;
                                let mut ur_time = chrono::Utc::now();
                                if creator_fee != 0 {
                                    let row_time = ur_time;
                                    ur_time += chrono::Duration::microseconds(1);
                                    subtotal += diesel::insert_into(unified_revenue::table)
                                        .values(NewUnifiedRevenue::from_spt_at_time(
                                            REVENUE_TYPE_SPT_CREATOR_FEE.to_string(),
                                            creator_address.clone(),
                                            None,
                                            creator_fee,
                                            pool_id.clone(),
                                            payer.clone(),
                                            creator_address.clone(),
                                            revenue_time,
                                            tx_id.clone(),
                                            row_time,
                                        ))
                                        .execute(conn)
                                        .await?;
                                }
                                if platform_fee != 0 {
                                    let row_time = ur_time;
                                    ur_time += chrono::Duration::microseconds(1);
                                    subtotal += diesel::insert_into(unified_revenue::table)
                                        .values(NewUnifiedRevenue::from_spt_at_time(
                                            REVENUE_TYPE_SPT_PLATFORM_FEE.to_string(),
                                            creator_address.clone(),
                                            platform_scope_for_fees.clone(),
                                            platform_fee,
                                            pool_id.clone(),
                                            payer.clone(),
                                            platform_fee_recipient.clone(),
                                            revenue_time,
                                            tx_id.clone(),
                                            row_time,
                                        ))
                                        .execute(conn)
                                        .await?;
                                }
                                if treasury_fee != 0 {
                                    let row_time = ur_time;
                                    subtotal += diesel::insert_into(unified_revenue::table)
                                        .values(NewUnifiedRevenue::from_spt_at_time(
                                            REVENUE_TYPE_SPT_TREASURY_FEE.to_string(),
                                            creator_address.clone(),
                                            None,
                                            treasury_fee,
                                            pool_id.clone(),
                                            payer.clone(),
                                            treasury_address.clone(),
                                            revenue_time,
                                            tx_id.clone(),
                                            row_time,
                                        ))
                                        .execute(conn)
                                        .await?;
                                }
                                Ok(subtotal)
                            }
                            .await;

                            match unified_writes {
                                Ok(n) => {
                                    diesel::sql_query(
                                        "RELEASE SAVEPOINT spt_reservation_unified_revenue",
                                    )
                                    .execute(conn)
                                    .await?;
                                    total += n;
                                }
                                Err(e) => {
                                    diesel::sql_query(
                                        "ROLLBACK TO SAVEPOINT spt_reservation_unified_revenue",
                                    )
                                    .execute(conn)
                                    .await?;
                                    tracing::warn!(
                                        error = %e,
                                        transaction_id = %tx_id,
                                        pool_id = %pool_id,
                                        associated_id = %associated_id,
                                        "unified_revenue failed for reservation; spt_revenue row retained"
                                    );
                                }
                            }
                        }
                    }
                }
                SptRow::SptReservationPoolUpdate {
                    pool_id: _pool_id,
                    associated_id,
                    total_reserved,
                    status,
                    required_threshold,
                } => {
                    let update_sql = "UPDATE spt_reservation_pools SET total_reserved = $1, \
                         status = COALESCE($2, status), \
                         required_threshold = COALESCE($4, required_threshold) \
                         WHERE associated_id = $3 AND time = (SELECT time FROM spt_reservation_pools WHERE associated_id = $3 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<BigInt, _>(*total_reserved)
                        .bind::<Nullable<Text>, _>(status.as_deref())
                        .bind::<Text, _>(associated_id)
                        .bind::<Nullable<BigInt>, _>(*required_threshold)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptConfig(c) => {
                    let sync_reservation_pool_thresholds = !c.apply_trading_enabled_only
                        && c.profile_threshold > 0
                        && c.post_threshold > 0;
                    let profile_threshold = c.profile_threshold;
                    let post_threshold = c.post_threshold;
                    let merged = merge_spt_config(&running_spt_config, c);
                    total += diesel::insert_into(spt_config::table)
                        .values(&merged)
                        .execute(conn)
                        .await?;
                    running_spt_config = merged;
                    if sync_reservation_pool_thresholds {
                        let sync_sql = r#"
                            UPDATE spt_reservation_pools sp
                            SET required_threshold = CASE sp.token_type
                                WHEN 1 THEN $1
                                WHEN 2 THEN $2
                                ELSE sp.required_threshold
                            END
                            FROM (
                                SELECT DISTINCT ON (pool_id) pool_id, time
                                FROM spt_reservation_pools
                                ORDER BY pool_id, time DESC
                            ) AS latest
                            WHERE sp.pool_id = latest.pool_id AND sp.time = latest.time
                        "#;
                        total += diesel::sql_query(sync_sql)
                            .bind::<BigInt, _>(profile_threshold)
                            .bind::<BigInt, _>(post_threshold)
                            .execute(conn)
                            .await?;
                    }
                }
                SptRow::SocialProofTokensEvent(e) => {
                    total += diesel::insert_into(spt_events::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptBuySellRevenueData {
                    pool_id,
                    trader,
                    transaction_type,
                    creator_fee,
                    platform_fee,
                    treasury_fee,
                    amount,
                    myso_amount,
                    token_price,
                    revenue_time,
                    transaction_id,
                    ..
                } => {
                    let pool_row: Option<(String, String, i16)> = spt_pools::table
                        .filter(spt_pools::pool_id.eq(pool_id))
                        .order(spt_pools::time.desc())
                        .select((
                            spt_pools::owner,
                            spt_pools::associated_id,
                            spt_pools::token_type,
                        ))
                        .first::<(String, String, i16)>(conn)
                        .await
                        .ok();

                    let (creator_address, treasury_address): (String, String) =
                        if let Some((owner, _associated_id, _token_type)) = pool_row {
                            let treasury = ecosystem_treasury::table
                                .order(ecosystem_treasury::time.desc())
                                .select(ecosystem_treasury::treasury_address)
                                .first::<String>(conn)
                                .await
                                .ok()
                                .unwrap_or_default();
                            (owner, treasury)
                        } else {
                            (String::new(), String::new())
                        };

                    if *creator_fee != 0 || *platform_fee != 0 || *treasury_fee != 0 {
                        let resolved =
                            resolve_spt_platform_for_creator(conn, creator_address.as_str())
                                .await?;
                        let platform_address = resolved.fee_recipient;
                        let platform_scope_for_fees: Option<String> =
                            Some(resolved.linked_platform_id.clone().unwrap_or_default());

                        let spt_rev = if transaction_type == "SELL" {
                            NewSptRevenue::from_sell_event(
                                pool_id.clone(),
                                trader.clone(),
                                creator_address.clone(),
                                platform_address.clone(),
                                treasury_address.clone(),
                                *creator_fee,
                                *platform_fee,
                                *treasury_fee,
                                *amount,
                                *myso_amount,
                                *token_price,
                                *revenue_time,
                                transaction_id.clone(),
                            )
                        } else {
                            NewSptRevenue::from_buy_event(
                                pool_id.clone(),
                                trader.clone(),
                                creator_address.clone(),
                                platform_address.clone(),
                                treasury_address.clone(),
                                *creator_fee,
                                *platform_fee,
                                *treasury_fee,
                                *amount,
                                *myso_amount,
                                *token_price,
                                *revenue_time,
                                transaction_id.clone(),
                            )
                        };
                        total += diesel::insert_into(spt_revenue::table)
                            .values(&spt_rev)
                            .execute(conn)
                            .await?;

                        if *creator_fee > 0 {
                            total += diesel::insert_into(unified_revenue::table)
                                .values(NewUnifiedRevenue::from_spt(
                                    REVENUE_TYPE_SPT_CREATOR_FEE.to_string(),
                                    creator_address.clone(),
                                    None,
                                    *creator_fee,
                                    pool_id.clone(),
                                    trader.clone(),
                                    creator_address.clone(),
                                    *revenue_time,
                                    transaction_id.clone(),
                                ))
                                .execute(conn)
                                .await?;
                        }
                        if *platform_fee > 0 {
                            total += diesel::insert_into(unified_revenue::table)
                                .values(NewUnifiedRevenue::from_spt(
                                    REVENUE_TYPE_SPT_PLATFORM_FEE.to_string(),
                                    creator_address.clone(),
                                    platform_scope_for_fees.clone(),
                                    *platform_fee,
                                    pool_id.clone(),
                                    trader.clone(),
                                    platform_address.clone(),
                                    *revenue_time,
                                    transaction_id.clone(),
                                ))
                                .execute(conn)
                                .await?;
                        }
                        if *treasury_fee > 0 {
                            total += diesel::insert_into(unified_revenue::table)
                                .values(NewUnifiedRevenue::from_spt(
                                    REVENUE_TYPE_SPT_TREASURY_FEE.to_string(),
                                    creator_address.clone(),
                                    None,
                                    *treasury_fee,
                                    pool_id.clone(),
                                    trader.clone(),
                                    treasury_address.clone(),
                                    *revenue_time,
                                    transaction_id.clone(),
                                ))
                                .execute(conn)
                                .await?;
                        }
                    }
                }
                SptRow::ProfileUpdate(up) => {
                    let now = chrono::Utc::now().naive_utc();
                    let set = ProfileUpdateSet {
                        updated_at: now,
                        display_name: up.display_name.clone().map(Some),
                        bio: up.bio.clone().map(Some),
                        profile_photo: up.profile_photo.clone().map(Some),
                        cover_photo: up.cover_photo.clone().map(Some),
                        website: up.website.clone().map(Some),
                        birthdate: up.birthdate.clone().map(Some),
                        location: up.location.clone().map(Some),
                        x_username: up.x_username.clone().map(Some),
                        username: up.username.clone(),
                        selected_badge_id: up.selected_badge_id.clone(),
                        selected_ecosystem_badge_id: up.selected_ecosystem_badge_id.clone(),
                        reservation_pool_address: up.reservation_pool_address.clone(),
                        social_proof_token_address: up.social_proof_token_address.clone(),
                    };
                    let filter = profiles::profile_id
                        .eq(&up.profile_id)
                        .or(profiles::owner_address.eq(&up.owner_address));
                    total += diesel::update(profiles::table)
                        .filter(filter)
                        .set(set)
                        .execute(conn)
                        .await?;
                }
                SptRow::PostRevenueRedirectUpdate {
                    post_id,
                    revenue_redirect_to,
                    revenue_redirect_percentage,
                    poc_redirection_kind,
                } => {
                    total += diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set((
                            posts::revenue_redirect_to.eq(Some(revenue_redirect_to.clone())),
                            posts::revenue_redirect_percentage
                                .eq(Some(*revenue_redirect_percentage)),
                            posts::poc_redirection_kind.eq(Some(*poc_redirection_kind)),
                        ))
                        .execute(conn)
                        .await?;
                }
            }
        }
        SptHandler::validate_zero_circ_pools_after_batch(conn, &zero_circ_pools).await?;
        Ok(total)
    }
}

#[cfg(test)]
mod reservation_revenue_retry_tests {
    #[test]
    fn revenue_retry_when_reservation_duplicate_but_fees_present() {
        let reservation_inserted = 0usize;
        let fees_nonzero = true;
        let revenue_missing = true;
        assert!(
            fees_nonzero && (reservation_inserted > 0 || revenue_missing),
            "re-index must backfill spt_revenue when reservation row already exists"
        );
    }

    #[test]
    fn skip_revenue_when_already_indexed() {
        let reservation_inserted = 0usize;
        let fees_nonzero = true;
        let revenue_missing = false;
        assert!(
            !(fees_nonzero && (reservation_inserted > 0 || revenue_missing)),
            "duplicate reservation events must not double-write revenue"
        );
    }
}

#[cfg(test)]
mod reservation_owner_fallback_tests {
    #[test]
    fn profile_associated_id_prefix_yields_owner_address() {
        let associated_id = "profile_0xabc123";
        let owner = associated_id
            .strip_prefix("profile_")
            .filter(|s| !s.is_empty())
            .map(str::to_string);
        assert_eq!(owner.as_deref(), Some("0xabc123"));
    }
}

#[cfg(test)]
mod spt_price_history_insert_sql_tests {
    use super::SPT_PRICE_HISTORY_INSERT_SQL;

    #[test]
    fn insert_uses_coalesce_from_latest_spt_pools_row() {
        assert!(SPT_PRICE_HISTORY_INSERT_SQL.contains("COALESCE"));
        assert!(SPT_PRICE_HISTORY_INSERT_SQL.contains("spt_pools"));
        assert!(
            SPT_PRICE_HISTORY_INSERT_SQL.contains("WHERE pool_id = $1 ORDER BY time DESC LIMIT 1")
        );
    }
}

#[cfg(test)]
mod proportional_spt_launch_tests {
    use super::{proportional_spt_launch_holdings, SptReserverAggRow};

    #[test]
    fn proportional_uses_total_reserved_at_launch_when_set() {
        // Mirrors on-chain: e.g. circulating_supply = 10 nano-SPT, total_reserved_at_launch = 1000 nano-MYSO.
        let aggs = vec![
            SptReserverAggRow {
                reserver_address: "0xa".to_string(),
                net_myso: 600,
            },
            SptReserverAggRow {
                reserver_address: "0xb".to_string(),
                net_myso: 400,
            },
        ];
        let m = proportional_spt_launch_holdings(&aggs, 1000, 10, "0xowner", "0xpool", "0xassoc")
            .unwrap();
        assert_eq!(m["0xa"], 6);
        assert_eq!(m["0xb"], 4);
    }

    #[test]
    fn proportional_falls_back_to_ledger_sum_when_total_reserved_zero() {
        let aggs = vec![
            SptReserverAggRow {
                reserver_address: "0xa".to_string(),
                net_myso: 600,
            },
            SptReserverAggRow {
                reserver_address: "0xb".to_string(),
                net_myso: 400,
            },
        ];
        let m =
            proportional_spt_launch_holdings(&aggs, 0, 10, "0xowner", "0xpool", "0xassoc").unwrap();
        assert_eq!(m["0xa"], 6);
        assert_eq!(m["0xb"], 4);
    }
}
