# NIP CA Server — Rust

Axum + SQLite implementation of the NIP Certificate Authority (NPS-3 §8).

## Quick Start

```bash
docker compose up -d
```

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `NIP_CA_NID` | Yes | — | CA NID |
| `NIP_CA_PASSPHRASE` | Yes | — | Key file passphrase |
| `NIP_CA_BASE_URL` | Yes | — | Public base URL |
| `NIP_CA_DISPLAY_NAME` | No | `NPS CA` | |
| `NIP_CA_KEY_FILE` | No | `/data/ca.key.enc` | |
| `NIP_CA_DB_PATH` | No | `/data/ca.db` | |
| `NIP_CA_AGENT_VALIDITY_DAYS` | No | `30` | |
| `NIP_CA_NODE_VALIDITY_DAYS` | No | `90` | |
| `NIP_CA_RENEWAL_WINDOW_DAYS` | No | `7` | |
| `PORT` | No | `17440` | |

## API

Same endpoints as all other NIP CA Server implementations — see [NPS-3 §8](https://github.com/labacacia/NPS/blob/main/spec/NPS-3-NIP.md).

## Local Development

```bash
NIP_CA_NID=urn:nps:org:ca.local \
  NIP_CA_PASSPHRASE=dev-pass \
  NIP_CA_BASE_URL=http://localhost:17440 \
  cargo run
```

## Stack

- **Runtime**: Rust stable
- **Framework**: Axum 0.8 + Tokio
- **Crypto**: ed25519-dalek 2 + aes-gcm + pbkdf2 + sha2
- **Storage**: SQLite via rusqlite (bundled)
