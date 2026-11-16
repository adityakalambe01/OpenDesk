# OpenDesk — Architectural Decision Records (ADR)

## Index of Architectural Decisions

- [ADR-001: Selection of Rust for the Core Remote Desktop Engine](#adr-001-selection-of-rust-for-the-core-remote-desktop-engine)
- [ADR-002: Desktop UI Framework Selection: Tauri v2 + React 19 vs. Electron](#adr-002-desktop-ui-framework-selection-tauri-v2--react-19-vs-electron)
- [ADR-003: Backend Stack Selection: ASP.NET Core 10 & Entity Framework Core](#adr-003-backend-stack-selection-aspnet-core-10--entity-framework-core)
- [ADR-004: Mobile Client Framework Selection: Flutter 3.x / Dart](#adr-004-mobile-client-framework-selection-flutter-3x--dart)
- [ADR-005: Transport Plane Selection: WebRTC (ICE/DTLS/SRTP/SCTP)](#adr-005-transport-plane-selection-webrtc-icedtlssrtpsctp)
- [ADR-006: Hardware Video Codec Strategy: H.264 as Baseline with AV1/VP9 Capability](#adr-006-hardware-video-codec-strategy-h264-as-baseline-with-av1vp9-capability)
- [ADR-007: Host Separation into System Daemon and Session Agent](#adr-007-host-separation-into-system-daemon-and-session-agent)
- [ADR-008: Zero-Knowledge Relay Standard: coturn over Custom Proprietary Relays](#adr-008-zero-knowledge-relay-standard-coturn-over-custom-proprietary-relays)
- [ADR-009: Cryptographic Primitives: Ed25519 & X25519 with ChaCha20-Poly1305](#adr-009-cryptographic-primitives-ed25519--x25519-with-chacha20-poly1305)
- [ADR-010: Persistence Strategy: PostgreSQL 17 with Redis for Ephemeral State](#adr-010-persistence-strategy-postgresql-17-with-redis-for-ephemeral-state)

---

### ADR-001: Selection of Rust for the Core Remote Desktop Engine
- **Status**: Accepted
- **Context**: The core engine (`libopendesk`) must handle high-throughput frame grabbing, pixel format conversion, hardware encoder memory mapping, low-latency audio capture, synthetic input injection, and WebRTC streaming across Windows, Linux, and macOS.
- **Alternatives Considered**:
  1. *C++20*: Established industry standard for multimedia and WebRTC (libwebrtc is written in C++). However, lack of memory safety introduces severe risks of buffer overflows, use-after-free, and concurrency race conditions in security-critical network code.
  2. *Go*: Excellent networking primitives and concurrency (goroutines), but the Go garbage collector (GC) introduces unpredictable pause times (stop-the-world latency spikes) that degrade real-time 60 FPS video pipelines, and CGo FFI calls incur non-trivial overhead.
  3. *C# / .NET AOT*: Modern .NET with NativeAOT is fast, but lacks first-class idiomatic bindings to macOS ScreenCaptureKit and Linux PipeWire C interfaces compared to Rust.
- **Decision**: Select **Rust (latest stable edition 2021/2024)**.
- **Rationale**:
  - Zero-cost abstractions and fearless concurrency with compile-time memory safety guarantees.
  - Predictable deterministic execution with no garbage collection pauses.
  - Direct, zero-overhead C-ABI interoperability with platform APIs (Win32, DirectX, CoreGraphics, Metal, PipeWire, D-Bus).
  - Rich modern ecosystem (`webrtc-rs`, `tokio`, `windows-rs`, `nix`, `tracing`).

---

### ADR-002: Desktop UI Framework Selection: Tauri v2 + React 19 vs. Electron
- **Status**: Accepted
- **Context**: The desktop client requires an intuitive, modern UI that runs on Windows, macOS, and Linux, displaying device lists, connection history, settings, and attended support panels.
- **Alternatives Considered**:
  1. *Electron*: Highly mature, but bundles an entire Chromium browser and Node.js runtime per application, resulting in 150MB+ installer sizes, 200MB+ idle RAM consumption, and heavy security overhead.
  2. *Flutter Desktop*: Excellent rendering performance, but inconsistent native desktop OS integration (window management, native menus, accessibility hooks) and larger binary footprint on Linux/macOS.
  3. *WPF / .NET MAUI*: Not genuinely cross-platform across macOS and Linux with high UI fidelity.
- **Decision**: Select **Tauri v2** with **React 19** and **TypeScript**.
- **Rationale**:
  - Leverages the OS's native webview (WebView2 on Windows, WebKit on macOS, WebKitGTK on Linux), reducing binary size to < 20 MB and idle RAM to < 50 MB.
  - Tauri v2 provides first-class Rust backend architecture with secure IPC command bindings directly into `libopendesk`.
  - Granular permissions model (Tauri Capability system) limits webview capabilities to strictly authorized native commands.

---

### ADR-003: Backend Stack Selection: ASP.NET Core 10 & Entity Framework Core
- **Status**: Accepted
- **Context**: The control plane backend must manage device registries, user accounts, authentication, session tokens, audit logging, and high-concurrency WebSocket signaling.
- **Alternatives Considered**:
  1. *Node.js / Express / NestJS*: Rapid prototyping, but weaker CPU-bound performance, dynamic typing risks in large codebases, and higher memory usage per concurrent WebSocket connection.
  2. *Go (Golang)*: High concurrency and small binaries, but less mature enterprise ORM/data tooling compared to EF Core, and lack of built-in enterprise features (e.g. ASP.NET Identity, data protection, SignalR backplanes).
  3. *Rust (Axum / Actix-web)*: Exceptional performance, but slower development velocity for complex CRUD, migrations, and enterprise business logic compared to modern C#.
- **Decision**: Select **ASP.NET Core 10 (C# 14)** with **Entity Framework Core 10**.
- **Rationale**:
  - Top-tier throughput and memory efficiency in TechEmpower benchmarks; native WebSockets and SignalR hub architecture.
  - Enterprise-grade tooling: EF Core code-first migrations, ASP.NET Core Identity, JWT authentication, OpenTelemetry integration.
  - First-class cross-platform support on Linux containers with minimal Alpine/Chiseled container images.

---

### ADR-004: Mobile Client Framework Selection: Flutter 3.x / Dart
- **Status**: Accepted
- **Context**: Mobile clients for iOS and Android must provide responsive touch-to-mouse translation, smooth remote screen rendering, and connection management.
- **Alternatives Considered**:
  1. *React Native*: Good cross-platform coverage, but bridge serialization overhead and UI rendering inconsistencies across complex custom rendering widgets (virtual trackpad, canvas zoom/pan).
  2. *Native Swift & Kotlin*: Maximum platform fidelity, but requires maintaining two separate codebases for mobile UI, doubling maintenance burden.
- **Decision**: Select **Flutter 3.x with Dart**.
- **Rationale**:
  - Direct hardware-accelerated Skia/Impeller rendering pipeline capable of 60/120 Hz fluid panning, zooming, and trackpad emulation.
  - Mature `flutter_webrtc` plugin utilizing Google's native C++ WebRTC engine.
  - High degree of code sharing between iOS and Android clients (> 90%).

---

### ADR-005: Transport Plane Selection: WebRTC (ICE/DTLS/SRTP/SCTP)
- **Status**: Accepted
- **Context**: Remote desktop streaming demands low latency, NAT traversal, dynamic congestion control, and secure transport over variable public networks.
- **Alternatives Considered**:
  1. *Custom UDP/TCP Protocol*: High development effort to reinvent STUN, TURN, ICE candidate harvesting, jitter buffers, and congestion control algorithms from scratch.
  2. *Raw QUIC (RFC 9000)*: Excellent multiplexing and connection migration, but browser support for raw QUIC video ingest without WebTransport remains evolving, and standard NAT hole-punching frameworks (STUN/ICE) are not standardized natively for pure QUIC.
  3. *VNC / RFB or RDP*: Legacy protocols ill-suited for WAN peer-to-peer streaming across consumer firewalls without VPN tunnels; high latency over lossy links.
- **Decision**: Select **WebRTC (RFC 8825 suite)** as the foundational transport plane.
- **Rationale**:
  - Industry gold standard for peer-to-peer real-time communication.
  - Integrated ICE (STUN/TURN) for automatic direct P2P hole-punching and seamless relay fallback.
  - Built-in DTLS 1.3 encryption and SRTP media framing.
  - Advanced Google Congestion Control (GCC) adapting video bitrate in real time to network conditions.
  - SCTP Data Channels providing reliable and unordered reliable messaging for input, clipboard, and file transfer.

---

### ADR-006: Hardware Video Codec Strategy: H.264 as Baseline with AV1/VP9 Capability
- **Status**: Accepted
- **Context**: The capture engine must encode desktop frame sequences (1920x1080 to 3840x2160) at 30-60 FPS with minimal latency and CPU impact.
- **Alternatives Considered**:
  1. *AV1 Only*: Superior compression efficiency, but hardware encoding is absent on older GPUs (requires NVIDIA RTX 4000+, AMD RX 7000+, Intel Arc, or Apple M3+). Software encoding is too CPU-heavy for real-time 60 FPS 1080p.
  2. *VP8/VP9*: Good open formats, but inconsistent hardware encoder support on Windows/macOS compared to H.264.
  3. *H.265 (HEVC)*: Excellent performance, but complex patent licensing and restricted browser decoding support in open-source distributions.
- **Decision**: Standardize on **H.264 (Constrained Baseline & High Profiles) as the universal baseline**, with dynamic runtime negotiation for **AV1 and VP9** when both endpoints report hardware support.
- **Rationale**: Universal hardware acceleration across virtually all consumer and enterprise hardware manufactured in the last 12 years; universal decoding support in browsers and mobile SoCs.

---

### ADR-007: Host Separation into System Daemon and Session Agent
- **Status**: Accepted
- **Context**: The host software must run continuously on unattended machines, survive user logouts/reboots, and access the secure Windows logon screen (Winlogon/UAC) without exposing the UI to privilege escalation vulnerabilities.
- **Decision**: Implement a **Three-Tier Architecture**:
  1. *System Daemon*: Runs as Windows Service (`NT AUTHORITY\SYSTEM`), Linux systemd service (`root`), or macOS LaunchDaemon. Maintains signaling connection and holds device identity.
  2. *Session Agent*: Launched dynamically by the System Daemon into the active interactive user session (Session 1+, console session). Executes screen capture, audio capture, and input injection.
  3. *Host UI*: User-level client for attended support codes and settings.
- **Rationale**: Prevents Session 0 Isolation blocks on Windows; adheres to least-privilege security principles; ensures unbroken streaming during UAC privilege elevation and user switching.

---

### ADR-008: Zero-Knowledge Relay Standard: coturn over Custom Proprietary Relays
- **Status**: Accepted
- **Context**: A relay service is mandatory when direct peer-to-peer UDP connectivity fails (symmetric NATs or restricted enterprise firewalls).
- **Decision**: Deploy **coturn (RFC 8489 / RFC 8656)** as the standard STUN/TURN relay infrastructure.
- **Rationale**:
  - Production-tested, hardened, C-based relay capable of forwarding hundreds of gigabits of encrypted media per second.
  - Eliminates the need to maintain custom relay server software.
  - Authentication via standard RFC 5766 REST API mechanism using shared HMAC-SHA1 secrets generated by the ASP.NET Core backend.
  - Coturn operates strictly at the datagram relay level, preserving zero-knowledge end-to-end encryption.

---

### ADR-009: Cryptographic Primitives: Ed25519 & X25519 with ChaCha20-Poly1305
- **Status**: Accepted
- **Context**: OpenDesk requires cryptographic identity for devices and users, secure session authorization, and authenticated end-to-end data channel encryption.
- **Decision**: Standardize on modern Edwards-curve and Montgomery-curve cryptography:
  - Device Identity & Signatures: **Ed25519** (RFC 8032).
  - Key Agreement: **X25519** (RFC 7748).
  - Authenticated Symmetric Encryption: **ChaCha20-Poly1305** (RFC 8439) with fallback to **AES-256-GCM** where hardware AES instructions (`AES-NI`, ARMv8 Cryptography Extensions) are present.
  - Password Hashing: **Argon2id** (RFC 9106).
- **Rationale**: Immunizes against timing attacks, provides high performance across all architectures without hardware acceleration dependencies, and avoids legacy RSA/NIST curve vulnerabilities.

---

### ADR-010: Persistence Strategy: PostgreSQL 17 with Redis for Ephemeral State
- **Status**: Accepted
- **Context**: OpenDesk requires reliable persistence for relational entities (Users, Devices, Organizations, Access Logs, Permissions) and sub-millisecond storage for volatile state (Device Presence, SignalR Backplane, Session Nonces).
- **Decision**:
  - Relational Database: **PostgreSQL 17** managed via Entity Framework Core.
  - Volatile & Caching Store: **Redis 7.4 (or Valkey)**.
- **Rationale**:
  - PostgreSQL provides ACID compliance, JSONB support for dynamic device capabilities, and rock-solid reliability in Docker containers.
  - Redis provides instantaneous pub/sub messaging for multi-node SignalR scaling and key expiration (TTL) for device heartbeats and one-time session PINs.
