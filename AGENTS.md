# AGENTS.md — Shadow Design Workspace Guide

Istruzioni operative, architettura e standard di sviluppo per agenti AI e sviluppatori che lavorano su **Shadow Design**.

> `GEMINI.md` e `CLAUDE.md` sono symlink a questo file: modifica **solo** `AGENTS.md`.

---

## ⚠️ Contratto di design (vincolante, leggere per primo)

Prima di toccare `crate/scss/`, `crate/src/` (componenti o tema) o `showcase/src/`:

1. **Leggi tutti gli ADR *Accepted* in [`docs/adr/`](docs/adr/README.md) e tutto [`docs/design/`](docs/design/README.md)**: `language.md` (linguaggio *Lit Glass* e ricette), `tokens.md` (ogni `--sd-*`), `component-checklist.md`.
2. **Il codice esistente non è un precedente.** Se il codice vicino contraddice un ADR, vale l'ADR: non copiare mai la deviazione.
3. **Nessun pattern nuovo in silenzio.** Un token nuovo, un trattamento visivo nuovo o un'eccezione richiedono prima un ADR *Proposed*, da implementare solo dopo l'approvazione.
4. **`python scripts/lint_design.py` deve passare** (dentro `nix develop`). Il pre-commit hook in `.githooks/` lo esegue: la dev shell imposta `core.hooksPath`. Non aggirarlo con `--no-verify`.

In sintesi (i dettagli sono negli ADR): un solo materiale per tutto (superficie, hairline, bordo speculare, ombra); il glow è un segnale tenue, mai una decorazione; ciò che è statico non reagisce all'hover, ciò che è interattivo ha sempre hover, press, focus e disabled; valori solo da `var(--sd-*)`, dai mixin e da `z()`; nel Rust della libreria solo classi `sd-*`; testi UI solo da `Locale`.

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

### Filosofia Estetica: *Lit Glass* (ADR-0002, `docs/design/language.md`)
- **Materiale unico**: vetro nero illuminato dall'alto. Void `--sd-void` (`#030305`), pozzetti `--sd-well` per i campi, superfici `--sd-surface-1..3`.
- **Accenti**: viola Ultraviolet di default, con preset Obsidian, Crimson, Cyan, Emerald o qualsiasi hex custom. L'accento arriva ovunque via `--sd-accent*` (ADR-0003).
- **Dettagli**: glow tenue solo come segnale di stato (`--sd-glow-*`), bordo speculare superiore, hairline, contrasti elevati, micro-scrollbar.
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
| `main.scss` | Entrypoint: `@use` di tutti i partial e di ogni `components/_*.scss` |
| `_layers.scss` | Ordine dei cascade layer: `sd-reset, sd-base, sd-components, sd-utilities, sd-theme` |
| `_tokens.scss` | **Token runtime** `--sd-*` su `:root` (layer `sd-base`) e classi `.sd-radius-sharp|medium|soft`. Fonte di verità: `docs/design/tokens.md` |
| `_variables.scss` | Solo costanti SCSS di build (breakpoint `$bp`, palette per le utility). Vietate nei componenti |
| `_fonts.scss` | Import Google Fonts dei tre ruoli: Kanit (titoli), Onest (testo), Source Code Pro (mono), ADR-0005. Self-hosting rinviato |
| `_zindex.scss` | Mappa `$z-layers`, funzione `z($layer)` e classi `.z-layer-*` |
| `_mixins.scss` | Ricette Lit Glass: `sd-surface`, `sd-shadow`, `sd-lit-fill`, `sd-field`, `sd-item`, `sd-selected`, `sd-semantic`, `sd-focus-ring`, `sd-transition`, `sd-press`, `sd-animate`, `sd-scrollbar`, `bp-up` |
| `_reset.scss` | Preflight reset (`button`, `input`, `textarea`, `select`, …) |
| `_animations.scss` | Keyframes `sd-*` (pop-in, fade-in, slide-in, spin, shimmer, …) e classi `.animate-*` |
| `_utilities.scss` | Utility di layout per lo **showcase**: vietate nel Rust della libreria (ADR-0004) |
| `_theme_presets.scss` | Classi provider (`sd-glow-*`); niente classi per-preset (ADR-0003) |
| `components/_<feature>.scss` | Un blocco BEM per componente (`.sd-btn`, `.sd-card`, …) che usa solo token e mixin |

### Regole (dettagli in ADR-0004)
- Classi `sd-` in BEM, tutte dentro `@layer sd-components`.
- Nei partial dei componenti niente letterali colore, niente `$violet-*`/`$radius-*`, niente z-index numerici (eccetto -1/0/1 locali), niente `!important`; niente `box-shadow`/`transition`/`animation`/`font-size`/`border-radius`/`@media` diretti: usa token e mixin.
- Nel Rust della libreria solo classi `sd-*`, e ogni classe emessa deve esistere nel CSS compilato.
- `python scripts/lint_design.py` verifica tutto questo.

### Gerarchia Z-Index (`_zindex.scss`)
| Layer | Valore |
| :--- | :--- |
| `base` / `sticky` / `header` | 0 / 20 / 30 |
| `speed-dial`, `affix` | 60 |
| `drawer-back` / `drawer` | 65 / 70 |
| `modal-back` / `modal` / `modal-content` | 75 / 80 / 85 |
| `command-palette` | 88 |
| `notification` | 90 |
| `coachmark` | 95 |
| `popover-back` / `popover` (anche `hover-card`) | 989 / 990 |
| `dropdown-back`, `context-menu-back` | 995 |
| `dropdown`, `context-menu` | 1000 |
| `keytip` | 1005 |
| `tooltip` | 1010 (sopra ai menu) |

Per evitare clipping dei menu fluttuanti, `Card` e `ShowcaseTabs` non usano `overflow: hidden` sull'intero contenitore. `Dropdown` supporta `DropdownPlacement` (`BottomRight`, `BottomLeft`, `TopRight`, `TopLeft`) per aprirsi verso l'alto vicino al fondo di un contenitore.

---

## 5. Theme Engine (`crate/src/theme.rs`, ADR-0003)

- **Solo variabili CSS.** `ThemeProvider` scrive l'intero set accento calcolato da un solo hex (`--sd-accent`, `-rgb`, `-strong`, `-deep`, `-soft`, `-border`, `-fg`, `--sd-focus-ring`), le superfici e `--sd-glow-strength`, e applica una classe radius (`sd-radius-*`). Niente classi per-preset e niente utility Tailwind emesse dal tema.
- **Preset**: `Theme::shadow()` (default, Ultraviolet), `obsidian()`, `crimson()`, `cyan()`, `emerald()`. Un preset è un nome più un hex. Cartoon è stato rimosso.
- **Scale**:
  - `PrimaryColor`: `Violet`, `Obsidian`, `Crimson`, `Cyan`, `Emerald`, `Custom { name, hex, css_prefix }`. Qualsiasi hex funziona su tutti i componenti.
  - `RadiusScale`: `Sharp`, `Medium` (default), `Soft`, che impostano `--sd-radius-xs..xl` (valori in `docs/design/tokens.md`).
  - `GlowMode`: `Off` (0), `Subtle` (0.6, default), `Expressive` (1), che impostano `--sd-glow-strength`.
  - `ElevationStyle`: `NeonGlow` (default), `Flat`. HardBlock e SoftElevated sono stati rimossi.
- **Override per componente**: `ComponentOverride` (base class, varianti, size, shadow, modifier) via `with_component_override` / `with_class_override`. È l'unico motivo per cui un componente legge `use_theme()`.
- **Mutazioni runtime**: `apply_primary_color`, `apply_radius_scale`, `apply_glow_mode`, `apply_elevation_style`.
- **Context**: `ThemeProvider` fornisce un `Signal<Theme>`; `use_theme()` lo legge o lo scrive (`theme.write()`).

### Supporto duale (`theme` on/off)
Ogni componente compila e **rende in modo identico** in entrambe le modalità: emette le stesse classi `sd-*` e i default dei token su `:root` coprono il caso senza tema. Con `theme` attivo legge solo i `ComponentOverride`.

### Locale (`crate/src/locale.rs`)
Le stringhe UI e ARIA vengono da `Locale` (default inglese), fornito globalmente con `LocaleProvider` e letto con `use_locale()`; i singoli componenti possono sovrascriverle via props. Il lint segnala il testo italiano hardcodato nella libreria.

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

### Aggiungere o modificare un componente
Segui `docs/design/component-checklist.md` (vincolante). In breve:

1. Scegli la ricetta in `docs/design/language.md` §3; se nessuna va bene, scrivi prima un ADR.
2. Crea `crate/src/<nome>.rs` (o `crate/src/<nome>/mod.rs` se composto da più parti).
3. In `crate/Cargo.toml`: aggiungi la feature `snake_case` con le sue dipendenze, l'alias `kebab-case` se il nome è composto, e inseriscila in `all`.
4. In `crate/src/lib.rs`: `#[cfg(feature = "<nome>")] pub mod <nome>;` e il re-export in `pub mod prelude`; aggiorna la tabella feature nella doc del crate.
5. Emetti le stesse classi `sd-*` con e senza `theme` (con `theme` leggi solo i `ComponentOverride`); usa `Locale` per le stringhe e `a11y` per ID/ARIA.
6. Non aggiungere classi per-preset a `Theme`: i colori arrivano dai token `--sd-*` (ADR-0003).
7. Crea `crate/scss/components/_<nome>.scss` (layer `sd-components`, solo token, mixin e `z()`) e aggiungi `@use 'components/<nome>' as *;` in `crate/scss/main.scss`.
8. Aggiungi test in `crate/tests/` se il componente ha logica non banale.
9. Showcase: crea `showcase/src/pages/<nome>.rs` (usa componenti della libreria per la chrome, mai markup utility per i colori), registralo in `pages/mod.rs`, aggiungi la variante a `Route` in `main.rs` e un `ComponentItem` in `COMPONENTS` (`nav.rs`).
10. Aggiorna `CHANGELOG.md`.

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

python scripts/lint_design.py    # Contratto di design (ADR-0004): token, mixin, z(), classi sd-*, Locale
python scripts/check_classes.py  # Utility RSX dello showcase vs CSS compilato

graphify update .                # Aggiorna il knowledge graph dopo modifiche al codice

nu deploy.nu                     # Build release web + deploy (usa sudo, solo su richiesta)
```
