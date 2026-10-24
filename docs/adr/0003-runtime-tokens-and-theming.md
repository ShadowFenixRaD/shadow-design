# ADR-0003: Runtime CSS tokens are the only theming mechanism

- Status: Accepted
- Date: 2026-10-24

## Context

Theming used three mechanisms that disagreed with each other:

1. CSS variables (`--sd-accent`, `--sd-radius-*`), read by only 11 of 83 component files;
2. per-preset class names generated in `theme.rs` (`sd-btn--primary-crimson`, `--checked-cyan`, `__bar--emerald`, …), each backed by hand-written SCSS, so adding a preset meant editing about 10 files. `PrimaryColor::custom_with_prefix` generated class names that didn't exist, so it only worked with the prefix `"custom"`;
3. Tailwind-style utility strings emitted from Rust (`RadiusScale::md()` → `rounded-lg`) in a higher cascade layer, overriding the SCSS. As a result Sharp left cards at 14px while Medium gave 8px, and the theme and no-theme builds rendered different shapes.

## Decision

1. **All theming is CSS custom properties.** Accent, surfaces, radius, glow strength and focus ring are runtime `--sd-*` variables, defined with defaults in `crate/scss/_tokens.scss` (layer `sd-base`, on `:root`). The token table in [`docs/design/tokens.md`](../design/tokens.md) is the source of truth.
2. **`Theme` only writes variables.** `ThemeProvider` emits the full accent set computed from a single hex (`--sd-accent`, `-rgb`, `-strong`, `-deep`, `-soft`, `-border`, `-fg`, `--sd-focus-ring`), the surface overrides, and `--sd-glow-strength`. It also applies one radius class (`sd-radius-sharp|medium|soft`), which sets `--sd-radius-*`. Any custom colour therefore works with every component.
3. **Removed:** per-preset component classes (`*--{preset}`, `*-crimson`, `*-cyan`, …); every Tailwind-style class emitted by the theme engine (`RadiusScale::sm()/md()/lg()` no longer return utility classes); the `rounded-*` coupling; the Cartoon preset; `ElevationStyle::HardBlock` and `ElevationStyle::SoftElevated`. `ElevationStyle` keeps `NeonGlow` (default) and `Flat`; `Flat` behaves like `GlowMode::Off` for glow.
4. **`ComponentOverride` stays.** It is the supported way for consumers to swap a component's classes. Library defaults never depend on it.
5. **The no-theme fallback renders identically.** With `feature = "theme"` off, components emit the same `sd-*` classes and the `:root` token defaults apply, so both builds look the same by construction.
6. **Presets are data.** A preset is a name plus an accent hex (plus, optionally, surface hexes). Adding one touches `theme.rs` only.

## Consequences

- Any accent hex works everywhere, including `custom_with_prefix` with any prefix.
- `--sd-accent-fg` is computed from luminance, so a light accent (Obsidian white, a custom yellow) gets dark text.
- Breaking changes for consumers who styled the removed per-preset classes; these are listed in `CHANGELOG.md`.
- Deferred, not decided here: self-hosting the Geist fonts (they are currently a Google Fonts `@import`) and gating SCSS partials by Cargo feature to shrink the 410 KB stylesheet. Each needs its own ADR.
