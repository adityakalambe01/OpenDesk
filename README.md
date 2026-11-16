# OpenDesk — Sovereign, High-Performance Open-Source Remote Desktop

<div align="center">

[![License](https://img.shields.io/badge/License-Apache_2.0_OR_MIT-blue.svg)](LICENSE)
[![Architecture](https://img.shields.io/badge/Architecture-Clean_%26_Zero--Trust-green.svg)](docs/01-architecture/SYSTEM_ARCHITECTURE.md)
[![Rust Core](https://img.shields.io/badge/Engine-Rust_1.82+-orange.svg)](docs/02-technical-design/RUST_CORE_ENGINE.md)
[![Backend](https://img.shields.io/badge/Backend-ASP.NET_Core_10-purple.svg)](docs/02-technical-design/BACKEND_ARCHITECTURE.md)
[![Desktop](https://img.shields.io/badge/Desktop-Tauri_v2_%2B_React_19-blue.svg)](docs/02-technical-design/DESKTOP_APPLICATION.md)
[![Mobile](https://img.shields.io/badge/Mobile-Flutter_3.x-cyan.svg)](docs/02-technical-design/MOBILE_APPLICATION.md)
[![Roadmap Status](https://img.shields.io/badge/Status-Phase_0_Complete-brightgreen.svg)](docs/08-tasks/IMPLEMENTATION_TRACKER.md)

**A modern, self-hostable, zero-trust remote desktop platform built with Rust, Tauri, React 19, ASP.NET Core 10, Flutter, and WebRTC.**

[Project Vision](docs/00-project-overview/PROJECT_VISION.md) •
[Documentation Index](#-documentation-directory) •
[Architecture](docs/01-architecture/SYSTEM_ARCHITECTURE.md) •
[Master Roadmap](docs/07-roadmap/MASTER_ROADMAP.md) •
[Task Checklist (MASTER_TODO)](docs/08-tasks/MASTER_TODO.md)

</div>

---

## 1. What is OpenDesk?

**OpenDesk** is a production-grade, open-source alternative to commercial remote control suites like TeamViewer, AnyDesk, and Splashtop, and a modular, clean-architecture successor to first-generation tools like RustDesk and Apache Guacamole.

OpenDesk is engineered to provide sub-30ms glass-to-glass latency, native GPU-accelerated screen capture, precision input injection, and end-to-end encrypted transport across Windows, Linux, macOS, Android, and iOS.

### Two Native Access Modes:
1. **Unattended Administrative Access**: Register your workstation or server fleet, bind devices to your account, and connect securely at any time through an autonomous OS-level background service without requiring someone to approve the connection.
2. **Attended Remote Support**: Generate spontaneous, ephemeral 9-digit session codes and 6-character PINs for on-demand technical assistance, with explicit interactive host consent dialogs and a persistent floating kill switch.

---

## 2. Core Architectural Pillars

- **Zero-Knowledge Transport Plane**: All media and data channels are protected by DTLS 1.3 / SRTP and application-layer ChaCha20-Poly1305 authenticated encryption. Relay servers (coturn) forward encrypted datagrams blindly without decryption keys.
- **Three-Process Host Model**: Background OS services (`Windows Service` / `systemd` / `LaunchDaemon`), user-session capture agents, and the UI shell are isolated processes. The UI crashing never breaks an active session, and UAC elevation / logon screens are handled natively.
- **Hardware-Accelerated Native Pipelines**:
  - **Windows**: Windows Graphics Capture (WGC) & DXGI Desktop Duplication with NVENC / AMF / QuickSync encoding.
  - **macOS**: ScreenCaptureKit framework with Apple Silicon VideoToolbox encoding.
  - **Linux**: Wayland PipeWire DmaBuf zero-copy capture via XDG Desktop Portal (`org.freedesktop.portal.ScreenCast`) with VAAPI encoding, and X11 XShm fallback.
- **100% Dual Deployment Parity**: The multi-tenant Public Cloud offering and the single-tenant Self-Hosted Docker Compose stack use the **exact same backend code, container images, and database schemas**.

---

## 3. Technology Stack

```
+-----------------------------------------------------------------------------------+
| COMPONENT                 | TECHNOLOGY STACK                                      |
+---------------------------+-------------------------------------------------------+
| Core Engine               | Rust (Cargo workspace, libopendesk, windows-rs, tokio)|
| Desktop Client UI         | Tauri v2, React 19, TypeScript, Vite, TailwindCSS     |
| Mobile Client UI          | Flutter 3.x, Dart, flutter_webrtc                     |
| Web Management Dashboard  | React 19, TypeScript, WebCodecs, TanStack Table       |
| Control Plane Backend     | ASP.NET Core 10, C# 14, Entity Framework Core 10      |
| Signaling & Presence      | ASP.NET Core SignalR (WebSockets), Redis 7.4 Cluster  |
| Relational Persistence    | PostgreSQL 17 (Clean Architecture, Code-First)        |
| Transport & Media Relay   | WebRTC (RFC 8825), ICE (RFC 8445), Coturn (RFC 8656)  |
| Ingress & Deployment      | Docker, Docker Compose, Nginx, Let's Encrypt TLS      |
+-----------------------------------------------------------------------------------+
```

---

## 4. Documentation Directory

OpenDesk is developed under a strict documentation-first discipline:

```
docs/
├── 00-project-overview/     # Vision, PRD, Scope, Non-Goals, Glossary
├── 01-architecture/         # System Architecture, ADRs, Components, Data Flow, Lifecycle, Platform Details
├── 02-technical-design/     # Rust Core, Desktop, Mobile, Backend, Database, WebRTC, Capture, Input, Media
├── 03-security/             # Threat Model (STRIDE), Authentication, Device Identity, E2EE, Authorization
├── 04-infrastructure/       # Docker, Turnkey Self-Hosting, Public Cloud, CI/CD, Monitoring, Backups
├── 05-api/                  # REST APIs, Endpoints, SignalR Signaling Events, Versioning Strategy
├── 06-development/          # Dev Setup, Coding Standards, Git Branching, Testing Pyramid, Release Runbooks
├── 07-roadmap/              # Master Roadmap & Phase-by-Phase Execution Specifications (Phases 0 - 10)
├── 08-tasks/                # MASTER_TODO Checklist, Implementation Tracker, Backlog, Known Limitations
└── diagrams/                # Mermaid Architecture, Connection, Auth, and Deployment Flowcharts
```

---

## 5. Phased Implementation Roadmap

Development progresses incrementally following verified acceptance criteria:

- [x] **[Phase 0 — Architecture & Project Foundation](docs/07-roadmap/PHASE_0_FOUNDATION.md)** (Status: **Completed Baseline**)
- [ ] **[Phase 1 — Repository & Development Environment Setup](docs/07-roadmap/PHASE_1_REPOSITORY_SETUP.md)**
- [ ] **[Phase 2 — Rust Core Engine Design & Implementation](docs/07-roadmap/PHASE_2_CORE_ENGINE.md)**
- [ ] **[Phase 3 — First Working Desktop Connection (Windows)](docs/07-roadmap/PHASE_3_DESKTOP_REMOTE_ACCESS.md)**
- [ ] **[Phase 4 — Networking, Signaling & NAT Traversal](docs/07-roadmap/PHASE_4_NETWORKING.md)**
- [ ] **[Phase 5 — Authentication, Identity & Security Hardening](docs/07-roadmap/PHASE_5_SECURITY.md)**
- [ ] **[Phase 6 — Desktop Cross-Platform Support (Linux & macOS)](docs/07-roadmap/PHASE_6_CROSS_PLATFORM.md)**
- [ ] **[Phase 7 — Mobile Applications (Flutter iOS & Android)](docs/07-roadmap/PHASE_7_MOBILE.md)**
- [ ] **[Phase 8 — Backend Control Plane & Web Management Dashboard](docs/07-roadmap/PHASE_8_BACKEND_AND_DASHBOARD.md)**
- [ ] **[Phase 9 — Self-Hosting Infrastructure & Turnkey Deployment](docs/07-roadmap/PHASE_9_SELF_HOSTING.md)**
- [ ] **[Phase 10 — Hardening, Code Signing & Public Beta Release](docs/07-roadmap/PHASE_10_TESTING_AND_RELEASE.md)**

Track active progress in [docs/08-tasks/IMPLEMENTATION_TRACKER.md](docs/08-tasks/IMPLEMENTATION_TRACKER.md) and task details in [docs/08-tasks/MASTER_TODO.md](docs/08-tasks/MASTER_TODO.md).

---

## 6. License & Community Governance

OpenDesk is open-source software dual-licensed under:
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE))
- **MIT License** ([LICENSE-MIT](LICENSE))

Contributions are welcome! Please review our [Contributing Guidelines](docs/06-development/CONTRIBUTING.md) and [Security Policy](SECURITY.md) before submitting code.
