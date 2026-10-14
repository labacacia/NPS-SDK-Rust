English | [中文版](./nps-rust.nwp.cn.md)

# `nps-nwp` — Reference

> Spec: [NPS-2 NWP v0.13](https://github.com/labacacia/NPS/blob/main/spec/NPS-2-NWP.md)

Agent-facing HTTP layer. Two frame types, one async client built on
`reqwest`.

---

## Table of contents

- [`QueryFrame` (0x10)](#queryframe-0x10)
- [`ActionFrame` (0x11)](#actionframe-0x11)
- [`AsyncActionResponse`](#asyncactionresponse)
- [`NwpClient`](#nwpclient)
- [`InvokeResult`](#invokeresult)

---

## `QueryFrame` (0x10)

Paginated / filtered query against a Memory Node.

```rust
pub struct QueryFrame {
    pub anchor_ref:   String,                // required
    pub filter:       Option<Value>,         // NPS-2 §4 filter DSL (free-form JSON)
    pub order:        Option<Value>,         // e.g. [{"field":"id","dir":"asc"}]
    pub token_budget: Option<u64>,           // CGN Budget cap (Cognon Budget spec)
    pub limit:        Option<u64>,
    pub offset:       Option<u64>,
}

impl QueryFrame {
    pub fn new(anchor_ref: impl Into<String>) -> Self;   // all other fields = None
    pub fn frame_type() -> FrameType;                    // FrameType::Query
    pub fn to_dict(&self)   -> FrameDict;
    pub fn from_dict(d: &FrameDict) -> NpsResult<Self>;
}
```

`anchor_ref` is non-optional. The Rust `QueryFrame` does not carry the
`vector_search` / `fields` / `depth` fields of the Java / Python / TS
SDKs — use `filter`/`order` for the equivalent logic and advertise the
vector-search dialect in the anchor schema.

---

## `ActionFrame` (0x11)

Invoke an action on a node.

```rust
pub struct ActionFrame {
    pub action:     String,
    pub params:     Option<Value>,
    pub anchor_ref: Option<String>,
    pub async_:     bool,              // serialises to "async" on the wire
}
```

The field is spelt `async_` in Rust because `async` is a reserved
keyword; `to_dict` / `from_dict` translate to and from the `"async"`
JSON key.

`idempotency_key` / `timeout_ms` are not modelled as struct fields —
attach them via `params` if the remote action supports them.

---

## `AsyncActionResponse`

Returned by `NwpClient::invoke` when the request frame has
`async_ == true` (wrapped in `InvokeResult::Async`).

```rust
pub struct AsyncActionResponse {
    pub task_id:      String,
    pub status_url:   Option<String>,
    pub callback_url: Option<String>,
}

impl AsyncActionResponse {
    pub fn from_dict(d: &FrameDict) -> NpsResult<Self>;
}
```

Poll `status_url` or hand `task_id` to
[`NopClient::wait`](./nps-rust.nop.md#nopclient) to reach a terminal
state.

---

## `NwpClient`

Async HTTP client.

```rust
pub struct NwpClient { /* … */ }

impl NwpClient {
    pub fn new(base_url: impl Into<String>) -> Self;
    pub fn with_tier(self, tier: EncodingTier) -> Self;       // builder

    pub async fn send_anchor(&self, frame: &AnchorFrame) -> NpsResult<()>;
    pub async fn query      (&self, frame: &QueryFrame)  -> NpsResult<CapsFrame>;
    pub async fn stream     (&self, frame: &QueryFrame)  -> NpsResult<Vec<StreamFrame>>;
    pub async fn invoke     (&self, frame: &ActionFrame) -> NpsResult<InvokeResult>;
}
```

### Defaults

- Trailing `/` is stripped from `base_url`; every call POSTs to
  `{base_url}/{route}`.
- Default `tier = EncodingTier::MsgPack`. Override with `with_tier`.
- Internal codec uses `FrameRegistry::create_full()` — queries and
  actions can reach nodes that respond with NCP, NIP, NDP or NOP
  frames.
- Injected HTTP client is `reqwest::Client::new()` — configure TLS /
  timeouts by patching the struct (`client.http = …`) in consumer
  code, or wrap your own transport.

### HTTP routes

| Method        | Path      | Request body                 | Response body |
|---------------|-----------|------------------------------|---------------|
| `send_anchor` | `/anchor` | encoded `AnchorFrame`        | — (2xx only) |
| `query`       | `/query`  | encoded `QueryFrame`         | encoded `CapsFrame` |
| `stream`      | `/stream` | encoded `QueryFrame`         | concatenated `StreamFrame`s |
| `invoke`      | `/invoke` | encoded `ActionFrame`        | encoded frame, JSON (async), or fallback JSON |

Request headers: `Content-Type: application/x-nps-frame`, `Accept:
application/x-nps-frame`.

### `stream` behaviour

Buffered: the response body is read into a `Vec<u8>`, then sliced
frame-by-frame via `FrameHeader::parse`. Iteration stops when a frame
reports `is_last == true` (the in-payload flag — not the wire FINAL
bit).

### `invoke` dispatch

| Request | Response Content-Type | Returned variant |
|---------|-----------------------|------------------|
| `async_ == true` | any | `InvokeResult::Async(AsyncActionResponse)` — body parsed as JSON |
| `async_ == false` | contains `application/x-nps-frame` | `InvokeResult::Frame(FrameDict)` — body decoded via codec |
| `async_ == false` | anything else | `InvokeResult::Json(FrameDict)` — body parsed as JSON |

### Errors

- Non-2xx HTTP → `NpsError::Io("NWP /{path} failed: HTTP {status}")`.
- Transport / TLS failure → `NpsError::Io(e.to_string())`.
- Payload decode failure → `NpsError::Codec`.
- Unexpected frame type (e.g. `query` returns non-Caps) →
  `NpsError::Frame`.

---

## `InvokeResult`

```rust
pub enum InvokeResult {
    Frame(FrameDict),              // NPS-encoded response, already decoded to dict
    Async(AsyncActionResponse),    // 202-style async handle
    Json(FrameDict),               // JSON response (non-NPS content-type)
}
```

Pattern-match on the variant; if you need a typed NCP frame from
`Frame(dict)` call `CapsFrame::from_dict(&dict)` / `ErrorFrame::from_dict(&dict)`
etc.

---

## End-to-end

```rust
use nps_nwp::{NwpClient, QueryFrame, ActionFrame, InvokeResult};

let client = NwpClient::new("http://node.example.com:17433");

// Query
let mut q = QueryFrame::new("sha256:abc123");
q.filter = Some(serde_json::json!({ "active": true }));
q.limit  = Some(50);
let caps = client.query(&q).await?;
println!("caps: {:?}", caps.caps);

// Invoke
let action = ActionFrame {
    action:     "summarise".into(),
    params:     Some(serde_json::json!({ "max_tokens": 500 })),
    anchor_ref: None,
    async_:     false,
};
match client.invoke(&action).await? {
    InvokeResult::Frame(dict) => { /* decoded NPS frame */ }
    InvokeResult::Async(r)    => { /* poll NopClient::wait(&r.task_id, …) */ }
    InvokeResult::Json(dict)  => { /* plain JSON fallback */ }
}
```
