// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Local messaging relayer for `myso start --with-messaging`.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, bail};
use colored::Colorize;
use tokio::process::Child;

use crate::local_sidecar_util::{
    http_base, resolve_repo_binary, resolve_sibling_repo, spawn_with_env, wait_http_ok,
};

pub const DEFAULT_MESSAGING_RELAYER_PORT: u16 = 3003;

pub struct MessagingLocalInfo {
    pub listen: SocketAddr,
    pub base_url: String,
    pub repo: PathBuf,
    pub myso_rpc: String,
    pub social_server_url: Option<String>,
}

pub fn resolve_messaging_repo(cli_override: Option<PathBuf>) -> PathBuf {
    resolve_sibling_repo(
        cli_override,
        "MYSO_MESSAGING_REPO",
        "myso-messaging-stack",
        "relayer/Cargo.toml",
    )
}

pub async fn spawn_messaging_relayer(
    listen: SocketAddr,
    cli_repo: Option<PathBuf>,
    fullnode_rpc_url: &str,
    social_server_url: Option<&str>,
) -> anyhow::Result<(MessagingLocalInfo, Child)> {
    let stack_root = resolve_messaging_repo(cli_repo);
    let relayer_dir = stack_root.join("relayer");
    if !relayer_dir.join("Cargo.toml").is_file() {
        bail!(
            "messaging relayer not found at {:?}.\n\
             Pass `--messaging-repo` or set MYSO_MESSAGING_REPO to myso-messaging-stack.",
            relayer_dir
        );
    }

    // Prefer binary built inside the messaging stack workspace.
    let bin = resolve_repo_binary(&stack_root, "messaging-relayer")
        .or_else(|_| resolve_repo_binary(&relayer_dir, "messaging-relayer"))
        .with_context(|| {
            format!(
                "Build the relayer first:\n  cd {:?} && cargo build -p messaging-relayer",
                relayer_dir
            )
        })?;

    let mut envs = vec![
        ("PORT", listen.port().to_string()),
        ("MYSO_RPC_URL", fullnode_rpc_url.to_string()),
        ("STORAGE_TYPE", "memory".to_string()),
        ("MEMBERSHIP_STORE_TYPE", "memory".to_string()),
        ("RUST_LOG", "messaging_relayer=info".to_string()),
    ];
    if let Some(social) = social_server_url {
        envs.push(("SOCIAL_SERVER_URL", social.to_string()));
    }

    let child = spawn_with_env(&bin, &envs, Some(&relayer_dir))?;
    let base_url = http_base(listen);
    wait_http_ok(
        &format!("{base_url}/health_check"),
        Duration::from_secs(120),
        "messaging relayer",
    )
    .await
    .context("waiting for messaging relayer /health_check")?;

    let info = MessagingLocalInfo {
        listen,
        base_url,
        repo: stack_root,
        myso_rpc: fullnode_rpc_url.to_string(),
        social_server_url: social_server_url.map(|s| s.to_string()),
    };
    Ok((info, child))
}

pub fn log_messaging_once(info: &MessagingLocalInfo) {
    println!(
        "{}",
        format!(
            r"Messaging relayer (local):
  listen:          {} ({})
  myso_rpc:        {}
  social_server:   {}
  storage:         memory
  repo:            {:?}
",
            info.base_url,
            info.listen,
            info.myso_rpc,
            info.social_server_url
                .as_deref()
                .unwrap_or("(not set)"),
            info.repo
        )
        .green()
    );
}
