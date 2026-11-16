# OpenDesk Phase 5 — Authentication, Identity & Security Hardening

## 1. Phase Objective
Implement cryptographic device identity, device pairing, separate attended support and unattended access authorization models, application-layer end-to-end encryption (E2EE), emergency session kill switches, and immutable audit logging.

## 2. Why This Phase Is Necessary
Remote desktop tools are high-value targets for ransomware, credential harvesting, and unauthorized corporate intrusions. Phase 5 enforces zero-trust security invariants across identity, session negotiation, and transport boundaries.

## 3. Scope
- Implementing persistent Ed25519 device key generation and platform secure storage (DPAPI / Keychain / Secret Service).
- Implementing device registration and attestation against the ASP.NET Core control plane.
- Implementing attended support session generation (9-digit code + 6-char PIN) with interactive consent dialogs.
- Implementing unattended access authentication via Argon2id password hashing or account public key challenges.
- Implementing application-layer E2EE (X25519 ECDH + ChaCha20-Poly1305) over SCTP data channels.
- Building the host-side persistent floating toolbar with the emergency disconnect kill switch.
- Implementing server-side immutable audit logging in PostgreSQL.

## 4. Prerequisites
Completion of Phase 4 (Networking & Signaling).

## 5. Technical Implementation Details
- Private keys never leave the host OS secure storage.
- Attended support tokens cannot modify unattended access policies.
- Input data channel requires active cryptographic authorization ticket; otherwise, packets are discarded by the host engine.
- Every session start, stop, permission grant, and file transfer generates an audit entry in PostgreSQL.

## 6. Exact Folder and File Changes Expected
```
core/crates/opendesk-crypto/src/storage/
├── mod.rs
├── windows.rs
├── macos.rs
└── linux.rs
server/src/OpenDesk.Application/
├── Devices/Commands/
│   ├── RegisterDeviceCommand.cs
│   └── RevokeDeviceCommand.cs
└── Sessions/Commands/
    ├── AuthorizeSessionCommand.cs
    └── RevokeSessionCommand.cs
clients/desktop/src/components/
├── OverlayToolbar/
│   ├── FloatingToolbar.tsx
│   └── KillSwitchButton.tsx
└── Modals/
    └── ConsentDialog.tsx
```

## 7. Individual Development Tasks
- **P5-001**: Implement native OS secure storage adapters for device Ed25519 private keys.
- **P5-002**: Implement device registration and public key attestation in ASP.NET Core backend.
- **P5-003**: Implement attended support session generation (9-digit code, PIN) with 10-minute Redis TTL.
- **P5-004**: Implement host interactive consent modal in Tauri React UI.
- **P5-005**: Implement unattended access Argon2id password hashing and account key validation.
- **P5-006**: Implement application-layer E2EE envelope (X25519 ECDH + ChaCha20-Poly1305) over SCTP data channels.
- **P5-007**: Implement persistent floating overlay toolbar with immediate disconnect kill switch.
- **P5-008**: Implement immutable audit logging in PostgreSQL recording all session events.
- **P5-009**: Execute automated security tests (rate limiting, traversal attacks, replay prevention).

## 8. Dependencies
Phase 4 (Networking & Signaling).

## 9. Expected Outputs
A verified, zero-trust remote desktop connection with explicit consent flows, tamper-evident device identities, and instant access revocation.

## 10. Unit Testing Requirements
- Unit tests verifying Argon2id password hashing and verification against test vectors.
- Unit tests verifying X25519 session key derivation and ChaCha20-Poly1305 envelope encryption.

## 11. Integration Testing Requirements
Integration tests verifying that an invalid device signature rejects connection negotiation at the backend.

## 12. Manual Verification Steps
1. Generate an attended support code on Host $\rightarrow$ Connect from Client.
2. Verify host displays interactive consent modal.
3. Click "Reject" $\rightarrow$ Verify client is immediately denied.
4. Re-initiate and click "Full Control" $\rightarrow$ Verify session establishes and floating banner appears.
5. Click "Disconnect Immediately" on host banner $\rightarrow$ Verify client drops instantly.

## 13. Acceptance Criteria
- Temporary support access never permits unattended access without separate re-authentication.
- Zero plaintext data visible on intermediate relay nodes.
- Emergency disconnect kill switch drops session in $< 100\text{ ms}$.

## 14. Definition of Done
All security workflows implemented, pass penetration test cases, and emit verified audit logs.

## 15. Known Risks
- User friction if consent dialogs are unintuitive.
  - *Mitigation*: Design clean, clear UI displaying technician identity, IP, and requested permissions.

## 16. Estimated Complexity
High (Security architecture, cryptographic verification, OS privilege controls).

## 17. Next-Phase Dependencies
Unlocks **Phase 6 (Desktop Cross-Platform)**, **Phase 7 (Mobile)**, and **Phase 8 (Backend Dashboard)**.
