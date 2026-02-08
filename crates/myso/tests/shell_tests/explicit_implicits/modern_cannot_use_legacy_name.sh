# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# tests that building a package that has explicit deps on legacy system names errors
myso move --client.config $CONFIG build -p modern_using_legacy_name
