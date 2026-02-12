// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

fn main() {
    cynic_codegen::register_schema("myso")
        .from_sdl_file("../myso-indexer-alt-graphql/schema.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
