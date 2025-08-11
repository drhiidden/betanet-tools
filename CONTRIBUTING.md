# Contributing

Thank you for contributing to this project. Please follow these guidelines to make your contributions easier to review and integrate.

## License
This repository is licensed under the GNU Affero General Public License v3 (AGPL-3.0-or-later). By contributing you agree your contributions will be licensed under AGPL-3.0-or-later.

## Branches and PRs
- Use feature branches named `feature/<short-name>` or `fix/<short-desc>`.
- Open a PR with a clear title and description. Use small, focused PRs.
- Mark PR as draft if work in progress.

## Tests and CI
- Run `cargo test --workspace` locally and ensure tests pass.
- Run `cargo clippy --workspace --all-targets -- -D warnings` and fix warnings.
- CI will run `cargo audit` — fix vulnerabilities or justify in the PR.

## Binary auditing
- For binary artifacts, include `cargo metadata` and `cargo tree --all-features` output in the PR as artifacts or paste them in the PR if small.

## Documentation and English requirement
- All public docs, READMEs and doc comments must be in English.

## Code style
- Follow Rust idioms. Keep PRs readable.

## Contact
- Use issues for design discussions and reference relevant docs in `docs/`.
