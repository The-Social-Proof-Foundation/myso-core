// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Network node configuration for MySo data stores.
//!
//! Defines the [`Node`] enum for specifying which MySo network to connect to
//! (mainnet, testnet, or custom) and provides URL resolution for both
//! GraphQL and JSON-RPC endpoints.

use myso_types::supported_protocol_versions::Chain;
use std::str::FromStr;

/// GraphQL endpoint for MySo mainnet.
pub const MAINNET_GQL_URL: &str = "https://graphql.mainnet.mysocial.network/graphql";
/// GraphQL endpoint for MySo testnet.
pub const TESTNET_GQL_URL: &str = "https://graphql.testnet.mysocial.network/graphql";
/// JSON-RPC endpoint for MySo mainnet.
pub const MAINNET_RPC_URL: &str = "https://fullnode.mainnet.mysocial.network:443";
/// JSON-RPC endpoint for MySo testnet.
pub const TESTNET_RPC_URL: &str = "https://fullnode.testnet.mysocial.network:443";

/// Represents a MySo network node configuration.
///
/// Used to specify which network the data store should connect to.
#[derive(Clone, Debug)]
pub enum Node {
    /// MySo mainnet
    Mainnet,
    /// MySo testnet
    Testnet,
    /// Custom network with a user-provided URL
    Custom(String),
}

impl Node {
    /// Returns the [`Chain`] identifier for this node.
    pub fn chain(&self) -> Chain {
        match self {
            Node::Mainnet => Chain::Mainnet,
            Node::Testnet => Chain::Testnet,
            Node::Custom(_) => Chain::Unknown,
        }
    }

    /// Returns a human-readable network name.
    pub fn network_name(&self) -> String {
        match self {
            Node::Mainnet => "mainnet".to_string(),
            Node::Testnet => "testnet".to_string(),
            Node::Custom(url) => url.clone(),
        }
    }

    /// Returns the GraphQL endpoint URL for this node.
    pub fn gql_url(&self) -> &str {
        match self {
            Node::Mainnet => MAINNET_GQL_URL,
            Node::Testnet => TESTNET_GQL_URL,
            Node::Custom(_url) => todo!("custom gql url not implemented"),
        }
    }

    /// Returns the JSON-RPC endpoint URL for this node.
    pub fn node_url(&self) -> &str {
        match self {
            Node::Mainnet => MAINNET_RPC_URL,
            Node::Testnet => TESTNET_RPC_URL,
            // For custom, assume it's already an RPC URL
            Node::Custom(url) => url.as_str(),
        }
    }
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mainnet" => Ok(Node::Mainnet),
            "testnet" => Ok(Node::Testnet),
            _ => Ok(Node::Custom(s.to_string())),
        }
    }
}
