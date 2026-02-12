// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Auto-relay module that automatically claims approved bridge transfers.
//! Config types only for now; full relay implementation to be ported.

use alloy::primitives::Address as EthAddress;

/// Relay configuration
#[derive(Debug, Clone)]
pub struct RelayConfig {
    pub enabled: bool,
    pub max_retries: u8,
    pub retry_delay_seconds: u64,
    pub myso_gas_budget: u64,
    pub evm: Option<EvmRelayConfig>,
}

/// EVM-specific relay configuration
#[derive(Debug, Clone)]
pub struct EvmRelayConfig {
    pub enabled: bool,
    pub rpc_url: String,
    pub bridge_contract_address: EthAddress,
    pub max_gas_price_gwei: u64,
    pub gas_estimate_buffer_percent: u8,
    pub confirmation_blocks: u64,
}
