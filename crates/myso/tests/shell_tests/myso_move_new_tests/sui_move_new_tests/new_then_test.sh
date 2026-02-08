# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# check that myso move new followed by myso move test succeeds
myso move --client.config $CONFIG new example
cd example && myso move --client.config $CONFIG test
