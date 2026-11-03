[English Version](./nps-rust.nop.md) | 中文版

# `nps-nop` — 参考

> 规范：[NPS-5 NOP v0.6](https://github.com/labacacia/NPS/blob/main/spec/NPS-5-NOP.md)

编排层 —— DAG 提交、fan-in 屏障、流式进度、异步状态轮询。

---

## 目录

- [`BackoffStrategy`](#backoffstrategy)
- [`TaskState`](#taskstate)
- [`NopTaskStatus`](#noptaskstatus)
- [`TaskFrame` (0x40)](#taskframe-0x40)
- [`DelegateFrame` (0x41)](#delegateframe-0x41)
- [`SyncFrame` (0x42)](#syncframe-0x42)
- [`AlignStreamFrame` (0x43)](#alignstreamframe-0x43)
- [`NopClient`](#nopclient)

---

## `BackoffStrategy`

```rust
pub enum BackoffStrategy { Fixed, Linear, Exponential }

impl BackoffStrategy {
    pub fn compute_delay_ms(self, base_ms: u64, max_ms: u64, attempt: u32) -> u64;
}
```

`compute_delay_ms`（`attempt` 从 0 开始）：

| 策略           | 公式                     |
|----------------|--------------------------|
| `Fixed`        | `base_ms`                |
| `Linear`       | `base_ms * (attempt + 1)`|
| `Exponential`  | `base_ms * 2^attempt`    |

结果以 `max_ms` 截顶。

---

## `TaskState`

```rust
pub enum TaskState {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl TaskState {
    pub fn from_str(s: &str) -> Option<Self>;   // 区分大小写："pending" | "running" | …
    pub fn is_terminal(self) -> bool;           // Completed | Failed | Cancelled
}
```

> Rust SDK 仅暴露以上五个常见状态。编排器响应若携带
> `"preflight"`、`"waiting_sync"` 或 `"skipped"` 将以
> `state() == None` 解码 —— 需要时用
> `NopTaskStatus::raw()["state"]` 查看原始字符串。

---

## `NopTaskStatus`

对编排器 JSON 状态 payload 的薄视图。

```rust
pub struct NopTaskStatus { /* raw: serde_json::Map<…, …> */ }

impl NopTaskStatus {
    pub fn from_dict(raw: serde_json::Map<String, Value>) -> Self;

    pub fn task_id(&self)       -> &str;
    pub fn state(&self)         -> Option<TaskState>;
    pub fn is_terminal(&self)   -> bool;
    pub fn error_code(&self)    -> Option<&str>;
    pub fn error_message(&self) -> Option<&str>;
    pub fn node_results(&self)  -> Option<&serde_json::Map<String, Value>>;
    pub fn raw(&self)           -> &serde_json::Map<String, Value>;
}

impl Display for NopTaskStatus { /* "NopTaskStatus(task_id=…, state=…)" */ }
```

用 `raw()` 访问有类型访问器未覆盖的编排器特有字段。

---

## `TaskFrame` (0x40)

提交一个 DAG 执行。DAG 本身以自由形式 `serde_json::Value` 保持，
匹配 NPS-5 线路形状（`{"nodes": [...], "edges": [...]}`）。

```rust
pub struct TaskFrame {
    pub task_id:      String,
    pub dag:          Value,               // 自由形式 DAG JSON
    pub timeout_ms:   Option<u64>,
    pub callback_url: Option<String>,      // 由编排器做 SSRF 校验
    pub context:      Option<Value>,       // { "session_key", "requester_nid", "trace_id" }
    pub priority:     Option<String>,      // "low" | "normal" | "high"
    pub depth:        Option<i64>,         // 委托链深度，最大 3
}
```

编排器强制执行的规范限制（NPS-5 §8.2）：每 DAG 最多 32 节点、
最多 3 层委托链、最大 timeout 3 600 000 ms（1 小时）。

---

## `DelegateFrame` (0x41)

编排器为每个 agent 发出的逐节点调用。

```rust
pub struct DelegateFrame {
    pub task_id:         String,
    pub subtask_id:      String,
    pub action:          String,
    pub target_nid:      String,
    pub inputs:          Option<Value>,
    pub config:          Option<Value>,
    pub idempotency_key: Option<String>,
}
```

> 字段命名与其他 SDK 不同：Rust SDK 用 `target_nid` + `config`，
> 而 .NET / Python / Java SDK 用 `agent_nid` + `params`。线路 payload
> 跟随以上字段名。

---

## `SyncFrame` (0x42)

Fan-in 屏障 —— 等待上游 K-of-N 子任务。

```rust
pub struct SyncFrame {
    pub task_id:      String,
    pub sync_id:      String,
    pub subtask_ids:  Vec<String>,
    pub min_required: i64,               // 0 = 严格全部
    pub aggregate:    String,            // "merge" | "first" | "fastest_k" | "all"
    pub timeout_ms:   Option<u64>,
}
```

字段缺失时 `from_dict` 将 `min_required` 缺省为 `0`，`aggregate`
缺省为 `"merge"`。

`min_required` 语义：

| 值 | 含义 |
|----|------|
| `0`| 等待全部 `subtask_ids`（严格 fan-in）|
| `K`| K 个上游子任务完成后即推进 |

---

## `AlignStreamFrame` (0x43)

被委托子任务的流式进度 / 部分结果帧。

```rust
pub struct AlignStreamFrame {
    pub sync_id:     String,
    pub task_id:     String,
    pub subtask_id:  String,
    pub seq:         u64,
    pub is_final:    bool,
    pub source_nid:  Option<String>,
    pub result:      Option<Value>,       // 不透明 payload
    pub error:       Option<Value>,       // { "error_code", "message" }
    pub window_size: Option<u64>,
}

impl AlignStreamFrame {
    pub fn error_code(&self)    -> Option<&str>;   // error["error_code"] 的快捷
    pub fn error_message(&self) -> Option<&str>;   // error["message"] 的快捷
}
```

`AlignStreamFrame` 替代已弃用的 `AlignFrame (0x05)` —— 携带 task
上下文（`task_id` + `subtask_id` + `sync_id`）并将流绑定到
`source_nid`。

---

## `NopClient`

NOP 编排器的异步 HTTP 客户端。

```rust
pub struct NopClient { /* … */ }

impl NopClient {
    pub fn new(base_url: impl Into<String>) -> Self;

    pub async fn submit    (&self, frame: &TaskFrame) -> NpsResult<String>;    // → task_id
    pub async fn get_status(&self, task_id: &str)     -> NpsResult<NopTaskStatus>;
    pub async fn cancel    (&self, task_id: &str)     -> NpsResult<()>;

    pub async fn wait(
        &self,
        task_id:       &str,
        timeout:       std::time::Duration,
        poll_interval: std::time::Duration,
    ) -> NpsResult<NopTaskStatus>;
}
```

### HTTP 路由

| 方法         | 路径               | 请求体                            | 响应 |
|--------------|--------------------|-----------------------------------|------|
| `submit`     | `POST   /tasks`    | `TaskFrame::to_dict()` 的 JSON    | JSON `{ "task_id": … }` |
| `get_status` | `GET    /tasks/{}` | —                                 | JSON 状态 dict |
| `cancel`     | `DELETE /tasks/{}` | —                                 | — |
| `wait`       | 轮询 `get_status` 直至终态或 `timeout`；轮询间用 `tokio::time::sleep` |

请求使用 `Content-Type: application/json` —— Rust NOP 客户端以普通
JSON 提交 task dict，而非成帧的 `application/x-nps-frame` payload。

若截止时间到达前任务未到达终态，`wait` 以
`NpsError::Io("timeout waiting for task …")` 失败；成功时返回终态的
`NopTaskStatus`。

### 错误

- 非 2xx HTTP → `NpsError::Io("NOP {path} failed: HTTP {status}")`。
- submit 响应缺少 `task_id` → `NpsError::Frame`。
- 传输 / 解码失败 → `NpsError::Io` / `NpsError::Codec`。

---

## 端到端

```rust
use nps_nop::{NopClient, TaskFrame, BackoffStrategy};
use serde_json::json;
use std::time::Duration;

let dag = json!({
    "nodes": [
        { "id": "fetch",    "action": "fetch",
          "agent": "urn:nps:node:ingest.example.com:http" },
        { "id": "classify", "action": "classify",
          "agent": "urn:nps:node:ml.example.com:classifier",
          "input_from": ["fetch"],
          "retry_policy": {
              "max_retries":   3,
              "backoff":       "exponential",
              "base_delay_ms": 500
          }}
    ],
    "edges": [ { "from": "fetch", "to": "classify" } ]
});

let nop = NopClient::new("http://orchestrator.example.com:17433");
let tid = nop.submit(&TaskFrame {
    task_id:      "job-42".into(),
    dag,
    timeout_ms:   Some(60_000),
    callback_url: None,
    context:      None,
    priority:     Some("normal".into()),
    depth:        None,
}).await?;

let status = nop.wait(&tid, Duration::from_secs(60), Duration::from_millis(500)).await?;
println!("{status}");

// Backoff 计算
let delay = BackoffStrategy::Exponential.compute_delay_ms(500, 30_000, 2);  // → 2000
```
