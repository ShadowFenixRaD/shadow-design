# Lit Glass: the Shadow Design language

**One sentence:** every element is black glass lit from above, with depth that tells you what it is and a violet (accent) pulse that appears only where something is active.

The decision is recorded in ADR-0002. This document is the operational spec. If a component doesn't map to one of the recipes below, write an ADR before you build it.

---

## 1. Principles

1. **One material.** Fill, hairline border, specular top edge and shadow. No bare containers.
2. **Depth = role.** The void is the page. Flat-lit surfaces are containers. Raised surfaces are floating layers. Lit controls are actions, and recessed wells are inputs.
3. **Binary affordance.** If it reacts to a pointer, it has hover, press, focus and disabled states. If it doesn't, it never reacts at all. Only buttons get the lit gradient fill.
4. **Accent budget.** Accent colour appears only on the primary action, selection, focus, value fills and links.
5. **Glow is a whisper.** Use only `--sd-glow-*` tokens, only on focus, selected, primary or value fills, and on at most one element per group.
6. **Semantics are soft.** Tint fill, border and strong text. Solid semantic colour is for small marks only.
7. **Text has three levels:** `--sd-text`, `--sd-text-muted`, `--sd-text-subtle`. Nothing dimmer, at any size.
8. **Motion is tactile and short.** Fast transitions, a press scale on actions, and a pop-in for floating layers. All of it respects reduced motion.

---

## 2. Depth ladder

| Level | Role | Fill | Border | Shadow (`sd-shadow` props) | Radius |
| :-- | :-- | :-- | :-- | :-- | :-- |
| Void | Page background | `--sd-void` | none | none | none |
| Well | Fields that receive input | `--sd-well` | `--sd-border-strong` | specular: `--sd-inset-well` | `--sd-radius-md` |
| Flat-lit | Containers: card, table, panel, list, accordion, alert, tree, code block, chart | `--sd-surface-1` | `--sd-border` | specular `--sd-specular`, elev `--sd-elev-1` | `--sd-radius-lg` |
| Raised | Floating: menu, popover, hover card, tooltip, toast, command palette | `--sd-surface-2` | `--sd-border-strong` | specular `--sd-specular`, elev `--sd-elev-2` | `--sd-radius-lg` (tooltip: `--sd-radius-md`) |
| Raised-max | Modal, drawer, alert dialog | `--sd-surface-2` | `--sd-border-strong` | specular `--sd-specular`, elev `--sd-elev-3` | `--sd-radius-xl` (drawer: edge-attached, radius only on the free side) |
| Lit control | Buttons and other actions | `sd-lit-fill` gradient | variant | specular `--sd-specular-strong`, plus bottom shade | `--sd-radius-md` |

Nested radius: an element inside a padded container uses **one radius step smaller** than the container. For example, menu items (`sm`) inside a menu (`lg`).

Pills (`--sd-radius-full`) are a **shape that carries meaning**: badge, chip, switch, avatar, slider thumb, status dot. They stay pills under every radius scale.

---

## 3. Recipes

Each recipe has a mixin in `_mixins.scss`. Use the mixin instead of re-typing the values.

### 3.1 Container: `@include sd-surface(1)`
Static. Its colours never change on hover. If the whole container is clickable (interactive card), add the **interactive container** modifier: `cursor: pointer`, on hover `border-color: var(--sd-border-hover)` and `--sd-shadow-elev: var(--sd-elev-2)`, plus press and focus ring. Even then there's no gradient and no glow unless it is *selected*.

Headers and footers inside a container: `--sd-tint-1` background and `--sd-border-subtle` divider.

### 3.2 Floating panel: `@include sd-surface(2)` or `sd-surface(3)`
Enter with `@include sd-animate(sd-pop-in, var(--sd-dur))`. Content padding is `var(--sd-space-2)` for menus and `var(--sd-space-4)` for popovers and dialogs. A scrollable body uses `@include sd-scrollbar`.

### 3.3 Backdrop
`background: var(--sd-scrim)`, `backdrop-filter: blur(var(--sd-backdrop-blur))` (plus the `-webkit-` prefix), entering with `sd-animate(sd-fade-in, var(--sd-dur))`. It is the same for modal, drawer, alert dialog and command palette. A backdrop is `position: fixed; inset: 0` and must not inherit margins from layout parents.

### 3.4 Item (menu item, list option, tab, segmented item, pagination page, tree node, nav link): `@include sd-item`
- Height `--sd-item-h` (`-sm` / `-lg` for sizes), padding-inline `var(--sd-space-3)`, radius `--sd-radius-sm`, font `--sd-text-sm` at weight 500, colour `--sd-text-muted`.
- Hover: background `--sd-tint-1`, colour `--sd-text`.
- Pressed: background `--sd-tint-2`.
- Focus: `sd-focus-ring` (inset offset `-2px` inside menus).
- Selected: `@include sd-selected` (see 3.7).
- Disabled: see 3.9.
- Danger item: colour `--sd-danger-strong`, hover background `--sd-danger-soft`.

A group of items (tabs list, segmented control, pagination, menubar) sits in a **track**: background `--sd-well`, border `--sd-border`, radius `--sd-radius-md`, padding `var(--sd-space-1)`. Never leave a group of items bare on the page.

### 3.5 Raised control (button): `@include sd-lit-fill(<variant>)` + `sd-press` + `sd-focus-ring`
Heights come from `--sd-control-{xs..xl}`. Only `md` is the default. Text is weight 500.

| Variant | Rest | Hover | Notes |
| :-- | :-- | :-- | :-- |
| primary | gradient `--sd-accent` → `--sd-accent-deep`, text `--sd-accent-fg`, border `--sd-accent-border`, glow `--sd-glow-sm` | gradient starts at `--sd-accent-strong`, glow `--sd-glow-md` | The only element with a solid accent fill. Use one per view. |
| secondary | gradient `--sd-surface-3` → `--sd-surface-2`, border `--sd-border-strong`, text `--sd-text` | border `--sd-border-hover` | The default for most actions. |
| outline | `--sd-surface-1`, border `--sd-accent-border`, text `--sd-accent-strong` | background `--sd-accent-soft` | |
| ghost | transparent, no border, text `--sd-text-muted` | `--sd-tint-1` fill, `--sd-border` border, text `--sd-text` | Toolbars and icon buttons only. |
| danger / success / warning | `--sd-<sem>-soft` fill, `--sd-<sem>-border`, text `--sd-<sem>-strong` | soft fill doubled (`--sd-<sem>-soft-hover`) | Never a solid fill, so it can't be confused with a red or green primary. |

Disabled buttons have no glow and no hover change.

### 3.6 Recessed field (input, select trigger, textarea, number, date, pin cell, color field): `@include sd-field`
- Background `--sd-well`, border `--sd-border-strong`, specular property `--sd-inset-well`, radius `--sd-radius-md`, height `--sd-control-md`, text `--sd-text`, placeholder `--sd-text-subtle`.
- Hover: `border-color: var(--sd-border-hover)`.
- Focus (`:focus-within` on the wrapper, or `:focus` on the bare field): `border-color: var(--sd-accent)` and `--sd-shadow-glow: var(--sd-glow-sm)`. The outline ring is not used on fields; the accent border is the focus signal.
- Error: `border-color: var(--sd-danger-border)`; on focus, `border-color: var(--sd-danger)` and a danger glow (`--sd-glow-danger`).
- Label: `--sd-text-sm`, weight 500, `--sd-text`. Description and help text: `--sd-text-xs`, `--sd-text-subtle`. Error text: `--sd-text-xs`, `--sd-danger-strong`.

Toggles (checkbox, radio, switch) are fields too. At rest: well fill and strong border. Checked: `--sd-accent` fill, `--sd-accent-fg` mark, `--sd-glow-sm`. Labels follow the field label spec, and all sizes share the same label alignment.

### 3.7 Selected / active: `@include sd-selected` (or `sd-selected(rail)` for vertical lists)
`background: var(--sd-accent-soft)`, `border-color: var(--sd-accent-border)` (rail variant: a 2px `--sd-accent` inset rail on the leading edge instead), `color: var(--sd-accent-strong)`, `--sd-shadow-glow: var(--sd-glow-sm)`. **Weight unchanged.** Exactly one item per group is selected.

Applies to: tabs, segmented control, pagination current page, nav link, sidebar link, bottom navigation, menubar open trigger, navigation menu active trigger, select or listbox selected option, tree node, table selected row (row variant: fill only, no border), calendar selected day, stepper active step, chip checked.

### 3.8 Focus: `@include sd-focus-ring`
`&:focus-visible { outline: 2px solid var(--sd-focus-ring); outline-offset: 2px; }`. Every interactive element gets it, with no exceptions. Never write `outline: none` without this mixin.

### 3.9 Disabled
`opacity: var(--sd-disabled-opacity)` (0.45), `cursor: not-allowed`, no hover change, no glow, and `pointer-events: none` on inner decorations. This is the only disabled treatment.

### 3.10 Semantic surface (alert, toast, badge, status pill, callout): `@include sd-semantic(<sem>)`
Fill `--sd-<sem>-soft`, border `--sd-<sem>-border`, title and icon `--sd-<sem>-strong`, body `--sd-text-muted`. Alerts and toasts add a 3px leading rail in `--sd-<sem>`. The neutral and brand variant uses the accent tokens with the same structure.

### 3.11 Value fill (progress, slider range, switch on, gauge, rating)
Solid `--sd-accent` (or `--sd-<sem>` for semantic progress) with `--sd-glow-sm`. Tracks are `--sd-well` with `--sd-border-subtle`, and never white.

### 3.12 Text and labels
Body text is `--sd-text-md` (14px). Uppercase section labels use `--sd-font-mono`, `--sd-text-2xs`, `letter-spacing: .08em` and `--sd-text-subtle`. Card and panel titles use `--sd-text-lg` at weight 600. Numbers in columns use `font-variant-numeric: tabular-nums`. Code uses `--sd-font-mono`, never bare `monospace`.

### 3.13 Data visualisation
Series colours are `--sd-chart-1..8`, where `--sd-chart-1` is the accent. Grid lines use `--sd-border-subtle`, axis text `--sd-text-subtle` in `--sd-font-mono` at `--sd-text-2xs`. Chart tooltips are floating panels (3.2). Paint is set in SCSS classes or with `var(--sd-chart-n)` in SVG attributes, never hex in Rust.

---

## 4. Glow budget

| Allowed | Token | Forbidden |
| :-- | :-- | :-- |
| Focused field | `--sd-glow-sm` | Static containers (card, table, grid cell, panel) at rest |
| Selected item | `--sd-glow-sm` | More than one glowing element per group |
| Primary button | `--sd-glow-sm`, hover `--sd-glow-md` | Secondary, ghost or outline buttons |
| Value fills, checked toggles | `--sd-glow-sm` | Text shadows on headings |
| Opt-in `glow` prop on card or modal | `--sd-glow-md` | Hard-coded `rgba(168,85,247,…)` anywhere |

Glow intensity is set globally by `--sd-glow-strength` (`GlowMode`): Off 0, **Subtle 0.6 (default)**, Expressive 1.

---

## 5. Motion

| Use | Token or mixin |
| :-- | :-- |
| Hover and colour changes | `sd-transition` (`--sd-dur-fast`, `--sd-ease`) |
| Press | `sd-press` (scale 0.97) |
| Floating layer enter | `sd-animate(sd-pop-in, var(--sd-dur))` |
| Backdrop enter | `sd-animate(sd-fade-in, var(--sd-dur))` |
| Drawer enter | `sd-animate(sd-slide-in-<side>, var(--sd-dur-slow))` |
| Decorative loops (skeleton shimmer, ping) | `sd-animate(...)`, which is removed under reduced motion |
| Essential loops (spinner, indeterminate progress) | `sd-animate-essential(...)`, which slows down under reduced motion |

Keyframes live in `_animations.scss` and are named `sd-*`. Components don't declare their own keyframes.
