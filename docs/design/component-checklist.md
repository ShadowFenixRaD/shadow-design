# Component checklist

Use this when you create or change a library component. A change isn't done until every box applies.

## Before writing

- [ ] Read the accepted ADRs (`docs/adr/`) and `language.md`.
- [ ] Identify the recipe(s) from `language.md` §3: container, floating panel, item, raised control, recessed field, selected, semantic, value fill. If nothing fits, stop and write an ADR.
- [ ] Check whether another library component already does part of the job (Pagination, LoadingOverlay, Avatar, Button, Card…). Reuse it; don't re-implement it.

## SCSS (`crate/scss/components/_<feature>.scss`)

- [ ] One BEM block `.sd-<component>`, everything inside `@layer sd-components`.
- [ ] Every value comes from a `var(--sd-*)` token, a mixin or `z()`. No hex/rgb literals, no `$violet-*`/`$radius-*`, no numeric z-index, no `!important`, no `transition: all`.
- [ ] Shadows only via `@include sd-shadow` / `sd-surface` plus the `--sd-shadow-*` slots.
- [ ] Interactive parts have hover, press (`sd-press` for actions), `sd-focus-ring` and disabled states. Static parts have none of them.
- [ ] Selection uses `sd-selected`. Weight doesn't change.
- [ ] Glow is only where `language.md` §4 allows it.
- [ ] Animations use `sd-animate`/`sd-animate-essential` and keyframes from `_animations.scss`.
- [ ] The partial is `@use`d in `main.scss`.

## Rust (`crate/src/<feature>.rs` or `<feature>/mod.rs`)

- [ ] Emits only `sd-*` classes, with no utility classes. Every emitted class exists in the CSS.
- [ ] Identical output with and without `feature = "theme"`. With the theme on, it applies `ComponentOverride` only.
- [ ] All UI and ARIA text comes from `Locale` (English defaults), overridable by props.
- [ ] IDs and ARIA come from `crate::a11y`. Hooks are called unconditionally.
- [ ] Props are `Clone + PartialEq` and own their data.

## Verify

- [ ] `python scripts/lint_design.py` passes (also run by the pre-commit hook).
- [ ] `cargo check -p shadow-design` passes, and so does `--no-default-features --features <feature>`.
- [ ] `cargo clippy --workspace` adds no new warnings.
- [ ] Showcase page updated (`showcase/src/pages/<feature>.rs`). The showcase uses library components for its own chrome (tabs, segmented pickers, badges), never hand-rolled utility markup.
- [ ] Check it in the browser under Crimson + Sharp + Glow Off as well as the defaults: nothing stays violet and nothing stays round.
- [ ] `CHANGELOG.md` entry; `graphify update .`.
