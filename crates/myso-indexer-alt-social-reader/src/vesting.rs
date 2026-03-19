// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Text};
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::schema::vesting_wallets;

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;
use crate::social_graph::{ProfileSummaryRow, get_profile_summaries_for_addresses};

#[derive(Debug, Clone)]
pub struct VestingWalletRow {
    pub wallet_id: String,
    pub owner_address: String,
    pub total_amount: i64,
    pub start_time: i64,
    pub duration: i64,
    pub curve_factor: i64,
    pub claimed_amount: i64,
    pub remaining_balance: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Debug, Clone)]
pub struct VestingWalletWithStatus {
    pub claimed_percentage: f64,
    pub is_fully_claimed: bool,
    pub has_started: bool,
    pub has_ended: bool,
    pub vesting_progress: f64,
    pub end_time: i64,
    pub wallet: VestingWalletRow,
}

impl VestingWalletWithStatus {
    fn from_wallet(wallet: VestingWalletRow, current_time_ms: u64) -> Self {
        let claimed_percentage = if wallet.total_amount == 0 {
            0.0
        } else {
            (wallet.claimed_amount as f64 / wallet.total_amount as f64) * 100.0
        };
        let end_time = wallet.start_time + wallet.duration;
        let has_started = wallet.start_time <= (current_time_ms as i64);
        let has_ended = (current_time_ms as i64) >= end_time;
        let vesting_progress = {
            let current_time = current_time_ms as i64;
            if current_time <= wallet.start_time {
                0.0
            } else if current_time >= end_time {
                1.0
            } else {
                let elapsed = current_time - wallet.start_time;
                elapsed as f64 / wallet.duration as f64
            }
        };
        Self {
            claimed_percentage,
            is_fully_claimed: wallet.remaining_balance == 0,
            has_started,
            has_ended,
            vesting_progress,
            end_time,
            wallet,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VestingLeaderboardEntry {
    pub owner_address: String,
    pub total_vested: i64,
    pub total_claimed: i64,
    pub active_wallets: i64,
    pub completed_wallets: i64,
    pub user: ProfileSummaryRow,
}

#[derive(Debug, Clone)]
pub struct VestingLeaderboardResponse {
    pub entries: Vec<VestingLeaderboardEntry>,
    pub total: i64,
}

fn vesting_wallet_row_from_tuple(
    wallet_id: String,
    owner_address: String,
    total_amount: i64,
    start_time: i64,
    duration: i64,
    curve_factor: i64,
    claimed_amount: i64,
    remaining_balance: i64,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    transaction_id: String,
) -> VestingWalletRow {
    VestingWalletRow {
        wallet_id,
        owner_address,
        total_amount,
        start_time,
        duration,
        curve_factor,
        claimed_amount,
        remaining_balance,
        created_at,
        updated_at,
        transaction_id,
    }
}

pub(crate) async fn get_vesting_wallet(
    conn: &mut Connection<'_>,
    wallet_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<VestingWalletWithStatus>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let result = vesting_wallets::table
        .filter(vesting_wallets::wallet_id.eq(wallet_id))
        .select((
            vesting_wallets::wallet_id,
            vesting_wallets::owner_address,
            vesting_wallets::total_amount,
            vesting_wallets::start_time,
            vesting_wallets::duration,
            vesting_wallets::curve_factor,
            vesting_wallets::claimed_amount,
            vesting_wallets::remaining_balance,
            vesting_wallets::created_at,
            vesting_wallets::updated_at,
            vesting_wallets::transaction_id,
        ))
        .first::<(
            String,
            String,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            NaiveDateTime,
            NaiveDateTime,
            String,
        )>(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    let current_time_ms = chrono::Utc::now().timestamp_millis() as u64;
    Ok(result.map(|r| {
        VestingWalletWithStatus::from_wallet(
            vesting_wallet_row_from_tuple(r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7, r.8, r.9, r.10),
            current_time_ms,
        )
    }))
}

pub(crate) async fn list_vesting_wallets(
    conn: &mut Connection<'_>,
    owner: Option<&str>,
    active_only: bool,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<VestingWalletWithStatus>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let current_time_ms = chrono::Utc::now().timestamp_millis();

    #[derive(QueryableByName)]
    struct WalletRow {
        #[diesel(sql_type = diesel::sql_types::Text)]
        wallet_id: String,
        #[diesel(sql_type = diesel::sql_types::Text)]
        owner_address: String,
        #[diesel(sql_type = BigInt)]
        total_amount: i64,
        #[diesel(sql_type = BigInt)]
        start_time: i64,
        #[diesel(sql_type = BigInt)]
        duration: i64,
        #[diesel(sql_type = BigInt)]
        curve_factor: i64,
        #[diesel(sql_type = BigInt)]
        claimed_amount: i64,
        #[diesel(sql_type = BigInt)]
        remaining_balance: i64,
        #[diesel(sql_type = diesel::sql_types::Timestamp)]
        created_at: NaiveDateTime,
        #[diesel(sql_type = diesel::sql_types::Timestamp)]
        updated_at: NaiveDateTime,
        #[diesel(sql_type = diesel::sql_types::Text)]
        transaction_id: String,
    }

    let wallets: Vec<VestingWalletRow> = if active_only || owner.is_some() {
        let (data_sql, bind_owner) = if owner.is_some() {
            if active_only {
                (
                    "SELECT wallet_id, owner_address, total_amount, start_time, duration, curve_factor, \
                     claimed_amount, remaining_balance, created_at, updated_at, transaction_id \
                     FROM vesting_wallets \
                     WHERE owner_address = $1 AND start_time <= $2 AND remaining_balance > 0 \
                     AND (start_time + duration) > $2 \
                     ORDER BY created_at DESC LIMIT $3 OFFSET $4",
                    true,
                )
            } else {
                (
                    "SELECT wallet_id, owner_address, total_amount, start_time, duration, curve_factor, \
                     claimed_amount, remaining_balance, created_at, updated_at, transaction_id \
                     FROM vesting_wallets WHERE owner_address = $1 \
                     ORDER BY created_at DESC LIMIT $2 OFFSET $3",
                    true,
                )
            }
        } else {
            (
                "SELECT wallet_id, owner_address, total_amount, start_time, duration, curve_factor, \
                 claimed_amount, remaining_balance, created_at, updated_at, transaction_id \
                 FROM vesting_wallets \
                 WHERE start_time <= $1 AND remaining_balance > 0 AND (start_time + duration) > $1 \
                 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
                false,
            )
        };

        let rows: Vec<WalletRow> = if bind_owner {
            let o = owner.unwrap();
            if active_only {
                diesel::sql_query(data_sql)
                    .bind::<diesel::sql_types::Text, _>(o)
                    .bind::<BigInt, _>(current_time_ms)
                    .bind::<BigInt, _>(limit)
                    .bind::<BigInt, _>(offset)
                    .load::<WalletRow>(conn)
                    .await?
            } else {
                diesel::sql_query(data_sql)
                    .bind::<diesel::sql_types::Text, _>(o)
                    .bind::<BigInt, _>(limit)
                    .bind::<BigInt, _>(offset)
                    .load::<WalletRow>(conn)
                    .await?
            }
        } else {
            diesel::sql_query(data_sql)
                .bind::<BigInt, _>(current_time_ms)
                .bind::<BigInt, _>(limit)
                .bind::<BigInt, _>(offset)
                .load::<WalletRow>(conn)
                .await?
        };

        rows.into_iter()
            .map(|r| {
                vesting_wallet_row_from_tuple(
                    r.wallet_id,
                    r.owner_address,
                    r.total_amount,
                    r.start_time,
                    r.duration,
                    r.curve_factor,
                    r.claimed_amount,
                    r.remaining_balance,
                    r.created_at,
                    r.updated_at,
                    r.transaction_id,
                )
            })
            .collect()
    } else {
        vesting_wallets::table
            .order_by(vesting_wallets::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                vesting_wallets::wallet_id,
                vesting_wallets::owner_address,
                vesting_wallets::total_amount,
                vesting_wallets::start_time,
                vesting_wallets::duration,
                vesting_wallets::curve_factor,
                vesting_wallets::claimed_amount,
                vesting_wallets::remaining_balance,
                vesting_wallets::created_at,
                vesting_wallets::updated_at,
                vesting_wallets::transaction_id,
            ))
            .load::<(
                String,
                String,
                i64,
                i64,
                i64,
                i64,
                i64,
                i64,
                NaiveDateTime,
                NaiveDateTime,
                String,
            )>(conn)
            .await?
            .into_iter()
            .map(
                |(
                    wallet_id,
                    owner_address,
                    total_amount,
                    start_time,
                    duration,
                    curve_factor,
                    claimed_amount,
                    remaining_balance,
                    created_at,
                    updated_at,
                    transaction_id,
                )| {
                    vesting_wallet_row_from_tuple(
                        wallet_id,
                        owner_address,
                        total_amount,
                        start_time,
                        duration,
                        curve_factor,
                        claimed_amount,
                        remaining_balance,
                        created_at,
                        updated_at,
                        transaction_id,
                    )
                },
            )
            .collect()
    };

    let current_time_ms_u64 = current_time_ms as u64;
    let results: Vec<VestingWalletWithStatus> = wallets
        .into_iter()
        .map(|w| VestingWalletWithStatus::from_wallet(w, current_time_ms_u64))
        .collect();

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_vesting_leaderboard(
    conn: &mut Connection<'_>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<VestingLeaderboardResponse> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let current_time_ms = chrono::Utc::now().timestamp_millis();

    #[derive(QueryableByName)]
    struct TotalRow {
        #[diesel(sql_type = BigInt)]
        count: i64,
    }
    let total = diesel::sql_query(
        "SELECT COUNT(DISTINCT owner_address)::bigint as count FROM vesting_wallets",
    )
    .get_result::<TotalRow>(conn)
    .await
    .map(|r| r.count)?;

    #[derive(QueryableByName)]
    struct LeaderboardRow {
        #[diesel(sql_type = Text)]
        owner_address: String,
        #[diesel(sql_type = BigInt)]
        total_vested: i64,
        #[diesel(sql_type = BigInt)]
        total_claimed: i64,
        #[diesel(sql_type = BigInt)]
        active_wallets: i64,
        #[diesel(sql_type = BigInt)]
        completed_wallets: i64,
    }

    let query = r#"
        SELECT
            owner_address,
            SUM(total_amount)::bigint as total_vested,
            SUM(claimed_amount)::bigint as total_claimed,
            SUM(CASE WHEN start_time <= $1 AND remaining_balance > 0 AND (start_time + duration) > $1 THEN 1 ELSE 0 END)::bigint as active_wallets,
            SUM(CASE WHEN (start_time + duration) <= $1 THEN 1 ELSE 0 END)::bigint as completed_wallets
        FROM vesting_wallets
        GROUP BY owner_address
        ORDER BY total_vested DESC
        LIMIT $2 OFFSET $3
    "#;

    let rows: Vec<LeaderboardRow> = diesel::sql_query(query)
        .bind::<BigInt, _>(current_time_ms)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<LeaderboardRow>(conn)
        .await?;

    let owner_addresses: Vec<String> = rows.iter().map(|r| r.owner_address.clone()).collect();
    let user_map = get_profile_summaries_for_addresses(conn, &owner_addresses, metrics).await?;

    let entries: Vec<VestingLeaderboardEntry> = rows
        .into_iter()
        .map(|r| {
            let user =
                user_map
                    .get(&r.owner_address)
                    .cloned()
                    .unwrap_or_else(|| ProfileSummaryRow {
                        owner_address: r.owner_address.clone(),
                        username: None,
                        display_name: None,
                        profile_photo: None,
                        bio: None,
                        selected_badge_id: None,
                        social_proof_token_address: None,
                        reservation_pool_address: None,
                        followers_count: None,
                        following_count: None,
                        post_count: None,
                        blocked_count: None,
                        is_following: None,
                        follows_viewer: None,
                    });
            VestingLeaderboardEntry {
                owner_address: r.owner_address,
                total_vested: r.total_vested,
                total_claimed: r.total_claimed,
                active_wallets: r.active_wallets,
                completed_wallets: r.completed_wallets,
                user,
            }
        })
        .collect();

    metrics.requests_succeeded.inc();
    Ok(VestingLeaderboardResponse { entries, total })
}
