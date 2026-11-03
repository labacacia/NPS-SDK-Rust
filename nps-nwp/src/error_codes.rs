// Copyright 2026 INNO LOTUS PTY LTD
// SPDX-License-Identifier: Apache-2.0

//! NWP error code wire constants — mirror of `spec/error-codes.md` NWP section.

// ── Auth / NID ────────────────────────────────────────────────────────────────
pub const AUTH_NID_SCOPE_VIOLATION: &str = "NWP-AUTH-NID-SCOPE-VIOLATION";
pub const AUTH_NID_EXPIRED: &str = "NWP-AUTH-NID-EXPIRED";
pub const AUTH_NID_REVOKED: &str = "NWP-AUTH-NID-REVOKED";
pub const AUTH_NID_UNTRUSTED_ISSUER: &str = "NWP-AUTH-NID-UNTRUSTED-ISSUER";
pub const AUTH_NID_CAPABILITY_MISSING: &str = "NWP-AUTH-NID-CAPABILITY-MISSING";
pub const AUTH_ASSURANCE_TOO_LOW: &str = "NWP-AUTH-ASSURANCE-TOO-LOW";
/// Deprecated — alias for NWP-REPUTATION-REJECTED / NWP-REPUTATION-BANNED.
pub const AUTH_REPUTATION_BLOCKED: &str = "NWP-AUTH-REPUTATION-BLOCKED";

// ── Reputation ────────────────────────────────────────────────────────────────
pub const REPUTATION_THROTTLED: &str = "NWP-REPUTATION-THROTTLED";
pub const REPUTATION_REJECTED: &str = "NWP-REPUTATION-REJECTED";
pub const REPUTATION_BANNED: &str = "NWP-REPUTATION-BANNED";

// ── Query ─────────────────────────────────────────────────────────────────────
pub const QUERY_FILTER_INVALID: &str = "NWP-QUERY-FILTER-INVALID";
pub const QUERY_FIELD_UNKNOWN: &str = "NWP-QUERY-FIELD-UNKNOWN";
pub const QUERY_CURSOR_INVALID: &str = "NWP-QUERY-CURSOR-INVALID";
pub const QUERY_REGEX_UNSAFE: &str = "NWP-QUERY-REGEX-UNSAFE";
pub const QUERY_VECTOR_UNSUPPORTED: &str = "NWP-QUERY-VECTOR-UNSUPPORTED";
pub const QUERY_AGGREGATE_UNSUPPORTED: &str = "NWP-QUERY-AGGREGATE-UNSUPPORTED";
pub const QUERY_AGGREGATE_INVALID: &str = "NWP-QUERY-AGGREGATE-INVALID";
pub const QUERY_STREAM_UNSUPPORTED: &str = "NWP-QUERY-STREAM-UNSUPPORTED";

// ── Action ────────────────────────────────────────────────────────────────────
pub const ACTION_NOT_FOUND: &str = "NWP-ACTION-NOT-FOUND";
pub const ACTION_PARAMS_INVALID: &str = "NWP-ACTION-PARAMS-INVALID";
pub const ACTION_IDEMPOTENCY_CONFLICT: &str = "NWP-ACTION-IDEMPOTENCY-CONFLICT";
pub const LLM_CONTEXT_NOT_FOUND: &str = "NWP-LLM-CONTEXT-NOT-FOUND";
pub const LLM_CONTEXT_EXPIRED: &str = "NWP-LLM-CONTEXT-EXPIRED";
pub const LLM_CONTEXT_VERSION_CONFLICT: &str = "NWP-LLM-CONTEXT-VERSION-CONFLICT";
pub const LLM_CONTEXT_BINDING_MISMATCH: &str = "NWP-LLM-CONTEXT-BINDING-MISMATCH";
pub const LLM_CONTEXT_FORBIDDEN: &str = "NWP-LLM-CONTEXT-FORBIDDEN";
pub const LLM_CONTEXT_LIMIT_EXCEEDED: &str = "NWP-LLM-CONTEXT-LIMIT-EXCEEDED";
pub const LLM_CONTEXT_OPERATION_UNSUPPORTED: &str = "NWP-LLM-CONTEXT-OPERATION-UNSUPPORTED";

// ── Task ──────────────────────────────────────────────────────────────────────
pub const TASK_NOT_FOUND: &str = "NWP-TASK-NOT-FOUND";
pub const TASK_ALREADY_CANCELLED: &str = "NWP-TASK-ALREADY-CANCELLED";
pub const TASK_ALREADY_COMPLETED: &str = "NWP-TASK-ALREADY-COMPLETED";
pub const TASK_ALREADY_FAILED: &str = "NWP-TASK-ALREADY-FAILED";

// ── Subscribe ─────────────────────────────────────────────────────────────────
pub const SUBSCRIBE_STREAM_NOT_FOUND: &str = "NWP-SUBSCRIBE-STREAM-NOT-FOUND";
pub const SUBSCRIBE_LIMIT_EXCEEDED: &str = "NWP-SUBSCRIBE-LIMIT-EXCEEDED";
pub const SUBSCRIBE_FILTER_UNSUPPORTED: &str = "NWP-SUBSCRIBE-FILTER-UNSUPPORTED";
pub const SUBSCRIBE_INTERRUPTED: &str = "NWP-SUBSCRIBE-INTERRUPTED";
pub const SUBSCRIBE_SEQ_TOO_OLD: &str = "NWP-SUBSCRIBE-SEQ-TOO-OLD";
pub const SUBSCRIBE_LEASE_INVALID: &str = "NWP-SUBSCRIBE-LEASE-INVALID";
pub const SUBSCRIBE_LEASE_EXPIRED: &str = "NWP-SUBSCRIBE-LEASE-EXPIRED";

// ── Budget / depth ────────────────────────────────────────────────────────────
pub const BUDGET_EXCEEDED: &str = "NWP-BUDGET-EXCEEDED";
pub const CGN_LIMIT_EXCEEDED: &str = "NWP-CGN-LIMIT-EXCEEDED";
pub const DEPTH_EXCEEDED: &str = "NWP-DEPTH-EXCEEDED";
pub const GRAPH_CYCLE: &str = "NWP-GRAPH-CYCLE";

// ── Node availability ─────────────────────────────────────────────────────────
pub const NODE_UNAVAILABLE: &str = "NWP-NODE-UNAVAILABLE";

// ── Manifest ──────────────────────────────────────────────────────────────────
pub const MANIFEST_VERSION_UNSUPPORTED: &str = "NWP-MANIFEST-VERSION-UNSUPPORTED";
pub const MANIFEST_NODE_TYPE_REMOVED: &str = "NWP-MANIFEST-NODE-TYPE-REMOVED";
pub const MANIFEST_NODE_TYPE_UNKNOWN: &str = "NWP-MANIFEST-NODE-TYPE-UNKNOWN";

// ── Rate / reserved ───────────────────────────────────────────────────────────
pub const RATE_LIMIT_EXCEEDED: &str = "NWP-RATE-LIMIT-EXCEEDED";
pub const RESERVED_TYPE_UNSUPPORTED: &str = "NWP-RESERVED-TYPE-UNSUPPORTED";

// ── HTTP binding / advertised capability ─────────────────────────────────────
pub const HTTP_ORIGIN_FORBIDDEN: &str = "NWP-HTTP-ORIGIN-FORBIDDEN";
pub const HTTP_CONTENT_TYPE_UNSUPPORTED: &str = "NWP-HTTP-CONTENT-TYPE-UNSUPPORTED";
pub const HTTP_ACCEPT_UNSATISFIABLE: &str = "NWP-HTTP-ACCEPT-UNSATISFIABLE";
pub const HTTP_REQUEST_ID_MISMATCH: &str = "NWP-HTTP-REQUEST-ID-MISMATCH";
pub const HTTP_FRAME_BODY_MALFORMED: &str = "NWP-HTTP-FRAME-BODY-MALFORMED";
pub const HTTP_BODY_TOO_LARGE: &str = "NWP-HTTP-BODY-TOO-LARGE";
pub const CAPABILITY_ADVERTISED_UNIMPLEMENTED: &str = "NWP-CAPABILITY-ADVERTISED-UNIMPLEMENTED";

// ── Topology (NPS-CR-0002) ────────────────────────────────────────────────────
pub const TOPOLOGY_UNAUTHORIZED: &str = "NWP-TOPOLOGY-UNAUTHORIZED";
pub const TOPOLOGY_UNSUPPORTED_SCOPE: &str = "NWP-TOPOLOGY-UNSUPPORTED-SCOPE";
pub const TOPOLOGY_DEPTH_UNSUPPORTED: &str = "NWP-TOPOLOGY-DEPTH-UNSUPPORTED";
pub const TOPOLOGY_FILTER_UNSUPPORTED: &str = "NWP-TOPOLOGY-FILTER-UNSUPPORTED";

// ── Multi-Anchor HA (NPS-CR-0009) ─────────────────────────────────────────────
/// A topology WRITE reached a standby, or the active owner while
/// read-only-degraded (quorum lost).
pub const ANCHOR_NOT_LEADER: &str = "NWP-ANCHOR-NOT-LEADER";
/// An inbound frame carried a `cluster_epoch` STRICTLY GREATER than the
/// receiver's own — the receiver is a superseded leader and self-fences.
/// `<=` the receiver's own epoch is deliberately NOT an error.
pub const ANCHOR_EPOCH_FENCED: &str = "NWP-ANCHOR-EPOCH-FENCED";

// ── Bridge (NPS-CR-0010) ──────────────────────────────────────────────────────
/// Both directions. The request targets a protocol/direction pair this Bridge
/// Node did not declare.
pub const BRIDGE_DIRECTION_UNSUPPORTED: &str = "NWP-BRIDGE-DIRECTION-UNSUPPORTED";
/// Outbound. No valid `bridge_target` object, or it fails schema validation.
pub const BRIDGE_TARGET_INVALID: &str = "NWP-BRIDGE-TARGET-INVALID";
/// Outbound. `bridge_target.protocol` is well-formed but has no dispatcher.
pub const BRIDGE_PROTOCOL_UNSUPPORTED: &str = "NWP-BRIDGE-PROTOCOL-UNSUPPORTED";
/// Outbound. `bridge_target.endpoint` is not a valid URL or is SSRF-blocked.
pub const BRIDGE_ENDPOINT_INVALID: &str = "NWP-BRIDGE-ENDPOINT-INVALID";
/// Outbound. The external call failed at the transport layer or timed out.
pub const BRIDGE_UPSTREAM_FAILED: &str = "NWP-BRIDGE-UPSTREAM-FAILED";
/// Inbound. The foreign client named a tool / action / resource not exposed.
pub const BRIDGE_SERVER_TOOL_NOT_FOUND: &str = "NWP-BRIDGE-SERVER-TOOL-NOT-FOUND";
/// Inbound. The Bridge was started without a backend for the node it fronts —
/// a deployment fault, not a client fault.
pub const BRIDGE_SERVER_DISPATCHER_MISSING: &str = "NWP-BRIDGE-SERVER-DISPATCHER-MISSING";
/// Inbound. Dispatch to the fronted NPS node failed unexpectedly.
pub const BRIDGE_SERVER_DISPATCH_FAILED: &str = "NWP-BRIDGE-SERVER-DISPATCH-FAILED";

// ── Mapping to NPS status ──────────────────────────────────────────────────────

/// Map a NWP error code to the corresponding NPS status code.
pub const CHALLENGE_INVALID: &str = "NWP-CHALLENGE-INVALID";
pub const CHALLENGE_NOT_FOUND: &str = "NWP-CHALLENGE-NOT-FOUND";
pub const CHALLENGE_BINDING_CONFLICT: &str = "NWP-CHALLENGE-BINDING-CONFLICT";
pub const CHALLENGE_NOT_APPROVED: &str = "NWP-CHALLENGE-NOT-APPROVED";
pub const CHALLENGE_EXPIRED: &str = "NWP-CHALLENGE-EXPIRED";
pub const CHALLENGE_STATE_UNAVAILABLE: &str = "NWP-CHALLENGE-STATE-UNAVAILABLE";
pub const FEATURE_REQUIREMENTS_INVALID: &str = "NWP-FEATURE-REQUIREMENTS-INVALID";
pub const FEATURE_REQUIRED_UNSUPPORTED: &str = "NWP-FEATURE-REQUIRED-UNSUPPORTED";

#[cfg(test)]
mod alpha20_status_tests {
    #[test]
    fn shared_alpha20_mappings() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .map(|root| root.join("spec/conformance/alpha20/error_status_vectors.json"))
            .find(|path| path.is_file())
            .expect("shared error corpus must be distributed with the SDK");
        let raw = std::fs::read_to_string(fixture).unwrap();
        let corpus: serde_json::Value = serde_json::from_str(&raw).unwrap();
        let cases = corpus["cases"].as_array().unwrap();
        assert_eq!(cases.len(), 17);
        let mut checked = 0;
        for vector in cases {
            let code = vector["code"].as_str().unwrap();
            if !code.starts_with("NWP-") {
                continue;
            }
            let status = super::to_nps_status(code);
            assert_eq!(status, vector["status"].as_str().unwrap(), "{code}");
            assert_eq!(
                nps_core::status_codes::to_http_status(status),
                Some(vector["http"].as_u64().unwrap() as u16),
                "{code}"
            );
            checked += 1;
        }
        assert_eq!(checked, 8);
    }
}

pub fn to_nps_status(code: &str) -> &'static str {
    match code {
        CHALLENGE_INVALID => "NPS-CLIENT-BAD-FRAME",
        CHALLENGE_NOT_FOUND => "NPS-CLIENT-NOT-FOUND",
        CHALLENGE_BINDING_CONFLICT => "NPS-CLIENT-CONFLICT",
        CHALLENGE_NOT_APPROVED => "NPS-CLIENT-CONFLICT",
        CHALLENGE_EXPIRED => "NPS-CLIENT-CONFLICT",
        CHALLENGE_STATE_UNAVAILABLE => "NPS-SERVER-UNAVAILABLE",
        FEATURE_REQUIREMENTS_INVALID => "NPS-CLIENT-BAD-FRAME",
        FEATURE_REQUIRED_UNSUPPORTED => "NPS-CLIENT-CONFLICT",
        AUTH_NID_SCOPE_VIOLATION => "NPS-AUTH-FORBIDDEN",
        AUTH_NID_EXPIRED => "NPS-AUTH-UNAUTHENTICATED",
        AUTH_NID_REVOKED => "NPS-AUTH-UNAUTHENTICATED",
        AUTH_NID_UNTRUSTED_ISSUER => "NPS-AUTH-UNAUTHENTICATED",
        AUTH_NID_CAPABILITY_MISSING => "NPS-AUTH-FORBIDDEN",
        AUTH_ASSURANCE_TOO_LOW => "NPS-AUTH-FORBIDDEN",
        AUTH_REPUTATION_BLOCKED => "NPS-AUTH-FORBIDDEN",

        REPUTATION_THROTTLED => "NPS-CLIENT-RATE-LIMITED",
        REPUTATION_REJECTED => "NPS-AUTH-FORBIDDEN",
        REPUTATION_BANNED => "NPS-AUTH-FORBIDDEN",

        QUERY_FILTER_INVALID => "NPS-CLIENT-BAD-PARAM",
        QUERY_FIELD_UNKNOWN => "NPS-CLIENT-BAD-PARAM",
        QUERY_CURSOR_INVALID => "NPS-CLIENT-BAD-PARAM",
        QUERY_REGEX_UNSAFE => "NPS-CLIENT-BAD-PARAM",
        QUERY_VECTOR_UNSUPPORTED => "NPS-SERVER-UNSUPPORTED",
        QUERY_AGGREGATE_UNSUPPORTED => "NPS-SERVER-UNSUPPORTED",
        QUERY_AGGREGATE_INVALID => "NPS-CLIENT-BAD-PARAM",
        QUERY_STREAM_UNSUPPORTED => "NPS-SERVER-UNSUPPORTED",

        ACTION_NOT_FOUND => "NPS-CLIENT-NOT-FOUND",
        ACTION_PARAMS_INVALID => "NPS-CLIENT-UNPROCESSABLE",
        ACTION_IDEMPOTENCY_CONFLICT => "NPS-CLIENT-CONFLICT",
        LLM_CONTEXT_NOT_FOUND => "NPS-CLIENT-NOT-FOUND",
        LLM_CONTEXT_EXPIRED => "NPS-CLIENT-GONE",
        LLM_CONTEXT_VERSION_CONFLICT => "NPS-CLIENT-CONFLICT",
        LLM_CONTEXT_BINDING_MISMATCH => "NPS-CLIENT-CONFLICT",
        LLM_CONTEXT_FORBIDDEN => "NPS-AUTH-FORBIDDEN",
        LLM_CONTEXT_LIMIT_EXCEEDED => "NPS-LIMIT-RESOURCE",
        LLM_CONTEXT_OPERATION_UNSUPPORTED => "NPS-SERVER-UNSUPPORTED",

        TASK_NOT_FOUND => "NPS-CLIENT-NOT-FOUND",
        TASK_ALREADY_CANCELLED => "NPS-CLIENT-CONFLICT",
        TASK_ALREADY_COMPLETED => "NPS-CLIENT-CONFLICT",
        TASK_ALREADY_FAILED => "NPS-CLIENT-CONFLICT",

        SUBSCRIBE_STREAM_NOT_FOUND => "NPS-CLIENT-NOT-FOUND",
        SUBSCRIBE_LIMIT_EXCEEDED => "NPS-LIMIT-EXCEEDED",
        SUBSCRIBE_FILTER_UNSUPPORTED => "NPS-SERVER-UNSUPPORTED",
        SUBSCRIBE_INTERRUPTED => "NPS-SERVER-UNAVAILABLE",
        SUBSCRIBE_SEQ_TOO_OLD => "NPS-CLIENT-CONFLICT",
        SUBSCRIBE_LEASE_INVALID => "NPS-CLIENT-BAD-PARAM",
        SUBSCRIBE_LEASE_EXPIRED => "NPS-CLIENT-GONE",

        BUDGET_EXCEEDED => "NPS-LIMIT-BUDGET",
        CGN_LIMIT_EXCEEDED => "NPS-CLIENT-REQUEST-TOO-LARGE",
        DEPTH_EXCEEDED => "NPS-CLIENT-BAD-PARAM",
        GRAPH_CYCLE => "NPS-CLIENT-UNPROCESSABLE",

        NODE_UNAVAILABLE => "NPS-SERVER-UNAVAILABLE",

        MANIFEST_VERSION_UNSUPPORTED => "NPS-CLIENT-BAD-PARAM",
        MANIFEST_NODE_TYPE_REMOVED => "NPS-CLIENT-BAD-FRAME",
        MANIFEST_NODE_TYPE_UNKNOWN => "NPS-CLIENT-BAD-FRAME",

        RATE_LIMIT_EXCEEDED => "NPS-LIMIT-RATE",
        RESERVED_TYPE_UNSUPPORTED => "NPS-SERVER-UNSUPPORTED",
        HTTP_ORIGIN_FORBIDDEN => "NPS-AUTH-FORBIDDEN",
        HTTP_CONTENT_TYPE_UNSUPPORTED => "NPS-CLIENT-BAD-FRAME",
        HTTP_ACCEPT_UNSATISFIABLE => "NPS-CLIENT-BAD-PARAM",
        HTTP_REQUEST_ID_MISMATCH => "NPS-CLIENT-BAD-PARAM",
        HTTP_FRAME_BODY_MALFORMED => "NPS-CLIENT-BAD-FRAME",
        HTTP_BODY_TOO_LARGE => "NPS-LIMIT-PAYLOAD",
        CAPABILITY_ADVERTISED_UNIMPLEMENTED => "NPS-SERVER-UNSUPPORTED",

        TOPOLOGY_UNAUTHORIZED => "NPS-AUTH-FORBIDDEN",
        TOPOLOGY_UNSUPPORTED_SCOPE => "NPS-CLIENT-BAD-PARAM",
        TOPOLOGY_DEPTH_UNSUPPORTED => "NPS-CLIENT-BAD-PARAM",
        TOPOLOGY_FILTER_UNSUPPORTED => "NPS-CLIENT-BAD-PARAM",

        ANCHOR_NOT_LEADER => "NPS-CLIENT-CONFLICT",
        ANCHOR_EPOCH_FENCED => "NPS-CLIENT-CONFLICT",

        BRIDGE_DIRECTION_UNSUPPORTED => "NPS-SERVER-UNSUPPORTED",
        BRIDGE_TARGET_INVALID => "NPS-CLIENT-UNPROCESSABLE",
        BRIDGE_PROTOCOL_UNSUPPORTED => "NPS-SERVER-UNSUPPORTED",
        BRIDGE_ENDPOINT_INVALID => "NPS-CLIENT-UNPROCESSABLE",
        BRIDGE_UPSTREAM_FAILED => "NPS-DOWNSTREAM-UNAVAILABLE",
        BRIDGE_SERVER_TOOL_NOT_FOUND => "NPS-CLIENT-NOT-FOUND",
        BRIDGE_SERVER_DISPATCHER_MISSING => "NPS-SERVER-INTERNAL",
        BRIDGE_SERVER_DISPATCH_FAILED => "NPS-SERVER-INTERNAL",

        _ => "NPS-SERVER-INTERNAL",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_nid_errors() {
        assert_eq!(
            to_nps_status(AUTH_NID_SCOPE_VIOLATION),
            "NPS-AUTH-FORBIDDEN"
        );
        assert_eq!(to_nps_status(AUTH_NID_EXPIRED), "NPS-AUTH-UNAUTHENTICATED");
        assert_eq!(to_nps_status(AUTH_NID_REVOKED), "NPS-AUTH-UNAUTHENTICATED");
        assert_eq!(
            to_nps_status(AUTH_NID_UNTRUSTED_ISSUER),
            "NPS-AUTH-UNAUTHENTICATED"
        );
        assert_eq!(
            to_nps_status(AUTH_NID_CAPABILITY_MISSING),
            "NPS-AUTH-FORBIDDEN"
        );
        assert_eq!(to_nps_status(AUTH_ASSURANCE_TOO_LOW), "NPS-AUTH-FORBIDDEN");
    }

    #[test]
    fn reputation_errors() {
        assert_eq!(
            to_nps_status(REPUTATION_THROTTLED),
            "NPS-CLIENT-RATE-LIMITED"
        );
        assert_eq!(to_nps_status(REPUTATION_REJECTED), "NPS-AUTH-FORBIDDEN");
        assert_eq!(to_nps_status(REPUTATION_BANNED), "NPS-AUTH-FORBIDDEN");
    }

    #[test]
    fn query_errors() {
        assert_eq!(to_nps_status(QUERY_FILTER_INVALID), "NPS-CLIENT-BAD-PARAM");
        assert_eq!(
            to_nps_status(QUERY_VECTOR_UNSUPPORTED),
            "NPS-SERVER-UNSUPPORTED"
        );
        assert_eq!(
            to_nps_status(QUERY_AGGREGATE_UNSUPPORTED),
            "NPS-SERVER-UNSUPPORTED"
        );
        assert_eq!(
            to_nps_status(QUERY_AGGREGATE_INVALID),
            "NPS-CLIENT-BAD-PARAM"
        );
        assert_eq!(
            to_nps_status(QUERY_STREAM_UNSUPPORTED),
            "NPS-SERVER-UNSUPPORTED"
        );
    }

    #[test]
    fn action_and_task_errors() {
        assert_eq!(to_nps_status(ACTION_NOT_FOUND), "NPS-CLIENT-NOT-FOUND");
        assert_eq!(
            to_nps_status(ACTION_PARAMS_INVALID),
            "NPS-CLIENT-UNPROCESSABLE"
        );
        assert_eq!(
            to_nps_status(ACTION_IDEMPOTENCY_CONFLICT),
            "NPS-CLIENT-CONFLICT"
        );
        assert_eq!(to_nps_status(TASK_NOT_FOUND), "NPS-CLIENT-NOT-FOUND");
        assert_eq!(to_nps_status(TASK_ALREADY_CANCELLED), "NPS-CLIENT-CONFLICT");
    }

    #[test]
    fn subscribe_errors() {
        assert_eq!(
            to_nps_status(SUBSCRIBE_STREAM_NOT_FOUND),
            "NPS-CLIENT-NOT-FOUND"
        );
        assert_eq!(
            to_nps_status(SUBSCRIBE_LIMIT_EXCEEDED),
            "NPS-LIMIT-EXCEEDED"
        );
        assert_eq!(
            to_nps_status(SUBSCRIBE_FILTER_UNSUPPORTED),
            "NPS-SERVER-UNSUPPORTED"
        );
        assert_eq!(
            to_nps_status(SUBSCRIBE_INTERRUPTED),
            "NPS-SERVER-UNAVAILABLE"
        );
        assert_eq!(to_nps_status(SUBSCRIBE_SEQ_TOO_OLD), "NPS-CLIENT-CONFLICT");
    }

    #[test]
    fn budget_and_limit_errors() {
        assert_eq!(to_nps_status(BUDGET_EXCEEDED), "NPS-LIMIT-BUDGET");
        assert_eq!(
            to_nps_status(CGN_LIMIT_EXCEEDED),
            "NPS-CLIENT-REQUEST-TOO-LARGE"
        );
        assert_eq!(to_nps_status(DEPTH_EXCEEDED), "NPS-CLIENT-BAD-PARAM");
        assert_eq!(to_nps_status(RATE_LIMIT_EXCEEDED), "NPS-LIMIT-RATE");
    }

    #[test]
    fn manifest_and_topology_errors() {
        assert_eq!(
            to_nps_status(MANIFEST_VERSION_UNSUPPORTED),
            "NPS-CLIENT-BAD-PARAM"
        );
        assert_eq!(
            to_nps_status(MANIFEST_NODE_TYPE_REMOVED),
            "NPS-CLIENT-BAD-FRAME"
        );
        assert_eq!(
            to_nps_status(MANIFEST_NODE_TYPE_UNKNOWN),
            "NPS-CLIENT-BAD-FRAME"
        );
        assert_eq!(to_nps_status(TOPOLOGY_UNAUTHORIZED), "NPS-AUTH-FORBIDDEN");
        assert_eq!(
            to_nps_status(TOPOLOGY_UNSUPPORTED_SCOPE),
            "NPS-CLIENT-BAD-PARAM"
        );
    }

    #[test]
    fn http_binding_errors() {
        assert_eq!(to_nps_status(HTTP_ORIGIN_FORBIDDEN), "NPS-AUTH-FORBIDDEN");
        assert_eq!(
            to_nps_status(HTTP_CONTENT_TYPE_UNSUPPORTED),
            "NPS-CLIENT-BAD-FRAME"
        );
        assert_eq!(
            to_nps_status(HTTP_ACCEPT_UNSATISFIABLE),
            "NPS-CLIENT-BAD-PARAM"
        );
        assert_eq!(
            to_nps_status(HTTP_REQUEST_ID_MISMATCH),
            "NPS-CLIENT-BAD-PARAM"
        );
        assert_eq!(
            to_nps_status(HTTP_FRAME_BODY_MALFORMED),
            "NPS-CLIENT-BAD-FRAME"
        );
        assert_eq!(
            to_nps_status(CAPABILITY_ADVERTISED_UNIMPLEMENTED),
            "NPS-SERVER-UNSUPPORTED"
        );
    }

    #[test]
    fn multi_anchor_ha_errors() {
        assert_eq!(ANCHOR_NOT_LEADER, "NWP-ANCHOR-NOT-LEADER");
        assert_eq!(ANCHOR_EPOCH_FENCED, "NWP-ANCHOR-EPOCH-FENCED");
        assert_eq!(to_nps_status(ANCHOR_NOT_LEADER), "NPS-CLIENT-CONFLICT");
        assert_eq!(to_nps_status(ANCHOR_EPOCH_FENCED), "NPS-CLIENT-CONFLICT");
    }

    #[test]
    fn bridge_errors() {
        assert_eq!(
            to_nps_status(BRIDGE_DIRECTION_UNSUPPORTED),
            "NPS-SERVER-UNSUPPORTED"
        );
        assert_eq!(
            to_nps_status(BRIDGE_TARGET_INVALID),
            "NPS-CLIENT-UNPROCESSABLE"
        );
        assert_eq!(
            to_nps_status(BRIDGE_PROTOCOL_UNSUPPORTED),
            "NPS-SERVER-UNSUPPORTED"
        );
        assert_eq!(
            to_nps_status(BRIDGE_ENDPOINT_INVALID),
            "NPS-CLIENT-UNPROCESSABLE"
        );
        assert_eq!(
            to_nps_status(BRIDGE_UPSTREAM_FAILED),
            "NPS-DOWNSTREAM-UNAVAILABLE"
        );
        assert_eq!(
            to_nps_status(BRIDGE_SERVER_TOOL_NOT_FOUND),
            "NPS-CLIENT-NOT-FOUND"
        );
        assert_eq!(
            to_nps_status(BRIDGE_SERVER_DISPATCHER_MISSING),
            "NPS-SERVER-INTERNAL"
        );
        assert_eq!(
            to_nps_status(BRIDGE_SERVER_DISPATCH_FAILED),
            "NPS-SERVER-INTERNAL"
        );
    }

    /// Statuses invented by the pre-CR-0010 implementations and removed by it.
    /// A port MUST NOT reintroduce them.
    #[test]
    fn removed_invented_statuses_are_never_produced() {
        let banned = [
            "NPS-SERVER-NOT-IMPLEMENTED",
            "NPS-SERVER-ERROR",
            "NPS-CLIENT-UNAUTHORIZED",
            "NPS-CLIENT-BAD-REQUEST",
            "NPS-SERVER-UPSTREAM-FAILED",
        ];
        for code in [
            BRIDGE_DIRECTION_UNSUPPORTED,
            BRIDGE_TARGET_INVALID,
            BRIDGE_PROTOCOL_UNSUPPORTED,
            BRIDGE_ENDPOINT_INVALID,
            BRIDGE_UPSTREAM_FAILED,
            BRIDGE_SERVER_TOOL_NOT_FOUND,
            BRIDGE_SERVER_DISPATCHER_MISSING,
            BRIDGE_SERVER_DISPATCH_FAILED,
        ] {
            assert!(!banned.contains(&to_nps_status(code)), "{code}");
        }
    }

    #[test]
    fn unknown_falls_back_to_internal() {
        assert_eq!(to_nps_status("NWP-BOGUS"), "NPS-SERVER-INTERNAL");
    }

    #[test]
    fn constants_have_correct_prefix() {
        let codes = [
            AUTH_NID_SCOPE_VIOLATION,
            AUTH_NID_EXPIRED,
            AUTH_NID_REVOKED,
            AUTH_NID_UNTRUSTED_ISSUER,
            AUTH_NID_CAPABILITY_MISSING,
            AUTH_ASSURANCE_TOO_LOW,
            AUTH_REPUTATION_BLOCKED,
            REPUTATION_THROTTLED,
            REPUTATION_REJECTED,
            REPUTATION_BANNED,
            QUERY_FILTER_INVALID,
            QUERY_FIELD_UNKNOWN,
            QUERY_CURSOR_INVALID,
            QUERY_REGEX_UNSAFE,
            QUERY_VECTOR_UNSUPPORTED,
            QUERY_AGGREGATE_UNSUPPORTED,
            QUERY_AGGREGATE_INVALID,
            QUERY_STREAM_UNSUPPORTED,
            ACTION_NOT_FOUND,
            ACTION_PARAMS_INVALID,
            ACTION_IDEMPOTENCY_CONFLICT,
            TASK_NOT_FOUND,
            TASK_ALREADY_CANCELLED,
            TASK_ALREADY_COMPLETED,
            TASK_ALREADY_FAILED,
            SUBSCRIBE_STREAM_NOT_FOUND,
            SUBSCRIBE_LIMIT_EXCEEDED,
            SUBSCRIBE_FILTER_UNSUPPORTED,
            SUBSCRIBE_INTERRUPTED,
            SUBSCRIBE_SEQ_TOO_OLD,
            BUDGET_EXCEEDED,
            CGN_LIMIT_EXCEEDED,
            DEPTH_EXCEEDED,
            GRAPH_CYCLE,
            NODE_UNAVAILABLE,
            MANIFEST_VERSION_UNSUPPORTED,
            MANIFEST_NODE_TYPE_REMOVED,
            MANIFEST_NODE_TYPE_UNKNOWN,
            RATE_LIMIT_EXCEEDED,
            RESERVED_TYPE_UNSUPPORTED,
            HTTP_ORIGIN_FORBIDDEN,
            HTTP_CONTENT_TYPE_UNSUPPORTED,
            HTTP_ACCEPT_UNSATISFIABLE,
            HTTP_REQUEST_ID_MISMATCH,
            HTTP_FRAME_BODY_MALFORMED,
            CAPABILITY_ADVERTISED_UNIMPLEMENTED,
            TOPOLOGY_UNAUTHORIZED,
            TOPOLOGY_UNSUPPORTED_SCOPE,
            TOPOLOGY_DEPTH_UNSUPPORTED,
            TOPOLOGY_FILTER_UNSUPPORTED,
            ANCHOR_NOT_LEADER,
            ANCHOR_EPOCH_FENCED,
            BRIDGE_DIRECTION_UNSUPPORTED,
            BRIDGE_TARGET_INVALID,
            BRIDGE_PROTOCOL_UNSUPPORTED,
            BRIDGE_ENDPOINT_INVALID,
            BRIDGE_UPSTREAM_FAILED,
            BRIDGE_SERVER_TOOL_NOT_FOUND,
            BRIDGE_SERVER_DISPATCHER_MISSING,
            BRIDGE_SERVER_DISPATCH_FAILED,
        ];
        for c in &codes {
            assert!(c.starts_with("NWP-"), "Expected NWP- prefix, got: {c}");
        }
    }
}
