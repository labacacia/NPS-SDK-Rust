[English Version](./nps-rust.nwp.md) | 中文版

# `nps-nwp` — 参考

> 规范：[NPS-2 NWP v0.13](https://github.com/labacacia/NPS/blob/main/spec/NPS-2-NWP.md)

面向 Agent 的 HTTP 层。两种帧类型、一个基于 `reqwest` 的异步客户端。

---

## 目录

- [`QueryFrame` (0x10)](#queryframe-0x10)
- [`ActionFrame` (0x11)](#actionframe-0x11)
- [`AsyncActionResponse`](#asyncactionresponse)
- [`NwpClient`](#nwpclient)
- [`InvokeResult`](#invokeresult)

---

## `QueryFrame` (0x10)

针对 Memory Node 的分页 / 过滤查询。

```rust
pub struct QueryFrame {
    pub anchor_ref:   String,                // 必填
    pub filter:       Option<Value>,         // NPS-2 §4 过滤 DSL（自由形式 JSON）
    pub order:        Option<Value>,         // 如 [{"field":"id","dir":"asc"}]
    pub token_budget: Option<u64>,           // CGN Budget 上限（Cognon Budget 规范）
    pub limit:        Option<u64>,
    pub offset:       Option<u64>,
}

impl QueryFrame {
    pub fn new(anchor_ref: impl Into<String>) -> Self;   // 其他字段 = None
    pub fn frame_type() -> FrameType;                    // FrameType::Query
    pub fn to_dict(&self)   -> FrameDict;
    pub fn from_dict(d: &FrameDict) -> NpsResult<Self>;
}
```

`anchor_ref` 非可选。Rust `QueryFrame` 不携带 Java / Python / TS SDK
的 `vector_search` / `fields` / `depth` 字段 —— 用 `filter` / `order`
表达等价逻辑，并在 anchor schema 中声明向量检索方言。

---

## `ActionFrame` (0x11)

在节点上调用动作。

```rust
pub struct ActionFrame {
    pub action:     String,
    pub params:     Option<Value>,
    pub anchor_ref: Option<String>,
    pub async_:     bool,              // 线路上序列化为 "async"
}
```

因 `async` 是保留关键字，字段在 Rust 中拼作 `async_`；`to_dict` /
`from_dict` 会往返翻译为 `"async"` JSON 键。

`idempotency_key` / `timeout_ms` 未作为 struct 字段建模 —— 若远端
动作支持，通过 `params` 附加。

---

## `AsyncActionResponse`

当请求帧 `async_ == true` 时由 `NwpClient::invoke` 返回（包装在
`InvokeResult::Async` 中）。

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

轮询 `status_url` 或把 `task_id` 交给
[`NopClient::wait`](./nps-rust.nop.cn.md#nopclient) 以到达终态。

---

## `NwpClient`

异步 HTTP 客户端。

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

### 默认值

- `base_url` 尾 `/` 被剥离；每次调用 POST 到 `{base_url}/{route}`。
- 默认 `tier = EncodingTier::MsgPack`。用 `with_tier` 覆盖。
- 内部编解码器使用 `FrameRegistry::create_full()` —— 查询与动作可抵达
  以 NCP、NIP、NDP、NOP 帧响应的节点。
- 注入的 HTTP 客户端为 `reqwest::Client::new()` —— 要配置 TLS /
  超时，请在消费代码中修补 struct（`client.http = …`），或包装自己
  的传输。

### HTTP 路由

| 方法         | 路径      | 请求体                        | 响应体 |
|--------------|-----------|-------------------------------|--------|
| `send_anchor`| `/anchor` | 编码的 `AnchorFrame`          | —（仅 2xx）|
| `query`      | `/query`  | 编码的 `QueryFrame`           | 编码的 `CapsFrame` |
| `stream`     | `/stream` | 编码的 `QueryFrame`           | 拼接的 `StreamFrame` 序列 |
| `invoke`     | `/invoke` | 编码的 `ActionFrame`          | 编码帧、JSON（异步）或回退 JSON |

请求头：`Content-Type: application/x-nps-frame`，
`Accept: application/x-nps-frame`。

### `stream` 行为

缓冲：响应体读入 `Vec<u8>`，然后通过 `FrameHeader::parse` 逐帧切分。
当某帧报告 `is_last == true`（payload 内标志 —— 非线路 FINAL bit）
时，迭代停止。

### `invoke` 分派

| 请求 | 响应 Content-Type | 返回 variant |
|------|-------------------|--------------|
| `async_ == true` | 任意 | `InvokeResult::Async(AsyncActionResponse)` —— body 解析为 JSON |
| `async_ == false` | 包含 `application/x-nps-frame` | `InvokeResult::Frame(FrameDict)` —— body 经编解码器解码 |
| `async_ == false` | 其他 | `InvokeResult::Json(FrameDict)` —— body 解析为 JSON |

### 错误

- 非 2xx HTTP → `NpsError::Io("NWP /{path} failed: HTTP {status}")`。
- 传输 / TLS 失败 → `NpsError::Io(e.to_string())`。
- Payload 解码失败 → `NpsError::Codec`。
- 意外帧类型（如 `query` 返回非 Caps）→ `NpsError::Frame`。

---

## `InvokeResult`

```rust
pub enum InvokeResult {
    Frame(FrameDict),              // NPS 编码响应，已解码为 dict
    Async(AsyncActionResponse),    // 202 风格异步句柄
    Json(FrameDict),               // JSON 响应（非 NPS content-type）
}
```

在 variant 上模式匹配；若需从 `Frame(dict)` 得到有类型 NCP 帧，
调用 `CapsFrame::from_dict(&dict)` / `ErrorFrame::from_dict(&dict)` 等。

---

## 端到端

```rust
use nps_nwp::{NwpClient, QueryFrame, ActionFrame, InvokeResult};

let client = NwpClient::new("http://node.example.com:17433");

// 查询
let mut q = QueryFrame::new("sha256:abc123");
q.filter = Some(serde_json::json!({ "active": true }));
q.limit  = Some(50);
let caps = client.query(&q).await?;
println!("caps: {:?}", caps.caps);

// 调用
let action = ActionFrame {
    action:     "summarise".into(),
    params:     Some(serde_json::json!({ "max_tokens": 500 })),
    anchor_ref: None,
    async_:     false,
};
match client.invoke(&action).await? {
    InvokeResult::Frame(dict) => { /* 解码的 NPS 帧 */ }
    InvokeResult::Async(r)    => { /* 轮询 NopClient::wait(&r.task_id, …) */ }
    InvokeResult::Json(dict)  => { /* 纯 JSON 回退 */ }
}
```
