// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use myso_framework::SystemPackage;
use myso_framework_snapshot::load_bytecode_snapshot;
use myso_protocol_config::ProtocolVersion;
use myso_types::base_types::ObjectID;
use std::collections::BTreeSet;

/// Ensures every module-imported package address is listed in genesis `dependencies`.
/// Extra declared deps are allowed (e.g. linkage-table supersession for transitive deps).
fn check_packages(label: &str, packages: impl IntoIterator<Item = SystemPackage>) {
    let mut any = false;
    for pkg in packages {
        let self_addr = pkg.id;
        let mut needed: BTreeSet<ObjectID> = BTreeSet::new();
        for module in pkg.modules() {
            for dep in module.immediate_dependencies() {
                let dep_id = ObjectID::from(*dep.address());
                if dep_id != self_addr {
                    needed.insert(dep_id);
                }
            }
        }
        let declared: BTreeSet<ObjectID> = pkg.dependencies.iter().copied().collect();
        let missing: Vec<_> = needed.difference(&declared).collect();
        if !missing.is_empty() {
            any = true;
            println!("{label} package {self_addr}");
            println!("  MISSING from declared deps: {missing:?}");
        }
    }
    assert!(!any, "package dependency mismatches found");
}

#[test]
fn check_v112_snapshot_package_dependencies() {
    let packages = load_bytecode_snapshot(ProtocolVersion::MAX.as_u64()).expect("snapshot");
    check_packages("Snapshot112", packages);
}
