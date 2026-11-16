# OpenDesk — Cryptography & End-to-End Encryption Specification

## 1. Cryptographic Standard & Ciphersuites

OpenDesk adopts modern, post-RSA cryptographic standards designed to eliminate side-channel timing attacks and deliver high performance across desktop and mobile architectures:

```
+-----------------------------------------------------------------------------------+
| OPENDESK CRYPTOGRAPHIC SUITE SPECIFICATION                                       |
+--------------------------+-----------------------+--------------------------------+
| Function                 | Algorithm Standard    | Parameter / Configuration      |
+--------------------------+-----------------------+--------------------------------+
| Asymmetric Identity      | Ed25519 (RFC 8032)    | Curve25519, SHA-512            |
| Key Agreement (ECDH)     | X25519 (RFC 7748)     | Montgomery curve, 256-bit      |
| Symmetric Data Channel   | ChaCha20-Poly1305     | RFC 8439, 256-bit Key, 96b IV  |
| WebRTC Transport Enc.    | DTLS 1.3 (RFC 9147)   | TLS_AES_128_GCM_SHA256 /       |
|                          |                       | TLS_CHACHA20_POLY1305_SHA256   |
| Real-time Media Framing  | SRTP (RFC 3711)       | AEAD_AES_128_GCM / AES-CTR     |
| Password Hashing         | Argon2id (RFC 9106)   | m=64MB, t=3 passes, p=4 lanes  |
| Key Derivation           | HKDF-SHA256 (RFC 5869)| Extract-and-Expand             |
| File Chunk Integrity     | SHA-256 (FIPS 180-4)  | 32-byte rolling chunk hash     |
+--------------------------+-----------------------+--------------------------------+
```

---

## 2. Ephemeral End-to-End Key Agreement (E2EE)

Even though WebRTC establishes DTLS 1.3 transport encryption, OpenDesk implements an **application-layer End-to-End Encryption (E2EE)** envelope for all sensitive data channels (Input, Clipboard, File Transfer). This ensures that even if a WebRTC endpoint implementation had a transport bug or trusted intermediate proxy, payload privacy remains mathematically guaranteed.

```mermaid
sequenceDiagram
    autonumber
    actor Controller as Controller Client
    actor Host as Host Machine

    Note over Controller: Generate Ephemeral X25519 Keypair:<br/>(priv_C, pub_C)
    Note over Host: Generate Ephemeral X25519 Keypair:<br/>(priv_H, pub_H)

    Controller->>Host: Encrypted Control Handshake: Send pub_C (32 bytes)
    Host->>Controller: Encrypted Control Handshake: Send pub_H (32 bytes)

    Note over Controller: Derive Shared Secret:<br/>K_shared = X25519(priv_C, pub_H)<br/>K_session = HKDF-SHA256(K_shared, salt, "OpenDesk-E2EE-v1")
    Note over Host: Derive Shared Secret:<br/>K_shared = X25519(priv_H, pub_C)<br/>K_session = HKDF-SHA256(K_shared, salt, "OpenDesk-E2EE-v1")

    Note over Controller,Host: Both peers now possess identical 32-byte K_session.<br/>Neither the Signaling Server nor Relay ever saw priv_C or priv_H.

    Controller->>Host: Input Message encrypted with ChaCha20-Poly1305(K_session, Nonce, Data)
    Host->>Host: Authenticated Decryption with K_session
```

---

## 3. Password Hashing Specification (Argon2id)

When configuring unattended access with a pre-shared master password, the password is never stored or transmitted in plaintext.

### Verification Parameters:
- **Algorithm**: `Argon2id` (v0x13).
- **Memory Cost ($m$)**: $65,536\text{ KiB}$ (64 MiB).
- **Time Cost ($t$)**: $3\text{ iterations}$.
- **Parallelism ($p$)**: $4\text{ threads}$.
- **Salt**: 16 bytes generated via CSPRNG.
- **Hash Output Length**: 32 bytes.

These parameters require $> 50\text{ ms}$ on a modern CPU to compute a single hash, rendering offline dictionary and GPU rainbow table attacks computationally intractable.
