// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Signature verification for deposit address registration.
//! Supports both MySo signatures and Ethereum EIP-191 signatures.

use crate::error::{BridgeError, BridgeResult};
use alloy::primitives::Address as EthAddress;
use alloy_signer::Signature as EthSignature;
use fastcrypto::encoding::{Base64, Encoding};
use myso_types::base_types::MySoAddress;
use myso_types::crypto::ToFromBytes;
use myso_types::signature::{AuthenticatorTrait, GenericSignature, VerifyParams};
use myso_types::signature_verification::VerifiedDigestCache;
use shared_crypto::intent::{Intent, IntentMessage, PersonalMessage};
use std::str::FromStr;
use std::sync::Arc;
use tracing::{info, warn};

/// Verify a MySo signature on a message
pub fn verify_myso_signature(
    message: &str,
    signature_b64: &str,
    expected_address: &MySoAddress,
) -> BridgeResult<bool> {
    let sig_bytes = Base64::decode(signature_b64)
        .map_err(|e| BridgeError::Generic(format!("Failed to decode MySo signature: {:?}", e)))?;

    let signature = GenericSignature::from_bytes(&sig_bytes)
        .map_err(|e| BridgeError::Generic(format!("Failed to parse MySo signature: {:?}", e)))?;

    let intent_msg = IntentMessage::new(
        Intent::personal_message(),
        PersonalMessage {
            message: message.as_bytes().to_vec(),
        },
    );

    signature
        .verify_claims::<PersonalMessage>(
            &intent_msg,
            *expected_address,
            &VerifyParams::default(),
            Arc::new(VerifiedDigestCache::new_empty()),
        )
        .map_err(|e| {
            warn!(?expected_address, ?e, "MySo signature verification failed");
            BridgeError::Generic(format!("MySo signature verification failed: {:?}", e))
        })?;

    info!(?expected_address, "MySo signature verified successfully");
    Ok(true)
}

/// Verify an Ethereum EIP-191 signature
pub fn verify_eth_signature(
    message: &str,
    signature_hex: &str,
    expected_address: &EthAddress,
) -> BridgeResult<bool> {
    let signature = EthSignature::from_str(signature_hex).map_err(|e| {
        BridgeError::Generic(format!("Failed to parse Ethereum signature: {:?}", e))
    })?;

    let recovered = signature
        .recover_address_from_msg(message)
        .map_err(|e| BridgeError::Generic(format!("Failed to recover Ethereum signer: {:?}", e)))?;

    if &recovered != expected_address {
        warn!(
            ?expected_address,
            ?recovered,
            "Ethereum address mismatch in signature verification"
        );
        return Err(BridgeError::Generic(
            "Ethereum signature verification failed: recovered address does not match".to_string(),
        ));
    }

    info!(
        ?expected_address,
        "Ethereum signature verified successfully"
    );
    Ok(true)
}

/// Verify timestamp is recent (within 5 minutes)
pub fn verify_timestamp_recent(timestamp: u64) -> BridgeResult<bool> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    const MAX_AGE_SECONDS: u64 = 300; // 5 minutes

    if timestamp > now + 60 {
        warn!(timestamp, now, "Timestamp is in the future");
        return Ok(false);
    }

    if now - timestamp > MAX_AGE_SECONDS {
        warn!(
            timestamp,
            now,
            age_seconds = now - timestamp,
            "Timestamp is too old"
        );
        return Ok(false);
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_validation() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert!(verify_timestamp_recent(now).unwrap());
        assert!(verify_timestamp_recent(now - 60).unwrap());
        assert!(verify_timestamp_recent(now - 240).unwrap());
        assert!(!verify_timestamp_recent(now - 600).unwrap());
        assert!(!verify_timestamp_recent(now + 120).unwrap());
    }
}
