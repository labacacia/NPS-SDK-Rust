// Copyright 2026 INNO LOTUS PTY LTD
// SPDX-License-Identifier: Apache-2.0

//! NCP error code wire constants — mirror of `spec/error-codes.md` NCP section.

// ── Anchor ────────────────────────────────────────────────────────────────────
pub const ANCHOR_NOT_FOUND: &str = "NCP-ANCHOR-NOT-FOUND";
pub const ANCHOR_SCHEMA_INVALID: &str = "NCP-ANCHOR-SCHEMA-INVALID";
pub const ANCHOR_ID_MISMATCH: &str = "NCP-ANCHOR-ID-MISMATCH";
pub const ANCHOR_STALE: &str = "NCP-ANCHOR-STALE";

// ── Frame ─────────────────────────────────────────────────────────────────────
pub const FRAME_UNKNOWN_TYPE: &str = "NCP-FRAME-UNKNOWN-TYPE";
pub const FRAME_PAYLOAD_TOO_LARGE: &str = "NCP-FRAME-PAYLOAD-TOO-LARGE";
pub const FRAME_FLAGS_INVALID: &str = "NCP-FRAME-FLAGS-INVALID";
pub const DIFF_FORMAT_UNSUPPORTED: &str = "NCP-DIFF-FORMAT-UNSUPPORTED";

// ── Stream ────────────────────────────────────────────────────────────────────
pub const STREAM_SEQ_GAP: &str = "NCP-STREAM-SEQ-GAP";
pub const STREAM_NOT_FOUND: &str = "NCP-STREAM-NOT-FOUND";
pub const STREAM_LIMIT_EXCEEDED: &str = "NCP-STREAM-LIMIT-EXCEEDED";
pub const STREAM_WINDOW_OVERFLOW: &str = "NCP-STREAM-WINDOW-OVERFLOW";

// ── Encoding ──────────────────────────────────────────────────────────────────
pub const ENCODING_UNSUPPORTED: &str = "NCP-ENCODING-UNSUPPORTED";

// ── Encryption ────────────────────────────────────────────────────────────────
pub const ENC_NOT_NEGOTIATED: &str = "NCP-ENC-NOT-NEGOTIATED";
pub const ENC_AUTH_FAILED: &str = "NCP-ENC-AUTH-FAILED";

// ── Version / Preamble ────────────────────────────────────────────────────────
pub const VERSION_INCOMPATIBLE: &str = "NCP-VERSION-INCOMPATIBLE";
pub const PREAMBLE_INVALID: &str = "NCP-PREAMBLE-INVALID";

// ── Keepalive (v0.8) ──────────────────────────────────────────────────────────
pub const KEEPALIVE_TIMEOUT: &str = "NCP-KEEPALIVE-TIMEOUT";
pub const REKEY_REQUIRED: &str = "NCP-REKEY-REQUIRED";
pub const EARLY_DATA_REJECTED: &str = "NCP-EARLY-DATA-REJECTED";

// ── Native-mode TLS binding (NPS-RFC-0006 §6.3–§6.4) ─────────────────────────
/// The mTLS client-certificate NID does not match the session `IdentFrame` NID,
/// or a resumed TLS session's certificate NID differs from the ticket-bound NID.
///
/// Reused (not introduced) by NPS-CR-0009 as the native-path failover trigger:
/// reaching a superseded Anchor produces this, and the failover connector
/// re-resolves rather than giving up.
pub const NID_MISMATCH: &str = "NCP-NID-MISMATCH";

// ── Mapping to NPS status ──────────────────────────────────────────────────────

/// Map a NCP error code to the corresponding NPS status code.
///
/// Returns `"NPS-SERVER-INTERNAL"` for any unrecognised code so callers always
/// get a valid status string.
pub const FEATURE_NEGOTIATION_INVALID: &str = "NCP-FEATURE-NEGOTIATION-INVALID";
pub const FEATURE_REQUIRED_UNSUPPORTED: &str = "NCP-FEATURE-REQUIRED-UNSUPPORTED";

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
            if !code.starts_with("NCP-") {
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
        assert_eq!(checked, 2);
    }
}

pub fn to_nps_status(code: &str) -> &'static str {
    match code {
        FEATURE_NEGOTIATION_INVALID => "NPS-CLIENT-BAD-FRAME",
        FEATURE_REQUIRED_UNSUPPORTED => "NPS-CLIENT-CONFLICT",
        ANCHOR_NOT_FOUND => "NPS-CLIENT-NOT-FOUND",
        ANCHOR_SCHEMA_INVALID => "NPS-CLIENT-BAD-FRAME",
        ANCHOR_ID_MISMATCH => "NPS-CLIENT-CONFLICT",
        ANCHOR_STALE => "NPS-CLIENT-CONFLICT",

        FRAME_UNKNOWN_TYPE => "NPS-CLIENT-BAD-FRAME",
        FRAME_PAYLOAD_TOO_LARGE => "NPS-LIMIT-PAYLOAD",
        FRAME_FLAGS_INVALID => "NPS-CLIENT-BAD-FRAME",
        DIFF_FORMAT_UNSUPPORTED => "NPS-CLIENT-BAD-FRAME",

        STREAM_SEQ_GAP => "NPS-STREAM-SEQ-GAP",
        STREAM_NOT_FOUND => "NPS-STREAM-NOT-FOUND",
        STREAM_LIMIT_EXCEEDED => "NPS-STREAM-LIMIT",
        STREAM_WINDOW_OVERFLOW => "NPS-STREAM-LIMIT",

        ENCODING_UNSUPPORTED => "NPS-SERVER-ENCODING-UNSUPPORTED",

        ENC_NOT_NEGOTIATED => "NPS-CLIENT-BAD-FRAME",
        ENC_AUTH_FAILED => "NPS-CLIENT-BAD-FRAME",

        VERSION_INCOMPATIBLE => "NPS-PROTO-VERSION-INCOMPATIBLE",
        PREAMBLE_INVALID => "NPS-PROTO-PREAMBLE-INVALID",
        KEEPALIVE_TIMEOUT => "NPS-SERVER-TIMEOUT",
        REKEY_REQUIRED => "NPS-CLIENT-BAD-FRAME",
        EARLY_DATA_REJECTED => "NPS-PROTO-VERSION-INCOMPATIBLE",

        NID_MISMATCH => "NPS-AUTH-UNAUTHENTICATED",

        _ => "NPS-SERVER-INTERNAL",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchor_errors() {
        assert_eq!(to_nps_status(ANCHOR_NOT_FOUND), "NPS-CLIENT-NOT-FOUND");
        assert_eq!(to_nps_status(ANCHOR_SCHEMA_INVALID), "NPS-CLIENT-BAD-FRAME");
        assert_eq!(to_nps_status(ANCHOR_ID_MISMATCH), "NPS-CLIENT-CONFLICT");
        assert_eq!(to_nps_status(ANCHOR_STALE), "NPS-CLIENT-CONFLICT");
    }

    #[test]
    fn frame_errors() {
        assert_eq!(to_nps_status(FRAME_UNKNOWN_TYPE), "NPS-CLIENT-BAD-FRAME");
        assert_eq!(to_nps_status(FRAME_PAYLOAD_TOO_LARGE), "NPS-LIMIT-PAYLOAD");
        assert_eq!(to_nps_status(FRAME_FLAGS_INVALID), "NPS-CLIENT-BAD-FRAME");
        assert_eq!(
            to_nps_status(DIFF_FORMAT_UNSUPPORTED),
            "NPS-CLIENT-BAD-FRAME"
        );
    }

    #[test]
    fn stream_errors() {
        assert_eq!(to_nps_status(STREAM_SEQ_GAP), "NPS-STREAM-SEQ-GAP");
        assert_eq!(to_nps_status(STREAM_NOT_FOUND), "NPS-STREAM-NOT-FOUND");
        assert_eq!(to_nps_status(STREAM_LIMIT_EXCEEDED), "NPS-STREAM-LIMIT");
        assert_eq!(to_nps_status(STREAM_WINDOW_OVERFLOW), "NPS-STREAM-LIMIT");
    }

    #[test]
    fn encoding_and_enc_errors() {
        assert_eq!(
            to_nps_status(ENCODING_UNSUPPORTED),
            "NPS-SERVER-ENCODING-UNSUPPORTED"
        );
        assert_eq!(to_nps_status(ENC_NOT_NEGOTIATED), "NPS-CLIENT-BAD-FRAME");
        assert_eq!(to_nps_status(ENC_AUTH_FAILED), "NPS-CLIENT-BAD-FRAME");
    }

    #[test]
    fn version_and_preamble() {
        assert_eq!(
            to_nps_status(VERSION_INCOMPATIBLE),
            "NPS-PROTO-VERSION-INCOMPATIBLE"
        );
        assert_eq!(
            to_nps_status(PREAMBLE_INVALID),
            "NPS-PROTO-PREAMBLE-INVALID"
        );
    }

    #[test]
    fn unknown_falls_back_to_internal() {
        assert_eq!(to_nps_status("NCP-BOGUS"), "NPS-SERVER-INTERNAL");
    }

    #[test]
    fn nid_mismatch() {
        assert_eq!(NID_MISMATCH, "NCP-NID-MISMATCH");
        assert_eq!(to_nps_status(NID_MISMATCH), "NPS-AUTH-UNAUTHENTICATED");
    }

    #[test]
    fn constants_have_correct_prefix() {
        let codes = [
            ANCHOR_NOT_FOUND,
            ANCHOR_SCHEMA_INVALID,
            ANCHOR_ID_MISMATCH,
            ANCHOR_STALE,
            FRAME_UNKNOWN_TYPE,
            FRAME_PAYLOAD_TOO_LARGE,
            FRAME_FLAGS_INVALID,
            DIFF_FORMAT_UNSUPPORTED,
            STREAM_SEQ_GAP,
            STREAM_NOT_FOUND,
            STREAM_LIMIT_EXCEEDED,
            STREAM_WINDOW_OVERFLOW,
            ENCODING_UNSUPPORTED,
            ENC_NOT_NEGOTIATED,
            ENC_AUTH_FAILED,
            VERSION_INCOMPATIBLE,
            PREAMBLE_INVALID,
            KEEPALIVE_TIMEOUT,
            REKEY_REQUIRED,
            NID_MISMATCH,
        ];
        for c in &codes {
            assert!(c.starts_with("NCP-"), "Expected NCP- prefix, got: {c}");
        }
    }
}
