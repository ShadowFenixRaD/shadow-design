# Token reference

This table is the source of truth. `crate/scss/_tokens.scss` implements it: defaults on `:root` inside `@layer sd-base`, and radius scale classes in the same layer. `crate/src/theme.rs` overrides the accent, surface and glow tokens at runtime.

To add a token, add it here and in `_tokens.scss` in the same change, and say why in the commit message. Never inline a value because a token is missing.

## Accent (theme-computed from one hex)

| Token | Default (Ultraviolet `#a855f7`) | Derivation in `theme.rs` | Use |
| :-- | :-- | :-- | :-- |
| `--sd-accent` | `#a855f7` | base | Solid accent: fills, rails, focused field border |
| `--sd-accent-rgb` | `168, 85, 247` | base | `rgba(var(--sd-accent-rgb), a)` |
| `--sd-accent-strong` | `#c691fa` | mix(base, white, 35%) | Accent text and icons on dark, selected text |
| `--sd-accent-deep` | `#653394` | mix(base, black, 40%) | Bottom of the primary lit fill |
| `--sd-accent-soft` | `rgba(168,85,247,.14)` | base @ .14 | Selected fill, outline-button hover |
| `--sd-accent-border` | `rgba(168,85,247,.45)` | base @ .45 | Selected border, primary border |
| `--sd-accent-fg` | `#ffffff` | `#000` if luminance(base) > 0.55, else `#fff` | Text on a solid accent |
| `--sd-focus-ring` | `#c691fa` | = accent-strong | `:focus-visible` outline |

Presets: Ultraviolet `#a855f7`, Obsidian `#f4f4f5`, Crimson `#ff1744`, Cyan `#00e5ff`, Emerald `#00ff9d`. Any `PrimaryColor::Custom` hex goes through the same derivation.

## Surfaces & neutrals

| Token | Default | Use |
| :-- | :-- | :-- |
| `--sd-void` | `#030305` | Page background (`body`) |
| `--sd-well` | `#060609` | Recessed fields, tracks |
| `--sd-surface-1` | `#09090d` | Flat-lit containers |
| `--sd-surface-2` | `#111118` | Floating layers, secondary button bottom |
| `--sd-surface-3` | `#181824` | Secondary button top, hover inside floating layers |
| `--sd-text` | `#f4f4f5` | Primary text |
| `--sd-text-muted` | `#a1a1aa` | Secondary text, item rest colour |
| `--sd-text-subtle` | `#8b8b96` | Labels, placeholders, captions (≥ 5:1 on surface-1, the minimum) |
| `--sd-border-subtle` | `rgba(255,255,255,.07)` | Dividers inside containers, chart grid |
| `--sd-border` | `rgba(255,255,255,.12)` | Container border |
| `--sd-border-strong` | `rgba(255,255,255,.20)` | Fields, floating layers, secondary button |
| `--sd-border-hover` | `rgba(255,255,255,.30)` | Hover on interactive borders |
| `--sd-tint-1` | `rgba(255,255,255,.04)` | Item hover, header/footer band |
| `--sd-tint-2` | `rgba(255,255,255,.07)` | Item pressed |
| `--sd-tint-3` | `rgba(255,255,255,.11)` | Strong neutral emphasis (kbd, code inline) |
| `--sd-scrim` | `rgba(0,0,0,.72)` | Backdrop |
| `--sd-backdrop-blur` | `8px` | Backdrop blur |
| `--sd-disabled-opacity` | `0.45` | The one disabled treatment |

## Semantic (fixed, never follow the accent)

For each `<sem>` in `success`, `warning`, `danger` and `info`:

| Token | success | warning | danger | info |
| :-- | :-- | :-- | :-- | :-- |
| `--sd-<sem>` | `#00e58f` | `#fbbf24` | `#ff2e57` | `#38bdf8` |
| `--sd-<sem>-rgb` | `0, 229, 143` | `251, 191, 36` | `255, 46, 87` | `56, 189, 248` |
| `--sd-<sem>-strong` | `#6ef7c4` | `#fde68a` | `#ff8fa6` | `#93dcfc` |
| `--sd-<sem>-soft` | rgb @ .12 | rgb @ .12 | rgb @ .12 | rgb @ .12 |
| `--sd-<sem>-soft-hover` | rgb @ .20 | rgb @ .20 | rgb @ .20 | rgb @ .20 |
| `--sd-<sem>-border` | rgb @ .38 | rgb @ .38 | rgb @ .38 | rgb @ .38 |

## Shadows & glow

| Token | Default | Use |
| :-- | :-- | :-- |
| `--sd-specular` | `inset 0 1px 0 0 rgba(255,255,255,.07)` | Top edge of every surface |
| `--sd-specular-strong` | `inset 0 1px 0 0 rgba(255,255,255,.18), inset 0 -1px 0 0 rgba(0,0,0,.35)` | Lit controls (top light and bottom shade) |
| `--sd-inset-well` | `inset 0 1px 2px 0 rgba(0,0,0,.55)` | Recessed fields |
| `--sd-elev-1` | `0 1px 2px rgba(0,0,0,.5), 0 4px 12px -4px rgba(0,0,0,.55)` | Containers |
| `--sd-elev-2` | `0 10px 28px -6px rgba(0,0,0,.75), 0 0 0 1px rgba(0,0,0,.4)` | Floating layers |
| `--sd-elev-3` | `0 28px 56px -12px rgba(0,0,0,.85), 0 0 0 1px rgba(0,0,0,.5)` | Modal, drawer |
| `--sd-glow-strength` | `0.6` | Off 0, Subtle 0.6, Expressive 1 (`GlowMode`) |
| `--sd-glow-sm` | `0 0 12px -2px rgba(var(--sd-accent-rgb), calc(.5 * var(--sd-glow-strength)))` | Focus, selected, primary, value fills |
| `--sd-glow-md` | `0 0 22px -4px rgba(var(--sd-accent-rgb), calc(.55 * var(--sd-glow-strength)))` | Primary hover, opt-in container glow |
| `--sd-glow-danger` | same as `glow-sm` with `--sd-danger-rgb` | Focused field in error |

The `sd-shadow` mixin composes `--sd-shadow-specular`, `--sd-shadow-elev` and `--sd-shadow-glow`. Those three are **per-element slots**: the mixin resets them on its element, so they never leak to children. An empty slot is `$sd-shadow-none` (`0 0 0 0 transparent`).

`sd-semantic(<sem>)` sets three more per-element slots for its children: `--sd-sem` (solid), `--sd-sem-rgb` and `--sd-sem-strong` (titles, icons). They are not defined on `:root`.

## Radius (scale class set by `ThemeProvider`)

| Token | Sharp | **Medium (default)** | Soft | Role |
| :-- | :-- | :-- | :-- | :-- |
| `--sd-radius-xs` | 2px | 3px | 4px | Checkbox box, kbd |
| `--sd-radius-sm` | 2px | 6px | 8px | Items inside containers |
| `--sd-radius-md` | 2px | 8px | 12px | Standalone controls, fields, tracks, tooltip |
| `--sd-radius-lg` | 3px | 12px | 16px | Containers, floating panels |
| `--sd-radius-xl` | 4px | 16px | 22px | Modal, dialog, drawer free edge |
| `--sd-radius-full` | 9999px | 9999px | 9999px | Pills (badge, chip, switch, avatar, thumb) |

Classes `.sd-radius-sharp`, `.sd-radius-medium` and `.sd-radius-soft` set these. `:root` carries the Medium values.

## Size

| Token | Value | Use |
| :-- | :-- | :-- |
| `--sd-control-xs / sm / md / lg / xl` | 26 / 32 / 40 / 48 / 54px | Standalone controls (button, field, select, pin cell) |
| `--sd-item-h-sm / item-h / item-h-lg` | 28 / 32 / 36px | Items inside a track or menu |
| `--sd-space-1 … 10` | 4, 8, 12, 16, 20, 24, —, 32, —, 40px (`1`, `2`, `3`, `4`, `5`, `6`, `8`, `10`) | Padding and gap. Group, Stack and Grid `xs/sm/md/lg/xl` = space-1/2/4/6/8 |
| `--sd-icon-sm / md / lg` | 14 / 16 / 20px | Icon sizes; stroke stays 1.5 |

## Typography

| Token | Value |
| :-- | :-- |
| `--sd-font-sans` | `'Geist', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif` |
| `--sd-font-mono` | `'Geist Mono', ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace` |
| `--sd-text-2xs / xs / sm / md / lg / xl / 2xl / 3xl` | 11 / 12 / 13 / 14 / 16 / 18 / 22 / 28px |
| `--sd-leading-tight / normal` | 1.25 / 1.5 |
| Weights | 400 body · 500 controls, items, labels · 600 titles · 700 page headings only |

## Motion

| Token | Value |
| :-- | :-- |
| `--sd-ease` | `cubic-bezier(.16, 1, .3, 1)` |
| `--sd-dur-fast` | `120ms` (hover, colour) |
| `--sd-dur` | `180ms` (enter, pop-in, fade) |
| `--sd-dur-slow` | `280ms` (drawer, accordion) |

## Charts

`--sd-chart-1` = `var(--sd-accent)`. `--sd-chart-2..8` = `#38bdf8`, `#00e58f`, `#fbbf24`, `#ff2e57`, `#f472b6`, `#a3e635`, `#94a3b8`.

## Code (component-requested)

| Token | Value | Use |
| :-- | :-- | :-- |
| `--sd-code-keyword` | `var(--sd-accent-strong)` | Keywords |
| `--sd-code-fn` | `var(--sd-info-strong)` | Function names |
| `--sd-code-type` | `var(--sd-chart-6)` | Types |
| `--sd-code-string` | `var(--sd-success-strong)` | String literals |
| `--sd-code-number` | `var(--sd-warning-strong)` | Numbers |
| `--sd-code-comment` | `var(--sd-text-subtle)` | Comments |
| `--sd-code-operator` | `var(--sd-text-muted)` | Operators and punctuation |

## Breakpoints (SCSS only, because media queries can't read variables)

`$bp: (sm: 640px, md: 768px, lg: 1024px, xl: 1280px)`, used through `@include bp-up(md)` (min-width). Never write a raw `@media (max-width: …)` in a component.
