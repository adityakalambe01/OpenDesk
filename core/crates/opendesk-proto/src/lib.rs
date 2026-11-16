//! Shared, platform-independent OpenDesk wire protocol types and codec.
#![forbid(unsafe_code)]

pub mod clipboard;
mod codec;
pub mod control;
pub mod file;
pub mod input;
mod serde_helpers;

pub use clipboard::ClipboardMessage;
pub use control::ControlMessage;
pub use file::{FileEntry, FileTransferMessage, RejectReason};
pub use input::{InputMessage, SystemAction};

/// Serializes a protocol message into an OpenDesk frame.
pub use codec::{decode, encode, CodecError, ProtocolMessage, HEADER_LEN, MAGIC, VERSION};
