// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use ed25519_dalek::{Signer, SigningKey};
use serde::{Deserialize, Serialize};

pub const INTENT_AI_CREDIT_USAGE: u8 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageReceipt {
    pub balance_id: [u8; 32],
    pub agent_object_id: [u8; 32],
    pub receipt_id: u128,
    pub amount_mist: u64,
    pub usage_kind: u8,
    pub timestamp_ms: u64,
    pub settlement_nonce: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct IntentMessage {
    intent: u8,
    timestamp_ms: u64,
    payload: UsageReceipt,
}

pub struct ReceiptSigner {
    signing_key: SigningKey,
}

impl Clone for ReceiptSigner {
    fn clone(&self) -> Self {
        Self {
            signing_key: self.signing_key.clone(),
        }
    }
}

impl ReceiptSigner {
    pub fn from_hex(private_key_hex: &str) -> Result<Self> {
        let bytes = hex::decode(private_key_hex.trim())
            .context("invalid private key hex")?;
        let key_bytes: [u8; 32] = bytes
            .try_into()
            .map_err(|_| anyhow::anyhow!("private key must be 32 bytes"))?;
        Ok(Self {
            signing_key: SigningKey::from_bytes(&key_bytes),
        })
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.signing_key.verifying_key().to_bytes())
    }

    pub fn sign_receipt(&self, receipt: &UsageReceipt) -> Result<Vec<u8>> {
        let intent_message = IntentMessage {
            intent: INTENT_AI_CREDIT_USAGE,
            timestamp_ms: receipt.timestamp_ms,
            payload: receipt.clone(),
        };
        let msg = bcs::to_bytes(&intent_message)?;
        let sig = self.signing_key.sign(&msg);
        Ok(sig.to_bytes().to_vec())
    }
}

pub fn parse_object_id_hex(id: &str) -> Result<[u8; 32]> {
    let trimmed = id.trim_start_matches("0x");
    let bytes = hex::decode(trimmed).context("invalid object id hex")?;
    bytes
        .try_into()
        .map_err(|_| anyhow::anyhow!("object id must be 32 bytes"))
}
