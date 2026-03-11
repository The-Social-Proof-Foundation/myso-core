// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::BigInt;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::schema::{vesting_events, vesting_wallets};

use crate::error::SocialError;
use crate::reader::types::{VestingEventRow, VestingWalletRow};
use myso_pg_db::Db;

pub(crate) async fn list_vesting_wallets(
    db: &Db,
    active_only: bool,
    owner: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<VestingWalletRow>, SocialError> {
    let mut conn = db.connect().await?;
    let mut query = vesting_wallets::table.into_boxed();
    if active_only {
        query = query.filter(vesting_wallets::remaining_balance.gt(0));
    }
    if let Some(o) = owner {
        query = query.filter(vesting_wallets::owner_address.eq(o));
    }
    let results = query
        .order_by(vesting_wallets::created_at.desc())
        .limit(limit)
        .offset(offset)
        .select((
            vesting_wallets::wallet_id,
            vesting_wallets::owner_address,
            vesting_wallets::total_amount,
            vesting_wallets::claimed_amount,
            vesting_wallets::remaining_balance,
            vesting_wallets::start_time,
            vesting_wallets::duration,
            vesting_wallets::created_at,
        ))
        .load::<(
            String,
            String,
            i64,
            i64,
            i64,
            i64,
            i64,
            chrono::NaiveDateTime,
        )>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(
                wallet_id,
                owner_address,
                total_amount,
                claimed_amount,
                remaining_balance,
                start_time,
                duration,
                created_at,
            )| VestingWalletRow {
                wallet_id,
                owner_address,
                total_amount,
                claimed_amount,
                remaining_balance,
                start_time,
                duration,
                created_at,
            },
        )
        .collect())
}

pub(crate) async fn get_vesting_wallet_by_id(
    db: &Db,
    wallet_id: &str,
) -> Result<Option<VestingWalletRow>, SocialError> {
    let mut conn = db.connect().await?;
    let result = vesting_wallets::table
        .filter(vesting_wallets::wallet_id.eq(wallet_id))
        .select((
            vesting_wallets::wallet_id,
            vesting_wallets::owner_address,
            vesting_wallets::total_amount,
            vesting_wallets::claimed_amount,
            vesting_wallets::remaining_balance,
            vesting_wallets::start_time,
            vesting_wallets::duration,
            vesting_wallets::created_at,
        ))
        .first::<(
            String,
            String,
            i64,
            i64,
            i64,
            i64,
            i64,
            chrono::NaiveDateTime,
        )>(&mut conn)
        .await
        .optional()?;
    Ok(result.map(
        |(
            wallet_id,
            owner_address,
            total_amount,
            claimed_amount,
            remaining_balance,
            start_time,
            duration,
            created_at,
        )| VestingWalletRow {
            wallet_id,
            owner_address,
            total_amount,
            claimed_amount,
            remaining_balance,
            start_time,
            duration,
            created_at,
        },
    ))
}

pub(crate) async fn get_vesting_wallet_events(
    db: &Db,
    wallet_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<VestingEventRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = vesting_events::table
        .filter(vesting_events::wallet_id.eq(wallet_id))
        .order_by(vesting_events::event_time.desc())
        .limit(limit)
        .offset(offset)
        .select((
            vesting_events::wallet_id,
            vesting_events::event_type,
            vesting_events::amount,
            vesting_events::event_time,
        ))
        .load::<(String, String, i64, i64)>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(wallet_id, event_type, amount, event_time)| VestingEventRow {
                wallet_id,
                event_type,
                amount,
                event_time,
            },
        )
        .collect())
}

pub(crate) async fn get_vesting_claimable(
    db: &Db,
    wallet_id: &str,
) -> Result<Option<i64>, SocialError> {
    let mut conn = db.connect().await?;
    let result = vesting_wallets::table
        .filter(vesting_wallets::wallet_id.eq(wallet_id))
        .select(vesting_wallets::remaining_balance)
        .first::<i64>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_user_vesting_wallets(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<VestingWalletRow>, SocialError> {
    list_vesting_wallets(db, false, Some(address), limit, offset).await
}

pub(crate) async fn list_vesting_events(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<VestingEventRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = vesting_events::table
        .order_by(vesting_events::event_time.desc())
        .limit(limit)
        .offset(offset)
        .select((
            vesting_events::wallet_id,
            vesting_events::event_type,
            vesting_events::amount,
            vesting_events::event_time,
        ))
        .load::<(String, String, i64, i64)>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(wallet_id, event_type, amount, event_time)| VestingEventRow {
                wallet_id,
                event_type,
                amount,
                event_time,
            },
        )
        .collect())
}

pub(crate) async fn get_vesting_analytics(db: &Db) -> Result<serde_json::Value, SocialError> {
    let mut conn = db.connect().await?;
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
    let wallets: i64 = diesel::sql_query("SELECT COUNT(*)::bigint as count FROM vesting_wallets")
        .get_result::<CountRow>(&mut conn)
        .await
        .map(|r| r.count)?;
    let total_vested: i64 = diesel::sql_query(
        "SELECT COALESCE(SUM(total_amount), 0)::bigint as total FROM vesting_wallets",
    )
    .get_result::<SumRow>(&mut conn)
    .await
    .map(|r| r.total)?;
    let total_claimed: i64 = diesel::sql_query(
        "SELECT COALESCE(SUM(claimed_amount), 0)::bigint as total FROM vesting_wallets",
    )
    .get_result::<SumRow>(&mut conn)
    .await
    .map(|r| r.total)?;
    Ok(serde_json::json!({
        "total_wallets": wallets,
        "total_vested": total_vested,
        "total_claimed": total_claimed,
        "total_remaining": total_vested - total_claimed,
    }))
}

pub(crate) async fn get_vesting_leaderboard(
    db: &Db,
    limit: i64,
) -> Result<Vec<VestingWalletRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = vesting_wallets::table
        .order_by(vesting_wallets::remaining_balance.desc())
        .limit(limit)
        .select((
            vesting_wallets::wallet_id,
            vesting_wallets::owner_address,
            vesting_wallets::total_amount,
            vesting_wallets::claimed_amount,
            vesting_wallets::remaining_balance,
            vesting_wallets::start_time,
            vesting_wallets::duration,
            vesting_wallets::created_at,
        ))
        .load::<(
            String,
            String,
            i64,
            i64,
            i64,
            i64,
            i64,
            chrono::NaiveDateTime,
        )>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(
                wallet_id,
                owner_address,
                total_amount,
                claimed_amount,
                remaining_balance,
                start_time,
                duration,
                created_at,
            )| VestingWalletRow {
                wallet_id,
                owner_address,
                total_amount,
                claimed_amount,
                remaining_balance,
                start_time,
                duration,
                created_at,
            },
        )
        .collect())
}
