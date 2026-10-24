# AGENTS.md — Shadow Design Workspace Guide

Istruzioni operative, architettura e standard di sviluppo per agenti AI e sviluppatori che lavorano su **Shadow Design**.

> `GEMINI.md` e `CLAUDE.md` sono symlink a questo file: modifica **solo** `AGENTS.md`.

---

## 0. Navigazione del Codice con graphify

Il progetto ha un knowledge graph in `graphify-out/` (god nodes, community structure, relazioni cross-file).

- Quando esiste `graphify-out/graph.json`, esegui `graphify query "<domanda>"` **prima** del primo grep, ricerca o lettura di file sorgente per una domanda sul codice, e includi questa regola in ogni prompt di subagent che esplora il codice. Leggi i file raw solo per le righe già localizzate dal grafo o da una lettura precedente.
- Usa `graphify path "<A>" "<B>"` per le relazioni e `graphify explain "<concetto>"` per concetti mirati: restituiscono un sottografo circoscritto, molto più piccolo di `GRAPH_REPORT.md` o dell'output di grep.
- Se esiste `graphify-out/wiki/index.md`, usalo per la navigazione ad ampio raggio invece di sfogliare i sorgenti.
- Leggi `graphify-out/GRAPH_REPORT.md` solo per review architetturali ampie o quando query/path/explain non bastano.
- Dopo aver modificato codice, esegui `graphify update .` per mantenere il grafo aggiornato (solo AST, nessun costo API).
- `graphify-out/graph.json` viene riscritto per intero a ogni build: un diff ampio in una PR è atteso. Non segnalarlo come churn e non suggerire di saltare il rebuild o i relativi hook.

---

## 1. Panoramica del Progetto

**Shadow Design** (v0.5.x) è una libreria di componenti UI modulare e ad alte prestazioni per **Dioxus 0.7** (Web, Desktop, Mobile), nata nell'ecosistema **Shadowfenix**. Licenza `MIT OR Apache-2.0`.

### Filosofia Estetica
- **Sfondo**: nero assoluto (`#000000`) con superfici a 4 livelli (`--color-void`, `--color-surface-1..3`).
- **Accenti**: viola Ultraviolet di default, con preset Obsidian, Crimson, Cyan, Emerald e Cartoon.
- **Dettagli**: micro-glow, specular highlights, bordi hairline semi-trasparenti, contrasti elevati, micro-scrollbar.
- **Modularità**: architettura *zero-bloat*, ogni componente è dietro una feature flag Cargo indipendente.
- **Zero tooling a valle**: nessun Node/npm/Tailwind richiesto ai consumatori; lo SCSS è compilato dalla pipeline `asset!()` di Dioxus.

---

## 2. Struttura del Workspace Cargo

```text
shadow-design/
├── Cargo.toml          # Workspace (resolver "2", dioxus 0.7.1, serde, serde_json)
├── flake.nix           # Dev shell Nix (toolchain, dx, sass, graphify) — vedi §8
├── clippy.toml         # await-holding-invalid-types per Signal / GenerationalBox
├── AGENTS.md           # Questo file (GEMINI.md e CLAUDE.md sono symlink)
├── CHANGELOG.md
├── deploy.nu           # Build release web dello showcase + deploy in /var/www/shadow-design
├── scripts/
│   └── check_classes.py   # Verifica che le classi usate in RSX esistano nel CSS compilato
├── graphify-out/       # Knowledge graph del codice (vedi §0)
├── .agents/            # Regole/workflow agent (graphify)
├── crate/              # Libreria `shadow-design`
│   ├── Cargo.toml      # Feature flags (vedi §3)
│   ├── build.rs        # rerun-if-changed=scss
│   ├── tests/          # Test di integrazione (*_tests.rs)
│   ├── scss/           # Sistema di stile 100% SCSS (vedi §4)
│   └── src/
│       ├── lib.rs      # Moduli condizionali, `STYLES`, `VERSION`, `pub mod prelude`
│       ├── a11y.rs     # use_unique_id, ControlSize, AriaHeading (sempre compilato)
│       ├── locale.rs   # Locale, LocaleProvider, use_locale (sempre compilato)
│       ├── util.rs     # Helper interni (hex_to_rgb, parse_hex_rgba, …)
│       ├── theme.rs    # Theme engine, preset, ThemeProvider, use_theme
│       ├── icon.rs     # IconKind: icone SVG native (stroke 1.5px)
│       ├── <componente>.rs        # Componenti semplici (button, card, input, …)
│       ├── <componente>/mod.rs    # Componenti composti (accordion, app_shell, menubar,
│       │                          #   stepper, pagination, nav_link, breadcrumbs, …)
│       ├── chart/      # Feature `charts`: primitives, area, bar, line, pie, radar,
│       │               #   scatter, sparkline, gauge, heatmap
│       └── hooks/      # Feature `hooks`: click_outside, clipboard, debounce, hotkey,
│                       #   idle, intersection, local_storage, media_query, scroll
└── showcase/           # App Dioxus di documentazione interattiva (publish = false)
    ├── Cargo.toml      # Dipende da shadow-design { features = ["all", "serde"] }; feature web/desktop/mobile
    ├── Dioxus.toml
    ├── assets/         # critical.css (boot dark), header.svg, favicon.ico
    └── src/
        ├── main.rs     # App root, `enum Route` (Dioxus Router), provider e STYLES
        ├── components/
        │   ├── layout.rs        # Shell: header, toolbar tema, sidebar, Outlet::<Route>
        │   ├── nav.rs           # Registry unico COMPONENTS / CATEGORIES + SearchPalette (Ctrl+K)
        │   ├── showcase_tabs.rs # Tabs Playground / Codice RSX / API Props
        │   ├── props_table.rs   # Tabella documentazione props
        │   ├── banner.rs        # Banner con comando Cargo feature
        │   └── clipboard.rs     # Copy-to-clipboard async con fallback
        └── pages/      # Una pagina per componente + home.rs, theme_builder.rs, icons.rs
```

Le fonti di verità per l'elenco dei componenti sono: `crate/Cargo.toml` (`[features]`), `crate/src/lib.rs` (moduli e `prelude`) e `showcase/src/components/nav.rs` (`COMPONENTS`). Non duplicare elenchi completi in questo file.

---

## 3. Feature Flags

- `default = ["all"]`. `all` abilita l'intera suite (esclusa `serde`).
- Nomi canonici in `snake_case` (es. `number_input`), con alias `kebab-case` (es. `number-input = ["number_input"]`).
- Le feature dichiarano le proprie dipendenze interne (es. `input = ["icon", "close_button"]`, `table = ["icon", "pagination", "loading_overlay"]`).
- `serde`: serializzazione JSON di `Theme`, preset e override (`dep:serde`, `dep:serde_json`).

| Area | Feature |
| :--- | :--- |
| Core & tema | `theme`, `icon`, `typography`, `animation`, `hooks` |
| Form & input | `button`, `input`, `select` (Select/MultiSelect/Combobox), `textarea`, `checkbox`, `radio`, `switch`, `slider`, `rating`, `number_input`, `pin_input`, `color_picker`, `chip`, `segmented_control`, `date_picker`, `dropzone`, `fieldset`, `form` |
| Layout | `app_shell`, `grid`, `group`, `stack`, `layout_primitives`, `center`, `aspect_ratio`, `divider`, `scroll_area`, `resizable`, `masonry` |
| Navigazione | `navbar`, `tabs`, `breadcrumbs`, `pagination`, `nav_link`, `bottom_navigation`, `stepper`, `menubar`, `navigation_menu`, `overflow_set`, `burger` |
| Overlay | `modal`, `drawer`, `dropdown`, `tooltip`, `popover`, `hover_card`, `context_menu`, `alert_dialog`, `coachmark`, `speed_dial`, `affix`, `command_palette`, `keytip` |
| Dati & feedback | `card`, `badge`, `alert`, `notification`, `progress`, `skeleton`, `loading_overlay`, `indicator`, `table`, `list`, `timeline`, `tree`, `virtual_list`, `transfer_list`, `accordion`, `spoiler`, `carousel` |
| Grafici | `charts` (= `chart_primitives`, `chart_area`, `chart_bar`, `chart_line`, `chart_pie`, `chart_radar`, `chart_scatter`, `chart_sparkline`, `chart_gauge`, `chart_heatmap`; base `chart_core`) |
| Rich content & social | `avatar`, `chat`, `attachment_card`, `contact_card`, `document_card`, `media_card`, `facepile`, `activity_feed`, `rich_text`, `code_highlight` |
| Micro-utility | `close_button`, `copy_button`, `highlight`, `marquee_selection` |

---

## 4. Sistema di Stile (100% SCSS)

TailwindCSS è stato rimosso. Tutti gli stili vivono in `crate/scss/` e sono esposti come asset Dioxus in `crate/src/lib.rs`:

```rust
pub const STYLES: Asset = asset!("/scss/main.scss");
```

Consumo nelle app:

```rust
document::Link { rel: "stylesheet", href: shadow_design::STYLES }
```

### File principali
| File | Ruolo |
| :--- | :--- |
| `main.scss` | Entrypoint: `@use` di tutti i partial e di ogni `components/_*.scss`, più `:root` vars e utility ottiche in `@layer sd-base` |
| `_layers.scss` | Ordine dei cascade layer: `sd-reset, sd-base, sd-components, sd-utilities, sd-theme` |
| `_variables.scss` | Token colore, superfici, font Geist, raggi |
| `_fonts.scss` | Font face |
| `_zindex.scss` | Mappa `$z-layers`, funzione `z($layer)` e classi `.z-layer-*` |
| `_mixins.scss` | Specular highlight, ambient glow, micro-scrollbar, tactile press |
| `_reset.scss` | Preflight reset (`button`, `input`, `textarea`, `select`, …) |
| `_animations.scss` | Keyframes (toast, modal, dropdown, drawer, spin, pulse, …) e classi `.animate-*` |
| `_utilities.scss` | Utility universali (flex, grid, sizing, spacing, typography, colori, bordi, ombre) |
| `_theme_presets.scss` + `_preset_cartoon.scss` | Classi dei preset, ring di selezione, glow matrix |
| `components/_<nome>.scss` | Stili BEM per componente (`.sd-btn`, `.sd-card`, `.sd-input`, …) |

### Regole
- Le classi di componente usano il prefisso `sd-` con BEM (`.sd-btn--primary`, `.sd-card__header`).
- Ogni regola deve stare nel cascade layer corretto (componenti in `sd-components`, utility in `sd-utilities`).
- Non usare z-index numerici: usa `z('<layer>')` in SCSS o le classi `.z-layer-*`.
- Se usi in RSX una classe utility nuova, verifica che esista con `scripts/check_classes.py`.

### Gerarchia Z-Index (`_zindex.scss`)
| Layer | Valore |
| :--- | :--- |
| `base` / `sticky` / `header` | 0 / 20 / 30 |
| `speed-dial`, `affix` | 60 |
| `drawer-back` / `drawer` | 65 / 70 |
| `modal-back` / `modal` / `modal-content` | 75 / 80 / 85 |
| `notification` | 90 |
| `coachmark` | 95 |
| `tooltip` | 100 |
| `popover-back` / `popover` | 989 / 990 |
| `dropdown-back`, `context-menu-back` | 995 |
| `dropdown`, `context-menu` | 1000 |

Per evitare clipping dei menu fluttuanti, `Card` e `ShowcaseTabs` non usano `overflow: hidden` sull'intero contenitore. `Dropdown` supporta `DropdownPlacement` (`BottomRight`, `BottomLeft`, `TopRight`, `TopLeft`) per aprirsi verso l'alto vicino al fondo di un contenitore.

---

## 5. Theme Engine (`crate/src/theme.rs`)

- **`Theme`**: token di layout, superfici, accenti e classi per componente. Si costruisce dai preset o con `Theme::builder()` / `ThemeBuilder`.
- **Preset**: `Theme::shadow()` (default, Ultraviolet), `obsidian()`, `crimson()`, `cyan()`, `emerald()`, `cartoon()`.
- **Scale**:
  - `PrimaryColor`: `Violet`, `Obsidian`, `Crimson`, `Cyan`, `Emerald`, `Custom { name, hex, css_prefix }` (`PrimaryColor::custom`, `custom_with_prefix`).
  - `RadiusScale`: `Sharp` (2px), `Medium` (8px, default), `Soft` (14px).
  - `GlowMode`: `Subtle` (default), `Expressive`, `Off`.
  - `ElevationStyle`: `NeonGlow` (default), `HardBlock`, `SoftElevated`, `Flat`.
- **Override per componente**: `ComponentOverride` (base class, varianti, size, shadow, modifier) via `with_component_override` / `with_class_override`; risoluzione con `resolve_variant_class`, `resolve_size_class`, `resolve_elevation_class`, ecc.
- **Mutazioni runtime**: `apply_primary_color`, `apply_radius_scale`, `apply_glow_mode`, `apply_elevation_style`.
- **CSS variables**: `to_css_variables()` / `to_inline_css_variables()`.
- **Context**: `ThemeProvider` fornisce un `Signal<Theme>`; `use_theme()` lo legge o lo scrive (`theme.write()`).

### Supporto duale (`theme` on/off)
Ogni componente deve compilare in entrambe le modalità tramite `#[cfg]`:
1. **Con `theme`**: legge le classi da `crate::theme::use_theme()` e si aggiorna reattivamente al cambio tema.
2. **Senza `theme`** (`#[cfg(not(feature = "theme"))]`): fallback su classi statiche hardcoded, zero overhead.

### Locale (`crate/src/locale.rs`)
Le stringhe UI (placeholder, "Nothing found", "Close", ecc.) vengono da `Locale` (default inglese), fornito globalmente con `LocaleProvider` e letto con `use_locale()`; i singoli componenti possono sovrascriverle via props. Non hardcodare testo visibile nei componenti della libreria.

---

## 6. Showcase (`showcase/`)

- **Routing**: `enum Route` in `showcase/src/main.rs`, tutte le pagine sotto `#[layout(Layout)]`.
- **Registry unico** (`showcase/src/components/nav.rs`): ogni `ComponentItem` in `COMPONENTS` (nome, `route`, `urls`, categoria `class`, `features`, `icon`, `description`, `keywords`) si propaga automaticamente a sidebar, `SearchPalette` (Ctrl+K) e `all_cargo_features()`. Le categorie sono in `CATEGORIES`.
- **Layout shell** (`layout.rs`): header con logo Shadowfenix, toolbar tema (swatch colore, radius, glow), sidebar, `Outlet::<Route> {}`.
- **`ShowcaseTabs`**: Playground (controlli live), Codice RSX (copia con feedback "Copiato!"), API Props (Nome, Tipo Rust, Default, Descrizione).
- I testi dello showcase sono in italiano; la documentazione rustdoc della libreria è in inglese.

---

## 7. Regole di Sviluppo

### Dioxus 0.7
- Niente API obsolete: `cx`, `Scope`, `use_state` non esistono.
- Stato: `use_signal(|| init)`; lettura `sig()` o `*sig.read()`; scrittura `*sig.write() = v` o `sig.with_mut(|v| …)`.
- Props: `Clone + PartialEq`, tipi posseduti (`String`, `Option<String>`, `Signal<T>`), mai `&str` / `&[T]`. Eventi come `Option<EventHandler<MouseEvent>>` o simili.
- RSX: `for item in items { … }`, `if cond { … }`, stringhe formattate `"{base} {extra}"`.
- **Hook**: chiamali sempre incondizionatamente e nello stesso ordine. Non chiamare hook dentro le closure di altri hook (es. `use_unique_id` dentro `use_signal(|| …)`): corrompe gli slot e causa panic.

### Concorrenza (`clippy.toml`)
Non trattenere mai attraverso un `.await`: `generational_box::GenerationalRef`, `generational_box::GenerationalRefMut`, `dioxus_signals::WriteLock`. Clona il valore o chiudi lo scope del lock prima dell'`.await`.

### Lint
`crate/Cargo.toml` attiva `clippy::must_use_candidate` e `clippy::return_self_not_must_use` come warning: annota con `#[must_use]` getter e builder.

### Aggiungere un nuovo componente
1. Crea `crate/src/<nome>.rs` (o `crate/src/<nome>/mod.rs` se composto da più parti).
2. In `crate/Cargo.toml`: aggiungi la feature `snake_case` con le sue dipendenze, l'alias `kebab-case` se il nome è composto, e inseriscila in `all`.
3. In `crate/src/lib.rs`: `#[cfg(feature = "<nome>")] pub mod <nome>;` e il re-export in `pub mod prelude`; aggiorna la tabella feature nella doc del crate.
4. Implementa sia il percorso `use_theme()` sia il fallback `#[cfg(not(feature = "theme"))]`; usa `Locale` per le stringhe e `a11y` per ID/ARIA.
5. Se servono classi nel tema, aggiungi campi e builder a `Theme` in `theme.rs` (inclusi tutti i preset).
6. Crea `crate/scss/components/_<nome>.scss` (layer `sd-components`, z-index via `z()`) e aggiungi `@use 'components/<nome>' as *;` in `crate/scss/main.scss`.
7. Aggiungi test in `crate/tests/` se il componente ha logica non banale.
8. Showcase: crea `showcase/src/pages/<nome>.rs`, registralo in `pages/mod.rs`, aggiungi la variante a `Route` in `main.rs` e un `ComponentItem` in `COMPONENTS` (`nav.rs`).
9. Aggiorna `CHANGELOG.md`.

---

## 8. Comandi

Esegui dalla root del workspace, dentro la dev shell Nix (`flake.nix`): fornisce toolchain Rust stable con target `wasm32-unknown-unknown`, `dx` (dioxus-cli), `sass`, `wasm-opt`, Python/uv, `nu` e installa graphify in `.tools/graphify` (venv locale, ignorato da git). Nessun tool va installato globalmente.

> Quando aggiorni `wasm-bindgen` in `Cargo.lock`, aggiorna anche `pkgs.wasm-bindgen-cli_0_2_XXX` in `flake.nix`: `dx` rifiuta versioni CLI diverse da quella del lockfile.

```bash
nix develop                      # Entra nella dev shell (o: nix develop --command <cmd>)

cargo check --workspace          # Compilazione rapida
cargo clippy --workspace         # Lint (rispetta clippy.toml)
cargo test --workspace           # Test (incluso crate/tests/)

# Verifica le combinazioni di feature quando tocchi #[cfg] o dipendenze
cargo check -p shadow-design --no-default-features --features <feature>

cd showcase && dx serve          # Dev server dello showcase (hot-reload)

python scripts/check_classes.py  # Classi RSX vs CSS compilato (richiede `npx sass`)

graphify update .                # Aggiorna il knowledge graph dopo modifiche al codice

nu deploy.nu                     # Build release web + deploy (usa sudo, solo su richiesta)
```
