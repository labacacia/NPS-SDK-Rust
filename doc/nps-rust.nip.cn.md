[English Version](./nps-rust.nip.md) | 中文版

# `nps-nip` — 参考

> 规范：[NPS-3 NIP v0.9](https://github.com/labacacia/NPS/blob/main/spec/NPS-3-NIP.md)

身份层。三种帧 + 一个带 AES-256-GCM 加密磁盘持久化的 Ed25519 助手。

---

## 目录

- [`IdentFrame` (0x20)](#identframe-0x20)
- [`TrustFrame` (0x21)](#trustframe-0x21)
- [`RevokeFrame` (0x22)](#revokeframe-0x22)
- [`NipIdentity`](#nipidentity)
- [密钥文件格式](#密钥文件格式)

---

## `IdentFrame` (0x20)

节点身份声明。

```rust
pub struct IdentFrame {
    pub nid:       String,                            // "urn:nps:node:{authority}:{name}"
    pub pub_key:   String,                            // "ed25519:{hex}"
    pub meta:      Option<serde_json::Map<String, Value>>,
    pub signature: Option<String>,                    // "ed25519:{base64}"
}

impl IdentFrame {
    pub fn unsigned_dict(&self) -> FrameDict;   // 剥离 `signature`
    pub fn to_dict(&self)       -> FrameDict;
    pub fn from_dict(d: &FrameDict) -> NpsResult<Self>;
}
```

签名流程：

1. 以 `signature: None` 构造。
2. `identity.sign(&frame.unsigned_dict())` → `"ed25519:{base64}"`。
3. 编码前以 `signature = Some(sig)` 重建。

---

## `TrustFrame` (0x21)

委托 / 信任断言。

```rust
pub struct TrustFrame {
    pub issuer_nid:  String,
    pub subject_nid: String,
    pub scopes:      Vec<String>,
    pub expires_at:  Option<String>,   // ISO 8601 UTC
    pub signature:   Option<String>,   // "ed25519:{base64}"
}
```

签名约定相同：对去除 `signature` 字段后的 dict 做规范 JSON。

---

## `RevokeFrame` (0x22)

吊销一个 NID —— 先于或伴随 `ttl == 0` 的 `AnnounceFrame` 发送。

```rust
pub struct RevokeFrame {
    pub nid:        String,
    pub reason:     Option<String>,
    pub revoked_at: Option<String>,
}
```

---

## `NipIdentity`

Ed25519 密钥对加规范 JSON 签名 / 验签。

```rust
pub struct NipIdentity { /* … */ }

impl NipIdentity {
    pub fn generate() -> Self;

    pub fn pub_key_string(&self) -> String;                // "ed25519:{hex}"

    pub fn sign(&self, payload: &FrameDict) -> String;     // "ed25519:{base64}"
    pub fn verify(&self, payload: &FrameDict, signature: &str) -> bool;

    pub fn verify_with_key(&self, vk: VerifyingKey,
                           payload: &FrameDict, signature: &str) -> bool;

    /// 静态：解析 "ed25519:{hex}" 公钥并对 `payload` 验签。
    pub fn verify_with_pub_key_str(
        payload: &FrameDict, pub_key: &str, signature: &str) -> bool;

    pub fn save(&self, path: &Path, passphrase: &str) -> NpsResult<()>;
    pub fn load(path: &Path, passphrase: &str)        -> NpsResult<Self>;
}
```

### 规范签名 payload

`sign` 和 verify 系列均通过 `BTreeMap<String, Value>`（键排序）
经 `serde_json::to_string` 序列化 payload —— 无空白、键按字典序。
这与 .NET / Python / Java / TS SDK 共用的键排序规范化器一致；
**不使用 RFC 8785 JCS**。

### 验签

- `verify(&self, …)` 对实例自身公钥验签。
- `verify_with_key` 允许传入任意第三方 `VerifyingKey` 材料。
- `verify_with_pub_key_str` 是
  [`NdpAnnounceValidator`](./nps-rust.ndp.cn.md#ndpannouncevalidator)
  使用的独立助手 —— 解析 `"ed25519:{hex}"` → 32 字节公钥 → 验签。

所有 verify 函数在任何解析、长度或签名不匹配错误时返回 `false` ——
从不 panic。

---

## 密钥文件格式

`save` 写出加密的 JSON 信封：

```json
{
  "version":    1,
  "algorithm":  "ed25519",
  "pub_key":    "ed25519:<hex>",
  "salt":       "<hex 16 字节>",
  "nonce":      "<hex 12 字节>",
  "ciphertext": "<hex —— 32 字节 seed 的 AES-256-GCM 密文>"
}
```

| 参数 | 值 |
|------|-----|
| PBKDF2 算法     | `PBKDF2-HMAC-SHA256` (`hmac::Hmac<Sha256>`) |
| PBKDF2 迭代数   | 600 000 |
| 派生密钥        | 32 字节（256 位）|
| Salt            | 16 字节（随机，`OsRng`）|
| Nonce           | 12 字节（随机，`OsRng`）|
| 加密算法        | `Aes256Gcm`（`aes-gcm` crate）|
| 明文            | 原始 Ed25519 seed —— `SigningKey::to_bytes()` 的 32 字节 |

`load` 重新计算 PBKDF2 密钥并解密；口令错误表现为
`NpsError::Identity("decryption failed — wrong passphrase?")`。

> **跨 SDK 提示。** Rust 信封存储的是原始 32 字节 Ed25519 seed。
> Java SDK 存储 PKCS#8 / X.509 DER。两种格式**按字节不互通** ——
> 跨 SDK 互操作请通过 `pub_key_string()` + `sign` 输出，而不要尝试
> 加载另一 SDK 的密钥文件。

---

## 端到端

```rust
use nps_nip::NipIdentity;
use std::path::Path;

let id   = NipIdentity::generate();
let nid  = "urn:nps:node:api.example.com:products";

// 签名一个 payload
let mut payload = serde_json::Map::new();
payload.insert("action".into(), serde_json::json!("announce"));
payload.insert("nid".into(),    serde_json::json!(nid));
let sig  = id.sign(&payload);
assert!(id.verify(&payload, &sig));

// 跨密钥验签（例如经 NDP announce validator）
assert!(NipIdentity::verify_with_pub_key_str(&payload, &id.pub_key_string(), &sig));

// 加密持久化
id.save(Path::new("node.key"), "my-passphrase")?;
let loaded = NipIdentity::load(Path::new("node.key"), "my-passphrase")?;
assert_eq!(loaded.pub_key_string(), id.pub_key_string());
```
