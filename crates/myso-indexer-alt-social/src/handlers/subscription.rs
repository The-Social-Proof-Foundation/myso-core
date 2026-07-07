// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::common;
use super::subscription_object::SubscriptionCreateContext;
use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewProfileSubscription, NewProfileSubscriptionService, NewSubscriptionConfig,
    NewSubscriptionEvent, THIRTY_DAYS_MS,
};
use serde::Deserialize;
use serde_json::Value;

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
    #[serde(default)]
    platform_fee: u64,
    #[serde(default)]
    ecosystem_fee: u64,
    #[serde(default)]
    creator_amount: u64,
    platform_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProfileSubscriptionRenewedEvent {
    subscription_id: String,
    subscriber: String,
    new_expires_at: u64,
    renewal_count: u64,
    auto_renewed: bool,
    #[serde(default)]
    platform_fee: u64,
    #[serde(default)]
    ecosystem_fee: u64,
    #[serde(default)]
    creator_amount: u64,
    platform_id: Option<String>,
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
    checkpoint_timestamp_ms: u64,
    create_context: Option<&SubscriptionCreateContext>,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "ProfileSubscriptionServiceCreatedEvent" => {
            process_subscription_service_created_event(data, event_id, checkpoint_timestamp_ms)
        }
        "ProfileSubscriptionCreatedEvent" => process_subscription_created_event(
            data,
            event_id,
            checkpoint_timestamp_ms,
            create_context,
        ),
        "ProfileSubscriptionRenewedEvent" => {
            process_subscription_renewed_event(data, event_id, checkpoint_timestamp_ms)
        }
        "ProfileSubscriptionCancelledEvent" => {
            process_subscription_cancelled_event(data, event_id, checkpoint_timestamp_ms)
        }
        "ProfileSubscriptionUpdatedEvent" => {
            process_subscription_updated_event(data, event_id, checkpoint_timestamp_ms)
        }
        "RenewalBalanceFundedEvent" => {
            process_renewal_balance_funded_event(data, event_id, checkpoint_timestamp_ms)
        }
        "ProfileSubscriptionServiceDeactivatedEvent" => {
            process_subscription_service_deactivated_event(data, event_id, checkpoint_timestamp_ms)
        }
        "SubscriptionConfigUpdatedEvent" => {
            process_subscription_config_updated_event(data, event_id, checkpoint_timestamp_ms)
        }
        _ => None,
    }
}

fn process_subscription_service_created_event(
    data: &Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let event: ProfileSubscriptionServiceCreatedEvent = common::deserialize_social_event_json(
        "subscription",
        "ProfileSubscriptionServiceCreatedEvent",
        event_id,
        data,
        "subscription ProfileSubscriptionServiceCreatedEvent JSON did not match struct",
    )?;
    let ms = common::chain_timestamp_ms(Some(event.created_at as i64), checkpoint_timestamp_ms);
    let now = common::chain_time_from_ms(ms);
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

fn process_subscription_created_event(
    data: &Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
    create_context: Option<&SubscriptionCreateContext>,
) -> Option<Vec<SocialEventRow>> {
    let event: ProfileSubscriptionCreatedEvent = common::deserialize_social_event_json(
        "subscription",
        "ProfileSubscriptionCreatedEvent",
        event_id,
        data,
        "subscription ProfileSubscriptionCreatedEvent JSON did not match struct",
    )?;
    let ctx = create_context?;
    let subscription_id = ctx.subscription_id.clone();
    let ms = common::chain_timestamp_ms(
        Some(ctx.created_at_ms),
        checkpoint_timestamp_ms,
    );
    let now = common::chain_time_from_ms(ms);
    let billing_period_ms = common::json_field_as_i64(data.get("billing_period_ms"))
        .unwrap_or(THIRTY_DAYS_MS);
    let payment_time = event.expires_at as i64 - billing_period_ms;

    let subscription = NewProfileSubscription {
        subscription_id: subscription_id.clone(),
        service_id: event.service_id.clone(),
        subscriber: event.subscriber.clone(),
        created_at: ms,
        expires_at: event.expires_at as i64,
        auto_renew: event.auto_renew,
        renewal_balance: ctx.renewal_balance as i64,
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
        platform_fee: event.platform_fee as i64,
        ecosystem_fee: event.ecosystem_fee as i64,
        creator_amount: if event.creator_amount > 0 {
            event.creator_amount as i64
        } else {
            event.monthly_fee as i64
                - event.platform_fee as i64
                - event.ecosystem_fee as i64
        },
        platform_address: event.platform_id.clone(),
        revenue_type: "subscription".to_string(),
        payment_time,
        transaction_id: event_id.to_string(),
    });

    Some(rows)
}

fn process_subscription_renewed_event(
    data: &Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let event: ProfileSubscriptionRenewedEvent = common::deserialize_social_event_json(
        "subscription",
        "ProfileSubscriptionRenewedEvent",
        event_id,
        data,
        "subscription ProfileSubscriptionRenewedEvent JSON did not match struct",
    )?;
    let ms = common::chain_timestamp_ms(Some(event.new_expires_at as i64), checkpoint_timestamp_ms);
    let now = common::chain_time_from_ms(ms);

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
        platform_fee: event.platform_fee as i64,
        ecosystem_fee: event.ecosystem_fee as i64,
        creator_amount: event.creator_amount as i64,
        platform_address: event.platform_id.clone(),
        transaction_id: event_id.to_string(),
    });

    Some(rows)
}

fn process_subscription_cancelled_event(
    data: &Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let event: ProfileSubscriptionCancelledEvent = common::deserialize_social_event_json(
        "subscription",
        "ProfileSubscriptionCancelledEvent",
        event_id,
        data,
        "subscription ProfileSubscriptionCancelledEvent JSON did not match struct",
    )?;
    let ms = common::chain_timestamp_ms(
        common::json_field_as_i64(data.get("cancelled_at")),
        checkpoint_timestamp_ms,
    );
    let now = common::chain_time_from_ms(ms);

    let sub_event = NewSubscriptionEvent {
        event_type: "ProfileSubscriptionCancelledEvent".to_string(),
        subscription_id: Some(event.subscription_id.clone()),
        service_id: None,
        subscriber: Some(event.subscriber.clone()),
        event_data: data.clone(),
        event_time: ms,
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

fn process_subscription_updated_event(
    data: &Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let event: ProfileSubscriptionUpdatedEvent = common::deserialize_social_event_json(
        "subscription",
        "ProfileSubscriptionUpdatedEvent",
        event_id,
        data,
        "subscription ProfileSubscriptionUpdatedEvent JSON did not match struct",
    )?;
    let ms = common::chain_timestamp_ms(
        common::json_field_as_i64(data.get("updated_at")),
        checkpoint_timestamp_ms,
    );
    let now = common::chain_time_from_ms(ms);

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
            "updated_at": ms,
        }),
        event_time: ms,
        time: now,
        transaction_id: event_id.to_string(),
        processing_success: true,
        processing_error: None,
    };

    Some(vec![
        SocialEventRow::ProfileSubscriptionServiceUpdate {
            service_id: event.service_id,
            monthly_fee: event.new_fee as i64,
            updated_at: ms,
        },
        SocialEventRow::SubscriptionEvent(sub_event),
    ])
}

fn process_renewal_balance_funded_event(
    data: &Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let event: RenewalBalanceFundedEvent = common::deserialize_social_event_json(
        "subscription",
        "RenewalBalanceFundedEvent",
        event_id,
        data,
        "subscription RenewalBalanceFundedEvent JSON did not match struct",
    )?;
    let ms = common::chain_timestamp_ms(Some(event.timestamp as i64), checkpoint_timestamp_ms);
    let now = common::chain_time_from_ms(ms);

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
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let event: ProfileSubscriptionServiceDeactivatedEvent = common::deserialize_social_event_json(
        "subscription",
        "ProfileSubscriptionServiceDeactivatedEvent",
        event_id,
        data,
        "subscription ProfileSubscriptionServiceDeactivatedEvent JSON did not match struct",
    )?;
    let ms = common::chain_timestamp_ms(Some(event.deactivated_at as i64), checkpoint_timestamp_ms);
    let now = common::chain_time_from_ms(ms);

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

#[derive(Debug, Deserialize)]
struct SubscriptionConfigUpdatedEvent {
    updated_by: String,
    billing_period_ms: u64,
    max_renewal_months: u64,
    #[serde(default)]
    platform_fee_bps: u64,
    #[serde(default)]
    ecosystem_fee_bps: u64,
    #[serde(default)]
    non_platform_platform_to_creator_bps: u64,
    #[serde(default)]
    non_platform_platform_to_treasury_bps: u64,
    timestamp: u64,
}

fn process_subscription_config_updated_event(
    data: &Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let ev: SubscriptionConfigUpdatedEvent = common::deserialize_social_event_json(
        "subscription",
        "SubscriptionConfigUpdatedEvent",
        event_id,
        data,
        "subscription SubscriptionConfigUpdatedEvent JSON did not match struct",
    )?;
    let ms = common::chain_timestamp_ms(Some(ev.timestamp as i64), checkpoint_timestamp_ms);
    let row = NewSubscriptionConfig::from_event(
        ev.updated_by,
        ev.billing_period_ms,
        ev.max_renewal_months,
        ev.platform_fee_bps,
        ev.ecosystem_fee_bps,
        ev.non_platform_platform_to_creator_bps,
        ev.non_platform_platform_to_treasury_bps,
        0,
        ms as u64,
        event_id.to_string(),
    );
    Some(vec![SocialEventRow::SubscriptionConfig(row)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::subscription_object::SubscriptionCreateContext;

    fn sample_create_context() -> SubscriptionCreateContext {
        SubscriptionCreateContext {
            subscription_id: "0xsub123".to_string(),
            renewal_balance: 1_000_000_000,
            created_at_ms: 1_700_000_000_000,
        }
    }

    #[test]
    fn test_handle_subscription_created_event_produces_rows() {
        let data = serde_json::json!({
            "service_id": "0xabc",
            "subscriber": "0xdef",
            "expires_at": 1735689600000i64,
            "monthly_fee": 100,
            "auto_renew": true,
            "platform_fee": 10,
            "ecosystem_fee": 5,
            "creator_amount": 85
        });
        let rows = handle_subscription_event(
            "ProfileSubscriptionCreatedEvent",
            &data,
            "tx123",
            1_700_000_000_000,
            Some(&sample_create_context()),
        );
        assert!(rows.is_some());
        let rows = rows.unwrap();
        assert!(!rows.is_empty());
        let has_subscription = rows.iter().any(|r| {
            if let SocialEventRow::ProfileSubscription(s) = r {
                s.subscription_id == "0xsub123" && s.renewal_balance == 1_000_000_000
            } else {
                false
            }
        });
        assert!(has_subscription);
        let has_sub_event = rows
            .iter()
            .any(|r| matches!(r, SocialEventRow::SubscriptionEvent(_)));
        assert!(has_sub_event);
    }

    #[test]
    fn test_handle_subscription_created_event_skips_without_context() {
        let data = serde_json::json!({
            "service_id": "0xabc",
            "subscriber": "0xdef",
            "expires_at": 1735689600000i64,
            "monthly_fee": 100,
            "auto_renew": true
        });
        let rows = handle_subscription_event(
            "ProfileSubscriptionCreatedEvent",
            &data,
            "tx123",
            1_700_000_000_000,
            None,
        );
        assert!(rows.is_none());
    }

    #[test]
    fn test_handle_subscription_renewed_updates_same_subscription_id() {
        let create_data = serde_json::json!({
            "service_id": "0xabc",
            "subscriber": "0xdef",
            "expires_at": 1735689600000i64,
            "monthly_fee": 100,
            "auto_renew": true
        });
        let create_rows = handle_subscription_event(
            "ProfileSubscriptionCreatedEvent",
            &create_data,
            "tx_create",
            1_700_000_000_000,
            Some(&sample_create_context()),
        )
        .unwrap();
        let sub_id = match &create_rows[0] {
            SocialEventRow::ProfileSubscription(s) => s.subscription_id.clone(),
            _ => panic!("expected subscription row"),
        };

        let renew_data = serde_json::json!({
            "subscription_id": sub_id,
            "subscriber": "0xdef",
            "new_expires_at": 1738281600000i64,
            "renewal_count": 1,
            "auto_renewed": false
        });
        let renew_rows = handle_subscription_event(
            "ProfileSubscriptionRenewedEvent",
            &renew_data,
            "tx_renew",
            1_700_000_000_000,
            None,
        )
        .unwrap();
        let update = renew_rows
            .iter()
            .find_map(|r| {
                if let SocialEventRow::ProfileSubscriptionUpdate {
                    subscription_id,
                    ..
                } = r
                {
                    Some(subscription_id.clone())
                } else {
                    None
                }
            })
            .unwrap();
        assert_eq!(update, sub_id);
    }

    #[test]
    fn test_handle_subscription_service_created_event() {
        let data = serde_json::json!({
            "service_id": "0xsvc",
            "profile_owner": "0xowner",
            "monthly_fee": 500,
            "created_at": 1234567890
        });
        let rows = handle_subscription_event(
            "ProfileSubscriptionServiceCreatedEvent",
            &data,
            "tx456",
            1_700_000_000_000,
            None,
        );
        assert!(rows.is_some());
        let rows = rows.unwrap();
        assert_eq!(rows.len(), 2);
    }
}
