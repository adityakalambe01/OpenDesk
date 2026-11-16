//! Cryptographic primitives used by OpenDesk.
#![forbid(unsafe_code)]

pub mod encryption;
pub mod identity;

pub use encryption::{
    EncryptedEnvelope, EncryptionError, EphemeralKeyExchange, HandshakeRole, HandshakeTranscript,
    SessionKeys,
};
pub use identity::{verify, DeviceIdentity, IdentityError, PublicIdentityKey, Signature};
