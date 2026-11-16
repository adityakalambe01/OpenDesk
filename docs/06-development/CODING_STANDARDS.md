# OpenDesk — Engineering Standards & Style Guidelines

## 1. Rust Engineering Standards (`core/`)

- **Formatting & Linting**: Strictly enforced via `cargo fmt --check` and `cargo clippy -- -D warnings`.
- **Memory Safety & Unsafe Code**:
  - `#![forbid(unsafe_code)]` must be present in all crates except `opendesk-capture` and `opendesk-input`.
  - Every `unsafe` block must be preceded by a `// SAFETY:` comment explicitly detailing the preconditions, invariants, and pointer validity guarantees.
- **Error Handling**:
  - Library crates use strongly-typed enums via `thiserror`.
  - Application entry points (`opendesk-cli`, `opendesk-daemon`) use `anyhow::Result` for context wrapping.
  - Panics (`unwrap()`, `expect()`) are forbidden in network packet parsing or runtime streaming loops.

---

## 2. C# / .NET 10 Coding Standards (`server/`)

- **Language Baseline**: C# 14 with `<Nullable>enable</Nullable>` and `<TreatWarningsAsErrors>true</TreatWarningsAsErrors>`.
- **Architectural Rules**:
  - Controllers must remain thin, delegating command and query dispatching to MediatR handlers.
  - All asynchronous methods must accept a `CancellationToken` and propagate it to underlying I/O or EF Core calls.
  - Use file-scoped namespaces (`namespace OpenDesk.Api;`) and primary constructors where appropriate.
  - Entities must encapsulate domain invariants through private setters and factory methods.

---

## 3. TypeScript & React 19 Standards (`clients/desktop/src/`)

- **Strict Typing**: TypeScript `strict: true` with `noImplicitAny: true`. Never use `any`; use `unknown` with type guards.
- **Functional Architecture**: React functional components with explicit hook dependencies; avoid complex `useEffect` chains in favor of event-driven Zustand actions.
- **Styling**: TailwindCSS utility classes; no inline CSS styles or un-scoped stylesheets.

---

## 4. Git Commit Message Standards (Conventional Commits)

All commits must follow the **Conventional Commits 1.0.0** specification:

$$\text{type}(\text{scope}):\ \text{short description in imperative present tense}$$

### Permitted Types:
- `feat`: A new user-facing feature.
- `fix`: A bug fix.
- `perf`: A code change that improves performance (e.g., reduces capture latency).
- `refactor`: A code change that neither fixes a bug nor adds a feature.
- `docs`: Documentation changes only.
- `test`: Adding missing tests or correcting existing tests.
- `chore`: Build scripts, dependencies, or CI changes.

*Example*: `feat(capture-win): implement Windows Graphics Capture yellow border toggle`
