# Architecture Decision Records

ADRs are **binding**. They record how this project does things, so that every contributor (human or AI agent) works from the same rules instead of from whatever the surrounding code happens to do.

| ADR | Title | Status |
| :-- | :---- | :----- |
| [0001](0001-record-decisions-and-bind-agents.md) | Record decisions in ADRs and bind agents to them | Accepted |
| [0002](0002-lit-glass-design-language.md) | One design language: Lit Glass | Accepted |
| [0003](0003-runtime-tokens-and-theming.md) | Runtime CSS tokens are the only theming mechanism | Accepted |
| [0004](0004-component-styling-contract.md) | Component styling contract | Accepted |

## Rules

1. **Read before you write.** Before changing anything under `crate/scss/`, `crate/src/` (components or theme) or `showcase/src/`, read every ADR with status *Accepted* and [`docs/design/`](../design/README.md).
2. **Existing code is not precedent.** If the code you are looking at contradicts an ADR, the ADR wins. Fix the code (or leave it alone), but never copy the deviation.
3. **No silent new patterns.** If a task needs something no ADR allows (a new token, a new visual treatment, a new styling mechanism, an exception to a rule), stop and write a new ADR first, with status *Proposed*. Implement it only after the maintainer accepts it.
4. **Changing a decision** means writing a new ADR that *supersedes* the old one. Mark the old one `Superseded by ADR-XXXX` and never delete it.
5. **The lint is the floor, not the ceiling.** `python scripts/lint_design.py` must pass before a commit; the pre-commit hook runs it. Passing the lint does not make a change compliant if it breaks a rule the lint can't see.

## Template

```markdown
# ADR-XXXX: <decision as a short imperative sentence>

- Status: Proposed | Accepted | Superseded by ADR-YYYY
- Date: YYYY-MM-DD

## Context
What problem forced the decision. Facts, numbers, file references.

## Decision
What we do from now on. Concrete enough that a reviewer can say "this diff violates it".

## Consequences
What gets easier, what gets harder, what was removed, and what is deferred.
```
