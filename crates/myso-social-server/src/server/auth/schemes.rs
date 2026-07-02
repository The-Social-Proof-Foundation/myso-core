// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::fmt;

/// MySo signature schemes (flag byte prefix on `X-Public-Key`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureScheme {
    Ed25519,
    Secp256k1,
    Secp256r1,
}

impl SignatureScheme {
    pub fn from_flag(flag: u8) -> Option<Self> {
        match flag {
            0x00 => Some(SignatureScheme::Ed25519),
            0x01 => Some(SignatureScheme::Secp256k1),
            0x02 => Some(SignatureScheme::Secp256r1),
            _ => None,
        }
    }

    pub fn flag(&self) -> u8 {
        match self {
            SignatureScheme::Ed25519 => 0x00,
            SignatureScheme::Secp256k1 => 0x01,
            SignatureScheme::Secp256r1 => 0x02,
        }
    }

    pub fn public_key_length(&self) -> usize {
        match self {
            SignatureScheme::Ed25519 => 32,
            SignatureScheme::Secp256k1 => 33,
            SignatureScheme::Secp256r1 => 33,
        }
    }
}

impl fmt::Display for SignatureScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignatureScheme::Ed25519 => write!(f, "Ed25519"),
            SignatureScheme::Secp256k1 => write!(f, "Secp256k1"),
            SignatureScheme::Secp256r1 => write!(f, "Secp256r1"),
        }
    }
}
