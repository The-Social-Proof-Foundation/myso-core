# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# test that `myso move new` works as expected with `<NAME>` containing uppercase letter(s)
myso move --client.config $CONFIG new _Example_A
echo ==== files in project ====
ls -A _Example_A
echo ==== files in sources ====
ls -A _Example_A/sources
echo ==== files in tests =====
ls -A _Example_A/tests
