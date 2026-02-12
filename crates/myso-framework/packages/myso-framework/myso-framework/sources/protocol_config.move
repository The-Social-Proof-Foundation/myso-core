// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// This module provides access to protocol configuration feature flags.
/// Feature flags control the availability of various protocol features and
/// are enabled/disabled at specific protocol versions during epoch changes.

module myso::protocol_config;

/// Checks if a specific protocol feature flag is enabled.
///
/// Restricted to internal use within the myso-framework package only.
/// If we need to use it in myso-system, we can add friend declarations.
/// We should never need to expose this to user packages.
///
/// # Arguments
/// * `feature_flag_name` - The name of the feature flag as bytes (e.g., b"enable_vdf")
///   - It is expected to be a valid UTF-8 string
///   - The flag should exist in the protocol config
///
/// # Returns
/// * `true` if the feature is enabled in the current protocol version
/// * `false` if the feature is disabled
///
/// # Example (for framework use only)
/// ```move
/// use myso::protocol_config;
///
/// if (protocol_config::is_feature_enabled(b"enable_accumulators")) {
///     // Accumulators are available
/// };
/// ```
public(package) native fun is_feature_enabled(feature_flag_name: vector<u8>): bool;
