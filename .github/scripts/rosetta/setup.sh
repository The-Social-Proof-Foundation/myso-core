#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

echo "Install binaries"
cargo install --locked --bin myso --path crates/myso
cargo install --locked --bin myso-rosetta --path crates/myso-rosetta

echo "run MySo genesis"
myso genesis

echo "generate rosetta configuration"
myso-rosetta generate-rosetta-cli-config --online-url http://127.0.0.1:9002 --offline-url http://127.0.0.1:9003

echo "install rosetta-cli"
curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s