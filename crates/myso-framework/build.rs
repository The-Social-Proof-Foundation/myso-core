// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let packages_compiled_dir = PathBuf::from(&manifest_dir).join("packages_compiled");
    
    // Create the directory if it doesn't exist
    std::fs::create_dir_all(&packages_compiled_dir).unwrap();
    
    // Create placeholder files if they don't exist
    // These will be overwritten by the test when UPDATE=1 is set
    let required_files = [
        "move-stdlib",
        "myso-framework",
        "myso-system",
        "deepbook",
        "bridge",
        "mydata",
        "myso-social",
    ];
    
    for file_name in &required_files {
        let file_path = packages_compiled_dir.join(file_name);
        if !file_path.exists() {
            // Create a minimal valid BCS-encoded empty Vec<Vec<u8>>
            // This represents an empty vector of compiled modules
            let empty_modules: Vec<Vec<u8>> = vec![];
            let bcs_bytes = bcs::to_bytes(&empty_modules).unwrap();
            std::fs::write(&file_path, bcs_bytes).unwrap();
        }
    }
    
    println!("cargo:rerun-if-changed=packages_compiled");
}
