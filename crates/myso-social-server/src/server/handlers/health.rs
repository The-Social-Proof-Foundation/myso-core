// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::http::StatusCode;
use axum::Json;

pub async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "healthy",
            "message": "Social API server is running",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
    )
}
