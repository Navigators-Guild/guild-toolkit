# guild-roast

**Difficulty: Advanced**

Point it at a codebase and get an adversarial review checklist. Automated first-pass before human review.

## Scope

- Analyze project structure (missing README, no tests, no CI config)
- Check code quality signals (clippy warnings, TODO/FIXME counts, dead code)
- Validate against guild standards (design doc present, crosslink initialized)
- Generate a review checklist with pass/fail/warn items
- Optional: LLM-powered deeper code review via API integration

## Key Skills

- Code analysis and static checks
- Process spawning (running cargo clippy, grep, etc.)
- Checklist generation and scoring
- API integration (for LLM review — stretch goal)
