English | [中文版](./nps-rust.nip.cn.md)

# `nps-nip` — Reference

> Spec: [NPS-3 NIP v0.9](https://github.com/labacacia/NPS/blob/main/spec/NPS-3-NIP.md)

Identity layer. Three frames + an Ed25519 helper with
AES-256-GCM-encrypted on-disk persistence.

---

## Table of contents

- [`IdentFrame` (0x20)](#identframe-0x20)
- [`TrustFrame` (0x21)](#trustframe-0x21)
- [`RevokeFrame` (0x22)](#revokeframe-0x22)
- [`NipIdentity`](#nipidentity)
- [Key file format](#key-file-format)

---

## `IdentFrame` (0x20)

Node identity declaration.

```rust
pub struct IdentFrame {
    pub nid:       String,                            // "urn:nps:node:{authority}:{name}"
    pub pub_key:   String,                            // "ed25519:{hex}"
    pub meta:      Option<serde_json::Map<String, Value>>,
    pub signature: Option<String>,                    // "ed25519:{base64}"
}

impl IdentFrame {
    pub fn unsigned_dict(&self) -> FrameDict;   // strips `signature`
    pub fn to_dict(&self)       -> FrameDict;
    pub fn from_dict(d: &FrameDict) -> NpsResult<Self>;
}
```

Signing workflow:

1. Construct with `signature: None`.
2. `identity.sign(&frame.unsigned_dict())` → `"ed25519:{base64}"`.
3. Rebuild with `signature = Some(sig)` before encoding.

---

## `TrustFrame` (0x21)

Delegation / trust assertion.

```rust
pub struct TrustFrame {
    pub issuer_nid:  String,
    pub subject_nid: String,
    pub scopes:      Vec<String>,
    pub expires_at:  Option<String>,   // ISO 8601 UTC
    pub signature:   Option<String>,   // "ed25519:{base64}"
}
```

Same signing convention: canonical-JSON of the dict minus the
`signature` field.

---

## `RevokeFrame` (0x22)

Revokes an NID — precede or accompany an `AnnounceFrame` with
`ttl == 0`.

```rust
pub struct RevokeFrame {
    pub nid:        String,
    pub reason:     Option<String>,
    pub revoked_at: Option<String>,
}
```

---

## `NipIdentity`

Ed25519 keypair plus canonical-JSON sign / verify.

```rust
pub struct NipIdentity { /* … */ }

impl NipIdentity {
    pub fn generate() -> Self;

    pub fn pub_key_string(&self) -> String;                // "ed25519:{hex}"

    pub fn sign(&self, payload: &FrameDict) -> String;     // "ed25519:{base64}"
    pub fn verify(&self, payload: &FrameDict, signature: &str) -> bool;

    pub fn verify_with_key(&self, vk: VerifyingKey,
                           payload: &FrameDict, signature: &str) -> bool;

    /// Static: parse an "ed25519:{hex}" public key and verify against `payload`.
    pub fn verify_with_pub_key_str(
        payload: &FrameDict, pub_key: &str, signature: &str) -> bool;

    pub fn save(&self, path: &Path, passphrase: &str) -> NpsResult<()>;
    pub fn load(path: &Path, passphrase: &str)        -> NpsResult<Self>;
}
```

### Canonical signing payload

Both `sign` and the verify family serialise the payload through a
`BTreeMap<String, Value>` (sorted-keys) via `serde_json::to_string` —
no whitespace, lexical key order. This matches the sorted-keys
canonicaliser shared with the .NET / Python / Java / TS SDKs;
**RFC 8785 JCS is NOT used**.

### Verification

- `verify(&self, …)` verifies against the instance's own public key.
- `verify_with_key` lets you supply a `VerifyingKey` for any third-party
  key material.
- `verify_with_pub_key_str` is the free-standing helper used by
  [`NdpAnnounceValidator`](./nps-rust.ndp.md#ndpannouncevalidator) — it
  parses `"ed25519:{hex}"` → 32-byte public key → verifies.

All verify functions return `false` on any parsing, length, or signature
mismatch error — they never panic.

---

## Key file format

`save` writes an encrypted JSON envelope:

```json
{
  "version":    1,
  "algorithm":  "ed25519",
  "pub_key":    "ed25519:<hex>",
  "salt":       "<hex 16 bytes>",
  "nonce":      "<hex 12 bytes>",
  "ciphertext": "<hex — AES-256-GCM of the 32-byte seed>"
}
```

| Parameter | Value |
|-----------|-------|
| PBKDF2 algorithm  | `PBKDF2-HMAC-SHA256` (`hmac::Hmac<Sha256>`) |
| PBKDF2 iterations | 600 000 |
| Derived key       | 32 bytes (256-bit) |
| Salt              | 16 bytes (random, `OsRng`) |
| Nonce             | 12 bytes (random, `OsRng`) |
| Cipher            | `Aes256Gcm` (`aes-gcm` crate) |
| Plaintext         | Raw Ed25519 seed — 32 bytes from `SigningKey::to_bytes()` |

`load` recomputes the PBKDF2 key and decrypts; a wrong passphrase
surfaces as `NpsError::Identity("decryption failed — wrong passphrase?")`.

> **Cross-SDK note.** The Rust envelope stores the raw 32-byte Ed25519
> seed. The Java SDK stores PKCS#8 / X.509 DER. The two formats are
> **not** interchangeable byte-for-byte — use `pub_key_string()` +
> `sign` output for cross-SDK interop instead of loading another SDK's
> key file.

---

## End-to-end

```rust
use nps_nip::NipIdentity;
use std::path::Path;

let id   = NipIdentity::generate();
let nid  = "urn:nps:node:api.example.com:products";

// Sign a payload
let mut payload = serde_json::Map::new();
payload.insert("action".into(), serde_json::json!("announce"));
payload.insert("nid".into(),    serde_json::json!(nid));
let sig  = id.sign(&payload);
assert!(id.verify(&payload, &sig));

// Cross-key verification (e.g. via NDP announce validator)
assert!(NipIdentity::verify_with_pub_key_str(&payload, &id.pub_key_string(), &sig));

// Encrypted persistence
id.save(Path::new("node.key"), "my-passphrase")?;
let loaded = NipIdentity::load(Path::new("node.key"), "my-passphrase")?;
assert_eq!(loaded.pub_key_string(), id.pub_key_string());
```
