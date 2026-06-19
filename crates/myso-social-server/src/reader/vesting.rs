// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Text};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::parse_pieces;
use myso_indexer_alt_social_schema::schema::{vesting_events, vesting_wallets};

use crate::error::SocialError;
use crate::reader::social_graph::enrich_users_with_universal_data;
use crate::reader::types::{
    ClaimableResponse, PaginationInfo, UniversalUserResult, VestingAnalyticsResponse,
    VestingEventRow, VestingEventsResponse, VestingLeaderboardEntry, VestingLeaderboardResponse,
    VestingWalletRow, VestingWalletWithProfile, VestingWalletWithStatus, VestingWalletsResponse,
};
use myso_pg_db::Db;

fn vesting_wallet_row_from_tuple(
    wallet_id: String,
    owner_address: String,
    total_amount: i64,
    start_time: i64,
    schedule_end: i64,
    pieces_json: serde_json::Value,
    claimed_amount: i64,
    remaining_balance: i64,
    created_at: chrono::NaiveDateTime,
    updated_at: chrono::NaiveDateTime,
    transaction_id: String,
) -> VestingWalletRow {
    VestingWalletRow {
        wallet_id,
        owner_address,
        total_amount,
        start_time,
        schedule_end,
        pieces: parse_pieces(&pieces_json),
        claimed_amount,
        remaining_balance,
        created_at,
        updated_at,
        transaction_id,
    }
}

pub(crate) async fn list_vesting_wallets(
    db: &Db,
    active_only: bool,
    owner: Option<&str>,
    limit: i64,
    offset: i64,
    page: i64,
) -> Result<VestingWalletsResponse, SocialError> {
    let mut conn = db.connect().await?;
    let current_time_ms = chrono::Utc::now().timestamp_millis();

    let (total, results) = if active_only || owner.is_some() {
        #[derive(QueryableByName)]
        struct CountRow {
            #[diesel(sql_type = BigInt)]
            count: i64,
        }

        let (count_sql, data_sql) = if owner.is_some() {
            if active_only {
                (
                    "SELECT COUNT(*)::bigint as count FROM vesting_wallets \
                     WHERE owner_address = $1 AND start_time <= $2 AND remaining_balance > 0 \
                     AND schedule_end > $2",
                    "SELECT wallet_id, owner_address, total_amount, start_time, schedule_end, pieces, \
                     claimed_amount, remaining_balance, created_at, updated_at, transaction_id \
                     FROM vesting_wallets \
                     WHERE owner_address = $1 AND start_time <= $2 AND remaining_balance > 0 \
                     AND schedule_end > $2 \
                     ORDER BY created_at DESC LIMIT $3 OFFSET $4",
                )
            } else {
                (
                    "SELECT COUNT(*)::bigint as count FROM vesting_wallets WHERE owner_address = $1",
                    "SELECT wallet_id, owner_address, total_amount, start_time, schedule_end, pieces, \
                     claimed_amount, remaining_balance, created_at, updated_at, transaction_id \
                     FROM vesting_wallets WHERE owner_address = $1 \
                     ORDER BY created_at DESC LIMIT $2 OFFSET $3",
                )
            }
        } else {
            (
                "SELECT COUNT(*)::bigint as count FROM vesting_wallets \
                 WHERE start_time <= $1 AND remaining_balance > 0 AND schedule_end > $1",
                "SELECT wallet_id, owner_address, total_amount, start_time, schedule_end, pieces, \
                 claimed_amount, remaining_balance, created_at, updated_at, transaction_id \
                 FROM vesting_wallets \
                 WHERE start_time <= $1 AND remaining_balance > 0 AND schedule_end > $1 \
                 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            )
        };

        let total: i64 = if let Some(o) = owner {
            if active_only {
                diesel::sql_query(count_sql)
                    .bind::<diesel::sql_types::Text, _>(o)
                    .bind::<BigInt, _>(current_time_ms)
                    .get_result::<CountRow>(&mut conn)
                    .await?
            } else {
                diesel::sql_query(count_sql)
                    .bind::<diesel::sql_types::Text, _>(o)
                    .get_result::<CountRow>(&mut conn)
                    .await?
            }
        } else {
            diesel::sql_query(count_sql)
                .bind::<BigInt, _>(current_time_ms)
                .get_result::<CountRow>(&mut conn)
                .await?
        }
        .count;

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
            schedule_end: i64,
            #[diesel(sql_type = diesel::sql_types::Jsonb)]
            pieces: serde_json::Value,
            #[diesel(sql_type = BigInt)]
            claimed_amount: i64,
            #[diesel(sql_type = BigInt)]
            remaining_balance: i64,
            #[diesel(sql_type = diesel::sql_types::Timestamp)]
            created_at: chrono::NaiveDateTime,
            #[diesel(sql_type = diesel::sql_types::Timestamp)]
            updated_at: chrono::NaiveDateTime,
            #[diesel(sql_type = diesel::sql_types::Text)]
            transaction_id: String,
        }

        let rows: Vec<WalletRow> = if let Some(o) = owner {
            if active_only {
                diesel::sql_query(data_sql)
                    .bind::<diesel::sql_types::Text, _>(o)
                    .bind::<BigInt, _>(current_time_ms)
                    .bind::<BigInt, _>(limit)
                    .bind::<BigInt, _>(offset)
                    .load(&mut conn)
                    .await?
            } else {
                diesel::sql_query(data_sql)
                    .bind::<diesel::sql_types::Text, _>(o)
                    .bind::<BigInt, _>(limit)
                    .bind::<BigInt, _>(offset)
                    .load(&mut conn)
                    .await?
            }
        } else {
            diesel::sql_query(data_sql)
                .bind::<BigInt, _>(current_time_ms)
                .bind::<BigInt, _>(limit)
                .bind::<BigInt, _>(offset)
                .load(&mut conn)
                .await?
        };
        let results: Vec<VestingWalletRow> = rows
            .into_iter()
            .map(|r| {
                vesting_wallet_row_from_tuple(
                    r.wallet_id,
                    r.owner_address,
                    r.total_amount,
                    r.start_time,
                    r.schedule_end,
                    r.pieces,
                    r.claimed_amount,
                    r.remaining_balance,
                    r.created_at,
                    r.updated_at,
                    r.transaction_id,
                )
            })
            .collect();
        (total, results)
    } else {
        let total = vesting_wallets::table
            .count()
            .get_result::<i64>(&mut conn)
            .await?;
        let rows = vesting_wallets::table
            .order_by(vesting_wallets::created_at.desc())
            .limit(limit)
            .offset(offset)
            .select((
                vesting_wallets::wallet_id,
                vesting_wallets::owner_address,
                vesting_wallets::total_amount,
                vesting_wallets::start_time,
                vesting_wallets::schedule_end,
                vesting_wallets::pieces,
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
                serde_json::Value,
                i64,
                i64,
                chrono::NaiveDateTime,
                chrono::NaiveDateTime,
                String,
            )>(&mut conn)
            .await?;
        let results: Vec<VestingWalletRow> = rows
            .into_iter()
            .map(
                |(
                    wallet_id,
                    owner_address,
                    total_amount,
                    start_time,
                    schedule_end,
                    pieces,
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
                        schedule_end,
                        pieces,
                        claimed_amount,
                        remaining_balance,
                        created_at,
                        updated_at,
                        transaction_id,
                    )
                },
            )
            .collect();
        (total, results)
    };

    let owner_addresses: Vec<String> = results.iter().map(|w| w.owner_address.clone()).collect();
    let user_map = enrich_users_with_universal_data(&mut conn, owner_addresses).await?;

    let wallets_with_profile: Vec<VestingWalletWithProfile> = results
        .into_iter()
        .map(|w| {
            let with_status =
                VestingWalletWithStatus::from_wallet(w.clone(), current_time_ms as u64);
            let user = user_map
                .get(&w.owner_address.to_lowercase())
                .cloned()
                .unwrap_or_else(|| UniversalUserResult {
                    owner_address: w.owner_address.clone(),
                    wallet_address: w.owner_address.clone(),
                    username: None,
                    fullname: None,
                    profile_photo: None,
                    social_proof_token: None,
                    selected_badge: None,
                });
            VestingWalletWithProfile {
                wallet: with_status,
                user,
            }
        })
        .collect();

    let total_pages = if total > 0 {
        ((total as f64) / (limit as f64)).ceil() as i64
    } else {
        1
    };

    Ok(VestingWalletsResponse {
        wallets: wallets_with_profile,
        total,
        pagination: PaginationInfo {
            total,
            limit,
            offset,
            page,
            total_pages,
        },
    })
}

pub(crate) async fn get_vesting_wallet_by_id(
    db: &Db,
    wallet_id: &str,
) -> Result<Option<VestingWalletWithStatus>, SocialError> {
    let mut conn = db.connect().await?;
    let result = vesting_wallets::table
        .filter(vesting_wallets::wallet_id.eq(wallet_id))
        .select((
            vesting_wallets::wallet_id,
            vesting_wallets::owner_address,
            vesting_wallets::total_amount,
            vesting_wallets::start_time,
            vesting_wallets::schedule_end,
            vesting_wallets::pieces,
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
            serde_json::Value,
            i64,
            i64,
            chrono::NaiveDateTime,
            chrono::NaiveDateTime,
            String,
        )>(&mut conn)
        .await
        .optional()?;
    let current_time_ms = chrono::Utc::now().timestamp_millis() as u64;
    Ok(result.map(|r| {
        VestingWalletWithStatus::from_wallet(
            vesting_wallet_row_from_tuple(r.0, r.1, r.2, r.3, r.4, r.5, r.6, r.7, r.8, r.9, r.10),
            current_time_ms,
        )
    }))
}

pub(crate) async fn get_vesting_wallet_events(
    db: &Db,
    wallet_id: &str,
    limit: i64,
    offset: i64,
    page: i64,
) -> Result<VestingEventsResponse, SocialError> {
    let mut conn = db.connect().await?;

    let total = vesting_events::table
        .filter(vesting_events::wallet_id.eq(wallet_id))
        .count()
        .get_result::<i64>(&mut conn)
        .await?;

    let results = vesting_events::table
        .filter(vesting_events::wallet_id.eq(wallet_id))
        .order_by(vesting_events::event_time.desc())
        .limit(limit)
        .offset(offset)
        .select((
            vesting_events::id,
            vesting_events::wallet_id,
            vesting_events::event_type,
            vesting_events::owner_address,
            vesting_events::amount,
            vesting_events::remaining_balance,
            vesting_events::start_time,
            vesting_events::schedule_end,
            vesting_events::pieces,
            vesting_events::event_time,
            vesting_events::time,
            vesting_events::transaction_id,
        ))
        .load::<(
            i32,
            String,
            String,
            String,
            i64,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<serde_json::Value>,
            i64,
            chrono::DateTime<chrono::Utc>,
            String,
        )>(&mut conn)
        .await?;

    let events: Vec<VestingEventRow> = results
        .into_iter()
        .map(
            |(
                id,
                wallet_id,
                event_type,
                owner_address,
                amount,
                remaining_balance,
                start_time,
                schedule_end,
                pieces,
                event_time,
                time,
                transaction_id,
            )| VestingEventRow {
                id,
                wallet_id,
                event_type,
                owner_address,
                amount,
                remaining_balance,
                start_time,
                schedule_end,
                pieces,
                event_time,
                time,
                transaction_id,
            },
        )
        .collect();

    let total_pages = if total > 0 {
        ((total as f64) / (limit as f64)).ceil() as i64
    } else {
        1
    };

    Ok(VestingEventsResponse {
        events,
        total,
        pagination: PaginationInfo {
            total,
            limit,
            offset,
            page,
            total_pages,
        },
    })
}

pub(crate) async fn get_vesting_claimable(
    db: &Db,
    wallet_id: &str,
) -> Result<Option<ClaimableResponse>, SocialError> {
    let mut conn = db.connect().await?;
    let current_time_ms = chrono::Utc::now().timestamp_millis();

    let query = r#"
        SELECT
            wallet_id,
            start_time,
            schedule_end,
            calculate_vesting_claimable(
                total_amount, start_time, schedule_end, pieces,
                claimed_amount, $2::bigint, remaining_balance
            )::bigint as claimable
        FROM vesting_wallets
        WHERE wallet_id = $1
    "#;

    #[derive(QueryableByName)]
    struct ClaimableRow {
        #[diesel(sql_type = diesel::sql_types::Text)]
        wallet_id: String,
        #[diesel(sql_type = BigInt)]
        start_time: i64,
        #[diesel(sql_type = BigInt)]
        schedule_end: i64,
        #[diesel(sql_type = BigInt)]
        claimable: i64,
    }

    let result = diesel::sql_query(query)
        .bind::<diesel::sql_types::Text, _>(wallet_id)
        .bind::<BigInt, _>(current_time_ms)
        .get_result::<ClaimableRow>(&mut conn)
        .await
        .optional()?;

    let Some(row) = result else {
        return Ok(None);
    };

    let end_time = row.schedule_end;
    let has_started = row.start_time <= current_time_ms;
    let has_ended = current_time_ms >= end_time;
    let vesting_progress = if current_time_ms <= row.start_time {
        0.0
    } else if current_time_ms >= end_time {
        1.0
    } else {
        let elapsed = current_time_ms - row.start_time;
        let total_duration = end_time - row.start_time;
        if total_duration <= 0 {
            1.0
        } else {
            elapsed as f64 / total_duration as f64
        }
    };
    let vesting_status = if !has_started {
        "not_started"
    } else if has_ended {
        "completed"
    } else {
        "in_progress"
    };

    Ok(Some(ClaimableResponse {
        wallet_id: row.wallet_id,
        claimable_amount: row.claimable,
        current_progress: vesting_progress,
        vesting_status: vesting_status.to_string(),
        timestamp: current_time_ms as u64,
    }))
}

pub(crate) async fn get_user_vesting_wallets(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
    page: i64,
) -> Result<VestingWalletsResponse, SocialError> {
    list_vesting_wallets(db, false, Some(address), limit, offset, page).await
}

pub(crate) async fn list_vesting_events(
    db: &Db,
    limit: i64,
    offset: i64,
    page: i64,
    owner_address: Option<&str>,
) -> Result<VestingEventsResponse, SocialError> {
    let mut conn = db.connect().await?;

    let mut query = vesting_events::table.into_boxed();
    if let Some(owner) = owner_address {
        query = query.filter(vesting_events::owner_address.eq(owner));
    }

    let mut count_query = vesting_events::table.into_boxed();
    if let Some(owner) = owner_address {
        count_query = count_query.filter(vesting_events::owner_address.eq(owner));
    }
    let total = count_query.count().get_result::<i64>(&mut conn).await?;

    let results = query
        .order_by(vesting_events::event_time.desc())
        .limit(limit)
        .offset(offset)
        .select((
            vesting_events::id,
            vesting_events::wallet_id,
            vesting_events::event_type,
            vesting_events::owner_address,
            vesting_events::amount,
            vesting_events::remaining_balance,
            vesting_events::start_time,
            vesting_events::schedule_end,
            vesting_events::pieces,
            vesting_events::event_time,
            vesting_events::time,
            vesting_events::transaction_id,
        ))
        .load::<(
            i32,
            String,
            String,
            String,
            i64,
            Option<i64>,
            Option<i64>,
            Option<i64>,
            Option<serde_json::Value>,
            i64,
            chrono::DateTime<chrono::Utc>,
            String,
        )>(&mut conn)
        .await?;

    let events: Vec<VestingEventRow> = results
        .into_iter()
        .map(
            |(
                id,
                wallet_id,
                event_type,
                owner_address,
                amount,
                remaining_balance,
                start_time,
                schedule_end,
                pieces,
                event_time,
                time,
                transaction_id,
            )| VestingEventRow {
                id,
                wallet_id,
                event_type,
                owner_address,
                amount,
                remaining_balance,
                start_time,
                schedule_end,
                pieces,
                event_time,
                time,
                transaction_id,
            },
        )
        .collect();

    let total_pages = if total > 0 {
        ((total as f64) / (limit as f64)).ceil() as i64
    } else {
        1
    };

    Ok(VestingEventsResponse {
        events,
        total,
        pagination: PaginationInfo {
            total,
            limit,
            offset,
            page,
            total_pages,
        },
    })
}

pub(crate) async fn get_vesting_analytics(
    db: &Db,
) -> Result<VestingAnalyticsResponse, SocialError> {
    let mut conn = db.connect().await?;
    let current_time_ms = chrono::Utc::now().timestamp_millis();

    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        count: i64,
    }
    #[derive(QueryableByName)]
    struct SumRow {
        #[diesel(sql_type = BigInt)]
        total: i64,
    }
    #[derive(QueryableByName)]
    struct AvgRow {
        #[diesel(sql_type = diesel::sql_types::Double)]
        avg: f64,
    }

    let total_wallets: i64 =
        diesel::sql_query("SELECT COUNT(*)::bigint as count FROM vesting_wallets")
            .get_result::<CountRow>(&mut conn)
            .await
            .map(|r| r.count)?;

    let total_vested_amount: i64 = diesel::sql_query(
        "SELECT COALESCE(SUM(total_amount), 0)::bigint as total FROM vesting_wallets",
    )
    .get_result::<SumRow>(&mut conn)
    .await
    .map(|r| r.total)?;

    let total_claimed_amount: i64 = diesel::sql_query(
        "SELECT COALESCE(SUM(claimed_amount), 0)::bigint as total FROM vesting_wallets",
    )
    .get_result::<SumRow>(&mut conn)
    .await
    .map(|r| r.total)?;

    let total_remaining_amount = total_vested_amount - total_claimed_amount;

    let active_count: i64 = diesel::sql_query(
        r#"
        SELECT COUNT(*)::bigint as count FROM vesting_wallets
        WHERE start_time <= $1
          AND remaining_balance > 0
          AND schedule_end > $1
        "#,
    )
    .bind::<BigInt, _>(current_time_ms)
    .get_result::<CountRow>(&mut conn)
    .await
    .map(|r| r.count)?;

    let completed_count: i64 = diesel::sql_query(
        r#"
        SELECT COUNT(*)::bigint as count FROM vesting_wallets
        WHERE schedule_end <= $1
        "#,
    )
    .bind::<BigInt, _>(current_time_ms)
    .get_result::<CountRow>(&mut conn)
    .await
    .map(|r| r.count)?;

    let average_schedule_duration: f64 = diesel::sql_query(
        "SELECT COALESCE(AVG(schedule_end - start_time)::double precision / 86400000.0, 0) as avg FROM vesting_wallets",
    )
    .get_result::<AvgRow>(&mut conn)
    .await
    .map(|r| r.avg)
    .unwrap_or(0.0);

    Ok(VestingAnalyticsResponse {
        total_wallets,
        total_vested_amount,
        total_claimed_amount,
        total_remaining_amount,
        active_wallets: active_count,
        completed_wallets: completed_count,
        average_schedule_duration,
    })
}

pub(crate) async fn get_vesting_leaderboard(
    db: &Db,
    limit: i64,
    offset: i64,
    _page: i64,
) -> Result<VestingLeaderboardResponse, SocialError> {
    let mut conn = db.connect().await?;
    let current_time_ms = chrono::Utc::now().timestamp_millis();

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

    #[derive(QueryableByName)]
    struct TotalRow {
        #[diesel(sql_type = BigInt)]
        count: i64,
    }
    let total = diesel::sql_query(
        "SELECT COUNT(DISTINCT owner_address)::bigint as count FROM vesting_wallets",
    )
    .get_result::<TotalRow>(&mut conn)
    .await
    .map(|r| r.count)?;

    let query = r#"
        SELECT
            owner_address,
            SUM(total_amount)::bigint as total_vested,
            SUM(claimed_amount)::bigint as total_claimed,
            SUM(CASE WHEN start_time <= $1 AND remaining_balance > 0 AND schedule_end > $1 THEN 1 ELSE 0 END)::bigint as active_wallets,
            SUM(CASE WHEN schedule_end <= $1 THEN 1 ELSE 0 END)::bigint as completed_wallets
        FROM vesting_wallets
        GROUP BY owner_address
        ORDER BY total_vested DESC
        LIMIT $2 OFFSET $3
    "#;

    let rows: Vec<LeaderboardRow> = diesel::sql_query(query)
        .bind::<BigInt, _>(current_time_ms)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<LeaderboardRow>(&mut conn)
        .await?;

    let owner_addresses: Vec<String> = rows.iter().map(|r| r.owner_address.clone()).collect();
    let user_map = enrich_users_with_universal_data(&mut conn, owner_addresses).await?;

    let entries: Vec<VestingLeaderboardEntry> = rows
        .into_iter()
        .map(|r| {
            let user = user_map
                .get(&r.owner_address.to_lowercase())
                .cloned()
                .unwrap_or_else(|| UniversalUserResult {
                    owner_address: r.owner_address.clone(),
                    wallet_address: r.owner_address.clone(),
                    username: None,
                    fullname: None,
                    profile_photo: None,
                    social_proof_token: None,
                    selected_badge: None,
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

    Ok(VestingLeaderboardResponse { entries, total })
}
