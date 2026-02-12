// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::displays::Pretty;
use std::fmt::{Display, Formatter};
use myso_json_rpc_types::MySoExecutionStatus::{self, Failure, Success};
use myso_types::execution_status::ExecutionStatus;

impl Display for Pretty<'_, MySoExecutionStatus> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Pretty(status) = self;

        let output = match status {
            Success => "success".to_string(),
            Failure { error } => format!("failed due to {error}"),
        };

        write!(f, "{}", output)
    }
}

impl Display for Pretty<'_, ExecutionStatus> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Pretty(status) = self;

        let output = match status {
            ExecutionStatus::Success => "success".to_string(),
            ExecutionStatus::Failure { error, command } => {
                if let Some(command) = command {
                    format!("failed due to {error:?} in command {command}")
                } else {
                    format!("failed due to {error:?}")
                }
            }
        };

        write!(f, "{}", output)
    }
}
