# OpenDesk — Technical Risk Register & External Dependency Inventory

## 1. Technical Risk Register

| Risk ID | Risk Description | Severity | Probability | Impact | Mitigation Strategy | Owner |
|---|---|:---:|:---:|---|---|---|
| **RSK-01** | Strict corporate firewalls block WebRTC UDP traffic entirely. | High | Medium | Direct P2P and UDP TURN fail, preventing session establishment. | Coturn relay listens on TCP port 443 with TURNS (TLS encapsulation), appearing identical to standard HTTPS traffic to deep-packet inspection firewalls. | DevOps / Net Eng |
| **RSK-02** | Hardware video encoder driver fragmentation (NVENC, AMF, QuickSync, VAAPI). | Medium | High | Crash or failure to initialize GPU encoder on diverse consumer workstations. | Robust fallback hierarchy: NVENC $\rightarrow$ QuickSync $\rightarrow$ AMF $\rightarrow$ VAAPI $\rightarrow$ OpenH264 (CPU-based software encoding). | Rust Core Eng |
| **RSK-03** | Wayland portal fragmentation across Linux desktop environments (GNOME vs. KDE vs. Sway/wlroots). | High | Medium | Screen capture or input injection fails on non-standard compositors. | Standardize strictly on XDG Desktop Portal specs (`ScreenCast` & `libei`); provide automated detection of `X11` fallback sessions. | Desktop Eng |
| **RSK-04** | Windows UAC Secure Desktop prompts freeze client screen or reject input. | High | Medium | Technician cannot assist user with administrative software installations. | Desktop agent signed with valid Authenticode certificate embedded with `<requestedExecutionLevel uiAccess="true" />` and installed in Program Files. | Windows Eng |
| **RSK-05** | SmartScreen / Gatekeeper untrusted application warnings block installation. | High | High | End-users cannot install OpenDesk without administrative security bypass warnings. | Azure Trusted Signing / EV Code Signing on Windows; Apple Developer ID signing and automated notarization via `xcrun notarytool` on macOS. | Release Eng |
| **RSK-06** | Brute-force guessing attacks on 9-digit support session codes. | High | Low | Unauthorized technician joins victim's unattended or attended session. | Rate limiting in Redis: 5 incorrect PIN attempts invalidates session code immediately and bans IP for 15 minutes; mandatory explicit user "Accept" prompt. | Security Eng |

---

## 2. External Dependency Inventory

OpenDesk standardizes on audited, battle-tested open-source dependencies:

### Core Rust Workspace Dependencies (`core/Cargo.toml`)
- `webrtc (0.12)`: Pure Rust implementation of WebRTC (ICE, DTLS, SRTP, SCTP). Audited by the Rust community.
- `windows (0.58)`: Official Microsoft Win32 and Direct3D bindings for Rust.
- `tokio (1.41)`: Asynchronous event runtime with multi-threaded work-stealing scheduler.
- `ed25519-dalek (2.1)` & `x25519-dalek (2.0)`: Formally audited Curve25519 implementations from the Dalek cryptography suite.
- `chacha20poly1305 (0.10)`: RustCrypto AEAD implementation of RFC 8439.
- `argon2 (0.5)`: RustCrypto implementation of the Password Hashing Competition winner (RFC 9106).
- `bincode (1.3)`: Zero-overhead binary serialization for network data channels.

### Backend .NET Dependencies (`server/Directory.Packages.props`)
- `Microsoft.AspNetCore.SignalR.StackExchangeRedis (10.0)`: Distributed SignalR backplane for multi-node WebSocket routing.
- `Npgsql.EntityFrameworkCore.PostgreSQL (10.0)`: High-performance PostgreSQL 17 provider for EF Core.
- `StackExchange.Redis (2.8)`: Asynchronous Redis client for high-throughput presence operations.
- `MediatR (12.4)`: In-process messaging mediator implementing CQRS command/query decoupling.

### Desktop & Mobile Dependencies
- `tauri (2.1)`: Rust-to-Webview desktop application framework.
- `react (19.0)` & `zustand (5.0)`: Reactive frontend UI and decoupled state management.
- `flutter_webrtc (0.11)`: Flutter WebRTC plugin binding directly to Google's native libwebrtc C++ engine.

### Infrastructure Images
- `postgres:17-alpine`: Relational persistence database.
- `redis:7.4-alpine`: In-memory presence cache and SignalR message broker.
- `coturn/coturn:latest`: Production-hardened RFC 8489/8656 STUN/TURN media relay.
- `nginx:alpine`: Reverse proxy with automated Let's Encrypt TLS renewal.
