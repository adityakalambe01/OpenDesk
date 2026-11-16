use crate::ProtocolMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientHello {
    pub protocol_version: u16,
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_bounded_vec")]
    pub supported_video_codecs: Vec<String>,
    pub capabilities: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostHello {
    pub selected_video_codec: String,
    #[serde(deserialize_with = "crate::serde_helpers::deserialize_bounded_vec")]
    pub displays: Vec<DisplayDescriptor>,
    pub audio_capabilities: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayDescriptor {
    pub index: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub dpi: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityHint {
    pub max_bitrate_kbps: Option<u32>,
    pub max_framerate: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TerminationReason {
    Normal,
    HostShutdown,
    IdleTimeout,
    ProtocolError,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ControlMessage {
    ClientHello(ClientHello),
    HostHello(HostHello),
    DisplayTopology(
        #[serde(deserialize_with = "crate::serde_helpers::deserialize_bounded_vec")]
        Vec<DisplayDescriptor>,
    ),
    SelectDisplay {
        display_index: u32,
    },
    QualityHint(QualityHint),
    Ping {
        timestamp_micros: u64,
    },
    Pong {
        timestamp_micros: u64,
    },
    Terminate {
        reason: TerminationReason,
    },
}

impl ProtocolMessage for ControlMessage {
    fn message_type(&self) -> u16 {
        match self {
            Self::ClientHello(_) => 0x0001,
            Self::HostHello(_) => 0x0002,
            Self::DisplayTopology(_) => 0x0003,
            Self::SelectDisplay { .. } => 0x0004,
            Self::QualityHint(_) => 0x0005,
            Self::Ping { .. } => 0x0006,
            Self::Pong { .. } => 0x0007,
            Self::Terminate { .. } => 0x0008,
        }
    }

    fn validate(&self) -> Result<(), &'static str> {
        let valid_display = |display: &DisplayDescriptor| {
            display.width > 0 && display.height > 0 && display.dpi.is_finite() && display.dpi > 0.0
        };
        match self {
            Self::ClientHello(hello)
                if hello.supported_video_codecs.iter().any(String::is_empty) =>
            {
                Err("codec names cannot be empty")
            }
            Self::HostHello(hello) if hello.selected_video_codec.is_empty() => {
                Err("selected video codec cannot be empty")
            }
            Self::HostHello(hello) if !hello.displays.iter().all(valid_display) => {
                Err("display dimensions and DPI must be positive and finite")
            }
            Self::DisplayTopology(displays) if !displays.iter().all(valid_display) => {
                Err("display dimensions and DPI must be positive and finite")
            }
            Self::QualityHint(hint)
                if hint.max_bitrate_kbps == Some(0) || hint.max_framerate == Some(0) =>
            {
                Err("quality limits must be greater than zero")
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{decode, encode};
    #[test]
    fn control_variants_roundtrip() {
        let messages = [
            ControlMessage::ClientHello(ClientHello {
                protocol_version: 1,
                supported_video_codecs: vec!["h264".into()],
                capabilities: 7,
            }),
            ControlMessage::HostHello(HostHello {
                selected_video_codec: "h264".into(),
                displays: vec![],
                audio_capabilities: 2,
            }),
            ControlMessage::DisplayTopology(vec![DisplayDescriptor {
                index: 0,
                x: -10,
                y: 0,
                width: 1920,
                height: 1080,
                dpi: 1.0,
            }]),
            ControlMessage::SelectDisplay { display_index: 1 },
            ControlMessage::QualityHint(QualityHint {
                max_bitrate_kbps: Some(5000),
                max_framerate: Some(60),
            }),
            ControlMessage::Ping {
                timestamp_micros: 99,
            },
            ControlMessage::Pong {
                timestamp_micros: 99,
            },
            ControlMessage::Terminate {
                reason: TerminationReason::Normal,
            },
        ];
        for message in messages {
            assert_eq!(
                decode::<ControlMessage>(&encode(&message).unwrap()).unwrap(),
                message
            );
        }
    }
}
