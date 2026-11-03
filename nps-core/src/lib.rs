// Copyright 2026 INNO LOTUS PTY LTD
// SPDX-License-Identifier: Apache-2.0

pub mod alpha20;
pub mod cache;
pub mod codec;
pub mod error;
pub mod frames;
pub mod observability;
pub mod registry;
pub mod status_codes;
pub mod telemetry;

pub use cache::AnchorFrameCache;
pub use codec::NpsFrameCodec;
pub use error::NpsError;
pub use frames::{EncodingTier, FrameHeader, FrameType};
pub use observability::{
    json_log_line, resolve_log_level, DelegateReadinessProbe, HealthProbeRenderer,
    HealthProbeResponse, LogLevel, MetricsRegistry, PromCounterHandle, PromGaugeHandle,
    ReadinessProbe, ShutdownState, DEFAULT_DRAIN_TIMEOUT_SECS, LOG_LEVEL_ENV_VAR,
    METRICS_CONTENT_TYPE,
};
pub use registry::FrameRegistry;
pub use telemetry::{
    Counter, Histogram, Instrument, InstrumentInfo, Meter, MetricKind, MetricSnapshot,
    RecordedSpan, Span, SpanRecorder, TraceSource,
};
