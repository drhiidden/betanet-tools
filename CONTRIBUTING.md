# Contributing to betanet-tools

Thank you for your interest in contributing. This document explains how to contribute, tests and the project rules.

## Code of conduct
Please follow the `CODE_OF_CONDUCT.md` included in this repo.

## Licensing & DCO
This project is dual-licensed under `MIT OR Apache-2.0`. By contributing you agree that your contributions will be distributed under the same terms.

All commits should include a DCO-style `Signed-off-by: Your Name <your.email@example.com>` line in the commit message or an equivalent statement in the PR.

## How to contribute
- Fork the repo and create a feature branch
- Run tests: `cargo test --all`
- Create small, focused PRs with clear descriptions and link relevant issues

## Code style
- Keep code idiomatic Rust; follow the guidelines in `code_style.md` (TBD)
- Add tests for new functionality and maintain ≥80% coverage for critical crates

## CI and tests
- We run CI on GitHub Actions with matrix OS (ubuntu-latest, windows-latest, macos-latest)
- Selftests that require Chrome for Testing are marked `#[ignore]` and run in CI where available

## Fixtures and PCAPs
- Do not upload PCAPs containing PII or private URLs. Anonymize or sanitize before adding to `fixtures/`.

## Reporting and Issues
- Please use the repository issues for coordination: `https://github.com/drhiidden/betanet-tools/issues`

Thanks — we appreciate your help!
