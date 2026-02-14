// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
pub enum SocialError {
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    #[error("Database error: {0}")]
    Database(String),

    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl SocialError {
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound {
            resource: resource.into(),
        }
    }

    pub fn database(msg: impl Into<String>) -> Self {
        Self::Database(msg.into())
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}

impl IntoResponse for SocialError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            SocialError::NotFound { .. } => (StatusCode::NOT_FOUND, self.to_string()),
            SocialError::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            SocialError::Database(_) | SocialError::Internal(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };

        (status, message).into_response()
    }
}

impl From<diesel::result::Error> for SocialError {
    fn from(err: diesel::result::Error) -> Self {
        Self::Database(err.to_string())
    }
}

impl From<anyhow::Error> for SocialError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err.to_string())
    }
}
