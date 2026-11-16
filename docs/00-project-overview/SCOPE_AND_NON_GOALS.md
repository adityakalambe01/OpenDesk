# OpenDesk — Project Scope & Non-Goals

## 1. Project Scope Boundaries

### In Scope for Initial Releases (v0.1 – v1.0)

1. **Native Screen Capture & Hardware Encoding**:
   - High-throughput screen capture on Windows (DXGI/WGC), macOS (ScreenCaptureKit), and Linux (PipeWire / X11).
   - Dynamic H.264 video encoding utilizing hardware acceleration (NVENC, AMF, QuickSync, VideoToolbox, VAAPI) and OpenH264 fallback.
2. **Precision Input Handling**:
   - Sub-5ms transmission of mouse clicks, movements, scrolls, and normalized international keyboard events.
   - Special keystroke injection (e.g., Ctrl+Alt+Del, Super/Meta key, system shortcuts).
3. **Robust Dual-Mode Sessions**:
   - **Attended Remote Support**: Ephemeral 9-digit session IDs, host interactive approval, floating on-screen toolbar with kill switch.
   - **Unattended Remote Access**: Cryptographic device pairing, pre-shared Argon2id master secrets or asymmetric user public keys, background OS service integration.
4. **Resilient WebRTC Transport**:
   - Direct P2P connectivity over STUN with ICE candidate harvesting.
   - Authenticated zero-knowledge coturn relay fallback for symmetric NAT/firewall traversal.
5. **Bidirectional Productivity Features**:
   - Audio streaming (WASAPI, CoreAudio, PipeWire) via Opus codec.
   - Secure clipboard synchronization (text and images).
   - Interactive bidirectional file transfer manager over SCTP data channels with resume capability.
   - Multi-monitor detection, switching, and combined viewport modes.
6. **Unified Dual Deployment**:
   - Single ASP.NET Core 10 backend supporting multi-tenant Public Cloud and single-tenant Self-Hosted Docker Compose deployments with 100% feature and code parity.

---

## 2. Explicit Non-Goals

To maintain rigorous architectural discipline and ensure predictable delivery, the following areas are strictly designated as **Non-Goals**:

### 1. Ultra-Low Latency Cloud Gaming (60–240 FPS HDR, Gamepad Emulation)
*Rationale*: OpenDesk is engineered for remote administration, desktop productivity, and technical support. It does not prioritize features specific to game streaming, such as virtual Xbox/PlayStation controller emulation, variable refresh rate (G-Sync/FreeSync) pass-through, 10-bit HDR gaming color spaces, or sub-5ms extreme gaming frame pipelining (which tools like Sunshine/Moonlight address).

### 2. Bypassing Operating System Security Boundaries or MDM Policies
*Rationale*: OpenDesk will **never** implement rootkits, unsigned kernel drivers, jailbreaks, or anti-tamper circumvention techniques.
- On Windows: UAC prompts must be handled via standard Microsoft accessibility interfaces or elevated background services signed with valid certificates.
- On macOS: Screen recording and Accessibility permissions must be explicitly granted by the user through macOS System Settings (TCC). We will not inject code into system processes to bypass TCC.
- On iOS: Apple sandboxing strictly prohibits global background touch injection into third-party apps or the SpringBoard. OpenDesk will **not** claim to provide unattended full remote control of an iOS device. iOS will operate as a Remote Controller Client, with screen broadcast available via ReplayKit Broadcast Extensions.
- On Android: Screen capture requires the standard Android `MediaProjection` consent dialog; touch injection relies on standard user-enabled `AccessibilityService`. We will not exploit OS vulnerabilities to silently gain control.

### 3. Server-Side Plaintext Relay Inspection or Decryption
*Rationale*: The OpenDesk relay infrastructure is strictly zero-knowledge. Under no circumstances will OpenDesk implement server-side SSL/TLS stripping, DTLS termination, or man-in-the-middle decryption of user screen streams, keystrokes, or files. Relay nodes only forward encrypted UDP/TCP packets.

### 4. Direct Public Inbound Port Exposure for Host Daemons
*Rationale*: Host machines will **never** require opening inbound router firewall ports (e.g., port forwarding port 3389 or 5900) or exposing unauthenticated listening sockets directly to the public internet. All connections are negotiated outbound via WebRTC signaling and ICE NAT hole punching or outbound authenticated relays.

### 5. Proprietary or Forked Protocols
*Rationale*: OpenDesk standardizes on open internet protocols: WebRTC, ICE (RFC 8445), STUN (RFC 8489), TURN (RFC 8656), DTLS 1.3 (RFC 9147), and SCTP (RFC 4960). Custom binary protocols are restricted strictly to application-layer payloads transmitted over standardized secure channels.

### 6. Premature Microservices Decomposition
*Rationale*: The backend shall begin as a modular, highly cohesive ASP.NET Core 10 service rather than a fragmented fleet of microservices. Decoupling into microservices will only occur if concrete scaling boundaries demand it.

---

## 3. Scope Evolution by Release Horizon

```
+----------------------------------------------------------------------------------+
| Release Horizon                                                                  |
+----------------------------------------------------------------------------------+
| v0.1 (Milestone 1) : Windows P2P screen streaming & input injection over LAN     |
| v0.5 (Milestone 2) : Cross-platform desktop (Win/Mac/Linux), NAT traversal       |
| v0.8 (Milestone 3) : Attended support codes, file transfer, clipboard, audio     |
| v1.0 (Production)  : Mobile controllers (iOS/Android), multi-monitor, self-host  |
| v2.0 (Future)      : Collaborative multi-viewer sessions, enterprise SAML/OIDC   |
+----------------------------------------------------------------------------------+
```
