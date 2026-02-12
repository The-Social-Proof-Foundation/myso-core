// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use myso_open_rpc_macros::open_rpc;

#[open_rpc(namespace = "mysox", tag = "DeepBook Read API")]
#[rpc(server, client, namespace = "mysox")]
pub trait DeepBookApi {
    #[method(name = "ping")]
    async fn ping(&self) -> RpcResult<String>;
}
