use crate::ProtocolMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClipboardMessage {
    Text {
        content: String,
    },
    ImagePng {
        #[serde(with = "crate::serde_helpers::bytes")]
        data: Vec<u8>,
    },
    FormatAnnouncement {
        #[serde(deserialize_with = "crate::serde_helpers::deserialize_bounded_vec")]
        available_formats: Vec<String>,
    },
}

impl ProtocolMessage for ClipboardMessage {
    fn message_type(&self) -> u16 {
        match self {
            Self::Text { .. } => 0x0201,
            Self::ImagePng { .. } => 0x0202,
            Self::FormatAnnouncement { .. } => 0x0203,
        }
    }

    fn validate(&self) -> Result<(), &'static str> {
        const MAX_CLIPBOARD_CONTENT: usize = 10 * 1024 * 1024;
        let content_len = match self {
            Self::Text { content } => content.len(),
            Self::ImagePng { data } => data.len(),
            Self::FormatAnnouncement { available_formats } => available_formats
                .iter()
                .try_fold(0usize, |total, format| total.checked_add(format.len()))
                .ok_or("clipboard format list is too large")?,
        };
        if content_len > MAX_CLIPBOARD_CONTENT {
            return Err("clipboard content exceeds 10 MiB");
        }
        if matches!(self, Self::FormatAnnouncement { available_formats } if available_formats.iter().any(String::is_empty))
        {
            return Err("clipboard format names cannot be empty");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{decode, encode};
    #[test]
    fn clipboard_variants_roundtrip() {
        for message in [
            ClipboardMessage::Text {
                content: "hi".into(),
            },
            ClipboardMessage::ImagePng {
                data: vec![1, 2, 3],
            },
            ClipboardMessage::FormatAnnouncement {
                available_formats: vec!["text/plain".into(), "image/png".into()],
            },
        ] {
            assert_eq!(
                decode::<ClipboardMessage>(&encode(&message).unwrap()).unwrap(),
                message
            );
        }
    }

    #[test]
    fn rejects_clipboard_payload_over_limit() {
        let message = ClipboardMessage::ImagePng {
            data: vec![0; 10 * 1024 * 1024 + 1],
        };
        assert!(crate::encode(&message).is_err());
    }

    #[test]
    fn clipboard_variants_use_distinct_wire_ids() {
        let messages = [
            (
                ClipboardMessage::Text {
                    content: String::new(),
                },
                0x0201,
            ),
            (ClipboardMessage::ImagePng { data: vec![] }, 0x0202),
            (
                ClipboardMessage::FormatAnnouncement {
                    available_formats: vec!["text/plain".into()],
                },
                0x0203,
            ),
        ];
        for (message, expected) in messages {
            assert_eq!(message.message_type(), expected);
        }
    }
}
