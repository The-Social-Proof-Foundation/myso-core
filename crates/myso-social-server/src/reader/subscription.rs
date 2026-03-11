// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Integer, Text};
use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_pg_db::Db;

use crate::error::SocialError;
use crate::reader::types::*;

pub(crate) async fn get_profile_subscription_service(
    db: &Db,
    service_id: &str,
) -> Result<Option<ProfileSubscriptionServiceInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT s.service_id, s.profile_owner, s.profile_id, s.monthly_fee, s.active,
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

pub(crate) async fn get_profile_subscription_services_by_owner(
    db: &Db,
    profile_owner: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProfileSubscriptionServiceInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT s.service_id, s.profile_owner, s.profile_id, s.monthly_fee, s.active,
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
        SELECT sub.subscription_id, sub.service_id, sub.subscriber, sub.created_at,
               sub.expires_at, sub.auto_renew, sub.renewal_balance, sub.renewal_count,
               sub.cancelled_at, s.monthly_fee, s.profile_owner,
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
        SELECT sub.subscription_id, sub.service_id, sub.subscriber, sub.created_at,
               sub.expires_at, sub.auto_renew, sub.renewal_balance, sub.renewal_count,
               sub.cancelled_at, s.monthly_fee, s.profile_owner,
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
) -> Result<bool, SocialError> {
    let mut conn = db.connect().await?;
    let now_ms = chrono::Utc::now().timestamp_millis();
    let query = "
        SELECT 1
        FROM (
            SELECT DISTINCT ON (subscription_id) subscription_id, expires_at, cancelled_at
            FROM profile_subscriptions
            WHERE subscriber = $1 AND service_id = $2
            ORDER BY subscription_id, time DESC
        ) sub
        WHERE sub.cancelled_at IS NULL AND sub.expires_at > $3
        LIMIT 1
    ";
    #[derive(QueryableByName)]
    struct ExistsRow {
        #[diesel(sql_type = Integer)]
        _exists: i32,
    }
    let result = diesel::sql_query(query)
        .bind::<Text, _>(subscriber)
        .bind::<Text, _>(service_id)
        .bind::<BigInt, _>(now_ms)
        .get_result::<ExistsRow>(&mut conn)
        .await
        .optional()?;
    Ok(result.is_some())
}

pub(crate) async fn list_subscription_services(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProfileSubscriptionServiceInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT s.service_id, s.profile_owner, s.profile_id, s.monthly_fee, s.active,
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
                    revenue_type, payment_time, time, transaction_id
             FROM subscription_revenue WHERE service_id = $1
             ORDER BY time DESC LIMIT $2 OFFSET $3",
            true,
        )
    } else {
        (
            "SELECT service_id, subscription_id, from_address, to_address, amount,
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
