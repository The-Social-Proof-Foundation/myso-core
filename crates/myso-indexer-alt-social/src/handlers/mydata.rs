// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewMyDataAccessLog, NewMyDataConfig, NewMyDataData, NewMyDataPurchase, NewMyDataRegistry,
    NewMyDataRevenue, NewMyDataSubscription,
};

fn json_to_i64(v: &serde_json::Value) -> i64 {
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| u.try_into().ok()))
        .unwrap_or(0)
}

fn json_opt_i64(v: &serde_json::Value) -> Option<i64> {
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| u.try_into().ok()))
}

pub fn handle_mydata_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let transaction_id = event_id.split(':').next().unwrap_or(event_id).to_string();
    match event_name {
        "MyDataRegisteredEvent" | "IPRegisteredEvent" => {
            process_mydata_registered_event(data, &transaction_id)
        }
        "MyDataUnregisteredEvent" | "IPUnregisteredEvent" => {
            process_mydata_unregistered_event(data, &transaction_id)
        }
        "MyDataCreatedEvent" | "DataCreatedEvent" => {
            process_mydata_created_event(data, &transaction_id)
        }
        "PurchaseEvent" | "DataPurchasedEvent" => {
            process_mydata_purchase_event(data, &transaction_id)
        }
        "AccessGrantedEvent" | "DataAccessGrantedEvent" => {
            process_mydata_access_granted_event(data, &transaction_id)
        }
        "MyDataConfigUpdatedEvent" | "ConfigUpdatedEvent" => {
            process_mydata_config_updated_event(data, &transaction_id)
        }
        _ => None,
    }
}

fn process_mydata_registered_event(
    data: &serde_json::Value,
    transaction_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ip_id = data.get("ip_id")?.as_str()?.to_string();
    let owner = data.get("owner")?.as_str()?.to_string();
    let registered_at = json_to_i64(data.get("registered_at")?);

    let reg = NewMyDataRegistry {
        ip_id: ip_id.clone(),
        owner: owner.clone(),
        registered_at,
        unregistered_at: None,
        is_active: true,
        transaction_id: transaction_id.to_string(),
    };

    Some(vec![SocialEventRow::MyDataRegistry(reg)])
}

fn process_mydata_unregistered_event(
    data: &serde_json::Value,
    transaction_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ip_id = data.get("ip_id")?.as_str()?.to_string();
    let owner = data.get("owner")?.as_str()?.to_string();
    let unregistered_at = json_to_i64(data.get("unregistered_at")?);

    Some(vec![SocialEventRow::MyDataRegistryUpdate {
        ip_id,
        owner,
        unregistered_at,
        transaction_id: transaction_id.to_string(),
    }])
}

fn process_mydata_created_event(
    data: &serde_json::Value,
    transaction_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ip_id = data.get("ip_id")?.as_str()?.to_string();
    let owner = data.get("owner")?.as_str()?.to_string();
    let media_type = data.get("media_type")?.as_str()?.to_string();
    let platform_id = data
        .get("platform_id")
        .and_then(|v| v.as_str())
        .map(String::from);
    let one_time_price = data.get("one_time_price").and_then(json_opt_i64);
    let subscription_price = data.get("subscription_price").and_then(json_opt_i64);
    let created_at = json_to_i64(data.get("created_at")?);

    let new_data = NewMyDataData {
        mydata_id: ip_id.clone(),
        owner: owner.clone(),
        media_type,
        tags: serde_json::json!([]),
        platform_id,
        timestamp_start: 0,
        timestamp_end: None,
        created_at,
        last_updated: created_at,
        one_time_price,
        subscription_price,
        subscription_duration_days: 30,
        geographic_region: None,
        data_quality: None,
        sample_size: None,
        collection_method: None,
        is_updating: false,
        update_frequency: None,
        version: 1,
        transaction_id: transaction_id.to_string(),
    };

    Some(vec![SocialEventRow::MyDataData(new_data)])
}

fn process_mydata_purchase_event(
    data: &serde_json::Value,
    transaction_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ip_id = data.get("ip_id")?.as_str()?.to_string();
    let buyer = data.get("buyer")?.as_str()?.to_string();
    let price = json_to_i64(data.get("price")?);
    let purchase_type = data.get("purchase_type")?.as_str()?.to_string();
    let timestamp = json_to_i64(data.get("timestamp")?);

    let mut rows = Vec::new();

    let purchase = NewMyDataPurchase {
        mydata_id: ip_id.clone(),
        buyer: buyer.clone(),
        price,
        purchase_type: purchase_type.clone(),
        purchase_time: timestamp,
        transaction_id: transaction_id.to_string(),
    };
    rows.push(SocialEventRow::MyDataPurchase(purchase));

    if purchase_type == "subscription" {
        let subscription_end = timestamp + (30 * 24 * 60 * 60);
        let subscription = NewMyDataSubscription {
            mydata_id: ip_id.clone(),
            subscriber: buyer.clone(),
            subscription_start: timestamp,
            subscription_end,
            price,
            transaction_id: transaction_id.to_string(),
        };
        rows.push(SocialEventRow::MyDataSubscription(subscription));
    }

    let access_log = NewMyDataAccessLog {
        mydata_id: ip_id.clone(),
        user_address: buyer.clone(),
        access_type: purchase_type.clone(),
        access_time: timestamp,
        transaction_id: transaction_id.to_string(),
    };
    rows.push(SocialEventRow::MyDataAccessLog(access_log));

    let revenue = NewMyDataRevenue {
        mydata_id: ip_id.clone(),
        from_address: buyer,
        to_address: String::new(),
        amount: price,
        revenue_type: purchase_type,
        revenue_time: timestamp,
        transaction_id: transaction_id.to_string(),
    };
    rows.push(SocialEventRow::MyDataRevenue(revenue));

    Some(rows)
}

fn process_mydata_access_granted_event(
    data: &serde_json::Value,
    transaction_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ip_id = data.get("ip_id")?.as_str()?.to_string();
    let user = data.get("user")?.as_str()?.to_string();
    let access_type = data.get("access_type")?.as_str()?.to_string();
    let timestamp = json_to_i64(data.get("timestamp")?);

    let mut rows = Vec::new();

    let access_type_for_log = match access_type.as_str() {
        "pricing_update" | "content_update" => "grant",
        _ => access_type.as_str(),
    };

    let access_log = NewMyDataAccessLog {
        mydata_id: ip_id.clone(),
        user_address: user.clone(),
        access_type: access_type_for_log.to_string(),
        access_time: timestamp,
        transaction_id: transaction_id.to_string(),
    };
    rows.push(SocialEventRow::MyDataAccessLog(access_log));

    if access_type == "content_update" {
        rows.push(SocialEventRow::MyDataContentUpdate {
            mydata_id: ip_id,
            last_updated: timestamp,
            transaction_id: transaction_id.to_string(),
        });
    } else if access_type == "subscription" {
        let subscription_end = timestamp + (30 * 24 * 60 * 60);
        let subscription = NewMyDataSubscription {
            mydata_id: ip_id,
            subscriber: user,
            subscription_start: timestamp,
            subscription_end,
            price: 0,
            transaction_id: transaction_id.to_string(),
        };
        rows.push(SocialEventRow::MyDataSubscription(subscription));
    }

    Some(rows)
}

fn process_mydata_config_updated_event(
    data: &serde_json::Value,
    transaction_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let updated_by = data.get("updated_by")?.as_str()?.to_string();
    let enable_flag = data.get("enable_flag")?.as_bool().unwrap_or(false);
    let max_tags = json_to_i64(data.get("max_tags")?);
    let max_subscription_days = json_to_i64(data.get("max_subscription_days")?);
    let max_free_access_grants = json_to_i64(data.get("max_free_access_grants")?);
    let timestamp = json_to_i64(data.get("timestamp")?);

    let config = NewMyDataConfig {
        updated_by,
        enable_flag,
        max_tags,
        max_subscription_days,
        max_free_access_grants,
        timestamp_ms: timestamp,
        transaction_id: transaction_id.to_string(),
    };

    Some(vec![SocialEventRow::MyDataConfig(config)])
}
