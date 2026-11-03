[English Version](./overview.md) | 中文版

# NPS Rust SDK — API 参考

> Neural Protocol Suite 的异步 Rust SDK —— Rust stable（1.75+），基于 Tokio。

本目录是 `nps-sdk` 的类与方法参考。叙事性快速开始与端到端示例参见
[`sdk-usage.md`](./sdk-usage.md)（English）或
[`sdk-usage.cn.md`](./sdk-usage.cn.md)（中文）。随包 CA Server
参见 [`ca-server.cn.md`](./ca-server.cn.md)。

---

## Workspace crates

| Crate | 用途 | 参考文档 |
|-------|------|----------|
| `nps-core` | 帧头、编解码（Tier-1 JSON / Tier-2 MsgPack）、帧注册表、AnchorFrame 缓存、错误 | [`nps-rust.core.cn.md`](./nps-rust.core.cn.md) |
| `nps-ncp`  | NCP 帧 —— `AnchorFrame`、`DiffFrame`、`StreamFrame`、`CapsFrame`、`ErrorFrame` | [`nps-rust.ncp.cn.md`](./nps-rust.ncp.cn.md) |
| `nps-nwp`  | NWP 帧 + 异步 `NwpClient`（reqwest） | [`nps-rust.nwp.cn.md`](./nps-rust.nwp.cn.md) |
| `nps-nip`  | NIP 帧 + `NipIdentity`（Ed25519，AES-256-GCM 密钥存储） | [`nps-rust.nip.cn.md`](./nps-rust.nip.cn.md) |
| `nps-ndp`  | NDP 帧、`InMemoryNdpRegistry`、`NdpAnnounceValidator` | [`nps-rust.ndp.cn.md`](./nps-rust.ndp.cn.md) |
| `nps-nop`  | NOP 帧、`BackoffStrategy`、`NopTaskStatus`、异步 `NopClient` | [`nps-rust.nop.cn.md`](./nps-rust.nop.cn.md) |
| `nps-sdk`  | 元 crate —— 将所有内容以 `nps_sdk::{core, ncp, nwp, nip, ndp, nop}` 形式重新导出 | （facade —— 只做重新导出） |

---

## 安装

`Cargo.toml`：

```toml
[dependencies]
nps-sdk = "1.0.0-alpha.13"                             # 完整 facade
# —— 或按需挑选 crate：
nps-core = "1.0.0-alpha.13"
nps-ncp  = "1.0.0-alpha.13"
nps-nwp  = "1.0.0-alpha.13"
nps-nip  = "1.0.0-alpha.13"
nps-ndp  = "1.0.0-alpha.13"
nps-nop  = "1.0.0-alpha.13"
tokio    = { version = "1", features = ["full"] }     # nwp/nop 异步客户端需要
```

`nps-sdk` 通过 feature flag 重新导出协议 crates（`nwp`、
`nip`、`ndp`、`nop`）；`core` + `ncp` 始终被重新导出。

---

## 最小编解码示例

```rust
use nps_core::{FrameRegistry, NpsFrameCodec};
use nps_core::frames::{EncodingTier, FrameType};
use nps_ncp::AnchorFrame;

let codec = NpsFrameCodec::new(FrameRegistry::create_full());

let mut schema = serde_json::Map::new();
schema.insert("fields".into(), serde_json::json!([
    { "name": "id",    "type": "uint64"  },
    { "name": "price", "type": "decimal" },
]));
let frame = AnchorFrame {
    anchor_id:   "sha256:abc123".into(),
    schema,
    namespace:   None,
    description: None,
    node_type:   None,
    ttl:         3600,
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
```

编解码器是基于 dict 的：没有泛型的 `encode<T: NpsFrame>`
方法。每种帧类型都提供 `frame_type()`、`to_dict()` 和
`from_dict()` —— 在 `codec.encode` / `codec.decode` 前后
显式调用它们。

---

## 编码分层

| Tier | `EncodingTier` | 线缆标志（bit 7） | 备注 |
|------|----------------|--------------------|------|
| Tier-1 JSON    | `EncodingTier::Json`    | `0` | UTF-8 JSON，调试 / 互操作 |
| Tier-2 MsgPack | `EncodingTier::MsgPack` | `1` | `rmp-serde`（`to_vec_named`），生产默认 |

**Rust 标志字节布局**（与 Java / Python / .NET SDK 不同）：

| Bit | 掩码   | 含义 |
|-----|--------|------|
| 7   | `0x80` | TIER —— `1` = MsgPack，`0` = JSON |
| 6   | `0x40` | FINAL —— 流中最后一帧 |
| 0   | `0x01` | EXT —— 8 字节扩展帧头（负载 > 65 535 字节） |

帧头大小：默认 4 字节，`EXT = 1` 时为 8 字节
（`[type][flags][0][0][len_b3..len_b0]`）。最大负载默认为 10 MiB
（`nps_core::codec::DEFAULT_MAX_PAYLOAD`）—— 通过
`NpsFrameCodec::new(r).with_max_payload(n)` 调整。

---

## 异步 I/O

- `NwpClient`（`nps-nwp`）和 `NopClient`（`nps-nop`）都是 `async` 的，
  需要 Tokio 运行时。
- 所有可失败操作返回 `NpsResult<T>` = `Result<T, NpsError>`。
- 非 2xx HTTP 响应以 `NpsError::Io("NWP /{path} failed: HTTP …")` 形式
  暴露 —— 不会 panic。

---

## 错误类型

`NpsError`（来自 `nps-core`）：

| 变体 | 抛出场景 |
|------|----------|
| `Frame(String)`          | 未知帧类型 / 缺失字段 / 类型不匹配 |
| `Codec(String)`          | JSON 或 MsgPack 编解码失败、负载过大 |
| `AnchorNotFound(String)` | `AnchorFrameCache::get_required` 命中缺失 / 过期 AnchorFrame |
| `AnchorPoison(String)`   | `AnchorFrameCache::set` 相同 `anchor_id` 下 schema 不一致 |
| `Identity(String)`       | 密钥生成 / 保存 / 加载 / PBKDF2 / AES-GCM 失败 |
| `Io(String)`             | `reqwest` 网络错误、非 2xx HTTP、文件 I/O |

所有变体都实现了 `Display` 和 `std::error::Error`。

---

## 规范链接

- [NPS-0 Overview](https://github.com/labacacia/NPS/blob/main/spec/NPS-0-Overview.cn.md)
- [NPS-1 NCP](https://github.com/labacacia/NPS/blob/main/spec/NPS-1-NCP.cn.md)
- [NPS-2 NWP](https://github.com/labacacia/NPS/blob/main/spec/NPS-2-NWP.cn.md)
- [NPS-3 NIP](https://github.com/labacacia/NPS/blob/main/spec/NPS-3-NIP.cn.md)
- [NPS-4 NDP](https://github.com/labacacia/NPS/blob/main/spec/NPS-4-NDP.cn.md)
- [NPS-5 NOP](https://github.com/labacacia/NPS/blob/main/spec/NPS-5-NOP.cn.md)
- [帧注册表](https://github.com/labacacia/NPS/blob/main/spec/frame-registry.yaml)
- [错误码](https://github.com/labacacia/NPS/blob/main/spec/error-codes.cn.md)
