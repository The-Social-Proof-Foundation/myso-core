# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# tests that building a package that implicitly depends on `myso` can build
myso move --client.config $CONFIG build -p example 2> /dev/null
