# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# check that myso move new correctly updates existing .gitignore
mkdir example
echo "existing_ignore" > example/.gitignore
myso move --client.config $CONFIG new example
cat example/.gitignore
