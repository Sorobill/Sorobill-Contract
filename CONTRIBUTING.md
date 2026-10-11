# Contributing to Sorobill Smart Contracts

Thank you for contributing to Sorobill! Please follow this pre-PR verification checklist before submitting changes.

## ðŸ“‹ Pre-PR Verification Gate
Before opening a Pull Request, ensure every item below passes cleanly:

1. **Local Test Suite**: Run cargo test and ensure 100% of unit and integration tests pass.
2. **Static Analysis & Compilation**: Run cargo check and cargo clippy --all-targets with zero warnings or errors.
3. **Format Standard**: Run cargo fmt -- --check.
4. **Network Guard**: Live contract invocations and tests must target **Testnet only**. Never commit Mainnet keys or configurations.
5. **Documentation Alignment**: If contract storage keys, public entrypoints, or error codes are modified, update docs/ and DEPLOYMENTS.md accordingly.
6. **PR Reference**: Ensure your Pull Request links to an approved tracking issue using Closes #<issue_number>.