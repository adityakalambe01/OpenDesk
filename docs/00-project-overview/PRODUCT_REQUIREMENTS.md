# OpenDesk — Product Requirements Document (PRD)

## 1. Document Control
- **Document Version**: 1.0.0
- **Status**: Approved Architectural Baseline
- **Classification**: Open Source Specification

---

## 2. Functional Requirements (FR)

### FR-01: Device Registration & Management
- **FR-01.1**: The system shall generate a globally unique, persistent 64-bit device identifier (`DeviceID`) derived from cryptographic hardware fingerprinting and an ephemeral installation seed.
- **FR-01.2**: Devices shall generate an asymmetric Ed25519 identity keypair upon initial launch. The public key shall be registered with the OpenDesk directory service.
- **FR-01.3**: Authenticated users shall be able to claim/bind a device to their user account via an authenticated pairing handshake (QR Code, 6-digit verification code, or account sign-in).
- **FR-01.4**: Users shall be able to assign friendly aliases, organizational tags, and group memberships to registered devices.
- **FR-01.5**: Users shall be able to view real-time online/offline presence status, system telemetry (OS, CPU, RAM, active user, network latency), and last-seen timestamps for all bound devices.
- **FR-01.6**: Device owners shall have the ability to unbind, revoke, or permanently wipe registration tokens for any assigned device from the web dashboard or desktop client.

### FR-02: Attended Remote Support Sessions
- **FR-02.1**: The application shall provide an "Instant Support" UI that displays a one-time 9-digit Session Code and a 6-character alphanumeric PIN.
- **FR-02.2**: The Session Code shall expire after 10 minutes of inactivity or immediately upon session completion.
- **FR-02.3**: Upon incoming connection requests, the host client shall display an interactive modal dialog detailing:
  - Connecting technician's verified username, IP address, and geographic location estimate.
  - Requested permission scope (View Only vs. Interactive Control vs. File Transfer).
- **FR-02.4**: The host user must explicitly click "Accept" or "Reject". If no response is received within 60 seconds, the request shall automatically reject with a timeout error.
- **FR-02.5**: The host user must be presented with a persistent floating on-screen overlay toolbar indicating:
  - Active session indicator (e.g., green recording badge and glowing screen border).
  - Name of connected party.
  - Independent toggles for: [Mute Remote Input], [Block Remote Clipboard], [Pause Video Stream].
  - A prominent red **"Disconnect Immediately"** button.

### FR-03: Unattended Remote Access
- **FR-03.1**: The host service shall support unattended access authenticated via a secure pre-shared Master Password (hashed locally with Argon2id) or an asymmetric authorized public key bound to the user's account.
- **FR-03.2**: When accessing an unattended host, the connecting client must present valid authorization credentials (device token + cryptographic challenge response signed by the authorized private key).
- **FR-03.3**: The host system shall support executing as a background system service (Windows Service / systemd daemon / macOS LaunchDaemon), capable of streaming the Windows Logon (GINA/Credential Provider) screen and Linux display managers (GDM/LightDM/SDDM).
- **FR-03.4**: The unattended host shall support post-session policies:
  - Automatically lock the operating system workstation upon session termination.
  - Enable "Curtain Mode" / Screen Blanking where supported by native display drivers.
  - Disable local audio playback during remote streaming.

### FR-04: Video Capture, Encoding, and Streaming
- **FR-04.1**: The capture engine shall acquire desktop screen frames with minimal CPU overhead using native hardware capture APIs:
  - Windows: Desktop Duplication (DXGI 1.2+) and Windows Graphics Capture (Windows 10 1903+).
  - macOS: ScreenCaptureKit (macOS 12.3+), CGDisplayStream (legacy fallback).
  - Linux: PipeWire via XDG Desktop Portal `org.freedesktop.portal.ScreenCast` (Wayland) and X11 XShm/MIT-SHM.
- **FR-04.2**: The engine shall feed captured video frames into hardware-accelerated video encoders:
  - NVENC (NVIDIA), AMF (AMD), QuickSync / OneVPL (Intel), VideoToolbox (Apple Silicon), VAAPI (Linux).
  - Fallback: Software OpenH264 or SVT-AV1 / VP9 when hardware encoders are absent.
- **FR-04.3**: The video pipeline shall support H.264 (baseline/main/high profiles), VP9, and AV1.
- **FR-04.4**: The engine shall dynamically adapt video bitrate, framerate (15–60 FPS), and quantization parameters based on WebRTC Google Congestion Control (GCC) and Transport Wide Congestion Control (TWCC) feedback.
- **FR-04.5**: The system shall support multi-monitor setups, allowing the client to view all monitors simultaneously, cycle through individual monitors, or switch active displays dynamically.

### FR-05: Input Injection & Remote Control
- **FR-05.1**: Mouse movements, clicks (left, right, middle, extra buttons), mouse wheel scrolling (vertical and horizontal), and dragging events shall be transmitted with sub-5ms serialization latency.
- **FR-05.2**: Keyboard events (scancodes, key down, key up, modifiers: Shift, Ctrl, Alt, Meta/Super) shall be normalized using standard W3C UI Events KeyboardEvent `code` values and mapped to native platform scancodes.
- **FR-05.3**: System key combinations (e.g., Ctrl+Alt+Del on Windows, Ctrl+Alt+Backspace on Linux, Cmd+Opt+Esc on macOS) shall be injectable via dedicated toolbar actions.
- **FR-05.4**: The host shall support disabling local input while remote input is active, and vice versa.

### FR-06: Audio Streaming
- **FR-06.1**: The host shall capture desktop system audio output using WASAPI loopback (Windows), CoreAudio process tap (macOS), and PulseAudio/PipeWire monitor sinks (Linux).
- **FR-06.2**: Audio shall be encoded using the Opus codec (16 kHz - 48 kHz dynamic bitrate, 16–64 Kbps) and multiplexed across the WebRTC media stream synchronized with video (lip-sync alignment within ±20 ms).

### FR-07: Bidirectional Clipboard Synchronization
- **FR-07.1**: The system shall synchronize plaintext and Unicode text clipboard contents bidirectionally when authorized.
- **FR-07.2**: The system shall synchronize image clipboard data (PNG/BMP formats) up to a configurable maximum size (default: 10 MB).
- **FR-07.3**: The host user shall have the ability to restrict clipboard synchronization to one-way (Host-to-Client only or Client-to-Host only) or disable it completely.

### FR-08: File Transfer Engine
- **FR-08.1**: The system shall provide a dedicated bidirectional File Manager UI displaying local and remote file directory trees.
- **FR-08.2**: File transfers shall execute over a dedicated reliable WebRTC SCTP Data Channel or QUIC stream.
- **FR-08.3**: File transfer shall support pause, resume, cancel, integrity validation (SHA-256 chunk checksums), and dynamic bandwidth throttling.
- **FR-08.4**: Direct drag-and-drop of files onto the remote desktop viewer window shall automatically queue file transfers to a designated remote "Received Files" directory.

---

## 3. Non-Functional Requirements (NFR)

### NFR-01: Performance & Latency
- **Glass-to-Glass Latency**: ≤ 30 ms over LAN; ≤ 65 ms over WAN under 50 ms network RTT.
- **Frame Rate**: Adaptive 30–60 FPS based on network bandwidth and CPU/GPU budget.
- **Client Render Latency**: ≤ 8 ms per frame on desktop; ≤ 12 ms per frame on mobile devices.
- **Connection Setup Time**: WebRTC peer-to-peer session establishment shall complete in < 1.5 seconds under direct ICE connectivity, and < 3.0 seconds when relay fallback is required.

### NFR-02: Security & Cryptography
- **Encryption**: All transport channels must be protected with DTLS 1.3 / SRTP for media and SCTP over DTLS for data channels.
- **End-to-End Key Exchange**: X25519 authenticated Diffie-Hellman key exchange directly between host and client; signaling servers never possess session keys.
- **Authentication**: JWT tokens with short expiry (15 min) for API access, backed by cryptographically signed refresh tokens stored in secure OS keychains (Windows Credential Locker, macOS Keychain, Linux Secret Service / Keyutils).
- **Password Hashing**: Argon2id with memory cost $m=65536$ (64 MiB), iterations $t=3$, parallelism $p=4$.

### NFR-03: Scalability & High Availability
- **Signaling Server**: Horizontally scalable stateless ASP.NET Core 10 backend nodes communicating via Redis Pub/Sub backplane.
- **Concurrent Connections**: Single signaling node shall support ≥ 50,000 active concurrent WebSocket presence connections.
- **Relay Server**: Coturn cluster deployable behind DNS round-robin or Anycast IP addressing for global geo-distributed TURN forwarding.

### NFR-04: Reliability & Network Resilience
- **Reconnection**: The client shall automatically attempt ICE restart and connection recovery upon transient network dropouts (e.g., Wi-Fi to cellular handover) with an exponential backoff window of 10 seconds before declaring session loss.
- **Packet Loss Tolerance**: Resilient video streaming without complete picture freeze up to 15% random packet loss via Forward Error Correction (ULPFEC / FlexFEC) and dynamic intra-refresh (IDR request) frames.

---

## 4. Operating System Compatibility Matrix

| OS Platform | Architecture | Min. OS Version | Capture Engine | Input Injection | Headless Service | Remote Host | Remote Client |
|---|---|---|---|---|---|:---:|:---:|
| **Windows** | x86_64, ARM64 | Windows 10 (1809+) / Server 2019+ | DXGI Duplication, WGC | `SendInput`, Virtual HID | Windows Service | Yes | Yes |
| **Linux (X11)** | x86_64, AArch64 | Ubuntu 22.04+, Debian 12+, Fedora 38+ | XShm, XComposite | XTest extension, `/dev/uinput` | systemd daemon | Yes | Yes |
| **Linux (Wayland)** | x86_64, AArch64 | GNOME 42+, KDE Plasma 5.27+ | PipeWire + Portal | `libei` / Portal RemoteDesktop | systemd daemon | Yes | Yes |
| **macOS** | x86_64, Apple Silicon | macOS 12.3 (Monterey)+ | ScreenCaptureKit | CoreGraphics `CGEvent` | LaunchDaemon / Agent | Yes | Yes |
| **Android** | AArch64, ARMv7 | Android 10 (API 29)+ | MediaProjection API | AccessibilityService (User-enabled) | Foreground Service | Yes (Limited) | Yes |
| **iOS / iPadOS** | ARM64 | iOS 16.0+ | ReplayKit Broadcast Ext. | N/A (OS restriction prevents injection) | N/A | Broadcast Only | Yes |
| **Web Browser** | WASM / WebRTC | Chrome 110+, Safari 16+, Firefox 115+ | `getDisplayMedia` | N/A (View Only) | N/A | No | Yes (Viewer) |
