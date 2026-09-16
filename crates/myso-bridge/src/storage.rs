// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::error::{BridgeError, BridgeResult};
use crate::types::{BridgeAction, BridgeActionDigest};
use alloy::primitives::Address as AlloyAddress;
use fastcrypto::encoding::{Encoding, Hex};
use myso_types::Identifier;
use myso_types::base_types::MySoAddress;
use myso_types::event::EventID;
use rand::RngCore;
use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
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
    /// Platform completion webhook (HTTPS). Optional for older RocksDB rows.
    #[serde(default)]
    pub deposit_callback_url: Option<String>,
    /// Optional shared secret sent as `x-internal-api-key`.
    #[serde(default)]
    pub deposit_callback_api_key: Option<String>,
    /// Outbound rail id when the Move type is shared (USDC=3, USDT=4). Older rows default to USDC.
    #[serde(default)]
    pub destination_token_id: Option<u8>,
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

    pub fn from_formatted(address: &str) -> Option<Self> {
        if let Ok(myso) = MySoAddress::from_str(address) {
            return Some(Self::from_myso(myso));
        }
        if let Ok(evm) = AlloyAddress::from_str(address) {
            return Some(Self::from_evm(evm));
        }
        None
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
    #[serde(default)]
    pub deposit_callback_url: Option<String>,
    #[serde(default)]
    pub deposit_callback_api_key: Option<String>,
    #[serde(default)]
    pub destination_token_id: Option<u8>,
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

/// Direction of a custodial bridge order.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BridgeOrderDirection {
    In,
    Out,
}

/// Lifecycle status owned by the bridge node.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BridgeOrderStatus {
    AwaitingDeposit,
    DepositReceived,
    Bridging,
    Completed,
    Failed,
}

/// Platform-facing bridge order (source of truth for lifecycle).
#[derive(Debug, Clone, Serialize)]
pub struct BridgeOrderRecord {
    pub order_id: String,
    pub direction: BridgeOrderDirection,
    pub status: BridgeOrderStatus,
    pub myso_wallet: String,
    pub deposit_address: String,
    pub destination_chain: String,
    pub destination_address: String,
    pub amount: Option<String>,
    pub deposit_tx_digest: Option<String>,
    pub bridge_tx_digest: Option<String>,
    pub evm_tx_hash: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub callback_url: Option<String>,
    pub callback_api_key: Option<String>,
    pub token_id: Option<u8>,
    pub evm_token_address: Option<String>,
    pub myso_token_type: Option<String>,
}

const BRIDGE_ORDER_RECORD_FIELDS: usize = 18;

impl<'de> Deserialize<'de> for BridgeOrderRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(BRIDGE_ORDER_RECORD_FIELDS, BridgeOrderRecordVisitor)
    }
}

struct BridgeOrderRecordVisitor;

impl<'de> Visitor<'de> for BridgeOrderRecordVisitor {
    type Value = BridgeOrderRecord;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("BridgeOrderRecord")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        fn required<'de, A, T>(seq: &mut A, name: &'static str) -> Result<T, A::Error>
        where
            A: SeqAccess<'de>,
            T: Deserialize<'de>,
        {
            seq.next_element()?
                .ok_or_else(|| de::Error::missing_field(name))
        }

        fn trailing<'de, A, T>(seq: &mut A) -> T
        where
            A: SeqAccess<'de>,
            T: Default + Deserialize<'de>,
        {
            match seq.next_element() {
                Ok(Some(value)) => value,
                _ => T::default(),
            }
        }

        Ok(BridgeOrderRecord {
            order_id: required(&mut seq, "order_id")?,
            direction: required(&mut seq, "direction")?,
            status: required(&mut seq, "status")?,
            myso_wallet: required(&mut seq, "myso_wallet")?,
            deposit_address: required(&mut seq, "deposit_address")?,
            destination_chain: required(&mut seq, "destination_chain")?,
            destination_address: required(&mut seq, "destination_address")?,
            amount: required(&mut seq, "amount")?,
            deposit_tx_digest: required(&mut seq, "deposit_tx_digest")?,
            bridge_tx_digest: required(&mut seq, "bridge_tx_digest")?,
            evm_tx_hash: required(&mut seq, "evm_tx_hash")?,
            created_at: required(&mut seq, "created_at")?,
            updated_at: required(&mut seq, "updated_at")?,
            callback_url: trailing(&mut seq),
            callback_api_key: trailing(&mut seq),
            token_id: trailing(&mut seq),
            evm_token_address: trailing(&mut seq),
            myso_token_type: trailing(&mut seq),
        })
    }
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

#[derive(Debug, Clone, Default)]
pub struct BridgeOrderPatch {
    pub amount: Option<String>,
    pub deposit_tx_digest: Option<String>,
    pub bridge_tx_digest: Option<String>,
    pub evm_tx_hash: Option<String>,
    pub token_id: Option<u8>,
    pub evm_token_address: Option<String>,
    pub myso_token_type: Option<String>,
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub(crate) fn new_order_id() -> String {
    let mut bytes = [0u8; 16];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    format!("brg_{}", Hex::encode(bytes))
}

fn apply_order_patch(
    order: &mut BridgeOrderRecord,
    status: BridgeOrderStatus,
    patch: BridgeOrderPatch,
) {
    order.status = status;
    order.updated_at = now_ms();
    if let Some(amount) = patch.amount {
        order.amount = Some(amount);
    }
    if let Some(digest) = patch.deposit_tx_digest {
        order.deposit_tx_digest = Some(digest);
    }
    if let Some(digest) = patch.bridge_tx_digest {
        order.bridge_tx_digest = Some(digest);
    }
    if let Some(hash) = patch.evm_tx_hash {
        order.evm_tx_hash = Some(hash);
    }
    if let Some(token_id) = patch.token_id {
        order.token_id = Some(token_id);
    }
    if let Some(address) = patch.evm_token_address {
        order.evm_token_address = Some(address);
    }
    if let Some(token_type) = patch.myso_token_type {
        order.myso_token_type = Some(token_type);
    }
}

fn should_fork_bridge_order(order: &BridgeOrderRecord, patch: &BridgeOrderPatch) -> bool {
    let Some(new_digest) = patch.deposit_tx_digest.as_deref() else {
        return false;
    };
    if order.deposit_tx_digest.as_deref() == Some(new_digest) {
        return false;
    }
    order.deposit_tx_digest.is_some()
        || matches!(
            order.status,
            BridgeOrderStatus::Completed | BridgeOrderStatus::Failed
        )
}

fn deposit_key_from_formatted(address: &str) -> Option<DepositAddressKey> {
    DepositAddressKey::from_formatted(address)
}

fn normalize_myso_wallet(wallet: &str) -> String {
    normalize_hex_address(wallet)
}

fn normalize_hex_address(address: &str) -> String {
    let trimmed = address.trim();
    let without_prefix = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);
    without_prefix.to_ascii_lowercase()
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
    /// Canonical bridge orders keyed by order id
    #[allow(dead_code)]
    pub(crate) bridge_orders: DBMap<String, BridgeOrderRecord>,
    /// Deposit address → latest order id
    #[allow(dead_code)]
    pub(crate) deposit_to_order: DBMap<DepositAddressKey, String>,
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
        self.myso_syncer_cursors
            .multi_get(identifiers)
            .map_err(|e| {
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
        self.relayed_transfers.get(key).map_err(|e| {
            BridgeError::StorageError(format!("Couldn't get from relayed_transfers: {:?}", e))
        })
    }

    #[allow(dead_code)] // Used by relay when wired
    pub(crate) fn get_failed_relays(&self) -> Vec<(RelayKey, RelayStatus)> {
        self.relayed_transfers
            .safe_iter()
            .filter_map(|r| r.ok())
            .filter(|(_, status)| status.status == RelayResult::Failed && status.retry_count < 5)
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
            .map_err(|e| {
                BridgeError::StorageError(format!("Failed to get registrations: {:?}", e))
            })?
            .unwrap_or_default();

        registrations.push(registration.clone());

        self.deposit_registrations
            .insert(&source_key, &registrations)
            .map_err(|e| {
                BridgeError::StorageError(format!("Failed to store registration: {:?}", e))
            })?;

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
            deposit_callback_url: registration.deposit_callback_url.clone(),
            deposit_callback_api_key: registration.deposit_callback_api_key.clone(),
            destination_token_id: registration.destination_token_id,
        };

        self.deposit_to_recipient
            .insert(&deposit_key, &recipient_info)
            .map_err(|e| {
                BridgeError::StorageError(format!("Failed to store recipient info: {:?}", e))
            })?;

        Ok(())
    }

    /// Update the completion callback on an existing registration + recipient row.
    pub(crate) fn update_deposit_callback(
        &self,
        source_key: &DepositAddressKey,
        deposit_address: &[u8],
        callback_url: Option<String>,
        callback_api_key: Option<String>,
    ) -> BridgeResult<()> {
        let mut registrations = self
            .deposit_registrations
            .get(source_key)
            .map_err(|e| {
                BridgeError::StorageError(format!("Failed to get registrations: {:?}", e))
            })?
            .unwrap_or_default();

        let Some(existing) = registrations
            .iter_mut()
            .find(|reg| reg.deposit_address == deposit_address)
        else {
            return Ok(());
        };

        existing.deposit_callback_url = callback_url.clone();
        existing.deposit_callback_api_key = callback_api_key.clone();

        self.deposit_registrations
            .insert(source_key, &registrations)
            .map_err(|e| {
                BridgeError::StorageError(format!("Failed to store registration: {:?}", e))
            })?;

        let deposit_key = DepositAddressKey {
            address: deposit_address.to_vec(),
        };
        if let Some(mut recipient) = self.deposit_to_recipient.get(&deposit_key).map_err(|e| {
            BridgeError::StorageError(format!("Failed to get recipient info: {:?}", e))
        })? {
            recipient.deposit_callback_url = callback_url;
            recipient.deposit_callback_api_key = callback_api_key;
            self.deposit_to_recipient
                .insert(&deposit_key, &recipient)
                .map_err(|e| {
                    BridgeError::StorageError(format!("Failed to store recipient info: {:?}", e))
                })?;
        }

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
        self.processed_deposits.contains_key(key).map_err(|e| {
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

    pub(crate) fn upsert_bridge_order(&self, order: BridgeOrderRecord) -> BridgeResult<()> {
        let deposit_key = deposit_key_from_formatted(&order.deposit_address);
        self.bridge_orders
            .insert(&order.order_id, &order)
            .map_err(|e| {
                BridgeError::StorageError(format!("Failed to store bridge order: {:?}", e))
            })?;
        if let Some(deposit_key) = deposit_key {
            self.deposit_to_order
                .insert(&deposit_key, &order.order_id)
                .map_err(|e| {
                    BridgeError::StorageError(format!("Failed to index deposit order: {:?}", e))
                })?;
        }
        Ok(())
    }

    pub(crate) fn get_bridge_order(&self, order_id: &str) -> BridgeResult<Option<BridgeOrderRecord>> {
        self.bridge_orders.get(&order_id.to_string()).map_err(|e| {
            BridgeError::StorageError(format!("Failed to get bridge order: {:?}", e))
        })
    }

    pub(crate) fn list_bridge_orders_for_wallet(
        &self,
        wallet: &str,
    ) -> BridgeResult<Vec<BridgeOrderRecord>> {
        const MAX_ORDERS: usize = 1000;
        let needle = normalize_myso_wallet(wallet);
        if needle.is_empty() {
            return Ok(Vec::new());
        }
        let mut orders: Vec<BridgeOrderRecord> = self
            .bridge_orders
            .safe_iter()
            .filter_map(|r| r.ok())
            .filter_map(|(_, order)| {
                if normalize_myso_wallet(&order.myso_wallet) == needle {
                    Some(order)
                } else {
                    None
                }
            })
            .collect();
        orders.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        if orders.len() > MAX_ORDERS {
            orders.truncate(MAX_ORDERS);
        }
        Ok(orders)
    }

    pub(crate) fn get_order_for_deposit(
        &self,
        deposit_key: &DepositAddressKey,
    ) -> BridgeResult<Option<BridgeOrderRecord>> {
        let order_id = self.deposit_to_order.get(deposit_key).map_err(|e| {
            BridgeError::StorageError(format!("Failed to get deposit order id: {:?}", e))
        })?;
        let Some(order_id) = order_id else {
            return Ok(None);
        };
        self.get_bridge_order(&order_id)
    }

    pub(crate) fn update_bridge_order_status(
        &self,
        deposit_key: &DepositAddressKey,
        status: BridgeOrderStatus,
        patch: BridgeOrderPatch,
    ) -> BridgeResult<Option<BridgeOrderRecord>> {
        let Some(order) = self.get_order_for_deposit(deposit_key)? else {
            return Ok(None);
        };
        if should_fork_bridge_order(&order, &patch) {
            let now = now_ms();
            let mut forked = BridgeOrderRecord {
                order_id: new_order_id(),
                direction: order.direction,
                status,
                myso_wallet: order.myso_wallet,
                deposit_address: order.deposit_address,
                destination_chain: order.destination_chain,
                destination_address: order.destination_address,
                amount: None,
                deposit_tx_digest: None,
                bridge_tx_digest: None,
                evm_tx_hash: None,
                created_at: now,
                updated_at: now,
                callback_url: order.callback_url,
                callback_api_key: order.callback_api_key,
                token_id: None,
                evm_token_address: None,
                myso_token_type: None,
            };
            apply_order_patch(&mut forked, status, patch);
            self.upsert_bridge_order(forked.clone())?;
            return Ok(Some(forked));
        }
        let mut order = order;
        apply_order_patch(&mut order, status, patch);
        self.bridge_orders
            .insert(&order.order_id, &order)
            .map_err(|e| {
                BridgeError::StorageError(format!("Failed to update bridge order: {:?}", e))
            })?;
        Ok(Some(order))
    }

    pub(crate) fn complete_out_bridge_order(
        &self,
        myso_wallet: &str,
        dest_evm: &str,
        token_id: Option<u8>,
        bridge_tx_digest: String,
        evm_tx_hash: Option<String>,
    ) -> BridgeResult<Option<BridgeOrderRecord>> {
        let dest = normalize_hex_address(dest_evm);
        let Some(mut order) = self
            .list_bridge_orders_for_wallet(myso_wallet)?
            .into_iter()
            .find(|order| {
                order.direction == BridgeOrderDirection::Out
                    && matches!(
                        order.status,
                        BridgeOrderStatus::Bridging | BridgeOrderStatus::DepositReceived
                    )
                    && normalize_hex_address(&order.destination_address) == dest
                    && match (token_id, order.token_id) {
                        (Some(want), Some(have)) => want == have,
                        _ => true,
                    }
            })
        else {
            return Ok(None);
        };
        order.status = BridgeOrderStatus::Completed;
        order.updated_at = now_ms();
        order.bridge_tx_digest = Some(bridge_tx_digest);
        if let Some(hash) = evm_tx_hash {
            order.evm_tx_hash = Some(hash);
        }
        if order.token_id.is_none() {
            order.token_id = token_id;
        }
        self.bridge_orders
            .insert(&order.order_id, &order)
            .map_err(|e| {
                BridgeError::StorageError(format!("Failed to complete out bridge order: {:?}", e))
            })?;
        Ok(Some(order))
    }

    // ========== EVM Deposit Monitor Cursor ==========

    pub(crate) fn get_evm_deposit_monitor_cursor(
        &self,
        chain_id: u64,
    ) -> BridgeResult<Option<u64>> {
        self.evm_deposit_monitor_cursor.get(&chain_id).map_err(|e| {
            BridgeError::StorageError(format!("Failed to get evm_deposit_monitor_cursor: {:?}", e))
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

    #[tokio::test]
    async fn test_bridge_order_roundtrip() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());
        let evm = alloy::primitives::Address::from_str("0x90f8bf6a479f320ead074411a4b0e7944ea8c9c1")
            .unwrap();
        let order = BridgeOrderRecord {
            order_id: "brg_test".to_string(),
            direction: BridgeOrderDirection::In,
            status: BridgeOrderStatus::AwaitingDeposit,
            myso_wallet: "0xmyso".to_string(),
            deposit_address: format!("{:?}", evm),
            destination_chain: "mysocial".to_string(),
            destination_address: "0xmyso".to_string(),
            amount: None,
            deposit_tx_digest: None,
            bridge_tx_digest: None,
            evm_tx_hash: None,
            created_at: 1,
            updated_at: 1,
            callback_url: None,
            callback_api_key: None,
            token_id: None,
            evm_token_address: None,
            myso_token_type: None,
        };
        store.upsert_bridge_order(order.clone()).unwrap();
        let loaded = store.get_bridge_order("brg_test").unwrap().unwrap();
        assert_eq!(loaded.order_id, "brg_test");
        let by_deposit = store
            .get_order_for_deposit(&DepositAddressKey::from_evm(evm))
            .unwrap()
            .unwrap();
        assert_eq!(by_deposit.order_id, "brg_test");
        let updated = store
            .update_bridge_order_status(
                &DepositAddressKey::from_evm(evm),
                BridgeOrderStatus::Completed,
                BridgeOrderPatch::default(),
            )
            .unwrap()
            .unwrap();
        assert_eq!(updated.status, BridgeOrderStatus::Completed);
    }

    #[tokio::test]
    async fn test_list_bridge_orders_for_wallet() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());
        let wallet_a = "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8";
        let wallet_b = "0xa8b70a01b8f6ab20ce71723badbdef1499b629fda23085ffb13347fd32342137";
        let inbound = BridgeOrderRecord {
            order_id: "brg_in".to_string(),
            direction: BridgeOrderDirection::In,
            status: BridgeOrderStatus::Completed,
            myso_wallet: wallet_a.to_string(),
            deposit_address: "0x19d1a1d630fef261d1425525487e612679490dd5".to_string(),
            destination_chain: "mysocial".to_string(),
            destination_address: wallet_a.to_string(),
            amount: Some("10000000".to_string()),
            deposit_tx_digest: None,
            bridge_tx_digest: None,
            evm_tx_hash: None,
            created_at: 1,
            updated_at: 20,
            callback_url: None,
            callback_api_key: None,
            token_id: None,
            evm_token_address: None,
            myso_token_type: None,
        };
        let outbound = BridgeOrderRecord {
            order_id: "brg_out".to_string(),
            direction: BridgeOrderDirection::Out,
            status: BridgeOrderStatus::Bridging,
            myso_wallet: wallet_a.replace("0x", "0X"),
            deposit_address: wallet_a.to_string(),
            destination_chain: "base".to_string(),
            destination_address: "0x0000000000000000000000000000000000000001".to_string(),
            amount: Some("1.5".to_string()),
            deposit_tx_digest: Some("digest".to_string()),
            bridge_tx_digest: None,
            evm_tx_hash: None,
            created_at: 2,
            updated_at: 30,
            callback_url: None,
            callback_api_key: None,
            token_id: None,
            evm_token_address: None,
            myso_token_type: None,
        };
        let other = BridgeOrderRecord {
            order_id: "brg_other".to_string(),
            direction: BridgeOrderDirection::In,
            status: BridgeOrderStatus::AwaitingDeposit,
            myso_wallet: wallet_b.to_string(),
            deposit_address: "0x90f8bf6a479f320ead074411a4b0e7944ea8c9c1".to_string(),
            destination_chain: "mysocial".to_string(),
            destination_address: wallet_b.to_string(),
            amount: None,
            deposit_tx_digest: None,
            bridge_tx_digest: None,
            evm_tx_hash: None,
            created_at: 3,
            updated_at: 40,
            callback_url: None,
            callback_api_key: None,
            token_id: None,
            evm_token_address: None,
            myso_token_type: None,
        };
        store.upsert_bridge_order(inbound).unwrap();
        store.upsert_bridge_order(outbound).unwrap();
        store.upsert_bridge_order(other).unwrap();

        let listed = store.list_bridge_orders_for_wallet(&wallet_a[2..]).unwrap();
        assert_eq!(listed.len(), 2);
        assert_eq!(listed[0].order_id, "brg_out");
        assert_eq!(listed[1].order_id, "brg_in");
        assert_eq!(
            store.list_bridge_orders_for_wallet(wallet_b).unwrap().len(),
            1
        );
    }

    #[tokio::test]
    async fn test_update_bridge_order_status_forks_on_new_deposit_tx() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());
        let evm = alloy::primitives::Address::from_str("0x90f8bf6a479f320ead074411a4b0e7944ea8c9c1")
            .unwrap();
        let deposit_key = DepositAddressKey::from_evm(evm);
        let wallet = "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8";
        store
            .upsert_bridge_order(BridgeOrderRecord {
                order_id: "brg_generate".to_string(),
                direction: BridgeOrderDirection::In,
                status: BridgeOrderStatus::AwaitingDeposit,
                myso_wallet: wallet.to_string(),
                deposit_address: format!("{:?}", evm),
                destination_chain: "mysocial".to_string(),
                destination_address: wallet.to_string(),
                amount: None,
                deposit_tx_digest: None,
                bridge_tx_digest: None,
                evm_tx_hash: None,
                created_at: 1,
                updated_at: 1,
                callback_url: Some("https://example.test/hook".to_string()),
                callback_api_key: Some("secret".to_string()),
                token_id: None,
                evm_token_address: None,
                myso_token_type: None,
            })
            .unwrap();

        let received = store
            .update_bridge_order_status(
                &deposit_key,
                BridgeOrderStatus::DepositReceived,
                BridgeOrderPatch {
                    amount: Some("1".to_string()),
                    deposit_tx_digest: Some("tx_a".to_string()),
                    ..Default::default()
                },
            )
            .unwrap()
            .unwrap();
        assert_eq!(received.order_id, "brg_generate");
        assert_eq!(received.status, BridgeOrderStatus::DepositReceived);

        let completed = store
            .update_bridge_order_status(
                &deposit_key,
                BridgeOrderStatus::Completed,
                BridgeOrderPatch {
                    deposit_tx_digest: Some("tx_a".to_string()),
                    ..Default::default()
                },
            )
            .unwrap()
            .unwrap();
        assert_eq!(completed.order_id, "brg_generate");
        assert_eq!(completed.status, BridgeOrderStatus::Completed);

        let forked = store
            .update_bridge_order_status(
                &deposit_key,
                BridgeOrderStatus::DepositReceived,
                BridgeOrderPatch {
                    amount: Some("2".to_string()),
                    deposit_tx_digest: Some("tx_b".to_string()),
                    ..Default::default()
                },
            )
            .unwrap()
            .unwrap();
        assert_ne!(forked.order_id, "brg_generate");
        assert!(forked.order_id.starts_with("brg_"));
        assert_eq!(forked.status, BridgeOrderStatus::DepositReceived);
        assert_eq!(forked.deposit_tx_digest.as_deref(), Some("tx_b"));
        assert_eq!(forked.amount.as_deref(), Some("2"));
        assert_eq!(forked.myso_wallet, wallet);
        assert_eq!(
            forked.callback_url.as_deref(),
            Some("https://example.test/hook")
        );

        let listed = store.list_bridge_orders_for_wallet(wallet).unwrap();
        assert_eq!(listed.len(), 2);
        let current = store.get_order_for_deposit(&deposit_key).unwrap().unwrap();
        assert_eq!(current.order_id, forked.order_id);
        let original = store.get_bridge_order("brg_generate").unwrap().unwrap();
        assert_eq!(original.status, BridgeOrderStatus::Completed);
        assert_eq!(original.deposit_tx_digest.as_deref(), Some("tx_a"));
    }

    #[tokio::test]
    async fn test_update_bridge_order_status_persists_token_fields() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());
        let evm = alloy::primitives::Address::from_str("0x90f8bf6a479f320ead074411a4b0e7944ea8c9c1")
            .unwrap();
        let deposit_key = DepositAddressKey::from_evm(evm);
        let wallet = "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8";
        store
            .upsert_bridge_order(BridgeOrderRecord {
                order_id: "brg_token".to_string(),
                direction: BridgeOrderDirection::In,
                status: BridgeOrderStatus::AwaitingDeposit,
                myso_wallet: wallet.to_string(),
                deposit_address: format!("{:?}", evm),
                destination_chain: "mysocial".to_string(),
                destination_address: wallet.to_string(),
                amount: None,
                deposit_tx_digest: None,
                bridge_tx_digest: None,
                evm_tx_hash: None,
                created_at: 1,
                updated_at: 1,
                callback_url: None,
                callback_api_key: None,
                token_id: None,
                evm_token_address: None,
                myso_token_type: None,
            })
            .unwrap();

        let updated = store
            .update_bridge_order_status(
                &deposit_key,
                BridgeOrderStatus::DepositReceived,
                BridgeOrderPatch {
                    amount: Some("100000000".to_string()),
                    deposit_tx_digest: Some("tx_btc".to_string()),
                    token_id: Some(1),
                    evm_token_address: Some(
                        "0x5fc748f1feb28d7b76fa1c6b07d8ba2d5535177c".to_string(),
                    ),
                    myso_token_type: Some(
                        "0xe621bbe7c5ab61c595da074a598f1fb8db10cbd1ea56e20028815028312b920e::btc::BTC"
                            .to_string(),
                    ),
                    ..Default::default()
                },
            )
            .unwrap()
            .unwrap();
        assert_eq!(updated.token_id, Some(1));
        assert_eq!(
            updated.evm_token_address.as_deref(),
            Some("0x5fc748f1feb28d7b76fa1c6b07d8ba2d5535177c")
        );
        assert!(
            updated
                .myso_token_type
                .as_deref()
                .is_some_and(|ty| ty.ends_with("::btc::BTC"))
        );

        let listed = store.list_bridge_orders_for_wallet(wallet).unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].token_id, Some(1));
        assert_eq!(
            listed[0].evm_token_address.as_deref(),
            Some("0x5fc748f1feb28d7b76fa1c6b07d8ba2d5535177c")
        );
    }

    #[test]
    fn test_bridge_order_bcs_reads_pre_token_rows() {
        #[derive(Serialize)]
        struct LegacyOrder {
            order_id: String,
            direction: BridgeOrderDirection,
            status: BridgeOrderStatus,
            myso_wallet: String,
            deposit_address: String,
            destination_chain: String,
            destination_address: String,
            amount: Option<String>,
            deposit_tx_digest: Option<String>,
            bridge_tx_digest: Option<String>,
            evm_tx_hash: Option<String>,
            created_at: u64,
            updated_at: u64,
            callback_url: Option<String>,
            callback_api_key: Option<String>,
        }

        let legacy = LegacyOrder {
            order_id: "brg_legacy".to_string(),
            direction: BridgeOrderDirection::In,
            status: BridgeOrderStatus::Completed,
            myso_wallet: "0xabc".to_string(),
            deposit_address: "0x19d1a1d630fef261d1425525487e612679490dd5".to_string(),
            destination_chain: "mysocial".to_string(),
            destination_address: "0xabc".to_string(),
            amount: Some("1".to_string()),
            deposit_tx_digest: Some("tx".to_string()),
            bridge_tx_digest: None,
            evm_tx_hash: None,
            created_at: 1,
            updated_at: 2,
            callback_url: None,
            callback_api_key: None,
        };
        let bytes = bcs::to_bytes(&legacy).unwrap();
        let loaded: BridgeOrderRecord = bcs::from_bytes(&bytes).unwrap();
        assert_eq!(loaded.order_id, "brg_legacy");
        assert_eq!(loaded.status, BridgeOrderStatus::Completed);
        assert_eq!(loaded.token_id, None);
        assert_eq!(loaded.evm_token_address, None);
        assert_eq!(loaded.myso_token_type, None);

        let current = BridgeOrderRecord {
            order_id: "brg_new".to_string(),
            direction: BridgeOrderDirection::In,
            status: BridgeOrderStatus::Completed,
            myso_wallet: "0xabc".to_string(),
            deposit_address: "0x19d1".to_string(),
            destination_chain: "mysocial".to_string(),
            destination_address: "0xabc".to_string(),
            amount: Some("1".to_string()),
            deposit_tx_digest: None,
            bridge_tx_digest: None,
            evm_tx_hash: None,
            created_at: 1,
            updated_at: 2,
            callback_url: None,
            callback_api_key: None,
            token_id: Some(2),
            evm_token_address: Some("0x38a0".to_string()),
            myso_token_type: Some("::eth::ETH".to_string()),
        };
        let roundtrip: BridgeOrderRecord =
            bcs::from_bytes(&bcs::to_bytes(&current).unwrap()).unwrap();
        assert_eq!(roundtrip.token_id, Some(2));
        assert_eq!(roundtrip.evm_token_address.as_deref(), Some("0x38a0"));
    }
}
