English | [中文版](./README.cn.md)

# NPS Rust SDK (`nps-rs`)

> alpha.20 candidate, unpublished. Six-SDK helper contracts and qualified runtime
> paths differ; full production/cross-SDK runtime parity is not qualified.

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://github.com/labacacia/NPS/blob/main/LICENSE)
[![Release](https://img.shields.io/badge/candidate-v1.0.0--alpha.20-orange.svg)](CHANGELOG.md)
[![NCP](https://img.shields.io/badge/NCP-v0.11-5b8cff.svg)]()
[![NWP](https://img.shields.io/badge/NWP-v0.21-4af0b0.svg)]()
[![NIP](https://img.shields.io/badge/NIP-v0.14-7b61ff.svg)]()
[![NDP](https://img.shields.io/badge/NDP-v0.12-f0a050.svg)]()
[![NOP](https://img.shields.io/badge/NOP-v0.9-ff8c42.svg)]()

Rust client library for the **Neural Protocol Suite (NPS)** — a complete internet protocol stack designed for AI agents and models.

Crate group: `com.labacacia.nps` namespace | Rust edition 2021 | Cargo workspace

## Status

**v1.0.0-alpha.20 release candidate — unpublished, bounded experimental support**

Covers all five NPS protocols: NCP + NWP + NIP + NDP + NOP, plus **full NPS-RFC-0002 X.509 + ACME `agent-01` NID certificate primitives** (`nps_nip::x509` + `nps_nip::acme`).

The full workspace suite and the shared Alpha.18 conformance fixtures pass.

Alpha.15 additions: typed remote NIP CA client (`nps_nip::ca_client::NipCaClient`), native-mode NWP serving helper (`nps_nwp::NwpNativeNodeServer`), and TC-N1/TC-N2 conformance manifest helpers (`nps_conformance`, re-exported by `nps-sdk`).

## Alpha.18 portable profiles

- NCP 0.11 bounded native-server handshake and deterministic Caps negotiation.
- NWP 0.21 portable Node/Bridge serving and bridge lifecycle.
- NIP 0.14 portable CA, live revocation, signed CRL, and verification policy.
- NDP 0.12 signed Announce admission and registry conflict/liveness policy.
- NOP 0.9 deterministic orchestration, callback security, delegation, leases,
  and CR-0007 runtime decisions.

All five profiles consume the language-neutral fixtures under
[`spec/conformance`](https://github.com/labacacia/NPS/blob/prep/alpha20-release-candidate/spec/conformance/).

### Earlier additions (nps-nip)

- `nps_nip::x509` — `issue_leaf` / `issue_root` / `verify` (built on `rcgen` + `x509-parser` + `ed25519-dalek`).
- `nps_nip::acme` — `AcmeClient` (reqwest async) + in-process `AcmeServer` (tiny_http) + JWS / messages helpers (RFC 8555 + EdDSA per RFC 8037).
- `IdentFrame` extended with non-breaking `assurance_level` / `cert_format` / `cert_chain` fields; v1 verifiers ignore the new fields.

## Requirements

- Rust stable (1.70+)
- Cargo

## Building & Testing

```bash
# Run all tests
cargo test --workspace

# Build all crates
cargo build --workspace

# Build release
cargo build --workspace --release
```

## Workspace Crates

| Crate | Description |
|-------|-------------|
| `nps-core`  | Frame header, codec (Tier-1 JSON / Tier-2 MsgPack / Tier-3 BinaryVector), frame registry, anchor cache, error types |
| `nps-ncp`   | NCP frames: `AnchorFrame`, `DiffFrame`, `StreamFrame`, `CapsFrame`, `HelloFrame`, `ErrorFrame` |
| `nps-nwp`   | NWP frames: `QueryFrame`, `ActionFrame`, `AsyncActionResponse`; async `NwpClient` (reqwest); native serving via `NwpNativeNodeServer` |
| `nps-nip`   | NIP frames: `IdentFrame`, `TrustFrame`, `RevokeFrame`; `NipIdentity` (Ed25519 key management) |
| `nps-ndp`   | NDP frames: `AnnounceFrame`, `ResolveFrame`, `GraphFrame`; `InMemoryNdpRegistry`; `NdpAnnounceValidator` |
| `nps-nop`   | NOP frames: `TaskFrame`, `DelegateFrame`, `SyncFrame`, `AlignStreamFrame`; `BackoffStrategy`; `NopClient` |
| `nps-conformance` | TC-N1/TC-N2 conformance catalog, manifest builder, and validator |
| `nps-sdk`   | Re-export umbrella crate — all protocols under `nps_sdk::` namespace |

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
nps-sdk = { path = "impl/rust/nps-sdk" }
tokio   = { version = "1", features = ["rt-multi-thread", "macros"] }
```

### Encoding / Decoding NCP Frames

```rust
use nps_core::codec::NpsFrameCodec;
use nps_core::frames::EncodingTier;
use nps_core::registry::FrameRegistry;
use nps_ncp::AnchorFrame;
use serde_json::json;

let codec = NpsFrameCodec::new(FrameRegistry::create_full());

let mut schema = serde_json::Map::new();
schema.insert("fields".into(), json!([{"name": "id", "type": "uint64"}]));

let frame = AnchorFrame {
    anchor_id: "sha256:abc123".into(),
    schema,
    namespace:   None,
    description: None,
    node_type:   None,
    ttl:         3600,
};

let wire = codec.encode(AnchorFrame::frame_type(), &frame.to_dict(), EncodingTier::MsgPack, true)?;
let (frame_type, dict) = codec.decode(&wire)?;
let back = AnchorFrame::from_dict(&dict)?;
```

### NWP Client — Query

```rust
use nps_nwp::{NwpClient, QueryFrame};

let client = NwpClient::new("http://node.example.com:17433");
let query  = QueryFrame::new("sha256:abc123");
let caps   = client.query(&query).await?;
```

### NWP Client — Stream

```rust
let frames = client.stream(&query).await?;
for sf in &frames {
    println!("{:?}", sf.payload);
    if sf.is_last { break; }
}
```

### NWP Native Serving

```rust
use nps_ncp::CapsFrame;
use nps_nwp::NwpNativeNodeServer;
use serde_json::json;

let server = NwpNativeNodeServer::new()
    .with_query_handler(|_| Ok(CapsFrame::new("native:orders", vec![json!({ "id": 42 })])))
    .with_action_handler(|action| Ok(json!({ "action": action.action })));

// `stream` is already past NCP preamble, TLS, and Hello negotiation.
server.serve_stream(&mut stream).await?;
```

### NIP Identity — Sign & Verify

```rust
use nps_nip::identity::NipIdentity;
use std::path::Path;

// Generate keypair
let identity = NipIdentity::generate();
println!("{}", identity.pub_key_string()); // "ed25519:<hex>"

// Sign a payload
let mut payload = serde_json::Map::new();
payload.insert("nid".into(), serde_json::json!("urn:nps:node:example.com:data"));
let sig = identity.sign(&payload);  // "ed25519:<base64>"
let ok  = identity.verify(&payload, &sig); // true

// Persist and load (AES-256-GCM + PBKDF2)
identity.save(Path::new("my-node.key"), "my-passphrase")?;
let loaded = NipIdentity::load(Path::new("my-node.key"), "my-passphrase")?;
```

### NIP Remote CA Client

```rust
use nps_nip::ca_client::{NipCaClient, NipCaRegisterRequest};

let ca = NipCaClient::with_client("https://ca.example.com", "/nip", reqwest::Client::new());
let discovery = ca.get_discovery().await?;
let ident = ca.register_agent(
    &NipCaRegisterRequest {
        identifier: "agent-a".into(),
        pub_key: "ed25519:<pub>".into(),
        capabilities: vec!["nwp:query".into()],
        scope_json: None,
        metadata_json: None,
    },
    Some("token"),
).await?;
let status = ca.verify_agent(&ident.nid).await?;
```

### Conformance Manifest

```rust
use nps_conformance::{
    catalog_for_profile, validate_manifest, NpsConformanceCaseResult,
    NpsConformanceManifest, NODE_L1,
};

let results = catalog_for_profile(NODE_L1)?
    .iter()
    .map(|case| NpsConformanceCaseResult {
        id: case.id.to_string(),
        result: "pass".to_string(),
        message: None,
    })
    .collect();
let manifest = NpsConformanceManifest::create(
    NODE_L1,
    "my-node",
    "1.0.0-alpha.19",
    "urn:nps:node:example.com:my-node",
    "labacacia-fixture",
    "1.0.0-alpha.19",
    results,
    "ci",
);
let validation = validate_manifest(&manifest);
```

### NDP Registry — Announce & Resolve

```rust
use nps_ndp::{AnnounceFrame, InMemoryNdpRegistry};
use nps_nip::identity::NipIdentity;
use serde_json::json;

let identity = NipIdentity::generate();
let mut addrs = serde_json::Map::new();
addrs.insert("host".into(),     json!("example.com"));
addrs.insert("port".into(),     json!(17433));
addrs.insert("protocol".into(), json!("nwp"));

let tmp = AnnounceFrame {
    nid: "urn:nps:node:example.com:data".into(),
    addresses: vec![addrs.clone()],
    caps: vec!["nwp/query".into()],
    ttl: 300,
    timestamp: "2026-01-01T00:00:00Z".into(),
    signature: "placeholder".into(),
    node_type: None,
};
let sig   = identity.sign(&tmp.unsigned_dict());
let frame = AnnounceFrame { signature: sig, ..tmp };

let mut registry = InMemoryNdpRegistry::new();
registry.announce(frame);

let result = registry.resolve("nwp://example.com/data/sub").unwrap();
// result.host == "example.com", result.port == 17433
```

### NDP Announce Validator

```rust
use nps_ndp::NdpAnnounceValidator;

let mut validator = NdpAnnounceValidator::new();
validator.register_public_key("urn:nps:node:example.com:data", identity.pub_key_string());

let result = validator.validate(&frame);
if result.is_valid {
    println!("Announce accepted");
} else {
    println!("Rejected: {} — {}", result.error_code.unwrap(), result.message.unwrap());
}
```

### NOP — Backoff Strategy

```rust
use nps_nop::models::BackoffStrategy;

let delay_ms = BackoffStrategy::Exponential.compute_delay_ms(1000, 30_000, 2);
// Returns 4000 (2^2 * 1000), capped at max_ms
```

## Frame Type Reference

| Frame | Type Code | Protocol | Description |
|-------|-----------|----------|-------------|
| `AnchorFrame`      | 0x01 | NCP | Schema anchor (cached schema definition) |
| `DiffFrame`        | 0x02 | NCP | Schema diff / patch |
| `StreamFrame`      | 0x03 | NCP | Streaming data chunk (is_last = final) |
| `CapsFrame`        | 0x04 | NCP | Capability advertisement |
| `HelloFrame`       | 0x06 | NCP | Native-mode handshake (client → node, JSON) |
| `ErrorFrame`       | 0xFE | NCP | Unified error frame (all protocols) |
| `QueryFrame`       | 0x10 | NWP | Data query with anchor ref + filter |
| `ActionFrame`      | 0x11 | NWP | Action invocation (sync or async) |
| `IdentFrame`       | 0x20 | NIP | Node identity declaration (signed) |
| `TrustFrame`       | 0x21 | NIP | Trust delegation between nodes |
| `RevokeFrame`      | 0x22 | NIP | Revocation notice |
| `AnnounceFrame`    | 0x30 | NDP | Node announcement with TTL |
| `ResolveFrame`     | 0x31 | NDP | Address resolution request/response |
| `GraphFrame`       | 0x32 | NDP | Network topology snapshot |
| `TaskFrame`        | 0x40 | NOP | Orchestration DAG task |
| `DelegateFrame`    | 0x41 | NOP | Subtask delegation |
| `SyncFrame`        | 0x42 | NOP | K-of-N synchronization barrier |
| `AlignStreamFrame` | 0x43 | NOP | Streaming alignment update |

## Encoding

| Tier | Variant | Description |
|------|---------|-------------|
| Tier-1 | `EncodingTier::Json`    | Human-readable JSON (debug, interop) |
| Tier-2 | `EncodingTier::MsgPack` | MsgPack binary (default, ~60% smaller) |
| Tier-3 | `EncodingTier::BinaryVector` | `binary_vector.v1` for vector-heavy frames |

## Error Handling

All operations return `NpsResult<T>` = `Result<T, NpsError>`.

| Variant | When |
|---------|------|
| `NpsError::Frame(msg)` | Unknown frame type, invalid field |
| `NpsError::Codec(msg)` | Encode/decode failure, oversized payload |
| `NpsError::AnchorNotFound(id)` | `get_required()` for missing/expired anchor |
| `NpsError::AnchorPoison(id)` | Attempt to overwrite anchor with different schema |
| `NpsError::Identity(msg)` | Key generation, sign/verify, save/load failure |
| `NpsError::Io(msg)` | Network or filesystem error |

## Feature Flags (`nps-sdk`)

| Feature | Default | Description |
|---------|---------|-------------|
| `nwp`   | ✅ | Include NWP frames and client |
| `nip`   | ✅ | Include NIP frames and identity |
| `ndp`   | ✅ | Include NDP frames, registry, validator |
| `nop`   | ✅ | Include NOP frames and client |

## Testing

755 tests across the full workspace:

```bash
cargo test --workspace
```

## License

[Apache 2.0](https://github.com/labacacia/NPS-Dev/blob/main/LICENSE) © 2026 INNO LOTUS PTY LTD
