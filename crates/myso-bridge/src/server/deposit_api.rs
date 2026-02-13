// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Deposit address registration API
//! Provides endpoints for generating custodial deposit addresses

use crate::deposit_addresses::{DepositAddressManager, HD_COUNTER_EVM, HD_COUNTER_MYSO};
use crate::deposit_sig_verification::{
    verify_eth_signature, verify_myso_signature, verify_timestamp_recent,
};
use crate::error::BridgeError;
use crate::storage::{
    BridgeOrchestratorTables, DepositAddressKey, DepositRegistration, RegistrationType,
};
use alloy::primitives::Address as EthAddress;
use axum::{Json, extract::State, http::StatusCode};
use fastcrypto::encoding::Encoding;
use myso_types::base_types::MySoAddress;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;
use tracing::{error, info};

/// Shared state for deposit API
pub struct DepositApiState {
    pub address_manager: Arc<DepositAddressManager>,
    pub storage: Arc<BridgeOrchestratorTables>,
    /// MySocial chain ID from bridge config
    pub myso_chain_id: u8,
    /// EVM chain ID from bridge config
    pub eth_chain_id: u8,
}

/// Request to generate a deposit address (Option A)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateDepositRequest {
    pub auth_type: AuthType,
    pub source_address: Option<String>,
    pub signature: Option<String>,
    pub message: MessagePayload,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthType {
    MySocial,
    Ethereum,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagePayload {
    pub action: String,
    pub destination_chain: String,
    pub destination_address: String,
    pub timestamp: u64,
}

/// Response with generated deposit address
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateDepositResponse {
    pub deposit_chain: String,
    pub deposit_address: String,
    pub destination_chain: String,
    pub destination_address: String,
    pub instructions: String,
}

/// Request to link addresses with both signatures (Option B)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkAddressesRequest {
    pub myso_address: String,
    pub myso_signature: String,
    pub eth_address: String,
    pub eth_signature: String,
    pub timestamp: u64,
}

/// Response for linked addresses
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkAddressesResponse {
    pub myso_deposit_address: String,
    pub evm_deposit_address: String,
    pub linked_myso_address: String,
    pub linked_eth_address: String,
    pub status: String,
}

/// Error response structure for JSON error responses
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub error: String,
}

/// Handler for generating deposit address (Option A)
pub async fn generate_deposit_address(
    State(state): State<Arc<DepositApiState>>,
    Json(req): Json<GenerateDepositRequest>,
) -> Result<Json<GenerateDepositResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!(?req.auth_type, "Received deposit address generation request");

    if req.signature.is_some() {
        if !verify_timestamp_recent(req.message.timestamp).map_err(to_status_error_json)? {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Timestamp too old or invalid".to_string(),
                }),
            ));
        }
    }

    let message_str = format!(
        "Generate deposit for {}:{} at {}",
        req.message.destination_chain, req.message.destination_address, req.message.timestamp
    );

    match req.auth_type {
        AuthType::MySocial => generate_for_myso_user(state, req, message_str).await,
        AuthType::Ethereum => generate_for_eth_user(state, req, message_str).await,
    }
}

/// Generate deposit address for MySocial user (wants to bridge TO EVM)
async fn generate_for_myso_user(
    state: Arc<DepositApiState>,
    req: GenerateDepositRequest,
    message_str: String,
) -> Result<Json<GenerateDepositResponse>, (StatusCode, Json<ErrorResponse>)> {
    let source_address = req.source_address.as_ref().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "source_address is required for MySocial auth type".to_string(),
            }),
        )
    })?;

    let myso_address = MySoAddress::from_str(source_address).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Invalid MySocial address: {:?}", e),
            }),
        )
    })?;

    if let Some(signature) = &req.signature {
        verify_myso_signature(&message_str, signature, &myso_address)
            .map_err(to_status_error_json)?;
    }

    let dest_eth_address = EthAddress::from_str(&req.message.destination_address).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Invalid destination address: {:?}", e),
            }),
        )
    })?;

    let dest_chain_id = parse_chain_id(
        &req.message.destination_chain,
        state.eth_chain_id,
        state.myso_chain_id,
    )?;

    let existing_registrations = state
        .storage
        .get_deposit_registrations(&DepositAddressKey::from_myso(myso_address))
        .map_err(to_status_error_json)?;

    if let Some(registrations) = existing_registrations {
        if let Some(existing_reg) = registrations.iter().find(|reg| {
            reg.deposit_chain == state.myso_chain_id && reg.destination_chain == dest_chain_id
        }) {
            if existing_reg.deposit_address.len() != 32 {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!(
                            "Invalid deposit address length: expected 32 bytes (MySocial address), got {} bytes",
                            existing_reg.deposit_address.len()
                        ),
                    }),
                ));
            }

            let existing_deposit_addr = MySoAddress::from_bytes(&existing_reg.deposit_address)
                .map(|addr| format!("{}", addr))
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: format!("Failed to parse MySocial deposit address: {:?}", e),
                        }),
                    )
                })?;

            info!(
                ?myso_address,
                ?existing_deposit_addr,
                "Returning existing MySocial deposit address for MySocial user"
            );

            return Ok(Json(GenerateDepositResponse {
                deposit_chain: chain_id_to_name(state.myso_chain_id),
                deposit_address: existing_deposit_addr.clone(),
                destination_chain: req.message.destination_chain.clone(),
                destination_address: req.message.destination_address.clone(),
                instructions: format!(
                    "Send tokens to {} on MySocial chain, they will bridge to {} on {}",
                    existing_deposit_addr, dest_eth_address, req.message.destination_chain
                ),
            }));
        }
    }

    let hd_index = state
        .address_manager
        .allocate_next_index(HD_COUNTER_MYSO)
        .map_err(to_status_error_json)?;

    let (deposit_myso_address, _) = state
        .address_manager
        .derive_myso_deposit_address(hd_index)
        .map_err(to_status_error_json)?;

    let registration = DepositRegistration {
        deposit_chain: state.myso_chain_id,
        deposit_address: deposit_myso_address.to_vec(),
        destination_chain: dest_chain_id,
        destination_address: dest_eth_address.as_slice().to_vec(),
        hd_index,
        registration_type: RegistrationType::ApiMySoSig,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        last_used: None,
    };

    state
        .storage
        .store_deposit_registration(DepositAddressKey::from_myso(myso_address), registration)
        .map_err(to_status_error_json)?;

    info!(
        ?myso_address,
        ?deposit_myso_address,
        ?dest_eth_address,
        "Generated MySocial deposit address for MySocial user"
    );

    Ok(Json(GenerateDepositResponse {
        deposit_chain: chain_id_to_name(state.myso_chain_id),
        deposit_address: format!("{}", deposit_myso_address),
        destination_chain: req.message.destination_chain.clone(),
        destination_address: req.message.destination_address.clone(),
        instructions: format!(
            "Send tokens to {} on MySocial chain, they will bridge to {} on {}",
            deposit_myso_address, dest_eth_address, req.message.destination_chain
        ),
    }))
}

/// Generate deposit address for EVM user (wants to bridge TO MySocial)
async fn generate_for_eth_user(
    state: Arc<DepositApiState>,
    req: GenerateDepositRequest,
    message_str: String,
) -> Result<Json<GenerateDepositResponse>, (StatusCode, Json<ErrorResponse>)> {
    let dest_myso_address =
        MySoAddress::from_str(&req.message.destination_address).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Invalid MySocial destination address: {:?}", e),
                }),
            )
        })?;

    if let (Some(source_address), Some(signature)) = (&req.source_address, &req.signature) {
        if let Ok(eth_addr) = EthAddress::from_str(source_address) {
            verify_eth_signature(&message_str, signature, &eth_addr)
                .map_err(to_status_error_json)?;
        } else if let Ok(myso_addr) = MySoAddress::from_str(source_address) {
            verify_myso_signature(&message_str, signature, &myso_addr)
                .map_err(to_status_error_json)?;
        } else {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: "Invalid source address format".to_string(),
                }),
            ));
        }
    }

    let dest_chain_id = parse_chain_id(
        &req.message.destination_chain,
        state.eth_chain_id,
        state.myso_chain_id,
    )?;
    let source_chain_id = state.eth_chain_id;

    let existing_registrations = state
        .storage
        .get_deposit_registrations(&DepositAddressKey::from_myso(dest_myso_address))
        .map_err(to_status_error_json)?;

    if let Some(registrations) = existing_registrations {
        if let Some(existing_reg) = registrations.iter().find(|reg| {
            reg.deposit_chain == source_chain_id && reg.destination_chain == dest_chain_id
        }) {
            if existing_reg.deposit_address.len() != 20 {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!(
                            "Invalid deposit address length: expected 20 bytes (EVM address), got {} bytes",
                            existing_reg.deposit_address.len()
                        ),
                    }),
                ));
            }

            let existing_deposit_addr = EthAddress::from_slice(&existing_reg.deposit_address);

            info!(
                ?dest_myso_address,
                ?existing_deposit_addr,
                "Returning existing EVM deposit address for MySocial user"
            );

            return Ok(Json(GenerateDepositResponse {
                deposit_chain: chain_id_to_name(state.eth_chain_id),
                deposit_address: format!("{:?}", existing_deposit_addr),
                destination_chain: req.message.destination_chain.clone(),
                destination_address: req.message.destination_address.clone(),
                instructions: format!(
                    "Send tokens to {:?} on {} chain, they will bridge to {} on MySocial",
                    existing_deposit_addr,
                    chain_id_to_name(state.eth_chain_id),
                    dest_myso_address
                ),
            }));
        }
    }

    let hd_index = state
        .address_manager
        .allocate_next_index(HD_COUNTER_EVM)
        .map_err(to_status_error_json)?;

    let (deposit_evm_address, _) = state
        .address_manager
        .derive_evm_deposit_address(hd_index)
        .map_err(to_status_error_json)?;

    let registration = DepositRegistration {
        deposit_chain: source_chain_id,
        deposit_address: deposit_evm_address.as_slice().to_vec(),
        destination_chain: dest_chain_id,
        destination_address: dest_myso_address.to_vec(),
        hd_index,
        registration_type: RegistrationType::ApiEthSig,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        last_used: None,
    };

    state
        .storage
        .store_deposit_registration(
            DepositAddressKey::from_myso(dest_myso_address),
            registration,
        )
        .map_err(to_status_error_json)?;

    info!(
        ?dest_myso_address,
        ?deposit_evm_address,
        "Generated EVM deposit address for MySocial user"
    );

    Ok(Json(GenerateDepositResponse {
        deposit_chain: chain_id_to_name(state.eth_chain_id),
        deposit_address: format!("{:?}", deposit_evm_address),
        destination_chain: req.message.destination_chain.clone(),
        destination_address: req.message.destination_address.clone(),
        instructions: format!(
            "Send tokens to {} on {} chain, they will bridge to {} on MySocial",
            deposit_evm_address,
            chain_id_to_name(state.eth_chain_id),
            dest_myso_address
        ),
    }))
}

/// Handler for linking addresses (Option B)
pub async fn link_addresses(
    State(state): State<Arc<DepositApiState>>,
    Json(req): Json<LinkAddressesRequest>,
) -> Result<Json<LinkAddressesResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!("Received address linking request");

    if !verify_timestamp_recent(req.timestamp).map_err(to_status_error_json)? {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Timestamp too old or invalid".to_string(),
            }),
        ));
    }

    let myso_address = MySoAddress::from_str(&req.myso_address).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Invalid MySocial address: {:?}", e),
            }),
        )
    })?;

    let eth_address = EthAddress::from_str(&req.eth_address).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Invalid Ethereum address: {:?}", e),
            }),
        )
    })?;

    let myso_message = format!("Link to ETH {} at {}", req.eth_address, req.timestamp);
    verify_myso_signature(&myso_message, &req.myso_signature, &myso_address)
        .map_err(to_status_error_json)?;

    let eth_message = format!("Link to MYSO {} at {}", req.myso_address, req.timestamp);
    verify_eth_signature(&eth_message, &req.eth_signature, &eth_address)
        .map_err(to_status_error_json)?;

    let myso_hd_index = state
        .address_manager
        .allocate_next_index(HD_COUNTER_MYSO)
        .map_err(to_status_error_json)?;

    let evm_hd_index = state
        .address_manager
        .allocate_next_index(HD_COUNTER_EVM)
        .map_err(to_status_error_json)?;

    let (deposit_myso_address, _) = state
        .address_manager
        .derive_myso_deposit_address(myso_hd_index)
        .map_err(to_status_error_json)?;

    let (deposit_evm_address, _) = state
        .address_manager
        .derive_evm_deposit_address(evm_hd_index)
        .map_err(to_status_error_json)?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let myso_registration = DepositRegistration {
        deposit_chain: state.myso_chain_id,
        deposit_address: deposit_myso_address.to_vec(),
        destination_chain: state.eth_chain_id,
        destination_address: eth_address.as_slice().to_vec(),
        hd_index: myso_hd_index,
        registration_type: RegistrationType::Linked,
        created_at: now,
        last_used: None,
    };

    state
        .storage
        .store_deposit_registration(
            DepositAddressKey::from_myso(myso_address),
            myso_registration,
        )
        .map_err(to_status_error_json)?;

    let evm_registration = DepositRegistration {
        deposit_chain: state.eth_chain_id,
        deposit_address: deposit_evm_address.as_slice().to_vec(),
        destination_chain: state.myso_chain_id,
        destination_address: myso_address.to_vec(),
        hd_index: evm_hd_index,
        registration_type: RegistrationType::Linked,
        created_at: now,
        last_used: None,
    };

    state
        .storage
        .store_deposit_registration(DepositAddressKey::from_evm(eth_address), evm_registration)
        .map_err(to_status_error_json)?;

    info!(
        ?myso_address,
        ?eth_address,
        ?deposit_myso_address,
        ?deposit_evm_address,
        "Linked addresses successfully"
    );

    Ok(Json(LinkAddressesResponse {
        myso_deposit_address: format!("{}", deposit_myso_address),
        evm_deposit_address: format!("{:?}", deposit_evm_address),
        linked_myso_address: req.myso_address,
        linked_eth_address: req.eth_address,
        status: "linked".to_string(),
    }))
}

/// Query deposit addresses for a given address
pub async fn query_deposit_addresses(
    State(state): State<Arc<DepositApiState>>,
    axum::extract::Path(address): axum::extract::Path<String>,
) -> Result<Json<QueryDepositResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!(address, "Querying deposit addresses");

    if let Ok(myso_addr) = MySoAddress::from_str(&address) {
        let registrations = state
            .storage
            .get_deposit_registrations(&DepositAddressKey::from_myso(myso_addr))
            .map_err(to_status_error_json)?
            .unwrap_or_default();

        return Ok(Json(QueryDepositResponse {
            source_address: address,
            registrations: registrations
                .into_iter()
                .map(|r| RegistrationInfo {
                    deposit_chain: chain_id_to_name(r.deposit_chain),
                    deposit_address: format_address(&r.deposit_address, r.deposit_chain),
                    destination_chain: chain_id_to_name(r.destination_chain),
                    destination_address: format_address(
                        &r.destination_address,
                        r.destination_chain,
                    ),
                    registration_type: format!("{:?}", r.registration_type),
                    created_at: r.created_at,
                })
                .collect(),
        }));
    }

    if let Ok(eth_addr) = EthAddress::from_str(&address) {
        let registrations = state
            .storage
            .get_deposit_registrations(&DepositAddressKey::from_evm(eth_addr))
            .map_err(to_status_error_json)?
            .unwrap_or_default();

        return Ok(Json(QueryDepositResponse {
            source_address: address,
            registrations: registrations
                .into_iter()
                .map(|r| RegistrationInfo {
                    deposit_chain: chain_id_to_name(r.deposit_chain),
                    deposit_address: format_address(&r.deposit_address, r.deposit_chain),
                    destination_chain: chain_id_to_name(r.destination_chain),
                    destination_address: format_address(
                        &r.destination_address,
                        r.destination_chain,
                    ),
                    registration_type: format!("{:?}", r.registration_type),
                    created_at: r.created_at,
                })
                .collect(),
        }));
    }

    Err((
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: "Invalid address format".to_string(),
        }),
    ))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryDepositResponse {
    pub source_address: String,
    pub registrations: Vec<RegistrationInfo>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationInfo {
    pub deposit_chain: String,
    pub deposit_address: String,
    pub destination_chain: String,
    pub destination_address: String,
    pub registration_type: String,
    pub created_at: u64,
}

fn parse_chain_id(
    chain_name: &str,
    eth_chain_id: u8,
    myso_chain_id: u8,
) -> Result<u8, (StatusCode, Json<ErrorResponse>)> {
    match chain_name.to_lowercase().as_str() {
        "mysocial" | "myso" => Ok(myso_chain_id),
        "base" | "base-sepolia" | "ethereum" | "eth" => Ok(eth_chain_id),
        _ => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Unsupported chain: {}", chain_name),
            }),
        )),
    }
}

fn chain_id_to_name(chain_id: u8) -> String {
    match chain_id {
        1 => "ethereum".to_string(),
        2 => "mysocial".to_string(),
        12 => "base".to_string(),
        _ => format!("chain_{}", chain_id),
    }
}

fn format_address(address_bytes: &[u8], _chain_id: u8) -> String {
    if address_bytes.len() == 32 {
        if let Ok(addr) = MySoAddress::from_bytes(address_bytes) {
            return format!("{}", addr);
        }
    }
    if address_bytes.len() == 20 {
        return format!("{:?}", EthAddress::from_slice(address_bytes));
    }
    format!("0x{}", fastcrypto::encoding::Hex::encode(address_bytes))
}

fn to_status_error_json(err: BridgeError) -> (StatusCode, Json<ErrorResponse>) {
    error!(?err, "API error");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            error: format!("{:?}", err),
        }),
    )
}
