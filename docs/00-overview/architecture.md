# Architecture

## Purpose and lifecycle

Public Rust workspace projection; canonical source is in NPS-Dev.

## Boundaries

nps-core/: primitives.

nps-ncp/, nps-ndp/, nps-nip/, nps-nop/, nps-nwp/: protocols.

nps-sdk/: facade.

nps-conformance/: evidence.

Keep responsibilities and dependency direction explicit. Do not add layers or cross-repository sources of truth without a concrete requirement. Record architecture-significant decisions under `docs/04-decisions/`.
