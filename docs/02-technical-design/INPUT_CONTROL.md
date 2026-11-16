# OpenDesk — Synthetic Input Control & Injection Specification

## 1. Input Architecture & Security Boundaries

The shared Rust `opendesk-input` crate defines `InputBackend` for native
platform implementations and `RateLimitedInputInjector` as the public,
rate-limited adapter over `opendesk_proto::InputMessage`. Its default limiter
allows at most 1,000 events in any rolling one-second interval, including
immediately after startup. Events over the limit are dropped before reaching
the backend; the configured overflow callback is invoked at most once per
limiter interval while events are being dropped, and the caller receives
`InjectionOutcome::RateLimited` for every dropped event. Keep the callback
non-blocking. Session authorization and protocol validation must happen before
native injection.

Input control represents the highest security surface of a remote desktop host. The input injection subsystem guarantees that:
1. Input events are only processed if the session has active `FullControl` authorization.
2. Coordinates are bounded and validated to prevent memory overflow or out-of-bounds pointer crashes.
3. Event rates are throttled to mitigate denial-of-service or input-flooding attacks.
4. Native injection respects platform privilege separation and accessibility boundaries.

```mermaid
flowchart TD
    subgraph DataChannelReceiver["WebRTC DataChannel (opendesk-input)"]
        SCTP["Encrypted SCTP Stream"]
    end

    subgraph SecurityGate["Input Security Gate (Host)"]
        PolicyCheck{"Is FullControl\nAuthorized?"}
        Sanitizer["Coordinate Normalization\n(0.0 <= x,y <= 1.0)\n& Scancode Bounds Check"]
        RateLimiter["Rolling Window Rate Limiter\n(Max 1000 events/sec)"]
    end

    subgraph NativeInjectors["Platform Native Input Injectors"]
        WinInput["Windows: Win32 SendInput / SendSAS"]
        MacInput["macOS: CoreGraphics CGEventTap"]
        LinuxWayland["Linux Wayland: libei / Portal RemoteDesktop"]
        LinuxX11["Linux X11: XTest Extension / /dev/uinput"]
        AndroidAcc["Android: AccessibilityService.dispatchGesture"]
    end

    SCTP --> PolicyCheck
    PolicyCheck -->|Yes| Sanitizer
    PolicyCheck -->|No (View Only)| Drop[Drop & Log Warning]
    Sanitizer --> RateLimiter
    RateLimiter --> WinInput
    RateLimiter --> MacInput
    RateLimiter --> LinuxWayland
    RateLimiter --> LinuxX11
    RateLimiter --> AndroidAcc
```

---

## 2. Windows Input Injection

### A. Mouse & Keyboard Injection via `SendInput`
- **Mouse Injection**: Uses `INPUT` with `INPUT_MOUSE`. Coordinates are converted from normalized $0.0 - 1.0$ floats to the virtual desktop space:
  $$\text{dx} = \text{round}\left(X_{\text{norm}} \times 65535.0\right)$$
  $$\text{dy} = \text{round}\left(Y_{\text{norm}} \times 65535.0\right)$$
  Injected with flags: `MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK | MOUSEEVENTF_MOVE`.
- **Keyboard Injection**: Uses physical hardware scancodes rather than Virtual-Key (VK) codes to ensure consistent behavior across international keyboard layouts:
  ```c
  INPUT input = {0};
  input.type = INPUT_KEYBOARD;
  input.ki.wScan = hardwareScancode;
  input.ki.dwFlags = KEYEVENTF_SCANCODE | (isKeyUp ? KEYEVENTF_KEYUP : 0);
  if (isExtendedKey) input.ki.dwFlags |= KEYEVENTF_EXTENDEDKEY;
  SendInput(1, &input, sizeof(INPUT));
  ```

### B. UAC Prompts & Secure Desktop Traversal
Standard applications cannot interact with User Account Control (UAC) prompts or the Windows logon screen due to Windows User Interface Privilege Isolation (UIPI). OpenDesk overcomes this using two compliant Microsoft mechanisms:
1. **Interactive Session Agent Elevation**: The OpenDesk desktop agent executable is embedded with an application manifest specifying:
   ```xml
   <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
     <security>
       <requestedPrivileges>
         <requestedExecutionLevel level="asInvoker" uiAccess="true" />
       </requestedPrivileges>
     </security>
   </trustInfo>
   ```
   *Requirement*: The binary must be code-signed by a trusted certificate and installed into `C:\Program Files\OpenDesk\`.
2. **Secure Attention Sequence (Ctrl+Alt+Del)**:
   - Injected via the documented Windows `SendSAS` API from `sas.dll`.
   - Requires enabling the software SAS policy in the local security registry (`SoftwareSASGeneration = 3`).

---

## 3. macOS Input Injection (CoreGraphics)

- **API**: `CGEventCreateMouseEvent`, `CGEventCreateKeyboardEvent`, and `CGEventPost(kCGHIDEventTap, event)`.
- **Accessibility Security Guardrail**:
  - macOS requires explicit user authorization in *System Settings $\rightarrow$ Privacy & Security $\rightarrow$ Accessibility*.
  - Verification check:
    ```c
    NSDictionary *options = @{(__bridge id)kAXTrustedCheckOptionPrompt: @YES};
    BOOL isTrusted = AXIsProcessTrustedWithOptions((__bridge CFDictionaryRef)options);
    ```
  - If `isTrusted == FALSE`, input injection fails gracefully, and the UI displays an actionable guidance card.

---

## 4. Linux Input Injection (Wayland vs. X11)

### A. Wayland: `libei` and `org.freedesktop.portal.RemoteDesktop`
- Wayland strictly prevents client processes from injecting synthetic events into neighboring windows.
- OpenDesk communicates with the `RemoteDesktop` portal via D-Bus, requesting an Emulated Input (`libei`) file descriptor.
- OpenDesk writes input events to the `libei` socket, which the Wayland compositor maps safely to the active seat.

### B. Linux X11: `XTest` Extension & `/dev/uinput`
- On X11, events are injected via `XTestFakeMotionEvent` and `XTestFakeKeyEvent`.
- Alternatively, on headless servers running without an active X server, OpenDesk creates a virtual kernel input device via `/dev/uinput`.

---

## 5. Android Input Injection (Accessibility Service)

- When remote-controlling an Android host:
  ```kotlin
  val path = Path().apply {
      moveTo(xNormalized * screenWidth, yNormalized * screenHeight)
  }
  val stroke = GestureDescription.StrokeDescription(path, 0, 50) // 50ms tap duration
  val gesture = GestureDescription.Builder().addStroke(stroke).build()
  accessibilityService.dispatchGesture(gesture, null, null)
  ```
- Special system actions (Back, Home, Recents) are injected via `accessibilityService.performGlobalAction(GLOBAL_ACTION_BACK)`.
