//! Ed25519 device identities and message attestation.

use ed25519_dalek::Signer;
use rand_core::RngCore;
use thiserror::Error;
use zeroize::Zeroize;

/// An Ed25519 public identity key in its canonical 32-byte representation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PublicIdentityKey([u8; 32]);

impl PublicIdentityKey {
    /// Returns the key's canonical bytes for registration or storage.
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Parses a canonical Ed25519 public key.
    pub fn from_bytes(bytes: [u8; 32]) -> Result<Self, IdentityError> {
        let key = ed25519_dalek::VerifyingKey::from_bytes(&bytes)
            .map_err(|_| IdentityError::InvalidPublicKey)?;
        if key.is_weak() {
            return Err(IdentityError::InvalidPublicKey);
        }
        Ok(Self(bytes))
    }
}

/// An Ed25519 signature in its canonical 64-byte representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Signature([u8; 64]);

impl Signature {
    /// Returns the signature's canonical bytes.
    pub const fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }

    /// Parses a 64-byte Ed25519 signature.
    pub const fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }
}

/// A persistent device identity. Keep this value secret and store it in an OS vault.
pub struct DeviceIdentity {
    signing_key: ed25519_dalek::SigningKey,
}

impl DeviceIdentity {
    /// Generates a new identity using the operating system's cryptographic RNG.
    pub fn generate() -> Result<Self, IdentityError> {
        let mut seed = [0_u8; 32];
        if rand_core::OsRng.try_fill_bytes(&mut seed).is_err() {
            seed.zeroize();
            return Err(IdentityError::RandomnessUnavailable);
        }
        let identity = Self::from_secret_seed(seed);
        seed.zeroize();
        Ok(identity)
    }

    /// Restores an identity from a 32-byte secret seed.
    ///
    /// The caller is responsible for loading the seed from secure storage.
    pub fn from_secret_seed(mut seed: [u8; 32]) -> Self {
        let signing_key = ed25519_dalek::SigningKey::from_bytes(&seed);
        seed.zeroize();
        Self { signing_key }
    }

    /// Returns the seed for secure persistence. Do not expose it to the network.
    ///
    /// The returned copy contains secret key material; clear it after use.
    pub fn secret_seed(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    /// Returns the public key suitable for device registration.
    pub fn public_key(&self) -> PublicIdentityKey {
        PublicIdentityKey(self.signing_key.verifying_key().to_bytes())
    }

    /// Signs an attestation payload (for example nonce || timestamp || session ID).
    pub fn sign(&self, message: &[u8]) -> Signature {
        Signature(self.signing_key.sign(message).to_bytes())
    }
}

/// Verifies a detached Ed25519 signature against a public device identity key.
pub fn verify(
    public_key: &PublicIdentityKey,
    message: &[u8],
    signature: &Signature,
) -> Result<(), IdentityError> {
    let key = ed25519_dalek::VerifyingKey::from_bytes(&public_key.0)
        .map_err(|_| IdentityError::InvalidPublicKey)?;
    let signature = ed25519_dalek::Signature::from_bytes(&signature.0);
    key.verify_strict(message, &signature)
        .map_err(|_| IdentityError::VerificationFailed)
}

/// Errors returned by device identity operations.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum IdentityError {
    /// The operating system could not provide cryptographically secure randomness.
    #[error("operating system cryptographic randomness unavailable")]
    RandomnessUnavailable,
    /// The supplied bytes do not encode a valid Ed25519 public key.
    #[error("invalid Ed25519 public key")]
    InvalidPublicKey,
    /// The signature does not authenticate the supplied message and key.
    #[error("Ed25519 signature verification failed")]
    VerificationFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_identity_signs_and_verifies() {
        let identity = DeviceIdentity::generate().expect("OS RNG should be available in tests");
        let message = b"nonce || timestamp || session id";
        let signature = identity.sign(message);

        assert_eq!(identity.public_key().as_bytes().len(), 32);
        assert_eq!(signature.as_bytes().len(), 64);
        assert_eq!(verify(&identity.public_key(), message, &signature), Ok(()));
    }

    #[test]
    fn verification_rejects_changed_message_and_key() {
        let identity = DeviceIdentity::from_secret_seed([7; 32]);
        let other = DeviceIdentity::from_secret_seed([8; 32]);
        let signature = identity.sign(b"approved session");

        assert_eq!(
            verify(&identity.public_key(), b"different session", &signature),
            Err(IdentityError::VerificationFailed)
        );
        assert_eq!(
            verify(&other.public_key(), b"approved session", &signature),
            Err(IdentityError::VerificationFailed)
        );
    }

    #[test]
    fn rejects_weak_public_keys_and_tampered_signatures() {
        // The compressed Edwards identity point is a small-order (weak) public key.
        let mut weak_key = [0_u8; 32];
        weak_key[0] = 1;
        assert_eq!(
            PublicIdentityKey::from_bytes(weak_key),
            Err(IdentityError::InvalidPublicKey)
        );

        let identity = DeviceIdentity::from_secret_seed([9; 32]);
        let message = b"signed payload";
        let mut signature_bytes = *identity.sign(message).as_bytes();
        signature_bytes[0] ^= 1;
        let signature = Signature::from_bytes(signature_bytes);
        assert_eq!(
            verify(&identity.public_key(), message, &signature),
            Err(IdentityError::VerificationFailed)
        );
    }

    #[test]
    fn matches_rfc_8032_test_vector_1() {
        let seed = [
            0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4, 0x92, 0xec,
            0x2c, 0xc4, 0x44, 0x49, 0xc5, 0x69, 0x7b, 0x32, 0x69, 0x19, 0x70, 0x3b, 0xac, 0x03,
            0x1c, 0xae, 0x7f, 0x60,
        ];
        let expected_public = [
            0xd7, 0x5a, 0x98, 0x01, 0x82, 0xb1, 0x0a, 0xb7, 0xd5, 0x4b, 0xfe, 0xd3, 0xc9, 0x64,
            0x07, 0x3a, 0x0e, 0xe1, 0x72, 0xf3, 0xda, 0xa6, 0x23, 0x25, 0xaf, 0x02, 0x1a, 0x68,
            0xf7, 0x07, 0x51, 0x1a,
        ];
        let expected_signature = [
            0xe5, 0x56, 0x43, 0x00, 0xc3, 0x60, 0xac, 0x72, 0x90, 0x86, 0xe2, 0xcc, 0x80, 0x6e,
            0x82, 0x8a, 0x84, 0x87, 0x7f, 0x1e, 0xb8, 0xe5, 0xd9, 0x74, 0xd8, 0x73, 0xe0, 0x65,
            0x22, 0x49, 0x01, 0x55, 0x5f, 0xb8, 0x82, 0x15, 0x90, 0xa3, 0x3b, 0xac, 0xc6, 0x1e,
            0x39, 0x70, 0x1c, 0xf9, 0xb4, 0x6b, 0xd2, 0x5b, 0xf5, 0xf0, 0x59, 0x5b, 0xbe, 0x24,
            0x65, 0x51, 0x41, 0x43, 0x8e, 0x7a, 0x10, 0x0b,
        ];
        let identity = DeviceIdentity::from_secret_seed(seed);
        let signature = identity.sign(b"");

        assert_eq!(*identity.public_key().as_bytes(), expected_public);
        assert_eq!(*signature.as_bytes(), expected_signature);
        assert_eq!(verify(&identity.public_key(), b"", &signature), Ok(()));
    }
}
