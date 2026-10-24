# ADR-0004: Component styling contract

- Status: Accepted
- Date: 2026-10-24

## Context

Library components were written in three dialects. Some used BEM partials with tokens. Some used BEM partials with literal values. A later batch (navigation_menu, masonry, transfer_list, media_card, activity_feed, rich_text, facepile, overflow_set, loading_overlay, the DataTable pager) built their styling from Tailwind-style utility strings inside Rust: hard-coded `purple-*`, raw `z-[985]`, utilities that don't exist in the compiled CSS, and Italian UI text. Two signature mixins (`specular-top`, `tactile-press`) emitted whole `box-shadow` and `transition` declarations, which overwrote the component's own values. That silently removed drop shadows from 13 floating surfaces and colour transitions from buttons, tabs and checkboxes.

## Decision

**Where styles live**

1. Every library component is styled by exactly one BEM partial, `crate/scss/components/_<feature_name>.scss`, using the snake_case feature name. Its block is `.sd-<component>`, with elements `__x` and modifiers `--y`. All rules sit inside `@layer sd-components`.
2. **Library Rust code emits only `sd-*` classes.** No utility classes for colour, background, border, radius, shadow, z-index or typography. Layout utilities (`flex`, `gap-*`, `grid`, …) are not allowed in the library either; give the element a BEM class instead. The showcase may use layout utilities, but never colour utilities.
3. Every `sd-*` class emitted from Rust must exist in the compiled CSS. Every rule in a partial must be reachable from some emitted class (no dead blocks).

**What values are allowed in a partial**

4. No colour literals (hex, `rgb()`, `rgba()` or `hsl()` with numeric channels). Use `var(--sd-*)` tokens, or `rgba(var(--sd-*-rgb), <alpha>)` when an alpha is genuinely needed.
5. No static SCSS colour or radius variables (`$violet-*`, `$zinc-*`, `$radius-*`, `$color-*`). Use the runtime `var(--sd-*)` tokens.
6. z-index only through `z('<layer>')` from `_zindex.scss`. Add a layer to the map (and to `AGENTS.md` §4) rather than writing a number. The only exception is `-1`, `0` and `1` for stacking siblings *inside* one component.
7. No `!important`. No direct `box-shadow:`, `transition:`, `animation:`, raw `font-size`, raw `border-radius` or `@media (min|max-width)` declarations: use the mixins and tokens (rules 9–12, `bp-up`). `border-radius: 0` and `50%` are allowed.
8. A line may opt out of rules 4–7 only with a trailing `// sd-lint-allow: <reason>` comment, which reviewers must agree with. Examples: a colour picker's checkerboard, or a gradient spectrum.

**Composition (mixins)**

9. `box-shadow` on a component is produced only by `@include sd-shadow;`. It renders `var(--sd-shadow-specular), var(--sd-shadow-elev), var(--sd-shadow-glow)` and resets those three properties on the element. States and modifiers change shadows by setting the custom properties, never by writing `box-shadow`. Shadows are therefore additive and cannot clobber each other.
10. Transitions only through `@include sd-transition;` (an optional property list). The press effect `@include sd-press;` only adds the `:active` transform and never declares `transition`.
11. Every interactive element includes `@include sd-focus-ring;` (`:focus-visible` outline from `--sd-focus-ring`). Fields additionally use the field-focus recipe.
12. Animations only through `@include sd-animate(...)`, which respects `prefers-reduced-motion`. Essential progress indicators (spinner, indeterminate bar) use `sd-animate-essential`, which slows down rather than stopping.

**Rust side**

13. Every component compiles and renders the same with and without `feature = "theme"` (see ADR-0003). With the theme on, it reads only `ComponentOverride` from `use_theme()`.
14. User-visible and ARIA strings come from `Locale` (`crate/src/locale.rs`, English defaults, overridable per component through props). No string literals of UI text in the library.
15. A component that depends on another feature (table → pagination, loading_overlay) **reuses** that component instead of re-implementing it.

## Consequences

- `scripts/lint_design.py` enforces rules 3–8, plus parts of 2 and 14. The pre-commit hook runs it.
- Writing a component means choosing a recipe from `docs/design/language.md` and expressing it with tokens and mixins; see `docs/design/component-checklist.md`.
- Removed mixins: `specular-top`, `specular-accent`, `glow`, `tactile-press`, `elevation`, `glass-panel`, `hairline-gradient`, `lit-fill`. Each is replaced by `sd-shadow`, `sd-surface`, `sd-press`, `sd-lit-fill` and the shadow tokens.
