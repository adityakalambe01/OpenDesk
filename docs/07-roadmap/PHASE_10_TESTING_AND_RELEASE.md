# OpenDesk Phase 10 — Hardening, Code Signing & Public Beta Release

## 1. Phase Objective
Execute comprehensive security audits, load testing, network chaos testing, production code signing, installer generation, release packaging, and launch the OpenDesk v1.0.0 Public Beta to the open-source community.

## 2. Why This Phase Is Necessary
Prior to a public open-source launch, all binaries must be signed with trusted certificates to prevent OS warnings (SmartScreen / Gatekeeper), third-party dependencies must be audited for known CVEs, and the platform must be proven stable under high concurrent session loads.

## 3. Scope
- Executing automated end-to-end load tests using k6 (10,000 concurrent signaling connections).
- Executing network resilience tests (packet loss, high latency, jitter, packet reordering).
- Conducting static and dynamic application security scans (CodeQL, Trivy, OWASP ZAP).
- Setting up automated code signing pipelines for Windows (`.msi` / `.exe`), macOS (`.dmg` notarization), and Linux (`.deb` / `.AppImage`).
- Generating release checksums, GPG signatures, and SLSA Level 3 provenance attestations.
- Publishing community documentation, contributing guidelines, and launch announcements.

## 4. Prerequisites
Completion of all prior implementation phases (Phases 1 through 9).

## 5. Technical Implementation Details
- Windows Authenticode signing via Azure Trusted Signing or Hardware Security Module (HSM).
- macOS code signing with Apple Developer ID certificate and automated notarization via `xcrun notarytool`.
- Release automation driven by GitHub Actions triggering on git tag pushes.

## 6. Exact Folder and File Changes Expected
```
.github/workflows/
└── cd-release.yml
scripts/release/
├── sign-windows.ps1
├── sign-macos.sh
└── verify-release.sh
docs/
└── RELEASE_NOTES_v1.0.0.md
```

## 7. Individual Development Tasks
- **P10-001**: Execute k6 load tests validating 10,000 concurrent SignalR signaling connections.
- **P10-002**: Conduct network chaos testing under 15% packet loss and 150ms jitter.
- **P10-003**: Execute comprehensive security vulnerability scans and address any reported CVEs.
- **P10-004**: Configure Windows code signing pipeline producing signed `.msi` installers.
- **P10-005**: Configure macOS code signing and Apple Notary Service submission for `.dmg`.
- **P10-006**: Configure Linux `.deb` and `.AppImage` packaging with GPG signature generation.
- **P10-007**: Finalize public documentation, quickstart guides, and community issue templates.
- **P10-008**: Tag and publish OpenDesk v1.0.0 Public Beta release.

## 8. Dependencies
All previous phases (Phases 1 through 9).

## 9. Expected Outputs
Signed, verified installation packages for Windows, macOS, Linux, Android, and Docker, ready for public production use.

## 10. Unit Testing Requirements
Validation of automated release packaging scripts and checksum verification utilities.

## 11. Integration Testing Requirements
Clean installation test on fresh virtual machines verifying un-prompted installation and execution.

## 12. Manual Verification Steps
1. Download Windows `.msi` on a non-developer Windows 11 PC $\rightarrow$ Verify installer opens without SmartScreen warnings.
2. Download macOS `.dmg` on a clean Mac $\rightarrow$ Verify app launches without Gatekeeper rejection.
3. Install Linux `.deb` on Ubuntu 24.04 $\rightarrow$ Verify systemd service registers and starts.
4. Establish remote session between clean nodes $\rightarrow$ Verify streaming and input control.

## 13. Acceptance Criteria
- 0 unaddressed high or critical security vulnerabilities.
- Code signed and notarized across Windows and macOS.
- 100% successful installation on target operating systems.

## 14. Definition of Done
OpenDesk v1.0.0 Public Beta is published on GitHub Releases with signed artifacts, documentation, and Docker images.

## 15. Known Risks
- SmartScreen reputation delays on newly issued code signing certificates.
  - *Mitigation*: Utilize EV code signing certificates or Microsoft Azure Trusted Signing to establish baseline reputation.

## 16. Estimated Complexity
High (Release engineering, compliance, code signing HSMs).

## 17. Next-Phase Dependencies
Transitions project into continuous maintenance and feature backlog execution.
