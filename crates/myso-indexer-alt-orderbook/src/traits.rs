//! MoveStruct trait for Orderbook indexer
//!
//! This module provides the MoveStruct trait for handling Move structs and event types.
//! Module definitions and related functions have been moved to lib.rs for centralized configuration.

use move_core_types::{
    account_address::AccountAddress,
    identifier::Identifier,
    language_storage::{StructTag, TypeTag},
};
use serde::Serialize;
use std::str::FromStr;

// Import types and functions from lib.rs
use crate::{get_module_type, ModuleType};

/// Trait for Move structs that can be matched against event types
pub trait MoveStruct: Serialize {
    // Event type matching constants
    const MODULE: &'static str;
    const NAME: &'static str;
    const TYPE_PARAMS: &'static [&'static str] = &[];

    /// Get the list of acceptable package addresses for this event type based on environment
    fn acceptable_package_addresses(
        env: crate::OrderbookEnv,
    ) -> Result<Vec<AccountAddress>, String> {
        get_package_addresses_for_module(Self::MODULE, env)
    }

    /// Check if a struct tag matches this event type from any supported package version
    fn matches_event_type(event_type: &StructTag, env: crate::OrderbookEnv) -> bool {
        // Get all possible struct types for this event
        let all_struct_types = Self::get_all_struct_types(env);

        // NOTE: We intentionally ignore type_params.len() because events may have phantom/generic
        // type parameters that don't affect the actual event structure (e.g., MysoBurned<BaseAsset, QuoteAsset>)
        all_struct_types.iter().any(|struct_type| {
            event_type.address == struct_type.address
                && event_type.module == struct_type.module
                && event_type.name == struct_type.name
        })
    }

    /// Generate all possible struct types for this event across all supported package versions
    fn get_all_struct_types(env: crate::OrderbookEnv) -> Vec<StructTag> {
        let acceptable_addresses = match Self::acceptable_package_addresses(env) {
            Ok(addresses) => addresses,
            Err(_) => return Vec::new(),
        };
        let mut struct_types = Vec::new();

        for address in acceptable_addresses {
            let struct_tag = StructTag {
                address,
                module: Identifier::from_str(Self::MODULE).unwrap(),
                name: Identifier::from_str(Self::NAME).unwrap(),
                type_params: Self::TYPE_PARAMS
                    .iter()
                    .map(|param| {
                        TypeTag::Struct(Box::new(StructTag {
                            address,
                            module: Identifier::from_str(Self::MODULE).unwrap(),
                            name: Identifier::from_str(param).unwrap(),
                            type_params: Vec::new(),
                        }))
                    })
                    .collect(),
            };
            struct_types.push(struct_tag);
        }

        struct_types
    }
}

/// Generic helper that reads package addresses from lib.rs at runtime
pub fn get_package_addresses_for_module(
    module: &str,
    env: crate::OrderbookEnv,
) -> Result<Vec<AccountAddress>, String> {
    match get_module_type(module) {
        ModuleType::Core => {
            let core_packages = crate::get_core_package_addresses(env);
            let mut addresses = Vec::new();

            for addr_str in core_packages {
                if let Ok(addr) = parse_address_from_hex(addr_str) {
                    addresses.push(addr);
                }
            }

            Ok(addresses)
        }
        ModuleType::Margin => {
            let margin_packages = crate::get_margin_package_addresses(env);
            let mut addresses = Vec::new();

            for addr_str in margin_packages {
                if let Ok(addr) = parse_address_from_hex(addr_str) {
                    addresses.push(addr);
                }
            }

            if addresses.is_empty() {
                Err(format!(
                    "Margin trading is not supported on {:?}. \
                    The margin package has not been deployed on this network. \
                    Requested module: '{}'",
                    env, module
                ))
            } else {
                Ok(addresses)
            }
        }
        ModuleType::MySo => {
            const MYSO_SYSTEM_ADDRESS: &str =
                "0000000000000000000000000000000000000000000000000000000000000002";
            if let Ok(addr) = parse_address_from_hex(MYSO_SYSTEM_ADDRESS) {
                Ok(vec![addr])
            } else {
                Err("Failed to parse MYSO system address".to_string())
            }
        }
        ModuleType::Unknown => Err(format!("Unknown module: {}", module)),
    }
}

/// Helper function to parse hex string addresses into AccountAddress
fn parse_address_from_hex(hex_str: &str) -> Result<AccountAddress, String> {
    let hex_str = if hex_str.starts_with("0x") {
        &hex_str[2..]
    } else {
        hex_str
    };

    // Package addresses come from `AccountAddress::to_hex_literal()`, which drops leading zeros
    // ("0xb0c"). Left-pad to the full 32 bytes: without this the orderbook package failed to parse,
    // was silently skipped, and no core/margin event ever matched (every table stayed empty).
    if hex_str.is_empty() || hex_str.len() > AccountAddress::LENGTH * 2 {
        return Err(format!("Invalid address length: {}", hex_str.len()));
    }
    let padded = format!("{:0>width$}", hex_str, width = AccountAddress::LENGTH * 2);
    let bytes = hex::decode(&padded).map_err(|e| format!("Failed to decode hex: {}", e))?;

    if bytes.len() != AccountAddress::LENGTH {
        return Err(format!(
            "Expected {} bytes, got {}",
            AccountAddress::LENGTH,
            bytes.len()
        ));
    }

    let mut addr_bytes = [0u8; AccountAddress::LENGTH];
    addr_bytes.copy_from_slice(&bytes);

    Ok(AccountAddress::new(addr_bytes))
}

#[cfg(test)]
mod address_parsing_tests {
    use super::*;

    #[test]
    fn short_and_full_package_addresses_parse_to_the_same_address() {
        let short = parse_address_from_hex("0xb0c").unwrap();
        let full = parse_address_from_hex(
            "0x0000000000000000000000000000000000000000000000000000000000000b0c",
        )
        .unwrap();
        assert_eq!(short, full);
        assert_eq!(short, AccountAddress::from_str("0xb0c").unwrap());
    }

    #[test]
    fn orderbook_core_events_have_an_acceptable_package() {
        let addrs = get_package_addresses_for_module("order_info", crate::OrderbookEnv::Testnet).unwrap();
        assert_eq!(addrs, vec![myso_types::ORDERBOOK_ADDRESS]);
    }

    #[test]
    fn rejects_overlong_addresses() {
        assert!(parse_address_from_hex(&format!("0x{}", "1".repeat(65))).is_err());
    }
}
