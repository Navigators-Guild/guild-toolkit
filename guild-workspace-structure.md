---
title: "Guild CLI Toolkit Workspace Structure"
tags: [design-doc]
sources: []
contributors: [unknown]
created: 2026-03-22
updated: 2026-03-22
---


## Design Specification

### Summary

Establish the Cargo workspace structure for the guild CLI toolkit — a collection of independent binary crates that apprentices build as learning projects. Each tool (`guild-scaffold`, `guild-review`, etc.) is its own crate producing a standalone binary. A shared `guild-core` library crate provides common types, config loading, and output formatting so tools feel cohesive without being coupled.

### Requirements

- REQ-1: Cargo workspace at the repo root with all crates under `crates/`
- REQ-2: `guild-core` shared library crate providing config loading, error types, output formatting, and guild directory discovery (`~/.guild/`)
- REQ-3: Eight binary crates, each producing a standalone executable: `guild-scaffold`, `guild-review`, `guild-progress`, `guild-portfolio`, `guild-search`, `guild-roast`, `guild-deps`, `guild-stats`
- REQ-4: Each binary crate depends on `guild-core` via workspace path dependency
- REQ-5: Each binary crate has a minimal `main.rs` that parses args with `clap`, prints a help message, and exits — enough structure for an apprentice to start implementing without needing to set up boilerplate
- REQ-6: Workspace-level settings: Rust Edition 2024, shared dependency versions via `[workspace.dependencies]`
- REQ-7: A `CONTRIBUTING.md` at the repo root explaining how to add a new tool crate to the workspace
- REQ-8: Each crate has its own `README.md` describing the tool's purpose, intended scope, and difficulty level (beginner/intermediate/advanced)

### Acceptance Criteria

- [ ] AC-1: `cargo build --workspace` succeeds with zero errors and zero warnings
- [ ] AC-2: `cargo test --workspace` succeeds (each crate has at least one placeholder test)
- [ ] AC-3: `cargo clippy --workspace -- -D warnings` passes clean
- [ ] AC-4: Running any tool binary with `--help` prints a clap-generated help message describing the tool's purpose
- [ ] AC-5: `guild-core` exposes at least: a `GuildConfig` struct, an error type, and a config-loading function that reads from `~/.guild/config.toml`
- [ ] AC-6: Each binary crate's `Cargo.toml` uses `guild-core.workspace = true` (workspace path dependency)
- [ ] AC-7: `CONTRIBUTING.md` exists and includes step-by-step instructions for adding a new tool crate
- [ ] AC-8: Each crate directory contains a `README.md` with tool purpose and difficulty rating

### Architecture

### Workspace layout

```
guild-toolkit/
├── Cargo.toml                  # workspace root, [workspace.dependencies]
├── CONTRIBUTING.md             # how to add a new tool
├── LICENSE
├── README.md
└── crates/
    ├── guild-core/             # shared library
    │   ├── Cargo.toml
    │   ├── README.md
    │   └── src/
    │       ├── lib.rs          # re-exports
    │       ├── config.rs       # GuildConfig, load from ~/.guild/config.toml
    │       ├── error.rs        # GuildError enum, thiserror-derived
    │       └── output.rs       # colored/structured terminal output helpers
    ├── guild-scaffold/         # beginner — file I/O and templating
    │   ├── Cargo.toml
    │   ├── README.md
    │   └── src/
    │       └── main.rs
    ├── guild-review/           # intermediate — API integration, state tracking
    │   ├── Cargo.toml
    │   ├── README.md
    │   └── src/
    │       └── main.rs
    ├── guild-progress/         # beginner — file parsing, display
    │   ├── Cargo.toml
    │   ├── README.md
    │   └── src/
    │       └── main.rs
    ├── guild-portfolio/        # intermediate — markdown parsing, HTML generation
    │   ├── Cargo.toml
    │   ├── README.md
    │   └── src/
    │       └── main.rs
    ├── guild-search/           # intermediate — indexing, fuzzy matching
    │   ├── Cargo.toml
    │   ├── README.md
    │   └── src/
    │       └── main.rs
    ├── guild-roast/            # advanced — code analysis, LLM API integration
    │   ├── Cargo.toml
    │   ├── README.md
    │   └── src/
    │       └── main.rs
    ├── guild-deps/             # intermediate — cargo metadata, advisory DB queries
    │   ├── Cargo.toml
    │   ├── README.md
    │   └── src/
    │       └── main.rs
    └── guild-stats/            # intermediate — git log parsing, metrics aggregation
        ├── Cargo.toml
        ├── README.md
        └── src/
            └── main.rs
```

### guild-core design

The shared crate is intentionally thin. It provides:

- **`GuildConfig`**: Deserialized from `~/.guild/config.toml` via `serde` + `toml`. Contains user identity, guild server URL (if any), and per-tool overrides.
- **`GuildError`**: A `thiserror`-derived enum. Each tool can wrap its own errors into this or define tool-specific error types that convert via `From`.
- **`output`**: Helpers for consistent terminal output — colored status lines, progress indicators, table formatting. Wraps a library like `console` or `colored` so tools don't each pick their own.

### Dependency strategy

Shared dependencies are declared in the workspace root `Cargo.toml` under `[workspace.dependencies]` with pinned versions. Tool crates reference them as `dep.workspace = true`. Initial shared dependencies:

- `clap` (derive) — CLI argument parsing for every tool
- `serde` + `toml` — config file handling via guild-core
- `thiserror` — error type derivation
- `anyhow` — application-level error handling in binaries

Tool-specific dependencies (e.g., `syntect` for `guild-roast`, `pulldown-cmark` for `guild-portfolio`) are added to individual crate `Cargo.toml` files as needed by the apprentice implementing them.

### Binary naming

Each binary crate's `Cargo.toml` sets `[[bin]] name = "guild-<tool>"`, matching the crate directory name. Users invoke tools as `guild-scaffold`, `guild-review`, etc. directly. No dispatcher binary.

### Adding a new tool

The `CONTRIBUTING.md` documents this process:
1. Create `crates/guild-<name>/` with `Cargo.toml`, `README.md`, `src/main.rs`
2. Add the crate to the workspace `members` list in the root `Cargo.toml`
3. Add `guild-core` as a dependency
4. Write a clap `Args` struct and `--help` text
5. Submit a design doc via `/design` before implementing

### Out of Scope

- No dispatcher binary — tools are invoked directly, not via `guild <subcommand>`
- No server/backend implementation — this design covers the CLI workspace only
- No CI/CD pipeline setup — that's a separate concern
- No actual tool implementation beyond clap stubs — apprentices own that
- No plugin/extension system — tools are added by creating new crates in the workspace

