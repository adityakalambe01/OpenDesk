# OpenDesk Phase 7 — Mobile Applications (Flutter 3.x - iOS & Android)

## 1. Phase Objective
Build, optimize, and deploy the OpenDesk mobile client using Flutter 3.x, enabling engineers and users to remotely control desktop hosts from iOS and Android devices with touch-to-mouse translation, virtual trackpads, on-screen desktop modifier keyboards, and supported mobile host streaming.

## 2. Why This Phase Is Necessary
Technicians and remote administrators require the ability to access critical servers or workstations while on-call or traveling using mobile smartphones and tablets.

## 3. Scope
- Implementing the Flutter 3.x UI with `flutter_webrtc` rendering.
- Implementing dual interaction modes: Virtual Trackpad Mode and Direct Touch Mode.
- Building the ergonomic mobile on-screen keyboard modifier bar (Ctrl, Alt, Shift, Esc, Win, F1-F12, Ctrl+Alt+Del).
- Implementing Android host capabilities using `MediaProjection` screen capture and `AccessibilityService` input injection.
- Implementing iOS screen broadcast using an iOS `ReplayKit Broadcast Upload Extension` (view-only).
- Documenting and enforcing OS-level limitations (e.g. no global unattended input injection on iOS).

## 4. Prerequisites
Completion of Phase 5 (Authentication & Security).

## 5. Technical Implementation Details
- `flutter_webrtc` plugin interfaces with native WebRTC C++ binaries on iOS and Android.
- Ballistic cursor acceleration maps touch velocity to smooth pointer movement on the remote host.
- Android `ForegroundService` with `FOREGROUND_SERVICE_TYPE_MEDIA_PROJECTION` ensures continuous streaming without termination by the Android system.

## 6. Exact Folder and File Changes Expected
```
clients/mobile/
├── lib/
│   ├── features/
│   │   ├── session/
│   │   │   ├── session_controller.dart
│   │   │   ├── remote_canvas_view.dart
│   │   │   ├── trackpad_gesture_detector.dart
│   │   │   └── virtual_keyboard_bar.dart
│   │   └── devices/
│   │       └── device_list_screen.dart
├── android/app/src/main/kotlin/io/opendesk/
│   ├── MediaProjectionService.kt
│   └── InputAccessibilityService.kt
└── ios/
    ├── OpenDeskBroadcastExtension/
    │   └── SampleHandler.swift
    └── Runner/
```

## 7. Individual Development Tasks
- **P7-001**: Scaffold Flutter 3.x project and integrate `flutter_webrtc` and SignalR Dart client.
- **P7-002**: Implement responsive WebRTC video rendering view with hardware acceleration.
- **P7-003**: Implement Virtual Trackpad touch-to-mouse gesture interpreter with ballistic acceleration.
- **P7-004**: Implement Direct Touch mode for tablet and stylus interactions.
- **P7-005**: Implement mobile modifier keyboard overlay bar with latching modifiers.
- **P7-006**: Implement Android native `MediaProjection` screen capture and foreground service.
- **P7-007**: Implement Android native `AccessibilityService` gesture injection.
- **P7-008**: Implement iOS ReplayKit Broadcast Upload Extension for screen streaming (view-only).
- **P7-009**: Conduct usability and network transition testing across 5G, 4G, and Wi-Fi networks.

## 8. Dependencies
Phase 5 (Authentication & Security).

## 9. Expected Outputs
Functional Android and iOS mobile applications capable of controlling remote workstations with touch gestures.

## 10. Unit Testing Requirements
- Unit tests verifying touch gesture coordinate transformation math.
- Unit tests validating modifier key latching and release logic.

## 11. Integration Testing Requirements
Integration tests running on Android and iOS simulators validating WebRTC peer connection establishment.

## 12. Manual Verification Steps
1. Launch OpenDesk on iPhone/Android device.
2. Connect to remote Windows 11 host.
3. Use Virtual Trackpad to launch browser, select text, and double-click icons.
4. Open Notepad $\rightarrow$ Use mobile virtual keyboard and modifier bar to type `Ctrl+S` $\rightarrow$ Verify save dialog appears.
5. Switch phone from Wi-Fi to cellular data $\rightarrow$ Verify stream recovers via ICE restart.

## 13. Acceptance Criteria
- Video playback sustains 60 FPS on modern mobile devices.
- Touch gesture responsiveness feels immediate and intuitive.
- Clear in-app documentation regarding Apple iOS host input restrictions.

## 14. Definition of Done
Android APK and iOS IPA build, deploy to test devices, and enable remote control of desktop machines.

## 15. Known Risks
- Mobile OS memory pressure terminating background capture services.
  - *Mitigation*: Use persistent foreground services with high priority and battery optimization exclusion prompts.

## 16. Estimated Complexity
High (Mobile gesture engineering, native Android services, iOS extensions).

## 17. Next-Phase Dependencies
Unlocks **Phase 10 — Testing, Hardening & Release**.
