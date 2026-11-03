# Security Overview

Treat decoded network and file data as untrusted. Review unsafe code, allocation bounds, crypto, identity, dependencies, logs, and publishing.

## Review rules

- Validate external input and fail closed.
- Keep secrets out of source, fixtures, logs, and errors.
- Bound network, parser, file, memory, and concurrency work where applicable.
- Pin and review automation dependencies.
- Add focused tests for changed security behavior.
- Follow [SECURITY.md](../../SECURITY.md) for reports.
