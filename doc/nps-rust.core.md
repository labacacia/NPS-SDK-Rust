English | [中文版](./nps-rust.core.cn.md)

# `nps-core` — Reference

> Spec: [NPS-1 NCP v0.7](https://github.com/labacacia/NPS/blob/main/spec/NPS-1-NCP.md)

Foundation crate. Defines the wire header, encoding tiers, a
registry-validated codec, the anchor-frame cache, and the `NpsError`
hierarchy.

---

## Table of contents

- [`FrameType`](#frametype)
- [`EncodingTier`](#encodingtier)
- [`FrameHeader`](#frameheader)
- [`FrameDict`](#framedict)
- [`NpsFrameCodec`](#npsframecodec)
- [`FrameRegistry`](#frameregistry)
- [`AnchorFrameCache`](#anchorframecache)
- [`NpsError` / `NpsResult`](#npserror--npsresult)

---

## `FrameType`

```rust
#[repr(u8)]
pub enum FrameType {
    Anchor      = 0x01,  Diff     = 0x02,  Stream   = 0x03,  Caps       = 0x04,
    Query       = 0x10,  Action   = 0x11,
    Ident       = 0x20,  Trust    = 0x21,  Revoke   = 0x22,
    Announce    = 0x30,  Resolve  = 0x31,  Graph    = 0x32,
    Task        = 0x40,  Delegate = 0x41,  Sync     = 0x42,  AlignStream = 0x43,
    Error       = 0xFE,
}

impl FrameType {
    pub fn from_u8(v: u8) -> NpsResult<Self>;   // Err(NpsError::Frame) on unknown
    pub fn as_u8(self) -> u8;
}
```

---

## `EncodingTier`

```rust
pub enum EncodingTier {
    Json    = 0,
    MsgPack = 1,
}
```

The value is the bit-7 state of the `flags` byte — `MsgPack = 1` sets
`0x80`, `Json = 0` leaves it clear.

---

## `FrameHeader`

Wire-format header.

```rust
pub struct FrameHeader {
    pub frame_type:     FrameType,
    pub flags:          u8,
    pub payload_length: u64,
    pub is_extended:    bool,
}

impl FrameHeader {
    pub fn new(frame_type: FrameType, tier: EncodingTier,
               is_final: bool, payload_length: u64) -> Self;

    pub fn encoding_tier(&self) -> EncodingTier;   // bit 7
    pub fn is_final(&self)      -> bool;           // bit 6
    pub fn header_size(&self)   -> usize;          // 4 or 8

    pub fn parse(wire: &[u8])   -> NpsResult<Self>;
    pub fn to_bytes(&self)      -> Vec<u8>;
}
```

### Flags byte

| Bit | Mask   | Meaning |
|-----|--------|---------|
| 7   | `0x80` | TIER — `1` = MsgPack, `0` = JSON |
| 6   | `0x40` | FINAL — last frame in a stream |
| 0   | `0x01` | EXT — 8-byte extended header |

### Wire layout

```
Default (EXT=0, 4 bytes):
  [frame_type][flags][len_hi][len_lo]         — u16 big-endian length

Extended (EXT=1, 8 bytes):
  [frame_type][flags][0][0][len_b3..len_b0]   — u32 big-endian length
```

`FrameHeader::new` auto-enables EXT when `payload_length > 0xFFFF`.

---

## `FrameDict`

```rust
pub type FrameDict = serde_json::Map<String, Value>;
```

All frames round-trip through `FrameDict`. Helpers:

```rust
pub fn encode_json   (dict: &FrameDict) -> NpsResult<Vec<u8>>;
pub fn encode_msgpack(dict: &FrameDict) -> NpsResult<Vec<u8>>;   // rmp_serde::to_vec_named
pub fn decode_json   (payload: &[u8])   -> NpsResult<FrameDict>;
pub fn decode_msgpack(payload: &[u8])   -> NpsResult<FrameDict>;
```

MsgPack encoding uses the **named-field** form (map keys stay as
strings) — the wire is interoperable with the JSON tier, just smaller.

---

## `NpsFrameCodec`

Registry-validated, tier-switchable codec.

```rust
pub const DEFAULT_MAX_PAYLOAD: u64 = 10 * 1024 * 1024;   // 10 MiB

pub struct NpsFrameCodec { /* … */ }

impl NpsFrameCodec {
    pub fn new(registry: FrameRegistry) -> Self;
    pub fn with_max_payload(self, max_payload: u64) -> Self;   // builder

    pub fn encode(
        &self,
        frame_type: FrameType,
        dict:       &FrameDict,
        tier:       EncodingTier,
        is_final:   bool,
    ) -> NpsResult<Vec<u8>>;

    pub fn decode(&self, wire: &[u8])      -> NpsResult<(FrameType, FrameDict)>;
    pub fn peek_header(wire: &[u8])        -> NpsResult<FrameHeader>;
}
```

- `encode` fails with `NpsError::Codec` if the serialized payload
  exceeds `max_payload`.
- `decode` fails with `NpsError::Frame` if the header's frame type is
  not registered against this codec's `FrameRegistry`.
- `peek_header` is an associated function (no `&self`) — useful when
  streaming to know the length before allocating the full frame.

---

## `FrameRegistry`

```rust
pub struct FrameRegistry { /* … */ }

impl FrameRegistry {
    pub fn new()           -> Self;             // empty
    pub fn register(&mut self, ft: FrameType);
    pub fn is_registered(&self, ft: FrameType) -> bool;

    pub fn create_default() -> Self;            // NCP only (Anchor/Diff/Stream/Caps/Error)
    pub fn create_full()    -> Self;            // NCP + NWP + NIP + NDP + NOP
}
```

`FrameRegistry::default()` returns an **empty** registry — if you use
`FrameRegistry::default()` with a codec, every `decode` will fail with
`"unregistered frame type …"`. Prefer `create_default()` or
`create_full()`.

---

## `AnchorFrameCache`

Thread-safe-by-construction (not `Sync`: use `Arc<Mutex<_>>` for
shared mutation) anchor-schema cache with lazy TTL expiry.

```rust
pub struct AnchorFrameCache {
    pub clock: Box<dyn Fn() -> Instant + Send + Sync>,   // swap in tests
    // …
}

impl AnchorFrameCache {
    pub fn new() -> Self;

    /// SHA-256 of canonical (sorted-key) JSON of `schema`, prefixed `sha256:`.
    pub fn compute_anchor_id(schema: &Map<String, Value>) -> String;

    pub fn set(&mut self, schema: Map<String, Value>, ttl_secs: u64)
                 -> NpsResult<String>;                  // → anchor_id
    pub fn get(&self, anchor_id: &str)          -> Option<&Map<String, Value>>;
    pub fn get_required(&self, anchor_id: &str) -> NpsResult<&Map<String, Value>>;

    pub fn invalidate(&mut self, anchor_id: &str);
    pub fn evict_expired(&mut self);

    pub fn len(&self)      -> usize;           // live entries only
    pub fn is_empty(&self) -> bool;
}
```

### Poisoning

`set` returns `NpsError::AnchorPoison` when the same `anchor_id` is
already cached with a **different** schema (and still live). Re-inserts
with an identical schema refresh the TTL.

### Lazy expiry

`get` / `get_required` / `len` / `is_empty` filter by `expires > now`
without mutating the store. Call `evict_expired()` to actually free
memory.

### Injectable clock

```rust
use std::time::{Duration, Instant};

let start = Instant::now();
let mut cache = AnchorFrameCache::new();
cache.clock = Box::new(move || start + Duration::from_secs(100_000));
```

---

## `NpsError` / `NpsResult`

```rust
pub enum NpsError {
    Frame(String),
    Codec(String),
    AnchorNotFound(String),
    AnchorPoison(String),
    Identity(String),
    Io(String),
}

pub type NpsResult<T> = Result<T, NpsError>;
```

`NpsError: Clone + Debug + Display + std::error::Error`.
