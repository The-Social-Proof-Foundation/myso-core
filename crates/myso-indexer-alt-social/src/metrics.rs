// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use prometheus::{register_counter_vec_with_registry, register_counter_with_registry, Counter, CounterVec, Registry};
use std::sync::OnceLock;

static METRICS: OnceLock<SocialMetrics> = OnceLock::new();

pub struct SocialMetrics {
    // Event processing metrics
    pub events_processed: CounterVec,
    pub events_bcs_parse_failed: CounterVec,
    pub events_json_deserialize_failed: CounterVec,
    pub events_empty_contents: CounterVec,
    
    // Profile-specific metrics
    pub profiles_created: Counter,
    pub profiles_updated: Counter,
    pub profile_insertions_failed: Counter,
    pub profile_updates_failed: Counter,
    
    // Module-level metrics
    pub module_events_routed: CounterVec,
    pub module_events_ignored: CounterVec,

    /// `TokenPoolCreatedEvent` parsed with legacy BCS (no circulating_supply / total_reserved_at_launch).
    pub spt_token_pool_created_legacy_bcs: Counter,
    /// Pool row inserted with zero circulating_supply while reservation ledger has net MYSO for that launch.
    pub spt_pool_zero_supply_with_reservations: Counter,
    /// Launch split used `SUM(spt_reservations)` as denominator because `total_reserved_at_launch` was 0.
    pub spt_launch_denominator_ledger_fallback: Counter,
}

impl SocialMetrics {
    pub fn new(registry: &Registry) -> anyhow::Result<Self> {
        let events_processed = register_counter_vec_with_registry!(
            "myso_social_events_processed_total",
            "Total number of social events processed by module and event type",
            &["module", "event_type"],
            registry
        )?;
        
        let events_bcs_parse_failed = register_counter_vec_with_registry!(
            "myso_social_events_bcs_parse_failed_total",
            "Total number of events that failed BCS parsing by module and event type",
            &["module", "event_type"],
            registry
        )?;
        
        let events_json_deserialize_failed = register_counter_vec_with_registry!(
            "myso_social_events_json_deserialize_failed_total",
            "Total number of events that failed JSON deserialization by module and event type",
            &["module", "event_type"],
            registry
        )?;
        
        let events_empty_contents = register_counter_vec_with_registry!(
            "myso_social_events_empty_contents_total",
            "Total number of events with empty contents by module and event type",
            &["module", "event_type"],
            registry
        )?;
        
        let profiles_created = register_counter_with_registry!(
            "myso_social_profiles_created_total",
            "Total number of profiles successfully created",
            registry
        )?;
        
        let profiles_updated = register_counter_with_registry!(
            "myso_social_profiles_updated_total",
            "Total number of profiles successfully updated",
            registry
        )?;
        
        let profile_insertions_failed = register_counter_with_registry!(
            "myso_social_profile_insertions_failed_total",
            "Total number of profile insertions that failed",
            registry
        )?;
        
        let profile_updates_failed = register_counter_with_registry!(
            "myso_social_profile_updates_failed_total",
            "Total number of profile updates that failed",
            registry
        )?;
        
        let module_events_routed = register_counter_vec_with_registry!(
            "myso_social_module_events_routed_total",
            "Total number of events successfully routed by module",
            &["module"],
            registry
        )?;
        
        let module_events_ignored = register_counter_vec_with_registry!(
            "myso_social_module_events_ignored_total",
            "Total number of events ignored (no handler) by module",
            &["module"],
            registry
        )?;

        let spt_token_pool_created_legacy_bcs = register_counter_with_registry!(
            "myso_social_spt_token_pool_created_legacy_bcs_total",
            "TokenPoolCreatedEvent BCS fell back to legacy layout (supply fields zeroed)",
            registry
        )?;

        let spt_pool_zero_supply_with_reservations = register_counter_with_registry!(
            "myso_social_spt_pool_zero_supply_with_reservations_total",
            "SPT pool inserted with circulating_supply=0 but spt_reservations net MYSO > 0 for associated_id",
            registry
        )?;

        let spt_launch_denominator_ledger_fallback = register_counter_with_registry!(
            "myso_social_spt_launch_denominator_ledger_fallback_total",
            "SPT launch proportional split used reservation ledger sum because total_reserved_at_launch was 0",
            registry
        )?;
        
        Ok(Self {
            events_processed,
            events_bcs_parse_failed,
            events_json_deserialize_failed,
            events_empty_contents,
            profiles_created,
            profiles_updated,
            profile_insertions_failed,
            profile_updates_failed,
            module_events_routed,
            module_events_ignored,
            spt_token_pool_created_legacy_bcs,
            spt_pool_zero_supply_with_reservations,
            spt_launch_denominator_ledger_fallback,
        })
    }
    
    pub fn record_event_processed(module: &str, event_type: &str) {
        if let Some(metrics) = Self::get() {
            metrics.events_processed.with_label_values(&[module, event_type]).inc();
        }
    }
    
    pub fn record_event_bcs_parse_failed(module: &str, event_type: &str) {
        if let Some(metrics) = Self::get() {
            metrics.events_bcs_parse_failed.with_label_values(&[module, event_type]).inc();
        }
    }
    
    pub fn record_event_json_deserialize_failed(module: &str, event_type: &str) {
        if let Some(metrics) = Self::get() {
            metrics.events_json_deserialize_failed.with_label_values(&[module, event_type]).inc();
        }
    }
    
    pub fn record_event_empty_contents(module: &str, event_type: &str) {
        if let Some(metrics) = Self::get() {
            metrics.events_empty_contents.with_label_values(&[module, event_type]).inc();
        }
    }
    
    pub fn record_profile_created() {
        if let Some(metrics) = Self::get() {
            metrics.profiles_created.inc();
        }
    }
    
    pub fn record_profile_updated() {
        if let Some(metrics) = Self::get() {
            metrics.profiles_updated.inc();
        }
    }
    
    pub fn record_profile_insertion_failed() {
        if let Some(metrics) = Self::get() {
            metrics.profile_insertions_failed.inc();
        }
    }
    
    pub fn record_profile_update_failed() {
        if let Some(metrics) = Self::get() {
            metrics.profile_updates_failed.inc();
        }
    }
    
    pub fn record_module_event_routed(module: &str) {
        if let Some(metrics) = Self::get() {
            metrics.module_events_routed.with_label_values(&[module]).inc();
        }
    }
    
    pub fn record_module_event_ignored(module: &str) {
        if let Some(metrics) = Self::get() {
            metrics.module_events_ignored.with_label_values(&[module]).inc();
        }
    }

    pub fn record_spt_token_pool_created_legacy_bcs() {
        if let Some(metrics) = Self::get() {
            metrics.spt_token_pool_created_legacy_bcs.inc();
        }
    }

    pub fn record_spt_pool_zero_supply_with_reservations() {
        if let Some(metrics) = Self::get() {
            metrics.spt_pool_zero_supply_with_reservations.inc();
        }
    }

    pub fn record_spt_launch_denominator_ledger_fallback() {
        if let Some(metrics) = Self::get() {
            metrics.spt_launch_denominator_ledger_fallback.inc();
        }
    }
    
    /// Get the global metrics instance. Returns None if not initialized.
    pub fn get() -> Option<&'static SocialMetrics> {
        METRICS.get()
    }
    
    /// Initialize the global metrics instance. Must be called once at startup.
    pub fn init(metrics: SocialMetrics) {
        if METRICS.set(metrics).is_err() {
            tracing::warn!("SocialMetrics already initialized");
        }
    }
}

/// Helper macro for registering counters
#[macro_export]
macro_rules! register_counter_with_registry {
    ($name:expr, $help:expr, $registry:expr) => {
        prometheus::register_counter_with_registry!($name, $help, $registry)
    }
}

/// Helper macro for registering counter vectors
#[macro_export]
macro_rules! register_counter_vec_with_registry {
    ($name:expr, $help:expr, $labels:expr, $registry:expr) => {
        prometheus::register_counter_vec_with_registry!($name, $help, $labels, $registry)
    }
}