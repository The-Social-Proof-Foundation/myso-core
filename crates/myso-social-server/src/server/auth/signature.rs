// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use blake2::{digest::consts::U32, Blake2b, Digest};
use myso_crypto::{MySoVerifier, UserSignatureVerifier};
use myso_sdk_types::{PersonalMessage, UserSignature};
use std::borrow::Cow;

use super::schemes::SignatureScheme;
use super::wallet::WalletAuthError;

pub const DEFAULT_WALLET_AUTH_TTL_SECONDS: i64 = 300;

pub fn validate_timestamp(timestamp: i64, ttl_seconds: i64) -> Result<(), WalletAuthError> {
    let now = chrono::Utc::now().timestamp();
    let diff = (now - timestamp).abs();
    if diff > ttl_seconds {
        return Err(WalletAuthError::RequestExpired {
            timestamp,
            server_time: now,
            ttl_seconds,
        });
    }
    Ok(())
}

pub fn verify_signature(
    message: &[u8],
    signature_bytes: &[u8],
    public_key_bytes: &[u8],
    scheme: SignatureScheme,
) -> Result<(), WalletAuthError> {
    if signature_bytes.len() != 64 {
        return Err(WalletAuthError::InvalidSignatureFormat(format!(
            "Expected 64 bytes, got {}",
            signature_bytes.len()
        )));
    }

    let expected_len = scheme.public_key_length();
    if public_key_bytes.len() != expected_len {
        return Err(WalletAuthError::InvalidPublicKeyFormat(format!(
            "Expected {} bytes for {}, got {}",
            expected_len,
            scheme,
            public_key_bytes.len()
        )));
    }

    let mut serialized_sig = Vec::with_capacity(1 + 64 + expected_len);
    serialized_sig.push(scheme.flag());
    serialized_sig.extend_from_slice(signature_bytes);
    serialized_sig.extend_from_slice(public_key_bytes);

    let user_signature = UserSignature::from_bytes(&serialized_sig).map_err(|e| {
        WalletAuthError::InvalidSignatureFormat(format!("Failed to parse signature: {}", e))
    })?;

    let personal_message = PersonalMessage(Cow::Borrowed(message));
    UserSignatureVerifier::default()
        .verify_personal_message(&personal_message, &user_signature)
        .map_err(|e| WalletAuthError::SignatureVerificationFailed(e.to_string()))?;

    Ok(())
}

pub fn derive_myso_address(
    public_key_bytes: &[u8],
    scheme: SignatureScheme,
) -> Result<String, WalletAuthError> {
    let expected_len = scheme.public_key_length();
    if public_key_bytes.len() != expected_len {
        return Err(WalletAuthError::InvalidPublicKeyFormat(format!(
            "Expected {} bytes for {}, got {}",
            expected_len,
            scheme,
            public_key_bytes.len()
        )));
    }

    let mut hash_input = vec![scheme.flag()];
    hash_input.extend_from_slice(public_key_bytes);

    type Blake2b256 = Blake2b<U32>;
    let hash = Blake2b256::digest(&hash_input);
    Ok(format!("0x{}", hex::encode(hash)))
}

pub fn verify_address_matches_pubkey(
    claimed_address: &str,
    public_key_bytes: &[u8],
    scheme: SignatureScheme,
) -> Result<String, WalletAuthError> {
    let derived_address = derive_myso_address(public_key_bytes, scheme)?;
    if claimed_address != derived_address {
        return Err(WalletAuthError::AddressMismatch {
            expected: derived_address,
            got: claimed_address.to_string(),
        });
    }
    Ok(derived_address)
}
