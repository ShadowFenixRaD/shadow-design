---
trigger: always_on
description: Binding design-system contract for any UI, SCSS, theme or showcase change in Shadow Design.
---

## Shadow Design: design-system contract (binding)

Before editing anything in `crate/scss/`, `crate/src/` or `showcase/src/`:

1. Read every *Accepted* ADR in `docs/adr/` and all of `docs/design/` (language.md, tokens.md, component-checklist.md).
2. Existing code is **not** precedent. If nearby code contradicts an ADR, follow the ADR and never copy the deviation.
3. Every component follows the Lit Glass recipes in `docs/design/language.md`. Values come only from `var(--sd-*)` tokens, the mixins in `_mixins.scss` and `z()`. Library Rust emits only `sd-*` classes. UI text comes from `Locale`.
4. If the task needs something no ADR allows (a new token, a new visual treatment, an exception), stop and write a *Proposed* ADR instead of improvising.
5. Run `python scripts/lint_design.py` (inside `nix develop`) and fix every violation before you finish. The pre-commit hook runs it too.
