# Repository Guidelines

## Project Structure & Module Organization
- Repo root: `Cargo.toml` defines the Cargo workspace (resolver = "2").
- `crates/domain` — Core domain model and logic. Dependencies: none.
- `crates/ports` — Hexagonal ports (traits) for infrastructure. Depends on: domain.
- `crates/application` — Use-case orchestration. Depends on: domain, ports, observability.
- `crates/adapters` — Inbound/outbound adapters. Depends on: domain, ports, observability, contracts-kafka.
- `crates/contracts-kafka` — Messaging contracts. Dependencies: none.
- `crates/observability` — Telemetry/tracing facade. Dependencies: none.
- `crates/bootstrap` — Binaries and composition root. Depends on: application, adapters, observability, contracts-kafka.
- `crates/tests` — Integration tests crate. Depends on: application, adapters, domain, ports, observability, contracts-kafka.

Note: File-level layouts evolve over time. This guide enforces crate-level boundaries and dependency rules; file listings are illustrative and not prescriptive.

- Layering rules (crate boundaries):
  - domain → none
  - ports → domain
  - application → domain, ports, observability
  - adapters → domain, ports, observability, contracts-kafka
  - bootstrap → application, adapters, observability, contracts-kafka
  - tests → application, adapters, domain, ports, observability, contracts-kafka
  - No reverse or out-of-graph imports.

## Build, Test, and Development Commands
- `cargo build` — compile in debug mode.
- `cargo build --release` — optimized build.
- `cargo test` — run unit + integration tests.
- `cargo run -- <args>` — run the default binary with args.
- `cargo clippy --all-targets -- -D warnings` — lint; treat warnings as errors.
- `cargo fmt --all -- --check` — verify formatting; use `cargo fmt --all` to apply.
- `cargo doc --no-deps --open` — build and open docs.

## Coding Style & Naming Conventions
- Use rustfmt defaults (4 spaces; no tabs). Always format before pushing.
- Follow Clippy; code must be `clippy`-clean.
- Naming: `snake_case` for functions/vars/modules, `UpperCamelCase` for types/traits, `SCREAMING_SNAKE_CASE` for consts/statics.
- Public items require `///` docs; prefer `#[must_use]` where appropriate.
- Keep modules small; prefer `mod.rs`-less layout (`foo/mod.rs` → `foo.rs` when simple).

## Testing Guidelines
- Unit tests co-located with code in `#[cfg(test)] mod tests { ... }`.
- Integration tests live in `tests/` and are black-box style.
- Name tests descriptively, e.g., `parses_invalid_header_returns_error`.
- Aim for meaningful coverage on new/changed code; include edge cases and error paths.

## Commit & Pull Request Guidelines
- Use Conventional Commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`.
- Keep commits focused and atomic; include rationale in the body if non-obvious.
- PRs must pass CI, link related issues, and include: summary, scope, screenshots or logs for behavior changes, and test notes.

## Security & Configuration Tips
- Never commit secrets; prefer env vars or `.env` (git-ignored).
- Validate all external input; avoid `unwrap()` in non-test code.

## Agent-Specific Notes
- Obey this file’s guidance. Make minimal, targeted changes.
- Run `cargo fmt` and `cargo clippy` before proposing patches.
- Update docs/tests when altering public APIs.
