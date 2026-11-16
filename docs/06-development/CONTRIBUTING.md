# Contributing to OpenDesk

Thank you for your interest in contributing to **OpenDesk**! We are committed to building an open, auditable, high-performance remote desktop platform for the global open-source community.

---

## 1. Code of Conduct

OpenDesk has adopted the **Contributor Covenant (version 2.1)**. By participating in this project, you agree to uphold a welcoming, respectful, and harassment-free environment for all contributors.

---

## 2. Developer Certificate of Origin (DCO)

To ensure clear open-source provenance, all commits must include a `Signed-off-by` line certifying that you wrote or have the right to submit the code under the project's open-source license:

```bash
git commit -s -m "feat(crypto): implement X25519 ephemeral key derivation"
```

---

## 3. Pull Request Guidelines

1. **Check the Issue Tracker**: Before starting work on major features, ensure an issue exists and is marked as approved by a maintainer.
2. **Branch from `main`**: Keep feature branches short-lived and rebased on `main`.
3. **Follow Coding Standards**: Run formatting and linting tools before submitting:
   - Rust: `cargo fmt --all && cargo clippy --workspace --all-targets`
   - C#: `dotnet format server/OpenDesk.sln`
   - Frontend: `pnpm lint && pnpm test`
4. **Include Automated Tests**: PRs introducing new functionality or bug fixes must include corresponding unit or integration tests.
5. **Update Documentation**: Any changes impacting APIs, configuration parameters, or architecture must include updates to the relevant documents under `/docs`.
