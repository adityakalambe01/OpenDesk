//! Platform-independent capture interfaces and frame types.
//!
//! Native capture backends keep their GPU or OS surface alive by storing an
//! owned, thread-safe resource in [`NativeSurface`]. Consumers can downcast
//! that resource to the backend-specific type without copying pixel data.

use std::{any::Any, fmt, sync::Arc};

/// The kind of native resource backing a captured frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceKind {
    D3D11,
    IOSurface,
    DmaBuf,
}

/// An owned reference to a native frame surface.
///
/// The resource is shared by reference-counting so an encoder may retain it
/// while capture advances. No pixel bytes are copied by this wrapper.
#[derive(Clone)]
pub struct NativeSurface {
    kind: SurfaceKind,
    resource: Arc<dyn Any + Send + Sync>,
}

impl NativeSurface {
    /// Wrap a backend resource while retaining its native ownership model.
    ///
    /// `kind` must describe the concrete resource. The resource must own (or
    /// retain) the underlying OS/GPU object for as long as this wrapper exists.
    /// For pointer-based APIs such as IOSurface, pass an owning, thread-safe
    /// backend wrapper rather than a bare pointer.
    pub fn new<T: Any + Send + Sync>(kind: SurfaceKind, resource: Arc<T>) -> Self {
        Self { kind, resource }
    }

    pub fn kind(&self) -> SurfaceKind {
        self.kind
    }

    /// Borrow the concrete backend resource without transferring ownership.
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.resource.downcast_ref()
    }
}

impl fmt::Debug for NativeSurface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeSurface")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}

/// Pixel layout for CPU-backed frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Bgra8,
    Rgba8,
    Nv12,
    I420,
    Unknown,
}

/// Captured image storage. Native variants preserve zero-copy GPU/OS handles.
#[derive(Debug, Clone)]
pub enum FrameBuffer {
    CpuMemory(Arc<[u8]>),
    Native(NativeSurface),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayInfo {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    /// Display scale in dots per inch.
    pub dpi: u32,
    pub is_primary: bool,
}

#[derive(Debug, Clone)]
pub struct RawVideoFrame {
    pub display_id: u32,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    /// Capture timestamp in the backend's monotonic microsecond clock domain.
    pub timestamp_us: u64,
    pub format: PixelFormat,
    /// CPU buffers use `stride` bytes per row. Native surfaces carry layout in
    /// their native resource and may not have a meaningful CPU stride.
    pub data: FrameBuffer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptureError {
    Backend(String),
    PermissionDenied,
    DisplayNotFound(u32),
    NotStarted,
    AlreadyStarted,
    Timeout,
}

impl fmt::Display for CaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backend(message) => write!(f, "capture backend failed: {message}"),
            Self::PermissionDenied => f.write_str("screen capture permission denied"),
            Self::DisplayNotFound(id) => write!(f, "display {id} was not found"),
            Self::NotStarted => f.write_str("capture has not been started"),
            Self::AlreadyStarted => f.write_str("capture is already started"),
            Self::Timeout => f.write_str("capture timed out"),
        }
    }
}

impl std::error::Error for CaptureError {}

/// Lifecycle contract implemented by each platform screen capture backend.
pub trait ScreenCapturer: Send + Sync {
    /// Return the current topology. Display IDs are backend-local and may
    /// change after topology updates; callers should re-enumerate on failure.
    fn enumerate_displays(&self) -> Result<Vec<DisplayInfo>, CaptureError>;
    fn select_display(&mut self, display_id: u32) -> Result<(), CaptureError>;
    /// Begin capture. Calling this twice should return
    /// [`CaptureError::AlreadyStarted`].
    fn start_capture(&mut self) -> Result<(), CaptureError>;
    /// Return the next frame, or [`CaptureError::Timeout`] when the deadline
    /// expires.
    fn capture_next_frame(&mut self, timeout_ms: u32) -> Result<RawVideoFrame, CaptureError>;
    /// Stop capture and release backend streaming resources; safe to call after
    /// a failed start or after capture has stopped.
    fn stop_capture(&mut self) -> Result<(), CaptureError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioSampleFormat {
    F32,
    I16,
}

#[derive(Debug, Clone)]
pub struct RawAudioFrame {
    pub sample_rate_hz: u32,
    pub channels: u16,
    pub format: AudioSampleFormat,
    pub timestamp_us: u64,
    /// Interleaved PCM samples in the declared format (little-endian).
    pub data: Arc<[u8]>,
}

/// Lifecycle contract for system or microphone audio capture backends.
pub trait AudioCapturer: Send + Sync {
    fn start_capture(&mut self) -> Result<(), CaptureError>;
    /// Return the next audio block, or [`CaptureError::Timeout`] on deadline.
    fn capture_next_frame(&mut self, timeout_ms: u32) -> Result<RawAudioFrame, CaptureError>;
    fn stop_capture(&mut self) -> Result<(), CaptureError>;
}
