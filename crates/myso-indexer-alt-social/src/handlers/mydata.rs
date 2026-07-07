// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewMyDataAccessLog, NewMyDataBroadPool, NewMyDataClaim, NewMyDataConfig, NewMyDataData,
    NewMyDataDistributionRound, NewMyDataListingSubPool, NewMyDataMerkleRoot, NewMyDataPurchase,
    NewMyDataRegistry, NewMyDataRevenue, NewMyDataSnapshotAnchor, NewMyDataSubPool,
    NewMyDataSubscription, ACCESS_TYPE_REVOKED,
};

pub(crate) fn json_to_i64(v: &serde_json::Value) -> i64 {
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| u.try_into().ok()))
        .unwrap_or(0)
}

pub(crate) fn json_opt_i64(v: &serde_json::Value) -> Option<i64> {
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| u.try_into().ok()))
}

pub(crate) fn json_opt_i64_field(data: &serde_json::Value, key: &str) -> Option<i64> {
    data.get(key).and_then(json_opt_i64)
}

pub(crate) fn json_opt_string_field(data: &serde_json::Value, key: &str) -> Option<String> {
    data.get(key).and_then(|v| v.as_str()).map(String::from)
}

pub(crate) fn json_tags_field(data: &serde_json::Value) -> serde_json::Value {
    data.get("tags")
        .and_then(|v| v.as_array())
        .map(|arr| {
            serde_json::json!(arr
                .iter()
                .filter_map(|t| t.as_str().map(String::from))
                .collect::<Vec<_>>())
        })
        .unwrap_or_else(|| serde_json::json!([]))
}

pub(crate) fn u64_to_db_i64(n: u64) -> i64 {
    i64::try_from(n).unwrap_or(i64::MAX)
}

pub(crate) fn new_mydata_registry_row(
    mydata_id: String,
    owner: String,
    registered_at: i64,
    transaction_id: String,
) -> NewMyDataRegistry {
    NewMyDataRegistry {
        mydata_id,
        owner,
        registered_at,
        unregistered_at: None,
        is_active: true,
        transaction_id,
    }
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
        "AccessRevokedEvent" | "DataAccessRevokedEvent" => {
            process_mydata_access_revoked_event(data, &transaction_id)
        }
        "MyDataConfigUpdatedEvent" | "ConfigUpdatedEvent" => {
            process_mydata_config_updated_event(data, &transaction_id)
        }
        "BroadPoolCreatedEvent" => process_query_broad_pool_created(data, event_id),
        "SubPoolCreatedEvent" => process_query_sub_pool_created(data, event_id),
        "MyDataAssignedToSubPoolEvent" => process_query_listing_sub_pools_assigned(data, event_id),
        "SnapshotAnchorRecordedEvent" => process_query_snapshot_anchor_recorded(data, event_id),
        "DistributionRecordedEvent" => process_query_distribution_recorded(data, event_id),
        "MerkleRootPublishedEvent" => process_query_merkle_root_published(data, event_id),
        "ClaimExecutedEvent" => process_query_claim_executed(data, event_id),
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

    Some(vec![SocialEventRow::MyDataRegistry(
        new_mydata_registry_row(ip_id, owner, registered_at, transaction_id.to_string()),
    )])
}

fn process_mydata_unregistered_event(
    data: &serde_json::Value,
    transaction_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ip_id = data.get("ip_id")?.as_str()?.to_string();
    let owner = data.get("owner")?.as_str()?.to_string();
    let unregistered_at = json_to_i64(data.get("unregistered_at")?);

    Some(vec![SocialEventRow::MyDataRegistryUpdate {
        mydata_id: ip_id,
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
    let platform_id = json_opt_string_field(data, "platform_id");
    let one_time_price = json_opt_i64_field(data, "one_time_price");
    let subscription_price = json_opt_i64_field(data, "subscription_price");
    let created_at = json_to_i64(data.get("created_at")?);

    let new_data = NewMyDataData {
        mydata_id: ip_id.clone(),
        owner: owner.clone(),
        media_type,
        tags: json_tags_field(data),
        platform_id,
        timestamp_start: json_opt_i64_field(data, "timestamp_start").unwrap_or(0),
        timestamp_end: json_opt_i64_field(data, "timestamp_end"),
        created_at,
        last_updated: json_opt_i64_field(data, "last_updated").unwrap_or(created_at),
        one_time_price,
        subscription_price,
        subscription_duration_days: json_opt_i64_field(data, "subscription_duration_days")
            .unwrap_or(30),
        geographic_region: json_opt_string_field(data, "geographic_region"),
        data_quality: json_opt_string_field(data, "data_quality"),
        sample_size: json_opt_i64_field(data, "sample_size"),
        collection_method: json_opt_string_field(data, "collection_method"),
        is_updating: data
            .get("is_updating")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
        update_frequency: json_opt_string_field(data, "update_frequency"),
        version: json_opt_i64_field(data, "version").unwrap_or(1),
        encrypted_content_hash: None,
        transaction_id: transaction_id.to_string(),
    };

    let registry = new_mydata_registry_row(ip_id, owner, created_at, transaction_id.to_string());

    Some(vec![
        SocialEventRow::MyDataData(new_data),
        SocialEventRow::MyDataRegistry(registry),
    ])
}

fn process_mydata_purchase_event(
    data: &serde_json::Value,
    transaction_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ip_id = data.get("ip_id")?.as_str()?.to_string();
    let buyer = data.get("buyer")?.as_str()?.to_string();
    let price = json_to_i64(data.get("price")?);
    let platform_fee = data.get("platform_fee").map(json_to_i64).unwrap_or(0);
    let ecosystem_fee = data.get("ecosystem_fee").map(json_to_i64).unwrap_or(0);
    let creator_amount = data
        .get("creator_amount")
        .map(json_to_i64)
        .unwrap_or(price - platform_fee - ecosystem_fee);
    let platform_id = json_opt_string_field(data, "platform_id");
    let purchase_type = data.get("purchase_type")?.as_str()?.to_string();
    let timestamp = json_to_i64(data.get("timestamp")?);

    let mut rows = Vec::new();

    let organization_id = data
        .get("organization_id")
        .and_then(|v| v.as_str())
        .map(str::to_string);

    let purchase = NewMyDataPurchase {
        mydata_id: ip_id.clone(),
        buyer: buyer.clone(),
        price,
        platform_fee,
        ecosystem_fee,
        creator_amount,
        platform_address: platform_id.clone(),
        purchase_type: purchase_type.clone(),
        purchase_time: timestamp,
        transaction_id: transaction_id.to_string(),
        organization_id,
    };
    rows.push(SocialEventRow::MyDataPurchase(purchase));

    if purchase_type == "subscription" {
        let subscription = NewMyDataSubscription {
            mydata_id: ip_id.clone(),
            subscriber: buyer.clone(),
            subscription_start: timestamp,
            // subscription_end computed at commit from mydata_data.subscription_duration_days
            subscription_end: 0,
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
        platform_fee,
        ecosystem_fee,
        creator_amount,
        platform_address: platform_id,
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
        let subscription = NewMyDataSubscription {
            mydata_id: ip_id,
            subscriber: user,
            subscription_start: timestamp,
            // subscription_end computed at commit from mydata_data.subscription_duration_days
            subscription_end: 0,
            price: 0,
            transaction_id: transaction_id.to_string(),
        };
        rows.push(SocialEventRow::MyDataSubscription(subscription));
    }

    Some(rows)
}

fn process_mydata_access_revoked_event(
    data: &serde_json::Value,
    transaction_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ip_id = data.get("ip_id")?.as_str()?.to_string();
    let user = data.get("user")?.as_str()?.to_string();
    let access_type = data.get("access_type")?.as_str()?.to_string();
    let revoked_by = data.get("revoked_by")?.as_str()?.to_string();
    let timestamp = json_to_i64(data.get("timestamp")?);

    let access_log = NewMyDataAccessLog {
        mydata_id: ip_id.clone(),
        user_address: user.clone(),
        access_type: ACCESS_TYPE_REVOKED.to_string(),
        access_time: timestamp,
        transaction_id: transaction_id.to_string(),
    };

    Some(vec![
        SocialEventRow::MyDataAccessLog(access_log),
        SocialEventRow::MyDataAccessRevoke {
            mydata_id: ip_id,
            user,
            access_type,
            revoked_at: timestamp,
            revoked_by,
            transaction_id: transaction_id.to_string(),
        },
    ])
}

fn process_mydata_config_updated_event(
    data: &serde_json::Value,
    transaction_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let updated_by = data.get("updated_by")?.as_str()?.to_string();
    let marketplace_enabled = data.get("marketplace_enabled")?.as_bool().unwrap_or(false);
    let max_tags = json_to_i64(data.get("max_tags")?);
    let max_subscription_days = json_to_i64(data.get("max_subscription_days")?);
    let max_free_access_grants = json_to_i64(data.get("max_free_access_grants")?);
    let max_encryption_id_bytes = json_to_i64(data.get("max_encryption_id_bytes")?);
    let p2p_platform_fee_bps = data
        .get("p2p_platform_fee_bps")
        .map(json_to_i64)
        .unwrap_or(250);
    let p2p_ecosystem_fee_bps = data
        .get("p2p_ecosystem_fee_bps")
        .map(json_to_i64)
        .unwrap_or(250);
    let mydata_marketplace_platform_fee_bps = data
        .get("mydata_marketplace_platform_fee_bps")
        .map(json_to_i64)
        .unwrap_or(250);
    let mydata_marketplace_ecosystem_fee_bps = data
        .get("mydata_marketplace_ecosystem_fee_bps")
        .map(json_to_i64)
        .unwrap_or(250);
    let non_platform_platform_to_creator_bps = data
        .get("non_platform_platform_to_creator_bps")
        .map(json_to_i64)
        .unwrap_or(0);
    let non_platform_platform_to_treasury_bps = data
        .get("non_platform_platform_to_treasury_bps")
        .map(json_to_i64)
        .unwrap_or(10_000);
    let version = data
        .get("version")
        .and_then(|v| json_opt_i64(v))
        .unwrap_or(0);
    let updated_at = json_to_i64(data.get("timestamp")?);

    let config = NewMyDataConfig {
        updated_by,
        marketplace_enabled,
        max_tags,
        max_subscription_days,
        max_free_access_grants,
        max_encryption_id_bytes,
        p2p_platform_fee_bps,
        p2p_ecosystem_fee_bps,
        mydata_marketplace_platform_fee_bps,
        mydata_marketplace_ecosystem_fee_bps,
        non_platform_platform_to_creator_bps,
        non_platform_platform_to_treasury_bps,
        version,
        updated_at,
        transaction_id: transaction_id.to_string(),
    };

    Some(vec![SocialEventRow::MyDataConfig(config)])
}

fn process_query_broad_pool_created(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let transaction_id = event_id.split(':').next()?.to_string();
    let pool_id = data.get("pool_id")?.as_str()?.to_string();
    let name = data.get("name")?.as_str()?.to_string();
    let created_at_ms = json_to_i64(data.get("created_at")?);
    Some(vec![SocialEventRow::MyDataBroadPool(NewMyDataBroadPool {
        pool_id,
        name,
        created_at_ms,
        event_id: event_id.to_string(),
        transaction_id,
    })])
}

fn process_query_sub_pool_created(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let transaction_id = event_id.split(':').next()?.to_string();
    let sub_pool_id = data.get("sub_pool_id")?.as_str()?.to_string();
    let broad_pool_id = data.get("broad_pool_id")?.as_str()?.to_string();
    let name = data.get("name")?.as_str()?.to_string();
    let created_at_ms = json_to_i64(data.get("created_at")?);
    Some(vec![SocialEventRow::MyDataSubPool(NewMyDataSubPool {
        sub_pool_id,
        broad_pool_id,
        name,
        created_at_ms,
        event_id: event_id.to_string(),
        transaction_id,
    })])
}

fn process_query_listing_sub_pools_assigned(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let transaction_id = event_id.split(':').next()?.to_string();
    let listing_id = data.get("ip_id")?.as_str()?.to_string();
    let assigned_at_ms = json_to_i64(data.get("assigned_at")?);
    let arr = data.get("sub_pool_ids")?.as_array()?;
    let mut rows = Vec::with_capacity(arr.len());
    for v in arr {
        let sub_pool_id = v.as_str()?.to_string();
        rows.push(NewMyDataListingSubPool {
            listing_id: listing_id.clone(),
            sub_pool_id,
            assigned_at_ms,
            event_id: event_id.to_string(),
            transaction_id: transaction_id.clone(),
        });
    }
    Some(vec![SocialEventRow::MyDataListingSubPoolsReplace {
        listing_id,
        rows,
    }])
}

fn process_query_snapshot_anchor_recorded(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let transaction_id = event_id.split(':').next()?.to_string();
    let snapshot_id = data.get("snapshot_id")?.as_str()?.to_string();
    let buyer_address = data.get("buyer_address")?.as_str()?.to_string();
    let price_raw = data.get("price_paid")?;
    let price_paid = price_raw
        .as_i64()
        .or_else(|| price_raw.as_u64().map(u64_to_db_i64))?;
    let created_at_ms = json_to_i64(data.get("created_at")?);
    let manifest_hash = data
        .get("manifest_hash")
        .and_then(|v| v.as_str())
        .map(String::from);
    let payment_reference = data
        .get("payment_reference")
        .and_then(|v| v.as_str())
        .map(String::from);
    Some(vec![SocialEventRow::MyDataSnapshotAnchor(
        NewMyDataSnapshotAnchor {
            snapshot_id,
            buyer_address,
            price_paid,
            created_at_ms,
            event_id: event_id.to_string(),
            transaction_id,
            manifest_hash,
            payment_reference,
        },
    )])
}

fn process_query_distribution_recorded(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let transaction_id = event_id.split(':').next()?.to_string();
    let snapshot_id = data.get("snapshot_id")?.as_str()?.to_string();
    let total_raw = data.get("total_amount")?;
    let total_amount = total_raw
        .as_i64()
        .or_else(|| total_raw.as_u64().map(u64_to_db_i64))?;
    let count_raw = data.get("contributor_count")?;
    let contributor_count = count_raw
        .as_i64()
        .or_else(|| count_raw.as_u64().map(u64_to_db_i64))?;
    let merkle_root = data.get("merkle_root")?.as_str()?.to_string();
    let published_at_ms = json_to_i64(data.get("published_at")?);
    Some(vec![SocialEventRow::MyDataDistributionRound(
        NewMyDataDistributionRound {
            snapshot_id,
            total_amount,
            contributor_count,
            merkle_root,
            published_at_ms,
            event_id: event_id.to_string(),
            transaction_id,
        },
    )])
}

fn process_query_merkle_root_published(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let transaction_id = event_id.split(':').next()?.to_string();
    let snapshot_id = data.get("snapshot_id")?.as_str()?.to_string();
    let root_hash = data.get("root_hash")?.as_str()?.to_string();
    let published_at_ms = json_to_i64(data.get("published_at")?);
    Some(vec![SocialEventRow::MyDataMerkleRoot(
        NewMyDataMerkleRoot {
            snapshot_id,
            root_hash,
            published_at_ms,
            event_id: event_id.to_string(),
            transaction_id,
        },
    )])
}

fn process_query_claim_executed(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let transaction_id = event_id.split(':').next()?.to_string();
    let snapshot_id = data.get("snapshot_id")?.as_str()?.to_string();
    let claimant = data.get("claimant")?.as_str()?.to_string();
    let gross_amount = data
        .get("gross_amount")
        .map(json_to_i64)
        .unwrap_or_else(|| data.get("amount").map(json_to_i64).unwrap_or(0));
    let platform_fee = data.get("platform_fee").map(json_to_i64).unwrap_or(0);
    let ecosystem_fee = data.get("ecosystem_fee").map(json_to_i64).unwrap_or(0);
    let net_amount = data
        .get("net_amount")
        .map(json_to_i64)
        .unwrap_or(gross_amount - platform_fee - ecosystem_fee);
    let platform_id = json_opt_string_field(data, "platform_id");
    let claimed_at_ms = json_to_i64(data.get("claimed_at")?);
    Some(vec![SocialEventRow::MyDataClaim(NewMyDataClaim {
        snapshot_id,
        claimant,
        amount: gross_amount,
        gross_amount,
        platform_fee,
        ecosystem_fee,
        net_amount,
        platform_address: platform_id,
        claimed_at_ms,
        event_id: event_id.to_string(),
        transaction_id,
    })])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::SocialEventRow;

    #[test]
    fn mydata_created_event_populates_metadata_from_json() {
        let data = serde_json::json!({
            "ip_id": "0x0206060712ef2d2a73c0a03a3502ad289907eabe7f7797cb2f4ff9cc01f0932c",
            "owner": "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
            "media_type": "demo:bf-hmac-encrypt-hmac",
            "tags": ["cli-demo"],
            "created_at": 1_000,
            "subscription_duration_days": 45,
            "geographic_region": "US",
            "data_quality": "high",
            "collection_method": "cli",
        });
        let rows = process_mydata_created_event(&data, "tx_digest").expect("rows");
        assert_eq!(rows.len(), 2);
        let SocialEventRow::MyDataData(d) = &rows[0] else {
            panic!("expected MyDataData");
        };
        assert_eq!(d.tags, serde_json::json!(["cli-demo"]));
        assert_eq!(d.geographic_region.as_deref(), Some("US"));
        assert_eq!(d.data_quality.as_deref(), Some("high"));
        assert_eq!(d.collection_method.as_deref(), Some("cli"));
        assert_eq!(d.subscription_duration_days, 45);
        assert!(matches!(&rows[1], SocialEventRow::MyDataRegistry(_)));
    }
}
