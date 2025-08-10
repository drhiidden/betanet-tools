# spec-linter (bn-lint) — Specification & Requirements

Purpose
-------
Define the responsibilities, API and acceptance criteria for the `bn-lint` tool: a pluggable validator for Betanet/HTX specifications (CBOR schemas, JA3/JA4 conformance, control-stream validation, config limits).

Scope
-----
- `crates/bn-lint` as a CLI tool that accepts paths and runs validators.
- A rules engine supporting pluggable validators and a minimal set of built-in rules for spec 1.1.

Acceptance Criteria
-------------------
- CLI: `bn-lint validate --spec 1.1 --path <file|dir>` returns structured output (JSON + human) and exit codes: 0 OK, 1 Warnings, 2 Errors.
- At least three initial rules implemented as examples:
  1. CBOR structure conformance (control-stream.schema)
  2. No forbidden fields (e.g., `public-signaling` flag)
  3. Node configuration limits (e.g., `max_inflight` <= N)
- Test vectors: a small set of positive/negative examples for each rule in `crates/bn-lint/tests/fixtures`.
- CI job that executes `cargo test` for bn-lint and runs an example validation.

Rule development
----------------
- Each rule implements a Rust trait `LintRule` that describes: `id()`, `description()`, `run(&self, ctx: &LintContext) -> LintResult`.
- Rules must be unit-tested and have example artifacts in `tests/fixtures`.

Outputs & integrations
---------------------
- Human readable output with file:line (if applicable) and JSON machine-readable mode via `--format json`.
- Exit codes described above for automation.
- Optionally integrate with GH Actions `annotations` for quick feedback in PRs.

Extensibility
-------------
- Provide a simple plugin interface (dynamic or feature-flag compile-time) for adding project-specific rules.
- Document how to add rules in `docs/spec-linter/README.md` (developer guide).

Security
--------
- Linting untrusted files must be done defensively. Avoid executing untrusted code and limit resource usage.

Documentation & examples
------------------------
- Add a `docs/spec-linter/checklist.md` with required test vectors and a sample invocation.
- Provide example rules in `crates/bn-lint/src/rules/` and wire them in `main.rs` under a feature flag.

Roadmap
-------
- Phase 1: core CLI + 3 example rules + fixtures (2–3 weeks)
- Phase 2: integrate into CI and GH annotations (1–2 weeks)
- Phase 3: extend rule set and plugin ecosystem (ongoing)
