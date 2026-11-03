[English Version](./nps-rust.ncp.md) | 中文版

# `nps-ncp` — 参考

> 规范：[NPS-1 NCP v0.7](https://github.com/labacacia/NPS/blob/main/spec/NPS-1-NCP.md)

五种 NCP 帧类型。每个 struct 都暴露同一组三件套：

```rust
pub fn frame_type() -> FrameType;
pub fn to_dict(&self) -> FrameDict;
pub fn from_dict(d: &FrameDict) -> NpsResult<Self>;
```

> **注 —— Rust 帧形状。** Rust NCP struct 的字段集与 Java / Python /
> .NET / TS SDK 略有差异：下列 Rust 布局是本 crate 的权威定义。

---

## 目录

- [`AnchorFrame` (0x01)](#anchorframe-0x01)
- [`DiffFrame` (0x02)](#diffframe-0x02)
- [`StreamFrame` (0x03)](#streamframe-0x03)
- [`CapsFrame` (0x04)](#capsframe-0x04)
- [`ErrorFrame` (0xFE)](#errorframe-0xfe)

---

## `AnchorFrame` (0x01)

发布 schema 锚点 + TTL。

```rust
pub struct AnchorFrame {
    pub anchor_id:   String,
    pub schema:      serde_json::Map<String, Value>,
    pub namespace:   Option<String>,
    pub description: Option<String>,
    pub node_type:   Option<String>,     // 如 "memory" | "action" | …
    pub ttl:         u64,                // 秒；`from_dict` 缺省 3600
}
```

`schema` 以自由形式 map 存储 —— 通常为
`{ "fields": [ { "name": …, "type": … }, … ] }`，但节点和客户端
同意的任何形状都合法。字段缺失时 `from_dict` 回退到 `ttl = 3600`。

要确定性生成内容寻址的 `anchor_id`，使用
[`AnchorFrameCache::compute_anchor_id`](./nps-rust.core.cn.md#anchorframecache)。

---

## `DiffFrame` (0x02)

两个 anchor 之间的 schema 演进。

```rust
pub struct DiffFrame {
    pub anchor_id:     String,      // 旧 anchor
    pub new_anchor_id: String,      // 新 anchor
    pub patch:         Vec<Value>,  // JSON-Patch 形状 ops（自由形式）
}
```

`patch` 原样序列化 —— 本 crate 不校验 ops；接收方需了解 patch 方言
（NPS-1 §5.2 使用 RFC 6902 兼容形状）。

---

## `StreamFrame` (0x03)

流式响应的一个分块。多个 `StreamFrame` 拼出结果；最后一个分块设
`is_last = true`。

```rust
pub struct StreamFrame {
    pub anchor_id: String,
    pub seq:       u64,
    pub payload:   Value,     // 不透明 —— 任何可 JSON 表示的值
    pub is_last:   bool,
}
```

线路级 `FINAL` flag（帧头 bit 6）与 `is_last` **分离**。`is_last` 是
payload 内业务标记，由
[`NwpClient::stream`](./nps-rust.nwp.cn.md#nwpclient) 用来停止迭代。

---

## `CapsFrame` (0x04)

节点能力 / 响应信封帧。

```rust
pub struct CapsFrame {
    pub node_id:    String,
    pub caps:       Vec<String>,         // 能力 URI
    pub anchor_ref: Option<String>,      // 被应答的 anchor
    pub payload:    Option<Value>,       // 不透明响应数据
}
```

在 Rust SDK 中 `CapsFrame` 是 NWP 的**默认响应信封**：
`NwpClient::query` 直接返回 `CapsFrame`（读取 `anchor_ref` + `payload`）。
Caps 广告用法和响应用法共用同一 struct —— 通过检查 `caps` 与
`payload` 区分。

---

## `ErrorFrame` (0xFE)

统一协议级错误。

```rust
pub struct ErrorFrame {
    pub error_code: String,          // "NWP-QUERY-ANCHOR-UNKNOWN", …
    pub message:    String,
    pub detail:     Option<Value>,   // 自由形式附加上下文
}
```

命名空间见
[`error-codes.md`](https://github.com/labacacia/NPS/blob/main/spec/error-codes.md)。

---

## 端到端

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
