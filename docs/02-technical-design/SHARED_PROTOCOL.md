# OpenDesk — Shared Protocol Specification

## 1. Protocol Architecture & Channel Multiplexing

All OpenDesk clients and hosts communicate over WebRTC SCTP Data Channels utilizing a strongly-typed binary protocol.

```
+-----------------------------------------------------------------------------------+
| SCTP STREAM MULTIPLEXING ON WEBRTC                                                |
|                                                                                   |
|  Stream 0: "opendesk-control"   [Reliable, Ordered]                               |
|            - Capability negotiation, display topology, bitrate hints, heartbeats  |
|                                                                                   |
|  Stream 1: "opendesk-input"     [Unreliable or Low-Latency Ordered, maxRetrans: 0]|
|            - Sub-5ms mouse movements, clicks, keyboard events, gestures           |
|                                                                                   |
|  Stream 2: "opendesk-clipboard" [Reliable, Ordered]                               |
|            - Clipboard text updates, image synchronization, format announcements |
|                                                                                   |
|  Stream 3: "opendesk-file"      [Reliable, Ordered]                               |
|            - Directory browsing, chunked file transfer, backpressure signals      |
+-----------------------------------------------------------------------------------+
```

---

## 2. Binary Framing & Endianness

- **Byte Order**: Big-Endian (Network Byte Order) for all integer and floating-point fields.
- **Serialization Standard**:
  - Internal Rust Engine and Native Channels: `bincode` binary format with variable-length integer encoding.
  - Cross-Language SignalR & REST APIs: JSON (OpenAPI schemas) and Protobuf contracts.
- **Header Structure**: Every data channel packet begins with a standard 4-byte header:

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|  Magic (0x4F) |  Version (0x01)|         Message Type          |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                        Payload Length                         |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                        Payload Bytes ...                      |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

---

## 3. Control Channel Messages (`opendesk-control`)

| Type ID | Message Name | Description & Payload |
|---|---|---|
| `0x0001` | `ClientHello` | Protocol version, supported video codecs, client capabilities flags. |
| `0x0002` | `HostHello` | Selected video codec, active display layout, audio capabilities. |
| `0x0003` | `DisplayTopology` | Array of `DisplayDescriptor` structs (Index, X, Y, Width, Height, DPI). |
| `0x0004` | `SelectDisplay` | Request to switch active stream to specified display index `u32`. |
| `0x0005` | `QualityHint` | Client request to adjust maximum bitrate (kbps) or framerate. |
| `0x0006` | `Ping` | Timestamp `u64` (microseconds) for RTT measurement. |
| `0x0007` | `Pong` | Echo of Ping timestamp for round-trip latency computation. |
| `0x0008` | `Terminate` | Graceful disconnection notice with reason code enum. |

---

## 4. Input Channel Messages (`opendesk-input`)

Optimized for absolute minimum serialization overhead:

```rust
#[repr(u16)]
pub enum InputMessageType {
    MouseMove = 0x0101,
    MouseDown = 0x0102,
    MouseUp   = 0x0103,
    MouseWheel = 0x0104,
    KeyDown   = 0x0105,
    KeyUp     = 0x0106,
    TouchGesture = 0x0107,
    SystemAction = 0x0108,
}

#[derive(Serialize, Deserialize)]
pub struct MouseMovePayload {
    pub display_id: u32,
    pub x_norm: f64, // 0.0 to 1.0 (64-bit IEEE float)
    pub y_norm: f64, // 0.0 to 1.0
}

#[derive(Serialize, Deserialize)]
pub struct MouseButtonPayload {
    pub button: u8, // 0: Left, 1: Middle, 2: Right, 3: Back, 4: Forward
    pub click_count: u8,
}

#[derive(Serialize, Deserialize)]
pub struct KeyboardPayload {
    pub scancode: u32,     // Hardware physical scancode
    pub modifiers: u16,    // Bitmask: Shift(1), Ctrl(2), Alt(4), Meta(8)
    pub unicode_char: u32, // Optional UTF-32 character
}

#[derive(Serialize, Deserialize)]
pub enum SystemActionPayload {
    SendCtrlAltDel,
    LockWorkstation,
    ShowDesktop,
    OpenTaskSwitcher,
}
```

---

## 5. Clipboard Channel Messages (`opendesk-clipboard`)

```rust
#[derive(Serialize, Deserialize)]
pub enum ClipboardPayload {
    Text { content: String },
    ImagePng { data: Vec<u8> },
    FormatAnnouncement { available_formats: Vec<String> },
}
```
- Transmitted reliably and ordered.
- Debounced by 200 ms on both host and client to avoid clipboard spam loops.
- Payload size capped at 10 MB.
