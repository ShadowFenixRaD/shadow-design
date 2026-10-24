# ADR-0002: One design language: Lit Glass

- Status: Accepted
- Date: 2026-10-24

## Context

The showcase mixed three looks on the same page. Some components were bare (transparent backgrounds, no border, no hover). Others glowed heavily (16–24px violet halos, `!important` glow on every primary button). Others looked interactive but weren't: static grid cells shared the button recipe, so users couldn't tell what was clickable. The maintainer's direction: keep the theme idea, make glow subtle, remove the minimal look, and use one design philosophy everywhere.

## Decision

Every component follows **Lit Glass**, which is specified in full in [`docs/design/language.md`](../design/language.md). Its non-negotiable rules:

1. **One material.** Every object is black glass lit from above: a surface fill, a 1px hairline border, a specular top edge and a role-based shadow. There are no bare, borderless or transparent containers, and no "minimal" or "plain" variants of containers. The only element that is transparent at rest is the *ghost* button, which is limited to toolbars and icon buttons and still gains the material on hover.
2. **Depth encodes role.** The page is the *void*. Containers are *flat-lit* (surface 1, elevation 1). Floating layers are *raised* (surface 2, elevation 2 or 3). Actions are *raised controls* with a lit gradient fill. Inputs are *recessed wells*.
3. **Affordance is binary.** Interactive elements always have visible hover, press, `:focus-visible` and disabled states. Static elements never change on hover: no pointer cursor, no lift, no gradient, no glow. The lit gradient fill is reserved for buttons, so a static tile can never look like a button.
4. **Accent budget.** The accent marks only the primary action, the selected or active state, focus, value fills (progress, slider, switch on) and links. It is never decoration on static content.
5. **Glow is a subtle signal.** It is allowed only on focus, on selected or active items, on the primary button and on value fills. It always comes from the `--sd-glow-*` tokens, whose strength is set by `GlowMode` (Off 0, Subtle 0.6 default, Expressive 1). There is at most one glowing element per group, and no glow on static containers.
6. **One selected state.** Selection is an accent-soft fill, an accent border (or a 2px accent rail for items in vertical lists), accent-strong text and `--sd-glow-sm`. Font weight never changes on selection.
7. **Semantics use the soft recipe.** Success, warning, danger and info use a soft tint fill, a matching border and strong text. Solid semantic colour is limited to small marks (dots, bars, rails). Semantic colours are fixed and never follow the accent.

## Consequences

- Removed: the Cartoon preset, `ElevationStyle::HardBlock` and `ElevationStyle::SoftElevated` (breaking API change). They contradict rule 1 and only ever applied to five components.
- Danger and success buttons become soft-tinted rather than solid, so a Crimson primary and a danger button can no longer be confused.
- Components that were bare (navbar, tabs list, plain list, ghost-style containers) gain material. Components that glowed by default (buttons, toasts, pagination, segmented control) lose most of their glow.
- Any new component must map to an existing recipe in `docs/design/language.md`. If none fits, it needs a new ADR.
