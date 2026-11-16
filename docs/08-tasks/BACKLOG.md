# OpenDesk — Feature Backlog (Post-v1.0 Enhancements)

This backlog documents high-value architectural enhancements and feature proposals evaluated for subsequent development iterations following the v1.0.0 Public Beta release.

---

## 1. Enterprise Identity & Compliance (Backlog)

- **BL-001: Enterprise SAML 2.0 & OIDC Single Sign-On**
  - Integrate ASP.NET Core with OpenID Connect and SAML 2.0 providers (Microsoft Entra ID, Okta, Ping Identity, Keycloak).
- **BL-002: SCIM 2.0 User Provisioning**
  - Automated user lifecycle synchronization from corporate identity directories.
- **BL-003: Tamper-Evident Session Video Recording**
  - Host-side real-time rendering of video/audio into cryptographic WebM containers streamed directly to S3 / MinIO object storage.

---

## 2. Advanced Multimedia & Headless Operations (Backlog)

- **BL-004: Windows Indirect Display Driver (IddCx)**
  - Native virtual display driver allowing OpenDesk to stream headless servers, cloud instances, and VMs without physical monitors or HDMI dummy plugs.
- **BL-005: Linux Virtual DRM/KMS Headless Driver**
  - Headless display server allocation on Wayland/X11 Linux nodes without connected monitors.
- **BL-006: Hardware AV1 Encoding on NVIDIA RTX 4000+ & AMD RX 7000+**
  - Enable next-generation AV1 real-time streaming for $30\%$ bandwidth reduction over H.264 at identical perceptual visual quality.

---

## 3. Collaborative Support & Systems Administration (Backlog)

- **BL-007: Multi-Party Collaborative Remote Support**
  - Allow a primary technician to invite supervisors or specialists into an active session with role-based permissions (view-only vs. interactive control).
- **BL-008: Integrated Out-of-Band Remote Shell Tab**
  - High-speed interactive PTY / PowerShell terminal channel operating directly over SCTP without video streaming overhead.
- **BL-009: Wake-on-LAN (WoL) Subnet Proxying**
  - Allow an online OpenDesk node to broadcast Wake-on-LAN magic packets to wake sleeping machines within the same local broadcast domain.
