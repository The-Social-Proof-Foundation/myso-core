// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::hash::{Blake2b256, HashFunction};
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;

use crate::twisted_elgamal::{Encryption, g_generator, h_generator, scalar_from_u64};

#[derive(Clone, Copy, Debug)]
pub struct DdhProof {
    pub a: RistrettoPoint,
    pub b: RistrettoPoint,
    pub z: Scalar,
}

#[derive(Clone, Copy, Debug)]
pub struct ElGamalProof {
    pub a: RistrettoPoint,
    pub b: RistrettoPoint,
    pub z1: Scalar,
    pub z2: Scalar,
}

pub fn fiat_shamir_challenge(parts: &[&[u8]]) -> Scalar {
    let bytes_list: Vec<Vec<u8>> = parts.iter().map(|p| p.to_vec()).collect();
    let hash = Blake2b256::digest(bcs::to_bytes(&bytes_list).expect("bcs"));
    let mut scalar_bytes = [0u8; 32];
    scalar_bytes.copy_from_slice(hash.as_ref());
    scalar_bytes[31] = 0;
    Scalar::from_bytes_mod_order(scalar_bytes)
}

fn challenge_ddh(
    dst: &[u8],
    g: &RistrettoPoint,
    h: &RistrettoPoint,
    x_g: &RistrettoPoint,
    x_h: &RistrettoPoint,
    a: &RistrettoPoint,
    b: &RistrettoPoint,
) -> Scalar {
    fiat_shamir_challenge(&[
        dst,
        g.compress().as_bytes(),
        h.compress().as_bytes(),
        x_g.compress().as_bytes(),
        x_h.compress().as_bytes(),
        a.compress().as_bytes(),
        b.compress().as_bytes(),
    ])
}

fn challenge_elgamal(
    dst: &[u8],
    g: &RistrettoPoint,
    h: &RistrettoPoint,
    pk: &RistrettoPoint,
    e1: &RistrettoPoint,
    e2: &RistrettoPoint,
    a: &RistrettoPoint,
    b: &RistrettoPoint,
) -> Scalar {
    fiat_shamir_challenge(&[
        dst,
        g.compress().as_bytes(),
        h.compress().as_bytes(),
        pk.compress().as_bytes(),
        e1.compress().as_bytes(),
        e2.compress().as_bytes(),
        a.compress().as_bytes(),
        b.compress().as_bytes(),
    ])
}

pub fn prove_ddh(
    dst: &[u8],
    x: &Scalar,
    g: &RistrettoPoint,
    h: &RistrettoPoint,
    x_g: &RistrettoPoint,
    x_h: &RistrettoPoint,
    r: &Scalar,
) -> DdhProof {
    let a = g * r;
    let b = h * r;
    let c = challenge_ddh(dst, g, h, x_g, x_h, &a, &b);
    let z = r + c * x;
    DdhProof { a, b, z }
}

pub fn prove_elgamal(
    dst: &[u8],
    pk: &RistrettoPoint,
    e: &Encryption,
    amount: u64,
    blinding: u64,
) -> ElGamalProof {
    let g = g_generator();
    let h = h_generator();
    let r1 = scalar_from_u64(1234);
    let r2 = scalar_from_u64(5678);
    let a = pk * r1;
    let b = g * r1 + h * r2;
    let c = challenge_elgamal(
        dst,
        &g,
        &h,
        pk,
        &e.ciphertext,
        &e.decryption_handle,
        &a,
        &b,
    );
    let z1 = r1 + c * scalar_from_u64(blinding);
    let z2 = r2 + c * scalar_from_u64(amount);
    ElGamalProof { a, b, z1, z2 }
}

pub fn sum_proof(
    dst: &[u8],
    sum: &Encryption,
    a: &Encryption,
    b: &Encryption,
    sk: &Scalar,
) -> DdhProof {
    let pk = g_generator() * sk;
    let zero_encryption = a.add(b).sub(sum);
    prove_ddh(
        dst,
        sk,
        &g_generator(),
        &zero_encryption.ciphertext,
        &pk,
        &zero_encryption.decryption_handle,
        &scalar_from_u64(1234567),
    )
}

pub fn encode_ddh_proof(proof: &DdhProof) -> [[u8; 32]; 3] {
    [
        proof.a.compress().to_bytes(),
        proof.b.compress().to_bytes(),
        proof.z.to_bytes(),
    ]
}

pub fn encode_elgamal_proof(proof: &ElGamalProof) -> [[u8; 32]; 4] {
    [
        proof.a.compress().to_bytes(),
        proof.b.compress().to_bytes(),
        proof.z1.to_bytes(),
        proof.z2.to_bytes(),
    ]
}

pub fn consistency_proof_parts(
    dst: &[u8],
    amount: u16,
    ea: &crate::twisted_elgamal::EncryptedAmount,
    blinding: u64,
    pk: &RistrettoPoint,
) -> [[u8; 32]; 16] {
    let e0 = &ea.limbs[0];
    let e1 = &ea.limbs[1];
    let e2 = &ea.limbs[2];
    let e3 = &ea.limbs[3];
    let b1 = if e1.decryption_handle == crate::twisted_elgamal::g_identity() {
        0
    } else {
        blinding
    };
    let b2 = if e2.decryption_handle == crate::twisted_elgamal::g_identity() {
        0
    } else {
        blinding
    };
    let b3 = if e3.decryption_handle == crate::twisted_elgamal::g_identity() {
        0
    } else {
        blinding
    };
    let p0 = prove_elgamal(dst, pk, e0, amount as u64, blinding);
    let p1 = prove_elgamal(dst, pk, e1, 0, b1);
    let p2 = prove_elgamal(dst, pk, e2, 0, b2);
    let p3 = prove_elgamal(dst, pk, e3, 0, b3);
    let mut out = [[0u8; 32]; 16];
    out[0..4].copy_from_slice(&encode_elgamal_proof(&p0));
    out[4..8].copy_from_slice(&encode_elgamal_proof(&p1));
    out[8..12].copy_from_slice(&encode_elgamal_proof(&p2));
    out[12..16].copy_from_slice(&encode_elgamal_proof(&p3));
    out
}

pub fn total_consistency_proof(
    dst: &[u8],
    value: u64,
    sender_pk: &RistrettoPoint,
    blinding: u64,
) -> ElGamalProof {
    let enc = crate::twisted_elgamal::encrypt_trivial_for_testing(value, sender_pk, blinding);
    prove_elgamal(dst, sender_pk, &enc, value, blinding)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::twisted_elgamal::g_identity;

    #[test]
    fn fiat_shamir_challenge_regression() {
        let dst: Vec<u8> = (0..21).collect();
        let p1: Vec<u8> = (0..32).collect();
        let c = fiat_shamir_challenge(&[&dst, &p1]);
        assert_eq!(
            c.to_bytes(),
            [
                0xaf, 0x00, 0xc4, 0x97, 0x60, 0x49, 0xed, 0x81, 0x80, 0x5c, 0x76, 0xd3, 0xc5,
                0xba, 0x7c, 0xfa, 0xeb, 0x15, 0x50, 0xe4, 0x4f, 0x59, 0x78, 0xcf, 0xfb, 0x12,
                0xb2, 0x85, 0xa5, 0xe2, 0x5a, 0x00
            ]
        );
    }

    #[test]
    fn prove_elgamal_round_trip_structure() {
        let sk = scalar_from_u64(12345);
        let pk = g_generator() * sk;
        let enc = crate::twisted_elgamal::encrypt_trivial_for_testing(42, &pk, 67890);
        let proof = prove_elgamal(&[], &pk, &enc, 42, 67890);
        assert_ne!(proof.a, g_identity());
    }
}
