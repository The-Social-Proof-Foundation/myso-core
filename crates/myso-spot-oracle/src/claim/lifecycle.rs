// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use uuid::Uuid;

use crate::store::markets::MarketRow;
use crate::types::MarketStatus;

/// Events that drive off-chain market status transitions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleEvent {
    ReviewEnqueued,
    ReviewRejected,
    ReviewAccepted,
    CreateTxConfirmed,
    ResolveAttemptStarted,
    ResolveTxConfirmed { high_confidence: bool },
    RefundTxConfirmed,
    DeadlineExceeded,
    Failed { reason: String },
}

/// Optional context recorded in `market_transitions`.
#[derive(Debug, Clone, Default)]
pub struct TransitionContext {
    pub trigger: String,
    pub job_id: Option<Uuid>,
    pub tx_digest: Option<String>,
    pub status_reason: Option<String>,
    pub on_chain_status: Option<i16>,
}

/// Compute the next status for a market given a lifecycle event.
pub fn apply_transition(
    current: MarketStatus,
    event: &LifecycleEvent,
) -> anyhow::Result<MarketStatus> {
    let next = match (current, event) {
        (MarketStatus::PostCreated, LifecycleEvent::ReviewEnqueued) => MarketStatus::PendingReview,
        (MarketStatus::PendingReview, LifecycleEvent::ReviewRejected) => MarketStatus::Rejected,
        (MarketStatus::PendingReview, LifecycleEvent::ReviewAccepted) => {
            MarketStatus::PendingCreate
        }
        (MarketStatus::PendingCreate, LifecycleEvent::CreateTxConfirmed) => MarketStatus::Waiting,
        (
            MarketStatus::Waiting | MarketStatus::Resolving,
            LifecycleEvent::ResolveAttemptStarted,
        ) => MarketStatus::Resolving,
        (
            MarketStatus::Resolving,
            LifecycleEvent::ResolveTxConfirmed {
                high_confidence: true,
            },
        ) => MarketStatus::Resolved,
        (
            MarketStatus::Resolving,
            LifecycleEvent::ResolveTxConfirmed {
                high_confidence: false,
            },
        ) => MarketStatus::DaoRequired,
        (
            MarketStatus::Waiting | MarketStatus::Resolving | MarketStatus::DaoRequired,
            LifecycleEvent::RefundTxConfirmed,
        ) => MarketStatus::Refunded,
        (_, LifecycleEvent::Failed { .. }) => MarketStatus::Failed,
        (terminal, _) if terminal.is_terminal() => {
            anyhow::bail!(
                "cannot transition from terminal status {}",
                terminal.as_str()
            )
        }
        (from, event) => {
            anyhow::bail!("illegal transition from {} via {:?}", from.as_str(), event)
        }
    };
    Ok(next)
}

pub fn market_status(row: &MarketRow) -> MarketStatus {
    MarketStatus::from_str(&row.status).unwrap_or(MarketStatus::PostCreated)
}

pub fn default_context_for(event: &LifecycleEvent) -> TransitionContext {
    let trigger = match event {
        LifecycleEvent::ReviewEnqueued => "review_enqueued",
        LifecycleEvent::ReviewRejected => "review_rejected",
        LifecycleEvent::ReviewAccepted => "review_accepted",
        LifecycleEvent::CreateTxConfirmed => "create_tx_confirmed",
        LifecycleEvent::ResolveAttemptStarted => "resolve_attempt_started",
        LifecycleEvent::ResolveTxConfirmed {
            high_confidence: true,
        } => "resolve_tx_confirmed",
        LifecycleEvent::ResolveTxConfirmed {
            high_confidence: false,
        } => "resolve_tx_dao_required",
        LifecycleEvent::RefundTxConfirmed => "refund_tx_confirmed",
        LifecycleEvent::DeadlineExceeded => "deadline_exceeded",
        LifecycleEvent::Failed { .. } => "failed",
    };
    TransitionContext {
        trigger: trigger.to_string(),
        job_id: None,
        tx_digest: None,
        status_reason: None,
        on_chain_status: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path_transitions() {
        let mut status = MarketStatus::PostCreated;
        status = apply_transition(status, &LifecycleEvent::ReviewEnqueued).unwrap();
        assert_eq!(status, MarketStatus::PendingReview);
        status = apply_transition(status, &LifecycleEvent::ReviewAccepted).unwrap();
        assert_eq!(status, MarketStatus::PendingCreate);
        status = apply_transition(status, &LifecycleEvent::CreateTxConfirmed).unwrap();
        assert_eq!(status, MarketStatus::Waiting);
        status = apply_transition(status, &LifecycleEvent::ResolveAttemptStarted).unwrap();
        assert_eq!(status, MarketStatus::Resolving);
        status = apply_transition(
            status,
            &LifecycleEvent::ResolveTxConfirmed {
                high_confidence: true,
            },
        )
        .unwrap();
        assert_eq!(status, MarketStatus::Resolved);
    }

    #[test]
    fn low_confidence_goes_to_dao_required() {
        let status = apply_transition(
            MarketStatus::Resolving,
            &LifecycleEvent::ResolveTxConfirmed {
                high_confidence: false,
            },
        )
        .unwrap();
        assert_eq!(status, MarketStatus::DaoRequired);
    }

    #[test]
    fn illegal_transition_rejected() {
        let err = apply_transition(MarketStatus::PostCreated, &LifecycleEvent::ReviewAccepted);
        assert!(err.is_err());
    }

    #[test]
    fn terminal_status_blocks_transitions() {
        let err = apply_transition(MarketStatus::Resolved, &LifecycleEvent::ReviewEnqueued);
        assert!(err.is_err());
    }
}
