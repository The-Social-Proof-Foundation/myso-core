// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Shared utilities for social event processing across pipelines.

use move_core_types::account_address::AccountAddress;
use myso_types::base_types::ObjectID;
use myso_types::MYSO_SOCIAL_PACKAGE_ID;

/// Returns true if the event belongs to the myso-social package.
pub fn is_social_package_event(package_id: &ObjectID, type_address: &AccountAddress) -> bool {
    use std::ops::Deref;
    *package_id == MYSO_SOCIAL_PACKAGE_ID || *type_address == *MYSO_SOCIAL_PACKAGE_ID.deref()
}
