# Contributing to Substrata

Thank you for your interest in contributing! Substrata is an open-source project and we welcome contributions of all kinds — bug fixes, new features, documentation improvements, and test coverage.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
  - [Branching](#branching)
  - [Commit Style](#commit-style)
  - [Pull Requests](#pull-requests)
- [Testing](#testing)
- [Code Style](#code-style)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)
- [Security Issues](#security-issues)

---

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating you agree to uphold it.

---

## Getting Started

1. Fork the repository on GitHub.
2. Clone your fork:
   ```bash
   git clone https://github.com/<your-username>/Substrata-Contract.git
   cd Substrata-Contract
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/your-org/Substrata-Contract.git
   ```

---

## Development Setup

### Requirements

- Rust stable (1.74+)
- `wasm32-unknown-unknown` target
- Soroban CLI

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked soroban-cli
```

### Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Test

```bash
cargo test
```

---

## Project Structure

See [README.md](../README.md#project-structure) for the full layout. Key points:

- Business logic lives in `plans.rs`, `subscriptions.rs`, `payments.rs`.
- `lib.rs` is a thin dispatch layer — keep it that way.
- `storage.rs` contains only typed read/write helpers — no logic.
- All new features need tests in `tests.rs`.

---

## Making Changes

### Branching

Branch from `main` using a descriptive name:

```
feat/grace-period
fix/double-charge-edge-case
docs/architecture-update
test/billing-failure-coverage
```

### Commit Style

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add grace period before auto-cancel
fix: prevent double-charge on same ledger timestamp
docs: clarify authorization model in ARCHITECTURE.md
test: add coverage for custom billing interval
refactor: extract token transfer helper
```

### Pull Requests

- Keep PRs focused — one logical change per PR.
- Fill in the PR template (summary, what was tested, any tradeoffs).
- Link any related issues with `Closes #<issue>`.
- All CI checks must pass before merge.
- At least one maintainer review is required.

---

## Testing

Every change to contract logic must include or update tests in `tests.rs`.

- Use `Env::default()` + `mock_all_auths()` for unit-style tests.
- Use `setup_token` helper to mint and approve tokens.
- Advance ledger time with `e.ledger().with_mut(|l| l.timestamp += ...)`.
- Test both the happy path and all relevant error cases.

Run tests with:

```bash
cargo test
```

For verbose output:

```bash
cargo test -- --nocapture
```

---

## Code Style

- Run `cargo fmt` before committing.
- Run `cargo clippy -- -D warnings` and fix all warnings.
- Keep functions small and single-purpose.
- Prefer explicit error returns over panics.
- Document public functions with a one-line doc comment.

---

## Reporting Bugs

Open a [GitHub Issue](https://github.com/your-org/Substrata-Contract/issues) with:

- A clear title
- Steps to reproduce
- Expected vs actual behaviour
- Rust / Soroban CLI version

---

## Suggesting Features

Open a GitHub Issue with the `enhancement` label. Describe:

- The problem you're solving
- Your proposed solution
- Any alternatives you considered

For large changes, open a discussion first before writing code.

---

## Security Issues

**Do not open public issues for security vulnerabilities.**

See [SECURITY.md](SECURITY.md) for the responsible disclosure process.
