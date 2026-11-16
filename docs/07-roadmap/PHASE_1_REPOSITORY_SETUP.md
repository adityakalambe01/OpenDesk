# OpenDesk Phase 1 — Repository & Development Environment Setup

## 1. Phase Objective
Bootstrap the polyglot monorepo structure, initialize the Rust workspace, scaffold the ASP.NET Core 10 backend solution, scaffold the Tauri v2 + React 19 desktop client, initialize the Flutter mobile project, configure Docker Compose local development dependencies, and establish continuous integration workflows.

## 2. Why This Phase Is Necessary
Prior to implementing complex multimedia streaming or input logic, developers must have a clean, reproducible, and automated build environment where every component compiles cleanly, dependencies resolve deterministically, and tests run in automated pipelines.

## 3. Scope
- Initializing top-level Cargo workspace and member crates (`opendesk-proto`, `opendesk-crypto`, etc.).
- Creating the ASP.NET Core 10 Clean Architecture solution (`OpenDesk.Server`, `Domain`, `Application`, `Infrastructure`).
- Scaffolding the Tauri v2 + React 19 + TypeScript + Vite desktop application.
- Initializing the Flutter 3.x mobile project skeleton for Android and iOS.
- Creating local development Docker Compose stacks (`PostgreSQL 17`, `Redis 7.4`).
- Setting up GitHub Actions CI workflows for Rust, .NET, and frontend.
- Adding root repository configuration (`.gitignore`, `.editorconfig`).

## 4. Prerequisites
- Completion and approval of Phase 0.
- Installed toolchains: Rust 1.82+, .NET 10 SDK, Node.js 22 LTS & pnpm, Flutter 3.24+, Docker Desktop.

## 5. Technical Implementation Details
- The Rust workspace defines shared dependencies in the workspace root `Cargo.toml`.
- The ASP.NET Core solution uses Central Package Management (`Directory.Packages.props`) to ensure consistent dependency versions across projects.
- Tauri v2 uses the official scaffolding toolchain configured with React 19 and TailwindCSS.
- Local PostgreSQL and Redis run in isolated Docker containers with persistent data volumes.

## 6. Exact Folder and File Changes Expected
```
/
├── .gitignore
├── .editorconfig
├── Cargo.toml
├── OpenDesk.sln
├── Directory.Packages.props
├── core/crates/opendesk-proto/...
├── server/src/OpenDesk.Api/...
├── clients/desktop/...
├── clients/mobile/...
├── deploy/docker-compose/docker-compose.dev.yml
└── .github/workflows/ci.yml
```

## 7. Individual Development Tasks
- **P1-001**: Configure root `.gitignore`, `.editorconfig`, and monorepo files.
- **P1-002**: Initialize root `Cargo.toml` and scaffold core Rust crates (`opendesk-proto`, `opendesk-crypto`, `opendesk-capture`, `opendesk-encoder`, `opendesk-input`, `opendesk-transport`, `opendesk-engine`, `opendesk-ipc`, `opendesk-daemon`, `opendesk-cli`).
- **P1-003**: Scaffold ASP.NET Core 10 solution and projects (`OpenDesk.Domain`, `Application`, `Infrastructure`, `Signaling`, `Api`).
- **P1-004**: Scaffold Tauri v2 + React 19 desktop client under `clients/desktop`.
- **P1-005**: Scaffold Flutter 3.x mobile client under `clients/mobile`.
- **P1-006**: Configure local development `docker-compose.dev.yml` (Postgres 17 + Redis 7.4).
- **P1-007**: Configure GitHub Actions CI workflow matrix (`.github/workflows/ci.yml`).
- **P1-008**: Verify build and test execution across all projects.

## 8. Dependencies
Phase 0 architectural approval.

## 9. Expected Outputs
A fully buildable monorepo where `cargo build`, `dotnet build`, `pnpm build`, and `docker compose up` execute without errors.

## 10. Unit Testing Requirements
- Baseline unit test in Rust crate verifying workspace compilation.
- Baseline xUnit test in .NET solution verifying dependency injection setup.
- Baseline Vitest test in React desktop client verifying component mounting.

## 11. Integration Testing Requirements
- Verification that Docker Compose boots PostgreSQL and Redis and that ASP.NET Core connects successfully on startup.

## 12. Manual Verification Steps
1. Run `cargo build --workspace` $\rightarrow$ Must exit with status 0.
2. Run `dotnet build OpenDesk.sln` $\rightarrow$ Must exit with status 0.
3. Run `docker compose -f deploy/docker-compose/docker-compose.dev.yml up -d` $\rightarrow$ Verify healthy containers.
4. Run `pnpm --filter opendesk-desktop build` $\rightarrow$ Must compile webview assets successfully.

## 13. Acceptance Criteria
- All application projects build cleanly across target environments.
- Zero compiler warnings with warnings treated as errors.
- CI pipeline triggers on pull requests and reports green status.

## 14. Definition of Done
All projects compile, pass baseline tests, and are tracked in source control with passing CI.

## 15. Known Risks
- Platform C++ build tool prerequisites missing on developer workstations.
  - *Mitigation*: Comprehensive automated setup instructions provided in `DEVELOPMENT_SETUP.md`.

## 16. Estimated Complexity
Medium (Scaffolding and build configuration).

## 17. Next-Phase Dependencies
Unlocks **Phase 2 — Rust Core Engine**.
