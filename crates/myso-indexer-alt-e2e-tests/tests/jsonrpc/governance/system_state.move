// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//# init --protocol-version 108 --simulator --accounts A

//# run-jsonrpc
{
  "method": "mysox_getLatestMySoSystemState",
  "params": []
}

//# programmable --sender A --inputs 1000000000 object(0x5) @validator_0
//> 0: SplitCoins(Gas, [Input(0)]);
//> 1: myso_system::myso_system::request_add_stake(Input(1), Result(0), Input(2))

//# create-checkpoint

//# run-jsonrpc
{
  "method": "mysox_getLatestMySoSystemState",
  "params": []
}

//# advance-clock --duration-ns 1000000

//# advance-epoch

//# run-jsonrpc
{
  "method": "mysox_getLatestMySoSystemState",
  "params": []
}

//# programmable --sender A --inputs object(0x5) object(2,1)
//> 0: myso_system::myso_system::request_withdraw_stake(Input(0), Input(1))

//# create-checkpoint

//# run-jsonrpc
{
  "method": "mysox_getLatestMySoSystemState",
  "params": []
}

//# advance-epoch

//# run-jsonrpc
{
  "method": "mysox_getLatestMySoSystemState",
  "params": []
}
