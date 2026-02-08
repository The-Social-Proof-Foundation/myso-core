// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use myso_sdk::MySoClientBuilder;

// This example shows the few basic ways to connect to a MySo network.
// There are several in-built methods for connecting to the
// MySo devnet, tesnet, and localnet (running locally),
// as well as a custom way for connecting to custom URLs.
// The example prints out the API versions of the different networks,
// and finally, it prints the list of available RPC methods
// and the list of subscriptions.
// Note that running this code will fail if there is no MySo network
// running locally on the default address: 127.0.0.1:9000

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let myso = MySoClientBuilder::default()
        .build("http://127.0.0.1:9000") // local network address
        .await?;
    println!("MySo local network version: {}", myso.api_version());

    // local MySo network, like the above one but using the dedicated function
    let myso_local = MySoClientBuilder::default().build_localnet().await?;
    println!("MySo local network version: {}", myso_local.api_version());

    // MySo devnet -- https://fullnode.devnet.myso.io:443
    let myso_devnet = MySoClientBuilder::default().build_devnet().await?;
    println!("MySo devnet version: {}", myso_devnet.api_version());

    // MySo testnet -- https://fullnode.testnet.myso.io:443
    let myso_testnet = MySoClientBuilder::default().build_testnet().await?;
    println!("MySo testnet version: {}", myso_testnet.api_version());

    // MySo mainnet -- https://fullnode.mainnet.myso.io:443
    let myso_mainnet = MySoClientBuilder::default().build_mainnet().await?;
    println!("MySo mainnet version: {}", myso_mainnet.api_version());

    println!("rpc methods: {:?}", myso_testnet.available_rpc_methods());
    println!(
        "available subscriptions: {:?}",
        myso_testnet.available_subscriptions()
    );

    Ok(())
}
