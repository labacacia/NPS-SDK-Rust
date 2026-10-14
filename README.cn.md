[English Version](./README.md) | 中文版

# NPS Rust SDK (`nps-rs`)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Release](https://img.shields.io/badge/release-v1.0.0--alpha.18-orange.svg)](CHANGELOG.cn.md)
[![NCP](https://img.shields.io/badge/NCP-v0.11-5b8cff.svg)]()
[![NWP](https://img.shields.io/badge/NWP-v0.21-4af0b0.svg)]()
[![NIP](https://img.shields.io/badge/NIP-v0.14-7b61ff.svg)]()
[![NDP](https://img.shields.io/badge/NDP-v0.12-f0a050.svg)]()
[![NOP](https://img.shields.io/badge/NOP-v0.9-ff8c42.svg)]()

面向 **Neural Protocol Suite (NPS)** 的 Rust 客户端库 —— 为 AI Agent 与模型设计的完整互联网协议栈。

Crate 命名空间：`com.labacacia.nps` | Rust edition 2021 | Cargo workspace

## 状态

**v1.0.0-alpha.19 发布候选版 —— 可移植协议一致性**

覆盖 NCP + NWP + NIP + NDP + NOP 全部五个协议，外加完整的 **NPS-RFC-0002 X.509 + ACME `agent-01` NID 证书原语**（`nps_nip::x509` + `nps_nip::acme`）。

完整 workspace 测试与 Alpha.18 共享一致性 fixture 均通过。

Alpha.14 候选新增：远程 NIP CA 类型化客户端（`nps_nip::ca_client::NipCaClient`）、native-mode NWP 服务端 helper（`nps_nwp::NwpNativeNodeServer`）和 TC-N1/TC-N2 一致性 manifest helper（`nps_conformance`，并由 `nps-sdk` re-export）。

## Alpha.18 可移植 Profile

- NCP 0.11 有界原生服务握手与确定性 Caps 协商。
- NWP 0.21 可移植 Node/Bridge serving 与 Bridge 生命周期。
- NIP 0.14 可移植 CA、实时吊销、签名 CRL 与验证策略。
- NDP 0.12 签名 Announce 准入及 registry 冲突/liveness 策略。
- NOP 0.9 确定性编排、callback 安全、委派、租约与 CR-0007 runtime 决策。

五个 Profile 均消费 [`spec/conformance`](spec/conformance/) 下的语言无关 fixture。

### 早期新增（nps-nip）

- `nps_nip::x509` —— `issue_leaf` / `issue_root` / `verify`（基于 `rcgen` + `x509-parser` + `ed25519-dalek`）。
- `nps_nip::acme` —— `AcmeClient`（reqwest 异步） + 进程内 `AcmeServer`（tiny_http） + JWS / messages helpers（RFC 8555 + RFC 8037 EdDSA）。
- `IdentFrame` 扩展非破坏性可选字段 `assurance_level` / `cert_format` / `cert_chain`；v1 verifier 忽略新字段。

## 环境要求

- Rust stable（1.70+）
- Cargo

## 构建与测试

```bash
# 运行全部测试
cargo test --workspace

# 构建全部 crate
cargo build --workspace

# Release 构建
cargo build --workspace --release
```

## Workspace Crates

| Crate | 说明 |
|-------|------|
| `nps-core`  | 帧头、编解码器（Tier-1 JSON / Tier-2 MsgPack / Tier-3 BinaryVector）、帧注册表、anchor 缓存、错误类型 |
| `nps-ncp`   | NCP 帧：`AnchorFrame`、`DiffFrame`、`StreamFrame`、`CapsFrame`、`HelloFrame`、`ErrorFrame` |
| `nps-nwp`   | NWP 帧：`QueryFrame`、`ActionFrame`、`AsyncActionResponse`；异步 `NwpClient`（reqwest）；`NwpNativeNodeServer` native 服务端 |
| `nps-nip`   | NIP 帧：`IdentFrame`、`TrustFrame`、`RevokeFrame`；`NipIdentity`（Ed25519 密钥管理） |
| `nps-ndp`   | NDP 帧：`AnnounceFrame`、`ResolveFrame`、`GraphFrame`；`InMemoryNdpRegistry`；`NdpAnnounceValidator` |
| `nps-nop`   | NOP 帧：`TaskFrame`、`DelegateFrame`、`SyncFrame`、`AlignStreamFrame`；`BackoffStrategy`；`NopClient` |
| `nps-conformance` | TC-N1/TC-N2 一致性用例目录、manifest 构造器和校验器 |
| `nps-sdk`   | 统一伞型 crate —— 所有协议挂在 `nps_sdk::` 命名空间下 |

## 快速开始

添加到 `Cargo.toml`：

```toml
[dependencies]
nps-sdk = { path = "impl/rust/nps-sdk" }
tokio   = { version = "1", features = ["rt-multi-thread", "macros"] }
```

### NCP 帧编解码

```rust
use nps_core::codec::NpsFrameCodec;
use nps_core::frames::EncodingTier;
use nps_core::registry::FrameRegistry;
use nps_ncp::AnchorFrame;
use serde_json::json;

let codec = NpsFrameCodec::new(FrameRegistry::create_full());

let mut schema = serde_json::Map::new();
schema.insert("fields".into(), json!([{"name": "id", "type": "uint64"}]));

let frame = AnchorFrame {
    anchor_id: "sha256:abc123".into(),
    schema,
    namespace:   None,
    description: None,
    node_type:   None,
    ttl:         3600,
};

let wire = codec.encode(AnchorFrame::frame_type(), &frame.to_dict(), EncodingTier::MsgPack, true)?;
let (frame_type, dict) = codec.decode(&wire)?;
let back = AnchorFrame::from_dict(&dict)?;
```

### NWP 客户端 —— 查询

```rust
use nps_nwp::{NwpClient, QueryFrame};

let client = NwpClient::new("http://node.example.com:17433");
let query  = QueryFrame::new("sha256:abc123");
let caps   = client.query(&query).await?;
```

### NWP 客户端 —— 流式

```rust
let frames = client.stream(&query).await?;
for sf in &frames {
    println!("{:?}", sf.payload);
    if sf.is_last { break; }
}
```

### NWP Native 服务端

```rust
use nps_ncp::CapsFrame;
use nps_nwp::NwpNativeNodeServer;
use serde_json::json;

let server = NwpNativeNodeServer::new()
    .with_query_handler(|_| Ok(CapsFrame::new("native:orders", vec![json!({ "id": 42 })])))
    .with_action_handler(|action| Ok(json!({ "action": action.action })));

// `stream` 已完成 NCP preamble、TLS 和 Hello negotiation。
server.serve_stream(&mut stream).await?;
```

### NIP 身份 —— 签名 & 验签

```rust
use nps_nip::identity::NipIdentity;
use std::path::Path;

// 生成密钥对
let identity = NipIdentity::generate();
println!("{}", identity.pub_key_string()); // "ed25519:<hex>"

// 对 payload 签名
let mut payload = serde_json::Map::new();
payload.insert("nid".into(), serde_json::json!("urn:nps:node:example.com:data"));
let sig = identity.sign(&payload);  // "ed25519:<base64>"
let ok  = identity.verify(&payload, &sig); // true

// 持久化与加载（AES-256-GCM + PBKDF2）
identity.save(Path::new("my-node.key"), "my-passphrase")?;
let loaded = NipIdentity::load(Path::new("my-node.key"), "my-passphrase")?;
```

### NIP 远程 CA Client

```rust
use nps_nip::ca_client::{NipCaClient, NipCaRegisterRequest};

let ca = NipCaClient::with_client("https://ca.example.com", "/nip", reqwest::Client::new());
let discovery = ca.get_discovery().await?;
let ident = ca.register_agent(
    &NipCaRegisterRequest {
        identifier: "agent-a".into(),
        pub_key: "ed25519:<pub>".into(),
        capabilities: vec!["nwp:query".into()],
        scope_json: None,
        metadata_json: None,
    },
    Some("token"),
).await?;
let status = ca.verify_agent(&ident.nid).await?;
```

### 一致性 Manifest

```rust
use nps_conformance::{
    catalog_for_profile, validate_manifest, NpsConformanceCaseResult,
    NpsConformanceManifest, NODE_L1,
};

let results = catalog_for_profile(NODE_L1)?
    .iter()
    .map(|case| NpsConformanceCaseResult {
        id: case.id.to_string(),
        result: "pass".to_string(),
        message: None,
    })
    .collect();
let manifest = NpsConformanceManifest::create(
    NODE_L1,
    "my-node",
    "1.0.0-alpha.19",
    "urn:nps:node:example.com:my-node",
    "labacacia-fixture",
    "1.0.0-alpha.19",
    results,
    "ci",
);
let validation = validate_manifest(&manifest);
```

### NDP 注册表 —— announce & resolve

```rust
use nps_ndp::{AnnounceFrame, InMemoryNdpRegistry};
use nps_nip::identity::NipIdentity;
use serde_json::json;

let identity = NipIdentity::generate();
let mut addrs = serde_json::Map::new();
addrs.insert("host".into(),     json!("example.com"));
addrs.insert("port".into(),     json!(17433));
addrs.insert("protocol".into(), json!("nwp"));

let tmp = AnnounceFrame {
    nid: "urn:nps:node:example.com:data".into(),
    addresses: vec![addrs.clone()],
    caps: vec!["nwp/query".into()],
    ttl: 300,
    timestamp: "2026-01-01T00:00:00Z".into(),
    signature: "placeholder".into(),
    node_type: None,
};
let sig   = identity.sign(&tmp.unsigned_dict());
let frame = AnnounceFrame { signature: sig, ..tmp };

let mut registry = InMemoryNdpRegistry::new();
registry.announce(frame);

let result = registry.resolve("nwp://example.com/data/sub").unwrap();
// result.host == "example.com", result.port == 17433
```

### NDP Announce 校验器

```rust
use nps_ndp::NdpAnnounceValidator;

let mut validator = NdpAnnounceValidator::new();
validator.register_public_key("urn:nps:node:example.com:data", identity.pub_key_string());

let result = validator.validate(&frame);
if result.is_valid {
    println!("Announce accepted");
} else {
    println!("Rejected: {} — {}", result.error_code.unwrap(), result.message.unwrap());
}
```

### NOP —— 退避策略

```rust
use nps_nop::models::BackoffStrategy;

let delay_ms = BackoffStrategy::Exponential.compute_delay_ms(1000, 30_000, 2);
// 返回 4000（2^2 * 1000），受 max_ms 上限约束
```

## 帧类型对照

| 帧 | 类型码 | 协议 | 说明 |
|----|--------|------|------|
| `AnchorFrame`      | 0x01 | NCP | Schema anchor（缓存的 schema 定义） |
| `DiffFrame`        | 0x02 | NCP | Schema diff / patch |
| `StreamFrame`      | 0x03 | NCP | 流式数据块（is_last = final） |
| `CapsFrame`        | 0x04 | NCP | Capability 公告 |
| `HelloFrame`       | 0x06 | NCP | 原生模式握手（客户端 → 节点，JSON） |
| `ErrorFrame`       | 0xFE | NCP | 统一错误帧（所有协议共用） |
| `QueryFrame`       | 0x10 | NWP | 携带 anchor_ref + 过滤条件的数据查询 |
| `ActionFrame`      | 0x11 | NWP | Action 调用（同步或异步） |
| `IdentFrame`       | 0x20 | NIP | 节点身份声明（已签名） |
| `TrustFrame`       | 0x21 | NIP | 节点间信任委托 |
| `RevokeFrame`      | 0x22 | NIP | 吊销通知 |
| `AnnounceFrame`    | 0x30 | NDP | 节点公告（含 TTL） |
| `ResolveFrame`     | 0x31 | NDP | 地址解析请求 / 响应 |
| `GraphFrame`       | 0x32 | NDP | 网络拓扑快照 |
| `TaskFrame`        | 0x40 | NOP | 编排 DAG 任务 |
| `DelegateFrame`    | 0x41 | NOP | 子任务委托 |
| `SyncFrame`        | 0x42 | NOP | K-of-N 同步屏障 |
| `AlignStreamFrame` | 0x43 | NOP | 流式对齐更新 |

## 编码

| Tier | Variant | 说明 |
|------|---------|------|
| Tier-1 | `EncodingTier::Json`    | 可读 JSON（调试 / 互操作） |
| Tier-2 | `EncodingTier::MsgPack` | MsgPack 二进制（默认，约 60% 压缩） |
| Tier-3 | `EncodingTier::BinaryVector` | `binary_vector.v1`，用于向量密集型帧 |

## 错误处理

所有操作返回 `NpsResult<T>` = `Result<T, NpsError>`。

| 变体 | 触发 |
|------|------|
| `NpsError::Frame(msg)` | 未知帧类型、字段非法 |
| `NpsError::Codec(msg)` | 编解码失败、payload 过大 |
| `NpsError::AnchorNotFound(id)` | `get_required()` 请求缺失/过期 anchor |
| `NpsError::AnchorPoison(id)` | 尝试用不同 schema 覆盖已缓存 anchor |
| `NpsError::Identity(msg)` | 密钥生成、签名/验签、save/load 失败 |
| `NpsError::Io(msg)` | 网络或文件系统错误 |

## Feature Flags（`nps-sdk`）

| Feature | 默认 | 说明 |
|---------|------|------|
| `nwp`   | ✅ | 包含 NWP 帧和客户端 |
| `nip`   | ✅ | 包含 NIP 帧和身份 |
| `ndp`   | ✅ | 包含 NDP 帧、注册表、校验器 |
| `nop`   | ✅ | 包含 NOP 帧和客户端 |

## 测试

完整 workspace 共 755 个测试：

```bash
cargo test --workspace
```

## 许可证

[Apache 2.0](https://github.com/labacacia/NPS-Dev/blob/main/LICENSE) © 2026 INNO LOTUS PTY LTD
