// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewProfileSubscription, NewProfileSubscriptionService, NewSubscriptionEvent,
};
use serde::Deserialize;
use serde_json::Value;

const THIRTY_DAYS_MS: i64 = 30 * 24 * 60 * 60 * 1000;

fn generate_subscription_id() -> String {
    format!("sub_{}", uuid::Uuid::new_v4().to_string().replace('-', ""))
}

#[derive(Debug, Deserialize)]
struct ProfileSubscriptionServiceCreatedEvent {
    service_id: String,
    profile_owner: String,
    monthly_fee: u64,
    created_at: u64,
}

#[derive(Debug, Deserialize)]
struct ProfileSubscriptionCreatedEvent {
    service_id: String,
    subscriber: String,
    expires_at: u64,
    monthly_fee: u64,
    auto_renew: bool,
}

#[derive(Debug, Deserialize)]
struct ProfileSubscriptionRenewedEvent {
    subscription_id: String,
    subscriber: String,
    new_expires_at: u64,
    renewal_count: u64,
    auto_renewed: bool,
}

#[derive(Debug, Deserialize)]
struct ProfileSubscriptionCancelledEvent {
    subscription_id: String,
    subscriber: String,
    refunded_amount: u64,
}

#[derive(Debug, Deserialize)]
struct ProfileSubscriptionUpdatedEvent {
    service_id: String,
    old_fee: u64,
    new_fee: u64,
    updated_by: String,
}

#[derive(Debug, Deserialize)]
struct RenewalBalanceFundedEvent {
    subscription_id: String,
    subscriber: String,
    funded_amount: u64,
    new_balance: u64,
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct ProfileSubscriptionServiceDeactivatedEvent {
    service_id: String,
    profile_owner: String,
    deactivated_at: u64,
}

pub fn handle_subscription_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "ProfileSubscriptionServiceCreatedEvent" => {
            process_subscription_service_created_event(data, event_id)
        }
        "ProfileSubscriptionCreatedEvent" => process_subscription_created_event(data, event_id),
        "ProfileSubscriptionRenewedEvent" => process_subscription_renewed_event(data, event_id),
        "ProfileSubscriptionCancelledEvent" => process_subscription_cancelled_event(data, event_id),
        "ProfileSubscriptionUpdatedEvent" => process_subscription_updated_event(data, event_id),
        "RenewalBalanceFundedEvent" => process_renewal_balance_funded_event(data, event_id),
        "ProfileSubscriptionServiceDeactivatedEvent" => {
            process_subscription_service_deactivated_event(data, event_id)
        }
        _ => None,
    }
}

fn process_subscription_service_created_event(
    data: &Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let event: ProfileSubscriptionServiceCreatedEvent =
        serde_json::from_value(data.clone()).ok()?;
    let now = chrono::Utc::now();
    let service = NewProfileSubscriptionService {
        service_id: event.service_id.clone(),
        profile_owner: event.profile_owner.clone(),
        profile_id: event.profile_owner.clone(),
        monthly_fee: event.monthly_fee as i64,
        active: true,
        subscriber_count: 0,
        created_at: event.created_at as i64,
        updated_at: None,
        time: now,
        transaction_id: event_id.to_string(),
    };
    let sub_event = NewSubscriptionEvent {
        event_type: "ProfileSubscriptionServiceCreatedEvent".to_string(),
        subscription_id: None,
        service_id: Some(event.service_id.clone()),
        subscriber: None,
        event_data: data.clone(),
        event_time: event.created_at as i64,
        time: now,
        transaction_id: event_id.to_string(),
        processing_success: true,
        processing_error: None,
    };
    Some(vec![
        SocialEventRow::ProfileSubscriptionService(service),
        SocialEventRow::SubscriptionEvent(sub_event),
    ])
}

fn process_subscription_created_event(data: &Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    let event: ProfileSubscriptionCreatedEvent = serde_json::from_value(data.clone()).ok()?;
    let now = chrono::Utc::now();
    let subscription_id = generate_subscription_id();
    let payment_time = event.expires_at as i64 - THIRTY_DAYS_MS;

    let subscription = NewProfileSubscription {
        subscription_id: subscription_id.clone(),
        service_id: event.service_id.clone(),
        subscriber: event.subscriber.clone(),
        created_at: now.timestamp_millis(),
        expires_at: event.expires_at as i64,
        auto_renew: event.auto_renew,
        renewal_balance: 0,
        renewal_count: 0,
        cancelled_at: None,
        time: now,
        transaction_id: event_id.to_string(),
        processing_success: true,
        processing_error: None,
    };

    let sub_event = NewSubscriptionEvent {
        event_type: "ProfileSubscriptionCreatedEvent".to_string(),
        subscription_id: Some(subscription_id.clone()),
        service_id: Some(event.service_id.clone()),
        subscriber: Some(event.subscriber.clone()),
        event_data: data.clone(),
        event_time: payment_time,
        time: now,
        transaction_id: event_id.to_string(),
        processing_success: true,
        processing_error: None,
    };

    let mut rows = vec![
        SocialEventRow::ProfileSubscription(subscription),
        SocialEventRow::ProfileSubscriptionServiceSubscriberIncrement {
            service_id: event.service_id.clone(),
        },
        SocialEventRow::SubscriptionEvent(sub_event),
    ];

    rows.push(SocialEventRow::SubscriptionRevenueFromCreated {
        service_id: event.service_id.clone(),
        subscription_id,
        from_address: event.subscriber,
        amount: event.monthly_fee as i64,
        revenue_type: "subscription".to_string(),
        payment_time,
        transaction_id: event_id.to_string(),
    });

    Some(rows)
}

fn process_subscription_renewed_event(data: &Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    let event: ProfileSubscriptionRenewedEvent = serde_json::from_value(data.clone()).ok()?;
    let now = chrono::Utc::now();

    let sub_event = NewSubscriptionEvent {
        event_type: "ProfileSubscriptionRenewedEvent".to_string(),
        subscription_id: Some(event.subscription_id.clone()),
        service_id: None,
        subscriber: Some(event.subscriber.clone()),
        event_data: data.clone(),
        event_time: event.new_expires_at as i64,
        time: now,
        transaction_id: event_id.to_string(),
        processing_success: true,
        processing_error: None,
    };

    let mut rows = vec![
        SocialEventRow::ProfileSubscriptionUpdate {
            subscription_id: event.subscription_id.clone(),
            expires_at: event.new_expires_at as i64,
            renewal_count: event.renewal_count as i64,
        },
        SocialEventRow::SubscriptionEvent(sub_event),
    ];

    rows.push(SocialEventRow::SubscriptionRevenueFromRenewal {
        subscription_id: event.subscription_id,
        subscriber: event.subscriber,
        new_expires_at: event.new_expires_at as i64,
        renewal_count: event.renewal_count as i64,
        auto_renewed: event.auto_renewed,
        transaction_id: event_id.to_string(),
    });

    Some(rows)
}

fn process_subscription_cancelled_event(
    data: &Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let event: ProfileSubscriptionCancelledEvent = serde_json::from_value(data.clone()).ok()?;
    let now = chrono::Utc::now();

    let sub_event = NewSubscriptionEvent {
        event_type: "ProfileSubscriptionCancelledEvent".to_string(),
        subscription_id: Some(event.subscription_id.clone()),
        service_id: None,
        subscriber: Some(event.subscriber.clone()),
        event_data: data.clone(),
        event_time: now.timestamp_millis(),
        time: now,
        transaction_id: event_id.to_string(),
        processing_success: true,
        processing_error: None,
    };

    let mut rows = vec![
        SocialEventRow::ProfileSubscriptionCancel {
            subscription_id: event.subscription_id.clone(),
        },
        SocialEventRow::ProfileSubscriptionServiceSubscriberDecrementBySubscription {
            subscription_id: event.subscription_id.clone(),
        },
        SocialEventRow::SubscriptionEvent(sub_event),
    ];

    if event.refunded_amount > 0 {
        rows.push(SocialEventRow::SubscriptionRevenueFromRefund {
            subscription_id: event.subscription_id,
            subscriber: event.subscriber,
            refunded_amount: event.refunded_amount as i64,
            transaction_id: event_id.to_string(),
        });
    }

    Some(rows)
}

fn process_subscription_updated_event(data: &Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    let event: ProfileSubscriptionUpdatedEvent = serde_json::from_value(data.clone()).ok()?;
    let now = chrono::Utc::now();

    let sub_event = NewSubscriptionEvent {
        event_type: "ProfileSubscriptionUpdatedEvent".to_string(),
        subscription_id: None,
        service_id: Some(event.service_id.clone()),
        subscriber: None,
        event_data: serde_json::json!({
            "service_id": event.service_id,
            "old_fee": event.old_fee,
            "new_fee": event.new_fee,
            "updated_by": event.updated_by,
            "updated_at": now.timestamp_millis(),
        }),
        event_time: now.timestamp_millis(),
        time: now,
        transaction_id: event_id.to_string(),
        processing_success: true,
        processing_error: None,
    };

    Some(vec![
        SocialEventRow::ProfileSubscriptionServiceUpdate {
            service_id: event.service_id,
            monthly_fee: event.new_fee as i64,
            updated_at: now.timestamp_millis(),
        },
        SocialEventRow::SubscriptionEvent(sub_event),
    ])
}

fn process_renewal_balance_funded_event(
    data: &Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let event: RenewalBalanceFundedEvent = serde_json::from_value(data.clone()).ok()?;
    let now = chrono::Utc::now();

    let sub_event = NewSubscriptionEvent {
        event_type: "RenewalBalanceFundedEvent".to_string(),
        subscription_id: Some(event.subscription_id.clone()),
        service_id: None,
        subscriber: Some(event.subscriber.clone()),
        event_data: serde_json::json!({
            "subscription_id": event.subscription_id,
            "subscriber": event.subscriber,
            "funded_amount": event.funded_amount,
            "new_balance": event.new_balance,
            "timestamp": event.timestamp,
        }),
        event_time: event.timestamp as i64,
        time: now,
        transaction_id: event_id.to_string(),
        processing_success: true,
        processing_error: None,
    };

    Some(vec![
        SocialEventRow::ProfileSubscriptionRenewalBalanceUpdate {
            subscription_id: event.subscription_id,
            new_balance: event.new_balance as i64,
        },
        SocialEventRow::SubscriptionEvent(sub_event),
    ])
}

fn process_subscription_service_deactivated_event(
    data: &Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let event: ProfileSubscriptionServiceDeactivatedEvent =
        serde_json::from_value(data.clone()).ok()?;
    let now = chrono::Utc::now();

    let sub_event = NewSubscriptionEvent {
        event_type: "ProfileSubscriptionServiceDeactivatedEvent".to_string(),
        subscription_id: None,
        service_id: Some(event.service_id.clone()),
        subscriber: None,
        event_data: serde_json::json!({
            "service_id": event.service_id,
            "profile_owner": event.profile_owner,
            "deactivated_at": event.deactivated_at,
        }),
        event_time: event.deactivated_at as i64,
        time: now,
        transaction_id: event_id.to_string(),
        processing_success: true,
        processing_error: None,
    };

    Some(vec![
        SocialEventRow::ProfileSubscriptionServiceDeactivate {
            service_id: event.service_id,
            updated_at: event.deactivated_at as i64,
        },
        SocialEventRow::SubscriptionEvent(sub_event),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_subscription_created_event_produces_rows() {
        let data = serde_json::json!({
            "service_id": "0xabc",
            "subscriber": "0xdef",
            "expires_at": 1735689600000i64,
            "monthly_fee": 100,
            "auto_renew": true
        });
        let rows = handle_subscription_event("ProfileSubscriptionCreatedEvent", &data, "tx123");
        assert!(rows.is_some());
        let rows = rows.unwrap();
        assert!(!rows.is_empty());
        let has_subscription = rows
            .iter()
            .any(|r| matches!(r, SocialEventRow::ProfileSubscription(_)));
        assert!(has_subscription);
        let has_sub_event = rows
            .iter()
            .any(|r| matches!(r, SocialEventRow::SubscriptionEvent(_)));
        assert!(has_sub_event);
    }

    #[test]
    fn test_handle_subscription_service_created_event() {
        let data = serde_json::json!({
            "service_id": "0xsvc",
            "profile_owner": "0xowner",
            "monthly_fee": 500,
            "created_at": 1234567890
        });
        let rows =
            handle_subscription_event("ProfileSubscriptionServiceCreatedEvent", &data, "tx456");
        assert!(rows.is_some());
        let rows = rows.unwrap();
        assert_eq!(rows.len(), 2);
    }
}
