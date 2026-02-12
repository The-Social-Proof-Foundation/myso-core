// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use myso_enum_compat_util::*;

use crate::{MySoMoveStruct, MySoMoveValue};

#[test]
fn enforce_order_test() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "myso_move_struct.yaml"]);
    check_enum_compat_order::<MySoMoveStruct>(path);

    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "myso_move_value.yaml"]);
    check_enum_compat_order::<MySoMoveValue>(path);
}
