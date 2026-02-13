// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use myso_config::{MYSO_CLIENT_CONFIG, myso_config_dir};
use myso_faucet::FaucetError;
use myso_sdk::wallet_context::WalletContext;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut wallet = create_wallet_context(60)?;
    let active_address = wallet
        .active_address()
        .map_err(|err| FaucetError::Wallet(err.to_string()))?;
    println!("SimpleFaucet::new with active address: {active_address}");
    Ok(())
}

pub fn create_wallet_context(_timeout_secs: u64) -> Result<WalletContext, anyhow::Error> {
    let wallet_conf = myso_config_dir()?.join(MYSO_CLIENT_CONFIG);
    WalletContext::new(&wallet_conf)
}
