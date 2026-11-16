# OpenDesk — System Architecture Specification

## 1. Architectural Philosophy & Principles

OpenDesk is designed around five foundational architectural tenets:
1. **Decoupled Three-Process Host Model**: The UI, the administrative service daemon, and the user-session capture agent are strictly separate processes. The crash or termination of the UI never terminates an active remote session or background service.
2. **Protocol Neutrality via Rust Core (`libopendesk`)**: All protocol serialization, cryptographic negotiation, video encoding orchestration, and WebRTC peer connection management reside in a single, high-performance, memory-safe Rust codebase shared across desktop platforms.
3. **Zero-Knowledge Transport Plane**: All network routing through intermediary relay nodes (coturn) operates on encrypted opaque datagrams. The control plane orchestrates session establishment, but never handles real-time media or inputs.
4. **Platform-Native Hardware Acceleration**: No generic software screen scraping. Each supported operating system utilizes native GPU-accelerated capture APIs and native hardware encoding pipelines.
5. **Unified Multi-Tenant Cloud and Self-Hosted Single-Tenant Topology**: The exact same backend binaries, Docker containers, and database schemas serve both OpenDesk Public Cloud and private self-hosted enterprise deployments.

---

## 2. High-Level System Architecture Diagram

```mermaid
flowchart TB
    subgraph ControllerTier["Controller / Client Tier (Technician / Remote User)"]
        ClientDesktop["Desktop Client\n(Tauri v2 + React 19)"]
        ClientMobile["Mobile Client\n(Flutter 3.x - iOS/Android)"]
        ClientWeb["Web Viewer\n(React 19 + WASM)"]
    end

    subgraph ControlPlane["OpenDesk Control Plane (Backend)"]
        APIGateway["Nginx / Cloudflare / Envoy (TLS 1.3)"]
        BackendAPI["ASP.NET Core 10 Web API\n(REST & GraphQL)"]
        SignalingHub["ASP.NET Core 10 SignalR Hub\n(WebSockets)"]
        PostgresDB[(PostgreSQL 17\nIdentity, Devices, Audit)]
        RedisCache[(Redis 7.4 Cluster\nPresence & SignalR Backplane)]
    end

    subgraph RelayPlane["Transport & Relay Infrastructure"]
        CoturnRelay["Coturn STUN / TURN Cluster\n(RFC 8489 / RFC 8656)"]
    end

    subgraph HostTier["Host Tier (Controlled Machine)"]
        subgraph HostOS["Host Operating System Boundaries"]
            ServiceDaemon["OpenDesk System Daemon\n(Privileged Service / Root / Systemd)"]
            SessionAgent["OpenDesk Desktop Agent\n(Interactive User Session - Rust)"]
            HostUI["OpenDesk Host UI Shell\n(Tauri v2 + React 19)"]
        end
    end

    %% Client to Control Plane
    ClientDesktop -->|HTTPS / REST API| APIGateway
    ClientDesktop -->|WSS / SignalR| SignalingHub
    ClientMobile -->|HTTPS / REST API| APIGateway
    ClientMobile -->|WSS / SignalR| SignalingHub
    ClientWeb -->|HTTPS / REST API| APIGateway
    ClientWeb -->|WSS / SignalR| SignalingHub

    %% Backend internal
    APIGateway --> BackendAPI
    APIGateway --> SignalingHub
    BackendAPI --> PostgresDB
    BackendAPI --> RedisCache
    SignalingHub --> RedisCache

    %% Host to Control Plane
    ServiceDaemon -->|HTTPS / Device API| APIGateway
    ServiceDaemon -->|WSS / Signaling| SignalingHub
    HostUI -->|Local IPC: Named Pipe / Domain Socket| ServiceDaemon
    ServiceDaemon -->|Spawn & IPC| SessionAgent

    %% WebRTC Signaling & Media
    SignalingHub -.->|SDP Offer / Answer & ICE Candidates| ClientDesktop
    SignalingHub -.->|SDP Offer / Answer & ICE Candidates| ServiceDaemon

    %% Media Transport Paths
    ClientDesktop <===>|Primary: Direct P2P WebRTC (UDP/DTLS-SRTP)| SessionAgent
    ClientDesktop <===>|Fallback: Relay WebRTC (TURN/UDP/TCP)| CoturnRelay
    CoturnRelay <===>|Relayed WebRTC (DTLS-SRTP)| SessionAgent
```

---

## 3. Tiered System Breakdown

### Tier 1: Client / Controller Tier
- **Desktop Application (Tauri v2 + React 19 + TypeScript)**:
  - Consists of a thin, security-hardened Webview frontend written in React 19 and TypeScript.
  - Interacts with a local Rust core via Tauri IPC commands.
  - Renders incoming WebRTC video frames via native hardware-accelerated Direct3D11 / Metal / OpenGL canvas textures or Tauri native windows.
  - Captures high-frequency keyboard, mouse, and touch gestures, sending them over WebRTC SCTP data channels.
- **Mobile Application (Flutter 3.x / Dart)**:
  - Implements responsive touch-to-mouse translation modes: Direct Touch (touch where you click) and Virtual Trackpad (mouse cursor with acceleration).
  - Integrates `flutter_webrtc` (wrapping Google's native libwebrtc C++ library).
  - Handles network transitions (Wi-Fi $\leftrightarrow$ 5G) gracefully with automatic ICE reconnect.
- **Web Client (React 19 + WASM WebRTC)**:
  - Zero-install remote desktop viewer operating directly in modern web browsers (Chrome, Edge, Safari, Firefox).
  - Utilizes WebRTC, WebCodecs API, and OffscreenCanvas for hardware video decoding in the browser.

### Tier 2: Control Plane (ASP.NET Core 10)
- **Identity & Access Management (IAM)**:
  - Issues short-lived signed JWT access tokens (15-minute validity) and rotating refresh tokens.
  - Enforces Multi-Factor Authentication (MFA / TOTP and FIDO2 / WebAuthn).
  - Manages organizational Role-Based Access Control (RBAC: Admin, Manager, Technician, Read-Only).
- **Device Directory & Presence**:
  - Maintains registry of hardware-fingerprinted devices, their Ed25519 identity public keys, assigned users, and policies.
  - Heartbeat-driven online/offline status using Redis TTL keys with instant WebSocket notification upon state changes.
- **Signaling Hub (ASP.NET Core SignalR)**:
  - Implements lightweight, bidirectional signaling for WebRTC session negotiation.
  - Relays encrypted SDP Offers, SDP Answers, and trickle ICE candidates between peers.
  - Generates time-limited, cryptographically signed TURN credentials (RFC 5766 REST API format: `username = timestamp:userID`, `password = HMAC-SHA1(secret, username)`).
- **Audit & Compliance Engine**:
  - Emits immutable structured audit logs for all security-relevant events (session start, session end, authorization grants, file transfer manifests, permission overrides) to PostgreSQL.

### Tier 3: Transport & Relay Plane (Coturn)
- **Stateless TURN/STUN Infrastructure**:
  - RFC 8489 STUN server discovering external reflex candidates.
  - RFC 8656 TURN relay allocating ephemeral UDP and TCP relay sockets for peers behind symmetric NAT or corporate enterprise firewalls.
  - Authenticates connection requests using HMAC-SHA1 credentials dynamically issued by the ASP.NET Core backend.
  - Strictly blind: Coturn inspects only TURN channel numbers and packet headers; payload data is protected by DTLS 1.3 / SRTP with end-to-end keys negotiated exclusively between host and client.

### Tier 4: Host Architecture (Three-Process Separation)

```
+-------------------------------------------------------------------------------+
| HOST OPERATING SYSTEM                                                         |
|                                                                               |
|  [ Privileged Host Service Daemon (Session 0 / root / LaunchDaemon) ]        |
|    - System-level service, auto-starts on boot                                |
|    - Manages persistent connection to OpenDesk Signaling Hub                  |
|    - Handles device identity private key in OS secure storage                 |
|    - Detects active GUI user session transitions                              |
|    - Spawns Desktop Agent into target user session (e.g., Winlogon or Desktop)|
|            |                                                                  |
|            | OS IPC (Named Pipes / Unix Domain Sockets + Auth Token)          |
|            v                                                                  |
|  [ Interactive Desktop Agent (Active User Session - Rust) ]                  |
|    - Native Screen Capture (DXGI, WGC, ScreenCaptureKit, PipeWire)            |
|    - Hardware Video Encoding (NVENC, AMF, QuickSync, VideoToolbox, VAAPI)    |
|    - Native Audio Capture (WASAPI, CoreAudio, PipeWire)                      |
|    - Synthetic Input Injection (SendInput, CGEvent, libei, /dev/uinput)      |
|    - WebRTC PeerConnection & SCTP Data Channels                               |
|            |                                                                  |
|            | Localhost IPC / WebSocket                                        |
|            v                                                                  |
|  [ OpenDesk Host UI (Tauri v2 + React 19) ]                                   |
|    - User interface for attended support codes, settings, pairing             |
|    - Floating overlay toolbar during active sessions                          |
+-------------------------------------------------------------------------------+
```

---

## 4. Deployment Topologies: Cloud vs. Self-Hosted

| Component | OpenDesk Public Cloud | Self-Hosted Enterprise Docker |
|---|---|---|
| **Identity & DB** | Managed PostgreSQL (Aurora/Cloud SQL) | Official PostgreSQL 17 Docker Container |
| **Cache & Presence**| Managed Redis Cluster | Redis 7.4 Alpine Docker Container |
| **Control Plane** | Multi-tenant ASP.NET Core 10 on Kubernetes | Single-container ASP.NET Core 10 |
| **Ingress & TLS** | Cloudflare / AWS ALB (Automated TLS) | Nginx Reverse Proxy with Let's Encrypt / Custom Cert |
| **Relay Network** | Multi-region Geo-distributed Coturn Pool | Single / Dual Coturn Container with Host Networking |
| **Codebase** | **Identical** (`OpenDesk.Server`) | **Identical** (`OpenDesk.Server`) |
| **Configuration** | Environment Variables & Secrets Manager | `.env` file mounted to Docker Compose |
