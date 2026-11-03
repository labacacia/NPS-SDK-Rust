English | [中文版](./nps-rust.ncp.cn.md)

# `nps-ncp` — Reference

> Spec: [NPS-1 NCP v0.7](https://github.com/labacacia/NPS/blob/main/spec/NPS-1-NCP.md)

The five NCP frame types. Every struct exposes the same trio:

```rust
pub fn frame_type() -> FrameType;
pub fn to_dict(&self) -> FrameDict;
pub fn from_dict(d: &FrameDict) -> NpsResult<Self>;
```

> **Note — Rust frame shapes.** The Rust NCP structs carry a slightly
> different field set from the Java / Python / .NET / TS SDKs: the
> Rust layouts below are authoritative for this crate.

---

## Table of contents

- [`AnchorFrame` (0x01)](#anchorframe-0x01)
- [`DiffFrame` (0x02)](#diffframe-0x02)
- [`StreamFrame` (0x03)](#streamframe-0x03)
- [`CapsFrame` (0x04)](#capsframe-0x04)
- [`ErrorFrame` (0xFE)](#errorframe-0xfe)

---

## `AnchorFrame` (0x01)

Publishes a schema anchor + TTL.

```rust
pub struct AnchorFrame {
    pub anchor_id:   String,
    pub schema:      serde_json::Map<String, Value>,
    pub namespace:   Option<String>,
    pub description: Option<String>,
    pub node_type:   Option<String>,     // e.g. "memory" | "action" | …
    pub ttl:         u64,                // seconds; `from_dict` defaults to 3600
}
```

`schema` is stored as a free-form map — typically
`{ "fields": [ { "name": …, "type": … }, … ] }` but any shape that your
nodes and clients agree on is valid. `from_dict` falls back to `ttl =
3600` when the field is missing.

To produce the content-addressed `anchor_id` deterministically use
[`AnchorFrameCache::compute_anchor_id`](./nps-rust.core.md#anchorframecache).

---

## `DiffFrame` (0x02)

Schema evolution between two anchors.

```rust
pub struct DiffFrame {
    pub anchor_id:     String,      // old anchor
    pub new_anchor_id: String,      // new anchor
    pub patch:         Vec<Value>,  // JSON-Patch-shaped ops (free-form)
}
```

`patch` is serialized verbatim — this crate does not validate the ops;
the receiver is expected to know the patch dialect (NPS-1 §5.2 uses
RFC 6902-compatible shape).

---

## `StreamFrame` (0x03)

One chunk of a streamed response. Multiple `StreamFrame`s tile out a
result; the final chunk sets `is_last = true`.

```rust
pub struct StreamFrame {
    pub anchor_id: String,
    pub seq:       u64,
    pub payload:   Value,     // opaque — any JSON-representable value
    pub is_last:   bool,
}
```

The wire-level `FINAL` flag (bit 6 of the header) is **separate** from
`is_last`. `is_last` is an in-payload business marker used by
[`NwpClient::stream`](./nps-rust.nwp.md#nwpclient) to stop iterating.

---

## `CapsFrame` (0x04)

Node capability / response-envelope frame.

```rust
pub struct CapsFrame {
    pub node_id:    String,
    pub caps:       Vec<String>,         // capability URIs
    pub anchor_ref: Option<String>,      // anchor being answered against
    pub payload:    Option<Value>,       // opaque response data
}
```

In the Rust SDK `CapsFrame` is the **default response envelope** for
NWP: `NwpClient::query` returns a `CapsFrame` directly (it reads
`anchor_ref` + `payload`). Caps-advertisement usage and response usage
share the same struct — differentiate by inspecting `caps` vs
`payload`.

---

## `ErrorFrame` (0xFE)

Unified protocol-level error.

```rust
pub struct ErrorFrame {
    pub error_code: String,          // "NWP-QUERY-ANCHOR-UNKNOWN", …
    pub message:    String,
    pub detail:     Option<Value>,   // free-form extra context
}
```

See [`error-codes.md`](https://github.com/labacacia/NPS/blob/main/spec/error-codes.md)
for the namespace.

---

## End-to-end

```rust
use nps_core::{FrameRegistry, NpsFrameCodec};
use nps_core::cache::AnchorFrameCache;
use nps_core::frames::{EncodingTier, FrameType};
use nps_ncp::AnchorFrame;

let codec = NpsFrameCodec::new(FrameRegistry::create_default());

let mut schema = serde_json::Map::new();
schema.insert("fields".into(), serde_json::json!([
    { "name": "id", "type": "uint64" }
]));

let anchor_id = AnchorFrameCache::compute_anchor_id(&schema);
let frame = AnchorFrame {
    anchor_id, schema,
    namespace: Some("example.products".into()),
    description: Some("product catalog v1".into()),
    node_type: Some("memory".into()),
    ttl: 3600,
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
assert_eq!(back.ttl, 3600);
```
