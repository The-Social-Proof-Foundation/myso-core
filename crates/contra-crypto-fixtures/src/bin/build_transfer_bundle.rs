// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use contra_crypto_fixtures::{
    account_id, build_transfer_bundle, build_unwrap_bundle, format_move_hex, parse_struct_tag,
    session_id, elgamal_dst, ddh_dst,
};
use myso_types::base_types::MySoAddress;
use myso_types::base_types::ObjectID;
use serde::Serialize;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(name = "build_transfer_bundle")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Build proof bundle for a single-receiver batched transfer.
    Transfer {
        #[arg(long)]
        sender_account_id: String,
        #[arg(long)]
        coin_type: String,
        #[arg(long)]
        sender_sk: u64,
        #[arg(long)]
        receiver_pk: String,
        #[arg(long, default_value_t = 50)]
        transfer_amount: u16,
        #[arg(long, default_value_t = 100)]
        sender_balance: u16,
        #[arg(long, default_value_t = 32533)]
        transfer_blinding: u64,
        #[arg(long, default_value_t = 10097)]
        new_balance_blinding: u64,
        #[arg(long)]
        output: PathBuf,
    },
    /// Build proof bundle for unwrap.
    Unwrap {
        #[arg(long)]
        account_id: String,
        #[arg(long)]
        coin_type: String,
        #[arg(long)]
        owner_sk: u64,
        #[arg(long, default_value_t = 50)]
        balance: u16,
        #[arg(long, default_value_t = 30)]
        unwrap_amount: u16,
        #[arg(long, default_value_t = 76520)]
        balance_blinding: u64,
        #[arg(long, default_value_t = 76520)]
        new_balance_blinding: u64,
        #[arg(long)]
        output: PathBuf,
    },
    /// Generate a Ristretto keypair for session setup.
    Keygen {
        #[arg(long, default_value_t = 12345)]
        secret: u64,
        #[arg(long)]
        output: PathBuf,
    },
    /// Print session id and DSTs for an account + coin type.
    SessionInfo {
        #[arg(long)]
        account_id: String,
        #[arg(long)]
        coin_type: String,
    },
    /// Derive shared Account object id from registry + owner address.
    AccountId {
        #[arg(long)]
        registry_id: String,
        #[arg(long)]
        owner: String,
    },
}

#[derive(Serialize)]
struct KeygenOutput {
    secret: u64,
    public_key: String,
}

fn parse_pk_hex(s: &str) -> [u8; 32] {
    let stripped = s.strip_prefix("0x").unwrap_or(s);
    let bytes = hex::decode(stripped).expect("valid hex pk");
    bytes.try_into().expect("pk is 32 bytes")
}

fn main() {
    match Cli::parse().command {
        Command::Transfer {
            sender_account_id,
            coin_type,
            sender_sk,
            receiver_pk,
            transfer_amount,
            sender_balance,
            transfer_blinding,
            new_balance_blinding,
            output,
        } => {
            let coin = parse_struct_tag(&coin_type);
            let bundle = build_transfer_bundle(
                ObjectID::from_str(&sender_account_id).expect("account id"),
                &coin,
                sender_sk,
                parse_pk_hex(&receiver_pk),
                transfer_amount,
                sender_balance,
                transfer_blinding,
                new_balance_blinding,
            );
            std::fs::write(&output, serde_json::to_string_pretty(&bundle).unwrap()).unwrap();
        },
        Command::Unwrap {
            account_id,
            coin_type,
            owner_sk,
            balance,
            unwrap_amount,
            balance_blinding,
            new_balance_blinding,
            output,
        } => {
            let coin = parse_struct_tag(&coin_type);
            let bundle = build_unwrap_bundle(
                ObjectID::from_str(&account_id).expect("account id"),
                &coin,
                owner_sk,
                balance,
                unwrap_amount,
                balance_blinding,
                new_balance_blinding,
            );
            std::fs::write(&output, serde_json::to_string_pretty(&bundle).unwrap()).unwrap();
        },
        Command::Keygen { secret, output } => {
            let sk = contra_crypto_fixtures::scalar_from_u64(secret);
            let pk = contra_crypto_fixtures::pk_from_sk(&sk);
            let out = KeygenOutput {
                secret,
                public_key: format!(
                    "0x{}",
                    format_move_hex(pk.compress().as_bytes())
                ),
            };
            std::fs::write(&output, serde_json::to_string_pretty(&out).unwrap()).unwrap();
        },
        Command::SessionInfo { account_id, coin_type } => {
            let coin = parse_struct_tag(&coin_type);
            let sid = session_id(
                ObjectID::from_str(&account_id).expect("account id"),
                &coin,
            );
            let info = serde_json::json!({
                "session_id": format!("0x{}", format_move_hex(&sid)),
                "elgamal_dst": format!("0x{}", format_move_hex(&elgamal_dst(&sid))),
                "ddh_dst": format!("0x{}", format_move_hex(&ddh_dst(&sid))),
            });
            println!("{}", serde_json::to_string_pretty(&info).unwrap());
        },
        Command::AccountId { registry_id, owner } => {
            let id = account_id(
                ObjectID::from_str(&registry_id).expect("registry id"),
                MySoAddress::from_str(&owner).expect("owner address"),
            );
            let info = serde_json::json!({ "account_id": id.to_string() });
            println!("{}", serde_json::to_string_pretty(&info).unwrap());
        },
    }
}
