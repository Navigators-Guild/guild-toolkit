# Guild Toolkit

A collection of CLI tools built by and for the Navigators Guild. Each tool is its own Rust crate — pick one, own it end to end, and ship it.

## What is this?

The guild toolkit is a Cargo workspace containing independent command-line tools that guild members use day to day. Think of it like how `git` works: `git-status`, `git-commit`, `git-log` are all separate programs that share conventions. Same pattern here — `guild-scaffold`, `guild-review`, `guild-roast`, etc.

Each tool is a standalone project that you can design, build, test, and submit for review. Some are simple, some are complex. You pick your challenge.

## The Tools

| Tool | What it does | Difficulty |
|------|-------------|------------|
| `guild-scaffold` | Set up a new project with the right structure, templates, and config | Beginner |
| `guild-progress` | Track where you are in the curriculum and what's next | Beginner |
| `guild-review` | Submit projects for adversarial review and track feedback | Intermediate |
| `guild-portfolio` | Generate a portfolio site from your project repos | Intermediate |
| `guild-search` | Search the guild knowledge base and curriculum docs | Intermediate |
| `guild-deps` | Audit dependencies for freshness, security, and license issues | Intermediate |
| `guild-stats` | Project health dashboard — commits, tests, issue velocity | Intermediate |
| `guild-roast` | Point it at a codebase and get an adversarial review checklist | Advanced |

All tools depend on `guild-core`, a shared library that provides config loading, a common data model, error types, output formatting, and crosslink integration.

## Getting Started

### Prerequisites

- **Rust 1.85+** — install via [rustup](https://rustup.rs/)
- **Git** — you already have this
- **crosslink** — the guild's issue tracker and coordination tool (ask your mentor if you don't have it set up)

### Clone and build

```bash
git clone https://github.com/Navigators-Guild/guild-toolkit.git
cd guild-toolkit
cargo build --workspace
```

If that succeeds with no errors, you're ready.

### Verify it works

Every tool responds to `--help`:

```bash
cargo run --bin guild-scaffold -- --help
cargo run --bin guild-roast -- --help
```

### Run the test suite

```bash
cargo test --workspace
```

### Run the linter

```bash
cargo clippy --workspace -- -D warnings
```

Both of these must pass clean before you submit any work.

## How to Contribute

### 1. Pick an issue

Browse the [issue tracker](https://github.com/Navigators-Guild/guild-toolkit/issues). Issues are labeled by crate and difficulty:

- **`good first issue`** — start here if this is your first contribution
- **`beginner`** — straightforward tasks, mostly file I/O, parsing, and CLI wiring
- **`intermediate`** — requires understanding of the data model, process spawning, or external integrations
- **`advanced`** — code analysis, JSON parsing of tool output, API integration

When you find one you want, comment on it to claim it.

### 2. Create a branch

Always branch from `develop`, not `main`:

```bash
git checkout develop
git pull origin develop
git checkout -b your-name/issue-description
```

For example: `alice/scaffold-gitignore-template` or `bob/progress-list-subcommand`.

### 3. Do the work

- Read the issue description carefully — it tells you which files to modify
- Read the existing code before changing it
- Run `cargo clippy --workspace -- -D warnings` and `cargo test --workspace` before submitting
- Keep your changes focused on the issue. Don't refactor unrelated code.

### 4. Open a pull request

Push your branch and open a PR **targeting `develop`**, not `main`:

```bash
git push -u origin your-name/issue-description
```

Then open a PR on GitHub. In your PR description:
- Reference the issue number (e.g., "Closes #42")
- Describe what you changed and why
- Confirm that tests and clippy pass

### 5. Review and merge

Your PR will be reviewed by other guild members. Expect feedback — that's the point. Address review comments, push updates, and once approved, a journeyman or master will merge it into `develop`.

Periodically, `develop` gets merged into `main` by guild leadership after integration testing.

## Branching Model

```
main        ← stable, tested releases (merged by journeymen/masters)
  └── develop   ← integration branch (PRs from apprentices land here)
        └── your-name/feature  ← your working branch
```

- **`main`** — always works. Only updated by journeymen and masters after review.
- **`develop`** — the active development branch. All apprentice PRs target this branch.
- **Feature branches** — your working branches. One branch per issue, branched from `develop`.

Never push directly to `main` or `develop`. Always use pull requests.

## Project Structure

```
guild-toolkit/
├── Cargo.toml              # workspace root
├── crates/
│   ├── guild-core/         # shared library (config, data model, errors, output)
│   ├── guild-scaffold/     # project scaffolding
│   ├── guild-review/       # review submission
│   ├── guild-progress/     # curriculum tracking
│   ├── guild-portfolio/    # portfolio site generator
│   ├── guild-search/       # knowledge base search
│   ├── guild-roast/        # adversarial code review
│   ├── guild-deps/         # dependency auditing
│   └── guild-stats/        # project health metrics
└── .design/                # design documents
```

Each crate has its own `README.md` with details about scope and what skills you'll practice.

## Need Help?

- Check the crate's `README.md` for scope and context
- Search the knowledge base: `crosslink knowledge search "your question"`
- Ask your mentor or post in the guild channel
- Read the design doc: `.design/guild-workspace-structure.md`

## License

MIT
