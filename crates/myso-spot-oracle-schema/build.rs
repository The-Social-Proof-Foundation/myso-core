// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Ensure `sqlx::migrate!` re-embeds when any migration file changes.

fn main() {
    println!("cargo:rerun-if-changed=./migrations");
}
