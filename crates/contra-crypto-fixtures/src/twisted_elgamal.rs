// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::{CompressedRistretto, RistrettoPoint};
use curve25519_dalek::scalar::Scalar;

/// Blinding generator `h` from [`contra::twisted_elgamal`].
pub fn h_generator() -> RistrettoPoint {
    let bytes: [u8; 32] = [
        0x34, 0xce, 0x14, 0x77, 0xc1, 0x45, 0x58, 0x17, 0x80, 0x89, 0x50, 0x0a, 0x39, 0xc8,
        0x64, 0xe0, 0xf6, 0x07, 0xb3, 0xc1, 0xf4, 0x1a, 0xb3, 0x98, 0x40, 0x0e, 0x4a, 0x9d,
        0xe6, 0xd2, 0xc4, 0x46,
    ];
    CompressedRistretto(bytes)
        .decompress()
        .expect("valid h point")
}

pub fn g_generator() -> RistrettoPoint {
    RISTRETTO_BASEPOINT_POINT
}

pub fn g_identity() -> RistrettoPoint {
    RistrettoPoint::default()
}

pub fn scalar_from_u64(x: u64) -> Scalar {
    Scalar::from(x)
}

#[derive(Clone, Copy, Debug)]
pub struct Encryption {
    pub ciphertext: RistrettoPoint,
    pub decryption_handle: RistrettoPoint,
}

impl Encryption {
    pub fn compress(&self) -> ([u8; 32], [u8; 32]) {
        (
            self.ciphertext.compress().to_bytes(),
            self.decryption_handle.compress().to_bytes(),
        )
    }

    pub fn add(&self, other: &Encryption) -> Encryption {
        Encryption {
            ciphertext: self.ciphertext + other.ciphertext,
            decryption_handle: self.decryption_handle + other.decryption_handle,
        }
    }

    pub fn sub(&self, other: &Encryption) -> Encryption {
        Encryption {
            ciphertext: self.ciphertext - other.ciphertext,
            decryption_handle: self.decryption_handle - other.decryption_handle,
        }
    }
}

pub fn encrypt_zero() -> Encryption {
    Encryption {
        ciphertext: g_identity(),
        decryption_handle: g_identity(),
    }
}

pub fn encrypt_trivial(amount: u64) -> Encryption {
    if amount == 0 {
        encrypt_zero()
    } else {
        Encryption {
            ciphertext: g_generator() * Scalar::from(0u64) + h_generator() * Scalar::from(amount),
            decryption_handle: g_identity(),
        }
    }
}

pub fn encrypt_trivial_for_testing(amount: u64, pk: &RistrettoPoint, blinding: u64) -> Encryption {
    let r = scalar_from_u64(blinding);
    let g = g_generator();
    let h = h_generator();
    Encryption {
        ciphertext: g * r + h * Scalar::from(amount),
        decryption_handle: pk * r,
    }
}

pub fn from_value(value: u64) -> [Encryption; 4] {
    [
        encrypt_trivial(value & 0xFFFF),
        encrypt_trivial((value >> 16) & 0xFFFF),
        encrypt_trivial((value >> 32) & 0xFFFF),
        encrypt_trivial((value >> 48) & 0xFFFF),
    ]
}

#[derive(Clone, Debug)]
pub struct EncryptedAmount {
    pub limbs: [Encryption; 4],
}

impl EncryptedAmount {
    pub fn from_limbs(limbs: [Encryption; 4]) -> Self {
        Self { limbs }
    }

    pub fn amount_for_testing(value: u16, pk: &RistrettoPoint, blinding: u64) -> Self {
        Self {
            limbs: [
                encrypt_trivial_for_testing(value as u64, pk, blinding),
                encrypt_zero(),
                encrypt_zero(),
                encrypt_zero(),
            ],
        }
    }

    pub fn from_public_value(value: u64) -> Self {
        Self {
            limbs: from_value(value),
        }
    }

    pub fn collapse(&self) -> Encryption {
        fn collapse_limbs(l0: &RistrettoPoint, l1: &RistrettoPoint, l2: &RistrettoPoint, l3: &RistrettoPoint) -> RistrettoPoint {
            let s16 = scalar_from_u64(1 << 16);
            let s32 = scalar_from_u64(1 << 32);
            let s48 = scalar_from_u64(1 << 48);
            l0 + l1 * s16 + l2 * s32 + l3 * s48
        }
        Encryption {
            ciphertext: collapse_limbs(
                &self.limbs[0].ciphertext,
                &self.limbs[1].ciphertext,
                &self.limbs[2].ciphertext,
                &self.limbs[3].ciphertext,
            ),
            decryption_handle: collapse_limbs(
                &self.limbs[0].decryption_handle,
                &self.limbs[1].decryption_handle,
                &self.limbs[2].decryption_handle,
                &self.limbs[3].decryption_handle,
            ),
        }
    }

    pub fn encode_parts(&self) -> Vec<[u8; 32]> {
        let mut parts = Vec::with_capacity(8);
        for limb in &self.limbs {
            let (c, d) = limb.compress();
            parts.push(c);
            parts.push(d);
        }
        parts
    }
}

pub fn pk_from_sk(sk: &Scalar) -> RistrettoPoint {
    g_generator() * sk
}
