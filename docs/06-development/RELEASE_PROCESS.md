# OpenDesk — Release Management & Distribution Process

## 1. Semantic Versioning & Release Cadence

OpenDesk strictly adheres to **Semantic Versioning 2.0.0** (`vMAJOR.MINOR.PATCH`):
- **MAJOR**: Incompatible API or protocol changes breaking older clients.
- **MINOR**: Backward-compatible new features (e.g. adding AV1 hardware codec support).
- **PATCH**: Backward-compatible bug fixes and security hardening.

### Release Cadence:
- **Nightly Releases**: Automated builds from `main` for early adopters.
- **Stable Minor Releases**: Scheduled every 4 weeks.
- **Security Hotfixes**: Deployed out-of-band within 24 hours of vulnerability mitigation.

---

## 2. Release Execution Runbook

```mermaid
sequenceDiagram
    autonumber
    actor Maintainer as Core Maintainer
    participant Git as GitHub Repo
    participant CI as GitHub Actions Release Pipeline
    participant Sign as Signing HSM / Notary
    participant DockerHub as Container Registry
    participant Releases as GitHub Releases Page

    Maintainer->>Git: Push Tag "v1.0.0"
    Git->>CI: Trigger cd-release.yml
    
    par Matrix Build
        CI->>Sign: Build & Sign Windows Installer (.msi)
        CI->>Sign: Build & Notarize macOS App (.dmg)
        CI->>Sign: Build & Sign Linux Package (.deb, .AppImage)
        CI->>DockerHub: Push Multi-Arch Docker Image (linux/amd64, linux/arm64)
    end

    CI->>CI: Compute SHA-256 Checksums for all artifacts
    CI->>CI: Generate SLSA Level 3 Provenance Attestation
    CI->>Releases: Publish GitHub Release v1.0.0 with Assets & Release Notes
```

---

## 3. Post-Release Verification

1. Download published `.msi` and `.dmg` on clean test machines (non-developer VMs).
2. Verify SmartScreen / Gatekeeper passes cleanly without untrusted publisher warnings.
3. Pull `opendesk/server:1.0.0` container and execute `docker compose up -d` against test database.
4. Verify client auto-updater detects the new version and updates seamlessly.
