# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

myso client --client.config config.yaml switch --env base

myso client --client.config config.yaml envs
myso client --client.config config.yaml --client.env one envs
myso client --client.config config.yaml --client.env two envs

myso client --client.config config.yaml active-env
myso client --client.config config.yaml --client.env one active-env
myso client --client.config config.yaml --client.env two active-env

# Unknown name -- Should give you None and nothing active
myso client --client.config config.yaml --client.env not_an_env envs
myso client --client.config config.yaml --client.env not_an_env active-env
