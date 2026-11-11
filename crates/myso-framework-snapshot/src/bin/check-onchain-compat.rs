// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Checks that the framework compiled into this binary can upgrade the system packages that are
//! live on a running network. Validators silently vote against a protocol upgrade when this check
//! fails, so run it before rolling out a release.
//!
//! Usage: cargo run -p myso-framework-snapshot --bin check-onchain-compat -- [GRAPHQL_URL]

use anyhow::{Context, Result, anyhow, bail};
use base64::Engine;
use myso_framework::{BuiltInFramework, SystemPackage, compare_system_package};
use myso_protocol_config::{Chain, ProtocolConfig, ProtocolVersion};
use myso_types::base_types::ObjectID;
use serde_json::{Value, json};
use std::collections::BTreeMap;

const DEFAULT_GRAPHQL_URL: &str = "https://graphql.testnet.mysocial.network/graphql";
const PAGE_SIZE: u32 = 20;

async fn graphql(client: &reqwest::Client, url: &str, query: String) -> Result<Value> {
    let resp: Value = client
        .post(url)
        .json(&json!({ "query": query }))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    if let Some(errors) = resp.get("errors") {
        bail!("GraphQL errors: {errors}");
    }
    Ok(resp["data"].clone())
}

/// Returns `None` when the package does not exist on-chain yet (a brand new system package).
async fn fetch_package_modules(
    client: &reqwest::Client,
    url: &str,
    id: &ObjectID,
) -> Result<Option<Vec<Vec<u8>>>> {
    let mut modules = Vec::new();
    let mut after: Option<String> = None;
    loop {
        let cursor = after
            .as_ref()
            .map(|c| format!(", after: \"{c}\""))
            .unwrap_or_default();
        let data = graphql(
            client,
            url,
            format!(
                "{{ package(address: \"{id}\") {{ modules(first: {PAGE_SIZE}{cursor}) {{ \
                 pageInfo {{ hasNextPage endCursor }} nodes {{ bytes }} }} }} }}"
            ),
        )
        .await?;
        if data["package"].is_null() {
            return Ok(None);
        }
        let page = &data["package"]["modules"];
        for node in page["nodes"].as_array().context("missing module nodes")? {
            let b64 = node["bytes"].as_str().context("missing module bytes")?;
            modules.push(base64::engine::general_purpose::STANDARD.decode(b64)?);
        }
        if !page["pageInfo"]["hasNextPage"].as_bool().unwrap_or(false) {
            return Ok(Some(modules));
        }
        after = Some(
            page["pageInfo"]["endCursor"]
                .as_str()
                .context("missing endCursor")?
                .to_string(),
        );
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_GRAPHQL_URL.to_string());
    let client = reqwest::Client::new();

    let data = graphql(
        &client,
        &url,
        "{ chainIdentifier epoch { epochId protocolConfigs { protocolVersion } } }".into(),
    )
    .await?;
    let protocol_version = data["epoch"]["protocolConfigs"]["protocolVersion"]
        .as_u64()
        .context("missing protocolVersion")?;
    println!(
        "Network {url}: chain {} epoch {} protocol {protocol_version}; binary supports up to {}",
        data["chainIdentifier"],
        data["epoch"]["epochId"],
        ProtocolVersion::MAX.as_u64()
    );

    let binary_config =
        ProtocolConfig::get_for_version(ProtocolVersion::new(protocol_version), Chain::Unknown)
            .binary_config(None);

    let mut failures = Vec::new();
    for package in BuiltInFramework::iter_system_packages() {
        let Some(bytes) = fetch_package_modules(&client, &url, &package.id).await? else {
            println!("NEW         {}", package.id);
            continue;
        };
        let on_chain = SystemPackage {
            id: package.id,
            bytes,
            dependencies: package.dependencies.clone(),
        };
        let store: BTreeMap<_, _> = [(package.id, on_chain.genesis_object())].into();
        // compare_system_package logs the failing module and reason at ERROR level.
        let compatible = compare_system_package(
            &store,
            &package.id,
            &package.modules(),
            package.dependencies.to_vec(),
            &binary_config,
        )
        .await
        .is_some();
        // GraphQL returns modules sorted by name, so compare order-independently.
        let mut on_chain_sorted = on_chain.bytes.clone();
        let mut local_sorted = package.bytes.clone();
        on_chain_sorted.sort();
        local_sorted.sort();
        let unchanged = on_chain_sorted == local_sorted;
        let status = match (compatible, unchanged) {
            (true, true) => "UNCHANGED",
            (true, false) => "UPGRADABLE",
            (false, _) => "INCOMPATIBLE",
        };
        println!("{status:<11} {}", package.id);
        if !compatible {
            failures.push(package.id);
        }
    }

    if failures.is_empty() {
        println!("OK: validators running this binary will vote for the framework upgrade.");
        Ok(())
    } else {
        Err(anyhow!(
            "{} incompatible package(s): {failures:?}. Validators will vote against the upgrade; \
             restore removed public functions/structs and original struct layouts.",
            failures.len()
        ))
    }
}
