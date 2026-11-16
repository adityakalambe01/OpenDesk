# OpenDesk — Rust Core Engine Design (`libopendesk`)

## 1. Architectural Role & Design Principles

The Rust Core Engine (`libopendesk`) is the computational nucleus of OpenDesk. It is compiled as a modular Cargo workspace and exposes both native Rust traits and a C-ABI / FFI layer for integration with Tauri (desktop) and Flutter (mobile).

### Core Design Rules:
1. **Memory Safety First**: All protocol serialization, cryptographic negotiation, and state-machine transitions strictly forbid unsafe code (`#![forbid(unsafe_code)]`).
2. **Contained Platform FFI**: Direct native OS invocations (Direct3D, Win32, PipeWire, CoreGraphics) are isolated in dedicated `sys` modules with thoroughly documented safety invariants.
3. **Zero-Copy Pipeline**: Video buffers captured from the GPU are passed directly as texture handles or hardware surface descriptors to the hardware encoder without roundtripping through user-space RAM.
4. **Asynchronous Concurrency**: Built upon `tokio` multi-threaded runtime, isolating CPU-bound video processing threads from I/O-bound networking and signaling tasks.

---

## 2. Cargo Workspace Architecture

```toml
# Top-level Cargo.toml
[workspace]
resolver = "2"
members = [
    "core/crates/opendesk-proto",
    "core/crates/opendesk-crypto",
    "core/crates/opendesk-capture",
    "core/crates/opendesk-encoder",
    "core/crates/opendesk-input",
    "core/crates/opendesk-transport",
    "core/crates/opendesk-ipc",
    "core/crates/opendesk-engine",
    "core/crates/opendesk-daemon",
    "core/crates/opendesk-agent",
    "core/crates/opendesk-cli",
]

[workspace.dependencies]
tokio = { version = "1.41", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"
bytes = "1.8"
thiserror = "2.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
webrtc = "0.12"
ed25519-dalek = { version = "2.1", features = ["rand_core"] }
x25519-dalek = { version = "2.0", features = ["static_secrets"] }
chacha20poly1305 = "0.10"
argon2 = "0.5"
```

---

## 3. Core Trait Specifications

### A. Screen Capture Trait (`opendesk-capture`)
```rust
use opendesk_proto::display::DisplayInfo;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CaptureError {
    #[error("Display index {0} not found")]
    DisplayNotFound(u32),
    #[error("Platform capture initialization failed: {0}")]
    InitializationFailed(String),
    #[error("Capture device lost (GPU reset or display reconfiguration)")]
    DeviceLost,
    #[error("Frame capture timed out")]
    Timeout,
    #[error("Permission denied by OS security subsystem")]
    PermissionDenied,
}

pub struct RawVideoFrame {
    pub display_id: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub timestamp_us: u64,
    pub format: PixelFormat,
    pub data: FrameBuffer,
}

pub enum FrameBuffer {
    CpuMemory(Arc<[u8]>),
    Native(NativeSurface),
}

// NativeSurface retains an owned, thread-safe platform wrapper. Its
// SurfaceKind identifies D3D11, IOSurface, or DmaBuf; raw pointers and raw
// file descriptors must not outlive their owning backend resource.

pub trait ScreenCapturer: Send + Sync {
    fn enumerate_displays(&self) -> Result<Vec<DisplayInfo>, CaptureError>;
    fn select_display(&mut self, display_id: u32) -> Result<(), CaptureError>;
    fn start_capture(&mut self) -> Result<(), CaptureError>;
    fn capture_next_frame(&mut self, timeout_ms: u32) -> Result<RawVideoFrame, CaptureError>;
    fn stop_capture(&mut self) -> Result<(), CaptureError>;
}
```

### B. Hardware Video Encoder Trait (`opendesk-encoder`)
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoCodecType {
    H264,
    AV1,
    VP9,
}

#[derive(Debug, Clone)]
pub struct EncoderConfig {
    pub codec: VideoCodecType,
    pub width: u32,
    pub height: u32,
    pub target_bitrate_kbps: u32,
    pub max_bitrate_kbps: u32,
    pub target_fps: u32,
    pub gop_size: u32, // IDR keyframe interval
}

pub struct EncodedPacket {
    pub is_keyframe: bool,
    pub timestamp_us: u64,
    pub data: bytes::Bytes,
}

pub trait VideoEncoder: Send + Sync {
    fn configure(&mut self, config: &EncoderConfig) -> Result<(), EncoderError>;
    fn encode_frame(&mut self, frame: RawVideoFrame) -> Result<Vec<EncodedPacket>, EncoderError>;
    fn request_keyframe(&mut self) -> Result<(), EncoderError>;
    fn update_bitrate(&mut self, target_kbps: u32) -> Result<(), EncoderError>;
}
```

### C. Synthetic Input Trait (`opendesk-input`)
```rust
use opendesk_proto::input::{InputEvent, MouseButton, KeyState};

#[derive(Error, Debug)]
pub enum InputError {
    #[error("Failed to inject input: {0}")]
    InjectionFailed(String),
    #[error("Operating system denied input privileges (TCC or Accessibility missing)")]
    Unauthorized,
}

pub trait InputInjector: Send + Sync {
    fn move_mouse_normalized(&mut self, x: f64, y: f64, display_id: u32) -> Result<(), InputError>;
    fn mouse_button(&mut self, button: MouseButton, state: KeyState) -> Result<(), InputError>;
    fn mouse_wheel(&mut self, delta_x: f64, delta_y: f64) -> Result<(), InputError>;
    fn keyboard_key(&mut self, scancode: u32, state: KeyState) -> Result<(), InputError>;
    fn send_system_command(&mut self, cmd: SystemCommand) -> Result<(), InputError>; // e.g. Ctrl+Alt+Del
}
```

---

## 4. Engine Lifecycle Coordinator (`opendesk-engine`)

The `EngineCoordinator` manages the lifecycle of an active session:

```rust
pub struct EngineCoordinator {
    capturer: Box<dyn ScreenCapturer>,
    encoder: Box<dyn VideoEncoder>,
    audio_capturer: Option<Box<dyn AudioCapturer>>,
    input_injector: Box<dyn InputInjector>,
    transport: Arc<WebRtcTransport>,
    event_rx: tokio::sync::mpsc::Receiver<SessionControlEvent>,
}

impl EngineCoordinator {
    pub async fn run_session_loop(mut self) -> Result<(), EngineError> {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(16)); // Target ~60 FPS
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if let Ok(frame) = self.capturer.capture_next_frame(10) {
                        if let Ok(packets) = self.encoder.encode_frame(frame) {
                            for packet in packets {
                                self.transport.send_video_packet(packet).await?;
                            }
                        }
                    }
                }
                Some(ctrl_event) = self.event_rx.recv() => {
                    match ctrl_event {
                        SessionControlEvent::Terminate => break,
                        SessionControlEvent::BitrateAdjust(kbps) => {
                            self.encoder.update_bitrate(kbps)?;
                        }
                        SessionControlEvent::SwitchDisplay(id) => {
                            self.capturer.select_display(id)?;
                            self.encoder.request_keyframe()?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
```
