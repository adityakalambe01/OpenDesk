# P2-008 Core Rust Coverage Report

Measured on 2026-11-16 with `cargo llvm-cov --workspace --summary-only` using the workspace test suite.

## Results

| Core library crate | Region coverage | Line coverage | Result |
| --- | ---: | ---: | --- |
| `opendesk-capture` | 98.88% | 98.55% | Pass |
| `opendesk-crypto` | 94.39% | 93.96% | Pass |
| `opendesk-encoder` | 100.00% | 100.00% | Pass |
| `opendesk-engine` | 85.71% | 85.51% | Pass |
| `opendesk-input` | 98.29% | 98.65% | Pass |
| `opendesk-proto` | 92.22% | 94.58% | Pass |

The complete workspace measured **92.74% region coverage** and **93.48% line coverage**. The empty `opendesk-agent`, `opendesk-ipc`, and `opendesk-transport` libraries have no executable code to cover. The CLI and daemon still contain only scaffold `main` functions and are reported at 0%; they are not core library crates.

`opendesk-proto::serde_helpers` is 51.16% region-covered in isolation. The uncovered visitor branches are not selected by the workspace's bincode deserializer, which uses its byte-buffer and sequence interfaces; the protocol crate aggregate remains above the 85% target.

## Test coverage added

- Unit tests exercise capture error formatting, native surface ownership and downcasting, and shared CPU frame storage.
- Input tests cover rolling capacity, backend failures, and poisoned limiter locks.
- Encoder tests cover identifier validation and generated identifiers.
- Protocol tests use Proptest (128 cases each) to check timestamp and clipboard byte roundtrips, plus deterministic malformed-frame and edge-case properties. Crypto tests use Proptest (64 cases) for generated payload roundtrips and associated-data authentication, with additional transcript, nonce replay, and RFC 8032 tests. Proptest is pinned to 1.6.0 to remain compatible with the workspace's Rust 1.82 minimum.

## Verification

- `cargo test --workspace --locked` — passed; 40 unit tests across the workspace, no failures.
- `cargo +1.82.0 test --workspace --locked` — passed, including both Proptest suites and the engine crate.
- `cargo llvm-cov --workspace --summary-only` — passed; all six core library crates meet the 85% line coverage target.
- `cargo fmt --all` — passed.
