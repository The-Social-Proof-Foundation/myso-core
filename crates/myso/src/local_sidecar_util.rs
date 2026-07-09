// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Shared helpers for `myso start --with-*` external sidecars.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use anyhow::{Context, anyhow, bail};
use tokio::process::{Child, Command};

/// myso-core repository root (`crates/myso` → `../..`).
pub fn myso_core_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .and_then(|p| p.parent())
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."))
        })
}

/// Resolve a sibling repo: CLI override → env → cwd/manifest sibling paths.
pub fn resolve_sibling_repo(
    cli_override: Option<PathBuf>,
    env_var: &str,
    dir_name: &str,
    marker_rel: &str,
) -> PathBuf {
    if let Some(p) = cli_override {
        return p;
    }
    if let Ok(p) = std::env::var(env_var) {
        return PathBuf::from(p);
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let core_root = myso_core_root();
    for base in [
        cwd,
        manifest_dir.clone(),
        manifest_dir.parent().unwrap_or(&manifest_dir).to_path_buf(),
        core_root.clone(),
        core_root
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or(core_root),
    ] {
        let candidate = base.join(dir_name);
        if candidate.join(marker_rel).exists() {
            return candidate;
        }
        if base.ends_with("myso-core")
            && let Some(parent) = base.parent()
        {
            let sibling = parent.join(dir_name);
            if sibling.join(marker_rel).exists() {
                return sibling;
            }
        }
    }
    PathBuf::from(format!("../{dir_name}"))
}

/// Find a workspace binary under myso-core `target/{release,debug}/`.
pub fn resolve_workspace_binary(name: &str) -> anyhow::Result<PathBuf> {
    let root = myso_core_root();
    for profile in ["release", "debug"] {
        let p = root.join("target").join(profile).join(name);
        if p.is_file() {
            return Ok(p);
        }
    }
    bail!(
        "Could not find `{name}` under {:?}/target/release or target/debug.\n\
         Build it first, e.g.:\n\
           cargo build -p {name}",
        root
    );
}

/// Find a binary under an external repo's `target/{release,debug}/`.
pub fn resolve_repo_binary(repo_root: &Path, name: &str) -> anyhow::Result<PathBuf> {
    for profile in ["release", "debug"] {
        let p = repo_root.join("target").join(profile).join(name);
        if p.is_file() {
            return Ok(p);
        }
    }
    bail!(
        "Could not find `{name}` under {:?}/target/release or target/debug.\n\
         Build it from that repo first.",
        repo_root
    );
}

pub async fn docker_compose_up(
    compose_file: &Path,
    services: &[&str],
    project_directory: Option<&Path>,
    profiles: &[&str],
) -> anyhow::Result<()> {
    let mut cmd = Command::new("docker");
    cmd.arg("compose");
    if let Some(dir) = project_directory {
        cmd.arg("--project-directory").arg(dir);
    }
    cmd.arg("-f").arg(compose_file);
    for profile in profiles {
        cmd.arg("--profile").arg(profile);
    }
    cmd.arg("up").arg("-d");
    for svc in services {
        cmd.arg(svc);
    }
    cmd.stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    let status = cmd
        .status()
        .await
        .with_context(|| format!("docker compose up for {:?}", compose_file))?;
    if !status.success() {
        bail!(
            "docker compose up failed (exit {:?}) for {:?}",
            status.code(),
            compose_file
        );
    }
    Ok(())
}

pub async fn docker_compose_down(
    compose_file: &Path,
    project_directory: Option<&Path>,
    profiles: &[&str],
) -> anyhow::Result<()> {
    let mut cmd = Command::new("docker");
    cmd.arg("compose");
    if let Some(dir) = project_directory {
        cmd.arg("--project-directory").arg(dir);
    }
    cmd.arg("-f").arg(compose_file);
    for profile in profiles {
        cmd.arg("--profile").arg(profile);
    }
    // Keep volumes; only stop containers.
    cmd.arg("down");
    cmd.stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    let status = cmd.status().await.context("docker compose down")?;
    if !status.success() {
        bail!("docker compose down failed (exit {:?})", status.code());
    }
    Ok(())
}

pub async fn wait_http_ok(url: &str, timeout: Duration, label: &str) -> anyhow::Result<()> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    let start = std::time::Instant::now();
    let mut last_err = None;
    while start.elapsed() < timeout {
        match client.get(url).send().await {
            Ok(resp) if resp.status().is_success() => return Ok(()),
            Ok(resp) => {
                last_err = Some(anyhow!("{label} returned HTTP {}", resp.status()));
            }
            Err(e) => {
                last_err = Some(anyhow!("{label}: {e}"));
            }
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    Err(last_err.unwrap_or_else(|| anyhow!("{label} did not become healthy at {url}")))
}

pub fn spawn_with_env(
    bin: &Path,
    envs: &[(&str, String)],
    cwd: Option<&Path>,
) -> anyhow::Result<Child> {
    let mut cmd = Command::new(bin);
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    for (k, v) in envs {
        cmd.env(k, v);
    }
    cmd.kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    cmd.spawn().with_context(|| format!("spawn {:?}", bin))
}

pub fn http_base(listen: SocketAddr) -> String {
    let host = match listen.ip() {
        std::net::IpAddr::V4(ip) if ip.is_unspecified() => "127.0.0.1".to_string(),
        std::net::IpAddr::V6(ip) if ip.is_unspecified() => "[::1]".to_string(),
        other => other.to_string(),
    };
    format!("http://{host}:{}", listen.port())
}
