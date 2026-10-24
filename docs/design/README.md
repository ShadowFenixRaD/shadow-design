# Shadow Design: design documentation

Read this folder before any UI work, after the ADRs in [`docs/adr/`](../adr/README.md).

| Document | Read it when |
| :-- | :-- |
| [language.md](language.md) | Always. Lit Glass principles and the **recipe for every kind of element** (container, floating panel, raised control, recessed field, item, selected, focus, disabled, error, backdrop). |
| [tokens.md](tokens.md) | Whenever you write a value. Every `--sd-*` token, its default, and what it's for. |
| [component-checklist.md](component-checklist.md) | When you create or change a component. A step-by-step checklist with a definition of done. |

Code that implements these documents:

- `crate/scss/_tokens.scss`: runtime token defaults (`:root`, layer `sd-base`) and the radius scale classes.
- `crate/scss/_mixins.scss`: `sd-surface`, `sd-surface-interactive`, `sd-shadow`, `sd-lit-fill`, `sd-field`, `sd-field-bare`, `sd-item`, `sd-item-danger`, `sd-track`, `sd-selected`, `sd-semantic`, `sd-focus-ring`, `sd-hover`, `sd-transition`, `sd-press`, `sd-animate`, `sd-animate-essential`, `sd-disabled`, `sd-scrollbar`, `bp-up`. Each has a doc comment with parameters and usage.
- `crate/scss/_animations.scss`: the `sd-*` keyframes (table at the top of the Lit Glass section).
- `crate/scss/_zindex.scss`: the z-index map, read through `z()`.
- `crate/src/theme.rs`: computes accent tokens from one hex and writes them as CSS variables.
- `scripts/lint_design.py`: enforces what a machine can check.
