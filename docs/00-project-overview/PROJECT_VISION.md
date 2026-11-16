# OpenDesk — Project Vision & Mission

## 1. Executive Summary

**OpenDesk** is a modern, production-grade, open-source remote desktop platform engineered to deliver low-latency, end-to-end encrypted screen capture, audio streaming, input control, and file transfer across desktop and mobile operating systems. OpenDesk serves as a sovereign, self-hostable, and auditable alternative to proprietary commercial solutions such as TeamViewer, AnyDesk, and Splashtop, and a modular, clean-architecture successor to first-generation open-source tools like RustDesk and Apache Guacamole.

The platform is designed around strict separation of concerns:
- **Rust Core Engine (`libopendesk`)**: High-performance, memory-safe native core managing platform screen capture, hardware-accelerated video/audio encoding, synthetic input injection, cryptographic identity, and WebRTC peer-to-peer transport.
- **Desktop UI (Tauri v2 + React 19 + TypeScript)**: Clean, lightweight user interface with minimal memory footprint, decoupled from the core background daemon.
- **Mobile Client (Flutter 3.x / Dart)**: Native touch-optimized remote controller and presentation client for iOS and Android.
- **Control Plane Backend (ASP.NET Core 10 & EF Core)**: Enterprise-grade identity, device registry, session signaling via WebSockets/SignalR, role-based authorization, and audit logging.
- **Data & Transport Plane (Coturn + P2P WebRTC / DTLS-SRTP)**: Zero-knowledge relay forwarding where intermediate servers never possess session decryption keys.

---

## 2. Core Pillars

```
+-----------------------------------------------------------------------------------+
|                                  OPENDESK PILLARS                                 |
+-------------------+--------------------+--------------------+---------------------+
| 1. Zero-Trust     | 2. Native          | 3. Dual Deployment | 4. Headless & Daemon |
|    Security       |    Performance     |    Parity          |    Architecture     |
| E2EE, Ephemeral   | Hardware capture & | 100% feature match | Unattended service  |
| keys, signed P2P  | encoding, sub-30ms | between Cloud &    | independent of user |
| authentication    | glass-to-glass     | Self-Hosted Docker | login session       |
+-------------------+--------------------+--------------------+---------------------+
```

### Pillar 1: Zero-Trust Security by Design
Every connection is end-to-end encrypted using Curve25519 (X25519 key exchange) and ChaCha20-Poly1305 / AES-256-GCM authenticated encryption. The signaling server and relay infrastructure act strictly as blind forwarders; relay nodes forward encrypted byte streams and cannot decrypt video, audio, keystrokes, or clipboard data. Devices maintain autonomous cryptographic key pairs that form immutable device identities.

### Pillar 2: Native High-Performance Pipelines
Rather than relying on cross-platform abstraction layers that compromise performance or introduce tearing, OpenDesk implements dedicated native capture pipelines:
- **Windows**: Modern Windows Graphics Capture (`Direct3D11CaptureFramePool`) and Desktop Duplication API (DXGI) with hardware NVENC/AMF/QuickSync/VAAPI H.264/H.265/AV1 encoding.
- **macOS**: ScreenCaptureKit framework on macOS 12.3+ utilizing Apple Silicon VideoToolbox hardware encoders.
- **Linux**: Wayland desktop capture via PipeWire and XDG Desktop Portal (`org.freedesktop.portal.ScreenCast`), with fallback to X11 XShm/XComposite and hardware VAAPI encoding.
- **Target Glass-to-Glass Latency**: ≤ 30 ms over 1 Gbps local LAN; ≤ 65 ms over WAN P2P connections under 50 ms RTT.

### Pillar 3: Dual Deployment Parity (Cloud & Self-Hosted)
OpenDesk delivers 100% architectural parity between its multi-tenant Public Cloud offering and the single-tenant Self-Hosted Docker Compose stack. Self-hosters run the exact same container images, database schemas, and signaling protocols as the public cloud, avoiding proprietary lock-in or artificial enterprise paywalls.

### Pillar 4: Autonomous Headless Host & User Separation
The remote desktop host engine executes as an independent OS-level service (`Windows Service`, `systemd` daemon, macOS `LaunchDaemon`/`LaunchAgent`), allowing remote management of machines before user logon, across session switches, and across UAC / Privilege Elevation boundaries without terminating the remote connection.

---

## 3. Primary Access Modes

### A. Unattended Remote Access (Administrative / Personal Fleet)
- **Target Audience**: System administrators, DevOps engineers, MSPs (Managed Service Providers), and remote professionals accessing their dedicated workstations or servers.
- **Workflow**:
  1. Host computer is registered to an OpenDesk account or authorized self-hosted instance.
  2. Device creates an asymmetric key pair; the public key is registered with the backend device directory.
  3. Pre-shared secret or user credential pairing enables unattended access.
  4. Remote client initiates connection: identity and authorization are verified against the backend and device key.
  5. The host accepts the connection without interactive user confirmation, applying the device owner's pre-configured security policies (e.g., auto-lock on disconnect, blank screen, mute audio).
- **Security Guardrails**:
  - Requires explicit device-owner pre-authorization during pairing.
  - Can be restricted by IP allowlists, hardware token MFA (FIDO2/WebAuthn), and time-bound access windows.
  - Revocable instantly from the central dashboard or locally on the host machine.

### B. Remote Support (Attended Helpdesk / Ephemeral Sessions)
- **Target Audience**: Helpdesk technicians, IT support teams, and peer-to-peer assistance.
- **Workflow**:
  1. The host (end-user needing support) opens OpenDesk desktop or mobile app.
  2. App generates a cryptographically random, rate-limited, time-sensitive 9-digit Session ID and 6-character alphanumeric PIN.
  3. End-user shares the Session ID with the support technician.
  4. Technician requests connection.
  5. The host displays an interactive dialog: *"Aditya from IT is requesting access to your screen. Grant Control? [View Only] [Full Control] [Reject]"*.
  6. The session begins only upon affirmative user approval.
- **Security Guardrails**:
  - Ephemeral credentials expire automatically (default: 10 minutes from generation or upon session completion).
  - Explicit granular permissions: View Only vs. Mouse/Keyboard Control vs. Clipboard vs. File Transfer.
  - Prominent persistent on-screen banner with a one-click **"Terminate Session"** kill switch.
  - Temporary support access **never** transitions into unattended access without a distinct, multi-step pairing ritual requiring elevated host credentials.

---

## 4. Market Differentiation vs. Existing Solutions

| Dimension | TeamViewer / AnyDesk | RustDesk | Apache Guacamole | OpenDesk |
|---|---|---|---|---|
| **License & Source** | Proprietary / Commercial | Dual License / AGPLv3 Core | Apache 2.0 | Apache 2.0 & MIT Permissive |
| **Backend Stack** | Proprietary Cloud | Rust (`hbbs`/`hbbr`) | Java (`guacd` C daemon) | ASP.NET Core 10 / C# |
| **Relay Architecture** | Centralized Proprietary | Custom TCP/UDP Relay | HTTP/WebSocket proxy to RDP/VNC | WebRTC P2P + coturn TURN/STUN + E2EE |
| **Screen Capture API** | Proprietary drivers | Scraping / DXGI / X11 | Remote protocol bridge | Modern Native (WGC, SCKit, PipeWire) |
| **Self-Hosting Parity** | Very limited / Enterprise only | Supported | Supported (gateway only) | 100% First-Class Parity with Cloud |
| **Client UI** | Native / C++ / Qt | Flutter (Desktop & Mobile) | Web Browser (HTML5 Canvas) | Tauri v2 + React (Desktop), Flutter (Mobile) |
| **Audit & Governance** | Enterprise plan only | Basic file logging | Basic session recording | Structured OpenTelemetry, Audit Trail, DB Log |

---

## 5. Success Metrics & Performance KPIs

1. **Connection Success Rate**: ≥ 92% direct P2P connection rate via STUN/ICE across standard NAT/firewalls; 100% fallback connection rate via authenticated TURN relays.
2. **Video Performance**: 60 FPS at 1080p, 30 FPS at 4K with adaptive dynamic bitrate (500 Kbps - 25 Mbps) based on real-time WebRTC congestion control (TWCC / GCC).
3. **Resource Footprint**:
   - Host Background Daemon (Idle): < 25 MB RAM, 0% CPU.
   - Host Background Daemon (Active 1080p60 Stream): < 150 MB RAM, < 5% CPU with hardware encoder.
   - Tauri Desktop Client (Idle): < 60 MB RAM.
4. **Security Integrity**: 0 unauthenticated access vulnerabilities, 0 plaintext data leaks through relay infrastructure, full OWASP ASVS Level 2 compliance.
