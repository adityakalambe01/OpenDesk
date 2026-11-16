# Security Policy

## 1. Reporting a Vulnerability

OpenDesk takes the security of remote connections, credentials, and host operating systems with the utmost seriousness.

**Please do not report security vulnerabilities through public GitHub issues or discussions.**

Instead, please report vulnerabilities directly to our security response team:
- **Email**: `security@opendesk.io`
- **PGP Key Fingerprint**: `4A8E 29B1 C048 9DF1 8234  7E91 289F 104A BC92 18E2`

Please include the following details in your report:
1. Product component and version affected (e.g. `core/crates/opendesk-proto`, v0.1.0).
2. Operating system and platform architecture (e.g., Windows 11 x86_64, macOS Apple Silicon).
3. Detailed steps to reproduce the vulnerability, proof-of-concept (PoC) code, or network packet traces.
4. Potential impact (e.g., unauthorized session takeover, relay decryption, privilege escalation).

---

## 2. Supported Versions

Security updates and critical vulnerability hotfixes are actively provided for:

| Version | Supported | Security Maintenance Status |
|---|:---:|---|
| **v1.0.x** | ✅ Yes | Active Security Maintenance (Public Beta & GA) |
| **< v1.0.0** | ❌ No | Pre-release development snapshots |

---

## 3. Vulnerability Response Timeline

- **Initial Acknowledgment**: Within **24 hours** of receipt.
- **Triage & Reproduction**: Within **72 hours**.
- **Remediation & Patch Release**: Within **14 days** for critical severity issues.
- **Coordinated Public Disclosure**: Within **30 to 90 days**, agreed upon jointly with the reporter.

OpenDesk commits to crediting security researchers who discover and responsibly disclose vulnerabilities in our security advisories and release notes.
