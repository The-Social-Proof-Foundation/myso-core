# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# tests that myso move new followed by myso move disassemble succeeds


myso move --client.config $CONFIG new example
cat > example/sources/example.move <<EOF
module example::example;

public fun foo(_ctx: &mut TxContext) {}
EOF
cd example

echo "=== Build ===" >&2
myso move --client.config $CONFIG build

echo "=== Disassemble ===" >&2
myso move --client.config $CONFIG disassemble build/example/bytecode_modules/example.mv
