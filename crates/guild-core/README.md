# guild-core

**Shared library for the Guild CLI toolkit.**

This crate is not a standalone tool — it provides the foundation that all guild tools depend on.

## Modules

- **config** — `GuildConfig` loaded from `~/.guild/config.toml`
- **data** — Shared data model: profile, projects, progress, reviews
- **error** — `GuildError` enum used across all tools
- **output** — Colored terminal output helpers (success, info, warn, error)
- **crosslink** — Thin wrappers for invoking crosslink/chainlink CLI
