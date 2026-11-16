use crate::ProtocolMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub is_directory: bool,
    pub size_bytes: u64,
    pub modified_unix_millis: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RejectReason {
    PermissionDenied,
    InvalidPath,
    Unsupported,
    InsufficientSpace,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileTransferMessage {
    ListDirectoryRequest {
        path: String,
    },
    ListDirectoryResponse {
        path: String,
        #[serde(deserialize_with = "crate::serde_helpers::deserialize_bounded_vec")]
        entries: Vec<FileEntry>,
    },
    FileOffer {
        transfer_id: u64,
        file_name: String,
        file_size_bytes: u64,
        sha256_hash: [u8; 32],
        is_directory: bool,
    },
    FileAccept {
        transfer_id: u64,
        destination_dir: String,
        resume_offset_bytes: u64,
    },
    FileReject {
        transfer_id: u64,
        reason: RejectReason,
    },
    FileChunk {
        transfer_id: u64,
        chunk_index: u64,
        offset_bytes: u64,
        #[serde(with = "crate::serde_helpers::bytes")]
        data: Vec<u8>,
        crc32: u32,
    },
    FileChunkAck {
        transfer_id: u64,
        acknowledged_chunk_index: u64,
    },
    FileComplete {
        transfer_id: u64,
        final_sha256: [u8; 32],
    },
    FileCancel {
        transfer_id: u64,
        reason: String,
    },
}

impl ProtocolMessage for FileTransferMessage {
    fn message_type(&self) -> u16 {
        match self {
            Self::ListDirectoryRequest { .. } => 0x0301,
            Self::ListDirectoryResponse { .. } => 0x0302,
            Self::FileOffer { .. } => 0x0303,
            Self::FileAccept { .. } => 0x0304,
            Self::FileReject { .. } => 0x0305,
            Self::FileChunk { .. } => 0x0306,
            Self::FileChunkAck { .. } => 0x0307,
            Self::FileComplete { .. } => 0x0308,
            Self::FileCancel { .. } => 0x0309,
        }
    }

    fn validate(&self) -> Result<(), &'static str> {
        const MAX_CHUNK_SIZE: usize = 64 * 1024;
        match self {
            Self::FileOffer { file_name, .. } if file_name.is_empty() => {
                Err("offered file name cannot be empty")
            }
            Self::FileChunk { data, .. } if data.len() > MAX_CHUNK_SIZE => {
                Err("file chunk exceeds 64 KiB")
            }
            Self::FileAccept {
                destination_dir, ..
            } if destination_dir.is_empty() => Err("file destination directory cannot be empty"),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{decode, encode};
    #[test]
    fn file_transfer_variants_roundtrip() {
        let hash = [7; 32];
        let messages = [
            FileTransferMessage::ListDirectoryRequest {
                path: "/tmp".into(),
            },
            FileTransferMessage::ListDirectoryResponse {
                path: "/tmp".into(),
                entries: vec![FileEntry {
                    name: "a.txt".into(),
                    is_directory: false,
                    size_bytes: 3,
                    modified_unix_millis: Some(12),
                }],
            },
            FileTransferMessage::FileOffer {
                transfer_id: 1,
                file_name: "a.txt".into(),
                file_size_bytes: 3,
                sha256_hash: hash,
                is_directory: false,
            },
            FileTransferMessage::FileAccept {
                transfer_id: 1,
                destination_dir: "/tmp".into(),
                resume_offset_bytes: 0,
            },
            FileTransferMessage::FileReject {
                transfer_id: 1,
                reason: RejectReason::PermissionDenied,
            },
            FileTransferMessage::FileChunk {
                transfer_id: 1,
                chunk_index: 0,
                offset_bytes: 0,
                data: vec![1, 2, 3],
                crc32: 42,
            },
            FileTransferMessage::FileChunkAck {
                transfer_id: 1,
                acknowledged_chunk_index: 0,
            },
            FileTransferMessage::FileComplete {
                transfer_id: 1,
                final_sha256: hash,
            },
            FileTransferMessage::FileCancel {
                transfer_id: 1,
                reason: "cancelled".into(),
            },
        ];
        for message in messages {
            assert_eq!(
                decode::<FileTransferMessage>(&encode(&message).unwrap()).unwrap(),
                message
            );
        }
    }

    #[test]
    fn rejects_file_chunks_over_64_kib() {
        let message = FileTransferMessage::FileChunk {
            transfer_id: 1,
            chunk_index: 0,
            offset_bytes: 0,
            data: vec![0; 64 * 1024 + 1],
            crc32: 0,
        };
        assert!(crate::encode(&message).is_err());
    }

    #[test]
    fn file_transfer_variants_use_distinct_wire_ids() {
        let messages = [
            (
                FileTransferMessage::ListDirectoryRequest {
                    path: String::new(),
                },
                0x0301,
            ),
            (
                FileTransferMessage::ListDirectoryResponse {
                    path: String::new(),
                    entries: vec![],
                },
                0x0302,
            ),
            (
                FileTransferMessage::FileOffer {
                    transfer_id: 0,
                    file_name: "file".into(),
                    file_size_bytes: 0,
                    sha256_hash: [0; 32],
                    is_directory: false,
                },
                0x0303,
            ),
            (
                FileTransferMessage::FileAccept {
                    transfer_id: 0,
                    destination_dir: "/tmp".into(),
                    resume_offset_bytes: 0,
                },
                0x0304,
            ),
            (
                FileTransferMessage::FileReject {
                    transfer_id: 0,
                    reason: RejectReason::Unsupported,
                },
                0x0305,
            ),
            (
                FileTransferMessage::FileChunk {
                    transfer_id: 0,
                    chunk_index: 0,
                    offset_bytes: 0,
                    data: vec![],
                    crc32: 0,
                },
                0x0306,
            ),
            (
                FileTransferMessage::FileChunkAck {
                    transfer_id: 0,
                    acknowledged_chunk_index: 0,
                },
                0x0307,
            ),
            (
                FileTransferMessage::FileComplete {
                    transfer_id: 0,
                    final_sha256: [0; 32],
                },
                0x0308,
            ),
            (
                FileTransferMessage::FileCancel {
                    transfer_id: 0,
                    reason: String::new(),
                },
                0x0309,
            ),
        ];
        for (message, expected) in messages {
            assert_eq!(message.message_type(), expected);
        }
    }
}
