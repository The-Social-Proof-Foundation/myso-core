// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Local MyData key-server bootstrap for `myso start --with-mydata`.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::process::Stdio;

use anyhow::{Context, anyhow, bail};
use colored::Colorize;
use fastcrypto::encoding::Encoding as _;
use fastcrypto::encoding::Hex;
use fastcrypto::traits::KeyPair;
use myso_config::{Config, MYSO_CLIENT_CONFIG, MYSO_KEYSTORE_FILENAME};
use myso_json::MySoJsonValue;
use myso_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use myso_rpc_api::client::ExecutedTransaction;
use myso_sdk::myso_client_config::{MySoClientConfig, MySoEnv};
use myso_sdk::wallet_context::WalletContext;
use myso_swarm::memory::Swarm;
use myso_types::MYDATA_PACKAGE_ID;
use myso_types::base_types::{MySoAddress, ObjectID};
use myso_types::crypto::{AccountKeyPair, MySoKeyPair};
use myso_types::effects::TransactionEffectsAPI;
use myso_types::object::Object;
use myso_types::transaction::TEST_ONLY_GAS_UNIT_FOR_OBJECT_BASICS;
use serde_json::json;
use tokio::process::{Child, Command};

use crate::client_commands::{
    GasDataArgs, MySoClientCommandResult, MySoClientCommands, PaymentArgs, TxProcessingArgs,
};

/// Visible secrets and paths after local MyData bootstrap (for console + files).
pub struct MydataLocalSecrets {
    pub master_key_hex: String,
    pub public_key_hex: String,
    pub mydata_package_id: ObjectID,
    pub key_server_object_id: ObjectID,
    pub key_server_listen: SocketAddr,
    pub key_server_public_url: String,
    pub config_path: PathBuf,
    pub metrics_port: u16,
}

/// Resolve myso-mydata repository root: CLI flag, `MYSO_MYDATA_REPO`, then common sibling paths.
pub fn resolve_mydata_repo(cli_override: Option<PathBuf>) -> PathBuf {
    if let Some(p) = cli_override {
        return p;
    }
    if let Ok(p) = std::env::var("MYSO_MYDATA_REPO") {
        return PathBuf::from(p);
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for base in [
        cwd,
        manifest_dir.clone(),
        manifest_dir.parent().unwrap_or(&manifest_dir).to_path_buf(),
    ] {
        let candidate = base.join("myso-mydata");
        if candidate.join("Cargo.toml").is_file() {
            return candidate;
        }
        if base.ends_with("myso-core") {
            let sibling = base.parent().map(|p| p.join("myso-mydata"));
            if let Some(ref c) = sibling {
                if c.join("Cargo.toml").is_file() {
                    return c.clone();
                }
            }
        }
    }
    PathBuf::from("../myso-mydata")
}

fn resolve_binary(repo_root: &Path, name: &str) -> anyhow::Result<PathBuf> {
    for profile in ["release", "debug"] {
        let p = repo_root.join("target").join(profile).join(name);
        if p.is_file() {
            return Ok(p);
        }
    }
    bail!(
        "Could not find `{name}` under {:?}/target/release or target/debug.\n\
         Build it from the myso-mydata repo, e.g.:\n\
           cd {:?} && cargo build -p key-server -p mydata-cli",
        repo_root,
        repo_root
    );
}

fn parse_prefixed_hex_line<'a>(line: &'a str, prefix: &str) -> anyhow::Result<&'a str> {
    let rest = line
        .strip_prefix(prefix)
        .ok_or_else(|| anyhow!("expected line starting with {:?}, got {:?}", prefix, line))?;
    Ok(rest.trim())
}

/// Parse stdout of `mydata-cli genkey` (Master key / Public key lines).
pub fn parse_genkey_output(stdout: &str) -> anyhow::Result<(String, String)> {
    let mut master = None;
    let mut public = None;
    for line in stdout.lines() {
        if let Ok(v) = parse_prefixed_hex_line(line, "Master key: ") {
            master = Some(v.to_string());
        }
        if let Ok(v) = parse_prefixed_hex_line(line, "Public key: ") {
            public = Some(v.to_string());
        }
    }
    let master = master.ok_or_else(|| anyhow!("missing \"Master key:\" in genkey output"))?;
    let public = public.ok_or_else(|| anyhow!("missing \"Public key:\" in genkey output"))?;
    Ok((master, public))
}

pub fn public_key_bytes_for_move(public_key_line: &str) -> anyhow::Result<Vec<u8>> {
    let s = public_key_line.trim().trim_start_matches("0x");
    Hex::decode(s).map_err(|e| anyhow!("invalid public key hex: {e}"))
}

/// Write [`ensure_regenesis_client_config`] when `myso start --force-regenesis` has no `client.yaml` yet.
pub async fn ensure_regenesis_client_config(
    swarm: &Swarm,
    config_dir: &Path,
    fullnode_rpc_url: &str,
) -> anyhow::Result<()> {
    let client_path = config_dir.join(MYSO_CLIENT_CONFIG);
    if client_path.exists() {
        return Ok(());
    }
    let kp: AccountKeyPair = swarm
        .config()
        .account_keys
        .first()
        .ok_or_else(|| anyhow!("swarm has no genesis account keys"))?
        .copy();
    let keystore_path = config_dir.join(MYSO_KEYSTORE_FILENAME);
    let mut keystore = Keystore::from(FileBasedKeystore::load_or_create(&keystore_path)?);
    let address: MySoAddress = kp.public().into();
    keystore.import(None, MySoKeyPair::Ed25519(kp)).await?;
    MySoClientConfig {
        keystore,
        external_keys: None,
        envs: vec![MySoEnv {
            alias: "localnet".to_string(),
            rpc: fullnode_rpc_url.to_string(),
            ws: None,
            basic_auth: None,
            chain_id: None,
        }],
        active_address: Some(address),
        active_env: Some("localnet".to_string()),
    }
    .persisted(&client_path)
    .save()?;
    Ok(())
}

async fn run_mydata_cli_genkey(mydata_cli: &Path) -> anyhow::Result<(String, String)> {
    let output = Command::new(mydata_cli)
        .arg("genkey")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .with_context(|| format!("failed to run {:?}", mydata_cli))?;
    if !output.status.success() {
        bail!(
            "mydata-cli genkey failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_genkey_output(stdout.trim())
}

async fn find_key_server_object_id(
    client: &mut myso_rpc_api::Client,
    executed: &ExecutedTransaction,
) -> anyhow::Result<ObjectID> {
    for ((object_id, _seq, _digest), _owner) in executed.effects.created() {
        let object = client
            .get_object((*object_id).into())
            .await
            .with_context(|| format!("get_object {}", object_id))?;
        if is_key_server_object(&object)? {
            return Ok((*object_id).into());
        }
    }
    bail!("did not find a created mydata::key_server::KeyServer object in transaction effects");
}

fn is_key_server_object(object: &Object) -> anyhow::Result<bool> {
    let Some(tag) = object.struct_tag() else {
        return Ok(false);
    };
    let s = tag.to_canonical_string(true);
    Ok(s.contains("::key_server::KeyServer"))
}

fn write_key_server_config_yaml(
    path: &Path,
    mydata_package_id: ObjectID,
    key_server_object_id: ObjectID,
    metrics_port: u16,
) -> anyhow::Result<()> {
    // key-server deserializes enums via serde_yaml tags (`!Variant`), not nested maps.
    let yaml = format!(
        r#"network: !Devnet
  mydata_package: "{pkg}"
server_mode: !Open
  key_server_object_id: "{ks}"
metrics_host_port: {metrics}
"#,
        pkg = mydata_package_id,
        ks = key_server_object_id,
        metrics = metrics_port,
    );
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).with_context(|| format!("create_dir_all {:?}", parent))?;
    }
    std::fs::write(path, yaml).with_context(|| format!("write {:?}", path))?;
    Ok(())
}

fn write_secrets_file(path: &Path, secrets: &MydataLocalSecrets) -> anyhow::Result<()> {
    let text = format!(
        r#"# Local MyData dev secrets — do not commit. Generated by `myso start --with-mydata`.
MASTER_KEY={mk}
PUBLIC_KEY={pk}
MYDATA_PACKAGE_ID={pkg}
KEY_SERVER_OBJECT_ID={ks}
KEY_SERVER_URL={url:?}
KEY_SERVER_LISTEN={listen}
CONFIG_PATH={cfg:?}
NODE_URL=<set in environment when running key-server; myso start sets NODE_URL for the child process>
METRICS_PORT={met}
"#,
        mk = secrets.master_key_hex,
        pk = secrets.public_key_hex,
        pkg = secrets.mydata_package_id,
        ks = secrets.key_server_object_id,
        url = secrets.key_server_public_url,
        listen = secrets.key_server_listen,
        cfg = secrets.config_path,
        met = secrets.metrics_port,
    );
    std::fs::write(path, text).with_context(|| format!("write {:?}", path))?;
    Ok(())
}

fn json_arg<T: serde::Serialize>(v: T) -> anyhow::Result<MySoJsonValue> {
    Ok(MySoJsonValue::new(json!(v))?)
}

/// Register a `KeyServer` on the genesis MyData system package ([`MYDATA_PACKAGE_ID`]), write config,
/// spawn `key-server`.
pub async fn bootstrap_and_spawn_key_server(
    repo_root: PathBuf,
    config_dir: &Path,
    fullnode_rpc_url: &str,
    listen: SocketAddr,
    metrics_port: u16,
) -> anyhow::Result<(MydataLocalSecrets, Child)> {
    let mydata_dir = config_dir.join("mydata");
    std::fs::create_dir_all(&mydata_dir)
        .with_context(|| format!("create_dir_all {:?}", mydata_dir))?;

    let client_path = config_dir.join(MYSO_CLIENT_CONFIG);
    let mut context = WalletContext::new(&client_path)
        .with_context(|| format!("WalletContext::new({:?})", client_path))?;
    context.cache_chain_id().await?;

    let key_server_bin = resolve_binary(&repo_root, "key-server")?;
    let mydata_cli = resolve_binary(&repo_root, "mydata-cli")?;

    let (master_hex, public_hex) = run_mydata_cli_genkey(&mydata_cli).await?;
    let pk_move = public_key_bytes_for_move(&public_hex)?;

    let rgp = context.get_reference_gas_price().await?;
    let (_sender, gas_ref) = context.get_one_gas_object().await?.ok_or_else(|| {
        anyhow!("no gas coins for active address; use `--with-faucet` or fund the account")
    })?;
    let gas_id = gas_ref.0;

    let mydata_package_id = ObjectID::from(MYDATA_PACKAGE_ID);

    let mut client = context.grpc_client()?;
    let port_u16 = listen.port();
    let public_url = key_server_listen_url_for_clients(listen);

    let call_result = MySoClientCommands::Call {
        package: mydata_package_id,
        module: "key_server".to_string(),
        function: "create_and_transfer_v1".to_string(),
        type_args: vec![],
        args: vec![
            json_arg("local-mydata")?,
            json_arg(&public_url)?,
            json_arg(0u8)?,
            json_arg(pk_move)?,
        ],
        payment: PaymentArgs { gas: vec![gas_id] },
        gas_data: GasDataArgs {
            gas_budget: Some(rgp * TEST_ONLY_GAS_UNIT_FOR_OBJECT_BASICS),
            ..Default::default()
        },
        processing: TxProcessingArgs::default(),
    }
    .execute(&mut context)
    .await?;

    let MySoClientCommandResult::TransactionBlock(call_executed) = call_result else {
        bail!("unexpected call result (expected TransactionBlock)");
    };
    if !call_executed.effects.status().is_ok() {
        bail!(
            "create_and_transfer_v1 failed: {:?}",
            call_executed.effects.status()
        );
    }
    let key_server_object_id = find_key_server_object_id(&mut client, &call_executed).await?;

    let config_path = mydata_dir.join("key-server-config.yaml");
    write_key_server_config_yaml(
        &config_path,
        mydata_package_id,
        key_server_object_id,
        metrics_port,
    )?;

    let secrets = MydataLocalSecrets {
        master_key_hex: master_hex.clone(),
        public_key_hex: public_hex,
        mydata_package_id,
        key_server_object_id,
        key_server_listen: listen,
        key_server_public_url: public_url,
        config_path: config_path.clone(),
        metrics_port,
    };
    write_secrets_file(&mydata_dir.join("local-mydata-secrets.env"), &secrets)?;

    eprintln!(
        "{}",
        "[warning] Wrote MyData dev secrets under network.config/mydata/ — do not commit; local development only."
            .yellow()
            .bold()
    );

    let mut cmd = Command::new(&key_server_bin);
    cmd.env("CONFIG_PATH", &config_path)
        .env("NODE_URL", fullnode_rpc_url)
        .env("MASTER_KEY", &master_hex)
        .env("PORT", port_u16.to_string())
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let child = cmd
        .spawn()
        .with_context(|| format!("spawn {:?}", key_server_bin))?;

    Ok((secrets, child))
}

pub fn log_mydata_secrets_once(secrets: &MydataLocalSecrets) {
    println!(
        r"MyData key server (local):
  MASTER_KEY={}
  PUBLIC_KEY={}
  MYDATA_PACKAGE_ID={}
  KEY_SERVER_OBJECT_ID={}
  KEY_SERVER_URL={:?}
  LISTEN_ADDRESS={}
  CONFIG_PATH={:?}
  METRICS_PORT={}
",
        secrets.master_key_hex,
        secrets.public_key_hex,
        secrets.mydata_package_id,
        secrets.key_server_object_id,
        secrets.key_server_public_url,
        secrets.key_server_listen,
        secrets.config_path,
        secrets.metrics_port,
    );
}

/// Best-effort: map unspecified bind address to localhost for client-facing URL (already applied in `public_url`).
pub fn key_server_listen_url_for_clients(listen: SocketAddr) -> String {
    let ip = match listen.ip() {
        IpAddr::V4(v4) if v4.is_unspecified() => IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V6(v6) if v6.is_unspecified() => IpAddr::V4(Ipv4Addr::LOCALHOST),
        ip => ip,
    };
    format!("http://{ip}:{}", listen.port())
}

#[cfg(test)]
mod tests {
    use super::parse_genkey_output;

    #[test]
    fn parse_genkey_output_ok() {
        let sample = "Master key: 0x01ab\nPublic key: 0x02cd\n";
        let (m, p) = parse_genkey_output(sample).unwrap();
        assert_eq!(m, "0x01ab");
        assert_eq!(p, "0x02cd");
    }
}
