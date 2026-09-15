// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Platform-facing bridge order API. Orders are the source of truth for lifecycle status.

use crate::server::deposit_api::{
    AuthType, DepositApiState, ErrorResponse, GenerateDepositRequest, MessagePayload,
    generate_deposit_address,
};
use crate::storage::BridgeOrderDirection;
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::info;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBridgeOrderRequest {
    pub direction: BridgeOrderDirection,
    pub myso_wallet: String,
    pub destination_chain: Option<String>,
    pub destination_address: Option<String>,
    pub amount: Option<String>,
    #[serde(default)]
    pub callback_url: Option<String>,
    #[serde(default)]
    pub callback_api_key: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BridgeOrderResponse {
    pub order_id: String,
    pub direction: BridgeOrderDirection,
    pub status: String,
    pub myso_wallet: String,
    pub deposit_address: String,
    pub destination_chain: String,
    pub destination_address: String,
    pub amount: Option<String>,
    pub deposit_tx_digest: Option<String>,
    pub bridge_tx_digest: Option<String>,
    pub evm_tx_hash: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

pub async fn create_bridge_order(
    State(state): State<Arc<DepositApiState>>,
    Json(req): Json<CreateBridgeOrderRequest>,
) -> Result<Json<BridgeOrderResponse>, (StatusCode, Json<ErrorResponse>)> {
    info!(?req.direction, wallet = %req.myso_wallet, "Create bridge order");
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let (auth_type, destination_chain, destination_address, source_address) = match req.direction {
        BridgeOrderDirection::In => (
            AuthType::Ethereum,
            req.destination_chain
                .unwrap_or_else(|| "mysocial".to_string()),
            req.destination_address
                .unwrap_or_else(|| req.myso_wallet.clone()),
            None,
        ),
        BridgeOrderDirection::Out => {
            let dest_chain = req.destination_chain.unwrap_or_else(|| "base".to_string());
            let dest_addr = req.destination_address.ok_or_else(|| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error: "destinationAddress is required for out orders".to_string(),
                    }),
                )
            })?;
            (
                AuthType::MySocial,
                dest_chain,
                dest_addr,
                Some(req.myso_wallet.clone()),
            )
        }
    };

    let generated = generate_deposit_address(
        State(state.clone()),
        Json(GenerateDepositRequest {
            auth_type,
            source_address,
            signature: None,
            message: MessagePayload {
                action: "generate".to_string(),
                destination_chain,
                destination_address,
                timestamp,
            },
            callback_url: req.callback_url,
            callback_api_key: req.callback_api_key,
        }),
    )
    .await?;

    let order_id = generated.order_id.clone().ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Bridge order id missing after generate".to_string(),
            }),
        )
    })?;

    if let Some(amount) = req.amount.as_ref() {
        if let Some(mut order) = state
            .storage
            .get_bridge_order(&order_id)
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("{:?}", e),
                    }),
                )
            })?
        {
            order.amount = Some(amount.clone());
            state.storage.upsert_bridge_order(order).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("{:?}", e),
                    }),
                )
            })?;
        }
    }

    let order = state.storage.get_bridge_order(&order_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("{:?}", e),
            }),
        )
    })?;
    let Some(order) = order else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Bridge order missing after generate".to_string(),
            }),
        ));
    };
    Ok(Json(order_response(order)))
}

pub async fn get_bridge_order(
    State(state): State<Arc<DepositApiState>>,
    Path(order_id): Path<String>,
) -> Result<Json<BridgeOrderResponse>, (StatusCode, Json<ErrorResponse>)> {
    let order = state.storage.get_bridge_order(&order_id).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("{:?}", e),
            }),
        )
    })?;
    let Some(order) = order else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Order not found".to_string(),
            }),
        ));
    };
    Ok(Json(order_response(order)))
}

fn order_response(order: crate::storage::BridgeOrderRecord) -> BridgeOrderResponse {
    BridgeOrderResponse {
        order_id: order.order_id,
        direction: order.direction,
        status: match order.status {
            crate::storage::BridgeOrderStatus::AwaitingDeposit => "awaiting_deposit".to_string(),
            crate::storage::BridgeOrderStatus::DepositReceived => "deposit_received".to_string(),
            crate::storage::BridgeOrderStatus::Bridging => "bridging".to_string(),
            crate::storage::BridgeOrderStatus::Completed => "completed".to_string(),
            crate::storage::BridgeOrderStatus::Failed => "failed".to_string(),
        },
        myso_wallet: order.myso_wallet,
        deposit_address: order.deposit_address,
        destination_chain: order.destination_chain,
        destination_address: order.destination_address,
        amount: order.amount,
        deposit_tx_digest: order.deposit_tx_digest,
        bridge_tx_digest: order.bridge_tx_digest,
        evm_tx_hash: order.evm_tx_hash,
        created_at: order.created_at,
        updated_at: order.updated_at,
    }
}
