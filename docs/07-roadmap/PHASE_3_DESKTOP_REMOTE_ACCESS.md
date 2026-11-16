# OpenDesk Phase 3 — First Working Desktop Connection (Windows)

## 1. Phase Objective
Achieve a working Windows-to-Windows peer-to-peer remote desktop connection over a trusted local test network, proving out the complete glass-to-glass pipeline: native screen capture, hardware video encoding, real-time streaming, and synthetic mouse/keyboard input injection.

## 2. Why This Phase Is Necessary
Validating the end-to-end media and input pipeline early on a single, well-defined platform (Windows) mitigates integration risks, proves latency targets ($\le 30\text{ ms}$ over LAN), and validates hardware encoder interoperability before introducing complex WAN signaling and cross-platform abstractions.

## 3. Scope
- Implementing Windows screen capture using Windows Graphics Capture (WGC) and DXGI Desktop Duplication.
- Implementing hardware-accelerated H.264 video encoding via NVIDIA NVENC, Intel QuickSync, or OpenH264 fallback.
- Implementing Windows synthetic input injection using Win32 `SendInput`.
- Constructing a local P2P WebRTC transport pipeline between host and client.
- Displaying the remote screen inside the Tauri React desktop client canvas viewport.
- Injecting mouse and keyboard events from the client back to the host.

## 4. Prerequisites
Completion of Phase 1 (Setup) and Phase 2 (Core Engine interfaces).

## 5. Technical Implementation Details
- Windows capture binds to Direct3D 11 textures (`ID3D11Texture2D`).
- Direct3D 11 texture handles are shared directly with the hardware encoder to achieve zero-copy GPU encoding.
- Input normalizer in React translates DOM mouse events into normalized float coordinates and physical scancodes.
- Direct WebRTC connection negotiated over local LAN using manual or file-based SDP exchange.

## 6. Exact Folder and File Changes Expected
```
core/crates/opendesk-capture/src/platform/windows/
├── mod.rs
├── dxgi.rs
└── wgc.rs
core/crates/opendesk-encoder/src/platform/windows/
├── mod.rs
├── nvenc.rs
└── qsv.rs
core/crates/opendesk-input/src/platform/windows/
├── mod.rs
└── sendinput.rs
clients/desktop/src/components/RemoteViewer/
├── RemoteViewer.tsx
├── CanvasRenderer.tsx
└── InputCaptureOverlay.tsx
```

## 7. Individual Development Tasks
- **P3-001**: Implement Windows DXGI Desktop Duplication capture driver.
- **P3-002**: Implement Windows Graphics Capture (WGC) modern capture driver with yellow border suppression.
- **P3-003**: Implement hardware H.264 video encoder integration (NVENC / QuickSync) with OpenH264 software fallback.
- **P3-004**: Implement Windows Win32 `SendInput` mouse and keyboard injection driver.
- **P3-005**: Integrate `webrtc-rs` local PeerConnection pipeline with video track and SCTP data channel.
- **P3-006**: Implement React 19 Remote Viewer canvas component with WebRTC video stream attachment.
- **P3-007**: Wire client mouse and keyboard event listeners to Tauri IPC `send_input` commands.
- **P3-008**: Conduct end-to-end latency and frame rate verification over local test network.

## 8. Dependencies
Phase 2 (Rust Core Engine).

## 9. Expected Outputs
A functional Windows desktop client capable of connecting to a local Windows host, viewing the desktop at 60 FPS 1080p, and interacting with full mouse and keyboard control.

## 10. Unit Testing Requirements
- Unit tests verifying Win32 scancode mapping tables.
- Unit tests validating coordinate bounds clamping in Windows input injector.

## 11. Integration Testing Requirements
Integration test capturing 100 test frames from a virtual display, encoding to H.264, and validating successful decoder output.

## 12. Manual Verification Steps
1. Launch Host application on Windows Test Machine A.
2. Launch Client application on Windows Test Machine B on the same LAN subnet.
3. Establish session $\rightarrow$ Verify remote desktop displays smoothly.
4. Move mouse and type in Notepad $\rightarrow$ Verify text appears with sub-30ms perceptual latency.
5. Resize client window $\rightarrow$ Verify video scales smoothly without tearing.

## 13. Acceptance Criteria
- Video stream sustains $\ge 50\text{ FPS}$ at $1920\times 1080$ resolution over 1 Gbps LAN.
- Perceptual glass-to-glass latency $\le 30\text{ ms}$.
- Mouse clicks, dragging, double-clicking, and text input function reliably.

## 14. Definition of Done
Windows-to-Windows remote connection verified over local network with hardware encoding and working input injection.

## 15. Known Risks
- `DXGI_ERROR_ACCESS_LOST` during display mode changes or UAC elevation.
  - *Mitigation*: Implement automatic pipeline teardown and re-initialization retry loops.

## 16. Estimated Complexity
High (DirectX 11, hardware video encoder APIs, Win32 subsystems).

## 17. Next-Phase Dependencies
Unlocks **Phase 4 — Networking & Connectivity (WebRTC & Traversal)**.
