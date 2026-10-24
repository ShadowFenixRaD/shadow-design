# ADR-0001: Record decisions in ADRs and bind agents to them

- Status: Accepted
- Date: 2026-10-24

## Context

Shadow Design was built mostly through a long series of AI-agent requests (DS-xx tickets). Each agent saw only the files near its task, not the project's current state or earlier decisions. Every new component was therefore styled by copying its nearest neighbour and adjusting it. A design audit on 2026-10-24 measured the result:

- 1,180 hard-coded colour literals against 302 token references in component SCSS;
- 249 hard-coded violet values across 62 files, so accent presets reached only about a third of the UI;
- 191 distinct `box-shadow` recipes and 29 distinct font sizes;
- three competing radius systems and three styling dialects (BEM SCSS, per-preset classes generated in Rust, Tailwind-style utility strings in Rust);
- tokens, mixins and maps that were defined and never used (`$success`, `$control-h`, `$type-scale`, `focus-ring()`, `elevation()`).

The decisions existed in someone's head and in a few files, but nothing made them visible or binding.

## Decision

1. Architectural and design decisions live in `docs/adr/` as numbered ADRs. The reference material that implements them lives in `docs/design/`: the design language, the token table, recipes and the component checklist.
2. `AGENTS.md` (read by Claude, Gemini/Antigravity, Codex and others through the `CLAUDE.md` and `GEMINI.md` symlinks) and `.agents/rules/design-system.md` (an Antigravity always-on rule) both require agents to read the accepted ADRs and `docs/design/` before any UI work.
3. Rules that a machine can check are enforced by `scripts/lint_design.py`, which runs in the git pre-commit hook (`.githooks/`, enabled automatically by the Nix dev shell). A rule that can't be linted is still binding; reviewers enforce it.
4. Inconsistent existing code is never precedent (see the rules in `docs/adr/README.md`).

## Consequences

- Agents start each task from the documented state rather than inferring it from nearby code.
- Adding a new visual pattern costs one short ADR. That cost is deliberate.
- The lint will reject quick fixes that hard-code values. Contributors who need a value that has no token must add the token (to `_tokens.scss` and `docs/design/tokens.md`) rather than inline it.
