# guild-deps

**Difficulty: Intermediate**

Audit a project's dependencies for freshness, security advisories, and license compatibility.

## Scope

- Parse `Cargo.toml` and `Cargo.lock` to list dependencies
- Check crates.io for newer versions
- Query RustSec advisory database for known vulnerabilities
- Check license compatibility against an allowlist
- Generate a report with outdated/vulnerable/incompatible deps

## Key Skills

- TOML parsing (Cargo.toml, Cargo.lock)
- HTTP requests to crates.io API
- Advisory database querying
- Report generation and formatting
