// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::object_runtime::ObjectRuntime;
use crate::{NativesCostTable, get_extension};
use bulletproofs::{BulletproofGens, PedersenGens, RangeProof as ExternalRangeProof};
use curve25519_dalek::ristretto::CompressedRistretto;
use fastcrypto::bulletproofs::Range;
use fastcrypto::error::FastCryptoError::InvalidInput;
use fastcrypto::error::FastCryptoResult;
use fastcrypto::groups::ristretto255::RistrettoPoint;
use fastcrypto::serde_helpers::ToFromByteArray;
use merlin::Transcript;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::gas_algebra::InternalGas;
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::native_charge_gas_early_exit;
use move_vm_runtime::native_functions::NativeContext;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Value, VectorRef},
};
use smallvec::smallvec;
use std::collections::VecDeque;

pub const NOT_SUPPORTED: u64 = 0;
pub const INVALID_PROOF: u64 = 1;
pub const INVALID_RANGE: u64 = 2;
pub const INVALID_BATCH_SIZE: u64 = 3;

pub const MAX_TOTAL_BITS: u64 = 512;
const MAX_PROOF_SIZE: usize = 864;
const RISTRETTO_POINT_BYTE_LENGTH: usize = 32;

#[derive(Clone)]
pub struct BulletproofsCostParams {
    pub verify_bulletproofs_ristretto255_base_cost: Option<InternalGas>,
    pub verify_bulletproofs_ristretto255_cost_per_bit_and_commitment: Option<InternalGas>,
}

fn is_supported(context: &NativeContext) -> PartialVMResult<bool> {
    Ok(get_extension!(context, ObjectRuntime)?
        .protocol_config
        .enable_verify_bulletproofs_ristretto255())
}

fn range_from_bits(bits: u8) -> FastCryptoResult<Range> {
    match bits {
        8 => Ok(Range::Bits8),
        16 => Ok(Range::Bits16),
        32 => Ok(Range::Bits32),
        64 => Ok(Range::Bits64),
        _ => Err(InvalidInput),
    }
}

fn bits_for_range(range: &Range) -> usize {
    match range {
        Range::Bits8 => 8,
        Range::Bits16 => 16,
        Range::Bits32 => 32,
        Range::Bits64 => 64,
    }
}

fn verify_external_range_proof_batch(
    proof: &ExternalRangeProof,
    compressed_commitments: &[CompressedRistretto],
    range: &Range,
    dst: &'static [u8],
) -> bool {
    let bits = bits_for_range(range);
    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(bits, compressed_commitments.len());
    let mut verifier_transcript = Transcript::new(dst);

    proof
        .verify_multiple(
            &bp_gens,
            &pc_gens,
            &mut verifier_transcript,
            compressed_commitments,
            bits,
        )
        .is_ok()
}

pub fn verify_bulletproofs_with_dst_ristretto255_internal(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 4);

    if !is_supported(context)? {
        return Ok(NativeResult::err(context.gas_used(), NOT_SUPPORTED));
    }

    let dst = pop_arg!(args, VectorRef);
    let dst = dst.as_bytes_ref().to_vec();
    let dst_label: &'static [u8] = Box::leak(dst.into_boxed_slice());
    let commitments = pop_arg!(args, VectorRef);
    let range_bits = pop_arg!(args, u8);
    let proof = pop_arg!(args, VectorRef);

    let cost_parameters = get_extension!(context, NativesCostTable)?
        .bulletproofs_cost_params
        .clone();

    native_charge_gas_early_exit!(
        context,
        cost_parameters
            .verify_bulletproofs_ristretto255_base_cost
            .ok_or_else(|| {
                PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                    .with_message("verify_bulletproofs_ristretto255_base_cost not available")
            })?
    );

    let proof_bytes = proof.as_bytes_ref();
    if proof_bytes.len() > MAX_PROOF_SIZE {
        return Ok(NativeResult::err(context.gas_used(), INVALID_PROOF));
    }

    let Ok(external_proof) = ExternalRangeProof::from_bytes(&proof_bytes) else {
        return Ok(NativeResult::err(context.gas_used(), INVALID_PROOF));
    };

    let Ok(range) = range_from_bits(range_bits) else {
        return Ok(NativeResult::err(context.gas_used(), INVALID_RANGE));
    };

    let vector_u8_type = Type::Vector(Box::new(Type::U8));
    let length = commitments.len(&vector_u8_type)?.value_as::<u64>()?;

    let total_bits = length * range_bits as u64;
    if length == 0 || !length.is_power_of_two() || total_bits > MAX_TOTAL_BITS {
        return Ok(NativeResult::err(context.gas_used(), INVALID_BATCH_SIZE));
    }

    native_charge_gas_early_exit!(
        context,
        cost_parameters
            .verify_bulletproofs_ristretto255_cost_per_bit_and_commitment
            .ok_or_else(|| {
                PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR).with_message(
                    "verify_bulletproofs_ristretto255_cost_per_bit_and_commitment not available",
                )
            })?
            * total_bits.into()
    );

    let compressed_commitments = (0..length)
        .map(|i| {
            commitments
                .borrow_elem(i as usize, &vector_u8_type)
                .and_then(|reference| reference.value_as::<VectorRef>())
                .map(|v| v.as_bytes_ref().to_vec())
                .and_then(|v| {
                    v.try_into()
                        .map_err(|_| PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR))
                })
                .and_then(|b: [u8; RISTRETTO_POINT_BYTE_LENGTH]| {
                    RistrettoPoint::from_byte_array(&b)
                        .map(|p| CompressedRistretto(p.to_byte_array()))
                        .map_err(|_| PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR))
                })
        })
        .collect::<PartialVMResult<Vec<_>>>()?;

    let result = verify_external_range_proof_batch(
        &external_proof,
        &compressed_commitments,
        &range,
        dst_label,
    );

    Ok(NativeResult::ok(
        context.gas_used(),
        smallvec![Value::bool(result)],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use fastcrypto::bulletproofs::{Range, RangeProof};
    use fastcrypto::groups::ristretto255::RistrettoScalar;
    use fastcrypto::pedersen::{Blinding, PedersenCommitment};
    use rand::RngCore;
    use rand::thread_rng;

    fn leak_dst(dst: &[u8]) -> &'static [u8] {
        Box::leak(dst.to_vec().into_boxed_slice())
    }

    #[test]
    fn range_proof_prove_verify_batch_round_trip_with_dst() {
        let mut rng = thread_rng();
        let dst = leak_dst(b"dst-match-21-byte-tag");
        let blindings: Vec<Blinding> = (0..4).map(|_| Blinding::rand(&mut rng)).collect();
        let values = vec![100u64, 0, 0, 0];
        let range = Range::Bits16;

        let proof = RangeProof::prove_batch(&values, &blindings, &range, dst, &mut rng).unwrap();
        let commitments: Vec<PedersenCommitment> = values
            .iter()
            .zip(&blindings)
            .map(|(&v, b)| PedersenCommitment::from_blinding(&RistrettoScalar::from(v), b))
            .collect();

        assert!(proof.verify_batch(&commitments, &range, dst).is_ok());
        assert!(
            proof
                .verify_batch(&commitments, &range, leak_dst(b"wrong-dst-21-byte-tag!!"))
                .is_err()
        );
    }

    #[test]
    fn wire_format_proof_matches_fastcrypto_verify_batch() {
        use curve25519_dalek::scalar::Scalar;

        let mut rng = thread_rng();
        let dst = leak_dst(b"dst-match-21-byte-tag");
        let blindings: Vec<Blinding> = (0..4).map(|_| Blinding::rand(&mut rng)).collect();
        let values = vec![50u64, 0, 0, 0];
        let range = Range::Bits16;

        let proof = RangeProof::prove_batch(&values, &blindings, &range, dst, &mut rng).unwrap();
        let commitments: Vec<PedersenCommitment> = values
            .iter()
            .zip(&blindings)
            .map(|(&v, b)| PedersenCommitment::from_blinding(&RistrettoScalar::from(v), b))
            .collect();
        assert!(proof.verify_batch(&commitments, &range, dst).is_ok());

        let bits = bits_for_range(&range);
        let bp_gens = BulletproofGens::new(bits, values.len());
        let pc_gens = PedersenGens::default();
        let mut prover_transcript = Transcript::new(dst);
        let dalek_blindings: Vec<Scalar> = (0..4).map(|_| Scalar::from(rng.next_u64())).collect();
        let (wire_proof, _) = ExternalRangeProof::prove_multiple_with_rng(
            &bp_gens,
            &pc_gens,
            &mut prover_transcript,
            &values,
            &dalek_blindings,
            bits,
            &mut rng,
        )
        .unwrap();

        let compressed: Vec<CompressedRistretto> = values
            .iter()
            .zip(&dalek_blindings)
            .map(|(&v, b)| pc_gens.commit(Scalar::from(v), *b).compress())
            .collect();
        assert!(verify_external_range_proof_batch(
            &wire_proof,
            &compressed,
            &range,
            dst,
        ));
    }
}
