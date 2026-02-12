// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use myso_config::NodeConfig;
use tokio::runtime::Runtime;

pub struct MySoRuntimes {
    // Order in this struct is the order in which runtimes are stopped
    pub myso_node: Runtime,
    pub metrics: Runtime,
}

impl MySoRuntimes {
    pub fn new(_confg: &NodeConfig) -> Self {
        let myso_node = tokio::runtime::Builder::new_multi_thread()
            .thread_name("myso-node-runtime")
            .enable_all()
            .build()
            .unwrap();
        let metrics = tokio::runtime::Builder::new_multi_thread()
            .thread_name("metrics-runtime")
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap();

        Self { myso_node, metrics }
    }
}
