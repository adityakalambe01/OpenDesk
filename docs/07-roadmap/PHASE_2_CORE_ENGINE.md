# OpenDesk Phase 2 — Rust Core Engine Design & Implementation

## 1. Phase Objective
Implement the platform-independent core abstraction traits, shared binary protocol serializers, cryptographic identity models, synthetic input definitions, and engine coordinator state machines in the Rust workspace (`core/crates/`).

## 2. Why This Phase Is Necessary
Decoupling the computational logic, protocol parsing, and hardware abstraction traits from platform-specific APIs ensures that all operating systems share identical business logic, error handling, and wire formats, preventing divergent implementations.

## 3. Scope
- Implementing `opendesk-proto`: binary serialization for control, input, clipboard, and file transfer messages using `bincode`.
- Implementing `opendesk-crypto`: Ed25519 device key generation, X25519 ECDH key agreement, and ChaCha20-Poly1305 symmetric AEAD envelope.
- Defining capture and encoder abstraction traits in `opendesk-capture` and `opendesk-encoder`.
- Defining synthetic input injection traits in `opendesk-input`.
- Implementing the `EngineCoordinator` state machine in `opendesk-engine`.
- Comprehensive unit test suite covering protocol parsing, fuzzing, and cryptographic derivation.

## 4. Prerequisites
Completion of Phase 1 (Cargo workspace setup).

## 5. Technical Implementation Details
- `#![forbid(unsafe_code)]` enforced in `opendesk-proto` and `opendesk-crypto`.
- Strongly-typed errors implemented via `thiserror`.
- Coordinate normalization using 64-bit IEEE floating-point numbers ($0.0 \le X, Y \le 1.0$).
- Token bucket rate limiter implementation for synthetic input streams.

## 6. Exact Folder and File Changes Expected
```
core/crates/
├── opendesk-proto/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── control.rs
│       ├── input.rs
│       ├── clipboard.rs
│       └── file.rs
├── opendesk-crypto/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── identity.rs
│       ├── e2ee.rs
│       └── hashing.rs
├── opendesk-capture/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── traits.rs
├── opendesk-encoder/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── traits.rs
├── opendesk-input/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── traits.rs
└── opendesk-engine/
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        └── coordinator.rs
```

## 7. Individual Development Tasks
- **P2-001**: Implement `opendesk-proto` data structures and binary bincode serialization.
- **P2-002**: Implement `opendesk-crypto` Ed25519 identity generation and signature attestation.
- **P2-003**: Implement `opendesk-crypto` X25519 ephemeral key exchange and ChaCha20-Poly1305 AEAD.
- **P2-004**: Define `ScreenCapturer` and `AudioCapturer` interfaces in `opendesk-capture`.
- **P2-005**: Define `VideoEncoder` and `AudioEncoder` interfaces in `opendesk-encoder`.
- **P2-006**: Define `InputInjector` interface and token bucket rate limiter in `opendesk-input`.
- **P2-007**: Implement `EngineCoordinator` lifecycle loop and event dispatcher in `opendesk-engine`.
- **P2-008**: Author comprehensive unit tests and property-based test suites.

## 8. Dependencies
Phase 1 (Repository setup).

## 9. Expected Outputs
A tested, modular Rust core engine exposing platform-independent interfaces and cryptographic primitives.

## 10. Unit Testing Requirements
- Unit tests verifying serialization roundtripping of every protocol message.
- Cryptographic test vectors verifying X25519 shared secret derivation and signature validation.
- Unit tests validating rate limiter behavior under high-frequency event bursts.

## 11. Integration Testing Requirements
Integration test in `opendesk-engine` verifying mock capturer $\rightarrow$ mock encoder $\rightarrow$ coordinator loop.

## 12. Manual Verification Steps
Run `cargo test --workspace` and verify that all test suites pass with zero warnings.

## 13. Acceptance Criteria
- Unit test code coverage $\ge 85\%$ across `opendesk-proto` and `opendesk-crypto`.
- 100% adherence to `#![forbid(unsafe_code)]` in protocol and crypto crates.
- All errors strongly typed with `thiserror`.

## 14. Definition of Done
All crate modules implemented, documented with Rustdoc, and passing all unit tests.

## 15. Known Risks
- Serialization performance overhead on low-power devices.
  - *Mitigation*: Using compact binary `bincode` serialization rather than JSON or XML.

## 16. Estimated Complexity
High (Rigorous systems programming and cryptographic engineering).

## 17. Next-Phase Dependencies
Unlocks **Phase 3 — First Working Desktop Connection (Windows)**.
