// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::error::{BridgeError, BridgeResult};
use crate::types::{BridgeAction, BridgeActionDigest};
use alloy::primitives::Address as AlloyAddress;
use myso_types::base_types::MySoAddress;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use myso_types::Identifier;
use myso_types::event::EventID;
use typed_store::DBMapUtils;
use typed_store::Map;
use typed_store::rocks::{DBMap, MetricConf};

// ========== Deposit Address Types ==========

/// Registration type for deposit addresses
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegistrationType {
    /// Registered via API with MySo signature
    ApiMySoSig,
    /// Registered via API with Ethereum signature
    ApiEthSig,
    /// Linked with both signatures (Option B)
    Linked,
}

/// Information about a deposit address registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRegistration {
    /// Chain where deposit address exists
    pub deposit_chain: u8,
    /// The deposit address itself
    pub deposit_address: Vec<u8>,
    /// Chain where tokens should be bridged to
    pub destination_chain: u8,
    /// Address where tokens should be sent on destination chain
    pub destination_address: Vec<u8>,
    /// HD wallet derivation index
    pub hd_index: u32,
    /// How this was registered
    pub registration_type: RegistrationType,
    /// When this was created (milliseconds since epoch)
    pub created_at: u64,
    /// Last time this deposit address was used
    pub last_used: Option<u64>,
}

/// Key for deposit address lookups (can be EVM or MySo address)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct DepositAddressKey {
    pub address: Vec<u8>,
}

impl DepositAddressKey {
    pub fn from_evm(addr: AlloyAddress) -> Self {
        Self {
            address: addr.as_slice().to_vec(),
        }
    }

    pub fn from_myso(addr: MySoAddress) -> Self {
        Self {
            address: addr.to_vec(),
        }
    }
}

/// Recipient information for a deposit address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientInfo {
    /// Chain where deposit address exists
    pub deposit_chain: u8,
    /// Chain where tokens should go
    pub destination_chain: u8,
    /// Address where tokens should be sent
    pub destination_address: Vec<u8>,
    /// HD wallet index for this deposit address
    pub hd_index: u32,
    /// Who created this registration
    pub source_address: Vec<u8>,
    /// Registration type
    pub registration_type: RegistrationType,
}

/// Key for tracking processed deposits
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct DepositTxKey {
    pub chain_id: u8,
    pub tx_hash_prefix: u64, // First 8 bytes of tx hash
    pub log_index: u16,
}

impl DepositTxKey {
    pub fn from_evm(tx_hash: alloy::primitives::B256, log_index: u16, chain_id: u8) -> Self {
        let prefix_bytes = tx_hash.as_slice()[0..8].try_into().unwrap();
        Self {
            chain_id,
            tx_hash_prefix: u64::from_be_bytes(prefix_bytes),
            log_index,
        }
    }

    pub fn from_myso(
        tx_digest: myso_types::digests::TransactionDigest,
        chain_id: u8,
        balance_change_index: u16,
    ) -> Self {
        let digest_bytes = tx_digest.inner();
        let mut prefix_bytes = [0u8; 8];
        prefix_bytes.copy_from_slice(&digest_bytes[0..8]);
        Self {
            chain_id,
            tx_hash_prefix: u64::from_be_bytes(prefix_bytes),
            log_index: balance_change_index,
        }
    }
}

/// Record of a processed deposit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRecord {
    pub bridge_tx_hash: String,
    pub processed_at: u64,
    pub amount: String,
}

/// Status of a relayed transfer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelayResult {
    /// Transfer was successfully relayed
    Success,
    /// Transfer relay failed (will retry)
    Failed,
    /// Transfer relay is in progress
    Pending,
}

/// Information about a relayed transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayStatus {
    pub relayed_at: u64,
    pub tx_digest: String,
    pub status: RelayResult,
    pub error: Option<String>,
    pub retry_count: u8,
}

/// Key for tracking relayed transfers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct RelayKey {
    pub source_chain: u8,
    pub seq_num: u64,
}

impl RelayKey {
    pub fn new(source_chain: u8, seq_num: u64) -> Self {
        Self {
            source_chain,
            seq_num,
        }
    }
}

#[derive(DBMapUtils)]
pub struct BridgeOrchestratorTables {
    /// pending BridgeActions that orchestrator received but not yet executed
    pub(crate) pending_actions: DBMap<BridgeActionDigest, BridgeAction>,
    /// module identifier to the last processed EventID
    pub(crate) myso_syncer_cursors: DBMap<Identifier, EventID>,
    /// contract address to the last processed block
    pub(crate) eth_syncer_cursors: DBMap<AlloyAddressSerializedAsEthers, u64>,
    /// sequence number for the next record to be processed from the bridge records table
    pub(crate) myso_syncer_sequence_number_cursor: DBMap<(), u64>,
    /// Track which transfers have been relayed to avoid duplicates
    #[allow(dead_code)] // Used by relay and deposit_api when wired
    pub(crate) relayed_transfers: DBMap<RelayKey, RelayStatus>,
    /// Deposit address registrations (source address → deposit registrations)
    #[allow(dead_code)] // Used by deposit_api when wired
    pub(crate) deposit_registrations: DBMap<DepositAddressKey, Vec<DepositRegistration>>,
    /// Reverse lookup: deposit address → recipient info
    pub(crate) deposit_to_recipient: DBMap<DepositAddressKey, RecipientInfo>,
    /// Track processed deposits to avoid double-processing
    pub(crate) processed_deposits: DBMap<DepositTxKey, DepositRecord>,
    /// HD wallet index counters per chain
    pub(crate) hd_wallet_counters: DBMap<u8, u32>,
    /// EVM deposit monitor: last checked block per chain (chain_id -> block_number)
    pub(crate) evm_deposit_monitor_cursor: DBMap<u64, u64>,
}

impl BridgeOrchestratorTables {
    pub fn new(path: &Path) -> Arc<Self> {
        Arc::new(Self::open_tables_read_write(
            path.to_path_buf(),
            MetricConf::new("bridge"),
            None,
            None,
        ))
    }

    pub(crate) fn insert_pending_actions(&self, actions: &[BridgeAction]) -> BridgeResult<()> {
        let mut batch = self.pending_actions.batch();
        batch
            .insert_batch(
                &self.pending_actions,
                actions.iter().map(|a| (a.digest(), a)),
            )
            .map_err(|e| {
                BridgeError::StorageError(format!("Couldn't insert into pending_actions: {:?}", e))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }

    pub(crate) fn remove_pending_actions(
        &self,
        actions: &[BridgeActionDigest],
    ) -> BridgeResult<()> {
        let mut batch = self.pending_actions.batch();
        batch
            .delete_batch(&self.pending_actions, actions)
            .map_err(|e| {
                BridgeError::StorageError(format!("Couldn't delete from pending_actions: {:?}", e))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }

    pub(crate) fn update_myso_sequence_number_cursor(&self, cursor: u64) -> BridgeResult<()> {
        let mut batch = self.myso_syncer_sequence_number_cursor.batch();

        batch
            .insert_batch(&self.myso_syncer_sequence_number_cursor, [((), cursor)])
            .map_err(|e| {
                BridgeError::StorageError(format!(
                    "Couldn't insert into myso_syncer_sequence_number_cursor: {:?}",
                    e
                ))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }

    pub(crate) fn update_eth_event_cursor(
        &self,
        contract_address: alloy::primitives::Address,
        cursor: u64,
    ) -> BridgeResult<()> {
        let mut batch = self.eth_syncer_cursors.batch();

        batch
            .insert_batch(
                &self.eth_syncer_cursors,
                [(AlloyAddressSerializedAsEthers(contract_address), cursor)],
            )
            .map_err(|e| {
                BridgeError::StorageError(format!(
                    "Coudln't insert into eth_syncer_cursors: {:?}",
                    e
                ))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }

    pub fn get_all_pending_actions(&self) -> HashMap<BridgeActionDigest, BridgeAction> {
        self.pending_actions
            .safe_iter()
            .collect::<Result<HashMap<_, _>, _>>()
            .expect("failed to get all pending actions")
    }

    pub fn get_myso_event_cursors(
        &self,
        identifiers: &[Identifier],
    ) -> BridgeResult<Vec<Option<EventID>>> {
        self.myso_syncer_cursors.multi_get(identifiers).map_err(|e| {
            BridgeError::StorageError(format!("Couldn't get myso_syncer_cursors: {:?}", e))
        })
    }

    pub fn get_myso_sequence_number_cursor(&self) -> BridgeResult<Option<u64>> {
        self.myso_syncer_sequence_number_cursor
            .get(&())
            .map_err(|e| {
                BridgeError::StorageError(format!(
                    "Couldn't get myso_syncer_sequence_number_cursor: {:?}",
                    e
                ))
            })
    }

    pub fn get_eth_event_cursors(
        &self,
        contract_addresses: &[alloy::primitives::Address],
    ) -> BridgeResult<Vec<Option<u64>>> {
        let wrapped_addresses: Vec<AlloyAddressSerializedAsEthers> = contract_addresses
            .iter()
            .map(|addr| AlloyAddressSerializedAsEthers(*addr))
            .collect();
        self.eth_syncer_cursors
            .multi_get(&wrapped_addresses)
            .map_err(|e| {
                BridgeError::StorageError(format!("Couldn't get eth_syncer_cursors: {:?}", e))
            })
    }

    // ========== Relay ==========

    #[allow(dead_code)] // Used by relay when wired
    pub(crate) fn record_relay(
        &self,
        key: RelayKey,
        tx_digest: String,
        status: RelayResult,
        error: Option<String>,
    ) -> BridgeResult<()> {
        let existing_status = self.get_relay_status(&key)?;
        let retry_count = existing_status.map(|s| s.retry_count + 1).unwrap_or(0);

        let relay_status = RelayStatus {
            relayed_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            tx_digest,
            status,
            error,
            retry_count,
        };

        let mut batch = self.relayed_transfers.batch();
        batch
            .insert_batch(&self.relayed_transfers, [(key, relay_status)])
            .map_err(|e| {
                BridgeError::StorageError(format!(
                    "Couldn't insert into relayed_transfers: {:?}",
                    e
                ))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }

    #[allow(dead_code)] // Used by relay when wired
    pub(crate) fn is_relayed(&self, key: &RelayKey) -> BridgeResult<bool> {
        match self.get_relay_status(key)? {
            Some(status) => Ok(status.status == RelayResult::Success),
            None => Ok(false),
        }
    }

    #[allow(dead_code)] // Used by relay when wired
    pub(crate) fn get_relay_status(&self, key: &RelayKey) -> BridgeResult<Option<RelayStatus>> {
        self.relayed_transfers
            .get(key)
            .map_err(|e| {
                BridgeError::StorageError(format!(
                    "Couldn't get from relayed_transfers: {:?}",
                    e
                ))
            })
    }

    #[allow(dead_code)] // Used by relay when wired
    pub(crate) fn get_failed_relays(&self) -> Vec<(RelayKey, RelayStatus)> {
        self.relayed_transfers
            .safe_iter()
            .filter_map(|r| r.ok())
            .filter(|(_, status)| {
                status.status == RelayResult::Failed && status.retry_count < 5
            })
            .collect()
    }

    // ========== Deposit Address Management ==========

    #[allow(dead_code)] // Used by deposit_api when wired
    pub(crate) fn store_deposit_registration(
        &self,
        source_key: DepositAddressKey,
        registration: DepositRegistration,
    ) -> BridgeResult<()> {
        let mut registrations = self
            .deposit_registrations
            .get(&source_key)
            .map_err(|e| BridgeError::StorageError(format!("Failed to get registrations: {:?}", e)))?
            .unwrap_or_default();

        registrations.push(registration.clone());

        self.deposit_registrations
            .insert(&source_key, &registrations)
            .map_err(|e| BridgeError::StorageError(format!("Failed to store registration: {:?}", e)))?;

        let deposit_key = DepositAddressKey {
            address: registration.deposit_address.clone(),
        };
        let recipient_info = RecipientInfo {
            deposit_chain: registration.deposit_chain,
            destination_chain: registration.destination_chain,
            destination_address: registration.destination_address,
            hd_index: registration.hd_index,
            source_address: source_key.address,
            registration_type: registration.registration_type,
        };

        self.deposit_to_recipient
            .insert(&deposit_key, &recipient_info)
            .map_err(|e| {
                BridgeError::StorageError(format!("Failed to store recipient info: {:?}", e))
            })?;

        Ok(())
    }

    #[allow(dead_code)] // Used by deposit_api when wired
    pub(crate) fn get_deposit_registrations(
        &self,
        source_key: &DepositAddressKey,
    ) -> BridgeResult<Option<Vec<DepositRegistration>>> {
        self.deposit_registrations.get(source_key).map_err(|e| {
            BridgeError::StorageError(format!("Failed to get deposit registrations: {:?}", e))
        })
    }

    pub(crate) fn get_recipient_for_deposit(
        &self,
        deposit_key: &DepositAddressKey,
    ) -> BridgeResult<Option<RecipientInfo>> {
        self.deposit_to_recipient.get(deposit_key).map_err(|e| {
            BridgeError::StorageError(format!("Failed to get recipient info: {:?}", e))
        })
    }

    pub(crate) fn is_deposit_processed(&self, key: &DepositTxKey) -> BridgeResult<bool> {
        self.processed_deposits
            .contains_key(key)
            .map_err(|e| {
                BridgeError::StorageError(format!("Failed to check deposit status: {:?}", e))
            })
    }

    pub(crate) fn mark_deposit_processed(
        &self,
        key: DepositTxKey,
        bridge_tx_hash: String,
        amount: String,
    ) -> BridgeResult<()> {
        let record = DepositRecord {
            bridge_tx_hash,
            processed_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            amount,
        };

        self.processed_deposits.insert(&key, &record).map_err(|e| {
            BridgeError::StorageError(format!("Failed to mark deposit processed: {:?}", e))
        })?;

        Ok(())
    }

    pub(crate) fn get_hd_wallet_counter(&self, chain_type: u8) -> BridgeResult<Option<u32>> {
        self.hd_wallet_counters.get(&chain_type).map_err(|e| {
            BridgeError::StorageError(format!("Failed to get HD wallet counter: {:?}", e))
        })
    }

    pub(crate) fn set_hd_wallet_counter(&self, chain_type: u8, value: u32) -> BridgeResult<()> {
        self.hd_wallet_counters
            .insert(&chain_type, &value)
            .map_err(|e| {
                BridgeError::StorageError(format!("Failed to set HD wallet counter: {:?}", e))
            })?;
        Ok(())
    }

    pub(crate) fn get_all_evm_deposit_addresses(&self) -> Vec<AlloyAddress> {
        self.deposit_to_recipient
            .safe_iter()
            .filter_map(|r| r.ok())
            .filter_map(|(key, _info)| {
                if key.address.len() == 20 {
                    let arr: [u8; 20] = key.address.as_slice().try_into().ok()?;
                    Some(AlloyAddress::from(arr))
                } else {
                    None
                }
            })
            .collect()
    }

    pub(crate) fn get_all_myso_deposit_addresses(&self) -> Vec<MySoAddress> {
        self.deposit_to_recipient
            .safe_iter()
            .filter_map(|r| r.ok())
            .filter_map(|(key, _info)| {
                if key.address.len() == 32 {
                    MySoAddress::from_bytes(&key.address).ok()
                } else {
                    None
                }
            })
            .collect()
    }

    // ========== EVM Deposit Monitor Cursor ==========

    pub(crate) fn get_evm_deposit_monitor_cursor(
        &self,
        chain_id: u64,
    ) -> BridgeResult<Option<u64>> {
        self.evm_deposit_monitor_cursor.get(&chain_id).map_err(|e| {
            BridgeError::StorageError(format!(
                "Failed to get evm_deposit_monitor_cursor: {:?}",
                e
            ))
        })
    }

    pub(crate) fn update_evm_deposit_monitor_cursor(
        &self,
        chain_id: u64,
        block_number: u64,
    ) -> BridgeResult<()> {
        let mut batch = self.evm_deposit_monitor_cursor.batch();
        batch
            .insert_batch(&self.evm_deposit_monitor_cursor, [(chain_id, block_number)])
            .map_err(|e| {
                BridgeError::StorageError(format!(
                    "Couldn't insert into evm_deposit_monitor_cursor: {:?}",
                    e
                ))
            })?;
        batch
            .write()
            .map_err(|e| BridgeError::StorageError(format!("Couldn't write batch: {:?}", e)))
    }
}

/// Wrapper around alloy::primitives::Address that serializes in the same format
/// as ethers::types::Address (as a hex string) for backward compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AlloyAddressSerializedAsEthers(pub alloy::primitives::Address);

impl Serialize for AlloyAddressSerializedAsEthers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex_string = format!("0x{:x}", self.0);
        hex_string.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AlloyAddressSerializedAsEthers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let address = s.parse().map_err(serde::de::Error::custom)?;
        Ok(AlloyAddressSerializedAsEthers(address))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::test_utils::get_test_myso_to_eth_bridge_action;

    use super::*;

    // async: existing runtime is required with typed-store
    #[tokio::test]
    async fn test_bridge_storage_basic() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        let action1 = get_test_myso_to_eth_bridge_action(
            None,
            Some(0),
            Some(99),
            Some(10000),
            None,
            None,
            None,
        );

        let action2 = get_test_myso_to_eth_bridge_action(
            None,
            Some(2),
            Some(100),
            Some(10000),
            None,
            None,
            None,
        );

        // in the beginning it's empty
        let actions = store.get_all_pending_actions();
        assert!(actions.is_empty());

        // remove non existing entry is ok
        store.remove_pending_actions(&[action1.digest()]).unwrap();

        store
            .insert_pending_actions(&[action1.clone(), action2.clone()])
            .unwrap();

        let actions = store.get_all_pending_actions();
        assert_eq!(
            actions,
            HashMap::from_iter(vec![
                (action1.digest(), action1.clone()),
                (action2.digest(), action2.clone())
            ])
        );

        // insert an existing action is ok
        store
            .insert_pending_actions(std::slice::from_ref(&action1))
            .unwrap();
        let actions = store.get_all_pending_actions();
        assert_eq!(
            actions,
            HashMap::from_iter(vec![
                (action1.digest(), action1.clone()),
                (action2.digest(), action2.clone())
            ])
        );

        // remove action 2
        store.remove_pending_actions(&[action2.digest()]).unwrap();
        let actions = store.get_all_pending_actions();
        assert_eq!(
            actions,
            HashMap::from_iter(vec![(action1.digest(), action1.clone())])
        );

        // remove action 1
        store.remove_pending_actions(&[action1.digest()]).unwrap();
        let actions = store.get_all_pending_actions();
        assert!(actions.is_empty());

        // update eth event cursor
        let eth_contract_address = alloy::primitives::Address::random();
        let eth_block_num = 199999u64;
        assert!(
            store
                .get_eth_event_cursors(&[eth_contract_address])
                .unwrap()[0]
                .is_none()
        );
        store
            .update_eth_event_cursor(eth_contract_address, eth_block_num)
            .unwrap();
        assert_eq!(
            store
                .get_eth_event_cursors(&[eth_contract_address])
                .unwrap()[0]
                .unwrap(),
            eth_block_num
        );

        // update myso seq cursor
        let myso_sequence_number_cursor = 100u64;
        assert!(store.get_myso_sequence_number_cursor().unwrap().is_none());
        store
            .update_myso_sequence_number_cursor(myso_sequence_number_cursor)
            .unwrap();
        assert_eq!(
            store.get_myso_sequence_number_cursor().unwrap().unwrap(),
            myso_sequence_number_cursor
        );
    }

    #[tokio::test]
    async fn test_address_serialization() {
        let alloy_address =
            alloy::primitives::Address::from_str("0x90f8bf6a479f320ead074411a4b0e7944ea8c9c1")
                .unwrap();
        let expected_ethers_serialized = vec![
            42, 0, 0, 0, 0, 0, 0, 0, 48, 120, 57, 48, 102, 56, 98, 102, 54, 97, 52, 55, 57, 102,
            51, 50, 48, 101, 97, 100, 48, 55, 52, 52, 49, 49, 97, 52, 98, 48, 101, 55, 57, 52, 52,
            101, 97, 56, 99, 57, 99, 49,
        ];
        let wrapped_address = AlloyAddressSerializedAsEthers(alloy_address);
        let alloy_serialized = bincode::serialize(&wrapped_address).unwrap();
        assert_eq!(alloy_serialized, expected_ethers_serialized);
    }

    #[tokio::test]
    async fn test_address_deserialization() {
        let ethers_serialized = vec![
            42, 0, 0, 0, 0, 0, 0, 0, 48, 120, 57, 48, 102, 56, 98, 102, 54, 97, 52, 55, 57, 102,
            51, 50, 48, 101, 97, 100, 48, 55, 52, 52, 49, 49, 97, 52, 98, 48, 101, 55, 57, 52, 52,
            101, 97, 56, 99, 57, 99, 49,
        ];
        let wrapped_address: AlloyAddressSerializedAsEthers =
            bincode::deserialize(&ethers_serialized).unwrap();
        let expected_address =
            alloy::primitives::Address::from_str("0x90f8bf6a479f320ead074411a4b0e7944ea8c9c1")
                .unwrap();
        assert_eq!(wrapped_address.0, expected_address);
    }
}
