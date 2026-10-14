English | [中文版](./nps-rust.ndp.cn.md)

# `nps-ndp` — Reference

> Spec: [NPS-4 NDP v0.8](https://github.com/labacacia/NPS/blob/main/spec/NPS-4-NDP.md)

Discovery layer — the NPS analogue of DNS. Three frames, an in-memory
TTL registry, and a signature validator.

---

## Table of contents

- [`AnnounceFrame` (0x30)](#announceframe-0x30)
- [`ResolveFrame` (0x31)](#resolveframe-0x31)
- [`GraphFrame` (0x32)](#graphframe-0x32)
- [`InMemoryNdpRegistry`](#inmemoryndpregistry)
- [`ResolveResult`](#resolveresult)
- [`NdpAnnounceValidator`](#ndpannouncevalidator)
- [`NdpAnnounceResult`](#ndpannounceresult)

---

## `AnnounceFrame` (0x30)

Publishes a node's physical reachability and TTL (NPS-4 §3.1).

```rust
pub struct AnnounceFrame {
    pub nid:       String,
    pub addresses: Vec<serde_json::Map<String, Value>>,   // [{"host","port","protocol"}, …]
    pub caps:      Vec<String>,
    pub ttl:       u64,                                   // seconds; 0 = shutdown
    pub timestamp: String,                                // ISO 8601 UTC
    pub signature: String,                                // "ed25519:{base64}"
    pub node_type: Option<String>,
}

impl AnnounceFrame {
    pub fn unsigned_dict(&self) -> FrameDict;   // canonical (sorted) + signature stripped
    pub fn to_dict(&self)       -> FrameDict;
    pub fn from_dict(d: &FrameDict) -> NpsResult<Self>;
}
```

`unsigned_dict()` in Rust already returns a sorted (`BTreeMap`-built)
dict, so signing via `NipIdentity::sign` is a single call — no extra
canonicalisation step. `from_dict` defaults `ttl` to `300` when absent.

Publishing `ttl = 0` SHOULD precede a graceful shutdown so subscribers
evict the entry promptly.

---

## `ResolveFrame` (0x31)

Request / response envelope for resolving an `nwp://` URL.

```rust
pub struct ResolveFrame {
    pub target:        String,                                    // "nwp://..."
    pub requester_nid: Option<String>,
    pub resolved:      Option<serde_json::Map<String, Value>>,    // set on response
}
```

---

## `GraphFrame` (0x32)

Topology sync between registries.

```rust
pub struct GraphFrame {
    pub seq:          u64,            // strictly monotonic per publisher
    pub initial_sync: bool,           // full snapshot flag
    pub nodes:        Vec<Value>,     // full dump when initial_sync = true
    pub patch:        Option<Vec<Value>>,   // RFC 6902 ops for incremental sync
}
```

Gaps in `seq` trigger a re-sync request signalled with
`NDP-GRAPH-SEQ-GAP` (see [`error-codes.md`](https://github.com/labacacia/NPS/blob/main/spec/error-codes.md)).

---

## `InMemoryNdpRegistry`

In-memory, single-writer registry with TTL expiry evaluated **lazily**
on every read.

```rust
pub struct InMemoryNdpRegistry {
    pub clock: Box<dyn Fn() -> Instant + Send + Sync>,   // swap in tests
    // …
}

impl InMemoryNdpRegistry {
    pub fn new() -> Self;

    pub fn announce(&mut self, frame: AnnounceFrame);

    pub fn get_by_nid(&self, nid: &str) -> Option<&AnnounceFrame>;
    pub fn resolve  (&self, target: &str) -> Option<ResolveResult>;
    pub fn get_all  (&self) -> Vec<&AnnounceFrame>;

    pub fn nwp_target_matches_nid(nid: &str, target: &str) -> bool;   // associated fn
}
```

### Behaviour

- `announce` with `ttl == 0` evicts the NID immediately. Otherwise the
  entry is stored with an absolute expiry of `(clock)() + ttl seconds`
  — subsequent announces refresh the entry in place.
- `get_by_nid` / `resolve` / `get_all` skip expired entries without
  mutating the store.
- `resolve` scans live entries, finds the **first** NID that covers
  `target`, and returns its **first** advertised address as
  `ResolveResult`.

### `nwp_target_matches_nid(nid, target)`

Covering rule — associated function (no `&self`):

```
NID:    urn:nps:node:{authority}:{path}
Target: nwp://{authority}/{path}[/sub/path]
```

A node NID covers a target when:

1. `target` starts with `"nwp://"`.
2. The NID authority equals the target authority (exact,
   case-sensitive).
3. The target path equals `{path}` exactly, or starts with `{path}/`
   (sibling prefixes like `"data"` vs `"dataset"` do **not** match).

Returns `false` on malformed inputs — never panics.

### Injectable clock

```rust
use std::time::{Duration, Instant};

let start = Instant::now();
let mut reg = InMemoryNdpRegistry::new();
reg.clock = Box::new(move || start + Duration::from_secs(86_400));  // skip a day ahead
```

---

## `ResolveResult`

```rust
pub struct ResolveResult {
    pub host:     String,
    pub port:     u64,         // defaults to 17433 when missing from the address map
    pub protocol: String,      // defaults to "nwp" when missing
}
```

---

## `NdpAnnounceValidator`

Verifies an `AnnounceFrame.signature` against a registered Ed25519
public key.

```rust
pub struct NdpAnnounceValidator { /* … */ }

impl NdpAnnounceValidator {
    pub fn new() -> Self;

    pub fn register_public_key(&mut self, nid: impl Into<String>,
                                           pub_key: impl Into<String>);
    pub fn remove_public_key(&mut self, nid: &str);
    pub fn known_public_keys(&self) -> &HashMap<String, String>;

    pub fn validate(&self, frame: &AnnounceFrame) -> NdpAnnounceResult;
}
```

Validation sequence (NPS-4 §7.1):

1. Look up `frame.nid` in the registered keys. Missing →
   `NdpAnnounceResult::fail("NDP-ANNOUNCE-NID-MISMATCH", …)`. Expected
   workflow: verify the announcer's `IdentFrame` first, then
   `register_public_key(nid, ident.pub_key)`.
2. `signature` MUST start with `"ed25519:"`, else
   `NDP-ANNOUNCE-SIG-INVALID`.
3. Rebuild the signing payload from `frame.unsigned_dict()` (already
   sorted) and call
   [`NipIdentity::verify_with_pub_key_str`](./nps-rust.nip.md#nipidentity).
4. Return `NdpAnnounceResult::ok()` on success, else
   `NdpAnnounceResult::fail("NDP-ANNOUNCE-SIG-INVALID", …)`.

Register keys using the exact string produced by
`NipIdentity::pub_key_string()` — i.e. `"ed25519:{hex}"`.

---

## `NdpAnnounceResult`

```rust
pub struct NdpAnnounceResult {
    pub is_valid:   bool,
    pub error_code: Option<String>,
    pub message:    Option<String>,
}

impl NdpAnnounceResult {
    pub fn ok()                                    -> Self;
    pub fn fail(code: impl Into<String>, msg: impl Into<String>) -> Self;
}
```

---

## End-to-end

```rust
use nps_nip::NipIdentity;
use nps_ndp::{AnnounceFrame, InMemoryNdpRegistry, NdpAnnounceValidator};
use serde_json::{json, Map};

let id  = NipIdentity::generate();
let nid = "urn:nps:node:api.example.com:products".to_string();

// Build + sign the announce
let mut addr = Map::new();
addr.insert("host".into(),     json!("10.0.0.5"));
addr.insert("port".into(),     json!(17433u16));
addr.insert("protocol".into(), json!("nwp+tls"));

let mut unsigned = AnnounceFrame {
    nid:       nid.clone(),
    addresses: vec![addr],
    caps:      vec!["nwp:query".into(), "nwp:stream".into()],
    ttl:       300,
    timestamp: chrono::Utc::now().to_rfc3339(),
    signature: String::new(),
    node_type: Some("memory".into()),
};
unsigned.signature = id.sign(&unsigned.unsigned_dict());

// Validate + register
let mut validator = NdpAnnounceValidator::new();
validator.register_public_key(nid.clone(), id.pub_key_string());
let res = validator.validate(&unsigned);
assert!(res.is_valid, "validation failed: {:?}", res);

// Resolve
let mut registry = InMemoryNdpRegistry::new();
registry.announce(unsigned);
let resolved = registry.resolve("nwp://api.example.com/products/items/42").unwrap();
println!("{}:{} via {}", resolved.host, resolved.port, resolved.protocol);
```
