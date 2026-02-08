# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

chain_id=$(myso client --client.config $CONFIG chain-identifier)
echo "[environments]" >> a/Move.toml
echo "localnet = \"$chain_id\"" >> a/Move.toml
echo "[environments]" >> b/Move.toml
echo "localnet = \"$chain_id\"" >> b/Move.toml

myso client --client.config $CONFIG publish "b" -e localnet > output.log 2>&1 || cat output.log
myso client --client.config $CONFIG verify-source "b" -e localnet


myso client --client.config $CONFIG publish "a" -e localnet > output.log 2>&1 || cat output.log
myso client --client.config $CONFIG verify-source "a" -e localnet
myso client --client.config $CONFIG verify-source "a" -e localnet --verify-deps
