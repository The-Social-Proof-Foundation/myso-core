// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let packages_compiled_dir = PathBuf::from(&manifest_dir).join("packages_compiled");

    let required_files = [
        "move-stdlib",
        "myso-framework",
        "myso-system",
        "orderbook",
        "bridge",
        "mydata",
        "myso-social",
        "messaging",
        "contra",
    ];

    let mut missing = Vec::new();
    for file_name in &required_files {
        let file_path = packages_compiled_dir.join(file_name);
        if !file_path.exists() {
            missing.push(file_name.to_string());
        } else if std::fs::metadata(&file_path).map(|m| m.len()).unwrap_or(0) <= 1 {
            missing.push(format!("{} (empty or placeholder)", file_name));
        }
    }

    if !missing.is_empty() {
        if env::var_os("MYFRAMEWORK_SKIP_PACKAGES_CHECK").is_some() {
            eprintln!(
                "warning: skipping packages_compiled check (MYFRAMEWORK_SKIP_PACKAGES_CHECK); \
                 missing: {:?}",
                missing
            );
        } else {
            panic!(
                "myso-framework packages_compiled is missing or has empty files: {:?}.\n\n\
                 Restore from git (crates/myso-framework/packages_compiled) or bootstrap with:\n\
                   MYFRAMEWORK_SKIP_PACKAGES_CHECK=1 UPDATE=1 cargo test -p myso-framework \
                   --test build-system-packages build_system_packages",
                missing
            );
        }
    }

    println!("cargo:rerun-if-changed=packages_compiled");
}
