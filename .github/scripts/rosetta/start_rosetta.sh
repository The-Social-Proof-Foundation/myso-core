#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

echo "Start Rosetta online server"
myso-rosetta start-online-server --data-path ./data &

echo "Start Rosetta offline server"
myso-rosetta start-offline-server &
