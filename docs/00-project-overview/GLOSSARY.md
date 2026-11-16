# OpenDesk — Technical Glossary & Terminology

This glossary establishes canonical definitions for technical terms, protocol acronyms, and operating system concepts used throughout the OpenDesk codebase and documentation.

---

## 1. Networking & Transport Protocols

| Term / Acronym | Definition & OpenDesk Context |
|---|---|
| **WebRTC** | *Web Real-Time Communication*. An open framework standard enabling real-time audio, video, and data transmission over peer-to-peer connections. OpenDesk utilizes WebRTC as its primary transport plane. |
| **ICE** | *Interactive Connectivity Establishment* (RFC 8445). A framework used by WebRTC to discover and negotiate the most direct and efficient network path between two peers using STUN and TURN. |
| **STUN** | *Session Traversal Utilities for NAT* (RFC 8489). A protocol allowing a client behind NAT to discover its public IP address and NAT mapping behavior. |
| **TURN** | *Traversal Using Relays around NAT* (RFC 8656). A protocol providing an authenticated relay server for forwarding encrypted media and data when direct P2P NAT hole punching fails. |
| **SDP** | *Session Description Protocol* (RFC 8866). A standard format for describing multimedia communication sessions (codecs, resolutions, network candidates, encryption parameters). |
| **DTLS** | *Datagram Transport Layer Security* (RFC 9147). Provides TLS-grade encryption and authentication for datagram-based communications. Secures WebRTC media keys and data channels. |
| **SRTP** | *Secure Real-time Transport Protocol* (RFC 3711). Provides encryption, message authentication, and integrity verification for real-time audio and video packets. |
| **SCTP** | *Stream Control Transmission Protocol* (RFC 4960). Provides reliable or partially-reliable, ordered or unordered data streams over DTLS in WebRTC. Used by OpenDesk for input events, clipboard sync, and file transfer. |
| **coturn** | An open-source, high-performance implementation of RFC 5389 (STUN) and RFC 5766/8656 (TURN). Serves as OpenDesk's default relay backend. |
| **SignalR** | An ASP.NET Core framework enabling real-time bi-directional communication over WebSockets, Server-Sent Events, or Long Polling. Used for OpenDesk signaling. |

---

## 2. Operating System & Capture Subsystems

| Term / Acronym | Definition & OpenDesk Context |
|---|---|
| **DXGI Desktop Duplication** | *DirectX Graphics Infrastructure Desktop Duplication API*. Introduced in Windows 8, provides fast access to desktop frame buffers directly in GPU memory. Used for high-efficiency Windows capture. |
| **WGC** | *Windows Graphics Capture*. Modern Windows 10 (1903+) capture API supporting per-window, per-display, and cursor-included screen capture with yellow border control. |
| **PipeWire** | A multimedia server subsystem for Linux that handles audio and video routing. Provides secure screen capture on modern Wayland desktop environments. |
| **XDG Desktop Portal** | A standardized D-Bus interface (`org.freedesktop.portal.ScreenCast`) allowing sandboxed and non-sandboxed Linux applications to request screen capture and input injection from the compositor. |
| **X11 / Xorg** | The legacy Linux windowing system. OpenDesk captures X11 using `XShm` (MIT-SHM shared memory) and injects input via `XTest` or `/dev/uinput`. |
| **ScreenCaptureKit** | High-performance screen recording and streaming framework introduced in macOS 12.3 Monterey, providing direct access to GPU frame buffers with minimal CPU overhead. |
| **Session 0 Isolation** | Windows security architecture introduced in Vista separating background services (Session 0) from interactive user desktop sessions (Session 1+). OpenDesk uses a privileged service coupled with user-session desktop agents to cross this boundary. |
| **TCC** | *Transparency, Consent, and Control*. The macOS security framework regulating access to sensitive resources, specifically Screen Recording and Accessibility permissions. |
| **MediaProjection** | Android API (API 21+) allowing applications to capture screen contents upon explicit user confirmation via an OS system dialog. |
| **ReplayKit** | Apple framework allowing iOS apps to record or broadcast screen contents. Used by OpenDesk iOS via a Broadcast Upload Extension. |

---

## 3. Cryptography & Security

| Term / Acronym | Definition & OpenDesk Context |
|---|---|
| **Ed25519** | Edwards-curve Digital Signature Algorithm over Curve25519. Used for OpenDesk device identity, authentication challenges, and binary release verification. |
| **X25519** | Elliptic Curve Diffie-Hellman (ECDH) key exchange protocol over Curve25519. Used for ephemeral end-to-end key agreement between host and client. |
| **ChaCha20-Poly1305** | An authenticated encryption with associated data (AEAD) cipher (RFC 8439) combining the ChaCha20 stream cipher with the Poly1305 authenticator. Primary cipher for internal control streams. |
| **Argon2id** | Memory-hard password hashing function winning the Password Hashing Competition (RFC 9106). Used for hashing unattended master passwords. |
| **Zero-Knowledge Relay** | A relay architecture in which data packets passing through the relay server are encrypted with keys known only to the endpoints; the relay operator cannot inspect or tamper with payloads. |
| **Device Fingerprint** | A deterministic hash derived from non-volatile hardware characteristics (CPU serial, BIOS UUID, primary MAC address) combined with an application salt to identify a host. |

---

## 4. OpenDesk Architecture Roles

| Role | Definition & Responsibility |
|---|---|
| **Host (Controlled Device)** | The computer or mobile device whose screen is captured, audio streamed, and whose input/files are controlled. Runs the background host engine. |
| **Client / Controller** | The device operated by the human user (technician or device owner) who views the remote screen and injects mouse/keyboard commands. |
| **Control Plane (Backend)** | The central ASP.NET Core server managing user identities, device registries, session metadata, authorization tokens, and WebSocket signaling. |
| **Relay Plane (Coturn)** | Stateless STUN/TURN servers deployed globally to forward encrypted UDP/TCP packets between peers when direct P2P connections cannot be established. |
| **Desktop Daemon** | The persistent, unattended OS service running on the Host machine with elevated OS privileges. |
| **Desktop Agent** | The user-session process running in the logged-in user's desktop context, responsible for native capture and input injection within that user's display session. |
| **Tauri UI Shell** | The desktop user interface application (React/TypeScript rendered inside Tauri's webview), communicating with the local host daemon via IPC. |
