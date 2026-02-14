// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Event handlers by module. Routing mirrors mys-indexer checkpoint_processor.
//! Stub handlers return empty vec; fill in functionality later.

mod blocking;
mod governance;
mod insurance;
mod mydata;
mod platform;
mod poc;
mod post;
mod profile;
mod social_graph;
mod spot;
mod spt;
mod subscription;
mod upgrade;

pub(super) use super::{ProfileUpdate, SocialEventRow};

pub fn route_event(
    module: &str,
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match module {
        "governance" => governance::handle_governance_event(event_name, data, event_id),
        "block_list" | "blocking" => blocking::handle_blocking_event(event_name, data, event_id),
        "mydata" | "my_ip" => mydata::handle_mydata_event(event_name, data, event_id),
        "profile" => profile::handle_profile_event(event_name, data, event_id),
        "social_graph" => social_graph::handle_social_graph_event(event_name, data, event_id),
        "platform" => platform::handle_platform_event(event_name, data, event_id),
        "post" | "comment" | "reaction" | "repost" | "tip" => {
            post::handle_post_event(event_name, data, event_id)
        }
        "subscription" | "profile_subscription" => {
            subscription::handle_subscription_event(event_name, data, event_id)
        }
        "insurance" => insurance::handle_insurance_event(event_name, data, event_id),
        "poc" | "proof_of_creativity" => poc::handle_poc_event(event_name, data, event_id),
        "social_proof_of_truth" | "spot" => spot::handle_spot_event(event_name, data, event_id),
        "social_proof_tokens" | "spt" => spt::handle_spt_event(event_name, data, event_id),
        "upgrade" => upgrade::handle_upgrade_event(event_name, data, event_id),
        _ => None,
    }
}
