//! Session lifecycle coordinator for the platform independent engine.
//!
//! Capture and codec interfaces are synchronous, so the coordinator invokes
//! them on its async task with bounded capture waits. Applications should run
//! this task on a dedicated runtime thread when their backend can block longer
//! than its requested timeout.

use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

use opendesk_capture::{CaptureError, ScreenCapturer};
use opendesk_encoder::{EncodedPacket, EncoderError, VideoEncoder, VideoEncoderConfig};
use opendesk_input::{InputError, InputInjector};
use opendesk_proto::InputMessage;
use thiserror::Error;
use tokio::sync::mpsc;

/// Future returned by an engine transport when sending a video packet.
pub type SendPacketFuture<'a> =
    Pin<Box<dyn Future<Output = Result<(), TransportError>> + Send + 'a>>;

/// Network boundary used by the coordinator. The concrete WebRTC transport
/// can implement this without making the engine depend on its signaling model.
pub trait EngineTransport: Send + Sync {
    fn send_video_packet<'a>(&'a self, packet: EncodedPacket) -> SendPacketFuture<'a>;
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("transport failed: {0}")]
pub struct TransportError(pub String);

/// Signals delivered to an active session by its owner or remote peer.
#[derive(Debug, Clone, PartialEq)]
pub enum SessionControlEvent {
    Terminate,
    /// Target bitrate in kilobits per second.
    BitrateAdjust(u32),
    SwitchDisplay(u32),
    Input(InputMessage),
}

#[derive(Debug, Error)]
pub enum EngineError {
    #[error(transparent)]
    Capture(#[from] CaptureError),
    #[error(transparent)]
    Encoder(#[from] EncoderError),
    #[error(transparent)]
    Input(#[from] InputError),
    #[error(transparent)]
    Transport(#[from] TransportError),
    #[error("transport send timed out after {0:?}")]
    TransportTimeout(Duration),
}

/// Owns the active session's capture, encoding, input, and dispatch loop.
pub struct EngineCoordinator {
    capturer: Box<dyn ScreenCapturer>,
    encoder: Box<dyn VideoEncoder>,
    encoder_config: VideoEncoderConfig,
    input_injector: Box<dyn InputInjector>,
    transport: Arc<dyn EngineTransport>,
    event_rx: mpsc::Receiver<SessionControlEvent>,
    frame_interval: Duration,
    capture_timeout_ms: u32,
    transport_timeout: Duration,
}

impl EngineCoordinator {
    pub fn new(
        capturer: Box<dyn ScreenCapturer>,
        encoder: Box<dyn VideoEncoder>,
        encoder_config: VideoEncoderConfig,
        input_injector: Box<dyn InputInjector>,
        transport: Arc<dyn EngineTransport>,
        event_rx: mpsc::Receiver<SessionControlEvent>,
    ) -> Self {
        Self {
            capturer,
            encoder,
            encoder_config,
            input_injector,
            transport,
            event_rx,
            frame_interval: Duration::from_millis(16),
            capture_timeout_ms: 10,
            transport_timeout: Duration::from_secs(10),
        }
    }

    /// Configure the target frame cadence and the maximum blocking capture
    /// wait. Zero values are clamped to one to keep the loop responsive.
    pub fn with_timing(mut self, frame_interval: Duration, capture_timeout_ms: u32) -> Self {
        self.frame_interval = frame_interval.max(Duration::from_millis(1));
        self.capture_timeout_ms = capture_timeout_ms.max(1);
        self
    }

    /// Bound how long a transport send can hold the session loop. A timed-out
    /// transport future is cancelled, so implementations should make sending
    /// cancellation-safe (for example, by enqueueing packets atomically).
    pub fn with_transport_timeout(mut self, timeout: Duration) -> Self {
        self.transport_timeout = timeout.max(Duration::from_millis(1));
        self
    }

    /// Start the encoder and capture backend, then run until a termination
    /// event arrives or the control channel closes. Capture timeouts are normal
    /// polling outcomes; other backend errors end the session. Both resources
    /// are stopped on every exit, including partial startup failures.
    pub async fn run_session_loop(mut self) -> Result<(), EngineError> {
        let mut result = self
            .encoder
            .start(self.encoder_config.clone())
            .map_err(EngineError::from);
        if result.is_ok() {
            result = self.capturer.start_capture().map_err(EngineError::from);
        }
        if result.is_ok() {
            let mut interval = tokio::time::interval(self.frame_interval);
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            'session: loop {
                tokio::select! {
                    _ = interval.tick() => {
                        match self.capturer.capture_next_frame(self.capture_timeout_ms) {
                            Ok(frame) => {
                                let packets = match self.encoder.encode(&frame) {
                                    Ok(packets) => packets,
                                    Err(error) => { result = Err(error.into()); break 'session; }
                                };
                                for packet in packets {
                                    match tokio::time::timeout(
                                        self.transport_timeout,
                                        self.transport.send_video_packet(packet),
                                    )
                                    .await
                                    {
                                        Ok(Ok(())) => {}
                                        Ok(Err(error)) => {
                                            result = Err(error.into());
                                            break 'session;
                                        }
                                        Err(_) => {
                                            result = Err(EngineError::TransportTimeout(
                                                self.transport_timeout,
                                            ));
                                            break 'session;
                                        }
                                    }
                                }
                            }
                            Err(CaptureError::Timeout) => {}
                            Err(error) => { result = Err(error.into()); break 'session; }
                        }
                    }
                    event = self.event_rx.recv() => {
                        match event {
                            None | Some(SessionControlEvent::Terminate) => break 'session,
                            Some(SessionControlEvent::BitrateAdjust(kbps)) => {
                                if kbps == 0 {
                                    result = Err(EncoderError::InvalidConfiguration("bitrate must be greater than zero").into());
                                    break 'session;
                                }
                                let Some(bps) = kbps.checked_mul(1_000) else {
                                    result = Err(EncoderError::InvalidConfiguration("bitrate overflows bits per second").into());
                                    break 'session;
                                };
                                if let Err(error) = self.encoder.set_bitrate(bps) {
                                    result = Err(error.into());
                                    break 'session;
                                }
                            }
                            Some(SessionControlEvent::SwitchDisplay(id)) => {
                                if let Err(error) = self.capturer.select_display(id) {
                                    result = Err(error.into());
                                    break 'session;
                                }
                                if let Err(error) = self.encoder.request_keyframe() {
                                    result = Err(error.into());
                                    break 'session;
                                }
                            }
                            Some(SessionControlEvent::Input(input)) => {
                                if let Err(error) = self.input_injector.inject(&input) {
                                    result = Err(error.into());
                                    break 'session;
                                }
                            }
                        }
                    }
                }
            }
        }

        let capture_stop = self.capturer.stop_capture().map_err(EngineError::from);
        let encoder_stop = self.encoder.stop().map_err(EngineError::from);
        result.and(capture_stop).and(encoder_stop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use opendesk_capture::{DisplayInfo, FrameBuffer, PixelFormat, RawVideoFrame};
    use opendesk_encoder::{CodecId, EncodedPacket};
    use opendesk_input::{InputBackend, RateLimitedInputInjector};
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Default)]
    struct CaptureState {
        starts: AtomicUsize,
        stops: AtomicUsize,
        frames: AtomicUsize,
        selected: AtomicUsize,
    }

    struct MockCapturer(Arc<CaptureState>);

    impl ScreenCapturer for MockCapturer {
        fn enumerate_displays(&self) -> Result<Vec<DisplayInfo>, CaptureError> {
            Ok(Vec::new())
        }

        fn select_display(&mut self, _display_id: u32) -> Result<(), CaptureError> {
            self.0.selected.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }

        fn start_capture(&mut self) -> Result<(), CaptureError> {
            self.0.starts.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }

        fn capture_next_frame(&mut self, _timeout_ms: u32) -> Result<RawVideoFrame, CaptureError> {
            self.0.frames.fetch_add(1, Ordering::Relaxed);
            Ok(RawVideoFrame {
                display_id: 1,
                width: 1,
                height: 1,
                stride: 4,
                timestamp_us: 42,
                format: PixelFormat::Bgra8,
                data: FrameBuffer::CpuMemory(Arc::from([0_u8; 4])),
            })
        }

        fn stop_capture(&mut self) -> Result<(), CaptureError> {
            self.0.stops.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    #[derive(Default)]
    struct EncoderState {
        starts: AtomicUsize,
        stops: AtomicUsize,
        encoded: AtomicUsize,
        bitrate: AtomicUsize,
        keyframes: AtomicUsize,
    }

    struct MockEncoder(Arc<EncoderState>);

    impl VideoEncoder for MockEncoder {
        fn start(&mut self, _config: VideoEncoderConfig) -> Result<(), EncoderError> {
            self.0.starts.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }

        fn encode(&mut self, _frame: &RawVideoFrame) -> Result<Vec<EncodedPacket>, EncoderError> {
            self.0.encoded.fetch_add(1, Ordering::Relaxed);
            Ok(vec![EncodedPacket {
                codec: CodecId::new("h264")?,
                timestamp_us: 42,
                keyframe: false,
                data: Arc::from([1_u8]),
            }])
        }

        fn set_bitrate(&mut self, _bitrate_bps: u32) -> Result<(), EncoderError> {
            self.0.bitrate.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }

        fn request_keyframe(&mut self) -> Result<(), EncoderError> {
            self.0.keyframes.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }

        fn stop(&mut self) -> Result<(), EncoderError> {
            self.0.stops.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    #[derive(Default)]
    struct MockInput(Arc<AtomicUsize>);

    impl InputBackend for MockInput {
        fn inject(&self, _event: &InputMessage) -> Result<(), InputError> {
            self.0.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    }

    struct MockTransport(Arc<AtomicUsize>);

    impl EngineTransport for MockTransport {
        fn send_video_packet<'a>(&'a self, _packet: EncodedPacket) -> SendPacketFuture<'a> {
            let sent = Arc::clone(&self.0);
            Box::pin(async move {
                sent.fetch_add(1, Ordering::Relaxed);
                Ok(())
            })
        }
    }

    #[tokio::test]
    async fn session_dispatches_frames_controls_and_stops_resources() {
        let capture = Arc::new(CaptureState::default());
        let encoder = Arc::new(EncoderState::default());
        let input_count = Arc::new(AtomicUsize::new(0));
        let sent_count = Arc::new(AtomicUsize::new(0));
        let (event_tx, event_rx) = mpsc::channel(8);
        let bitrate = event_tx
            .send(SessionControlEvent::BitrateAdjust(2_000))
            .await;
        assert!(bitrate.is_ok());
        assert!(event_tx
            .send(SessionControlEvent::SwitchDisplay(7))
            .await
            .is_ok());
        assert!(event_tx
            .send(SessionControlEvent::Input(InputMessage::SystemAction(
                opendesk_proto::SystemAction::ShowDesktop,
            )))
            .await
            .is_ok());

        let terminator = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(20)).await;
            let _ = event_tx.send(SessionControlEvent::Terminate).await;
        });
        let injector = RateLimitedInputInjector::new(MockInput(Arc::clone(&input_count)), || {});
        let config = VideoEncoderConfig {
            codec: CodecId::new("h264").unwrap(),
            width: 1,
            height: 1,
            bitrate_bps: 2_000_000,
            framerate: 60,
        };

        EngineCoordinator::new(
            Box::new(MockCapturer(Arc::clone(&capture))),
            Box::new(MockEncoder(Arc::clone(&encoder))),
            config,
            Box::new(injector),
            Arc::new(MockTransport(Arc::clone(&sent_count))),
            event_rx,
        )
        .with_timing(Duration::from_millis(1), 1)
        .run_session_loop()
        .await
        .unwrap();
        terminator.await.unwrap();

        assert_eq!(encoder.starts.load(Ordering::Relaxed), 1);
        assert_eq!(capture.starts.load(Ordering::Relaxed), 1);
        assert!(capture.frames.load(Ordering::Relaxed) > 0);
        assert!(encoder.encoded.load(Ordering::Relaxed) > 0);
        assert!(sent_count.load(Ordering::Relaxed) > 0);
        assert_eq!(encoder.bitrate.load(Ordering::Relaxed), 1);
        assert_eq!(capture.selected.load(Ordering::Relaxed), 1);
        assert_eq!(encoder.keyframes.load(Ordering::Relaxed), 1);
        assert_eq!(input_count.load(Ordering::Relaxed), 1);
        assert_eq!(capture.stops.load(Ordering::Relaxed), 1);
        assert_eq!(encoder.stops.load(Ordering::Relaxed), 1);
    }
}
