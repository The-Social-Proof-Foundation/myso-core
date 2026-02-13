// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! HD Wallet derivation for custodial deposit addresses.
//! Generates deterministic deposit addresses from master bridge authority key.

use crate::error::{BridgeError, BridgeResult};
use crate::storage::BridgeOrchestratorTables;
use alloy::primitives::Address as EthAddress;
use alloy::signers::local::PrivateKeySigner;
use fastcrypto::hash::{HashFunction, Keccak256};
use fastcrypto::secp256k1::{Secp256k1KeyPair, Secp256k1PrivateKey};
use fastcrypto::traits::{KeyPair as KeyPairTrait, ToFromBytes};
use myso_types::base_types::MySoAddress;
use myso_types::crypto::MySoKeyPair;
use std::sync::{Arc, Mutex};
use tracing::info;

#[allow(dead_code)]
pub const HD_COUNTER_EVM: u8 = 0;
#[allow(dead_code)]
pub const HD_COUNTER_MYSO: u8 = 1;

/// Manages HD wallet derivation for deposit addresses
pub struct DepositAddressManager {
    master_key: Secp256k1KeyPair,
    store: Arc<BridgeOrchestratorTables>,
    /// Mutex to serialize HD index allocation (prevents race condition)
    allocation_lock: Mutex<()>,
}

impl DepositAddressManager {
    pub fn new(master_key: Secp256k1KeyPair, store: Arc<BridgeOrchestratorTables>) -> Self {
        info!("Initializing DepositAddressManager");
        Self {
            master_key,
            store,
            allocation_lock: Mutex::new(()),
        }
    }

    /// Derive EVM deposit address using hash-based derivation
    pub fn derive_evm_deposit_address(
        &self,
        index: u32,
    ) -> BridgeResult<(EthAddress, PrivateKeySigner)> {
        let mut derivation_data = Vec::new();
        derivation_data.extend_from_slice(self.master_key.copy().private().as_bytes());
        derivation_data.extend_from_slice(b"evm_deposit");
        derivation_data.extend_from_slice(&index.to_be_bytes());

        let hash = Keccak256::digest(&derivation_data);

        let signer = PrivateKeySigner::from_slice(&hash.digest)
            .map_err(|e| BridgeError::Generic(format!("Failed to create signing key: {:?}", e)))?;
        let address = signer.address();

        info!(index, ?address, "Derived EVM deposit address");

        Ok((address, signer))
    }

    /// Derive MySo deposit address using hash-based derivation
    pub fn derive_myso_deposit_address(
        &self,
        index: u32,
    ) -> BridgeResult<(MySoAddress, MySoKeyPair)> {
        let mut derivation_data = Vec::new();
        derivation_data.extend_from_slice(self.master_key.public().as_bytes());
        derivation_data.extend_from_slice(b"myso_deposit");
        derivation_data.extend_from_slice(&index.to_be_bytes());

        let hash = Keccak256::digest(&derivation_data);

        let child_privkey = Secp256k1PrivateKey::from_bytes(&hash.digest).map_err(|e| {
            BridgeError::Generic(format!("Failed to create MySo private key: {:?}", e))
        })?;

        let child_keypair = Secp256k1KeyPair::from(child_privkey);
        let address = MySoAddress::from(child_keypair.public());

        info!(index, ?address, "Derived MySo deposit address");

        Ok((address, MySoKeyPair::Secp256k1(child_keypair)))
    }

    /// Allocate next HD wallet index for a specific chain.
    /// Uses mutex to prevent race condition (two concurrent requests getting same index).
    pub fn allocate_next_index(&self, chain_type: u8) -> BridgeResult<u32> {
        let _guard = self
            .allocation_lock
            .lock()
            .map_err(|e| BridgeError::Generic(format!("Mutex poisoned: {:?}", e)))?;

        let current_index = self.store.get_hd_wallet_counter(chain_type)?.unwrap_or(0);

        let next_index = current_index + 1;
        self.store.set_hd_wallet_counter(chain_type, next_index)?;

        info!(chain_type, next_index, "Allocated HD wallet index");

        Ok(current_index)
    }

    /// Get EVM signer for a specific index (for signing bridge transactions)
    pub fn get_evm_signer_for_index(&self, index: u32) -> BridgeResult<PrivateKeySigner> {
        let (_address, signer) = self.derive_evm_deposit_address(index)?;
        Ok(signer)
    }

    /// Get MySo keypair for a specific index (for signing bridge transactions)
    pub fn get_myso_keypair_for_index(&self, index: u32) -> BridgeResult<MySoKeyPair> {
        let (_address, keypair) = self.derive_myso_deposit_address(index)?;
        Ok(keypair)
    }

    /// Verify an address was derived from our master key
    pub fn verify_deposit_address(&self, address: &EthAddress, index: u32) -> BridgeResult<bool> {
        let (derived_address, _) = self.derive_evm_deposit_address(index)?;
        Ok(&derived_address == address)
    }

    /// Verify a MySo address was derived from our master key
    pub fn verify_myso_deposit_address(
        &self,
        address: &MySoAddress,
        index: u32,
    ) -> BridgeResult<bool> {
        let (derived_address, _) = self.derive_myso_deposit_address(index)?;
        Ok(&derived_address == address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fastcrypto::traits::KeyPair;
    use myso_types::crypto::get_key_pair;

    #[test]
    fn test_evm_derivation_deterministic() {
        let (_, master_key): (_, Secp256k1KeyPair) = get_key_pair();
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        let manager = DepositAddressManager::new(master_key, store);

        let (addr1, _) = manager.derive_evm_deposit_address(0).unwrap();
        let (addr2, _) = manager.derive_evm_deposit_address(0).unwrap();
        assert_eq!(addr1, addr2);

        let (addr3, _) = manager.derive_evm_deposit_address(1).unwrap();
        assert_ne!(addr1, addr3);
    }

    #[test]
    fn test_myso_derivation_deterministic() {
        let (_, master_key): (_, Secp256k1KeyPair) = get_key_pair();
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        let manager = DepositAddressManager::new(master_key, store);

        let (addr1, _) = manager.derive_myso_deposit_address(0).unwrap();
        let (addr2, _) = manager.derive_myso_deposit_address(0).unwrap();
        assert_eq!(addr1, addr2);

        let (addr3, _) = manager.derive_myso_deposit_address(1).unwrap();
        assert_ne!(addr1, addr3);
    }

    #[test]
    fn test_index_allocation() {
        let (_, master_key): (_, Secp256k1KeyPair) = get_key_pair();
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        let manager = DepositAddressManager::new(master_key, store);

        let idx1 = manager.allocate_next_index(HD_COUNTER_EVM).unwrap();
        let idx2 = manager.allocate_next_index(HD_COUNTER_EVM).unwrap();
        assert_eq!(idx1, 0);
        assert_eq!(idx2, 1);

        let idx3 = manager.allocate_next_index(HD_COUNTER_MYSO).unwrap();
        assert_eq!(idx3, 0);
    }
}
