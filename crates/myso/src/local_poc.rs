// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Local Proof of Creativity stack for `myso start --with-poc`.

use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, bail};
use colored::Colorize;

use crate::local_sidecar_util::{
    docker_compose_down, docker_compose_up, myso_core_root, resolve_sibling_repo, wait_http_ok,
};

pub const DEFAULT_POC_API_PORT: u16 = 8000;

/// Stable localnet oracle key (matches `network.config/poc/oracle-localnet.env`).
const LOCALNET_ORACLE_PRIVATE_KEY_HEX: &str =
    "736c869f584b6fdf1d961541e515304cdbeaf8e3d7789ae79fd05e2d9da34578";

pub struct PocLocalInfo {
    pub repo: PathBuf,
    pub compose_file: PathBuf,
    pub api_port: u16,
    pub api_base_url: String,
    pub oracle_address: String,
}

pub fn resolve_poc_repo(cli_override: Option<PathBuf>) -> PathBuf {
    resolve_sibling_repo(
        cli_override,
        "MYSO_POC_REPO",
        "proof-of-creativity",
        "docker-compose.yml",
    )
}

fn compose_file(repo: &Path) -> PathBuf {
    repo.join("docker-compose.yml")
}

/// Write/merge PoC docker `.env` for compose interpolation and chain integration.
fn ensure_poc_env(repo: &Path, api_port: u16) -> anyhow::Result<()> {
    let env_path = repo.join(".env");
    let mut lines: Vec<String> = if env_path.is_file() {
        std::fs::read_to_string(&env_path)?
            .lines()
            .map(|l| l.to_string())
            .collect()
    } else {
        Vec::new()
    };

    let upsert = |lines: &mut Vec<String>, key: &str, value: &str| {
        let prefix = format!("{key}=");
        if let Some(existing) = lines.iter_mut().find(|l| l.starts_with(&prefix)) {
            *existing = format!("{key}={value}");
        } else {
            lines.push(format!("{key}={value}"));
        }
    };

    upsert(&mut lines, "API_PORT", &api_port.to_string());
    upsert(&mut lines, "MYSO_INTEGRATION_ENABLED", "true");
    upsert(&mut lines, "MYSO_REFRESH_SESSION_OBJECTS", "true");
    upsert(&mut lines, "POC_E2E_SUBMIT_OVERRIDE", "1");
    upsert(&mut lines, "POC_IDENTITY_VERIFIER", "mock");
    upsert(&mut lines, "DISCOVERY_ENABLED", "true");
    upsert(&mut lines, "DISCOVERY_EMBED_ENABLED", "true");
    upsert(&mut lines, "POC_USE_MANUAL_CURATED", "1");
    upsert(
        &mut lines,
        "DISCOVERY_SOURCES_CONFIG",
        "config/discovery/sources.localnet.yaml",
    );
    upsert(
        &mut lines,
        "ORACLE_PRIVATE_KEY_LOCALNET",
        LOCALNET_ORACLE_PRIVATE_KEY_HEX,
    );
    upsert(
        &mut lines,
        "POC_ADMIN_PRIVATE_KEY_LOCALNET",
        LOCALNET_ORACLE_PRIVATE_KEY_HEX,
    );

    std::fs::write(&env_path, lines.join("\n") + "\n")
        .with_context(|| format!("write {:?}", env_path))?;
    Ok(())
}

fn default_oracle_address() -> String {
    let template = myso_core_root().join("network.config/poc/oracle-localnet.env");
    if template.is_file() {
        if let Ok(text) = std::fs::read_to_string(&template) {
            for line in text.lines() {
                if let Some(rest) = line.strip_prefix("POC_DEFAULT_ORACLE_ADDRESS=") {
                    let addr = rest.trim();
                    if !addr.is_empty() {
                        return addr.to_string();
                    }
                }
            }
        }
    }
    "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8".to_string()
}

pub async fn spawn_poc_stack(
    cli_repo: Option<PathBuf>,
    api_port: u16,
) -> anyhow::Result<PocLocalInfo> {
    let repo = resolve_poc_repo(cli_repo);
    let compose = compose_file(&repo);
    if !compose.is_file() {
        bail!(
            "PoC docker-compose.yml not found at {:?}.\n\
             Pass `--poc-repo` or set MYSO_POC_REPO to the proof-of-creativity checkout.",
            compose
        );
    }

    ensure_poc_env(&repo, api_port)?;

    docker_compose_up(
        &compose,
        &[
            "postgres",
            "redis",
            "api",
            "grpc-sync",
            "oracle-worker",
            "discovery-worker",
        ],
        Some(&repo),
        &["app"],
    )
    .await
    .context("starting PoC docker compose --profile app")?;

    let api_base_url = format!("http://127.0.0.1:{api_port}");
    wait_http_ok(
        &format!("{api_base_url}/health"),
        Duration::from_secs(600),
        "PoC API",
    )
    .await
    .context("waiting for PoC /health")?;

    wait_http_ok(
        &format!("{api_base_url}/oracle/sync/status?network=localnet"),
        Duration::from_secs(120),
        "PoC oracle sync status",
    )
    .await
    .context("waiting for PoC /oracle/sync/status")?;

    let oracle_address = default_oracle_address();

    Ok(PocLocalInfo {
        repo,
        compose_file: compose,
        api_port,
        api_base_url,
        oracle_address,
    })
}

pub async fn stop_poc_stack(info: &PocLocalInfo) -> anyhow::Result<()> {
    docker_compose_down(&info.compose_file, Some(&info.repo), &["app"]).await
}

pub fn log_poc_once(info: &PocLocalInfo) {
    println!(
        "{}",
        format!(
            r"Proof of Creativity (local docker):
  api:             {} (port {})
  repo:            {:?}
  oracle_address:  {}
  grpc-sync:       running (profile app)
  discovery-worker:   running (profile app)
  validate:        ASSUME_YES=1 ./scripts/poc-e2e-runnable.sh --run-all
  note:            on-chain PoCConfig.oracle_address must match oracle_address (poc-e2e syncs it)
",
            info.api_base_url,
            info.api_port,
            info.repo,
            info.oracle_address,
        )
        .green()
    );
}
