[English Version](./nps-rust.core.md) | 中文版

# `nps-core` — 参考

> 规范：[NPS-1 NCP v0.7](https://github.com/labacacia/NPS/blob/main/spec/NPS-1-NCP.md)

基础 crate。定义线路帧头、编码 tier、注册表校验的编解码器、
anchor-frame 缓存，以及 `NpsError` 层级。

---

## 目录

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
    pub fn from_u8(v: u8) -> NpsResult<Self>;   // 未知时 Err(NpsError::Frame)
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

其值为 `flags` 字节的 bit-7 状态 —— `MsgPack = 1` 置 `0x80`，
`Json = 0` 保持清零。

---

## `FrameHeader`

线路格式帧头。

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
    pub fn header_size(&self)   -> usize;          // 4 或 8

    pub fn parse(wire: &[u8])   -> NpsResult<Self>;
    pub fn to_bytes(&self)      -> Vec<u8>;
}
```

### Flags 字节

| Bit | Mask   | 含义 |
|-----|--------|------|
| 7   | `0x80` | TIER —— `1` = MsgPack，`0` = JSON |
| 6   | `0x40` | FINAL —— 流中最后一帧 |
| 0   | `0x01` | EXT —— 8 字节扩展帧头 |

### 线路布局

```
默认（EXT=0，4 字节）：
  [frame_type][flags][len_hi][len_lo]         — u16 大端长度

扩展（EXT=1，8 字节）：
  [frame_type][flags][0][0][len_b3..len_b0]   — u32 大端长度
```

`FrameHeader::new` 在 `payload_length > 0xFFFF` 时自动启用 EXT。

---

## `FrameDict`

```rust
pub type FrameDict = serde_json::Map<String, Value>;
```

所有帧都通过 `FrameDict` 往返。辅助函数：

```rust
pub fn encode_json   (dict: &FrameDict) -> NpsResult<Vec<u8>>;
pub fn encode_msgpack(dict: &FrameDict) -> NpsResult<Vec<u8>>;   // rmp_serde::to_vec_named
pub fn decode_json   (payload: &[u8])   -> NpsResult<FrameDict>;
pub fn decode_msgpack(payload: &[u8])   -> NpsResult<FrameDict>;
```

MsgPack 编码使用**具名字段**形式（map 键保持字符串）—— 线路与 JSON
tier 互操作，只是更小。

---

## `NpsFrameCodec`

注册表校验、可切换 tier 的编解码器。

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

- 若序列化 payload 超过 `max_payload`，`encode` 以 `NpsError::Codec` 失败。
- 若帧头的 frame type 未在此编解码器的 `FrameRegistry` 中注册，
  `decode` 以 `NpsError::Frame` 失败。
- `peek_header` 是关联函数（无 `&self`）—— 流式处理时用来在分配完整帧
  之前得知长度。

---

## `FrameRegistry`

```rust
pub struct FrameRegistry { /* … */ }

impl FrameRegistry {
    pub fn new()           -> Self;             // 空
    pub fn register(&mut self, ft: FrameType);
    pub fn is_registered(&self, ft: FrameType) -> bool;

    pub fn create_default() -> Self;            // 仅 NCP（Anchor/Diff/Stream/Caps/Error）
    pub fn create_full()    -> Self;            // NCP + NWP + NIP + NDP + NOP
}
```

`FrameRegistry::default()` 返回一个**空**注册表 —— 若与编解码器配合
使用 `FrameRegistry::default()`，每次 `decode` 都会以
`"unregistered frame type …"` 失败。建议使用 `create_default()` 或
`create_full()`。

---

## `AnchorFrameCache`

构造上线程安全（非 `Sync`：共享可变时使用 `Arc<Mutex<_>>`）的
anchor-schema 缓存，带惰性 TTL 过期。

```rust
pub struct AnchorFrameCache {
    pub clock: Box<dyn Fn() -> Instant + Send + Sync>,   // 测试中替换
    // …
}

impl AnchorFrameCache {
    pub fn new() -> Self;

    /// `schema` 的规范（键排序）JSON 的 SHA-256，前缀 `sha256:`。
    pub fn compute_anchor_id(schema: &Map<String, Value>) -> String;

    pub fn set(&mut self, schema: Map<String, Value>, ttl_secs: u64)
                 -> NpsResult<String>;                  // → anchor_id
    pub fn get(&self, anchor_id: &str)          -> Option<&Map<String, Value>>;
    pub fn get_required(&self, anchor_id: &str) -> NpsResult<&Map<String, Value>>;

    pub fn invalidate(&mut self, anchor_id: &str);
    pub fn evict_expired(&mut self);

    pub fn len(&self)      -> usize;           // 仅活跃条目
    pub fn is_empty(&self) -> bool;
}
```

### 投毒

当同一 `anchor_id` 已以**不同** schema 缓存（且仍活跃）时，`set`
返回 `NpsError::AnchorPoison`。以相同 schema 重新插入仅刷新 TTL。

### 惰性过期

`get` / `get_required` / `len` / `is_empty` 按 `expires > now` 过滤，
不改动存储。调用 `evict_expired()` 实际释放内存。

### 可注入 clock

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

`NpsError: Clone + Debug + Display + std::error::Error`。
