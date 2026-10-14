English | [中文版](./overview.cn.md)

# NPS Rust SDK — API Reference

> Async Rust SDK for the Neural Protocol Suite — Rust stable (1.75+), Tokio-based.

This directory is the class-and-method reference for `nps-sdk`. For a
narrative quick-start and end-to-end examples see
[`sdk-usage.md`](./sdk-usage.md) (English) or
[`sdk-usage.cn.md`](./sdk-usage.cn.md) (中文). For the bundled CA server
see [`ca-server.md`](./ca-server.md).

---

## Workspace crates

| Crate | Purpose | Reference |
|-------|---------|-----------|
| `nps-core` | Frame header, codec (Tier-1 JSON / Tier-2 MsgPack), frame registry, anchor cache, errors | [`nps-rust.core.md`](./nps-rust.core.md) |
| `nps-ncp`  | NCP frames — `AnchorFrame`, `DiffFrame`, `StreamFrame`, `CapsFrame`, `ErrorFrame` | [`nps-rust.ncp.md`](./nps-rust.ncp.md) |
| `nps-nwp`  | NWP frames + async `NwpClient` (reqwest) | [`nps-rust.nwp.md`](./nps-rust.nwp.md) |
| `nps-nip`  | NIP frames + `NipIdentity` (Ed25519, AES-256-GCM key store) | [`nps-rust.nip.md`](./nps-rust.nip.md) |
| `nps-ndp`  | NDP frames, `InMemoryNdpRegistry`, `NdpAnnounceValidator` | [`nps-rust.ndp.md`](./nps-rust.ndp.md) |
| `nps-nop`  | NOP frames, `BackoffStrategy`, `NopTaskStatus`, async `NopClient` | [`nps-rust.nop.md`](./nps-rust.nop.md) |
| `nps-sdk`  | Meta-crate — re-exports everything under `nps_sdk::{core, ncp, nwp, nip, ndp, nop}` | (façade — re-exports only) |

---

## Install

`Cargo.toml`:

```toml
[dependencies]
nps-sdk = "1.0.0-alpha.13"                             # full façade
# — or pick individual crates:
nps-core = "1.0.0-alpha.13"
nps-ncp  = "1.0.0-alpha.13"
nps-nwp  = "1.0.0-alpha.13"
nps-nip  = "1.0.0-alpha.13"
nps-ndp  = "1.0.0-alpha.13"
nps-nop  = "1.0.0-alpha.13"
tokio    = { version = "1", features = ["full"] }     # required for nwp/nop async clients
```

`nps-sdk` re-exports the protocol crates behind feature flags (`nwp`,
`nip`, `ndp`, `nop`); `core` + `ncp` are always re-exported.

---

## Minimal encode / decode

```rust
use nps_core::{FrameRegistry, NpsFrameCodec};
use nps_core::frames::{EncodingTier, FrameType};
use nps_ncp::AnchorFrame;

let codec = NpsFrameCodec::new(FrameRegistry::create_full());

let mut schema = serde_json::Map::new();
schema.insert("fields".into(), serde_json::json!([
    { "name": "id",    "type": "uint64"  },
    { "name": "price", "type": "decimal" },
]));
let frame = AnchorFrame {
    anchor_id:   "sha256:abc123".into(),
    schema,
    namespace:   None,
    description: None,
    node_type:   None,
    ttl:         3600,
};

let wire = codec.encode(
    AnchorFrame::frame_type(),
    &frame.to_dict(),
    EncodingTier::MsgPack,
    /* is_final = */ true,
)?;

let (ft, dict) = codec.decode(&wire)?;
assert_eq!(ft, FrameType::Anchor);
let back = AnchorFrame::from_dict(&dict)?;
```

The codec is dict-oriented: there is no typed `encode<T: NpsFrame>`
method. Each frame type exposes `frame_type()`, `to_dict()` and
`from_dict()` — call them explicitly around `codec.encode` /
`codec.decode`.

---

## Encoding tiers

| Tier | `EncodingTier` | Wire flag (bit 7) | Notes |
|------|----------------|-------------------|-------|
| Tier-1 JSON    | `EncodingTier::Json`    | `0` | UTF-8 JSON, debug / interop |
| Tier-2 MsgPack | `EncodingTier::MsgPack` | `1` | `rmp-serde` (`to_vec_named`), production default |

**Rust flag byte layout** (differs from the Java / Python / .NET SDKs):

| Bit | Mask   | Meaning |
|-----|--------|---------|
| 7   | `0x80` | TIER — `1` = MsgPack, `0` = JSON |
| 6   | `0x40` | FINAL — last frame in a stream |
| 0   | `0x01` | EXT — 8-byte extended header (payload > 65 535 bytes) |

Header sizes: 4 bytes default, 8 bytes when `EXT = 1`
(`[type][flags][0][0][len_b3..len_b0]`). Max payload defaults to 10 MiB
(`nps_core::codec::DEFAULT_MAX_PAYLOAD`) — raise with
`NpsFrameCodec::new(r).with_max_payload(n)`.

---

## Async I/O

- `NwpClient` (`nps-nwp`) and `NopClient` (`nps-nop`) are `async` and
  require a Tokio runtime.
- All fallible operations return `NpsResult<T>` = `Result<T, NpsError>`.
- Non-2xx HTTP responses surface as `NpsError::Io("NWP /{path} failed: HTTP …")`
  — not panics.

---

## Error type

`NpsError` (from `nps-core`):

| Variant | Raised by |
|---------|-----------|
| `Frame(String)`          | Unknown frame type / missing fields / type mismatches |
| `Codec(String)`          | JSON or MsgPack encode / decode failure, payload oversized |
| `AnchorNotFound(String)` | `AnchorFrameCache::get_required` on missing / expired anchor |
| `AnchorPoison(String)`   | `AnchorFrameCache::set` with schema mismatch for same `anchor_id` |
| `Identity(String)`       | Key gen / save / load / PBKDF2 / AES-GCM failure |
| `Io(String)`             | `reqwest` network error, non-2xx HTTP, file I/O |

All variants implement `Display` and `std::error::Error`.

---

## Spec links

- [NPS-0 Overview](https://github.com/labacacia/NPS/blob/main/spec/NPS-0-Overview.md)
- [NPS-1 NCP](https://github.com/labacacia/NPS/blob/main/spec/NPS-1-NCP.md)
- [NPS-2 NWP](https://github.com/labacacia/NPS/blob/main/spec/NPS-2-NWP.md)
- [NPS-3 NIP](https://github.com/labacacia/NPS/blob/main/spec/NPS-3-NIP.md)
- [NPS-4 NDP](https://github.com/labacacia/NPS/blob/main/spec/NPS-4-NDP.md)
- [NPS-5 NOP](https://github.com/labacacia/NPS/blob/main/spec/NPS-5-NOP.md)
- [Frame registry](https://github.com/labacacia/NPS/blob/main/spec/frame-registry.yaml)
- [Error codes](https://github.com/labacacia/NPS/blob/main/spec/error-codes.md)
