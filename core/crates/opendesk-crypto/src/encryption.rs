//! Authenticated ephemeral X25519 key agreement and replay-protected AEAD traffic keys.

use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    ChaCha20Poly1305, Nonce,
};
use hkdf::Hkdf;
use rand_core::RngCore;
use sha2::Sha256;
use thiserror::Error;
use zeroize::Zeroize;

use crate::identity::{verify, DeviceIdentity, IdentityError, PublicIdentityKey, Signature};

const TRANSCRIPT_DOMAIN: &[u8] = b"OpenDesk-E2EE-handshake-v1";
const KEY_DERIVATION_DOMAIN: &[u8] = b"OpenDesk-E2EE-v1/";
const MAX_REORDER: u64 = 64;

/// The fixed client/server role used to order handshake fields and derive directional keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HandshakeRole {
    /// The side that initiated the connection.
    Controller,
    /// The side that accepted the connection.
    Host,
}

/// An ephemeral X25519 key agreement participant. Create a fresh instance per session.
pub struct EphemeralKeyExchange {
    secret: x25519_dalek::StaticSecret,
    public: x25519_dalek::PublicKey,
}

impl EphemeralKeyExchange {
    /// Generates a fresh ephemeral keypair using the operating system RNG.
    pub fn generate() -> Result<Self, EncryptionError> {
        let mut seed = [0_u8; 32];
        if rand_core::OsRng.try_fill_bytes(&mut seed).is_err() {
            seed.zeroize();
            return Err(EncryptionError::RandomnessUnavailable);
        }
        let secret = x25519_dalek::StaticSecret::from(seed);
        seed.zeroize();
        let public = x25519_dalek::PublicKey::from(&secret);
        Ok(Self { secret, public })
    }

    /// Returns the 32-byte public key to send to the peer.
    pub fn public_key(&self) -> [u8; 32] {
        self.public.to_bytes()
    }

    /// Signs the canonical transcript with this peer's persistent Ed25519 identity.
    pub fn sign_transcript(
        &self,
        role: HandshakeRole,
        identity: &DeviceIdentity,
        transcript: &HandshakeTranscript,
    ) -> Result<Signature, EncryptionError> {
        if !transcript.contains_local(role, self.public_key(), identity.public_key()) {
            return Err(EncryptionError::TranscriptMismatch);
        }
        Ok(identity.sign(transcript.as_bytes()))
    }

    /// Verifies the trusted peer's signature and derives independent send/receive keys.
    ///
    /// `trusted_peer_identity` must come from an authenticated device record or a user-approved
    /// key fingerprint. Accepting an identity delivered only by the signaling server does not
    /// authenticate the exchange.
    pub fn derive_session_keys(
        &self,
        role: HandshakeRole,
        identity: &DeviceIdentity,
        trusted_peer_identity: &PublicIdentityKey,
        peer_signature: &Signature,
        transcript: &HandshakeTranscript,
    ) -> Result<SessionKeys, EncryptionError> {
        if !transcript.contains_local(role, self.public_key(), identity.public_key()) {
            return Err(EncryptionError::TranscriptMismatch);
        }
        if transcript.peer_identity(role) != *trusted_peer_identity {
            return Err(EncryptionError::TranscriptMismatch);
        }
        verify(trusted_peer_identity, transcript.as_bytes(), peer_signature)
            .map_err(EncryptionError::PeerAuthenticationFailed)?;

        let peer_public = match role {
            HandshakeRole::Controller => transcript.host_ephemeral,
            HandshakeRole::Host => transcript.controller_ephemeral,
        };
        let shared_secret = self
            .secret
            .diffie_hellman(&x25519_dalek::PublicKey::from(peer_public));
        if !shared_secret.was_contributory() {
            return Err(EncryptionError::InvalidPeerPublicKey);
        }

        let hkdf = Hkdf::<Sha256>::new(Some(&transcript.salt), shared_secret.as_bytes());
        let controller_to_host =
            derive_traffic_key(&hkdf, transcript.as_bytes(), b"controller-to-host")?;
        let host_to_controller =
            derive_traffic_key(&hkdf, transcript.as_bytes(), b"host-to-controller")?;
        let (send, receive) = match role {
            HandshakeRole::Controller => (controller_to_host, host_to_controller),
            HandshakeRole::Host => (host_to_controller, controller_to_host),
        };
        Ok(SessionKeys {
            send: TrafficKey::new(send),
            receive: TrafficKey::new(receive),
        })
    }
}

fn derive_traffic_key(
    hkdf: &Hkdf<Sha256>,
    transcript: &[u8],
    direction: &[u8],
) -> Result<[u8; 32], EncryptionError> {
    let mut info =
        Vec::with_capacity(KEY_DERIVATION_DOMAIN.len() + direction.len() + transcript.len());
    info.extend_from_slice(KEY_DERIVATION_DOMAIN);
    info.extend_from_slice(direction);
    info.extend_from_slice(transcript);
    let mut key = [0_u8; 32];
    hkdf.expand(&info, &mut key)
        .map_err(|_| EncryptionError::KeyDerivationFailed)?;
    info.zeroize();
    Ok(key)
}

/// Canonical, signed handshake context shared by both peers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandshakeTranscript {
    session_id: [u8; 16],
    salt: [u8; 32],
    controller_identity: PublicIdentityKey,
    host_identity: PublicIdentityKey,
    controller_ephemeral: [u8; 32],
    host_ephemeral: [u8; 32],
    encoded: Vec<u8>,
}

impl HandshakeTranscript {
    /// Generates a fresh 32-byte handshake salt with the operating system CSPRNG.
    pub fn generate_salt() -> Result<[u8; 32], EncryptionError> {
        let mut salt = [0_u8; 32];
        if rand_core::OsRng.try_fill_bytes(&mut salt).is_err() {
            salt.zeroize();
            return Err(EncryptionError::RandomnessUnavailable);
        }
        Ok(salt)
    }

    /// Builds a transcript in fixed controller/host order. Generate `salt` from a CSPRNG once per
    /// session and exchange it alongside the ephemeral keys before both peers sign this value.
    pub fn new(
        session_id: [u8; 16],
        salt: [u8; 32],
        controller_identity: PublicIdentityKey,
        host_identity: PublicIdentityKey,
        controller_ephemeral: [u8; 32],
        host_ephemeral: [u8; 32],
    ) -> Self {
        let mut encoded = Vec::with_capacity(TRANSCRIPT_DOMAIN.len() + 16 + 32 + 128);
        encoded.extend_from_slice(TRANSCRIPT_DOMAIN);
        encoded.extend_from_slice(&session_id);
        encoded.extend_from_slice(&salt);
        encoded.extend_from_slice(controller_identity.as_bytes());
        encoded.extend_from_slice(host_identity.as_bytes());
        encoded.extend_from_slice(&controller_ephemeral);
        encoded.extend_from_slice(&host_ephemeral);
        Self {
            session_id,
            salt,
            controller_identity,
            host_identity,
            controller_ephemeral,
            host_ephemeral,
            encoded,
        }
    }

    /// Returns the exact bytes signed by both peers.
    pub fn as_bytes(&self) -> &[u8] {
        &self.encoded
    }

    fn contains_local(
        &self,
        role: HandshakeRole,
        ephemeral: [u8; 32],
        identity: PublicIdentityKey,
    ) -> bool {
        match role {
            HandshakeRole::Controller => {
                self.controller_ephemeral == ephemeral && self.controller_identity == identity
            }
            HandshakeRole::Host => {
                self.host_ephemeral == ephemeral && self.host_identity == identity
            }
        }
    }

    fn peer_identity(&self, role: HandshakeRole) -> PublicIdentityKey {
        match role {
            HandshakeRole::Controller => self.host_identity,
            HandshakeRole::Host => self.controller_identity,
        }
    }
}

/// Direction-specific send and receive keys with nonce sequencing and replay protection.
pub struct SessionKeys {
    send: TrafficKey,
    receive: TrafficKey,
}

impl SessionKeys {
    /// Encrypts with a monotonically increasing nonce sequence and binds `aad` to the message.
    pub fn encrypt(
        &mut self,
        plaintext: &[u8],
        aad: &[u8],
    ) -> Result<EncryptedEnvelope, EncryptionError> {
        let sequence = self
            .send
            .next_sequence
            .ok_or(EncryptionError::SequenceExhausted)?;
        let nonce_bytes = sequence_nonce(sequence);
        let cipher = ChaCha20Poly1305::new((&self.send.key).into());
        let ciphertext = cipher
            .encrypt(
                Nonce::from_slice(&nonce_bytes),
                Payload {
                    msg: plaintext,
                    aad,
                },
            )
            .map_err(|_| EncryptionError::EncryptionFailed)?;
        self.send.next_sequence = sequence.checked_add(1);
        Ok(EncryptedEnvelope {
            nonce: nonce_bytes,
            ciphertext,
        })
    }

    /// Authenticates, decrypts, and rejects duplicate or stale envelopes.
    pub fn decrypt(
        &mut self,
        envelope: &EncryptedEnvelope,
        aad: &[u8],
    ) -> Result<Vec<u8>, EncryptionError> {
        if envelope.nonce[..4] != [0; 4] {
            return Err(EncryptionError::MalformedEnvelope);
        }
        let sequence = u64::from_be_bytes(
            envelope.nonce[4..]
                .try_into()
                .map_err(|_| EncryptionError::MalformedEnvelope)?,
        );
        if !self.receive.replay.can_accept(sequence) {
            return Err(EncryptionError::ReplayRejected);
        }
        let cipher = ChaCha20Poly1305::new((&self.receive.key).into());
        let plaintext = cipher
            .decrypt(
                Nonce::from_slice(&envelope.nonce),
                Payload {
                    msg: &envelope.ciphertext,
                    aad,
                },
            )
            .map_err(|_| EncryptionError::AuthenticationFailed)?;
        self.receive.replay.accept(sequence);
        Ok(plaintext)
    }
}

struct TrafficKey {
    key: [u8; 32],
    next_sequence: Option<u64>,
    replay: ReplayWindow,
}

impl Drop for TrafficKey {
    fn drop(&mut self) {
        self.key.zeroize();
    }
}

impl TrafficKey {
    fn new(key: [u8; 32]) -> Self {
        Self {
            key,
            next_sequence: Some(0),
            replay: ReplayWindow::default(),
        }
    }
}

#[derive(Default)]
struct ReplayWindow {
    highest: Option<u64>,
    seen: u64,
}

impl ReplayWindow {
    fn can_accept(&self, sequence: u64) -> bool {
        match self.highest {
            None => true,
            Some(highest) if sequence > highest => true,
            Some(highest) => {
                let distance = highest - sequence;
                distance < MAX_REORDER && self.seen & (1 << distance) == 0
            }
        }
    }

    fn accept(&mut self, sequence: u64) {
        match self.highest {
            None => {
                self.highest = Some(sequence);
                self.seen = 1;
            }
            Some(highest) if sequence > highest => {
                let shift = sequence - highest;
                self.seen = if shift >= MAX_REORDER {
                    1
                } else {
                    (self.seen << shift) | 1
                };
                self.highest = Some(sequence);
            }
            Some(highest) => self.seen |= 1 << (highest - sequence),
        }
    }
}

fn sequence_nonce(sequence: u64) -> [u8; 12] {
    let mut nonce = [0_u8; 12];
    nonce[4..].copy_from_slice(&sequence.to_be_bytes());
    nonce
}

/// AEAD envelope. Wire representation is the 12-byte sequence nonce followed by ciphertext/tag.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EncryptedEnvelope {
    nonce: [u8; 12],
    ciphertext: Vec<u8>,
}

impl EncryptedEnvelope {
    /// Parses the wire representation (12-byte nonce || ciphertext || 16-byte tag).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, EncryptionError> {
        if bytes.len() < 12 + 16 {
            return Err(EncryptionError::MalformedEnvelope);
        }
        let mut nonce = [0_u8; 12];
        nonce.copy_from_slice(&bytes[..12]);
        Ok(Self {
            nonce,
            ciphertext: bytes[12..].to_vec(),
        })
    }

    /// Serializes as nonce followed by authenticated ciphertext.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12 + self.ciphertext.len());
        bytes.extend_from_slice(&self.nonce);
        bytes.extend_from_slice(&self.ciphertext);
        bytes
    }
}

/// Errors returned by ephemeral key agreement and AEAD operations.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum EncryptionError {
    /// The operating system could not provide cryptographically secure randomness.
    #[error("operating system cryptographic randomness unavailable")]
    RandomnessUnavailable,
    /// The peer public key produces an all-zero X25519 shared secret.
    #[error("invalid or non-contributory X25519 public key")]
    InvalidPeerPublicKey,
    /// The local key or identity does not match the handshake transcript.
    #[error("local key or identity does not match handshake transcript")]
    TranscriptMismatch,
    /// The peer signature failed verification against the supplied trusted identity.
    #[error("peer handshake authentication failed: {0}")]
    PeerAuthenticationFailed(#[source] IdentityError),
    /// HKDF rejected the requested output length.
    #[error("HKDF-SHA256 key derivation failed")]
    KeyDerivationFailed,
    /// Cipher encryption failed.
    #[error("ChaCha20-Poly1305 encryption failed")]
    EncryptionFailed,
    /// Cipher authentication failed or associated data did not match.
    #[error("ChaCha20-Poly1305 authentication failed")]
    AuthenticationFailed,
    /// Envelope is shorter than a nonce and authentication tag or has an invalid nonce format.
    #[error("malformed encrypted envelope")]
    MalformedEnvelope,
    /// The envelope sequence is duplicated or outside the accepted replay window.
    #[error("replayed or stale encrypted envelope")]
    ReplayRejected,
    /// The send sequence reached its maximum representable value.
    #[error("encrypted message sequence exhausted; establish a new session")]
    SequenceExhausted,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (SessionKeys, SessionKeys) {
        let controller_identity = DeviceIdentity::from_secret_seed([1; 32]);
        let host_identity = DeviceIdentity::from_secret_seed([2; 32]);
        let controller_exchange = EphemeralKeyExchange::generate().unwrap();
        let host_exchange = EphemeralKeyExchange::generate().unwrap();
        let transcript = HandshakeTranscript::new(
            [3; 16],
            [4; 32],
            controller_identity.public_key(),
            host_identity.public_key(),
            controller_exchange.public_key(),
            host_exchange.public_key(),
        );
        let controller_signature = controller_exchange
            .sign_transcript(HandshakeRole::Controller, &controller_identity, &transcript)
            .unwrap();
        let host_signature = host_exchange
            .sign_transcript(HandshakeRole::Host, &host_identity, &transcript)
            .unwrap();
        let controller_keys = controller_exchange
            .derive_session_keys(
                HandshakeRole::Controller,
                &controller_identity,
                &host_identity.public_key(),
                &host_signature,
                &transcript,
            )
            .unwrap();
        let host_keys = host_exchange
            .derive_session_keys(
                HandshakeRole::Host,
                &host_identity,
                &controller_identity.public_key(),
                &controller_signature,
                &transcript,
            )
            .unwrap();
        (controller_keys, host_keys)
    }

    #[test]
    fn peers_derive_directional_keys_and_exchange_messages() {
        let (mut controller, mut host) = fixture();
        let envelope = controller.encrypt(b"payload", b"input-channel").unwrap();
        assert_eq!(
            host.decrypt(&envelope, b"input-channel").unwrap(),
            b"payload"
        );

        let reply = host.encrypt(b"reply", b"input-channel").unwrap();
        assert_eq!(
            controller.decrypt(&reply, b"input-channel").unwrap(),
            b"reply"
        );
    }

    #[test]
    fn rejects_untrusted_signature_and_transcript_tampering() {
        let controller = DeviceIdentity::from_secret_seed([1; 32]);
        let host = DeviceIdentity::from_secret_seed([2; 32]);
        let controller_exchange = EphemeralKeyExchange::generate().unwrap();
        let host_exchange = EphemeralKeyExchange::generate().unwrap();
        let transcript = HandshakeTranscript::new(
            [3; 16],
            [4; 32],
            controller.public_key(),
            host.public_key(),
            controller_exchange.public_key(),
            host_exchange.public_key(),
        );
        let signature = host_exchange
            .sign_transcript(HandshakeRole::Host, &host, &transcript)
            .unwrap();
        let altered = HandshakeTranscript::new(
            [9; 16],
            [4; 32],
            controller.public_key(),
            host.public_key(),
            controller_exchange.public_key(),
            host_exchange.public_key(),
        );
        let error = match controller_exchange.derive_session_keys(
            HandshakeRole::Controller,
            &controller,
            &host.public_key(),
            &signature,
            &altered,
        ) {
            Ok(_) => panic!("altered transcript must fail authentication"),
            Err(error) => error,
        };
        assert_eq!(
            error,
            EncryptionError::PeerAuthenticationFailed(IdentityError::VerificationFailed)
        );
    }

    #[test]
    fn rejects_tampering_wrong_aad_and_replays_but_allows_bounded_reordering() {
        let (mut controller, mut host) = fixture();
        let first = controller.encrypt(b"first", b"clipboard").unwrap();
        let second = controller.encrypt(b"second", b"clipboard").unwrap();
        assert_eq!(host.decrypt(&second, b"clipboard").unwrap(), b"second");
        assert_eq!(host.decrypt(&first, b"clipboard").unwrap(), b"first");
        assert_eq!(
            host.decrypt(&first, b"clipboard"),
            Err(EncryptionError::ReplayRejected)
        );

        let third = controller.encrypt(b"third", b"clipboard").unwrap();
        assert_eq!(
            host.decrypt(&third, b"wrong-channel"),
            Err(EncryptionError::AuthenticationFailed)
        );
        assert_eq!(host.decrypt(&third, b"clipboard").unwrap(), b"third");

        let mut bytes = controller
            .encrypt(b"tamper", b"clipboard")
            .unwrap()
            .to_bytes();
        *bytes.last_mut().unwrap() ^= 1;
        let tampered = EncryptedEnvelope::from_bytes(&bytes).unwrap();
        assert_eq!(
            host.decrypt(&tampered, b"clipboard"),
            Err(EncryptionError::AuthenticationFailed)
        );
    }

    #[test]
    fn rejects_malformed_envelopes_and_non_contributory_keys() {
        assert_eq!(
            EncryptedEnvelope::from_bytes(&[0; 27]),
            Err(EncryptionError::MalformedEnvelope)
        );
        let identity = DeviceIdentity::from_secret_seed([1; 32]);
        let exchange = EphemeralKeyExchange::generate().unwrap();
        let host_identity = DeviceIdentity::from_secret_seed([2; 32]);
        let transcript = HandshakeTranscript::new(
            [3; 16],
            [4; 32],
            identity.public_key(),
            host_identity.public_key(),
            exchange.public_key(),
            [0; 32],
        );
        let signature = host_identity.sign(transcript.as_bytes());
        let error = match exchange.derive_session_keys(
            HandshakeRole::Controller,
            &identity,
            &host_identity.public_key(),
            &signature,
            &transcript,
        ) {
            Ok(_) => panic!("zero peer key must fail key agreement"),
            Err(error) => error,
        };
        assert_eq!(error, EncryptionError::InvalidPeerPublicKey);
    }
}
