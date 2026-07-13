// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use anyhow::Result;
use axum::http::{HeaderMap, StatusCode};
use blake2::{digest::consts::U32, Blake2b, Digest};
use myso_crypto::{MySoVerifier, UserSignatureVerifier};
use myso_sdk_types::{PersonalMessage, UserSignature};

use crate::config::OracleArgs;
use crate::signature_schemes::SignatureScheme;
use crate::sub_agent_object::{address_to_hex, fetch_on_chain_sub_agent};

pub const DEFAULT_AGENT_AUTH_TTL_SECONDS: i64 = 300;
pub const ORACLE_SECRET_HEADER: &str = "x-ai-credit-oracle-secret";
pub const SIGNATURE_HEADER: &str = "x-signature";
pub const PUBLIC_KEY_HEADER: &str = "x-public-key";
pub const AGENT_ADDRESS_HEADER: &str = "x-agent-address";
pub const TIMESTAMP_HEADER: &str = "x-timestamp";

#[derive(Debug)]
pub enum AgentAuthError {
    MissingHeader(&'static str),
    InvalidFormat(String),
    RequestExpired { timestamp: i64, server_time: i64 },
    SignatureVerificationFailed(String),
    AddressMismatch { expected: String, got: String },
    ChainBindingFailed(String),
}

impl AgentAuthError {
    pub fn status(&self) -> StatusCode {
        match self {
            AgentAuthError::MissingHeader(_) => StatusCode::UNAUTHORIZED,
            AgentAuthError::InvalidFormat(_) => StatusCode::BAD_REQUEST,
            AgentAuthError::RequestExpired { .. } => StatusCode::UNAUTHORIZED,
            AgentAuthError::SignatureVerificationFailed(_) => StatusCode::UNAUTHORIZED,
            AgentAuthError::AddressMismatch { .. } => StatusCode::UNAUTHORIZED,
            AgentAuthError::ChainBindingFailed(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

pub fn check_oracle_api_secret(
    headers: &HeaderMap,
    secret: &Option<String>,
) -> Result<(), StatusCode> {
    let Some(expected) = secret else {
        return Ok(());
    };
    let provided = headers
        .get(ORACLE_SECRET_HEADER)
        .and_then(|v| v.to_str().ok());
    if provided != Some(expected.as_str()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(())
}

pub fn build_usage_auth_message(
    timestamp_ms: i64,
    balance_id: &str,
    agent_object_id: &str,
    usage_kind: u8,
    tokens_in: Option<u64>,
    tokens_out: Option<u64>,
    model_id: &Option<String>,
    tool_id: &Option<String>,
    idempotency_key: &str,
) -> String {
    format!(
        "ai_credit_usage_v1:{timestamp_ms}:{balance_id}:{agent_object_id}:{usage_kind}:{}:{}:{}:{}:{idempotency_key}",
        tokens_in.unwrap_or(0),
        tokens_out.unwrap_or(0),
        model_id.as_deref().unwrap_or(""),
        tool_id.as_deref().unwrap_or(""),
    )
}

pub fn validate_timestamp(timestamp: i64, ttl_seconds: i64) -> Result<(), AgentAuthError> {
    let now = chrono::Utc::now().timestamp();
    let diff = (now - timestamp).abs();
    if diff > ttl_seconds {
        return Err(AgentAuthError::RequestExpired {
            timestamp,
            server_time: now,
        });
    }
    Ok(())
}

pub fn verify_signature(
    message: &[u8],
    signature_bytes: &[u8],
    public_key_bytes: &[u8],
    scheme: SignatureScheme,
) -> Result<(), AgentAuthError> {
    if signature_bytes.len() != 64 {
        return Err(AgentAuthError::InvalidFormat(format!(
            "Expected 64 bytes, got {}",
            signature_bytes.len()
        )));
    }

    let expected_len = scheme.public_key_length();
    if public_key_bytes.len() != expected_len {
        return Err(AgentAuthError::InvalidFormat(format!(
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

    let user_signature = UserSignature::from_bytes(&serialized_sig)
        .map_err(|e| AgentAuthError::InvalidFormat(format!("Failed to parse signature: {}", e)))?;

    let personal_message = PersonalMessage(Cow::Borrowed(message));
    UserSignatureVerifier::default()
        .verify_personal_message(&personal_message, &user_signature)
        .map_err(|e| AgentAuthError::SignatureVerificationFailed(e.to_string()))?;

    Ok(())
}

pub fn derive_myso_address(
    public_key_bytes: &[u8],
    scheme: SignatureScheme,
) -> Result<String, AgentAuthError> {
    let expected_len = scheme.public_key_length();
    if public_key_bytes.len() != expected_len {
        return Err(AgentAuthError::InvalidFormat(format!(
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
) -> Result<String, AgentAuthError> {
    let derived_address = derive_myso_address(public_key_bytes, scheme)?;
    if claimed_address != derived_address {
        return Err(AgentAuthError::AddressMismatch {
            expected: derived_address,
            got: claimed_address.to_string(),
        });
    }
    Ok(derived_address)
}

fn parse_public_key_header(value: &str) -> Result<(SignatureScheme, Vec<u8>), AgentAuthError> {
    let bytes = hex::decode(value.strip_prefix("0x").unwrap_or(value))
        .map_err(|e| AgentAuthError::InvalidFormat(format!("Invalid public key hex: {}", e)))?;
    if bytes.is_empty() {
        return Err(AgentAuthError::InvalidFormat(
            "Empty public key header".to_string(),
        ));
    }
    let scheme = SignatureScheme::from_flag(bytes[0]).ok_or_else(|| {
        AgentAuthError::InvalidFormat("Unknown signature scheme flag".to_string())
    })?;
    let key_bytes = bytes[1..].to_vec();
    Ok((scheme, key_bytes))
}

pub async fn verify_agent_usage_auth(
    headers: &HeaderMap,
    balance_id: &str,
    agent_object_id: &str,
    usage_kind: u8,
    tokens_in: Option<u64>,
    tokens_out: Option<u64>,
    model_id: &Option<String>,
    tool_id: &Option<String>,
    idempotency_key: &str,
    args: &OracleArgs,
) -> Result<(), AgentAuthError> {
    if !args.agent_auth_enabled {
        return Ok(());
    }

    let signature_hex = headers
        .get(SIGNATURE_HEADER)
        .and_then(|v| v.to_str().ok())
        .ok_or(AgentAuthError::MissingHeader(SIGNATURE_HEADER))?;
    let public_key_hex = headers
        .get(PUBLIC_KEY_HEADER)
        .and_then(|v| v.to_str().ok())
        .ok_or(AgentAuthError::MissingHeader(PUBLIC_KEY_HEADER))?;
    let agent_address = headers
        .get(AGENT_ADDRESS_HEADER)
        .and_then(|v| v.to_str().ok())
        .ok_or(AgentAuthError::MissingHeader(AGENT_ADDRESS_HEADER))?;
    let timestamp_str = headers
        .get(TIMESTAMP_HEADER)
        .and_then(|v| v.to_str().ok())
        .ok_or(AgentAuthError::MissingHeader(TIMESTAMP_HEADER))?;

    let timestamp_ms: i64 = timestamp_str
        .parse()
        .map_err(|_| AgentAuthError::InvalidFormat("Invalid X-Timestamp".to_string()))?;
    validate_timestamp(timestamp_ms / 1000, args.agent_auth_ttl_secs)?;

    let signature_bytes = hex::decode(signature_hex.strip_prefix("0x").unwrap_or(signature_hex))
        .map_err(|e| AgentAuthError::InvalidFormat(format!("Invalid signature hex: {}", e)))?;
    let (scheme, public_key_bytes) = parse_public_key_header(public_key_hex)?;
    verify_address_matches_pubkey(agent_address, &public_key_bytes, scheme)?;

    let message = build_usage_auth_message(
        timestamp_ms,
        balance_id,
        agent_object_id,
        usage_kind,
        tokens_in,
        tokens_out,
        model_id,
        tool_id,
        idempotency_key,
    );
    verify_signature(
        message.as_bytes(),
        &signature_bytes,
        &public_key_bytes,
        scheme,
    )?;

    let on_chain = fetch_on_chain_sub_agent(&args.myso_rpc, agent_object_id)
        .await
        .map_err(|e| AgentAuthError::ChainBindingFailed(e.to_string()))?;
    let chain_address = address_to_hex(&on_chain.derived_address);
    if chain_address != agent_address {
        return Err(AgentAuthError::AddressMismatch {
            expected: chain_address,
            got: agent_address.to_string(),
        });
    }
    if on_chain.public_key != public_key_bytes {
        return Err(AgentAuthError::ChainBindingFailed(
            "public key does not match on-chain SubAgent".to_string(),
        ));
    }

    Ok(())
}

pub fn agent_auth_error_to_status(err: AgentAuthError) -> StatusCode {
    tracing::warn!(error = ?err, "agent auth rejected");
    err.status()
}

pub fn derive_receipt_id(idempotency_key: &str, balance_id: &str, agent_object_id: &str) -> u128 {
    type Blake2b256 = Blake2b<U32>;
    let mut hasher = Blake2b256::new();
    hasher.update(idempotency_key.as_bytes());
    hasher.update(balance_id.as_bytes());
    hasher.update(agent_object_id.as_bytes());
    let hash = hasher.finalize();
    u128::from_le_bytes(hash[..16].try_into().expect("blake2b256 produces 32 bytes"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_receipt_id_is_deterministic() {
        let a = derive_receipt_id("key-1", "0xbal", "0xagent");
        let b = derive_receipt_id("key-1", "0xbal", "0xagent");
        assert_eq!(a, b);
        let c = derive_receipt_id("key-2", "0xbal", "0xagent");
        assert_ne!(a, c);
    }

    #[test]
    fn ed25519_usage_message_roundtrip() {
        use myso_types::crypto::{get_key_pair, AccountKeyPair, MySoSignature, Signature};
        use shared_crypto::intent::{Intent, IntentMessage, PersonalMessage};

        let (_addr, keypair): (_, AccountKeyPair) = get_key_pair();
        let scheme = SignatureScheme::Ed25519;
        let timestamp_ms = 1_700_000_000_000i64;
        let message = build_usage_auth_message(
            timestamp_ms,
            "0xbalance",
            "0xagent",
            1,
            Some(100),
            Some(50),
            &Some("openai/gpt-4o-mini".to_string()),
            &None,
            "idem-key-abc",
        );
        let intent_message = IntentMessage::new(
            Intent::personal_message(),
            PersonalMessage {
                message: message.as_bytes().to_vec(),
            },
        );
        let signature = Signature::new_secure(&intent_message, &keypair);
        verify_signature(
            message.as_bytes(),
            signature.signature_bytes(),
            signature.public_key_bytes(),
            scheme,
        )
        .unwrap();
        let derived = derive_myso_address(signature.public_key_bytes(), scheme).unwrap();
        verify_address_matches_pubkey(&derived, signature.public_key_bytes(), scheme).unwrap();
    }
}
