# OpenDesk — Master Implementation Task List (MASTER_TODO)

This document tracks all implementation tasks across the entire OpenDesk roadmap. Every task contains a unique identifier, priority level, explicit dependencies, expected outputs, acceptance criteria, and technical documentation references.

---

## Phase 0 — Architecture & Project Foundation

- [x] **P0-001** — Analyze product requirements, access modes, and success criteria
  - **Priority**: P0 (Blocker)
  - **Dependencies**: None
  - **Expected Output**: Approved PRD and Project Vision documents
  - **Acceptance Criteria**: Functional, non-functional, and platform matrix requirements fully documented without ambiguity
  - **Documentation Reference**: `docs/00-project-overview/PRODUCT_REQUIREMENTS.md`

- [x] **P0-002** — Finalize system architecture, process separation, and component boundaries
  - **Priority**: P0 (Blocker)
  - **Dependencies**: P0-001
  - **Expected Output**: System architecture specification and Mermaid topologies
  - **Acceptance Criteria**: Decoupled 3-tier host process model, WebRTC data plane, and SignalR control plane fully specified
  - **Documentation Reference**: `docs/01-architecture/SYSTEM_ARCHITECTURE.md`

- [x] **P0-003** — Research platform APIs (Windows, Linux, macOS, Android, iOS)
  - **Priority**: P0 (Blocker)
  - **Dependencies**: P0-001
  - **Expected Output**: Platform technical capability matrix and limitation registry
  - **Acceptance Criteria**: Concrete APIs (WGC, SCKit, PipeWire, MediaProjection) documented; OS limitations explicitly recorded
  - **Documentation Reference**: `docs/01-architecture/PLATFORM_ARCHITECTURE.md`

- [x] **P0-004** — Formulate Architectural Decision Records (ADRs)
  - **Priority**: P0 (Blocker)
  - **Dependencies**: P0-002
  - **Expected Output**: 10 approved ADR records with options and rationale
  - **Acceptance Criteria**: Language, UI framework, backend, and protocol choices justified with tradeoffs
  - **Documentation Reference**: `docs/01-architecture/ARCHITECTURE_DECISIONS.md`

- [x] **P0-005** — Author STRIDE threat model and zero-knowledge relay specifications
  - **Priority**: P0 (Blocker)
  - **Dependencies**: P0-002
  - **Expected Output**: Comprehensive STRIDE security analysis and mitigation matrix
  - **Acceptance Criteria**: Threat vectors analyzed with concrete algorithmic and architecture mitigations
  - **Documentation Reference**: `docs/03-security/THREAT_MODEL.md`

- [x] **P0-006** — Define shared binary and signaling wire protocol contracts
  - **Priority**: P0 (Blocker)
  - **Dependencies**: P0-002
  - **Expected Output**: Binary protocol specification and channel multiplexing layout
  - **Acceptance Criteria**: Packet headers, control, input, clipboard, and file transfer messages defined with exact binary schemas
  - **Documentation Reference**: `docs/02-technical-design/SHARED_PROTOCOL.md`

- [x] **P0-007** — Design PostgreSQL 17 relational schema and Redis caching key structures
  - **Priority**: P0 (Blocker)
  - **Dependencies**: P0-002
  - **Expected Output**: Relational SQL DDL, indexes, and Redis key architecture
  - **Acceptance Criteria**: Tables for Organizations, Users, Devices, Sessions, and Audit Logs fully specified with constraints
  - **Documentation Reference**: `docs/02-technical-design/DATABASE_DESIGN.md`

- [x] **P0-008** — Formulate Docker Compose and Coturn relay configurations
  - **Priority**: P0 (Blocker)
  - **Dependencies**: P0-002
  - **Expected Output**: Turnkey Docker Compose topology and Coturn configuration file
  - **Acceptance Criteria**: All services, volumes, networks, and environment variables defined
  - **Documentation Reference**: `docs/04-infrastructure/DOCKER_ARCHITECTURE.md`

- [x] **P0-009** — Establish engineering standards, CI/CD matrices, and test strategies
  - **Priority**: P0 (Blocker)
  - **Dependencies**: P0-002
  - **Expected Output**: Coding style guides, test pyramid, and GitHub Actions workflows
  - **Acceptance Criteria**: Linters, formatters, commit rules, and CI matrices documented for Rust, C#, TypeScript, and Dart
  - **Documentation Reference**: `docs/06-development/CODING_STANDARDS.md`

- [x] **P0-010** — Compile Master Implementation Roadmap and Tracking Checklists
  - **Priority**: P0 (Blocker)
  - **Dependencies**: P0-001 through P0-009
  - **Expected Output**: Complete phase specifications and task trackers
  - **Acceptance Criteria**: All 11 phases authored with 17 mandatory sections and tracking files
  - **Documentation Reference**: `docs/07-roadmap/MASTER_ROADMAP.md`

---

## Phase 1 — Repository & Development Environment Setup

- [x] **P1-001** — Configure root `.gitignore`, `.editorconfig`, and monorepo files
  - **Priority**: P1 (High)
  - **Dependencies**: Phase 0 completion
  - **Expected Output**: Root repository configuration files
  - **Acceptance Criteria**: Clean git hygiene; ignores build artifacts across Rust, .NET, Node, and Flutter
  - **Documentation Reference**: `docs/07-roadmap/PHASE_1_REPOSITORY_SETUP.md`

- [x] **P1-002** — Initialize root `Cargo.toml` and scaffold core Rust crates
  - **Priority**: P1 (High)
  - **Dependencies**: P1-001
  - **Expected Output**: Buildable Rust workspace with 11 member crates
  - **Acceptance Criteria**: `cargo check --workspace` succeeds with zero errors
  - **Documentation Reference**: `docs/02-technical-design/RUST_CORE_ENGINE.md`

- [x] **P1-003** — Scaffold ASP.NET Core 10 backend solution and projects
  - **Priority**: P1 (High)
  - **Dependencies**: P1-001
  - **Expected Output**: `OpenDesk.sln` with Clean Architecture projects
  - **Acceptance Criteria**: `dotnet build` succeeds with zero errors and zero warnings
  - **Documentation Reference**: `docs/02-technical-design/BACKEND_ARCHITECTURE.md`

- [x] **P1-004** — Scaffold Tauri v2 + React 19 desktop client under `clients/desktop`
  - **Priority**: P1 (High)
  - **Dependencies**: P1-001, P1-002
  - **Expected Output**: Buildable Tauri desktop project
  - **Acceptance Criteria**: `pnpm build` compiles React frontend and links to Tauri Rust backend
  - **Documentation Reference**: `docs/02-technical-design/DESKTOP_APPLICATION.md`

- [x] **P1-005** — Scaffold Flutter 3.x mobile client under `clients/mobile`
  - **Priority**: P2 (Medium)
  - **Dependencies**: P1-001
  - **Expected Output**: Buildable Flutter project skeleton
  - **Acceptance Criteria**: `flutter analyze` passes with zero issues
  - **Documentation Reference**: `docs/02-technical-design/MOBILE_APPLICATION.md`

- [x] **P1-006** — Configure local development `docker-compose.dev.yml` (Postgres 17 + Redis 7.4)
  - **Priority**: P1 (High)
  - **Dependencies**: P1-001
  - **Expected Output**: Running database and cache containers
  - **Acceptance Criteria**: Containers are healthy on internal ports 5432 and 6379 and expose configurable loopback host ports (defaults 5433 and 6380)
  - **Documentation Reference**: `docs/04-infrastructure/DOCKER_ARCHITECTURE.md`

- [x] **P1-007** — Configure GitHub Actions CI workflow matrix (`.github/workflows/ci.yml`)
  - **Priority**: P1 (High)
  - **Dependencies**: P1-002, P1-003, P1-004
  - **Expected Output**: Automated CI testing on Windows, Linux, and macOS
  - **Acceptance Criteria**: CI triggers on PRs and passes all matrix jobs
  - **Documentation Reference**: `docs/04-infrastructure/CI_CD.md`

- [x] **P1-008** — Verify build and test execution across all projects
  - **Priority**: P1 (High)
  - **Dependencies**: P1-001 through P1-007
  - **Expected Output**: Verified local build scripts
  - **Acceptance Criteria**: Full workspace builds and tests pass cleanly
  - **Documentation Reference**: `docs/06-development/DEVELOPMENT_SETUP.md`

---

## Phase 2 — Rust Core Engine Design & Implementation

- [x] **P2-001** — Implement `opendesk-proto` data structures and binary bincode serialization
  - **Priority**: P1 (High)
  - **Dependencies**: P1-002
  - **Expected Output**: Strongly-typed protocol structs with serialization tests
  - **Acceptance Criteria**: Roundtrip serialization passes; 100% test coverage on message types
  - **Documentation Reference**: `docs/02-technical-design/SHARED_PROTOCOL.md`

- [x] **P2-002** — Implement `opendesk-crypto` Ed25519 identity generation and signature attestation
  - **Priority**: P1 (High)
  - **Dependencies**: P1-002
  - **Expected Output**: Device keypair generator and signature verifier
  - **Acceptance Criteria**: Generates valid Ed25519 signatures; passes RFC 8032 test vectors
  - **Documentation Reference**: `docs/03-security/DEVICE_IDENTITY.md`

- [ ] **P2-003** — Implement `opendesk-crypto` X25519 ephemeral key exchange and ChaCha20-Poly1305 AEAD
  - **Priority**: P1 (High)
  - **Dependencies**: P1-002
  - **Expected Output**: E2EE envelope wrapper
  - **Acceptance Criteria**: Derives 32-byte shared key via HKDF-SHA256; encrypts/decrypts payloads
  - **Documentation Reference**: `docs/03-security/ENCRYPTION.md`

- [ ] **P2-004** — Define `ScreenCapturer` and `AudioCapturer` interfaces in `opendesk-capture`
  - **Priority**: P1 (High)
  - **Dependencies**: P1-002
  - **Expected Output**: Abstract capture traits and display enumeration types
  - **Acceptance Criteria**: Zero-copy surface representations defined for D3D11, IOSurface, and DmaBuf
  - **Documentation Reference**: `docs/02-technical-design/SCREEN_CAPTURE.md`

- [ ] **P2-005** — Define `VideoEncoder` and `AudioEncoder` interfaces in `opendesk-encoder`
  - **Priority**: P1 (High)
  - **Dependencies**: P1-002
  - **Expected Output**: Abstract video and audio codec traits
  - **Acceptance Criteria**: Support for bitrate updating, keyframe requests, and packetization
  - **Documentation Reference**: `docs/02-technical-design/VIDEO_AND_AUDIO.md`

- [ ] **P2-006** — Define `InputInjector` interface and token bucket rate limiter in `opendesk-input`
  - **Priority**: P1 (High)
  - **Dependencies**: P1-002
  - **Expected Output**: Input injection traits and rate limiter
  - **Acceptance Criteria**: Caps event injection to 1,000 events/sec; drops overflow with alerts
  - **Documentation Reference**: `docs/02-technical-design/INPUT_CONTROL.md`

- [ ] **P2-007** — Implement `EngineCoordinator` lifecycle loop and event dispatcher in `opendesk-engine`
  - **Priority**: P1 (High)
  - **Dependencies**: P2-001 through P2-006
  - **Expected Output**: Active session execution loop
  - **Acceptance Criteria**: Dispatches captured frames, polls encoders, and handles control signals
  - **Documentation Reference**: `docs/02-technical-design/RUST_CORE_ENGINE.md`

- [ ] **P2-008** — Author comprehensive unit tests and property-based test suites
  - **Priority**: P1 (High)
  - **Dependencies**: P2-001 through P2-007
  - **Expected Output**: Full test coverage report
  - **Acceptance Criteria**: Code coverage $\ge 85\%$ across core crates
  - **Documentation Reference**: `docs/06-development/TESTING_STRATEGY.md`

---

## Phase 3 — First Working Desktop Connection (Windows)

- [ ] **P3-001** — Implement Windows DXGI Desktop Duplication capture driver
  - **Priority**: P1 (High)
  - **Dependencies**: Phase 2
  - **Expected Output**: Functional DXGI frame grabber
  - **Acceptance Criteria**: Captures D3D11 textures at display refresh rate; handles mode switches
  - **Documentation Reference**: `docs/02-technical-design/SCREEN_CAPTURE.md`

- [ ] **P3-002** — Implement Windows Graphics Capture (WGC) modern capture driver
  - **Priority**: P1 (High)
  - **Dependencies**: Phase 2
  - **Expected Output**: WGC WinRT frame grabber
  - **Acceptance Criteria**: Disables yellow border (`IsBorderRequired = false`); captures cleanly
  - **Documentation Reference**: `docs/02-technical-design/SCREEN_CAPTURE.md`

- [ ] **P3-003** — Implement hardware H.264 video encoder integration (NVENC / QuickSync)
  - **Priority**: P1 (High)
  - **Dependencies**: Phase 2
  - **Expected Output**: Hardware video encoder with OpenH264 fallback
  - **Acceptance Criteria**: Encodes 1080p frames in $< 5\text{ ms}$; emits NAL units
  - **Documentation Reference**: `docs/02-technical-design/VIDEO_AND_AUDIO.md`

- [ ] **P3-004** — Implement Windows Win32 `SendInput` mouse and keyboard injection driver
  - **Priority**: P1 (High)
  - **Dependencies**: Phase 2
  - **Expected Output**: Win32 input injector
  - **Acceptance Criteria**: Injects mouse clicks, movements, and hardware scancodes
  - **Documentation Reference**: `docs/02-technical-design/INPUT_CONTROL.md`

- [ ] **P3-005** — Integrate `webrtc-rs` local PeerConnection pipeline with video track and data channel
  - **Priority**: P1 (High)
  - **Dependencies**: P3-001 through P3-004
  - **Expected Output**: Local WebRTC streaming pipeline
  - **Acceptance Criteria**: Streams video and exchanges input packets over local socket
  - **Documentation Reference**: `docs/02-technical-design/NETWORKING_AND_WEBRTC.md`

- [ ] **P3-006** — Implement React 19 Remote Viewer canvas component in Tauri
  - **Priority**: P1 (High)
  - **Dependencies**: P1-004, P3-005
  - **Expected Output**: Hardware canvas rendering incoming video
  - **Acceptance Criteria**: Renders decoded frames smoothly with aspect ratio scaling
  - **Documentation Reference**: `docs/02-technical-design/DESKTOP_APPLICATION.md`

- [ ] **P3-007** — Wire client mouse and keyboard event listeners to Tauri IPC `send_input` commands
  - **Priority**: P1 (High)
  - **Dependencies**: P3-004, P3-006
  - **Expected Output**: Interactive control from client to host
  - **Acceptance Criteria**: Mouse clicks on viewer canvas trigger matching actions on host
  - **Documentation Reference**: `docs/02-technical-design/INPUT_CONTROL.md`

- [ ] **P3-008** — Conduct end-to-end latency and frame rate verification over local test network
  - **Priority**: P1 (High)
  - **Dependencies**: P3-001 through P3-007
  - **Expected Output**: Performance benchmark report
  - **Acceptance Criteria**: Glass-to-glass latency $\le 30\text{ ms}$; framerate $\ge 50\text{ FPS}$ at 1080p
  - **Documentation Reference**: `docs/07-roadmap/PHASE_3_DESKTOP_REMOTE_ACCESS.md`

---

## Phase 4 — Networking, Signaling & NAT Traversal

- [ ] **P4-001** — Implement `SignalingHub` in ASP.NET Core with room routing and presence tracking
  - **Priority**: P1 (High)
  - **Dependencies**: Phase 3
  - **Expected Output**: Real-time SignalR WebSocket hub
  - **Acceptance Criteria**: Routes SDP offers, answers, and candidates between paired peers
  - **Documentation Reference**: `docs/05-api/SIGNALING_EVENTS.md`

- [ ] **P4-002** — Implement `CoturnTokenService` issuing RFC 5766 HMAC-SHA1 ephemeral credentials
  - **Priority**: P1 (High)
  - **Dependencies**: P4-001
  - **Expected Output**: Token generation service
  - **Acceptance Criteria**: Generates authenticated TURN credentials validated by Coturn
  - **Documentation Reference**: `docs/02-technical-design/BACKEND_ARCHITECTURE.md`

- [ ] **P4-003** — Implement Rust WebSocket signaling client communicating with SignalR hub
  - **Priority**: P1 (High)
  - **Dependencies**: P4-001
  - **Expected Output**: Asynchronous signaling client
  - **Acceptance Criteria**: Handles connection lifecycle, reconnects, and message dispatching
  - **Documentation Reference**: `docs/02-technical-design/NETWORKING_AND_WEBRTC.md`

- [ ] **P4-004** — Configure STUN candidate gathering in `webrtc-rs` transport layer
  - **Priority**: P1 (High)
  - **Dependencies**: P4-003
  - **Expected Output**: STUN discovery engine
  - **Acceptance Criteria**: Discovers server-reflexive candidates (`srflx`) within 500ms
  - **Documentation Reference**: `docs/02-technical-design/NETWORKING_AND_WEBRTC.md`

- [ ] **P4-005** — Configure authenticated TURN relay allocation in `webrtc-rs`
  - **Priority**: P1 (High)
  - **Dependencies**: P4-002, P4-004
  - **Expected Output**: TURN relay allocation client
  - **Acceptance Criteria**: Successfully allocates relay sockets on Coturn when P2P is blocked
  - **Documentation Reference**: `docs/02-technical-design/NETWORKING_AND_WEBRTC.md`

- [ ] **P4-006** — Implement Trickle ICE candidate streaming over the signaling channel
  - **Priority**: P1 (High)
  - **Dependencies**: P4-004, P4-005
  - **Expected Output**: Asynchronous candidate streaming
  - **Acceptance Criteria**: Reduces ICE connection setup time to $< 1.5\text{ seconds}$
  - **Documentation Reference**: `docs/01-architecture/CONNECTION_LIFECYCLE.md`

- [ ] **P4-007** — Implement automatic ICE restart and connection recovery logic upon network drops
  - **Priority**: P1 (High)
  - **Dependencies**: P4-006
  - **Expected Output**: Connection recovery state machine
  - **Acceptance Criteria**: Recovers active video stream within 10 seconds of network switch
  - **Documentation Reference**: `docs/01-architecture/CONNECTION_LIFECYCLE.md`

- [ ] **P4-008** — Validate traversal across diverse NAT types (Full Cone, Port Restricted, Symmetric)
  - **Priority**: P1 (High)
  - **Dependencies**: P4-001 through P4-007
  - **Expected Output**: Traversal verification matrix
  - **Acceptance Criteria**: 100% connection success rate across tested NAT topologies
  - **Documentation Reference**: `docs/07-roadmap/PHASE_4_NETWORKING.md`

---

## Phase 5 — Authentication, Identity & Security Hardening

- [ ] **P5-001** — Implement native OS secure storage adapters for device Ed25519 private keys
  - **Priority**: P0 (Blocker)
  - **Dependencies**: Phase 4
  - **Expected Output**: Key storage modules for DPAPI, Keychain, and Secret Service
  - **Acceptance Criteria**: Private keys persisted securely; inaccessible to unprivileged processes
  - **Documentation Reference**: `docs/03-security/DEVICE_IDENTITY.md`

- [ ] **P5-002** — Implement device registration and public key attestation in ASP.NET Core backend
  - **Priority**: P0 (Blocker)
  - **Dependencies**: P5-001
  - **Expected Output**: Device registry API endpoints
  - **Acceptance Criteria**: Verifies signed hardware attestation before registering device
  - **Documentation Reference**: `docs/05-api/API_ENDPOINTS.md`

- [ ] **P5-003** — Implement attended support session generation (9-digit code, PIN) with Redis TTL
  - **Priority**: P1 (High)
  - **Dependencies**: P5-002
  - **Expected Output**: Ephemeral session code generator
  - **Acceptance Criteria**: Codes expire in 10 minutes; locks out after 5 failed PIN attempts
  - **Documentation Reference**: `docs/03-security/AUTHENTICATION.md`

- [ ] **P5-004** — Implement host interactive consent modal in Tauri React UI
  - **Priority**: P1 (High)
  - **Dependencies**: P5-003
  - **Expected Output**: Modal prompt for incoming support requests
  - **Acceptance Criteria**: Displays caller info; requires explicit "Accept"; auto-rejects on timeout
  - **Documentation Reference**: `docs/03-security/SESSION_AUTHORIZATION.md`

- [ ] **P5-005** — Implement unattended access Argon2id password hashing and account key validation
  - **Priority**: P1 (High)
  - **Dependencies**: P5-002
  - **Expected Output**: Unattended authentication module
  - **Acceptance Criteria**: Verifies Argon2id hash or user asymmetric signature
  - **Documentation Reference**: `docs/03-security/ENCRYPTION.md`

- [ ] **P5-006** — Implement application-layer E2EE envelope (X25519 + ChaCha20) over data channels
  - **Priority**: P0 (Blocker)
  - **Dependencies**: P5-005
  - **Expected Output**: End-to-end encrypted SCTP channels
  - **Acceptance Criteria**: Relays and signaling servers cannot decrypt input, clipboard, or files
  - **Documentation Reference**: `docs/03-security/ENCRYPTION.md`

- [ ] **P5-007** — Implement persistent floating overlay toolbar with immediate disconnect kill switch
  - **Priority**: P1 (High)
  - **Dependencies**: P5-004
  - **Expected Output**: Host floating banner widget
  - **Acceptance Criteria**: Pinned above all windows; clicking disconnect drops session in $< 100\text{ms}$
  - **Documentation Reference**: `docs/03-security/SESSION_AUTHORIZATION.md`

- [ ] **P5-008** — Implement immutable audit logging in PostgreSQL recording all session events
  - **Priority**: P1 (High)
  - **Dependencies**: P5-002
  - **Expected Output**: Append-only audit logger
  - **Acceptance Criteria**: Records session start, end, permissions, and file hashes with timestamps
  - **Documentation Reference**: `docs/02-technical-design/DATABASE_DESIGN.md`

- [ ] **P5-009** — Execute automated security tests (rate limiting, traversal attacks, replay prevention)
  - **Priority**: P1 (High)
  - **Dependencies**: P5-001 through P5-008
  - **Expected Output**: Security test execution report
  - **Acceptance Criteria**: Passes all test cases defined in `SECURITY_TESTING.md`
  - **Documentation Reference**: `docs/03-security/SECURITY_TESTING.md`

---

## Phase 6 — Desktop Cross-Platform Support (Linux & macOS)

- [ ] **P6-001** — Implement macOS ScreenCaptureKit frame capture and VideoToolbox hardware encoder
  - **Priority**: P1 (High)
  - **Dependencies**: Phase 5
  - **Expected Output**: Native macOS capture and encoding pipeline
  - **Acceptance Criteria**: 60 FPS 1080p capture on Apple Silicon via Metal textures
  - **Documentation Reference**: `docs/02-technical-design/SCREEN_CAPTURE.md`

- [ ] **P6-002** — Implement macOS CoreGraphics `CGEventTap` input injection and TCC permission validator
  - **Priority**: P1 (High)
  - **Dependencies**: P6-001
  - **Expected Output**: macOS input injection module
  - **Acceptance Criteria**: Validates Accessibility permissions; injects mouse and keyboard events
  - **Documentation Reference**: `docs/02-technical-design/INPUT_CONTROL.md`

- [ ] **P6-003** — Implement Linux Wayland PipeWire DmaBuf capture via XDG Desktop Portal
  - **Priority**: P1 (High)
  - **Dependencies**: Phase 5
  - **Expected Output**: Wayland screen capture driver
  - **Acceptance Criteria**: Captures frames via PipeWire node; feeds VAAPI hardware encoder
  - **Documentation Reference**: `docs/02-technical-design/SCREEN_CAPTURE.md`

- [ ] **P6-004** — Implement Linux Wayland `libei` emulated input injection
  - **Priority**: P1 (High)
  - **Dependencies**: P6-003
  - **Expected Output**: Wayland emulated input driver
  - **Acceptance Criteria**: Injects input via `RemoteDesktop` portal or `libei` socket
  - **Documentation Reference**: `docs/02-technical-design/INPUT_CONTROL.md`

- [ ] **P6-005** — Implement Linux X11 `XShm` capture and `XTest` input injection fallbacks
  - **Priority**: P2 (Medium)
  - **Dependencies**: Phase 5
  - **Expected Output**: X11 capture and input driver
  - **Acceptance Criteria**: Functions on traditional Xorg desktop environments
  - **Documentation Reference**: `docs/01-architecture/PLATFORM_ARCHITECTURE.md`

- [ ] **P6-006** — Implement native audio loopback capture (WASAPI, CoreAudio, PipeWire) with Opus
  - **Priority**: P1 (High)
  - **Dependencies**: P6-001, P6-003
  - **Expected Output**: Audio streaming subsystem
  - **Acceptance Criteria**: Streams 48 kHz stereo audio synchronized with video
  - **Documentation Reference**: `docs/02-technical-design/VIDEO_AND_AUDIO.md`

- [ ] **P6-007** — Implement bidirectional clipboard synchronization (text and images)
  - **Priority**: P2 (Medium)
  - **Dependencies**: Phase 5
  - **Expected Output**: Clipboard sync engine
  - **Acceptance Criteria**: Syncs text and PNG images; suppresses infinite copy-paste loops
  - **Documentation Reference**: `docs/02-technical-design/SHARED_PROTOCOL.md`

- [ ] **P6-008** — Implement file transfer manager over SCTP data channel with resume support
  - **Priority**: P1 (High)
  - **Dependencies**: Phase 5
  - **Expected Output**: Interactive dual-pane file manager
  - **Acceptance Criteria**: Transfers files with SHA-256 validation; enforces path sandbox
  - **Documentation Reference**: `docs/02-technical-design/FILE_TRANSFER.md`

- [ ] **P6-009** — Implement multi-monitor topology detection, switching, and combined viewport
  - **Priority**: P2 (Medium)
  - **Dependencies**: P6-001, P6-003
  - **Expected Output**: Multi-monitor management UI
  - **Acceptance Criteria**: Switches displays dynamically without dropping session
  - **Documentation Reference**: `docs/02-technical-design/SCREEN_CAPTURE.md`

---

## Phase 7 — Mobile Applications (Flutter 3.x - iOS & Android)

- [ ] **P7-001** — Scaffold Flutter 3.x project and integrate `flutter_webrtc` and SignalR client
  - **Priority**: P1 (High)
  - **Dependencies**: Phase 5
  - **Expected Output**: Working Flutter client framework
  - **Acceptance Criteria**: Connects to signaling hub and establishes WebRTC peer connection
  - **Documentation Reference**: `docs/02-technical-design/MOBILE_APPLICATION.md`

- [ ] **P7-002** — Implement responsive WebRTC video rendering view with hardware acceleration
  - **Priority**: P1 (High)
  - **Dependencies**: P7-001
  - **Expected Output**: High-performance mobile video viewport
  - **Acceptance Criteria**: Decodes 60 FPS video with hardware acceleration on iOS and Android
  - **Documentation Reference**: `docs/02-technical-design/MOBILE_APPLICATION.md`

- [ ] **P7-003** — Implement Virtual Trackpad touch-to-mouse interpreter with ballistic acceleration
  - **Priority**: P1 (High)
  - **Dependencies**: P7-002
  - **Expected Output**: Trackpad gesture controller
  - **Acceptance Criteria**: Single tap clicks, double-tap drags, two-finger tap right-clicks
  - **Documentation Reference**: `docs/02-technical-design/MOBILE_APPLICATION.md`

- [ ] **P7-004** — Implement Direct Touch mode for tablet and stylus interactions
  - **Priority**: P2 (Medium)
  - **Dependencies**: P7-002
  - **Expected Output**: Tablet direct touch interpreter
  - **Acceptance Criteria**: Absolute coordinate touch mapping with Apple Pencil / stylus support
  - **Documentation Reference**: `docs/02-technical-design/MOBILE_APPLICATION.md`

- [ ] **P7-005** — Implement mobile modifier keyboard overlay bar with latching modifiers
  - **Priority**: P1 (High)
  - **Dependencies**: P7-002
  - **Expected Output**: On-screen modifier toolbar
  - **Acceptance Criteria**: Injects Ctrl, Alt, Shift, Esc, Win, F1-F12, and Ctrl+Alt+Del
  - **Documentation Reference**: `docs/02-technical-design/MOBILE_APPLICATION.md`

- [ ] **P7-006** — Implement Android native `MediaProjection` screen capture and foreground service
  - **Priority**: P2 (Medium)
  - **Dependencies**: P7-001
  - **Expected Output**: Android host screen streaming module
  - **Acceptance Criteria**: Streams Android screen content with persistent notification
  - **Documentation Reference**: `docs/01-architecture/PLATFORM_ARCHITECTURE.md`

- [ ] **P7-007** — Implement Android native `AccessibilityService` gesture injection
  - **Priority**: P2 (Medium)
  - **Dependencies**: P7-006
  - **Expected Output**: Android remote control module
  - **Acceptance Criteria**: Injects taps and swipes when Accessibility permission is enabled
  - **Documentation Reference**: `docs/02-technical-design/INPUT_CONTROL.md`

- [ ] **P7-008** — Implement iOS ReplayKit Broadcast Upload Extension for screen streaming
  - **Priority**: P2 (Medium)
  - **Dependencies**: P7-001
  - **Expected Output**: iOS screen broadcast extension (view-only)
  - **Acceptance Criteria**: Streams iOS screen to session; documents OS input restriction
  - **Documentation Reference**: `docs/01-architecture/PLATFORM_ARCHITECTURE.md`

- [ ] **P7-009** — Conduct usability and network transition testing across 5G, 4G, and Wi-Fi
  - **Priority**: P1 (High)
  - **Dependencies**: P7-001 through P7-008
  - **Expected Output**: Mobile QA test report
  - **Acceptance Criteria**: Seamless handover during cellular to Wi-Fi network switching
  - **Documentation Reference**: `docs/07-roadmap/PHASE_7_MOBILE.md`

---

## Phase 8 — Backend Control Plane & Web Management Dashboard

- [ ] **P8-001** — Implement multi-tenant Organization and User RBAC models in ASP.NET Core
  - **Priority**: P1 (High)
  - **Dependencies**: Phase 5
  - **Expected Output**: RBAC domain entities and middleware
  - **Acceptance Criteria**: Enforces Admin, Manager, Technician, and Member roles
  - **Documentation Reference**: `docs/02-technical-design/BACKEND_ARCHITECTURE.md`

- [ ] **P8-002** — Implement Device fleet querying, grouping, and tag management APIs
  - **Priority**: P1 (High)
  - **Dependencies**: P8-001
  - **Expected Output**: Fleet management REST endpoints
  - **Acceptance Criteria**: Supports sorting, filtering, and tag assignments for devices
  - **Documentation Reference**: `docs/05-api/API_ENDPOINTS.md`

- [ ] **P8-003** — Implement real-time Redis presence subscriber pushing device status to web
  - **Priority**: P1 (High)
  - **Dependencies**: P8-002
  - **Expected Output**: Live presence notification pipeline
  - **Acceptance Criteria**: Device state changes broadcast to dashboards in $< 200\text{ ms}$
  - **Documentation Reference**: `docs/02-technical-design/BACKEND_ARCHITECTURE.md`

- [ ] **P8-004** — Scaffold React 19 Web Dashboard using Vite, TailwindCSS, and Lucide icons
  - **Priority**: P1 (High)
  - **Dependencies**: P8-001
  - **Expected Output**: Modern responsive web frontend
  - **Acceptance Criteria**: Clean UI with login, fleet overview, and audit log pages
  - **Documentation Reference**: `docs/07-roadmap/PHASE_8_BACKEND_AND_DASHBOARD.md`

- [ ] **P8-005** — Implement interactive device fleet table with instant filtering and search
  - **Priority**: P1 (High)
  - **Dependencies**: P8-004
  - **Expected Output**: Fleet management data table
  - **Acceptance Criteria**: Renders 1,000+ devices with sub-second search and pagination
  - **Documentation Reference**: `docs/07-roadmap/PHASE_8_BACKEND_AND_DASHBOARD.md`

- [ ] **P8-006** — Implement zero-install WebRTC browser viewer component using WebCodecs/Canvas
  - **Priority**: P1 (High)
  - **Dependencies**: P8-004
  - **Expected Output**: Browser-based remote desktop viewer
  - **Acceptance Criteria**: Streams remote screen directly in Chrome, Edge, Safari, and Firefox
  - **Documentation Reference**: `docs/02-technical-design/DESKTOP_APPLICATION.md`

- [ ] **P8-007** — Implement audit log viewer with filtering by date, technician, device, and action
  - **Priority**: P2 (Medium)
  - **Dependencies**: P8-004
  - **Expected Output**: Audit trail explorer UI
  - **Acceptance Criteria**: Displays event details and supports exporting records to CSV
  - **Documentation Reference**: `docs/02-technical-design/DATABASE_DESIGN.md`

- [ ] **P8-008** — Implement automated OpenAPI (Swagger) documentation generation
  - **Priority**: P2 (Medium)
  - **Dependencies**: P8-001, P8-002
  - **Expected Output**: Interactive API documentation at `/swagger`
  - **Acceptance Criteria**: All REST endpoints documented with schemas and auth requirements
  - **Documentation Reference**: `docs/05-api/API_ENDPOINTS.md`

---

## Phase 9 — Self-Hosting Infrastructure & Turnkey Deployment

- [ ] **P9-001** — Author optimized multi-stage `Dockerfile.server` for ASP.NET Core 10 backend
  - **Priority**: P1 (High)
  - **Dependencies**: Phase 6, Phase 8
  - **Expected Output**: Minimal production container image
  - **Acceptance Criteria**: Image size $< 180\text{ MB}$; runs as unprivileged user
  - **Documentation Reference**: `docs/04-infrastructure/DOCKER_ARCHITECTURE.md`

- [ ] **P9-002** — Author production `docker-compose.yml` orchestrating all services
  - **Priority**: P1 (High)
  - **Dependencies**: P9-001
  - **Expected Output**: Production Docker Compose configuration
  - **Acceptance Criteria**: Deploys Server, Postgres 17, Redis 7.4, Coturn, and Nginx cleanly
  - **Documentation Reference**: `docs/04-infrastructure/DOCKER_ARCHITECTURE.md`

- [ ] **P9-003** — Configure Nginx reverse proxy with automated Let's Encrypt TLS renewal
  - **Priority**: P1 (High)
  - **Dependencies**: P9-002
  - **Expected Output**: Nginx TLS ingress container
  - **Acceptance Criteria**: Obtains and auto-renews TLS certificates via Certbot ACME
  - **Documentation Reference**: `docs/04-infrastructure/SELF_HOSTING.md`

- [ ] **P9-004** — Configure hardened `turnserver.conf` with IP filtering and ephemeral authentication
  - **Priority**: P1 (High)
  - **Dependencies**: P9-002
  - **Expected Output**: Production Coturn configuration
  - **Acceptance Criteria**: Enforces HMAC-SHA1 auth; denies private subnet forwarding
  - **Documentation Reference**: `docs/02-technical-design/NETWORKING_AND_WEBRTC.md`

- [ ] **P9-005** — Implement automated database migration execution on container startup
  - **Priority**: P1 (High)
  - **Dependencies**: P9-002
  - **Expected Output**: Container entrypoint migration hook
  - **Acceptance Criteria**: Runs pending EF Core migrations automatically before starting server
  - **Documentation Reference**: `docs/04-infrastructure/SELF_HOSTING.md`

- [ ] **P9-006** — Author shell scripts for secret generation, database backup, and disaster restoration
  - **Priority**: P1 (High)
  - **Dependencies**: P9-002
  - **Expected Output**: Turnkey administrative scripts
  - **Acceptance Criteria**: Creates automated `.sql.gz` backups; restores database cleanly
  - **Documentation Reference**: `docs/04-infrastructure/BACKUP_AND_RECOVERY.md`

- [ ] **P9-007** — Validate full clean installation on fresh Linux cloud VPS instances
  - **Priority**: P1 (High)
  - **Dependencies**: P9-001 through P9-006
  - **Expected Output**: Turnkey deployment verification report
  - **Acceptance Criteria**: Deploys in $< 3\text{ minutes}$; clients connect successfully
  - **Documentation Reference**: `docs/04-infrastructure/SELF_HOSTING.md`

---

## Phase 10 — Hardening, Code Signing & Public Beta Release

- [ ] **P10-001** — Execute k6 load tests validating 10,000 concurrent SignalR connections
  - **Priority**: P1 (High)
  - **Dependencies**: Phases 1 through 9
  - **Expected Output**: High-concurrency load test report
  - **Acceptance Criteria**: Backend sustains 10,000 active connections with CPU $< 60\%$
  - **Documentation Reference**: `docs/04-infrastructure/MONITORING.md`

- [ ] **P10-002** — Conduct network chaos testing under 15% packet loss and 150ms jitter
  - **Priority**: P1 (High)
  - **Dependencies**: P10-001
  - **Expected Output**: Network resilience test report
  - **Acceptance Criteria**: Video stream adapts dynamically without freezing or crashing
  - **Documentation Reference**: `docs/06-development/TESTING_STRATEGY.md`

- [ ] **P10-003** — Execute comprehensive security vulnerability scans and address any CVEs
  - **Priority**: P0 (Blocker)
  - **Dependencies**: P10-001
  - **Expected Output**: Clean security audit report (Trivy, CodeQL, OWASP ZAP)
  - **Acceptance Criteria**: 0 high or critical vulnerabilities in application or dependencies
  - **Documentation Reference**: `docs/03-security/SECURITY_TESTING.md`

- [ ] **P10-004** — Configure Windows code signing pipeline producing signed `.msi` installers
  - **Priority**: P1 (High)
  - **Dependencies**: P10-001
  - **Expected Output**: Code-signed Windows installer packages
  - **Acceptance Criteria**: Installs on clean Windows 11 without untrusted publisher warnings
  - **Documentation Reference**: `docs/04-infrastructure/CI_CD.md`

- [ ] **P10-005** — Configure macOS code signing and Apple Notary Service submission for `.dmg`
  - **Priority**: P1 (High)
  - **Dependencies**: P10-001
  - **Expected Output**: Notarized macOS application disk image
  - **Acceptance Criteria**: Passes Apple Gatekeeper verification cleanly on macOS
  - **Documentation Reference**: `docs/04-infrastructure/CI_CD.md`

- [ ] **P10-006** — Configure Linux `.deb` and `.AppImage` packaging with GPG signature generation
  - **Priority**: P1 (High)
  - **Dependencies**: P10-001
  - **Expected Output**: Signed Linux application packages
  - **Acceptance Criteria**: Installs on Ubuntu, Debian, and Fedora distributions cleanly
  - **Documentation Reference**: `docs/04-infrastructure/CI_CD.md`

- [ ] **P10-007** — Finalize public documentation, quickstart guides, and community issue templates
  - **Priority**: P1 (High)
  - **Dependencies**: P10-001 through P10-006
  - **Expected Output**: Polished documentation and issue templates
  - **Acceptance Criteria**: Comprehensive guides for users, self-hosters, and contributors
  - **Documentation Reference**: `docs/06-development/CONTRIBUTING.md`

- [ ] **P10-008** — Tag and publish OpenDesk v1.0.0 Public Beta release
  - **Priority**: P0 (Blocker)
  - **Dependencies**: P10-001 through P10-007
  - **Expected Output**: Official GitHub Release with assets, signatures, and notes
  - **Acceptance Criteria**: Production release live; binaries downloadable and verified
  - **Documentation Reference**: `docs/06-development/RELEASE_PROCESS.md`
