//! Cryptographic primitives used by OpenDesk.
#![forbid(unsafe_code)]

pub mod identity;

pub use identity::{verify, DeviceIdentity, IdentityError, PublicIdentityKey, Signature};
