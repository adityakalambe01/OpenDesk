# OpenDesk — Known Technical Limitations & Operating System Boundaries

OpenDesk provides transparent, accurate technical assessments of platform-specific boundaries. The following limitations stem directly from operating system security models, kernel sandboxing, and hardware architectures.

---

## 1. Microsoft Windows Limitations

| Limitation | Technical Cause | Architectural Mitigation / Workaround |
|---|---|---|
| **Session 0 Isolation** | Windows Vista+ strictly isolates background services (`Session 0`) from interactive desktop sessions (`Session 1+`). Services cannot access display surfaces or inject input. | OpenDesk implements a three-process model: `OpenDeskService.exe` (Session 0) detects active sessions and spawns `OpenDeskAgent.exe` directly into the active interactive console session using `CreateProcessAsUser`. |
| **Secure Desktop / UAC Prompts** | User Interface Privilege Isolation (UIPI) blocks standard applications from receiving or sending input to elevated UAC dialogs or the Windows logon screen. | The Desktop Agent binary is embedded with `<requestedExecutionLevel level="asInvoker" uiAccess="true" />`, code-signed with a trusted certificate, and installed into `C:\Program Files\OpenDesk\`. |
| **Headless Windows Servers** | DXGI and Windows Graphics Capture fail to acquire frames if no physical monitor or active display adapter output is detected. | Unattended headless servers require an inexpensive HDMI dummy plug or the OpenDesk Virtual Display Driver (Backlog item BL-004). |

---

## 2. Apple macOS Limitations

| Limitation | Technical Cause | Architectural Mitigation / Workaround |
|---|---|---|
| **Mandatory TCC User Prompts** | Apple's Transparency, Consent, and Control (TCC) prohibits applications from recording screens (`kTCCServiceScreenCapture`) or injecting input (`kTCCServiceAccessibility`) without explicit manual user approval in System Settings. | OpenDesk detects missing TCC permissions on launch and displays an interactive onboarding card with deep-links to *System Settings $\rightarrow$ Privacy & Security*. |
| **ScreenCaptureKit OS Requirement** | `ScreenCaptureKit` was introduced in macOS 12.3 Monterey. Older macOS releases (Big Sur, Catalina) lack modern GPU screen capture. | Minimum supported macOS version is standardized on **macOS 12.3+**. Legacy `CGDisplayStream` is provided as an un-optimized fallback only. |

---

## 3. Linux (Wayland vs. X11) Limitations

| Limitation | Technical Cause | Architectural Mitigation / Workaround |
|---|---|---|
| **Wayland Global Scraping Prohibition** | Modern Wayland compositors (GNOME Mutter, KDE KWin) strictly isolate client windows; no process can read another window's frame buffer directly. | OpenDesk utilizes the standardized XDG Desktop Portal (`org.freedesktop.portal.ScreenCast`) negotiating PipeWire streams with zero-copy `DmaBuf` GPU buffers. |
| **Wayland Synthetic Input Restrictions** | Wayland prohibits synthetic pointer injection via legacy `XTest` or X11 calls. | Utilizes the modern `libei` (Emulated Input) protocol via `org.freedesktop.portal.RemoteDesktop` D-Bus portals. |

---

## 4. Mobile Operating System Limitations (Android & iOS)

| Platform | Limitation & Boundary | Architectural Status |
|---|---|---|
| **Apple iOS** | **System-Wide Input Control Prohibited**: Apple App Store Review Guidelines (Guideline 2.5.2) and iOS sandboxing strictly forbid third-party apps or extensions from injecting synthetic touch gestures or keystrokes globally into the SpringBoard or into other applications. | **OpenDesk iOS operates strictly as a Remote Controller Client** (to control remote PCs) or as a view-only screen broadcaster via a ReplayKit Broadcast Extension. OpenDesk will never claim to provide unattended full remote control of un-jailbroken iOS devices. |
| **Android** | **Interactive Screen Capture Consent**: Android `MediaProjection` requires an interactive OS prompt on session startup unless running on a rooted enterprise device. Touch injection requires explicit user enablement in Android Accessibility Settings. | OpenDesk provides a dedicated `ForegroundService` with notification to maintain streaming; touch events are injected via `AccessibilityService.dispatchGesture()`. |
