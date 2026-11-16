//! Platform-independent video and audio encoder interfaces.
//!
//! Backend implementations can wrap hardware codecs or software codecs while
//! exposing the same bitrate, keyframe, and encoded-packet contract.

use std::sync::Arc;

use opendesk_capture::{RawAudioFrame, RawVideoFrame};
use thiserror::Error;

/// Codec identifier carried alongside encoded packets.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodecId(Arc<str>);

impl CodecId {
    /// Create an identifier such as `h264`, `av1`, or `opus`.
    pub fn new(value: impl AsRef<str>) -> Result<Self, EncoderError> {
        let value = value.as_ref();
        if value.trim().is_empty() || value.trim() != value {
            return Err(EncoderError::InvalidConfiguration(
                "codec identifier cannot be empty",
            ));
        }
        Ok(Self(Arc::from(value)))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Complete codec packet (access unit), before transport-specific fragmentation.
///
/// A transport packetizer may split `data` into multiple network packets. Audio
/// encoders should always set `keyframe` to `false`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncodedPacket {
    pub codec: CodecId,
    /// Presentation timestamp in microseconds, in the input clock domain.
    pub timestamp_us: u64,
    /// Marks independently decodable video packets (for example, an IDR).
    pub keyframe: bool,
    /// One codec packet/access unit, ready for transport packetization.
    pub data: Arc<[u8]>,
}

/// Requested video encoder settings. Bitrate is expressed in bits per second.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoEncoderConfig {
    pub codec: CodecId,
    pub width: u32,
    pub height: u32,
    pub bitrate_bps: u32,
    pub framerate: u32,
}

/// Requested audio encoder settings. Bitrate is expressed in bits per second.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioEncoderConfig {
    pub codec: CodecId,
    pub sample_rate_hz: u32,
    pub channels: u16,
    pub bitrate_bps: u32,
}

/// Failures shared by encoder backends.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EncoderError {
    #[error("invalid encoder configuration: {0}")]
    InvalidConfiguration(&'static str),
    #[error("unsupported encoder configuration: {0}")]
    UnsupportedConfiguration(String),
    #[error("invalid encoder input: {0}")]
    InvalidInput(String),
    #[error("encoder backend failed: {0}")]
    Backend(String),
    #[error("encoder has not been started")]
    NotStarted,
    #[error("encoder is already started")]
    AlreadyStarted,
}

/// Lifecycle and control contract implemented by video codec backends.
pub trait VideoEncoder: Send + Sync {
    /// Validate settings and initialize the codec. Dimensions, framerate, and
    /// bitrate must be non-zero; return `UnsupportedConfiguration` if unavailable.
    fn start(&mut self, config: VideoEncoderConfig) -> Result<(), EncoderError>;

    /// Encode a captured frame into zero or more complete codec packets.
    fn encode(&mut self, frame: &RawVideoFrame) -> Result<Vec<EncodedPacket>, EncoderError>;

    /// Change the target bitrate without recreating the encoder where supported.
    /// A zero bitrate is invalid; return `NotStarted` before initialization.
    fn set_bitrate(&mut self, bitrate_bps: u32) -> Result<(), EncoderError>;

    /// Request that the next suitable output packet be independently decodable.
    fn request_keyframe(&mut self) -> Result<(), EncoderError>;

    /// Release codec resources; safe to call after a failed start or stop.
    fn stop(&mut self) -> Result<(), EncoderError>;
}

/// Lifecycle and control contract implemented by audio codec backends.
pub trait AudioEncoder: Send + Sync {
    /// Validate settings and initialize the codec. Sample rate, channel count,
    /// and bitrate must be non-zero; return `UnsupportedConfiguration` if unavailable.
    fn start(&mut self, config: AudioEncoderConfig) -> Result<(), EncoderError>;

    /// Encode a captured PCM block into zero or more complete codec packets.
    fn encode(&mut self, frame: &RawAudioFrame) -> Result<Vec<EncodedPacket>, EncoderError>;

    /// Change the target bitrate without recreating the encoder where supported.
    /// A zero bitrate is invalid; return `NotStarted` before initialization.
    fn set_bitrate(&mut self, bitrate_bps: u32) -> Result<(), EncoderError>;

    /// Release codec resources; safe to call after a failed start or stop.
    fn stop(&mut self) -> Result<(), EncoderError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codec_identifier_rejects_empty_and_padded_values() {
        assert!(matches!(
            CodecId::new(""),
            Err(EncoderError::InvalidConfiguration(_))
        ));
        assert!(matches!(
            CodecId::new(" h264"),
            Err(EncoderError::InvalidConfiguration(_))
        ));
        assert_eq!(CodecId::new("h264").unwrap().as_str(), "h264");
    }

    #[test]
    fn property_codec_identifier_preserves_non_whitespace_text() {
        for length in 1..256 {
            let identifier = "x".repeat(length);
            let codec = CodecId::new(&identifier).unwrap();
            assert_eq!(codec.as_str(), identifier);
        }
    }
}
