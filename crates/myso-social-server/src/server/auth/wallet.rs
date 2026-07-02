// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Wallet-scoped authentication for bodyless GET requests.
//!
//! Signed message: `timestamp:sender_address`
//!
//! Headers:
//! - `X-Signature`: hex-encoded 64-byte raw signature
//! - `X-Public-Key`: hex-encoded (flag_byte || public_key_bytes)
//! - `X-Sender-Address`: claimed MySo address
//! - `X-Timestamp`: unix seconds

use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use http_body_util::BodyExt;
use serde::Serialize;
use std::sync::Arc;

use super::schemes::SignatureScheme;
use super::signature::{validate_timestamp, verify_address_matches_pubkey, verify_signature};
use crate::server::AppState;

#[derive(Debug, Clone)]
pub struct WalletAuthContext {
    pub sender_address: String,
    pub public_key: Vec<u8>,
    pub scheme: SignatureScheme,
}

#[derive(Debug, Clone)]
pub enum WalletAuthError {
    InvalidPublicKeyFormat(String),
    InvalidSignatureFormat(String),
    SignatureVerificationFailed(String),
    RequestExpired {
        timestamp: i64,
        server_time: i64,
        ttl_seconds: i64,
    },
    AddressMismatch { expected: String, got: String },
}

impl std::fmt::Display for WalletAuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalletAuthError::InvalidPublicKeyFormat(msg) => write!(f, "{msg}"),
            WalletAuthError::InvalidSignatureFormat(msg) => write!(f, "{msg}"),
            WalletAuthError::SignatureVerificationFailed(msg) => write!(f, "{msg}"),
            WalletAuthError::RequestExpired {
                timestamp,
                server_time,
                ttl_seconds,
            } => write!(
                f,
                "Request expired: timestamp {timestamp} is more than {ttl_seconds}s from server time {server_time}"
            ),
            WalletAuthError::AddressMismatch { expected, got } => {
                write!(f, "Address mismatch: expected {expected}, got {got}")
            }
        }
    }
}

#[derive(Serialize)]
struct AuthErrorResponse {
    error: String,
    code: String,
}

fn get_header(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.trim().to_string())
}

fn error_response(status: StatusCode, error: impl Into<String>, code: &str) -> Response {
    (
        status,
        Json(AuthErrorResponse {
            error: error.into(),
            code: code.to_string(),
        }),
    )
        .into_response()
}

fn auth_error_response(status: StatusCode, err: WalletAuthError) -> Response {
    error_response(status, err.to_string(), "AUTH_ERROR")
}

pub async fn wallet_auth_middleware(
    State(state): State<Arc<AppState>>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let (parts, body) = request.into_parts();

    let signature_hex = match get_header(&parts.headers, "x-signature") {
        Some(v) => v,
        None => {
            return error_response(
                StatusCode::UNAUTHORIZED,
                "Missing X-Signature header",
                "MISSING_SIGNATURE",
            );
        }
    };

    let public_key_hex = match get_header(&parts.headers, "x-public-key") {
        Some(v) => v,
        None => {
            return error_response(
                StatusCode::UNAUTHORIZED,
                "Missing X-Public-Key header",
                "MISSING_PUBLIC_KEY",
            );
        }
    };

    let public_key_with_flag = match hex::decode(&public_key_hex) {
        Ok(bytes) if !bytes.is_empty() => bytes,
        Ok(_) => {
            return auth_error_response(
                StatusCode::UNAUTHORIZED,
                WalletAuthError::InvalidPublicKeyFormat("Empty public key".to_string()),
            );
        }
        Err(e) => {
            return auth_error_response(
                StatusCode::UNAUTHORIZED,
                WalletAuthError::InvalidPublicKeyFormat(e.to_string()),
            );
        }
    };

    let scheme_flag = public_key_with_flag[0];
    let scheme = match SignatureScheme::from_flag(scheme_flag) {
        Some(s) => s,
        None => {
            return auth_error_response(
                StatusCode::UNAUTHORIZED,
                WalletAuthError::InvalidPublicKeyFormat(format!(
                    "Unknown signature scheme flag: 0x{scheme_flag:02x}"
                )),
            );
        }
    };
    let public_key_bytes = &public_key_with_flag[1..];
    if public_key_bytes.len() != scheme.public_key_length() {
        return auth_error_response(
            StatusCode::UNAUTHORIZED,
            WalletAuthError::InvalidPublicKeyFormat(format!(
                "Expected {} bytes for {}, got {}",
                scheme.public_key_length(),
                scheme,
                public_key_bytes.len()
            )),
        );
    }

    let signature_bytes = match hex::decode(&signature_hex) {
        Ok(bytes) => bytes,
        Err(e) => {
            return auth_error_response(
                StatusCode::UNAUTHORIZED,
                WalletAuthError::InvalidSignatureFormat(e.to_string()),
            );
        }
    };

    let body_bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "Failed to read request body",
                "BODY_READ_ERROR",
            );
        }
    };

    if !body_bytes.is_empty() {
        return error_response(
            StatusCode::BAD_REQUEST,
            "Wallet auth on enterprise reads expects an empty GET body",
            "UNEXPECTED_BODY",
        );
    }

    let sender_address = match get_header(&parts.headers, "x-sender-address") {
        Some(v) => v,
        None => {
            return error_response(
                StatusCode::UNAUTHORIZED,
                "Missing X-Sender-Address header",
                "MISSING_SENDER_ADDRESS",
            );
        }
    };
    let timestamp_str = match get_header(&parts.headers, "x-timestamp") {
        Some(v) => v,
        None => {
            return error_response(
                StatusCode::UNAUTHORIZED,
                "Missing X-Timestamp header",
                "MISSING_TIMESTAMP",
            );
        }
    };
    let timestamp: i64 = match timestamp_str.parse() {
        Ok(t) => t,
        Err(_) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "Invalid X-Timestamp header",
                "INVALID_TIMESTAMP",
            );
        }
    };

    let message_bytes = format!("{timestamp}:{sender_address}").into_bytes();

    if let Err(e) = validate_timestamp(timestamp, state.wallet_auth_ttl_seconds) {
        return auth_error_response(StatusCode::UNAUTHORIZED, e);
    }

    if let Err(e) = verify_signature(&message_bytes, &signature_bytes, public_key_bytes, scheme) {
        return auth_error_response(StatusCode::UNAUTHORIZED, e);
    }

    if let Err(e) = verify_address_matches_pubkey(&sender_address, public_key_bytes, scheme) {
        return auth_error_response(StatusCode::UNAUTHORIZED, e);
    }

    let auth_context = WalletAuthContext {
        sender_address,
        public_key: public_key_bytes.to_vec(),
        scheme,
    };
    tracing::debug!(
        sender = %auth_context.sender_address,
        scheme = ?auth_context.scheme,
        pubkey_len = auth_context.public_key.len(),
        "wallet authenticated"
    );

    let mut request = Request::from_parts(parts, Body::from(body_bytes));
    request.extensions_mut().insert(auth_context);
    next.run(request).await
}
