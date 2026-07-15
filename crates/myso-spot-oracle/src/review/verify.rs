// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Past-claim verification: resolve an already-happened claim against trusted sources *now* and
//! emit a `true`/`false`/`unverifiable` verdict with cited evidence. Never opens a market; when a
//! prior prediction market exists for the same claim it is attached as a related reference.

use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::api::AppState;
use crate::review::canonicalize::{market_key_hash_hex, CanonicalClaim};
use crate::review::compiler::ResolverCompiler;
use crate::resolver::engine::fetch_and_evaluate;
use crate::store::SpotTrustedSourceRow;

/// Verdict values, mirroring the Move/indexer encoding.
pub const VERDICT_TRUE: u8 = 1;
pub const VERDICT_FALSE: u8 = 2;
pub const VERDICT_UNVERIFIABLE: u8 = 3;

#[derive(Debug, Clone)]
pub struct PastVerdict {
    pub verdict: u8,
    pub evidence_urls: Vec<String>,
    /// 32-byte SHA-256 over the ordered evidence urls (empty-safe).
    pub evidence_hash: Vec<u8>,
    /// On-chain object id of a related historical/open market, if one exists for this claim.
    pub related_market_object_id: Option<String>,
    pub summary: String,
}

/// Verify a past claim end to end. Looks up any related market by exact `market_key_hash`,
/// compiles a verification policy, fetches its trusted sources, and evaluates them into a verdict.
/// Uncompilable claims and source/adapter failures degrade to `unverifiable` rather than erroring
/// — a post always finalizes. Never opens a market.
pub async fn verify_and_build_verdict(
    state: &Arc<AppState>,
    canonical: &CanonicalClaim,
    source_rows: &[SpotTrustedSourceRow],
) -> PastVerdict {
    let market_hex = market_key_hash_hex(canonical);
    let related_market_object_id =
        match crate::store::claims::find_market_by_key_hash(state.store.pool(), &market_hex).await {
            Ok(Some(m)) => m.spot_market_object_id,
            _ => None,
        };

    let compiled = match ResolverCompiler::compile(canonical, &state.sources, source_rows) {
        Ok(c) => c,
        Err(err) => {
            return PastVerdict {
                verdict: VERDICT_UNVERIFIABLE,
                evidence_urls: Vec::new(),
                evidence_hash: hash_evidence(&[]),
                related_market_object_id,
                summary: format!("uncompilable claim: {err}"),
            };
        }
    };

    match fetch_and_evaluate(&state.sources, &compiled.resolver_definition).await {
        Ok((draft, _evidence)) => {
            let verdict = verdict_from_outcome(draft.outcome_label.as_deref());
            let evidence_hash = hash_evidence(&draft.evidence_urls);
            PastVerdict {
                verdict,
                evidence_urls: draft.evidence_urls,
                evidence_hash,
                related_market_object_id,
                summary: draft.reasoning,
            }
        }
        Err(err) => PastVerdict {
            verdict: VERDICT_UNVERIFIABLE,
            evidence_urls: Vec::new(),
            evidence_hash: hash_evidence(&[]),
            related_market_object_id,
            summary: format!("no trusted source could verify: {err}"),
        },
    }
}

/// Map a resolved outcome label to a verdict. The compiler emits the claim's asserted-true option
/// first (typically "Yes"/"true"); the opposite second.
fn verdict_from_outcome(label: Option<&str>) -> u8 {
    match label.map(|s| s.to_ascii_lowercase()).as_deref() {
        Some("yes") | Some("true") => VERDICT_TRUE,
        Some("no") | Some("false") => VERDICT_FALSE,
        _ => VERDICT_UNVERIFIABLE,
    }
}

fn hash_evidence(urls: &[String]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    for url in urls {
        hasher.update(url.as_bytes());
        hasher.update(b"\n");
    }
    hasher.finalize().to_vec()
}
