// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use bulletproofs::{BulletproofGens, PedersenGens, RangeProof as ExternalRangeProof};
use curve25519_dalek::scalar::Scalar;
use fastcrypto::bulletproofs::Range;
use merlin::Transcript;

pub const SINGLE_AMOUNT_DST: &[u8] = b"dst-match-21-byte-tag";
pub const TWO_AMOUNT_DST: &[u8] = b"two-amt-dst-21-bytes!!";
pub const WRONG_DST: &[u8] = b"wrong-dst-21-byte-tag!!";

pub fn leak_dst(dst: &[u8]) -> &'static [u8] {
    Box::leak(dst.to_vec().into_boxed_slice())
}

fn dalek_scalar_from_u64(x: u64) -> Scalar {
    Scalar::from(x)
}

fn bits_for_range(range: &Range) -> usize {
    match range {
        Range::Bits8 => 8,
        Range::Bits16 => 16,
        Range::Bits32 => 32,
        Range::Bits64 => 64,
    }
}

pub fn range_from_bits(bit_size: u32) -> Range {
    match bit_size {
        8 => Range::Bits8,
        16 => Range::Bits16,
        32 => Range::Bits32,
        64 => Range::Bits64,
        _ => panic!("unsupported bit size {bit_size}"),
    }
}

/// Produce a bulletproofs wire-format batch proof with DST binding (`Transcript::new(dst)`).
pub fn batch_range_proof_wire(
    values: &[u64],
    blindings: &[u64],
    bit_size: u32,
    dst: &[u8],
    rng: &mut (impl rand::RngCore + rand::CryptoRng),
) -> Vec<u8> {
    assert_eq!(values.len(), blindings.len());
    assert!(values.len().is_power_of_two());

    let range = range_from_bits(bit_size);
    let bits = bits_for_range(&range);
    let bp_gens = BulletproofGens::new(bits, values.len());
    let pc_gens = PedersenGens::default();
    let dst_label = leak_dst(dst);
    let mut prover_transcript = Transcript::new(dst_label);

    let blindings: Vec<Scalar> = blindings
        .iter()
        .map(|&b| dalek_scalar_from_u64(b))
        .collect();

    let (proof, _) = ExternalRangeProof::prove_multiple_with_rng(
        &bp_gens,
        &pc_gens,
        &mut prover_transcript,
        values,
        &blindings,
        bits,
        rng,
    )
    .expect("prove_multiple_with_rng");

    proof.to_bytes()
}

pub fn pedersen_commitment_bytes(value: u64, blinding: u64) -> [u8; 32] {
    let pc_gens = PedersenGens::default();
    pc_gens
        .commit(
            dalek_scalar_from_u64(value),
            dalek_scalar_from_u64(blinding),
        )
        .compress()
        .to_bytes()
}

pub fn assert_wire_proof_valid(
    proof_bytes: &[u8],
    values: &[u64],
    blindings: &[u64],
    bit_size: u32,
    dst: &[u8],
) {
    let range = range_from_bits(bit_size);
    let bits = bits_for_range(&range);
    let external = ExternalRangeProof::from_bytes(proof_bytes).expect("wire proof bytes");
    let bp_gens = BulletproofGens::new(bits, values.len());
    let pc_gens = PedersenGens::default();
    let dst_label = leak_dst(dst);
    let mut verifier_transcript = Transcript::new(dst_label);
    let compressed: Vec<_> = values
        .iter()
        .zip(blindings)
        .map(|(&v, &b)| {
            pc_gens
                .commit(dalek_scalar_from_u64(v), dalek_scalar_from_u64(b))
                .compress()
        })
        .collect();
    assert!(
        external
            .verify_multiple(
                &bp_gens,
                &pc_gens,
                &mut verifier_transcript,
                &compressed,
                bits,
            )
            .is_ok()
    );
}

pub fn format_move_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn format_move_vector_u8(bytes: &[u8]) -> String {
    format!("x\"{}\"", format_move_hex(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn wire_proof_round_trip() {
        let mut rng = StdRng::seed_from_u64(42);
        let values = [1234u64, 0, 0, 0];
        let blindings = [7777u64, 0, 0, 0];
        let proof = batch_range_proof_wire(&values, &blindings, 16, SINGLE_AMOUNT_DST, &mut rng);
        assert_wire_proof_valid(&proof, &values, &blindings, 16, SINGLE_AMOUNT_DST);
    }
}
