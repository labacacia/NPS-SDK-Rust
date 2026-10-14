English | [中文版](./nps-rust.nop.cn.md)

# `nps-nop` — Reference

> Spec: [NPS-5 NOP v0.6](https://github.com/labacacia/NPS/blob/main/spec/NPS-5-NOP.md)

Orchestration layer — DAG submission, fan-in barriers, streaming
progress, async status polling.

---

## Table of contents

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

`compute_delay_ms` (0-indexed `attempt`):

| Strategy       | Formula                  |
|----------------|--------------------------|
| `Fixed`        | `base_ms`                |
| `Linear`       | `base_ms * (attempt + 1)`|
| `Exponential`  | `base_ms * 2^attempt`    |

Result is clamped at `max_ms`.

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
    pub fn from_str(s: &str) -> Option<Self>;   // case-sensitive: "pending" | "running" | …
    pub fn is_terminal(self) -> bool;           // Completed | Failed | Cancelled
}
```

> The Rust SDK exposes only the five common states above. Orchestrator
> responses carrying `"preflight"`, `"waiting_sync"` or `"skipped"`
> will decode with `state() == None` — use
> `NopTaskStatus::raw()["state"]` to inspect the raw string when
> needed.

---

## `NopTaskStatus`

Thin view over the orchestrator's JSON status payload.

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

Use `raw()` to reach orchestrator-specific fields that the typed
accessors don't cover.

---

## `TaskFrame` (0x40)

Submit a DAG for execution. The DAG itself is kept as a free-form
`serde_json::Value` that matches the NPS-5 wire shape
(`{"nodes": [...], "edges": [...]}`).

```rust
pub struct TaskFrame {
    pub task_id:      String,
    pub dag:          Value,               // free-form DAG JSON
    pub timeout_ms:   Option<u64>,
    pub callback_url: Option<String>,      // SSRF-validated by orchestrator
    pub context:      Option<Value>,       // { "session_key", "requester_nid", "trace_id" }
    pub priority:     Option<String>,      // "low" | "normal" | "high"
    pub depth:        Option<i64>,         // delegate chain depth, max 3
}
```

Spec limits the orchestrator enforces (NPS-5 §8.2): max 32 nodes per
DAG, max 3 levels of delegate chain, max timeout 3 600 000 ms (1 h).

---

## `DelegateFrame` (0x41)

Per-node invocation emitted by the orchestrator to each agent.

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

> Field naming differs from other SDKs: the Rust SDK uses
> `target_nid` + `config` where the .NET / Python / Java SDKs use
> `agent_nid` + `params`. Wire payloads follow the field names above.

---

## `SyncFrame` (0x42)

Fan-in barrier — waits for K-of-N upstream subtasks.

```rust
pub struct SyncFrame {
    pub task_id:      String,
    pub sync_id:      String,
    pub subtask_ids:  Vec<String>,
    pub min_required: i64,               // 0 = strict all-of
    pub aggregate:    String,            // "merge" | "first" | "fastest_k" | "all"
    pub timeout_ms:   Option<u64>,
}
```

`from_dict` defaults `min_required` to `0` and `aggregate` to
`"merge"` when those fields are missing.

`min_required` semantics:

| Value | Meaning |
|-------|---------|
| `0`   | Wait for all of `subtask_ids` (strict fan-in). |
| `K`   | Proceed as soon as K upstream subtasks have completed. |

---

## `AlignStreamFrame` (0x43)

Streaming progress / partial result frame for a delegated subtask.

```rust
pub struct AlignStreamFrame {
    pub sync_id:     String,
    pub task_id:     String,
    pub subtask_id:  String,
    pub seq:         u64,
    pub is_final:    bool,
    pub source_nid:  Option<String>,
    pub result:      Option<Value>,       // opaque payload
    pub error:       Option<Value>,       // { "error_code", "message" }
    pub window_size: Option<u64>,
}

impl AlignStreamFrame {
    pub fn error_code(&self)    -> Option<&str>;   // shortcut for error["error_code"]
    pub fn error_message(&self) -> Option<&str>;   // shortcut for error["message"]
}
```

`AlignStreamFrame` replaces the deprecated `AlignFrame (0x05)` — it
carries task context (`task_id` + `subtask_id` + `sync_id`) and binds
the stream to a `source_nid`.

---

## `NopClient`

Async HTTP client for an NOP orchestrator.

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

### HTTP routes

| Method       | Path               | Request body                      | Response |
|--------------|--------------------|-----------------------------------|----------|
| `submit`     | `POST   /tasks`    | JSON of `TaskFrame::to_dict()`    | JSON `{ "task_id": … }` |
| `get_status` | `GET    /tasks/{}` | —                                 | JSON status dict |
| `cancel`     | `DELETE /tasks/{}` | —                                 | — |
| `wait`       | polls `get_status` until terminal or `timeout`; `tokio::time::sleep` between polls |

Requests use `Content-Type: application/json` — the Rust NOP client
submits the task dict as plain JSON, not as a framed
`application/x-nps-frame` payload.

`wait` fails with `NpsError::Io("timeout waiting for task …")` if the
deadline elapses before the task reaches a terminal state; it returns
the terminal `NopTaskStatus` on success.

### Errors

- Non-2xx HTTP → `NpsError::Io("NOP {path} failed: HTTP {status}")`.
- Missing `task_id` in the submit response → `NpsError::Frame`.
- Transport / decode failures → `NpsError::Io` / `NpsError::Codec`.

---

## End-to-end

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

// Backoff computation
let delay = BackoffStrategy::Exponential.compute_delay_ms(500, 30_000, 2);  // → 2000
```
