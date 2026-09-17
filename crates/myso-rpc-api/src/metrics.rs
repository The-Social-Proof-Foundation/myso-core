// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::http;
use std::{borrow::Cow, sync::Arc, time::Instant};

use mysten_network::callback::{MakeCallbackHandler, ResponseHandler};
use prometheus::{
    HistogramVec, IntCounterVec, IntGauge, IntGaugeVec, Registry,
    register_histogram_vec_with_registry, register_int_counter_vec_with_registry,
    register_int_gauge_vec_with_registry, register_int_gauge_with_registry,
};

#[derive(Clone)]
pub struct RpcMetrics {
    inflight_requests: IntGaugeVec,
    num_requests: IntCounterVec,
    request_latency: HistogramVec,
    request_handler_latency: HistogramVec,
}

const LATENCY_SEC_BUCKETS: &[f64] = &[
    0.001, 0.005, 0.01, 0.05, 0.1, 0.25, 0.5, 1., 2.5, 5., 10., 20., 30., 60., 90.,
];

impl RpcMetrics {
    pub fn new(registry: &Registry) -> Self {
        Self {
            inflight_requests: register_int_gauge_vec_with_registry!(
                "rpc_inflight_requests",
                "Total in-flight RPC requests per route",
                &["path"],
                registry,
            )
            .unwrap(),
            num_requests: register_int_counter_vec_with_registry!(
                "rpc_requests",
                "Total RPC requests per route and their http status",
                &["path", "status"],
                registry,
            )
            .unwrap(),
            request_latency: register_histogram_vec_with_registry!(
                "rpc_request_latency",
                "Latency of RPC requests per route, measured from receipt of the request \
                 until the response body finished streaming back to the client",
                &["path"],
                LATENCY_SEC_BUCKETS.to_vec(),
                registry,
            )
            .unwrap(),
            request_handler_latency: register_histogram_vec_with_registry!(
                "rpc_request_handler_latency",
                "Latency of RPC requests per route, measured from receipt of the request \
                 until the request handler produced a response, excluding the time spent \
                 streaming the response body back to the client",
                &["path"],
                LATENCY_SEC_BUCKETS.to_vec(),
                registry,
            )
            .unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct RpcMetricsMakeCallbackHandler {
    metrics: Arc<RpcMetrics>,
}

impl RpcMetricsMakeCallbackHandler {
    pub fn new(metrics: Arc<RpcMetrics>) -> Self {
        Self { metrics }
    }
}

impl MakeCallbackHandler for RpcMetricsMakeCallbackHandler {
    type Handler = RpcMetricsCallbackHandler;

    fn make_handler(&self, request: &http::request::Parts) -> Self::Handler {
        let start = Instant::now();
        let metrics = self.metrics.clone();

        let path =
            if let Some(matched_path) = request.extensions.get::<axum::extract::MatchedPath>() {
                if request
                    .headers
                    .get(&http::header::CONTENT_TYPE)
                    .is_some_and(|header| header == tonic::metadata::GRPC_CONTENT_TYPE)
                {
                    Cow::Owned(request.uri.path().to_owned())
                } else {
                    Cow::Owned(matched_path.as_str().to_owned())
                }
            } else {
                Cow::Borrowed("unknown")
            };

        metrics
            .inflight_requests
            .with_label_values(&[path.as_ref()])
            .inc();

        RpcMetricsCallbackHandler {
            metrics,
            path,
            start,
            counted_response: false,
        }
    }
}

pub struct RpcMetricsCallbackHandler {
    metrics: Arc<RpcMetrics>,
    path: Cow<'static, str>,
    start: Instant,
    // Indicates if we successfully counted the response. In some cases when a request is
    // prematurely canceled this will remain false
    counted_response: bool,
}

impl ResponseHandler for RpcMetricsCallbackHandler {
    fn on_response(&mut self, response: &http::response::Parts) {
        const GRPC_STATUS: http::HeaderName = http::HeaderName::from_static("grpc-status");

        // Unlike `request_latency` (observed in `Drop`, after the response
        // body finished streaming), this fires as soon as the handler
        // produced a response, so it excludes client-side network latency.
        self.metrics
            .request_handler_latency
            .with_label_values(&[self.path.as_ref()])
            .observe(self.start.elapsed().as_secs_f64());

        let status = if response
            .headers
            .get(&http::header::CONTENT_TYPE)
            .is_some_and(|content_type| {
                content_type
                    .as_bytes()
                    // check if the content-type starts_with 'application/grpc' in order to
                    // consider this as a gRPC request. A prefix comparison is done instead of a
                    // full equality check in order to account for the various types of
                    // content-types that are considered as gRPC traffic.
                    .starts_with(tonic::metadata::GRPC_CONTENT_TYPE.as_bytes())
            }) {
            let code = response
                .headers
                .get(&GRPC_STATUS)
                .map(http::HeaderValue::as_bytes)
                .map(tonic::Code::from_bytes)
                .unwrap_or(tonic::Code::Ok);

            code_as_str(code)
        } else {
            response.status.as_str()
        };

        self.metrics
            .num_requests
            .with_label_values(&[self.path.as_ref(), status])
            .inc();

        self.counted_response = true;
    }

    fn on_error<E>(&mut self, _error: &E) {
        // Do nothing if the whole service errored
        //
        // in Axum this isn't possible since all services are required to have an error type of
        // Infallible
    }
}

impl Drop for RpcMetricsCallbackHandler {
    fn drop(&mut self) {
        self.metrics
            .inflight_requests
            .with_label_values(&[self.path.as_ref()])
            .dec();

        let latency = self.start.elapsed().as_secs_f64();
        self.metrics
            .request_latency
            .with_label_values(&[self.path.as_ref()])
            .observe(latency);

        if !self.counted_response {
            self.metrics
                .num_requests
                .with_label_values(&[self.path.as_ref(), "canceled"])
                .inc();
        }
    }
}

fn code_as_str(code: tonic::Code) -> &'static str {
    match code {
        tonic::Code::Ok => "ok",
        tonic::Code::Cancelled => "canceled",
        tonic::Code::Unknown => "unknown",
        tonic::Code::InvalidArgument => "invalid-argument",
        tonic::Code::DeadlineExceeded => "deadline-exceeded",
        tonic::Code::NotFound => "not-found",
        tonic::Code::AlreadyExists => "already-exists",
        tonic::Code::PermissionDenied => "permission-denied",
        tonic::Code::ResourceExhausted => "resource-exhausted",
        tonic::Code::FailedPrecondition => "failed-precondition",
        tonic::Code::Aborted => "aborted",
        tonic::Code::OutOfRange => "out-of-range",
        tonic::Code::Unimplemented => "unimplemented",
        tonic::Code::Internal => "internal",
        tonic::Code::Unavailable => "unavailable",
        tonic::Code::DataLoss => "data-loss",
        tonic::Code::Unauthenticated => "unauthenticated",
    }
}

#[derive(Clone)]
pub(crate) struct SubscriptionMetrics {
    pub inflight_subscribers: IntGauge,
    pub last_recieved_checkpoint: IntGauge,
}

impl SubscriptionMetrics {
    pub fn new(registry: &Registry) -> Self {
        Self {
            inflight_subscribers: register_int_gauge_with_registry!(
                "subscription_inflight_subscribers",
                "Total in-flight subscriptions",
                registry,
            )
            .unwrap(),
            last_recieved_checkpoint: register_int_gauge_with_registry!(
                "subscription_last_recieved_checkpoint",
                "Last recieved checkpoint by the subscription service",
                registry,
            )
            .unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a handler for a request with no matched path, so all metric
    /// observations land on the "unknown" label.
    fn make_test_handler(metrics: &Arc<RpcMetrics>) -> RpcMetricsCallbackHandler {
        let make = RpcMetricsMakeCallbackHandler::new(metrics.clone());
        let (parts, _) = http::Request::new(()).into_parts();
        make.make_handler(&parts)
    }

    // The handler latency is observed as soon as the handler produces a
    // response, while the total request latency is only observed once the
    // handler is dropped (i.e. the response body finished streaming).
    #[test]
    fn handler_latency_observed_on_response_and_total_latency_on_drop() {
        let metrics = Arc::new(RpcMetrics::new(&Registry::new()));
        let mut handler = make_test_handler(&metrics);

        let handler_latency = metrics
            .request_handler_latency
            .with_label_values(&["unknown"]);
        let total_latency = metrics.request_latency.with_label_values(&["unknown"]);

        assert_eq!(handler_latency.get_sample_count(), 0);

        let (parts, _) = http::Response::new(()).into_parts();
        handler.on_response(&parts);

        assert_eq!(handler_latency.get_sample_count(), 1);
        assert_eq!(total_latency.get_sample_count(), 0);

        drop(handler);

        assert_eq!(handler_latency.get_sample_count(), 1);
        assert_eq!(total_latency.get_sample_count(), 1);
    }

    // A request canceled before the handler produces a response records the
    // total latency and the canceled count, but no handler latency.
    #[test]
    fn handler_latency_not_observed_for_canceled_requests() {
        let metrics = Arc::new(RpcMetrics::new(&Registry::new()));
        let handler = make_test_handler(&metrics);

        drop(handler);

        assert_eq!(
            metrics
                .request_handler_latency
                .with_label_values(&["unknown"])
                .get_sample_count(),
            0
        );
        assert_eq!(
            metrics
                .request_latency
                .with_label_values(&["unknown"])
                .get_sample_count(),
            1
        );
        assert_eq!(
            metrics
                .num_requests
                .with_label_values(&["unknown", "canceled"])
                .get(),
            1
        );
    }
}
