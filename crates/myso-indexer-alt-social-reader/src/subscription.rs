// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Bool, Nullable, Text, Timestamptz};
use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, QueryableByName)]
pub struct SubscriptionConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub default_billing_period_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub max_renewal_months: i64,
    #[diesel(sql_type = BigInt)]
    pub platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub ecosystem_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub non_platform_platform_to_creator_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub non_platform_platform_to_treasury_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct ProfileSubscriptionServiceRow {
    #[diesel(sql_type = Text)]
    pub service_id: String,
    #[diesel(sql_type = Text)]
    pub profile_owner: String,
    #[diesel(sql_type = Text)]
    pub profile_id: String,
    #[diesel(sql_type = BigInt)]
    pub plan_count: i64,
    #[diesel(sql_type = Bool)]
    pub active: bool,
    #[diesel(sql_type = BigInt)]
    pub subscriber_count: i64,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct ProfileSubscriptionPlanRow {
    #[diesel(sql_type = Text)]
    pub plan_id: String,
    #[diesel(sql_type = Text)]
    pub service_id: String,
    #[diesel(sql_type = Text)]
    pub title: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub description: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub price: i64,
    #[diesel(sql_type = BigInt)]
    pub duration_ms: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub tier_level: Option<i64>,
    #[diesel(sql_type = Nullable<Text>)]
    pub platform_id: Option<String>,
    #[diesel(sql_type = Bool)]
    pub active: bool,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub updated_at: Option<i64>,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct ProfileSubscriptionRow {
    #[diesel(sql_type = Text)]
    pub subscription_id: String,
    #[diesel(sql_type = Text)]
    pub service_id: String,
    #[diesel(sql_type = Text)]
    pub plan_id: String,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub tier_level: Option<i64>,
    #[diesel(sql_type = Nullable<Text>)]
    pub platform_id: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub price: i64,
    #[diesel(sql_type = BigInt)]
    pub duration_ms: i64,
    #[diesel(sql_type = Text)]
    pub subscriber: String,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = BigInt)]
    pub expires_at: i64,
    #[diesel(sql_type = Bool)]
    pub auto_renew: bool,
    #[diesel(sql_type = BigInt)]
    pub renewal_balance: i64,
    #[diesel(sql_type = BigInt)]
    pub renewal_count: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub cancelled_at: Option<i64>,
    #[diesel(sql_type = Text)]
    pub profile_owner: String,
}

fn effective_tier_level(tier_level: Option<i64>) -> i64 {
    tier_level.unwrap_or(0)
}

fn tier_satisfies(subscription_tier: Option<i64>, min_tier: Option<i64>) -> bool {
    match min_tier {
        None => true,
        Some(min) => effective_tier_level(subscription_tier) >= min,
    }
}

/// Latest subscription configuration.
pub(crate) async fn get_subscription_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SubscriptionConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT updated_by, default_billing_period_ms, max_renewal_months,
               platform_fee_bps, ecosystem_fee_bps,
               non_platform_platform_to_creator_bps, non_platform_platform_to_treasury_bps,
               version, updated_at, time, transaction_id
        FROM subscription_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<SubscriptionConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_profile_subscription_service(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
    service_id: &str,
) -> anyhow::Result<Option<ProfileSubscriptionServiceRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT service_id, profile_owner, profile_id, plan_count, active,
               subscriber_count, created_at, updated_at
        FROM profile_subscription_services
        WHERE service_id = $1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(service_id)
        .get_result::<ProfileSubscriptionServiceRow>(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_profile_subscription_services_by_owner(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
    profile_owner: &str,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<ProfileSubscriptionServiceRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT service_id, profile_owner, profile_id, plan_count, active,
               subscriber_count, created_at, updated_at
        FROM profile_subscription_services
        WHERE profile_owner = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let rows = diesel::sql_query(query)
        .bind::<Text, _>(profile_owner)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<ProfileSubscriptionServiceRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn list_profile_subscription_plans_by_service(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
    service_id: &str,
    active_only: bool,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<ProfileSubscriptionPlanRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
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
    let rows = diesel::sql_query(query)
        .bind::<Text, _>(service_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<ProfileSubscriptionPlanRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn get_profile_subscription_plan_by_id(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
    plan_id: &str,
) -> anyhow::Result<Option<ProfileSubscriptionPlanRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT plan_id, service_id, title, description, price, duration_ms,
               tier_level, platform_id, active, created_at, updated_at
        FROM profile_subscription_plans
        WHERE plan_id = $1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(plan_id)
        .get_result::<ProfileSubscriptionPlanRow>(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_profile_subscription_by_id(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
    subscription_id: &str,
) -> anyhow::Result<Option<ProfileSubscriptionRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT sub.subscription_id, sub.service_id, sub.plan_id, sub.tier_level, sub.platform_id,
               sub.price, sub.duration_ms, sub.subscriber, sub.created_at,
               sub.expires_at, sub.auto_renew, sub.renewal_balance, sub.renewal_count,
               sub.cancelled_at, s.profile_owner
        FROM (
            SELECT * FROM profile_subscriptions
            WHERE subscription_id = $1
            ORDER BY time DESC
            LIMIT 1
        ) sub
        JOIN profile_subscription_services s ON s.service_id = sub.service_id
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(subscription_id)
        .get_result::<ProfileSubscriptionRow>(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_active_profile_subscriptions_by_subscriber(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
    subscriber: &str,
    service_id: Option<&str>,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<ProfileSubscriptionRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let now_ms = chrono::Utc::now().timestamp_millis();
    let query = if service_id.is_some() {
        "
        SELECT sub.subscription_id, sub.service_id, sub.plan_id, sub.tier_level, sub.platform_id,
               sub.price, sub.duration_ms, sub.subscriber, sub.created_at,
               sub.expires_at, sub.auto_renew, sub.renewal_balance, sub.renewal_count,
               sub.cancelled_at, s.profile_owner
        FROM (
            SELECT DISTINCT ON (subscription_id) *
            FROM profile_subscriptions
            WHERE subscriber = $1 AND service_id = $4
              AND cancelled_at IS NULL AND expires_at > $2
            ORDER BY subscription_id, time DESC
        ) sub
        JOIN profile_subscription_services s ON s.service_id = sub.service_id
        ORDER BY sub.expires_at DESC
        LIMIT $3 OFFSET $5
        "
    } else {
        "
        SELECT sub.subscription_id, sub.service_id, sub.plan_id, sub.tier_level, sub.platform_id,
               sub.price, sub.duration_ms, sub.subscriber, sub.created_at,
               sub.expires_at, sub.auto_renew, sub.renewal_balance, sub.renewal_count,
               sub.cancelled_at, s.profile_owner
        FROM (
            SELECT DISTINCT ON (subscription_id) *
            FROM profile_subscriptions
            WHERE subscriber = $1 AND cancelled_at IS NULL AND expires_at > $2
            ORDER BY subscription_id, time DESC
        ) sub
        JOIN profile_subscription_services s ON s.service_id = sub.service_id
        ORDER BY sub.expires_at DESC
        LIMIT $3 OFFSET $4
        "
    };
    let rows = if let Some(sid) = service_id {
        diesel::sql_query(query)
            .bind::<Text, _>(subscriber)
            .bind::<BigInt, _>(now_ms)
            .bind::<BigInt, _>(limit)
            .bind::<Text, _>(sid)
            .bind::<BigInt, _>(offset)
            .load::<ProfileSubscriptionRow>(conn)
            .await?
    } else {
        diesel::sql_query(query)
            .bind::<Text, _>(subscriber)
            .bind::<BigInt, _>(now_ms)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<ProfileSubscriptionRow>(conn)
            .await?
    };
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn check_profile_subscription_access(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
    subscriber: &str,
    service_id: &str,
    min_tier_level: Option<i64>,
) -> anyhow::Result<bool> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    #[derive(QueryableByName)]
    struct OwnerRow {
        #[diesel(sql_type = Text)]
        profile_owner: String,
    }
    let owner_row = diesel::sql_query(
        "SELECT profile_owner FROM profile_subscription_services WHERE service_id = $1",
    )
    .bind::<Text, _>(service_id)
    .get_result::<OwnerRow>(conn)
    .await
    .optional()?;

    if let Some(owner_row) = owner_row {
        if either_profile_blocked(conn, subscriber, &owner_row.profile_owner).await? {
            metrics.requests_succeeded.inc();
            return Ok(false);
        }
    }

    let now_ms = chrono::Utc::now().timestamp_millis();
    #[derive(QueryableByName)]
    struct SubAccessRow {
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
        .get_result::<SubAccessRow>(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result
        .map(|row| tier_satisfies(row.tier_level, min_tier_level))
        .unwrap_or(false))
}

async fn either_profile_blocked(
    conn: &mut Connection<'_>,
    a: &str,
    b: &str,
) -> anyhow::Result<bool> {
    #[derive(QueryableByName)]
    struct ExistsRow {
        #[diesel(sql_type = Bool)]
        exists: bool,
    }
    let result = diesel::sql_query(
        "SELECT EXISTS(
            SELECT 1 FROM blocked_profiles
            WHERE (blocker_address = $1 AND blocked_address = $2)
               OR (blocker_address = $2 AND blocked_address = $1)
        ) AS exists",
    )
    .bind::<Text, _>(a)
    .bind::<Text, _>(b)
    .get_result::<ExistsRow>(conn)
    .await?;
    Ok(result.exists)
}
