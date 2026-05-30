// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Index `mydata::MyData` shared objects from checkpoint `object_set` (source of truth for metadata).

use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use myso_indexer_alt_framework::types::full_checkpoint_content::{
    Checkpoint, ExecutedTransaction, ObjectSet,
};
use myso_indexer_alt_social_schema::models::NewMyDataData;
use myso_types::collection_types::Table;
use myso_types::id::UID;
use myso_types::storage::ObjectKey;
use myso_types::storage::WriteKind;
use myso_types::MYSO_SOCIAL_ADDRESS;
use serde::{Deserialize, Serialize};

use super::mydata::{new_mydata_registry_row, u64_to_db_i64};
use crate::handlers::mydata_handler::MyDataRow;
use crate::metrics::SocialMetrics;

/// BCS layout for `social_contracts::mydata::MyData` (field order must match Move).
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BcsMyData {
    _id: UID,
    owner: AccountAddress,
    media_type: String,
    tags: Vec<String>,
    platform_id: Option<AccountAddress>,
    timestamp_start: u64,
    timestamp_end: Option<u64>,
    created_at: u64,
    last_updated: u64,
    _encrypted_data: Vec<u8>,
    _encryption_id: Vec<u8>,
    one_time_price: Option<u64>,
    subscription_price: Option<u64>,
    subscription_duration_days: u64,
    _purchasers: Table,
    _subscribers: Table,
    geographic_region: Option<String>,
    data_quality: Option<String>,
    sample_size: Option<u64>,
    collection_method: Option<String>,
    is_updating: bool,
    update_frequency: Option<String>,
    version: u64,
}

fn addr_to_string(addr: &AccountAddress) -> String {
    format!("0x{}", hex::encode(addr))
}

pub(crate) fn parse_mydata_object_contents(contents: &[u8]) -> Result<BcsMyData, bcs::Error> {
    bcs::from_bytes(contents)
}

pub(crate) fn bcs_mydata_to_new_row(
    mydata: &BcsMyData,
    mydata_id: String,
    transaction_id: String,
) -> NewMyDataData {
    NewMyDataData {
        mydata_id,
        owner: addr_to_string(&mydata.owner),
        media_type: mydata.media_type.clone(),
        tags: serde_json::json!(mydata.tags),
        platform_id: mydata.platform_id.as_ref().map(addr_to_string),
        timestamp_start: u64_to_db_i64(mydata.timestamp_start),
        timestamp_end: mydata.timestamp_end.map(u64_to_db_i64),
        created_at: u64_to_db_i64(mydata.created_at),
        last_updated: u64_to_db_i64(mydata.last_updated),
        one_time_price: mydata.one_time_price.map(u64_to_db_i64),
        subscription_price: mydata.subscription_price.map(u64_to_db_i64),
        subscription_duration_days: u64_to_db_i64(mydata.subscription_duration_days),
        geographic_region: mydata.geographic_region.clone(),
        data_quality: mydata.data_quality.clone(),
        sample_size: mydata.sample_size.map(u64_to_db_i64),
        collection_method: mydata.collection_method.clone(),
        is_updating: mydata.is_updating,
        update_frequency: mydata.update_frequency.clone(),
        version: u64_to_db_i64(mydata.version),
        transaction_id,
    }
}

fn is_mydata_object_type(type_address: &AccountAddress, module: &str, name: &str) -> bool {
    type_address == &MYSO_SOCIAL_ADDRESS
        && module == ident_str!("mydata").as_str()
        && name == ident_str!("MyData").as_str()
}

/// Walk changed objects in `tx` and emit `MyDataData` (+ registry on create) rows from object BCS.
pub(crate) fn process_mydata_objects_from_tx(
    object_set: &ObjectSet,
    tx: &ExecutedTransaction,
) -> Vec<MyDataRow> {
    let tx_digest = tx.transaction.digest().to_string();
    let mut rows = Vec::new();

    for ((oid, version, _), _owner, write_kind) in tx.effects.all_changed_objects() {
        let Some(obj) = object_set.get(&ObjectKey(oid, version)) else {
            continue;
        };
        let Some(t) = obj.type_() else {
            continue;
        };
        if !is_mydata_object_type(&t.address(), t.module().as_str(), t.name().as_str()) {
            continue;
        };
        let Some(move_obj) = obj.as_inner().data.try_as_move() else {
            continue;
        };
        let contents = move_obj.contents();
        let parsed = match parse_mydata_object_contents(contents) {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!(
                    tx_digest = %tx_digest,
                    object_id = %oid,
                    error = %e,
                    "mydata pipeline: failed to parse MyData object BCS"
                );
                SocialMetrics::record_event_bcs_parse_failed("mydata", "MyDataObject");
                continue;
            }
        };

        let mydata_id = oid.to_string();
        let data_row = bcs_mydata_to_new_row(&parsed, mydata_id.clone(), tx_digest.clone());
        rows.push(MyDataRow::MyDataData(data_row));

        if matches!(write_kind, WriteKind::Create | WriteKind::Unwrap) {
            rows.push(MyDataRow::MyDataRegistry(new_mydata_registry_row(
                mydata_id,
                addr_to_string(&parsed.owner),
                u64_to_db_i64(parsed.created_at),
                tx_digest.clone(),
            )));
        }
    }

    rows
}

pub(crate) fn process_mydata_objects_from_checkpoint(checkpoint: &Checkpoint) -> Vec<MyDataRow> {
    let mut rows = Vec::new();
    for tx in &checkpoint.transactions {
        rows.extend(process_mydata_objects_from_tx(&checkpoint.object_set, tx));
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use myso_types::base_types::ObjectID;
    fn sample_bcs_mydata() -> BcsMyData {
        BcsMyData {
            _id: UID::new(ObjectID::random()),
            owner: AccountAddress::ZERO,
            media_type: "demo:bf-hmac-encrypt-hmac".to_string(),
            tags: vec!["cli-demo".to_string()],
            platform_id: None,
            timestamp_start: 1_779_182_444,
            timestamp_end: Some(1_779_186_444),
            created_at: 1_000,
            last_updated: 1_000,
            _encrypted_data: vec![1, 2, 3],
            _encryption_id: vec![4, 5, 6],
            one_time_price: Some(1_000_000_000),
            subscription_price: Some(5_000_000_000),
            subscription_duration_days: 30,
            _purchasers: Table::default(),
            _subscribers: Table::default(),
            geographic_region: Some("US".to_string()),
            data_quality: Some("high".to_string()),
            sample_size: None,
            collection_method: Some("cli".to_string()),
            is_updating: false,
            update_frequency: None,
            version: 2,
        }
    }

    #[test]
    fn bcs_mydata_roundtrip() {
        let original = sample_bcs_mydata();
        let bytes = bcs::to_bytes(&original).unwrap();
        let decoded: BcsMyData = bcs::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.media_type, original.media_type);
        assert_eq!(decoded.tags, original.tags);
        assert_eq!(decoded.geographic_region, Some("US".to_string()));
        assert_eq!(decoded.data_quality, Some("high".to_string()));
        assert_eq!(decoded.collection_method, Some("cli".to_string()));
        assert_eq!(decoded.subscription_duration_days, 30);
    }

    #[test]
    fn bcs_mydata_maps_to_new_row() {
        let mydata = sample_bcs_mydata();
        let row = bcs_mydata_to_new_row(&mydata, "0xabc".to_string(), "digest".to_string());
        assert_eq!(row.mydata_id, "0xabc");
        assert_eq!(row.media_type, "demo:bf-hmac-encrypt-hmac");
        assert_eq!(row.tags, serde_json::json!(["cli-demo"]));
        assert_eq!(row.geographic_region.as_deref(), Some("US"));
        assert_eq!(row.data_quality.as_deref(), Some("high"));
        assert_eq!(row.collection_method.as_deref(), Some("cli"));
        assert_eq!(row.subscription_duration_days, 30);
        assert_eq!(row.timestamp_start, 1_779_182_444);
        assert_eq!(row.timestamp_end, Some(1_779_186_444));
        assert!(!row.is_updating);
    }
}
