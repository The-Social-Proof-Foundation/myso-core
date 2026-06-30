// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use move_core_types::language_storage::{StructTag, TypeTag};
use myso_types::base_types::{MySoAddress, ObjectID};
use myso_types::derived_object;
use myso_types::CONTRA_ADDRESS;
use std::str::FromStr;

fn account_key_type_tag() -> TypeTag {
    TypeTag::Struct(Box::new(StructTag {
        address: CONTRA_ADDRESS,
        module: ident_str!("contra").to_owned(),
        name: ident_str!("AccountKey").to_owned(),
        type_params: vec![],
    }))
}

/// Deterministic `Account` object id for `owner` under `registry_id` (matches on-chain `new_account`).
pub fn account_id(registry_id: ObjectID, owner: MySoAddress) -> ObjectID {
    let key_bytes = bcs::to_bytes(&AccountAddress::from(owner)).expect("bcs owner");
    derived_object::derive_object_id(registry_id, &account_key_type_tag(), &key_bytes)
        .expect("derive account id")
}

pub const DST_DDH: u8 = 0x01;
pub const DST_ELGAMAL: u8 = 0x02;

pub fn token_account_key_type_tag(coin_type: &StructTag) -> TypeTag {
    TypeTag::Struct(Box::new(StructTag {
        address: CONTRA_ADDRESS,
        module: ident_str!("contra").to_owned(),
        name: ident_str!("TokenAccountKey").to_owned(),
        type_params: vec![TypeTag::Struct(Box::new(coin_type.clone()))],
    }))
}

pub fn session_id(account_id: ObjectID, coin_type: &StructTag) -> [u8; 20] {
    let key_tag = token_account_key_type_tag(coin_type);
    let derived = derived_object::derive_object_id(account_id, &key_tag, &[]).expect("derive");
    let addr = AccountAddress::from(derived);
    let full = addr.to_vec();
    let mut out = [0u8; 20];
    out.copy_from_slice(&full[..20]);
    out
}

pub fn dst(session_id: &[u8; 20], protocol_id: u8) -> Vec<u8> {
    let mut out = session_id.to_vec();
    out.push(protocol_id);
    out
}

pub fn elgamal_dst(session_id: &[u8; 20]) -> Vec<u8> {
    dst(session_id, DST_ELGAMAL)
}

pub fn ddh_dst(session_id: &[u8; 20]) -> Vec<u8> {
    dst(session_id, DST_DDH)
}

pub fn parse_struct_tag(type_str: &str) -> StructTag {
    match TypeTag::from_str(type_str).expect("valid type tag") {
        TypeTag::Struct(tag) => *tag,
        other => panic!("expected struct type tag, got {other}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn account_id_is_deterministic() {
        let registry = ObjectID::from_str("0xabc").unwrap();
        let owner = AccountAddress::from_hex_literal("0x100").unwrap();
        let id1 = account_id(registry, owner.into());
        let id2 = account_id(registry, owner.into());
        assert_eq!(id1, id2);
    }

    #[test]
    fn session_id_length() {
        let account = ObjectID::from_str("0x100").unwrap();
        let coin = StructTag {
            address: AccountAddress::from_hex_literal("0xabc").unwrap(),
            module: ident_str!("test_coin").to_owned(),
            name: ident_str!("TEST_COIN").to_owned(),
            type_params: vec![],
        };
        let sid = session_id(account, &coin);
        assert_eq!(sid.len(), 20);
    }
}
