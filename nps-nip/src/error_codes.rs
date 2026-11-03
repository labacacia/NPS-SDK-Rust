// Copyright 2026 INNO LOTUS PTY LTD
// SPDX-License-Identifier: Apache-2.0

//! NIP error code wire constants — mirror of `spec/error-codes.md` NIP section.

// ── Cert verification (v1 + v2) ──────────────────────────────────────────────
pub const CERT_EXPIRED: &str = "NIP-CERT-EXPIRED";
pub const CERT_REVOKED: &str = "NIP-CERT-REVOKED";
pub const CERT_SIGNATURE_INVALID: &str = "NIP-CERT-SIGNATURE-INVALID";
pub const CERT_UNTRUSTED_ISSUER: &str = "NIP-CERT-UNTRUSTED-ISSUER";
pub const CERT_CAPABILITY_MISSING: &str = "NIP-CERT-CAPABILITY-MISSING";
pub const CERT_SCOPE_VIOLATION: &str = "NIP-CERT-SCOPE-VIOLATION";

// ── CA service ───────────────────────────────────────────────────────────────
pub const CA_NID_NOT_FOUND: &str = "NIP-CA-NID-NOT-FOUND";
pub const CA_NID_ALREADY_EXISTS: &str = "NIP-CA-NID-ALREADY-EXISTS";
pub const CA_SERIAL_DUPLICATE: &str = "NIP-CA-SERIAL-DUPLICATE";
pub const CA_RENEWAL_TOO_EARLY: &str = "NIP-CA-RENEWAL-TOO-EARLY";
pub const CA_SCOPE_EXPANSION_DENIED: &str = "NIP-CA-SCOPE-EXPANSION-DENIED";
pub const RA_TOKEN_INVALID: &str = "NIP-RA-TOKEN-INVALID";
pub const RA_TOKEN_EXPIRED: &str = "NIP-RA-TOKEN-EXPIRED";
pub const RA_NID_NOT_ALLOWED: &str = "NIP-RA-NID-NOT-ALLOWED";
pub const RA_PENDING_REJECTED: &str = "NIP-RA-PENDING-REJECTED";

pub const OCSP_UNAVAILABLE: &str = "NIP-OCSP-UNAVAILABLE";
pub const TRUST_FRAME_INVALID: &str = "NIP-TRUST-FRAME-INVALID";

// ── RFC-0003 (assurance level) ───────────────────────────────────────────────
pub const ASSURANCE_MISMATCH: &str = "NIP-ASSURANCE-MISMATCH";
pub const ASSURANCE_UNKNOWN: &str = "NIP-ASSURANCE-UNKNOWN";

// ── RFC-0004 (reputation log) ────────────────────────────────────────────────
pub const REPUTATION_ENTRY_INVALID: &str = "NIP-REPUTATION-ENTRY-INVALID";
pub const REPUTATION_LOG_UNREACHABLE: &str = "NIP-REPUTATION-LOG-UNREACHABLE";

// ── RFC-0002 (X.509 + ACME) ──────────────────────────────────────────────────
pub const CERT_FORMAT_INVALID: &str = "NIP-CERT-FORMAT-INVALID";
pub const CERT_EKU_MISSING: &str = "NIP-CERT-EKU-MISSING";
pub const CERT_SUBJECT_NID_MISMATCH: &str = "NIP-CERT-SUBJECT-NID-MISMATCH";
pub const ACME_CHALLENGE_FAILED: &str = "NIP-ACME-CHALLENGE-FAILED";

// ── TrustFrame (NPS-3 §5.2) ──────────────────────────────────────────────────
pub const TRUST_FRAME_EXPIRED: &str = "NIP-TRUST-FRAME-EXPIRED";
pub const TRUST_FRAME_GRANTOR_REVOKED: &str = "NIP-TRUST-FRAME-GRANTOR-REVOKED";
pub const TRUST_FRAME_SCOPE_EXCEEDS_GRANTOR: &str = "NIP-TRUST-FRAME-SCOPE-EXCEEDS-GRANTOR";
pub const TRUST_FRAME_NODES_PATTERN_INVALID: &str = "NIP-TRUST-FRAME-NODES-PATTERN-INVALID";

// ── RevokeFrame (NPS-3 §5.3) ─────────────────────────────────────────────────
pub const REVOKE_FRAME_INVALID: &str = "NIP-REVOKE-FRAME-INVALID";
pub const REVOKE_FRAME_UNAUTHORIZED_ISSUER: &str = "NIP-REVOKE-FRAME-UNAUTHORIZED-ISSUER";
pub const REVOKE_FRAME_SERIAL_MISMATCH: &str = "NIP-REVOKE-FRAME-SERIAL-MISMATCH";
pub const REVOKE_FRAME_REASON_UNKNOWN: &str = "NIP-REVOKE-FRAME-REASON-UNKNOWN";

// ── Reputation gossip (RFC-0004) ──────────────────────────────────────────────
pub const REPUTATION_GOSSIP_FORK: &str = "NIP-REPUTATION-GOSSIP-FORK";
pub const REPUTATION_GOSSIP_SIG_INVALID: &str = "NIP-REPUTATION-GOSSIP-SIG-INVALID";

// ── Group / session NIDs (NPS-CR-0003) ───────────────────────────────────────
pub const CA_GROUP_REVOKED: &str = "NIP-CA-GROUP-REVOKED";
pub const CA_PARENT_NOT_FOUND: &str = "NIP-CA-PARENT-NOT-FOUND";
pub const CA_PARENT_NOT_GROUP: &str = "NIP-CA-PARENT-NOT-GROUP";
pub const CA_SESSION_VALIDITY_INVALID: &str = "NIP-CA-SESSION-VALIDITY-INVALID";
pub const CA_JWS_INVALID: &str = "NIP-CA-JWS-INVALID";
pub const CA_JWS_EXPIRED: &str = "NIP-CA-JWS-EXPIRED";
pub const CERT_PARENT_REVOKED: &str = "NIP-CERT-PARENT-REVOKED";

// ── OCSP staple ───────────────────────────────────────────────────────────────
pub const OCSP_STAPLE_EXPIRED: &str = "NIP-OCSP-STAPLE-EXPIRED";
pub const OCSP_UNKNOWN: &str = "NIP-OCSP-UNKNOWN";
pub const REVOCATION_STATE_STALE: &str = "NIP-REVOCATION-STATE-STALE";

// ── NIP v0.10 — node_roles ────────────────────────────────────────────────────
pub const CERT_NODE_ROLES_MISMATCH: &str = "NIP-CERT-NODE-ROLES-MISMATCH";

// ── NIP v0.12 — Phase-3 enforcement ──────────────────────────────────────────
/// `IdentFrame.capabilities` claims a capability absent from the CA-attested
/// `id-nps-capabilities` extension.
///
/// Note the asymmetry with its sibling `NIP-CERT-NODE-ROLES-MISMATCH`: this one
/// maps to `NPS-AUTH-FORBIDDEN`, not `NPS-CLIENT-BAD-FRAME`.
pub const CERT_CAPABILITIES_EXCEEDED: &str = "NIP-CERT-CAPABILITIES-EXCEEDED";

// ── Mapping to NPS status ──────────────────────────────────────────────────────

/// Map a NIP error code to the corresponding NPS status code.
pub const CONTEXT_INVALID: &str = "NIP-CONTEXT-INVALID";
pub const CONTEXT_UNTRUSTED: &str = "NIP-CONTEXT-UNTRUSTED";
pub const CONTEXT_BINDING_MISMATCH: &str = "NIP-CONTEXT-BINDING-MISMATCH";
pub const CONTEXT_EXPIRED: &str = "NIP-CONTEXT-EXPIRED";
pub const CONTEXT_REPLAY: &str = "NIP-CONTEXT-REPLAY";
pub const CONTEXT_VERIFIER_UNAVAILABLE: &str = "NIP-CONTEXT-VERIFIER-UNAVAILABLE";

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
            if !code.starts_with("NIP-") {
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
        assert_eq!(checked, 6);
    }
}

pub fn to_nps_status(code: &str) -> &'static str {
    match code {
        CONTEXT_INVALID => "NPS-CLIENT-BAD-FRAME",
        CONTEXT_UNTRUSTED => "NPS-AUTH-FORBIDDEN",
        CONTEXT_BINDING_MISMATCH => "NPS-AUTH-FORBIDDEN",
        CONTEXT_EXPIRED => "NPS-AUTH-FORBIDDEN",
        CONTEXT_REPLAY => "NPS-CLIENT-CONFLICT",
        CONTEXT_VERIFIER_UNAVAILABLE => "NPS-SERVER-UNAVAILABLE",
        // Cert verification
        CERT_EXPIRED => "NPS-AUTH-UNAUTHENTICATED",
        CERT_REVOKED => "NPS-AUTH-UNAUTHENTICATED",
        CERT_SIGNATURE_INVALID => "NPS-AUTH-UNAUTHENTICATED",
        CERT_UNTRUSTED_ISSUER => "NPS-AUTH-UNAUTHENTICATED",
        CERT_CAPABILITY_MISSING => "NPS-AUTH-FORBIDDEN",
        CERT_SCOPE_VIOLATION => "NPS-AUTH-FORBIDDEN",

        // CA service
        CA_NID_NOT_FOUND => "NPS-CLIENT-NOT-FOUND",
        CA_NID_ALREADY_EXISTS => "NPS-CLIENT-CONFLICT",
        CA_SERIAL_DUPLICATE => "NPS-CLIENT-CONFLICT",
        CA_RENEWAL_TOO_EARLY => "NPS-CLIENT-BAD-PARAM",
        CA_SCOPE_EXPANSION_DENIED => "NPS-AUTH-FORBIDDEN",
        RA_TOKEN_INVALID => "NPS-AUTH-UNAUTHENTICATED",
        RA_TOKEN_EXPIRED => "NPS-AUTH-UNAUTHENTICATED",
        RA_NID_NOT_ALLOWED => "NPS-AUTH-FORBIDDEN",
        RA_PENDING_REJECTED => "NPS-AUTH-FORBIDDEN",

        OCSP_UNAVAILABLE => "NPS-SERVER-UNAVAILABLE",
        OCSP_STAPLE_EXPIRED => "NPS-AUTH-UNAUTHENTICATED",
        OCSP_UNKNOWN => "NPS-AUTH-UNAUTHENTICATED",
        REVOCATION_STATE_STALE => "NPS-SERVER-UNAVAILABLE",

        TRUST_FRAME_INVALID => "NPS-CLIENT-BAD-FRAME",
        TRUST_FRAME_EXPIRED => "NPS-AUTH-UNAUTHENTICATED",
        TRUST_FRAME_GRANTOR_REVOKED => "NPS-AUTH-UNAUTHENTICATED",
        TRUST_FRAME_SCOPE_EXCEEDS_GRANTOR => "NPS-AUTH-FORBIDDEN",
        TRUST_FRAME_NODES_PATTERN_INVALID => "NPS-CLIENT-BAD-FRAME",

        REVOKE_FRAME_INVALID => "NPS-CLIENT-BAD-FRAME",
        REVOKE_FRAME_UNAUTHORIZED_ISSUER => "NPS-AUTH-FORBIDDEN",
        REVOKE_FRAME_SERIAL_MISMATCH => "NPS-CLIENT-BAD-PARAM",
        REVOKE_FRAME_REASON_UNKNOWN => "NPS-CLIENT-BAD-FRAME",

        // RFC-0003 (assurance)
        ASSURANCE_MISMATCH => "NPS-CLIENT-BAD-FRAME",
        ASSURANCE_UNKNOWN => "NPS-CLIENT-BAD-FRAME",

        // RFC-0004 (reputation)
        REPUTATION_ENTRY_INVALID => "NPS-CLIENT-BAD-FRAME",
        REPUTATION_LOG_UNREACHABLE => "NPS-DOWNSTREAM-UNAVAILABLE",
        REPUTATION_GOSSIP_FORK => "NPS-SERVER-INTERNAL",
        REPUTATION_GOSSIP_SIG_INVALID => "NPS-CLIENT-BAD-FRAME",

        // RFC-0002 (X.509 + ACME)
        CERT_FORMAT_INVALID => "NPS-CLIENT-BAD-FRAME",
        CERT_EKU_MISSING => "NPS-CLIENT-BAD-FRAME",
        CERT_SUBJECT_NID_MISMATCH => "NPS-CLIENT-BAD-FRAME",
        ACME_CHALLENGE_FAILED => "NPS-CLIENT-BAD-FRAME",

        // NPS-CR-0003 (group / session NIDs)
        CA_GROUP_REVOKED => "NPS-AUTH-FORBIDDEN",
        CA_PARENT_NOT_FOUND => "NPS-CLIENT-NOT-FOUND",
        CA_PARENT_NOT_GROUP => "NPS-CLIENT-BAD-PARAM",
        CA_SESSION_VALIDITY_INVALID => "NPS-CLIENT-BAD-PARAM",
        CA_JWS_INVALID => "NPS-AUTH-UNAUTHENTICATED",
        CA_JWS_EXPIRED => "NPS-AUTH-UNAUTHENTICATED",
        CERT_PARENT_REVOKED => "NPS-AUTH-UNAUTHENTICATED",

        // NIP v0.10 (node_roles)
        CERT_NODE_ROLES_MISMATCH => "NPS-AUTH-FORBIDDEN",

        // NIP v0.12 (Phase-3 enforcement)
        CERT_CAPABILITIES_EXCEEDED => "NPS-AUTH-FORBIDDEN",

        _ => "NPS-SERVER-INTERNAL",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cert_verification_errors() {
        assert_eq!(to_nps_status(CERT_EXPIRED), "NPS-AUTH-UNAUTHENTICATED");
        assert_eq!(to_nps_status(CERT_REVOKED), "NPS-AUTH-UNAUTHENTICATED");
        assert_eq!(
            to_nps_status(CERT_SIGNATURE_INVALID),
            "NPS-AUTH-UNAUTHENTICATED"
        );
        assert_eq!(
            to_nps_status(CERT_UNTRUSTED_ISSUER),
            "NPS-AUTH-UNAUTHENTICATED"
        );
        assert_eq!(to_nps_status(CERT_CAPABILITY_MISSING), "NPS-AUTH-FORBIDDEN");
        assert_eq!(to_nps_status(CERT_SCOPE_VIOLATION), "NPS-AUTH-FORBIDDEN");
    }

    #[test]
    fn ca_service_errors() {
        assert_eq!(to_nps_status(CA_NID_NOT_FOUND), "NPS-CLIENT-NOT-FOUND");
        assert_eq!(to_nps_status(CA_NID_ALREADY_EXISTS), "NPS-CLIENT-CONFLICT");
        assert_eq!(to_nps_status(CA_SERIAL_DUPLICATE), "NPS-CLIENT-CONFLICT");
        assert_eq!(to_nps_status(CA_RENEWAL_TOO_EARLY), "NPS-CLIENT-BAD-PARAM");
        assert_eq!(
            to_nps_status(CA_SCOPE_EXPANSION_DENIED),
            "NPS-AUTH-FORBIDDEN"
        );
    }

    #[test]
    fn ocsp_errors() {
        assert_eq!(to_nps_status(OCSP_UNAVAILABLE), "NPS-SERVER-UNAVAILABLE");
        assert_eq!(
            to_nps_status(OCSP_STAPLE_EXPIRED),
            "NPS-AUTH-UNAUTHENTICATED"
        );
    }

    #[test]
    fn trust_frame_errors() {
        assert_eq!(to_nps_status(TRUST_FRAME_INVALID), "NPS-CLIENT-BAD-FRAME");
        assert_eq!(
            to_nps_status(TRUST_FRAME_EXPIRED),
            "NPS-AUTH-UNAUTHENTICATED"
        );
        assert_eq!(
            to_nps_status(TRUST_FRAME_GRANTOR_REVOKED),
            "NPS-AUTH-UNAUTHENTICATED"
        );
        assert_eq!(
            to_nps_status(TRUST_FRAME_SCOPE_EXCEEDS_GRANTOR),
            "NPS-AUTH-FORBIDDEN"
        );
        assert_eq!(
            to_nps_status(TRUST_FRAME_NODES_PATTERN_INVALID),
            "NPS-CLIENT-BAD-FRAME"
        );
    }

    #[test]
    fn revoke_frame_errors() {
        assert_eq!(to_nps_status(REVOKE_FRAME_INVALID), "NPS-CLIENT-BAD-FRAME");
        assert_eq!(
            to_nps_status(REVOKE_FRAME_UNAUTHORIZED_ISSUER),
            "NPS-AUTH-FORBIDDEN"
        );
        assert_eq!(
            to_nps_status(REVOKE_FRAME_SERIAL_MISMATCH),
            "NPS-CLIENT-BAD-PARAM"
        );
        assert_eq!(
            to_nps_status(REVOKE_FRAME_REASON_UNKNOWN),
            "NPS-CLIENT-BAD-FRAME"
        );
    }

    #[test]
    fn assurance_and_reputation_errors() {
        assert_eq!(to_nps_status(ASSURANCE_MISMATCH), "NPS-CLIENT-BAD-FRAME");
        assert_eq!(to_nps_status(ASSURANCE_UNKNOWN), "NPS-CLIENT-BAD-FRAME");
        assert_eq!(
            to_nps_status(REPUTATION_ENTRY_INVALID),
            "NPS-CLIENT-BAD-FRAME"
        );
        assert_eq!(
            to_nps_status(REPUTATION_LOG_UNREACHABLE),
            "NPS-DOWNSTREAM-UNAVAILABLE"
        );
        assert_eq!(to_nps_status(REPUTATION_GOSSIP_FORK), "NPS-SERVER-INTERNAL");
        assert_eq!(
            to_nps_status(REPUTATION_GOSSIP_SIG_INVALID),
            "NPS-CLIENT-BAD-FRAME"
        );
    }

    #[test]
    fn group_session_nid_errors() {
        assert_eq!(to_nps_status(CA_GROUP_REVOKED), "NPS-AUTH-FORBIDDEN");
        assert_eq!(to_nps_status(CA_PARENT_NOT_FOUND), "NPS-CLIENT-NOT-FOUND");
        assert_eq!(to_nps_status(CA_PARENT_NOT_GROUP), "NPS-CLIENT-BAD-PARAM");
        assert_eq!(
            to_nps_status(CA_SESSION_VALIDITY_INVALID),
            "NPS-CLIENT-BAD-PARAM"
        );
        assert_eq!(to_nps_status(CA_JWS_INVALID), "NPS-AUTH-UNAUTHENTICATED");
        assert_eq!(to_nps_status(CA_JWS_EXPIRED), "NPS-AUTH-UNAUTHENTICATED");
        assert_eq!(
            to_nps_status(CERT_PARENT_REVOKED),
            "NPS-AUTH-UNAUTHENTICATED"
        );
    }

    #[test]
    fn x509_acme_errors() {
        assert_eq!(to_nps_status(CERT_FORMAT_INVALID), "NPS-CLIENT-BAD-FRAME");
        assert_eq!(to_nps_status(CERT_EKU_MISSING), "NPS-CLIENT-BAD-FRAME");
        assert_eq!(
            to_nps_status(CERT_SUBJECT_NID_MISMATCH),
            "NPS-CLIENT-BAD-FRAME"
        );
        assert_eq!(to_nps_status(ACME_CHALLENGE_FAILED), "NPS-CLIENT-BAD-FRAME");
    }

    #[test]
    fn phase3_errors() {
        assert_eq!(CERT_CAPABILITIES_EXCEEDED, "NIP-CERT-CAPABILITIES-EXCEEDED");
        // Deliberately asymmetric with its node_roles sibling.
        assert_eq!(
            to_nps_status(CERT_CAPABILITIES_EXCEEDED),
            "NPS-AUTH-FORBIDDEN"
        );
        assert_eq!(
            to_nps_status(CERT_NODE_ROLES_MISMATCH),
            "NPS-AUTH-FORBIDDEN"
        );
    }

    #[test]
    fn unknown_falls_back_to_internal() {
        assert_eq!(to_nps_status("NIP-BOGUS"), "NPS-SERVER-INTERNAL");
    }

    #[test]
    fn constants_have_correct_prefix() {
        let codes = [
            CERT_EXPIRED,
            CERT_REVOKED,
            CERT_SIGNATURE_INVALID,
            CERT_UNTRUSTED_ISSUER,
            CERT_CAPABILITY_MISSING,
            CERT_SCOPE_VIOLATION,
            CA_NID_NOT_FOUND,
            CA_NID_ALREADY_EXISTS,
            CA_SERIAL_DUPLICATE,
            CA_RENEWAL_TOO_EARLY,
            CA_SCOPE_EXPANSION_DENIED,
            OCSP_UNAVAILABLE,
            OCSP_STAPLE_EXPIRED,
            TRUST_FRAME_INVALID,
            TRUST_FRAME_EXPIRED,
            TRUST_FRAME_GRANTOR_REVOKED,
            TRUST_FRAME_SCOPE_EXCEEDS_GRANTOR,
            TRUST_FRAME_NODES_PATTERN_INVALID,
            REVOKE_FRAME_INVALID,
            REVOKE_FRAME_UNAUTHORIZED_ISSUER,
            REVOKE_FRAME_SERIAL_MISMATCH,
            REVOKE_FRAME_REASON_UNKNOWN,
            ASSURANCE_MISMATCH,
            ASSURANCE_UNKNOWN,
            REPUTATION_ENTRY_INVALID,
            REPUTATION_LOG_UNREACHABLE,
            REPUTATION_GOSSIP_FORK,
            REPUTATION_GOSSIP_SIG_INVALID,
            CERT_FORMAT_INVALID,
            CERT_EKU_MISSING,
            CERT_SUBJECT_NID_MISMATCH,
            ACME_CHALLENGE_FAILED,
            CA_GROUP_REVOKED,
            CA_PARENT_NOT_FOUND,
            CA_PARENT_NOT_GROUP,
            CA_SESSION_VALIDITY_INVALID,
            CA_JWS_INVALID,
            CA_JWS_EXPIRED,
            CERT_PARENT_REVOKED,
            CERT_NODE_ROLES_MISMATCH,
            CERT_CAPABILITIES_EXCEEDED,
        ];
        for c in &codes {
            assert!(c.starts_with("NIP-"), "Expected NIP- prefix, got: {c}");
        }
    }
}
