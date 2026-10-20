// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_pg_db::Db;

use crate::error::SocialError;
use crate::reader::social_graph::check_either_profile_blocked;
use crate::reader::types::*;

fn effective_tier_level(tier_level: Option<i64>) -> i64 {
    tier_level.unwrap_or(0)
}

fn tier_satisfies(subscription_tier: Option<i64>, min_tier: Option<i64>) -> bool {
    match min_tier {
        None => true,
        Some(min) => effective_tier_level(subscription_tier) >= min,
    }
}

pub(crate) async fn get_profile_subscription_service(
    db: &Db,
    service_id: &str,
) -> Result<Option<ProfileSubscriptionServiceInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT s.service_id, s.profile_owner, s.profile_id, s.plan_count, s.active,
               s.subscriber_count, s.created_at, s.updated_at,
               p.username, p.display_name, p.profile_photo
        FROM profile_subscription_services s
        LEFT JOIN profiles p ON p.owner_address = s.profile_owner
        WHERE s.service_id = $1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(service_id)
        .get_result::<ProfileSubscriptionServiceInfo>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_profile_subscription_plans_by_service(
    db: &Db,
    service_id: &str,
    active_only: bool,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProfileSubscriptionPlanInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = if active_only {
        "
        SELECT plan_id, service_id, title, description, price, duration_ms,
               tier_level, platform_id, active, created_at, updated_at
        FROM profile_subscription_plans
        WHERE service_id = $1 AND active = true
        ORDER BY tier_level NULLS FIRST, price ASC
        LIMIT $2 OFFSET $3
        "
    } else {
        "
        SELECT plan_id, service_id, title, description, price, duration_ms,
               tier_level, platform_id, active, created_at, updated_at
        FROM profile_subscription_plans
        WHERE service_id = $1
        ORDER BY created_at ASC
        LIMIT $2 OFFSET $3
        "
    };
    let results = diesel::sql_query(query)
        .bind::<Text, _>(service_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<ProfileSubscriptionPlanInfo>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_profile_subscription_services_by_owner(
    db: &Db,
    profile_owner: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProfileSubscriptionServiceInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT s.service_id, s.profile_owner, s.profile_id, s.plan_count, s.active,
               s.subscriber_count, s.created_at, s.updated_at,
               p.username, p.display_name, p.profile_photo
        FROM profile_subscription_services s
        LEFT JOIN profiles p ON p.owner_address = s.profile_owner
        WHERE s.profile_owner = $1
        ORDER BY s.created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(profile_owner)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<ProfileSubscriptionServiceInfo>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_active_subscriptions_by_subscriber(
    db: &Db,
    subscriber: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProfileSubscriptionInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let now_ms = chrono::Utc::now().timestamp_millis();
    let query = "
        SELECT sub.subscription_id, sub.service_id, sub.plan_id, sub.tier_level, sub.platform_id,
               sub.price, sub.duration_ms, sub.subscriber, sub.created_at,
               sub.expires_at, sub.auto_renew, sub.renewal_balance, sub.renewal_count,
               sub.cancelled_at, s.profile_owner,
               p.username, p.display_name
        FROM (
            SELECT DISTINCT ON (subscription_id) *
            FROM profile_subscriptions
            WHERE subscriber = $1 AND cancelled_at IS NULL AND expires_at > $2
            ORDER BY subscription_id, time DESC
        ) sub
        JOIN profile_subscription_services s ON s.service_id = sub.service_id
        LEFT JOIN profiles p ON p.owner_address = s.profile_owner
        ORDER BY sub.expires_at DESC
        LIMIT $3 OFFSET $4
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(subscriber)
        .bind::<BigInt, _>(now_ms)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<ProfileSubscriptionInfo>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_subscription_by_id(
    db: &Db,
    subscription_id: &str,
) -> Result<Option<ProfileSubscriptionInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT sub.subscription_id, sub.service_id, sub.plan_id, sub.tier_level, sub.platform_id,
               sub.price, sub.duration_ms, sub.subscriber, sub.created_at,
               sub.expires_at, sub.auto_renew, sub.renewal_balance, sub.renewal_count,
               sub.cancelled_at, s.profile_owner,
               p.username, p.display_name
        FROM (
            SELECT * FROM profile_subscriptions
            WHERE subscription_id = $1
            ORDER BY time DESC
            LIMIT 1
        ) sub
        JOIN profile_subscription_services s ON s.service_id = sub.service_id
        LEFT JOIN profiles p ON p.owner_address = s.profile_owner
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(subscription_id)
        .get_result::<ProfileSubscriptionInfo>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_subscription_revenue_by_service(
    db: &Db,
    service_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProfileSubscriptionRevenueRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT service_id, subscription_id, from_address, to_address, amount,
               platform_fee, ecosystem_fee, creator_amount, platform_address,
               revenue_type, payment_time, time, transaction_id
        FROM subscription_revenue
        WHERE service_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(service_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<ProfileSubscriptionRevenueRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn check_subscription_access(
    db: &Db,
    subscriber: &str,
    service_id: &str,
    min_tier_level: Option<i64>,
) -> Result<bool, SocialError> {
    if let Some(service) = get_profile_subscription_service(db, service_id).await? {
        if check_either_profile_blocked(db, subscriber, &service.profile_owner).await? {
            return Ok(false);
        }
    }

    let mut conn = db.connect().await?;
    let now_ms = chrono::Utc::now().timestamp_millis();
    #[derive(QueryableByName)]
    struct TierRow {
        #[diesel(sql_type = Nullable<BigInt>)]
        tier_level: Option<i64>,
    }
    let query = "
        SELECT sub.tier_level
        FROM (
            SELECT DISTINCT ON (subscription_id) subscription_id, expires_at, cancelled_at, tier_level
            FROM profile_subscriptions
            WHERE subscriber = $1 AND service_id = $2
            ORDER BY subscription_id, time DESC
        ) sub
        WHERE sub.cancelled_at IS NULL AND sub.expires_at > $3
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(subscriber)
        .bind::<Text, _>(service_id)
        .bind::<BigInt, _>(now_ms)
        .get_result::<TierRow>(&mut conn)
        .await
        .optional()?;
    Ok(result
        .map(|row| tier_satisfies(row.tier_level, min_tier_level))
        .unwrap_or(false))
}

pub(crate) async fn list_subscription_services(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProfileSubscriptionServiceInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT s.service_id, s.profile_owner, s.profile_id, s.plan_count, s.active,
               s.subscriber_count, s.created_at, s.updated_at,
               p.username, p.display_name, p.profile_photo
        FROM profile_subscription_services s
        LEFT JOIN profiles p ON p.owner_address = s.profile_owner
        ORDER BY s.subscriber_count DESC, s.created_at DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<ProfileSubscriptionServiceInfo>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn list_subscription_revenue(
    db: &Db,
    service_id: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProfileSubscriptionRevenueRow>, SocialError> {
    let mut conn = db.connect().await?;
    let (query, bind_service) = if service_id.is_some() {
        (
            "SELECT service_id, subscription_id, from_address, to_address, amount,
                    platform_fee, ecosystem_fee, creator_amount, platform_address,
                    revenue_type, payment_time, time, transaction_id
             FROM subscription_revenue WHERE service_id = $1
             ORDER BY time DESC LIMIT $2 OFFSET $3",
            true,
        )
    } else {
        (
            "SELECT service_id, subscription_id, from_address, to_address, amount,
                    platform_fee, ecosystem_fee, creator_amount, platform_address,
                    revenue_type, payment_time, time, transaction_id
             FROM subscription_revenue
             ORDER BY time DESC LIMIT $1 OFFSET $2",
            false,
        )
    };
    let results = if bind_service {
        diesel::sql_query(query)
            .bind::<Text, _>(service_id.unwrap())
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ProfileSubscriptionRevenueRow>(&mut conn)
            .await?
    } else {
        diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ProfileSubscriptionRevenueRow>(&mut conn)
            .await?
    };
    Ok(results)
}

pub(crate) async fn get_subscriber_summary(
    db: &Db,
    address: &str,
) -> Result<SubscriberSummaryRow, SocialError> {
    let mut conn = db.connect().await?;
    let now_ms = chrono::Utc::now().timestamp_millis();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = BigInt)]
        active: i64,
        #[diesel(sql_type = BigInt)]
        revenue: i64,
    }
    let query = "
        SELECT
            (SELECT COUNT(DISTINCT subscription_id)::bigint FROM profile_subscriptions
             WHERE subscriber = $1 AND cancelled_at IS NULL AND expires_at > $2) as active,
            (SELECT COALESCE(SUM(amount), 0)::bigint FROM subscription_revenue
             WHERE from_address = $1) as revenue
    ";
    let row = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<BigInt, _>(now_ms)
        .get_result::<Row>(&mut conn)
        .await?;
    Ok(SubscriberSummaryRow {
        active_subscriptions: row.active,
        total_revenue: row.revenue,
    })
}

pub(crate) async fn get_subscription_configuration(
    db: &Db,
) -> Result<Option<SubscriptionConfigInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT updated_by, default_billing_period_ms, max_renewal_months,
               platform_fee_bps, ecosystem_fee_bps,
               non_platform_platform_to_creator_bps, non_platform_platform_to_treasury_bps,
               version, updated_at
        FROM subscription_config
        ORDER BY time DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .get_result::<SubscriptionConfigInfo>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}
