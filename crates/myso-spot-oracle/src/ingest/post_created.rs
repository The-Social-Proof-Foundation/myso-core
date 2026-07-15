// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! BCS parsers for `PostCreatedEvent` variants.
//! Source of truth: `myso-indexer-alt-social/src/handlers/events.rs`.

use move_core_types::account_address::AccountAddress;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct ParsedPostCreated {
    pub post_id: String,
    pub owner: String,
    pub content: String,
    pub post_type: String,
    pub created_at_ms: i64,
}

#[derive(Debug, Deserialize)]
struct BcsPostCreatedEvent {
    post_id: AccountAddress,
    owner: AccountAddress,
    #[allow(dead_code)]
    profile_id: AccountAddress,
    #[allow(dead_code)]
    platform_id: AccountAddress,
    #[allow(dead_code)]
    permissions: u8,
    content: String,
    post_type: String,
    #[allow(dead_code)]
    parent_post_id: Option<AccountAddress>,
    #[allow(dead_code)]
    mentions: Option<Vec<AccountAddress>>,
    #[allow(dead_code)]
    media_urls: Option<Vec<String>>,
    #[allow(dead_code)]
    metadata_json: Option<String>,
    #[allow(dead_code)]
    mydata_id: Option<AccountAddress>,
    #[allow(dead_code)]
    promotion_id: Option<AccountAddress>,
    #[allow(dead_code)]
    revenue_redirect_to: Option<AccountAddress>,
    #[allow(dead_code)]
    revenue_redirect_percentage: Option<u64>,
    #[allow(dead_code)]
    enable_spt: bool,
    #[allow(dead_code)]
    spt_id: Option<AccountAddress>,
    #[allow(dead_code)]
    poc_redirection_kind: u8,
    #[allow(dead_code)]
    actor_address: AccountAddress,
    #[allow(dead_code)]
    sub_agent_id: Option<AccountAddress>,
    #[allow(dead_code)]
    organization_id: Option<AccountAddress>,
    #[allow(dead_code)]
    action_identity_class: u8,
}

pub fn parse_post_created(contents: &[u8]) -> anyhow::Result<ParsedPostCreated> {
    let ev = bcs::from_bytes::<BcsPostCreatedEvent>(contents)?;
    Ok(to_parsed(ev.post_id, ev.owner, ev.content, ev.post_type))
}

fn to_parsed(
    post_id: AccountAddress,
    owner: AccountAddress,
    content: String,
    post_type: String,
) -> ParsedPostCreated {
    ParsedPostCreated {
        post_id: format!("0x{post_id}"),
        owner: format!("0x{owner}"),
        content,
        post_type,
        created_at_ms: chrono::Utc::now().timestamp_millis(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_bytes() {
        assert!(parse_post_created(&[]).is_err());
    }
}
