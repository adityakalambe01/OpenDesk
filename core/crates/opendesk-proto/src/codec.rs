use bincode::Options;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

pub const MAGIC: u8 = 0x4f;
pub const VERSION: u8 = 1;
/// Header size: magic (1), version (1), message type (2), payload length (4).
pub const HEADER_LEN: usize = 8;
/// Maximum serialized payload accepted in one frame (16 MiB).
pub const MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;

/// Stable frame identifiers for protocol message variants.
pub trait ProtocolMessage: Serialize + DeserializeOwned {
    /// Stable ID for this specific message variant, written in the frame header.
    fn message_type(&self) -> u16;

    /// Validate semantic constraints before sending or after decoding untrusted data.
    fn validate(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum CodecError {
    #[error("protocol frame is shorter than its header")]
    TruncatedHeader,
    #[error("invalid protocol magic byte: {0:#04x}")]
    InvalidMagic(u8),
    #[error("unsupported protocol version: {0}")]
    UnsupportedVersion(u8),
    #[error("unexpected message type {actual:#06x}; expected {expected:#06x}")]
    UnexpectedMessageType { expected: u16, actual: u16 },
    #[error("payload length is {actual} bytes; header declares {declared}")]
    InvalidPayloadLength { declared: usize, actual: usize },
    #[error("protocol payload exceeds the {MAX_PAYLOAD_LEN}-byte limit: {0} bytes")]
    PayloadTooLarge(usize),
    #[error("invalid protocol message: {0}")]
    InvalidMessage(&'static str),
    #[error("protocol serialization failed: {0}")]
    Serialization(#[source] Box<bincode::ErrorKind>),
}

fn options() -> impl Options {
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_varint_encoding()
        .with_limit(MAX_PAYLOAD_LEN as u64)
}

pub fn encode<T: ProtocolMessage>(message: &T) -> Result<Vec<u8>, CodecError> {
    message.validate().map_err(CodecError::InvalidMessage)?;
    let payload = options()
        .serialize(message)
        .map_err(|error| match error.as_ref() {
            bincode::ErrorKind::SizeLimit => CodecError::PayloadTooLarge(MAX_PAYLOAD_LEN + 1),
            _ => CodecError::Serialization(error),
        })?;
    if payload.len() > MAX_PAYLOAD_LEN {
        return Err(CodecError::PayloadTooLarge(payload.len()));
    }
    let length =
        u32::try_from(payload.len()).map_err(|_| CodecError::PayloadTooLarge(payload.len()))?;
    let mut frame = Vec::with_capacity(HEADER_LEN + payload.len());
    frame.extend_from_slice(&[MAGIC, VERSION]);
    frame.extend_from_slice(&message.message_type().to_be_bytes());
    frame.extend_from_slice(&length.to_be_bytes());
    frame.extend_from_slice(&payload);
    Ok(frame)
}

pub fn decode<T: ProtocolMessage>(frame: &[u8]) -> Result<T, CodecError> {
    if frame.len() < HEADER_LEN {
        return Err(CodecError::TruncatedHeader);
    }
    if frame[0] != MAGIC {
        return Err(CodecError::InvalidMagic(frame[0]));
    }
    if frame[1] != VERSION {
        return Err(CodecError::UnsupportedVersion(frame[1]));
    }
    let actual_type = u16::from_be_bytes([frame[2], frame[3]]);
    let declared = u32::from_be_bytes([frame[4], frame[5], frame[6], frame[7]]) as usize;
    let payload = &frame[HEADER_LEN..];
    if declared > MAX_PAYLOAD_LEN {
        return Err(CodecError::PayloadTooLarge(declared));
    }
    if payload.len() != declared {
        return Err(CodecError::InvalidPayloadLength {
            declared,
            actual: payload.len(),
        });
    }
    let message: T = options()
        .reject_trailing_bytes()
        .deserialize(payload)
        .map_err(CodecError::Serialization)?;
    let expected_type = message.message_type();
    if actual_type != expected_type {
        return Err(CodecError::UnexpectedMessageType {
            expected: expected_type,
            actual: actual_type,
        });
    }
    message.validate().map_err(CodecError::InvalidMessage)?;
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ControlMessage;
    #[test]
    fn frame_header_is_network_byte_order() {
        let frame = encode(&ControlMessage::Ping {
            timestamp_micros: 7,
        })
        .unwrap();
        assert_eq!(&frame[..4], &[MAGIC, VERSION, 0, 6]);
        assert_eq!(
            u32::from_be_bytes(frame[4..8].try_into().unwrap()) as usize,
            frame.len() - HEADER_LEN
        );
    }
    #[test]
    fn rejects_malformed_or_wrong_channel_frames() {
        let frame = encode(&ControlMessage::Ping {
            timestamp_micros: 7,
        })
        .unwrap();
        assert!(matches!(
            decode::<ControlMessage>(&frame[0..3]),
            Err(CodecError::TruncatedHeader)
        ));
        let mut invalid = frame.clone();
        invalid[0] = 0;
        assert!(matches!(
            decode::<ControlMessage>(&invalid),
            Err(CodecError::InvalidMagic(0))
        ));
        assert!(matches!(
            decode::<crate::InputMessage>(&frame),
            Err(CodecError::Serialization(_)) | Err(CodecError::UnexpectedMessageType { .. })
        ));
        let mut invalid = frame.clone();
        invalid[7] += 1;
        assert!(matches!(
            decode::<ControlMessage>(&invalid),
            Err(CodecError::InvalidPayloadLength { .. })
        ));
        let mut invalid = frame;
        invalid[1] += 1;
        assert!(matches!(
            decode::<ControlMessage>(&invalid),
            Err(CodecError::UnsupportedVersion(_))
        ));
    }

    #[test]
    fn rejects_declared_payload_over_limit_before_deserializing() {
        let mut frame = vec![MAGIC, VERSION, 0, 6];
        frame.extend_from_slice(&((MAX_PAYLOAD_LEN as u32) + 1).to_be_bytes());
        assert!(matches!(
            decode::<ControlMessage>(&frame),
            Err(CodecError::PayloadTooLarge(_))
        ));
    }

    #[test]
    fn encoding_oversized_message_returns_payload_limit_error() {
        let message = crate::InputMessage::TextComposition {
            text: "x".repeat(MAX_PAYLOAD_LEN),
        };
        assert!(matches!(
            encode(&message),
            Err(CodecError::PayloadTooLarge(_))
        ));
    }

    #[test]
    fn rejects_truncated_huge_collection_without_preallocating() {
        // ClientHello variant, protocol version 1, then an untrusted u64::MAX
        // codec count with no corresponding elements.
        let payload = [0, 1, 253, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let mut frame = vec![MAGIC, VERSION, 0, 1];
        frame.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        frame.extend_from_slice(&payload);
        assert!(matches!(
            decode::<ControlMessage>(&frame),
            Err(CodecError::Serialization(_))
        ));
    }

    #[test]
    fn rejects_collections_over_item_limit() {
        // ClientHello, version 1, 4097 empty codec strings.
        let mut payload = vec![0, 1, 251, 0x10, 0x01];
        payload.resize(payload.len() + 4097, 0);
        let mut frame = vec![MAGIC, VERSION, 0, 1];
        frame.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        frame.extend_from_slice(&payload);
        assert!(matches!(
            decode::<ControlMessage>(&frame),
            Err(CodecError::Serialization(_))
        ));
    }

    #[test]
    fn message_type_identifies_each_variant() {
        let cases = [
            (
                ControlMessage::ClientHello(crate::control::ClientHello {
                    protocol_version: 1,
                    supported_video_codecs: vec![],
                    capabilities: 0,
                }),
                0x0001,
            ),
            (
                ControlMessage::HostHello(crate::control::HostHello {
                    selected_video_codec: "h264".into(),
                    displays: vec![],
                    audio_capabilities: 0,
                }),
                0x0002,
            ),
            (ControlMessage::DisplayTopology(vec![]), 0x0003),
            (ControlMessage::SelectDisplay { display_index: 0 }, 0x0004),
            (
                ControlMessage::QualityHint(crate::control::QualityHint {
                    max_bitrate_kbps: None,
                    max_framerate: None,
                }),
                0x0005,
            ),
            (
                ControlMessage::Ping {
                    timestamp_micros: 0,
                },
                0x0006,
            ),
            (
                ControlMessage::Pong {
                    timestamp_micros: 0,
                },
                0x0007,
            ),
            (
                ControlMessage::Terminate {
                    reason: crate::control::TerminationReason::Normal,
                },
                0x0008,
            ),
        ];
        for (message, expected_type) in cases {
            let frame = encode(&message).unwrap();
            assert_eq!(u16::from_be_bytes([frame[2], frame[3]]), expected_type);
        }
        let input = crate::InputMessage::MouseWheel {
            delta_x: 0.0,
            delta_y: 0.0,
        };
        let frame = encode(&input).unwrap();
        assert_eq!(u16::from_be_bytes([frame[2], frame[3]]), 0x0104);
        let variants = [
            (
                crate::InputMessage::MouseMove {
                    display_id: 0,
                    x_norm: 0.0,
                    y_norm: 0.0,
                },
                0x0101,
            ),
            (
                crate::InputMessage::MouseDown {
                    button: crate::input::MouseButton::Left,
                    click_count: 1,
                },
                0x0102,
            ),
            (
                crate::InputMessage::MouseUp {
                    button: crate::input::MouseButton::Left,
                },
                0x0103,
            ),
            (input, 0x0104),
            (
                crate::InputMessage::KeyDown {
                    scancode: 0,
                    modifiers: 0,
                    unicode_char: None,
                },
                0x0105,
            ),
            (
                crate::InputMessage::KeyUp {
                    scancode: 0,
                    modifiers: 0,
                },
                0x0106,
            ),
            (
                crate::InputMessage::TouchGesture {
                    gesture: "tap".into(),
                    x_norm: 0.0,
                    y_norm: 0.0,
                },
                0x0107,
            ),
            (
                crate::InputMessage::SystemAction(crate::input::SystemAction::ShowDesktop),
                0x0108,
            ),
            (
                crate::InputMessage::TextComposition {
                    text: String::new(),
                },
                0x0109,
            ),
        ];
        for (message, expected_type) in variants {
            let frame = encode(&message).unwrap();
            assert_eq!(u16::from_be_bytes([frame[2], frame[3]]), expected_type);
        }
    }

    #[test]
    fn rejects_header_id_that_does_not_match_payload_variant() {
        let mut frame = encode(&ControlMessage::Ping {
            timestamp_micros: 1,
        })
        .unwrap();
        frame[3] = 7;
        assert!(matches!(
            decode::<ControlMessage>(&frame),
            Err(CodecError::UnexpectedMessageType {
                expected: 6,
                actual: 7
            })
        ));
    }
}
