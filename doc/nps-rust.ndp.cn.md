[English Version](./nps-rust.ndp.md) | 中文版

# `nps-ndp` — 参考

> 规范：[NPS-4 NDP v0.8](https://github.com/labacacia/NPS/blob/main/spec/NPS-4-NDP.md)

发现层 —— NPS 对标 DNS。三种帧、一个内存 TTL 注册表、一个签名校验器。

---

## 目录

- [`AnnounceFrame` (0x30)](#announceframe-0x30)
- [`ResolveFrame` (0x31)](#resolveframe-0x31)
- [`GraphFrame` (0x32)](#graphframe-0x32)
- [`InMemoryNdpRegistry`](#inmemoryndpregistry)
- [`ResolveResult`](#resolveresult)
- [`NdpAnnounceValidator`](#ndpannouncevalidator)
- [`NdpAnnounceResult`](#ndpannounceresult)

---

## `AnnounceFrame` (0x30)

发布节点的物理可达性与 TTL（NPS-4 §3.1）。

```rust
pub struct AnnounceFrame {
    pub nid:       String,
    pub addresses: Vec<serde_json::Map<String, Value>>,   // [{"host","port","protocol"}, …]
    pub caps:      Vec<String>,
    pub ttl:       u64,                                   // 秒；0 = 关停
    pub timestamp: String,                                // ISO 8601 UTC
    pub signature: String,                                // "ed25519:{base64}"
    pub node_type: Option<String>,
}

impl AnnounceFrame {
    pub fn unsigned_dict(&self) -> FrameDict;   // 规范化（键排序）+ 去掉 signature
    pub fn to_dict(&self)       -> FrameDict;
    pub fn from_dict(d: &FrameDict) -> NpsResult<Self>;
}
```

Rust 中 `unsigned_dict()` 已返回键排序（由 `BTreeMap` 构造）的 dict，
因此经 `NipIdentity::sign` 签名只需一次调用 —— 无需额外规范化步骤。
`from_dict` 在字段缺失时将 `ttl` 缺省为 `300`。

发布 `ttl = 0` 应该先于优雅关停执行，以便订阅者及时驱逐条目。

---

## `ResolveFrame` (0x31)

解析 `nwp://` URL 的请求 / 响应信封。

```rust
pub struct ResolveFrame {
    pub target:        String,                                    // "nwp://..."
    pub requester_nid: Option<String>,
    pub resolved:      Option<serde_json::Map<String, Value>>,    // 响应时设置
}
```

---

## `GraphFrame` (0x32)

注册表间的拓扑同步。

```rust
pub struct GraphFrame {
    pub seq:          u64,            // 按发布者严格单调
    pub initial_sync: bool,           // 全量快照标志
    pub nodes:        Vec<Value>,     // initial_sync = true 时为完整 dump
    pub patch:        Option<Vec<Value>>,   // 增量同步的 RFC 6902 ops
}
```

`seq` 出现跳变时，应发起重新同步请求，信号为 `NDP-GRAPH-SEQ-GAP`
（见 [`error-codes.md`](https://github.com/labacacia/NPS/blob/main/spec/error-codes.md)）。

---

## `InMemoryNdpRegistry`

内存、单写注册表，在每次读取时**惰性**评估 TTL 过期。

```rust
pub struct InMemoryNdpRegistry {
    pub clock: Box<dyn Fn() -> Instant + Send + Sync>,   // 测试中替换
    // …
}

impl InMemoryNdpRegistry {
    pub fn new() -> Self;

    pub fn announce(&mut self, frame: AnnounceFrame);

    pub fn get_by_nid(&self, nid: &str) -> Option<&AnnounceFrame>;
    pub fn resolve  (&self, target: &str) -> Option<ResolveResult>;
    pub fn get_all  (&self) -> Vec<&AnnounceFrame>;

    pub fn nwp_target_matches_nid(nid: &str, target: &str) -> bool;   // 关联函数
}
```

### 行为

- `announce` 时 `ttl == 0` 立即驱逐该 NID。否则以绝对过期时间
  `(clock)() + ttl 秒` 存储条目 —— 后续 announce 原地刷新。
- `get_by_nid` / `resolve` / `get_all` 跳过已过期条目，不改动存储。
- `resolve` 扫描活跃条目，找到**第一个**覆盖 `target` 的 NID，
  返回其**第一个**广告地址作为 `ResolveResult`。

### `nwp_target_matches_nid(nid, target)`

覆盖规则 —— 关联函数（无 `&self`）：

```
NID:    urn:nps:node:{authority}:{path}
Target: nwp://{authority}/{path}[/sub/path]
```

节点 NID 覆盖 target 的条件：

1. `target` 以 `"nwp://"` 开头。
2. NID authority 与 target authority 完全相等（区分大小写，精确匹配）。
3. target 路径等于 `{path}` 完全一致，或以 `{path}/` 开头
   （`"data"` 与 `"dataset"` 这样的兄弟前缀**不**匹配）。

输入畸形时返回 `false` —— 从不 panic。

### 可注入 clock

```rust
use std::time::{Duration, Instant};

let start = Instant::now();
let mut reg = InMemoryNdpRegistry::new();
reg.clock = Box::new(move || start + Duration::from_secs(86_400));  // 跳到一天后
```

---

## `ResolveResult`

```rust
pub struct ResolveResult {
    pub host:     String,
    pub port:     u64,         // 地址 map 缺失时缺省 17433
    pub protocol: String,      // 缺失时缺省 "nwp"
}
```

---

## `NdpAnnounceValidator`

根据已注册的 Ed25519 公钥校验 `AnnounceFrame.signature`。

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

校验序列（NPS-4 §7.1）：

1. 在已注册密钥中查找 `frame.nid`。缺失 →
   `NdpAnnounceResult::fail("NDP-ANNOUNCE-NID-MISMATCH", …)`。期望
   流程：先验证发布者的 `IdentFrame`，再
   `register_public_key(nid, ident.pub_key)`。
2. `signature` 必须以 `"ed25519:"` 开头，否则 `NDP-ANNOUNCE-SIG-INVALID`。
3. 从 `frame.unsigned_dict()`（已排序）重建签名 payload，调用
   [`NipIdentity::verify_with_pub_key_str`](./nps-rust.nip.cn.md#nipidentity)。
4. 成功返回 `NdpAnnounceResult::ok()`，否则
   `NdpAnnounceResult::fail("NDP-ANNOUNCE-SIG-INVALID", …)`。

注册密钥使用 `NipIdentity::pub_key_string()` 产出的字符串原样 ——
即 `"ed25519:{hex}"`。

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

## 端到端

```rust
use nps_nip::NipIdentity;
use nps_ndp::{AnnounceFrame, InMemoryNdpRegistry, NdpAnnounceValidator};
use serde_json::{json, Map};

let id  = NipIdentity::generate();
let nid = "urn:nps:node:api.example.com:products".to_string();

// 构造 + 签名 announce
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

// 校验 + 注册
let mut validator = NdpAnnounceValidator::new();
validator.register_public_key(nid.clone(), id.pub_key_string());
let res = validator.validate(&unsigned);
assert!(res.is_valid, "validation failed: {:?}", res);

// 解析
let mut registry = InMemoryNdpRegistry::new();
registry.announce(unsigned);
let resolved = registry.resolve("nwp://api.example.com/products/items/42").unwrap();
println!("{}:{} via {}", resolved.host, resolved.port, resolved.protocol);
```
