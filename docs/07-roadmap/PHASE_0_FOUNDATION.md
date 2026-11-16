# OpenDesk Phase 0 — Architecture & Project Foundation

## 1. Phase Objective
Establish complete, unambiguous architectural blueprints, protocol definitions, threat models, operating-system capability analyses, and an actionable phased implementation roadmap before authoring application code.

## 2. Why This Phase Is Necessary
Remote desktop applications cross the most demanding technical boundaries in modern software: kernel/display server capture, hardware video encoders, real-time UDP congestion control, multi-process privilege escalation (Session 0 / UAC / TCC), and zero-trust end-to-end cryptography. Embarking on implementation without rigorous architectural planning guarantees costly rewrites, protocol incompatibilities, and critical security vulnerabilities.

## 3. Scope
- Researching and documenting platform APIs (DXGI, WGC, ScreenCaptureKit, PipeWire, MediaProjection, ReplayKit).
- Establishing Architectural Decision Records (ADRs).
- Formulating the binary and signaling protocol specifications.
- Threat modeling using STRIDE and defining defense-in-depth security mitigations.
- Designing the database schema and ephemeral Redis architecture.
- Designing Docker, Coturn relay, and cloud deployment topologies.
- Defining coding standards, git workflows, and testing strategies.
- Creating the phased roadmap and master task checklist (`MASTER_TODO.md`).

## 4. Prerequisites
- Principal understanding of systems programming (Rust, C++), .NET enterprise architecture, WebRTC protocols, and native desktop windowing systems.
- Access to official platform developer documentation (Microsoft Learn, Apple Developer, FreeDesktop, Android Developers).

## 5. Technical Implementation Details
All specifications, schemas, interfaces, and architecture diagrams are compiled directly into the `/docs` tree organized by domain. Every architectural claim regarding OS capabilities must be verified against official platform documentation.

## 6. Exact Folder and File Changes Expected
- `docs/00-project-overview/*`
- `docs/01-architecture/*`
- `docs/02-technical-design/*`
- `docs/03-security/*`
- `docs/04-infrastructure/*`
- `docs/05-api/*`
- `docs/06-development/*`
- `docs/07-roadmap/*`
- `docs/08-tasks/*`
- `docs/diagrams/*`
- `README.md`, `LICENSE`, `SECURITY.md`, `CONTRIBUTING.md`

## 7. Individual Development Tasks
- **P0-001**: Analyze product requirements, access modes, and success criteria.
- **P0-002**: Finalize system architecture, process boundaries, and component breakdown.
- **P0-003**: Research platform APIs (Windows, Linux X11/Wayland, macOS, Android, iOS).
- **P0-004**: Formulate Architectural Decision Records (ADR-001 through ADR-010).
- **P0-005**: Author STRIDE threat model and zero-knowledge relay specifications.
- **P0-006**: Define shared binary and signaling wire protocol contracts.
- **P0-007**: Design PostgreSQL 17 relational schema and Redis caching key structures.
- **P0-008**: Formulate Docker Compose and Coturn relay configurations.
- **P0-009**: Establish engineering standards, CI/CD matrices, and test strategies.
- **P0-010**: Compile Master Roadmap and `MASTER_TODO.md`.

## 8. Dependencies
None. This is the foundational planning phase.

## 9. Expected Outputs
A complete, production-grade documentation suite providing end-to-end clarity for all subsequent engineering phases.

## 10. Unit Testing Requirements
Validation of Markdown formatting, link integrity, and Mermaid diagram syntax.

## 11. Integration Testing Requirements
Verification that architectural components (e.g. coturn port ranges, WebRTC DTLS parameters, EF Core schemas) are mutually compatible without conflict.

## 12. Manual Verification Steps
1. Execute repository structure validation scripts.
2. Review all Mermaid diagrams for rendering correctness.
3. Validate that every planned task in `MASTER_TODO.md` maps directly to concrete technical specifications.

## 13. Acceptance Criteria
- 100% of required documentation files are authored with actionable, in-depth technical specifications.
- No placeholder text, empty stubs, or unverified claims.
- Platform-specific limitations for iOS, Android, macOS, Linux, and Windows are clearly documented.

## 14. Definition of Done
The master project documentation suite is fully written, committed to the repository, and approved by the lead architect.

## 15. Known Risks
- *Risk*: Platform API deprecations (e.g. legacy ScreenCapture APIs on macOS or X11 phaseout on Linux).
  - *Mitigation*: Architecture prioritizes modern native APIs (ScreenCaptureKit, PipeWire) with legacy fallbacks.

## 16. Estimated Complexity
High (Architectural systems design and multi-platform research).

## 17. Next-Phase Dependencies
Unlocks **Phase 1 — Repository & Development Environment Setup**.
