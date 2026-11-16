# OpenDesk — Future Enhancements & Post-v1.0 Vision

This document tracks strategic architectural capabilities planned for post-v1.0 releases:

---

## 1. Enterprise Identity & SSO Integration (v1.1)
- **SAML 2.0 & OIDC**: Native integration with enterprise identity providers including Okta, Microsoft Entra ID (Azure AD), Google Workspace, and Keycloak.
- **SCIM 2.0 Automated User Provisioning**: Automated provisioning and de-provisioning of technician accounts based on Active Directory group memberships.

---

## 2. Multi-Party Collaborative Remote Sessions (v1.2)
- **Collaborative Tech Support**: Ability for a primary technician to invite a senior engineer or supervisor into an active session.
- **Role Assignment**: Secondary technician can be granted View-Only observation mode or collaborative full control with cursor ownership toggling.

---

## 3. Cryptographically Signed Session Recording (v1.3)
- **Audit Video Recording**: Real-time recording of session video, audio, and keystroke metadata into encrypted, tamper-evident WebM / MP4 video containers.
- **Compliance Storage**: Direct streaming of recording archives to S3-compatible cloud storage (AWS S3, Cloudflare R2, MinIO) for regulatory compliance (HIPAA, PCI-DSS, SOC 2).

---

## 4. Virtual Display Driver for Headless Servers (v1.4)
- **Windows Indirect Display Driver (IddCx)**: A native virtual monitor driver allowing OpenDesk to stream headless servers or cloud VMs that lack physical monitors or HDMI dummy plugs, supporting custom resolutions up to 4K.
- **Linux Virtual DRM/KMS Driver**: Virtual display allocation under Wayland and X11 for headless data center servers.

---

## 5. Remote Terminal & Out-of-Band Power Management (v1.5)
- **Integrated Remote Shell**: Direct lightweight PTY/SSH and PowerShell terminal tab operating over an SCTP data channel without requiring a heavy video capture stream.
- **Wake-on-LAN (WoL)**: Ability to broadcast magic packets from neighboring online OpenDesk nodes within the same local subnet to wake sleeping computers remotely.
- **Intel vPro / AMT Integration**: Hardware-level out-of-band power cycling and BIOS-level remote KVM.
