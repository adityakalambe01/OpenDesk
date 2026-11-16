# OpenDesk Phase 6 — Desktop Cross-Platform Support (Linux & macOS)

## 1. Phase Objective
Extend native screen capture, hardware video encoding, synthetic input injection, audio loopback streaming, clipboard synchronization, and file transfer capabilities to Linux (Wayland and X11) and macOS (12.3+ Monterey through latest releases).

## 2. Why This Phase Is Necessary
OpenDesk aims to be a universal open-source tool. Enterprise and DevOps environments frequently operate Linux engineering workstations and macOS developer laptops. Delivering native, high-performance capture and control across these operating systems eliminates platform silos.

## 3. Scope
- Implementing Linux Wayland capture via PipeWire and XDG Desktop Portal (`org.freedesktop.portal.ScreenCast`).
- Implementing Linux X11 capture via `XShm` (MIT-SHM) and input injection via `XTest` and `/dev/uinput`.
- Implementing Linux Wayland input injection via `libei` / `org.freedesktop.portal.RemoteDesktop`.
- Implementing macOS screen capture via `ScreenCaptureKit` and hardware encoding via `VideoToolbox`.
- Implementing macOS synthetic input injection via CoreGraphics `CGEventTap` with TCC permission guides.
- Implementing native audio loopback (WASAPI on Windows, CoreAudio on macOS, PipeWire on Linux) with Opus encoding.
- Implementing bidirectional clipboard synchronization and multi-monitor management.
- Implementing file transfer engine over SCTP data channels with resume support.

## 4. Prerequisites
Completion of Phase 5 (Authentication & Security).

## 5. Technical Implementation Details
- `ScreenCaptureKit` on macOS delivers `CVImageBufferRef` (IOSurface) frames directly into Apple's `VTCompressionSession` for zero-copy encoding.
- Linux Wayland uses `ashpd` to negotiate portal sessions and `pipewire` for DmaBuf zero-copy GPU frames into VAAPI.
- Multi-monitor management allows clients to toggle active display outputs or view an all-monitors virtual desktop.

## 6. Exact Folder and File Changes Expected
```
core/crates/opendesk-capture/src/platform/
├── linux/
│   ├── mod.rs
│   ├── pipewire.rs
│   └── x11.rs
└── macos/
    ├── mod.rs
    └── screencapturekit.rs
core/crates/opendesk-input/src/platform/
├── linux/
│   ├── mod.rs
│   ├── ei.rs
│   └── xtest.rs
└── macos/
    ├── mod.rs
    └── cgevent.rs
core/crates/opendesk-capture/src/audio/
├── wasapi.rs
├── coreaudio.rs
└── pipewire_audio.rs
```

## 7. Individual Development Tasks
- **P6-001**: Implement macOS ScreenCaptureKit frame capture and Apple VideoToolbox hardware encoder.
- **P6-002**: Implement macOS CoreGraphics `CGEventTap` input injection and TCC permission validator.
- **P6-003**: Implement Linux Wayland PipeWire DmaBuf capture via XDG Desktop Portal.
- **P6-004**: Implement Linux Wayland `libei` emulated input injection.
- **P6-005**: Implement Linux X11 `XShm` capture and `XTest` input injection fallbacks.
- **P6-006**: Implement native audio loopback capture (WASAPI, CoreAudio, PipeWire) with Opus compression.
- **P6-007**: Implement bidirectional clipboard synchronization (text and images) with loopback suppression.
- **P6-008**: Implement file transfer manager over SCTP data channel with chunk hashing and resume support.
- **P6-009**: Implement multi-monitor topology detection, switching, and combined viewport rendering.

## 8. Dependencies
Phase 5 (Authentication & Security).

## 9. Expected Outputs
Fully functional OpenDesk host and client applications running natively on Windows, macOS, and Linux.

## 10. Unit Testing Requirements
- Unit tests verifying multi-monitor bounding box calculations and virtual coordinate translations.
- Unit tests validating file chunk slicer and SHA-256 rolling hash integrity.

## 11. Integration Testing Requirements
Integration tests running on Linux and macOS CI runners validating capture initialization and encoder output.

## 12. Manual Verification Steps
1. Connect from macOS client to Linux Wayland host $\rightarrow$ Verify smooth video and mouse control.
2. Connect from Windows client to macOS host $\rightarrow$ Verify ScreenCaptureKit stream at 60 FPS.
3. Copy text on host $\rightarrow$ Verify text pastes cleanly on client clipboard.
4. Drag and drop 100 MB test file onto remote viewer $\rightarrow$ Verify file transfers and SHA-256 matches.

## 13. Acceptance Criteria
- Linux (Wayland & X11) and macOS capture sustain $\ge 45\text{ FPS}$ at 1080p.
- Clipboard synchronization completes in $< 250\text{ ms}$.
- File transfer completes with 100% SHA-256 hash match without stalling video streaming.

## 14. Definition of Done
Cross-platform desktop host and client features verified on physical or VM instances of Windows, Linux, and macOS.

## 15. Known Risks
- Permission friction on macOS TCC (Screen Recording / Accessibility).
  - *Mitigation*: Interactive in-app onboarding checklist linking directly to macOS System Settings.

## 16. Estimated Complexity
Very High (Native display subsystems, OS permission models, audio loopback).

## 17. Next-Phase Dependencies
Unlocks **Phase 7 (Mobile Applications)** and **Phase 9 (Self-Hosting)**.
