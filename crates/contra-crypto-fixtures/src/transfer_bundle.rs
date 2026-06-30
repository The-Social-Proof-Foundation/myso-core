// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::language_storage::StructTag;
use myso_types::base_types::ObjectID;
use rand::SeedableRng;
use rand::rngs::StdRng;
use serde::Serialize;

use crate::{
    contra_batch_range_proof_wire, format_move_hex, format_move_vector_u8, contra_commitment_bytes,
    nizk::{
        consistency_proof_parts, encode_ddh_proof, encode_elgamal_proof, sum_proof,
        total_consistency_proof,
    },
    session::{ddh_dst, elgamal_dst, session_id},
    twisted_elgamal::{EncryptedAmount, pk_from_sk, scalar_from_u64, Encryption},
    TWO_AMOUNT_DST,
};

#[derive(Debug, Serialize)]
pub struct TransferBundle {
    pub receiver_amount_parts: Vec<String>,
    pub sender_amount_parts: Vec<String>,
    pub new_balance_parts: Vec<String>,
    pub well_formed_consistency_proofs: Vec<Vec<String>>,
    pub range_proofs: Vec<String>,
    pub total_consistency_proof_parts: Vec<String>,
    pub balance_proof_parts: Vec<String>,
    pub transfer_amount: u64,
    pub remaining_balance: u64,
}

#[derive(Debug, Serialize)]
pub struct UnwrapBundle {
    pub new_balance_parts: Vec<String>,
    pub well_formed_consistency_proof_parts: Vec<String>,
    pub range_proofs: Vec<String>,
    pub balance_proof_parts: Vec<String>,
    pub unwrap_amount: u64,
}

fn encode_parts_vec(parts: Vec<[u8; 32]>) -> Vec<String> {
    parts.iter().map(|p| format!("0x{}", format_move_hex(p))).collect()
}

fn encode_parts_fixed<const N: usize>(parts: [[u8; 32]; N]) -> Vec<String> {
    parts.iter().map(|p| format!("0x{}", format_move_hex(p))).collect()
}

fn range_proof_for_two_amounts(
    _receiver: &EncryptedAmount,
    _new_balance: &EncryptedAmount,
    recv_value: u16,
    recv_blinding: u64,
    remain_value: u16,
    remain_blinding: u64,
    rng: &mut StdRng,
) -> Vec<u8> {
    let mut values = [0u64; 8];
    let mut blindings = [0u64; 8];
    values[0] = recv_value as u64;
    blindings[0] = recv_blinding;
    values[4] = remain_value as u64;
    blindings[4] = remain_blinding;
    contra_batch_range_proof_wire(&values, &blindings, 16, TWO_AMOUNT_DST, rng)
}

fn range_proof_for_one_amount(value: u16, blinding: u64, rng: &mut StdRng) -> Vec<u8> {
    let values = [value as u64, 0, 0, 0];
    let blindings = [blinding, 0, 0, 0];
    contra_batch_range_proof_wire(&values, &blindings, 16, crate::SINGLE_AMOUNT_DST, rng)
}

fn assert_ciphertext_matches_commitment(e: &Encryption, value: u64, blinding: u64) {
    let expected = contra_commitment_bytes(value, blinding);
    let actual = e.ciphertext.compress().to_bytes();
    assert_eq!(
        expected, actual,
        "ciphertext must match contra Pedersen commitment for range proof"
    );
}

pub fn build_transfer_bundle(
    sender_account_id: ObjectID,
    coin_type: &StructTag,
    sender_sk: u64,
    receiver_pk_bytes: [u8; 32],
    transfer_amount: u16,
    sender_balance: u16,
    transfer_blinding: u64,
    new_balance_blinding: u64,
) -> TransferBundle {
    let mut rng = StdRng::seed_from_u64(42);
    let sk = scalar_from_u64(sender_sk);
    let sender_pk = pk_from_sk(&sk);
    let receiver_pk = curve25519_dalek::ristretto::CompressedRistretto(receiver_pk_bytes)
        .decompress()
        .expect("valid receiver pk");

    let sid = session_id(sender_account_id, coin_type);
    let elgamal_dst = elgamal_dst(&sid);
    let ddh = ddh_dst(&sid);

    let remaining = sender_balance - transfer_amount;

    let receiver_amount = EncryptedAmount::amount_for_testing(transfer_amount, &receiver_pk, transfer_blinding);
    let sender_amount = EncryptedAmount::amount_for_testing(transfer_amount, &sender_pk, transfer_blinding);
    let new_balance = EncryptedAmount::amount_for_testing(remaining, &sender_pk, new_balance_blinding);

    assert_ciphertext_matches_commitment(&receiver_amount.limbs[0], transfer_amount as u64, transfer_blinding);
    assert_ciphertext_matches_commitment(&new_balance.limbs[0], remaining as u64, new_balance_blinding);

    let wf_receiver = consistency_proof_parts(&elgamal_dst, transfer_amount, &receiver_amount, transfer_blinding, &receiver_pk);
    let wf_new = consistency_proof_parts(&elgamal_dst, remaining, &new_balance, new_balance_blinding, &sender_pk);

    let range_proof = range_proof_for_two_amounts(
        &receiver_amount,
        &new_balance,
        transfer_amount,
        transfer_blinding,
        remaining,
        new_balance_blinding,
        &mut rng,
    );

    let total = total_consistency_proof(
        &elgamal_dst,
        transfer_amount as u64,
        &sender_pk,
        transfer_blinding,
    );

    let old_balance = EncryptedAmount::from_public_value(sender_balance as u64);
    let total_sender = sender_amount.collapse();
    let balance_proof = sum_proof(
        &ddh,
        &old_balance.collapse(),
        &new_balance.collapse(),
        &total_sender,
        &sk,
    );

    TransferBundle {
        receiver_amount_parts: encode_parts_vec(receiver_amount.encode_parts()),
        sender_amount_parts: encode_parts_vec(sender_amount.encode_parts()),
        new_balance_parts: encode_parts_vec(new_balance.encode_parts()),
        well_formed_consistency_proofs: vec![
            encode_parts_fixed(wf_receiver),
            encode_parts_fixed(wf_new),
        ],
        range_proofs: vec![format_move_vector_u8(&range_proof)],
        total_consistency_proof_parts: encode_parts_fixed(encode_elgamal_proof(&total)),
        balance_proof_parts: encode_parts_fixed(encode_ddh_proof(&balance_proof)),
        transfer_amount: transfer_amount as u64,
        remaining_balance: remaining as u64,
    }
}

pub fn build_unwrap_bundle(
    account_id: ObjectID,
    coin_type: &StructTag,
    owner_sk: u64,
    balance: u16,
    unwrap_amount: u16,
    balance_blinding: u64,
    new_balance_blinding: u64,
) -> UnwrapBundle {
    let mut rng = StdRng::seed_from_u64(43);
    let sk = scalar_from_u64(owner_sk);
    let pk = pk_from_sk(&sk);

    let sid = session_id(account_id, coin_type);
    let elgamal_dst = elgamal_dst(&sid);
    let ddh = ddh_dst(&sid);

    let remaining = balance - unwrap_amount;
    let new_balance = EncryptedAmount::amount_for_testing(remaining, &pk, new_balance_blinding);
    assert_ciphertext_matches_commitment(&new_balance.limbs[0], remaining as u64, new_balance_blinding);

    let wf = consistency_proof_parts(&elgamal_dst, remaining, &new_balance, new_balance_blinding, &pk);
    let range_proof = range_proof_for_one_amount(remaining, new_balance_blinding, &mut rng);

    let old_balance = EncryptedAmount::from_public_value(balance as u64);
    let taken = EncryptedAmount::amount_for_testing(unwrap_amount, &pk, balance_blinding);
    let balance_proof = sum_proof(
        &ddh,
        &old_balance.collapse(),
        &new_balance.collapse(),
        &taken.collapse(),
        &sk,
    );

    UnwrapBundle {
        new_balance_parts: encode_parts_vec(new_balance.encode_parts()),
        well_formed_consistency_proof_parts: encode_parts_fixed(wf),
        range_proofs: vec![format_move_vector_u8(&range_proof)],
        balance_proof_parts: encode_parts_fixed(encode_ddh_proof(&balance_proof)),
        unwrap_amount: unwrap_amount as u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use move_core_types::account_address::AccountAddress;
    use move_core_types::ident_str;
    use std::str::FromStr;

    fn test_coin() -> StructTag {
        StructTag {
            address: AccountAddress::from_hex_literal("0xabc").unwrap(),
            module: ident_str!("test_coin").to_owned(),
            name: ident_str!("TEST_COIN").to_owned(),
            type_params: vec![],
        }
    }

    #[test]
    fn transfer_bundle_serializes() {
        let receiver_pk = pk_from_sk(&scalar_from_u64(67890));
        let bundle = build_transfer_bundle(
            ObjectID::from_str("0x100").unwrap(),
            &test_coin(),
            12345,
            receiver_pk.compress().to_bytes(),
            50,
            100,
            32533,
            10097,
        );
        let json = serde_json::to_string(&bundle).unwrap();
        assert!(json.contains("receiver_amount_parts"));
    }
}
