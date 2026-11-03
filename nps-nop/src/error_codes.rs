// Copyright 2026 INNO LOTUS PTY LTD
// SPDX-License-Identifier: Apache-2.0

//! NOP error code wire constants — mirror of `spec/error-codes.md` NOP section.

// ── Task ──────────────────────────────────────────────────────────────────────
pub const TASK_NOT_FOUND: &str = "NOP-TASK-NOT-FOUND";
pub const TASK_TIMEOUT: &str = "NOP-TASK-TIMEOUT";
pub const TASK_DAG_INVALID: &str = "NOP-TASK-DAG-INVALID";
pub const TASK_DAG_CYCLE: &str = "NOP-TASK-DAG-CYCLE";
pub const TASK_DAG_TOO_LARGE: &str = "NOP-TASK-DAG-TOO-LARGE";
pub const TASK_ALREADY_COMPLETED: &str = "NOP-TASK-ALREADY-COMPLETED";
pub const TASK_CANCELLED: &str = "NOP-TASK-CANCELLED";

// ── Delegate ──────────────────────────────────────────────────────────────────
pub const DELEGATE_SCOPE_VIOLATION: &str = "NOP-DELEGATE-SCOPE-VIOLATION";
pub const DELEGATE_REJECTED: &str = "NOP-DELEGATE-REJECTED";
pub const DELEGATE_CHAIN_TOO_DEEP: &str = "NOP-DELEGATE-CHAIN-TOO-DEEP";
pub const DELEGATE_TIMEOUT: &str = "NOP-DELEGATE-TIMEOUT";

// ── Sync ──────────────────────────────────────────────────────────────────────
pub const SYNC_TIMEOUT: &str = "NOP-SYNC-TIMEOUT";
pub const SYNC_DEPENDENCY_FAILED: &str = "NOP-SYNC-DEPENDENCY-FAILED";

// ── Stream ────────────────────────────────────────────────────────────────────
pub const STREAM_SEQ_GAP: &str = "NOP-STREAM-SEQ-GAP";
pub const STREAM_NID_MISMATCH: &str = "NOP-STREAM-NID-MISMATCH";
pub const STREAM_NAK: &str = "NOP-STREAM-NAK";
pub const STREAM_NAK_UNRESOLVABLE: &str = "NOP-STREAM-NAK-UNRESOLVABLE";

// ── Result TTL (NOP v0.7) ─────────────────────────────────────────────────────
pub const TASK_RESULT_EXPIRED: &str = "NOP-TASK-RESULT-EXPIRED";

// ── Resource / condition / mapping ────────────────────────────────────────────
pub const RESOURCE_INSUFFICIENT: &str = "NOP-RESOURCE-INSUFFICIENT";
pub const CONDITION_EVAL_ERROR: &str = "NOP-CONDITION-EVAL-ERROR";
pub const INPUT_MAPPING_ERROR: &str = "NOP-INPUT-MAPPING-ERROR";
pub const COMPENSATION_FAILED: &str = "NOP-COMPENSATION-FAILED";
pub const COMPENSATION_PARTIAL_FAILED: &str = "NOP-COMPENSATION-PARTIAL-FAILED";
pub const COMPENSATION_NOT_SUPPORTED: &str = "NOP-COMPENSATION-NOT-SUPPORTED";

// ── Callback ──────────────────────────────────────────────────────────────────
pub const CALLBACK_HMAC_MISSING: &str = "NOP-CALLBACK-HMAC-MISSING";
pub const CALLBACK_INVALID: &str = "NOP-CALLBACK-INVALID";
pub const CALLBACK_HMAC_INVALID: &str = "NOP-CALLBACK-HMAC-INVALID";

// ── NPS-CR-0007 L3 runtime ────────────────────────────────────────────────────
pub const CLAIM_CONFLICT: &str = "NOP-CLAIM-CONFLICT";
pub const SPAWN_SPEC_INVALID: &str = "NOP-SPAWN-SPEC-INVALID";
pub const RUNTIME_IDLE_TIMEOUT: &str = "NOP-RUNTIME-IDLE-TIMEOUT";
pub const RUNTIME_MAX_RUNTIME: &str = "NOP-RUNTIME-MAX-RUNTIME";
pub const REPLAY_CONFLICT: &str = "NOP-REPLAY-CONFLICT";
pub const REPLAY_LIMIT: &str = "NOP-REPLAY-LIMIT";
pub const AGGREGATION_INVALID: &str = "NOP-AGGREGATION-INVALID";

// ── Mapping to NPS status ──────────────────────────────────────────────────────

/// Map a NOP error code to the corresponding NPS status code.
pub const EXECUTION_INDETERMINATE: &str = "NOP-EXECUTION-INDETERMINATE";

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
            if !code.starts_with("NOP-") {
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
        assert_eq!(checked, 1);
    }
}

pub fn to_nps_status(code: &str) -> &'static str {
    match code {
        EXECUTION_INDETERMINATE => "NPS-SERVER-INTERNAL",
        TASK_NOT_FOUND => "NPS-CLIENT-NOT-FOUND",
        TASK_TIMEOUT => "NPS-SERVER-TIMEOUT",
        TASK_DAG_INVALID => "NPS-CLIENT-BAD-FRAME",
        TASK_DAG_CYCLE => "NPS-CLIENT-BAD-FRAME",
        TASK_DAG_TOO_LARGE => "NPS-CLIENT-BAD-FRAME",
        TASK_ALREADY_COMPLETED => "NPS-CLIENT-CONFLICT",
        TASK_CANCELLED => "NPS-CLIENT-CONFLICT",

        DELEGATE_SCOPE_VIOLATION => "NPS-AUTH-FORBIDDEN",
        DELEGATE_REJECTED => "NPS-CLIENT-UNPROCESSABLE",
        DELEGATE_CHAIN_TOO_DEEP => "NPS-CLIENT-BAD-PARAM",
        DELEGATE_TIMEOUT => "NPS-SERVER-TIMEOUT",

        SYNC_TIMEOUT => "NPS-SERVER-TIMEOUT",
        SYNC_DEPENDENCY_FAILED => "NPS-CLIENT-UNPROCESSABLE",

        STREAM_SEQ_GAP => "NPS-STREAM-SEQ-GAP",
        STREAM_NID_MISMATCH => "NPS-AUTH-UNAUTHENTICATED",
        STREAM_NAK => "NPS-STREAM-SEQ-GAP",
        STREAM_NAK_UNRESOLVABLE => "NPS-STREAM-SEQ-GAP",

        TASK_RESULT_EXPIRED => "NPS-CLIENT-NOT-FOUND",

        RESOURCE_INSUFFICIENT => "NPS-SERVER-UNAVAILABLE",
        CONDITION_EVAL_ERROR => "NPS-CLIENT-BAD-PARAM",
        INPUT_MAPPING_ERROR => "NPS-CLIENT-UNPROCESSABLE",
        COMPENSATION_FAILED => "NPS-CLIENT-UNPROCESSABLE",
        COMPENSATION_PARTIAL_FAILED => "NPS-CLIENT-UNPROCESSABLE",
        COMPENSATION_NOT_SUPPORTED => "NPS-CLIENT-UNPROCESSABLE",

        CALLBACK_HMAC_MISSING => "NPS-AUTH-UNAUTHENTICATED",
        CALLBACK_INVALID => "NPS-CLIENT-BAD-PARAM",
        CALLBACK_HMAC_INVALID => "NPS-AUTH-UNAUTHENTICATED",
        CLAIM_CONFLICT => "NPS-CLIENT-CONFLICT",
        SPAWN_SPEC_INVALID => "NPS-CLIENT-BAD-PARAM",
        RUNTIME_IDLE_TIMEOUT | RUNTIME_MAX_RUNTIME => "NPS-SERVER-TIMEOUT",
        REPLAY_CONFLICT => "NPS-CLIENT-CONFLICT",
        REPLAY_LIMIT => "NPS-LIMIT-RESOURCE",
        AGGREGATION_INVALID => "NPS-CLIENT-BAD-PARAM",

        _ => "NPS-SERVER-INTERNAL",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_errors() {
        assert_eq!(to_nps_status(TASK_NOT_FOUND), "NPS-CLIENT-NOT-FOUND");
        assert_eq!(to_nps_status(TASK_TIMEOUT), "NPS-SERVER-TIMEOUT");
        assert_eq!(to_nps_status(TASK_DAG_INVALID), "NPS-CLIENT-BAD-FRAME");
        assert_eq!(to_nps_status(TASK_DAG_CYCLE), "NPS-CLIENT-BAD-FRAME");
        assert_eq!(to_nps_status(TASK_DAG_TOO_LARGE), "NPS-CLIENT-BAD-FRAME");
        assert_eq!(to_nps_status(TASK_ALREADY_COMPLETED), "NPS-CLIENT-CONFLICT");
        assert_eq!(to_nps_status(TASK_CANCELLED), "NPS-CLIENT-CONFLICT");
    }

    #[test]
    fn delegate_errors() {
        assert_eq!(
            to_nps_status(DELEGATE_SCOPE_VIOLATION),
            "NPS-AUTH-FORBIDDEN"
        );
        assert_eq!(to_nps_status(DELEGATE_REJECTED), "NPS-CLIENT-UNPROCESSABLE");
        assert_eq!(
            to_nps_status(DELEGATE_CHAIN_TOO_DEEP),
            "NPS-CLIENT-BAD-PARAM"
        );
        assert_eq!(to_nps_status(DELEGATE_TIMEOUT), "NPS-SERVER-TIMEOUT");
    }

    #[test]
    fn sync_errors() {
        assert_eq!(to_nps_status(SYNC_TIMEOUT), "NPS-SERVER-TIMEOUT");
        assert_eq!(
            to_nps_status(SYNC_DEPENDENCY_FAILED),
            "NPS-CLIENT-UNPROCESSABLE"
        );
    }

    #[test]
    fn stream_errors() {
        assert_eq!(to_nps_status(STREAM_SEQ_GAP), "NPS-STREAM-SEQ-GAP");
        assert_eq!(
            to_nps_status(STREAM_NID_MISMATCH),
            "NPS-AUTH-UNAUTHENTICATED"
        );
        assert_eq!(to_nps_status(STREAM_NAK), "NPS-STREAM-SEQ-GAP");
    }

    #[test]
    fn resource_and_condition_errors() {
        assert_eq!(
            to_nps_status(RESOURCE_INSUFFICIENT),
            "NPS-SERVER-UNAVAILABLE"
        );
        assert_eq!(to_nps_status(CONDITION_EVAL_ERROR), "NPS-CLIENT-BAD-PARAM");
        assert_eq!(
            to_nps_status(INPUT_MAPPING_ERROR),
            "NPS-CLIENT-UNPROCESSABLE"
        );
        assert_eq!(
            to_nps_status(COMPENSATION_FAILED),
            "NPS-CLIENT-UNPROCESSABLE"
        );
        assert_eq!(
            to_nps_status(COMPENSATION_PARTIAL_FAILED),
            "NPS-CLIENT-UNPROCESSABLE"
        );
        assert_eq!(
            to_nps_status(COMPENSATION_NOT_SUPPORTED),
            "NPS-CLIENT-UNPROCESSABLE"
        );
    }

    #[test]
    fn callback_hmac_missing_maps_to_unauthenticated() {
        assert_eq!(
            to_nps_status(CALLBACK_HMAC_MISSING),
            "NPS-AUTH-UNAUTHENTICATED"
        );
    }

    #[test]
    fn unknown_falls_back_to_internal() {
        assert_eq!(to_nps_status("NOP-BOGUS"), "NPS-SERVER-INTERNAL");
    }

    #[test]
    fn constants_have_correct_prefix() {
        let codes = [
            TASK_NOT_FOUND,
            TASK_TIMEOUT,
            TASK_DAG_INVALID,
            TASK_DAG_CYCLE,
            TASK_DAG_TOO_LARGE,
            TASK_ALREADY_COMPLETED,
            TASK_CANCELLED,
            DELEGATE_SCOPE_VIOLATION,
            DELEGATE_REJECTED,
            DELEGATE_CHAIN_TOO_DEEP,
            DELEGATE_TIMEOUT,
            SYNC_TIMEOUT,
            SYNC_DEPENDENCY_FAILED,
            STREAM_SEQ_GAP,
            STREAM_NID_MISMATCH,
            STREAM_NAK,
            STREAM_NAK_UNRESOLVABLE,
            TASK_RESULT_EXPIRED,
            RESOURCE_INSUFFICIENT,
            CONDITION_EVAL_ERROR,
            INPUT_MAPPING_ERROR,
            COMPENSATION_FAILED,
            COMPENSATION_PARTIAL_FAILED,
            COMPENSATION_NOT_SUPPORTED,
            CALLBACK_HMAC_MISSING,
        ];
        for c in &codes {
            assert!(c.starts_with("NOP-"), "Expected NOP- prefix, got: {c}");
        }
    }
}
