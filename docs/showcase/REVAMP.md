# Showcase revamp: build spec

Implements ADR-0006. Binding ADRs: 0001–0006, `docs/design/language.md`, `docs/design/tokens.md`. The showcase is a consumer of the library: it never changes `crate/`. Anything that needs a library change is in §8 (proposals), not in the plan.

Phases and owners:

| Phase | Owner | Output |
| :-- | :-- | :-- |
| 1. Scaffold | one engineer | `story/*`, `stories/mod.rs` + 85 stubs, `app/*` chrome, `scenes/*`, `pages/{home,tokens,not_found}.rs`, restyled `pages/{theme_builder,typography,icons}.rs`, `showcase.scss`, `main.rs`. Old pages moved to `showcase/legacy/pages/` (not compiled). `cargo check -p showcase` and `cargo test -p showcase` pass with every story a stub. |
| 2. Migrate | three engineers in parallel | each edits only `stories/<slug>.rs` of their batch (§6) and deletes the matching `legacy/pages/<file>.rs`. No shared file is touched. |
| 3. Verify | verifier | §7 acceptance criteria, then delete `showcase/legacy/`. |

Rust constraints for every snippet: Dioxus 0.7 (`#[component]`, `use_signal`, `Signal<T>`, `rsx!`, `Element`), props `Clone + PartialEq` and owned, hooks called unconditionally and never inside other hooks' closures, nothing held across `.await`. Italian copy, English identifiers.

---

## 1. Visual direction

Lit Glass does not change (ADR-0002). What changes is **what the glass shows**: compositions instead of lone widgets, the theme engine demonstrated live and side by side, and generated code that moves when the component moves.

### 1.1 Hero concept: "Un hex, tutto il tema"

The first screen of the home page is a working product, not a paragraph.

- Left column (lg: 5/12): eyebrow (mono 2xs, `Dioxus 0.7 · Lit Glass · v{VERSION}`), title "Shadow Design" at `calc(var(--sd-text-3xl) * 1.5)` in Kanit 700 with `--sd-leading-tight`, one-sentence lede in `--sd-text-lg` muted, two CTAs (`Button` Primary "Esplora i componenti" → `/buttons`, Secondary "Theme Builder"), and **the five accent swatches inline** (the same `ThemeSwatches` component the header menu uses). Under them a mono caption: "Clicca un colore: tutto lo showcase, compresa la scena qui accanto, cambia all'istante."
- Right column (lg: 7/12): `scenes::OpsDashboard` rendered live inside a flat-lit frame (`.sc-scene`), full height ≈ 30rem. It contains three metric `Card`s with `Sparkline`s, an `AreaChart` (3 series), a `DataTable` (5 rows, sortable), a `SegmentedControl` range picker, one Primary button, one `Badge`, one `Avatar` with presence. One accent element per group (ADR-0002 rule 5).
- Below the fold, in order: the **preset wall** (§1.2), **four scenes** in `Tabs` (§1.3), **zero bloat** counters and install code (§2.4), **fondamenta** cards (Tokens, Typography, Icons, Theme Builder), **categories** grid (8 cards, counts from the registry).

### 1.2 Presets, radius and glow demonstrated live

- **Preset wall** (home): five `PresetTile`s in a `SimpleGrid { cols: 1, sm: 2, lg: 5 }`. Each tile is wrapped in its own `ThemeProvider { theme: Some(local_signal) }` (it renders a `display: contents` wrapper with inline variables, so five accents coexist on one page). Content of a tile: preset name (Kanit `--sd-text-lg`), a Primary `Button`, a `Switch` on, a `Badge` primary, a `ProgressBar` at 64, and an Outline "Usa questo preset" button that writes the **global** theme. The tile whose preset is active gets `sd-selected`-style treatment via the library (`Card` with `glow: Some(true)`), never a hand-written ring.
- **Radius and glow rows** (Tokens page): three nested providers for `Sharp / Medium / Soft` showing `Card + Button + TextInput + Badge`, then three for `Off / Subtle / Expressive` showing a Primary button, a focused field (`glow: Some(true)`), a selected tab. Each row has an "Applica" button that sets the global value.
- **Story pages**: the playground header carries the five swatch dots (`ThemeSwatches { compact: true }`) so a reader can re-colour the component they are looking at without leaving the page.
- Header: the toolbar is gone. One `Button` (Ghost, icon `Palette`, with a `.sc-theme-dot` showing the current accent via `background: var(--sd-accent)`) opens a `Popover` (`PopoverPosition::BottomEnd`) with `ThemeSwatches`, the Radius and Glow `SegmentedControl`s and a link to the Theme Builder. This frees the header on every breakpoint.

### 1.3 Composed scenes versus isolated widgets

Four reusable scenes live in `showcase/src/scenes/` and are written in phase 1. Stories reference them as `Span::Full` examples titled "In contesto"; the home page shows all four in `Tabs`.

| Scene | Components it must use | Where it appears |
| :-- | :-- | :-- |
| `OpsDashboard` | `Card` (metric), `Sparkline`, `AreaChart`, `DataTable`, `SegmentedControl`, `Badge`, `Avatar`, `Button`, `Tooltip` | home hero, stories: cards, charts, tables, badges, avatars, segmented-controls |
| `SettingsForm` | `Fieldset`, `TextInput`, `Select`, `Switch`, `Slider`, `RadioGroup`, `Checkbox`, `ColorInput`, `Button` (one Primary), `Alert` info | home, stories: inputs, select, switch, slider, radio, checkbox, fieldset, form, color-picker |
| `InboxPane` | `List`/`ListItem`, `Avatar`, `Badge`, `Indicator`, `ContextMenu`, `Dropdown`, `Tabs`, `Skeleton` (loading row), `Pagination` | home, stories: lists, indicator, context-menu, dropdown, tabs, pagination, skeletons |
| `AuthCard` | `Card`, `TextInput` (password with right icon), `PinInput`, `Checkbox`, `Button`, `Divider` with label, `Alert` danger (wrong code) | home, stories: pin-input, dividers, alerts, modals (opens in a Modal) |

Scenes take no props except an optional `compact: bool`. They use realistic Italian content (a fictional "Shadowfenix Mesh" ops console: nodi, latenza, commit, utenti), no lorem ipsum, no "Zero-Knowledge Validated" filler.

### 1.4 Density, rhythm, type

- Page column: `--sc-page-w: 72rem` for stories, `80rem` for home (`.sc-page--wide`). Content is no longer a 56rem strip in a 1440 viewport.
- Vertical rhythm: sections are separated by `--sd-space-10`; inside a section, head → body gap `--sd-space-6`; grids `--sd-space-4`.
- Section head (`SectionHead`): eyebrow (mono 2xs uppercase subtle), title (Kanit `--sd-text-xl`, weight 600), optional one-line description muted. Same component on every page, including home and Tokens.
- Three text levels only (`--sd-text`, `-muted`, `-subtle`). Body `--sd-text-md`. Code inline through the library `Code`.
- Stage heights are driven by content, with a floor of `12rem` (`Stage::Center/Start`), `20rem` for `Stage::Fill` (layout demos). Never a huge black box around a 32px widget: `Stage::Form` caps the column at `24rem` and centres it.
- Static demo tiles (`DemoCell`, kept) stay dashed hairline, mono label, no hover.

### 1.5 Motion moments

All motion comes from library tokens/mixins or existing `animate-*` utilities.

1. Knob flip: the generated code re-renders and `CodeBlock { highlight_lines }` marks the changed line; the library handles the highlight style.
2. Preset click: everything re-colours through CSS variables; no extra animation.
3. Home scenes tab switch: `TabPanel` content gets `class: "animate-fade-in"`.
4. Floating layers (theme popover, search palette, drawer) use the library's own enter animations.

### 1.6 Mobile (390)

- Header is one row, 56px: burger (`Burger`), brand (logo + name), search icon button, theme dot button. Nothing wraps.
- The mobile `Drawer` holds a `TextInput` filter at the top (left icon `Search`) that filters the sidebar list live, then the categories; the active item is scrolled into view (keep the current `document::eval` snippet, scoped to the drawer).
- Playground: stage above knobs (single column). Knob row groups wrap. Code block scrolls horizontally inside its own container, never the page.
- Preset wall becomes a horizontal `ScrollArea { scroll_x: true }` of fixed-width tiles (`16rem`).
- Scenes: `OpsDashboard` compact mode stacks metric cards 1-col and hides the table's two least important columns via `DataTableColumn` selection (`compact: true` passes a shorter `columns` vec).
- Acceptance: `document.documentElement.scrollWidth == 390` on home, buttons, select, tables, charts.

---

## 2. Information architecture

### 2.1 Navigation

Sidebar (desktop, `md+`) and `Drawer` (below `md`) render the same `SidebarNav`, which iterates `Category::ALL` and `registry::by_category`. Foundation pages are listed first from the static `FOUNDATION` table (`pages/mod.rs`). Category headers are the mono eyebrow with the category icon. Items are router `Link`s with the library classes `theme().sidebar_link` / `sidebar_link_active` (the library's `sd-sidebar-link`), label = `story.name`. The "Tema attivo" card at the bottom is kept but rebuilt on `Card`.

Categories (8), order and membership (slug list is the truth, see §6 for files):

| Category | `Category::` | Icon | Members (slugs) |
| :-- | :-- | :-- | :-- |
| Fondamenta | `Fondamenta` | `Sparkles` | home, theme-builder, tokens, typography, icons (pages) + `animations` (story) |
| Azioni & Form | `Form` | `Terminal` | buttons, inputs, number-input, pin-input, textarea, select, checkbox, radio, switch, slider, rating, chip, color-picker, segmented-controls, date-picker, dropzone, fieldset, form |
| Layout | `Layout` | `Box` | app-shell, navbar, grid, group, stack, center, aspect-ratio, cards, scroll-area, dividers, resizable, masonry, accordion, spoiler |
| Navigazione | `Navigazione` | `ChevronRight` | navigation-menu, overflow-set, nav-link, tabs, breadcrumbs, pagination, stepper, bottom-navigation, menubar, burger, command-palette |
| Overlay | `Overlay` | `Layers` | modals, alert-dialog, drawer, dropdown, popover, hover-card, context-menu, tooltip, coachmark, speed-dial, affix, keytip |
| Dati & Feedback | `Dati` | `Info` | tables, lists, timelines, badges, alerts, notifications, progress, skeletons, trees, indicator, loading-overlay, virtual-list, transfer-list, carousel, charts |
| Contenuti & Social | `Contenuti` | `MessageSquare` | avatars, facepile, chat, activity-feed, attachment-cards, document-card, media-card, rich-text, code-highlight |
| Utility | `Utility` | `Settings` | close-button, copy-button, highlight, marquee-selection, reactive-hooks |

Slugs are the **existing URL paths** without the slash. Every old URL keeps working. Story names are the Rust component names ("Button", "TextInput", "Select", "DataTable", "Modal", "Tree", "Chat", "RichText"...).

### 2.2 Search

`CommandPalette` (Ctrl+K and the header button) is fed from: every story (`id = "story:<slug>"`, category = category label, keywords = `story.keywords` + features), the foundation pages (`page:<route>`), and theme actions (`theme:preset:<name>`, `theme:radius:<sharp|medium|soft>`, `theme:glow:<off|subtle|expressive>`, category "Tema"). Selecting a theme action applies it and shows a `use_notification().success(...)` toast: the palette demonstrates itself.

### 2.3 Story page anatomy (`StoryPage`)

Top to bottom; a section renders only if its data is non-empty.

1. **Head**: `Breadcrumbs` (Componenti / category / name), icon tile + `h1.sc-title` (Kanit) + tagline, feature `Badge`s (pill) and a `Beta` badge when applicable, then the generated Cargo line in a compact `CodeBlock { language: "toml", filename: "Cargo.toml", with_line_numbers: false }`.
2. **Playground** (`id="playground"`): `SectionHead` + 2-column grid (lg: stage 7/12, knobs 5/12). Stage holds the live component, immediately followed by the generated `CodeBlock` (`language: "rsx"`, highlighted changed line). Knobs come from `KnobPanel` (library `SegmentedControl`/`Select`/`Switch`/`TextInput`/`Slider`/`ColorInput`). Panel head shows `ThemeSwatches { compact: true }`.
3. **Esempi**: grid of `ExampleCard`s (`Card` with title/subtitle; stage; footer button "Codice" toggling a `Collapsible` with the `CodeBlock`). `Span::Full` cards take the whole row (scenes).
4. **Ricetta & accessibilità**: two columns. Left: recipe (`Ricetta Lit Glass`, depth level, token chips as `Code`, note, link text "language.md §3.x"). Right: a11y bullets in a library `List` with `CheckCircle` icons.
5. **API**: one `PropsTable` per `PropGroup`; with more than one group they sit in `Tabs`.
6. **Correlati**: `NavLink`s (icon, name, tagline) navigating by slug.
7. **Prev / Next**: two Secondary `Button`s with chevrons within the registry order.

A `Status::Stub` story renders head + an `Alert` info "Storia in migrazione" and nothing else, so the app always builds.

### 2.4 Non-component pages

| Route | Page | Must do |
| :-- | :-- | :-- |
| `/` | `Home` | §1.1–1.3. Counters computed from the registry: `registry::all().len()` componenti, `registry::feature_count()` feature Cargo, "0 dipendenze JS", fonts "3". Install code uses `VERSION`. |
| `/theme-builder` | `ThemeBuilderPage` | Keep features; move its own tabs/controls onto `SectionHead` + `Card` grid; fix the two layout bugs (checkbox label squeezed to one word per line, uneven preview card heights: give `.sc-builder-grid > *` `min-width: 0` and let labels wrap in the Switch/Checkbox label slot). Its live preview becomes `scenes::SettingsForm { compact: true }` + the preset wall tile of the current theme. |
| `/tokens` | `TokensPage` (new) | Every token group from `docs/design/tokens.md` rendered live: accent (swatches + derived `strong/deep/soft/border/fg` chips, computed by reading `use_theme().primary_color.accent_tokens()`), surfaces (nested boxes), semantic (four `Alert`s), shadows (three `Card`s with `shadow: Sm/Md/Lg`), radius row, glow row (§1.2), size scale (five `Button` sizes next to a ruler), type scale (`Heading`/`Text`), motion (three durations as `ProgressBar` indeterminate? no: three `Button`s whose hover demonstrates `--sd-dur-fast`; keep it simple). Each group has a `CodeBlock` with the token names. |
| `/typography` | `TypographyPage` | Three roles side by side (Kanit / Onest / Source Code Pro) with the actual token names; a specimen block per role; `Kbd`/`Code` row. |
| `/icons` | `IconsPage` | Keep search + size slider; grid of `Card`s is replaced by a `SimpleGrid` of static tiles (`.sc-icon-tile`, not clickable-looking) with a `CopyButton { icon_only: true }` per tile for `IconKind::Name`. |
| `/:slug` | `StoryPage` | §2.3 |
| `/*` | `NotFound` | `Alert` warning + `Button` to home + 5 suggested stories. |

---

## 3. Code structure

```text
showcase/
├── assets/showcase.scss            # .sc-* chrome; tokens + library mixins only
├── legacy/pages/*.rs               # phase 1 moves the 85 old pages here (NOT compiled); batch engineers read then delete
└── src/
    ├── main.rs                     # Route (7 variants), App, assets
    ├── app/                        # chrome, library components only
    │   ├── mod.rs
    │   ├── layout.rs               # Layout: header + sidebar/drawer + Outlet
    │   ├── header.rs               # brand, search button, ThemeMenuButton
    │   ├── theme_menu.rs           # ThemeSwatches, ThemeControls (radius/glow), ThemeMenuButton (Popover)
    │   ├── sidebar.rs              # SidebarNav (desktop + drawer), SidebarLink, theme card
    │   └── search.rs               # SearchPalette (CommandPalette fed by registry + theme actions)
    ├── story/                      # the engine
    │   ├── mod.rs                  # pub mod schema; pub mod knobs; pub mod controls; pub mod registry; pub mod page; pub mod sections;
    │   ├── schema.rs               # Story, Playground, Control, Example, PropDoc, Recipe, Category ... (data only)
    │   ├── knobs.rs                # Knobs state + generate() code generator
    │   ├── controls.rs             # KnobPanel and the five knob components
    │   ├── registry.rs             # STORIES (LazyLock), find/by_category/neighbours + tests
    │   ├── page.rs                 # StoryPage, StoryView, LivePlayground
    │   └── sections.rs             # SectionHead, ExampleCard, PropsTable, NotesSection, RelatedSection, PrevNext, DemoCell
    ├── stories/
    │   ├── mod.rs                  # pub mod <slug_snake>; (85) + pub const ALL: &[fn() -> Story]
    │   └── <slug_snake>.rs         # one per story: pub fn story() -> Story (+ private #[component] helpers)
    ├── scenes/
    │   ├── mod.rs                  # pub use
    │   ├── ops_dashboard.rs, settings_form.rs, inbox_pane.rs, auth_card.rs
    ├── pages/
    │   ├── mod.rs                  # pub use + FOUNDATION table
    │   ├── home.rs, tokens.rs, theme_builder.rs, typography.rs, icons.rs, not_found.rs
    └── components/
        └── clipboard.rs            # kept (copy_text) for theme builder
```

Deleted: `components/{showcase_tabs,props_table,banner,controls,nav,layout}.rs`, `pages/<component>.rs` (85), the `use pages::{…}` list.

### 3.1 Routing (`main.rs`)

Static routes are declared before the dynamic one; the Dioxus router matches static segments first, but keeping them first in the enum also keeps the intent visible.

```rust
use dioxus::prelude::*;
use shadow_design::prelude::*;
pub use shadow_design::VERSION;

mod app;
mod components;
mod pages;
mod scenes;
mod stories;
mod story;

use app::Layout;
use pages::{Home, IconsPage, NotFound, ThemeBuilderPage, TokensPage, TypographyPage};
use story::page::StoryPage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/theme-builder")]
        ThemeBuilderPage {},
        #[route("/tokens")]
        TokensPage {},
        #[route("/typography")]
        TypographyPage {},
        #[route("/icons")]
        IconsPage {},
        #[route("/:slug")]
        StoryPage { slug: String },
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}

impl Route {
    pub fn story(slug: &str) -> Self {
        Route::StoryPage { slug: slug.to_string() }
    }
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const STYLES: Asset = shadow_design::STYLES;
pub const SHOWCASE_STYLES: Asset = asset!("/assets/showcase.scss");
pub const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let locale = use_signal(shadow_design::locale::Locale::italian);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link { rel: "stylesheet", href: STYLES }
        document::Link { rel: "stylesheet", href: SHOWCASE_STYLES }
        shadow_design::locale::LocaleProvider { locale: Some(locale),
            ThemeProvider { persist_key: "shadow_design_custom_theme".to_string(),
                NotificationProvider { Router::<Route> {} }
            }
        }
    }
}
```

`pages/mod.rs` exposes the foundation table used by sidebar, search and home:

```rust
use crate::Route;
use shadow_design::prelude::IconKind;

pub struct FoundationPage { pub name: &'static str, pub route: Route, pub icon: IconKind, pub blurb: &'static str }

pub const FOUNDATION: &[FoundationPage] = &[
    FoundationPage { name: "Panoramica",    route: Route::Home {},             icon: IconKind::Sparkles, blurb: "Scene reali, preset e installazione." },
    FoundationPage { name: "Theme Builder", route: Route::ThemeBuilderPage {}, icon: IconKind::Palette,  blurb: "Progetta il tema e copia il codice Rust." },
    FoundationPage { name: "Tokens",        route: Route::TokensPage {},       icon: IconKind::Layers,   blurb: "Ogni variabile --sd-* resa dal vivo." },
    FoundationPage { name: "Typography",    route: Route::TypographyPage {},   icon: IconKind::Code,     blurb: "Kanit, Onest, Source Code Pro." },
    FoundationPage { name: "Icons",         route: Route::IconsPage {},        icon: IconKind::Search,   blurb: "54 icone SVG native, stroke 1.5." },
];
```

### 3.2 How a knob feeds the component and the code

1. A `Control` declares `key`, `kind`, `default` and `emit`.
2. `LivePlayground` owns `Signal<Knobs>` (one value per control, in declaration order) and `Signal<Option<&'static str>>` for the last changed key.
3. `KnobPanel` renders one library control per `Control` and writes `knobs.write().set(key, Value)` on change.
4. `playground.render(&knobs)` builds the live element. Authors read values with `k.pick("variant", &VARIANTS)`, `k.bool("loading")`, `k.text("label")`, `k.number("step")`, `k.color("accent")`.
5. `knobs::generate(&playground, &knobs, changed)` writes the RSX sample: prelude lines, `Component {`, fixed lines, one line per control according to `Emit`, children template with `{key}` placeholders substituted, `}`. It also returns the 1-indexed line of the changed key, passed to `CodeBlock.highlight_lines`.

Stateful components (Select, Modal, Switch, Tabs...) cannot create signals inside `render` (it is a plain `fn`, not a component). The standard pattern is a private `#[component] fn XDemo(knobs: Knobs)` that owns the signals, and `fn render(k: &Knobs) -> Element { rsx! { XDemo { knobs: k.clone() } } }`. `Knobs` is `Clone + PartialEq`, so it is a valid prop. See the Select story (§4.8).

### 3.3 Parallel work without shared files

- Phase 1 writes `stories/mod.rs` **complete**: all 85 `pub mod` lines and all 85 `ALL` entries, in sidebar order, each module a `Story::stub(...)`.
- A batch engineer edits only `showcase/src/stories/<slug>.rs` files listed in their batch and deletes `showcase/legacy/pages/<old>.rs` for each. If a story needs a helper component, it is private to that file. If it needs a scene variant, it composes the scene with `compact` or inlines its own composition; scenes are read-only for batches.
- Missing shared vocabulary (a new `.sc-*` class, a new `Kind`) is reported to the phase-1 engineer as a one-line request; the batch engineer does not edit `showcase.scss`, `story/*`, `scenes/*`, `app/*` or `pages/*`.

---

## 4. Snippets

### 4.1 `story/schema.rs`

```rust
//! Story schema (ADR-0006). Data only: no hooks, no rendering here.
use dioxus::prelude::*;
use shadow_design::prelude::IconKind;

pub use crate::story::knobs::Knobs;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Category { Fondamenta, Form, Layout, Navigazione, Overlay, Dati, Contenuti, Utility }

impl Category {
    pub const ALL: [Category; 8] = [
        Category::Fondamenta, Category::Form, Category::Layout, Category::Navigazione,
        Category::Overlay, Category::Dati, Category::Contenuti, Category::Utility,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Category::Fondamenta => "Fondamenta",
            Category::Form => "Azioni & Form",
            Category::Layout => "Layout",
            Category::Navigazione => "Navigazione",
            Category::Overlay => "Overlay",
            Category::Dati => "Dati & Feedback",
            Category::Contenuti => "Contenuti & Social",
            Category::Utility => "Utility",
        }
    }

    pub fn icon(self) -> IconKind {
        match self {
            Category::Fondamenta => IconKind::Sparkles,
            Category::Form => IconKind::Terminal,
            Category::Layout => IconKind::Box,
            Category::Navigazione => IconKind::ChevronRight,
            Category::Overlay => IconKind::Layers,
            Category::Dati => IconKind::Info,
            Category::Contenuti => IconKind::MessageSquare,
            Category::Utility => IconKind::Settings,
        }
    }

    pub fn blurb(self) -> &'static str {
        match self {
            Category::Fondamenta => "Tema, token, tipografia e icone.",
            Category::Form => "Pulsanti e campi: tutto ciò che riceve input.",
            Category::Layout => "Shell, griglie, stack e contenitori.",
            Category::Navigazione => "Tab, breadcrumb, paginazione, menu e palette.",
            Category::Overlay => "Finestre, pannelli e menu flottanti.",
            Category::Dati => "Tabelle, liste, stati e grafici.",
            Category::Contenuti => "Avatar, chat, feed e schede media.",
            Category::Utility => "Micro-componenti e hook reattivi.",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status { Stable, Beta, Stub }

/// Runtime value of one knob.
#[derive(Clone, PartialEq, Debug)]
pub enum Value { Bool(bool), Choice(usize), Text(String), Number(f64), Color(String) }

/// One option of a `Kind::Choice`: what the user sees and what the code sample prints.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ChoiceOption { pub label: &'static str, pub code: &'static str }

pub const fn opt(label: &'static str, code: &'static str) -> ChoiceOption {
    ChoiceOption { label, code }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind {
    Bool,
    Choice(&'static [ChoiceOption]),
    Text,
    /// `decimals` controls the printed literal: 0 → `20`, 1 → `1.0`.
    Number { min: f64, max: f64, step: f64, decimals: u8 },
    Color,
}

/// How a knob appears in the generated code.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Emit {
    /// `key: <value>,` on every render.
    Always,
    /// `key: <value>,` only when the value differs from the default.
    IfChanged,
    /// A template emitted when the value differs from the default (a `Bool` only when true).
    /// `{v}` is replaced by the raw value (text unquoted, choice code, number, bool).
    Template(&'static str),
    /// Never printed: the knob only shapes the demo (stage width, sample text...).
    Skip,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Control {
    pub key: &'static str,
    pub label: &'static str,
    pub kind: Kind,
    pub default: Value,
    pub emit: Emit,
    pub hint: Option<&'static str>,
}

impl Control {
    pub fn bool(key: &'static str, label: &'static str, default: bool) -> Self {
        Self { key, label, kind: Kind::Bool, default: Value::Bool(default), emit: Emit::IfChanged, hint: None }
    }
    pub fn choice(key: &'static str, label: &'static str, options: &'static [ChoiceOption], default: usize) -> Self {
        Self { key, label, kind: Kind::Choice(options), default: Value::Choice(default), emit: Emit::IfChanged, hint: None }
    }
    pub fn text(key: &'static str, label: &'static str, default: &str) -> Self {
        Self { key, label, kind: Kind::Text, default: Value::Text(default.to_string()), emit: Emit::IfChanged, hint: None }
    }
    pub fn number(key: &'static str, label: &'static str, default: f64, min: f64, max: f64, step: f64, decimals: u8) -> Self {
        Self { key, label, kind: Kind::Number { min, max, step, decimals }, default: Value::Number(default), emit: Emit::IfChanged, hint: None }
    }
    pub fn color(key: &'static str, label: &'static str, default: &str) -> Self {
        Self { key, label, kind: Kind::Color, default: Value::Color(default.to_string()), emit: Emit::IfChanged, hint: None }
    }
    pub fn emit(mut self, emit: Emit) -> Self { self.emit = emit; self }
    pub fn hint(mut self, hint: &'static str) -> Self { self.hint = Some(hint); self }
}

/// How the live component sits in the stage.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Stage {
    /// Centred, content-sized (buttons, badges, switches).
    #[default]
    Center,
    /// Top-left, full width (lists, tables, navigation).
    Start,
    /// A centred 24rem column (fields, selects, cards).
    Form,
    /// Full width and at least 20rem tall (layout and overlay demos).
    Fill,
}

#[derive(Clone, PartialEq)]
pub struct Playground {
    /// Rust component name printed as the block head: `Button {`.
    pub component: &'static str,
    pub controls: Vec<Control>,
    pub render: fn(&Knobs) -> Element,
    /// Lines printed before the block (signals, data). Verbatim.
    pub prelude_code: Option<&'static str>,
    /// Lines printed first inside the block, always (e.g. `data: data.clone(),`, `value,`).
    pub fixed_code: &'static [&'static str],
    /// Children printed last inside the block. `{key}` placeholders take the raw knob value.
    pub children_code: Option<&'static str>,
    pub stage: Stage,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Span { #[default] Half, Full }

#[derive(Clone, PartialEq)]
pub struct Example {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    /// Must be the exact RSX of `render`'s body (migration checklist §5, item 6).
    pub code: &'static str,
    pub render: fn() -> Element,
    pub span: Span,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PropDoc { pub name: &'static str, pub ty: &'static str, pub default: &'static str, pub doc: &'static str }

pub const fn prop(name: &'static str, ty: &'static str, default: &'static str, doc: &'static str) -> PropDoc {
    PropDoc { name, ty, default, doc }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PropGroup { pub component: &'static str, pub rows: &'static [PropDoc] }

/// Which Lit Glass recipe the component implements (docs/design/language.md §3).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Recipe {
    /// e.g. "Raised control (§3.5)"
    pub recipe: &'static str,
    /// e.g. "Lit control" (depth ladder §2)
    pub depth: &'static str,
    pub tokens: &'static [&'static str],
    pub notes: &'static str,
}

#[derive(Clone, PartialEq)]
pub struct Story {
    /// URL segment, kebab-case, equal to the old route path.
    pub slug: &'static str,
    /// Rust component name.
    pub name: &'static str,
    pub category: Category,
    pub features: &'static [&'static str],
    pub icon: IconKind,
    /// One Italian sentence.
    pub tagline: &'static str,
    pub keywords: &'static [&'static str],
    pub status: Status,
    pub playground: Option<Playground>,
    pub examples: Vec<Example>,
    pub props: Vec<PropGroup>,
    pub recipe: Option<Recipe>,
    pub a11y: &'static [&'static str],
    /// Slugs of related stories.
    pub related: &'static [&'static str],
}

impl Story {
    /// Phase-1 placeholder. Renders head + "Storia in migrazione".
    pub fn stub(slug: &'static str, name: &'static str, category: Category, features: &'static [&'static str], icon: IconKind, tagline: &'static str) -> Self {
        Self {
            slug, name, category, features, icon, tagline,
            keywords: &[], status: Status::Stub, playground: None, examples: Vec::new(),
            props: Vec::new(), recipe: None, a11y: &[], related: &[],
        }
    }

    pub fn cargo_line(&self) -> String {
        let feats = self.features.iter().map(|f| format!("\"{f}\"")).collect::<Vec<_>>().join(", ");
        format!("shadow-design = {{ version = \"{}\", default-features = false, features = [{feats}] }}", shadow_design::VERSION)
    }
}
```

### 4.2 `story/knobs.rs`

```rust
//! Knob state and the RSX code generator.
use super::schema::{Control, Emit, Kind, Playground, Value};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Knobs {
    values: Vec<(&'static str, Value)>,
}

impl Knobs {
    pub fn from_controls(controls: &[Control]) -> Self {
        Self { values: controls.iter().map(|c| (c.key, c.default.clone())).collect() }
    }
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.iter().find(|(k, _)| *k == key).map(|(_, v)| v)
    }
    pub fn set(&mut self, key: &'static str, value: Value) {
        match self.values.iter_mut().find(|(k, _)| *k == key) {
            Some(slot) => slot.1 = value,
            None => self.values.push((key, value)),
        }
    }
    pub fn bool(&self, key: &str) -> bool { matches!(self.get(key), Some(Value::Bool(true))) }
    pub fn index(&self, key: &str) -> usize {
        match self.get(key) { Some(Value::Choice(i)) => *i, _ => 0 }
    }
    /// Maps a `Choice` knob to a typed table declared in the same order as its options.
    pub fn pick<T: Copy>(&self, key: &str, table: &[T]) -> T {
        table[self.index(key).min(table.len().saturating_sub(1))]
    }
    pub fn text(&self, key: &str) -> String {
        match self.get(key) { Some(Value::Text(s)) => s.clone(), _ => String::new() }
    }
    pub fn number(&self, key: &str) -> f64 {
        match self.get(key) { Some(Value::Number(n)) => *n, _ => 0.0 }
    }
    pub fn color(&self, key: &str) -> String {
        match self.get(key) { Some(Value::Color(s)) => s.clone(), _ => String::new() }
    }
}

pub struct Generated {
    pub code: String,
    /// 1-indexed lines to highlight (the knob changed last).
    pub highlight: Vec<usize>,
}

/// Raw value for `{v}` / `{key}` substitution: text unquoted, choice code, number, bool.
fn raw(value: &Value, kind: &Kind) -> String {
    match (value, kind) {
        (Value::Bool(b), _) => b.to_string(),
        (Value::Choice(i), Kind::Choice(opts)) => opts.get(*i).map(|o| o.code.to_string()).unwrap_or_default(),
        (Value::Text(s), _) | (Value::Color(s), _) => s.replace('\\', "\\\\").replace('"', "\\\""),
        (Value::Number(n), Kind::Number { decimals, .. }) => format!("{:.*}", *decimals as usize, n),
        (Value::Number(n), _) => n.to_string(),
        _ => String::new(),
    }
}

/// Literal as it appears after `key:` (text and colour quoted).
fn literal(value: &Value, kind: &Kind) -> String {
    match value {
        Value::Text(_) | Value::Color(_) => format!("\"{}\"", raw(value, kind)),
        _ => raw(value, kind),
    }
}

pub fn generate(pg: &Playground, knobs: &Knobs, changed: Option<&str>) -> Generated {
    let mut lines: Vec<String> = Vec::new();
    let mut highlight = Vec::new();

    if let Some(prelude) = pg.prelude_code {
        lines.extend(prelude.lines().map(str::to_string));
        lines.push(String::new());
    }
    lines.push(format!("{} {{", pg.component));
    for fixed in pg.fixed_code {
        lines.push(format!("    {fixed}"));
    }
    for c in &pg.controls {
        let Some(v) = knobs.get(c.key) else { continue };
        let is_default = *v == c.default;
        let line = match c.emit {
            Emit::Skip => None,
            Emit::Always => Some(format!("{}: {},", c.key, literal(v, &c.kind))),
            Emit::IfChanged if is_default => None,
            Emit::IfChanged => Some(format!("{}: {},", c.key, literal(v, &c.kind))),
            Emit::Template(_) if is_default || matches!(v, Value::Bool(false)) => None,
            Emit::Template(t) => Some(format!("{},", t.replace("{v}", &raw(v, &c.kind)))),
        };
        if let Some(l) = line {
            if changed == Some(c.key) {
                highlight.push(lines.len() + 1);
            }
            lines.push(format!("    {l}"));
        }
    }
    if let Some(children) = pg.children_code {
        let mut text = children.to_string();
        for c in &pg.controls {
            if let Some(v) = knobs.get(c.key) {
                text = text.replace(&format!("{{{}}}", c.key), &raw(v, &c.kind));
            }
        }
        for l in text.lines() {
            lines.push(format!("    {l}"));
        }
    }
    lines.push("}".to_string());
    Generated { code: lines.join("\n"), highlight }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::schema::{opt, Stage};
    use dioxus::prelude::*;

    fn pg() -> Playground {
        Playground {
            component: "Button",
            controls: vec![
                Control::choice("variant", "Variante", &[opt("Primary", "ButtonVariant::Primary"), opt("Ghost", "ButtonVariant::Ghost")], 0),
                Control::bool("loading", "Loading", false),
                Control::bool("left_icon", "Icona", false).emit(Emit::Template("left_icon: Some(rsx! { Icon { kind: IconKind::Check, size: 15 } })")),
                Control::text("label", "Etichetta", "Salva").emit(Emit::Skip),
            ],
            render: |_| rsx! { "" },
            prelude_code: None,
            fixed_code: &[],
            children_code: Some("\"{label}\""),
            stage: Stage::Center,
        }
    }

    #[test]
    fn defaults_print_only_children() {
        let p = pg();
        let g = generate(&p, &Knobs::from_controls(&p.controls), None);
        assert_eq!(g.code, "Button {\n    \"Salva\"\n}");
    }

    #[test]
    fn changed_knob_is_printed_and_highlighted() {
        let p = pg();
        let mut k = Knobs::from_controls(&p.controls);
        k.set("variant", Value::Choice(1));
        k.set("left_icon", Value::Bool(true));
        let g = generate(&p, &k, Some("left_icon"));
        assert!(g.code.contains("    variant: ButtonVariant::Ghost,"));
        assert!(g.code.contains("    left_icon: Some(rsx! { Icon { kind: IconKind::Check, size: 15 } }),"));
        assert_eq!(g.highlight, vec![3]);
    }
}
```

### 4.3 `story/controls.rs` (knob panel)

```rust
//! Knob panel: one library control per `Control`. Pages never style controls by hand.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use super::knobs::Knobs;
use super::schema::{Control, Kind, Value};

/// Above this many options a choice knob is a `Select` instead of a `SegmentedControl`.
const SEGMENT_MAX: usize = 5;

#[component]
pub fn KnobPanel(controls: Vec<Control>, knobs: Signal<Knobs>, changed: Signal<Option<&'static str>>) -> Element {
    rsx! {
        div { class: "sc-knobs",
            for c in controls {
                Knob { key: "{c.key}", control: c, knobs, changed }
            }
        }
    }
}

#[component]
fn Knob(control: Control, knobs: Signal<Knobs>, changed: Signal<Option<&'static str>>) -> Element {
    let kind = control.kind;
    match kind {
        Kind::Bool => rsx! { BoolKnob { control, knobs, changed } },
        Kind::Choice(_) => rsx! { ChoiceKnob { control, knobs, changed } },
        Kind::Text => rsx! { TextKnob { control, knobs, changed } },
        Kind::Number { .. } => rsx! { NumberKnob { control, knobs, changed } },
        Kind::Color => rsx! { ColorKnob { control, knobs, changed } },
    }
}

#[component]
fn BoolKnob(control: Control, mut knobs: Signal<Knobs>, mut changed: Signal<Option<&'static str>>) -> Element {
    let key = control.key;
    let mut checked = use_signal(|| knobs.peek().bool(key));
    use_effect(move || {
        let v = knobs().bool(key);
        if *checked.peek() != v {
            checked.set(v);
        }
    });
    rsx! {
        Switch {
            checked,
            label: Some(control.label.to_string()),
            description: control.hint.map(str::to_string),
            on_change: move |v: bool| {
                knobs.write().set(key, Value::Bool(v));
                changed.set(Some(key));
            },
        }
    }
}

#[component]
fn ChoiceKnob(control: Control, mut knobs: Signal<Knobs>, mut changed: Signal<Option<&'static str>>) -> Element {
    let key = control.key;
    let options = match control.kind { Kind::Choice(o) => o, _ => &[] };
    let mut idx = use_signal(|| knobs.peek().index(key).to_string());
    let mut sel = use_signal(|| Some(knobs.peek().index(key).to_string()));
    use_effect(move || {
        let i = knobs().index(key).to_string();
        if *idx.peek() != i {
            idx.set(i.clone());
        }
        if sel.peek().as_deref() != Some(i.as_str()) {
            sel.set(Some(i));
        }
    });
    let mut pick = move |s: String| {
        if let Ok(i) = s.parse::<usize>() {
            knobs.write().set(key, Value::Choice(i));
            changed.set(Some(key));
        }
    };
    rsx! {
        div { class: "sc-knob",
            span { class: "sc-eyebrow", "{control.label}" }
            if options.len() > SEGMENT_MAX {
                Select {
                    data: options.iter().enumerate().map(|(i, o)| SelectItem::new(i.to_string(), o.label)).collect::<Vec<_>>(),
                    value: sel,
                    on_change: move |v: Option<String>| { if let Some(v) = v { pick(v); } },
                }
            } else {
                SegmentedControl {
                    size: SegmentedControlSize::Sm,
                    options: options.iter().enumerate().map(|(i, o)| SegmentOption::new(i.to_string(), o.label)).collect::<Vec<_>>(),
                    value: idx,
                    on_change: move |v: String| pick(v),
                }
            }
            if let Some(h) = control.hint {
                span { class: "sc-hint", "{h}" }
            }
        }
    }
}

#[component]
fn TextKnob(control: Control, mut knobs: Signal<Knobs>, mut changed: Signal<Option<&'static str>>) -> Element {
    let key = control.key;
    let text = use_signal(|| knobs.peek().text(key));
    use_effect(move || {
        let v = text();
        if knobs.peek().text(key) != v {
            knobs.write().set(key, Value::Text(v));
            changed.set(Some(key));
        }
    });
    rsx! {
        TextInput {
            value: text,
            label: Some(control.label.to_string()),
            help_text: control.hint.map(str::to_string),
            clearable: true,
        }
    }
}

#[component]
fn NumberKnob(control: Control, mut knobs: Signal<Knobs>, mut changed: Signal<Option<&'static str>>) -> Element {
    let key = control.key;
    let (min, max, step, decimals) = match control.kind {
        Kind::Number { min, max, step, decimals } => (min, max, step, decimals),
        _ => (0.0, 100.0, 1.0, 0),
    };
    let value = use_signal(|| knobs.peek().number(key));
    let shown = format!("{:.*}", decimals as usize, value());
    rsx! {
        div { class: "sc-knob",
            div { class: "sc-knob__row",
                span { class: "sc-eyebrow", "{control.label}" }
                span { class: "sc-readout", "{shown}" }
            }
            Slider {
                value,
                min, max, step,
                show_tooltip: false,
                on_change: move |v: f64| {
                    knobs.write().set(key, Value::Number(v));
                    changed.set(Some(key));
                },
            }
        }
    }
}

#[component]
fn ColorKnob(control: Control, mut knobs: Signal<Knobs>, mut changed: Signal<Option<&'static str>>) -> Element {
    let key = control.key;
    let value = use_signal(|| knobs.peek().color(key));
    rsx! {
        ColorInput {
            value,
            label: Some(control.label.to_string()),
            description: control.hint.map(str::to_string),
            on_change: move |v: String| {
                knobs.write().set(key, Value::Color(v));
                changed.set(Some(key));
            },
        }
    }
}
```

### 4.4 `story/registry.rs`

```rust
//! Static registry built once from `stories::ALL`. Order = sidebar order.
use std::sync::LazyLock;

use super::schema::{Category, Status, Story};

static STORIES: LazyLock<Vec<Story>> = LazyLock::new(|| crate::stories::ALL.iter().map(|f| f()).collect());

pub fn all() -> &'static [Story] { &STORIES }

pub fn find(slug: &str) -> Option<&'static Story> { STORIES.iter().find(|s| s.slug == slug) }

pub fn by_category(category: Category) -> Vec<&'static Story> {
    STORIES.iter().filter(|s| s.category == category).collect()
}

pub fn neighbours(slug: &str) -> (Option<&'static Story>, Option<&'static Story>) {
    let Some(i) = STORIES.iter().position(|s| s.slug == slug) else { return (None, None) };
    (i.checked_sub(1).and_then(|j| STORIES.get(j)), STORIES.get(i + 1))
}

/// Distinct Cargo features referenced by stories (for the home counters).
pub fn feature_count() -> usize {
    let mut set = std::collections::BTreeSet::new();
    for s in STORIES.iter() {
        for f in s.features {
            set.insert(*f);
        }
    }
    set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CRATE_MANIFEST: &str = include_str!("../../../crate/Cargo.toml");

    fn feature_exists(f: &str) -> bool {
        CRATE_MANIFEST.lines().any(|l| l.trim_start().starts_with(&format!("{f} =")) || l.trim_start().starts_with(&format!("{f}=")))
    }

    #[test]
    fn slugs_are_unique_and_kebab() {
        let mut seen = std::collections::BTreeSet::new();
        for s in all() {
            assert!(seen.insert(s.slug), "duplicate slug {}", s.slug);
            assert!(s.slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'), "slug {} is not kebab-case", s.slug);
        }
    }

    #[test]
    fn related_slugs_resolve() {
        for s in all() {
            for r in s.related {
                assert!(find(r).is_some(), "{}: related slug {r} does not exist", s.slug);
                assert_ne!(*r, s.slug, "{}: relates to itself", s.slug);
            }
        }
    }

    #[test]
    fn features_exist_in_crate_manifest() {
        for s in all() {
            for f in s.features {
                assert!(feature_exists(f), "{}: feature `{f}` is not in crate/Cargo.toml", s.slug);
            }
        }
    }

    #[test]
    fn non_stub_stories_are_complete() {
        for s in all().iter().filter(|s| s.status != Status::Stub) {
            assert!(s.playground.is_some() || s.examples.len() >= 2, "{}: needs a playground or two examples", s.slug);
            assert!(!s.props.is_empty(), "{}: needs at least one PropGroup", s.slug);
            assert!(!s.a11y.is_empty(), "{}: needs accessibility notes", s.slug);
            assert!(!s.related.is_empty(), "{}: needs related stories", s.slug);
            assert!(!s.keywords.is_empty(), "{}: needs search keywords", s.slug);
            for e in &s.examples {
                assert!(!e.code.trim().is_empty(), "{}: example {} has empty code", s.slug, e.id);
            }
        }
    }

    #[test]
    fn no_stub_remains() {
        // Phase 3 flips this on by removing the `ignore`.
        let stubs: Vec<_> = all().iter().filter(|s| s.status == Status::Stub).map(|s| s.slug).collect();
        if std::env::var("SHOWCASE_ALLOW_STUBS").is_err() {
            assert!(stubs.is_empty(), "stub stories left: {stubs:?}");
        }
    }
}
```

During phases 1–2 run tests with `SHOWCASE_ALLOW_STUBS=1 cargo test -p showcase`; the verifier runs them without the variable.

### 4.5 `stories/mod.rs` and a stub

```rust
//! Story registry list (ADR-0006). ONE `pub mod` and ONE `ALL` entry per story, in sidebar order.
//! Phase 1 writes this file complete; batch engineers never edit it.
use crate::story::schema::Story;

// Fondamenta
pub mod animations;
// Azioni & Form
pub mod buttons; pub mod inputs; pub mod number_input; pub mod pin_input; pub mod textarea; pub mod select;
pub mod checkbox; pub mod radio; pub mod switch; pub mod slider; pub mod rating; pub mod chip;
pub mod color_picker; pub mod segmented_controls; pub mod date_picker; pub mod dropzone; pub mod fieldset; pub mod form;
// Layout
pub mod app_shell; pub mod navbar; pub mod grid; pub mod group; pub mod stack; pub mod center; pub mod aspect_ratio;
pub mod cards; pub mod scroll_area; pub mod dividers; pub mod resizable; pub mod masonry; pub mod accordion; pub mod spoiler;
// Navigazione
pub mod navigation_menu; pub mod overflow_set; pub mod nav_link; pub mod tabs; pub mod breadcrumbs; pub mod pagination;
pub mod stepper; pub mod bottom_navigation; pub mod menubar; pub mod burger; pub mod command_palette;
// Overlay
pub mod modals; pub mod alert_dialog; pub mod drawer; pub mod dropdown; pub mod popover; pub mod hover_card;
pub mod context_menu; pub mod tooltip; pub mod coachmark; pub mod speed_dial; pub mod affix; pub mod keytip;
// Dati & Feedback
pub mod tables; pub mod lists; pub mod timelines; pub mod badges; pub mod alerts; pub mod notifications; pub mod progress;
pub mod skeletons; pub mod trees; pub mod indicator; pub mod loading_overlay; pub mod virtual_list; pub mod transfer_list;
pub mod carousel; pub mod charts;
// Contenuti & Social
pub mod avatars; pub mod facepile; pub mod chat; pub mod activity_feed; pub mod attachment_cards; pub mod document_card;
pub mod media_card; pub mod rich_text; pub mod code_highlight;
// Utility
pub mod close_button; pub mod copy_button; pub mod highlight; pub mod marquee_selection; pub mod reactive_hooks;

pub const ALL: &[fn() -> Story] = &[
    animations::story,
    buttons::story, inputs::story, number_input::story, pin_input::story, textarea::story, select::story,
    checkbox::story, radio::story, switch::story, slider::story, rating::story, chip::story,
    color_picker::story, segmented_controls::story, date_picker::story, dropzone::story, fieldset::story, form::story,
    app_shell::story, navbar::story, grid::story, group::story, stack::story, center::story, aspect_ratio::story,
    cards::story, scroll_area::story, dividers::story, resizable::story, masonry::story, accordion::story, spoiler::story,
    navigation_menu::story, overflow_set::story, nav_link::story, tabs::story, breadcrumbs::story, pagination::story,
    stepper::story, bottom_navigation::story, menubar::story, burger::story, command_palette::story,
    modals::story, alert_dialog::story, drawer::story, dropdown::story, popover::story, hover_card::story,
    context_menu::story, tooltip::story, coachmark::story, speed_dial::story, affix::story, keytip::story,
    tables::story, lists::story, timelines::story, badges::story, alerts::story, notifications::story, progress::story,
    skeletons::story, trees::story, indicator::story, loading_overlay::story, virtual_list::story, transfer_list::story,
    carousel::story, charts::story,
    avatars::story, facepile::story, chat::story, activity_feed::story, attachment_cards::story, document_card::story,
    media_card::story, rich_text::story, code_highlight::story,
    close_button::story, copy_button::story, highlight::story, marquee_selection::story, reactive_hooks::story,
];
```

Stub (`stories/tooltip.rs`, phase 1 generates one per slug from the old `COMPONENTS` entries: name without decorations, features, icon, description as tagline):

```rust
//! Story: Tooltip (feature `tooltip`). STUB: migrate from legacy/pages/tooltip.rs (§5).
use shadow_design::prelude::IconKind;
use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub("tooltip", "Tooltip", Category::Overlay, &["tooltip"], IconKind::Info,
        "Etichetta flottante al passaggio del puntatore, in quattro direzioni.")
}
```

### 4.6 `story/page.rs` (the template)

```rust
//! The one page template that renders every Story (ADR-0006).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use super::controls::KnobPanel;
use super::knobs::{self, Knobs};
use super::registry;
use super::schema::{Playground, Stage, Status, Story};
use super::sections::{ExamplesSection, NotesSection, PrevNext, PropsSection, RelatedSection, SectionHead};
use crate::app::theme_menu::ThemeSwatches;

#[component]
pub fn StoryPage(slug: String) -> Element {
    match registry::find(&slug) {
        // `key` forces a fresh scope per story: knob signals never leak between stories.
        Some(story) => rsx! { StoryView { key: "{story.slug}", story: story.clone() } },
        None => rsx! { crate::pages::NotFound { segments: vec![slug] } },
    }
}

#[component]
fn StoryView(story: Story) -> Element {
    let (prev, next) = registry::neighbours(story.slug);
    let is_stub = story.status == Status::Stub;
    rsx! {
        article { class: "sc-story",
            StoryHead { story: story.clone() }
            if is_stub {
                Alert { variant: AlertVariant::Info, title: Some("Storia in migrazione".to_string()),
                    "Questa pagina viene riscritta sul nuovo template. Il componente è già disponibile nella libreria."
                }
            }
            if let Some(pg) = story.playground.clone() {
                LivePlayground { playground: pg }
            }
            if !story.examples.is_empty() {
                ExamplesSection { examples: story.examples.clone() }
            }
            if story.recipe.is_some() || !story.a11y.is_empty() {
                NotesSection { recipe: story.recipe, a11y: story.a11y }
            }
            if !story.props.is_empty() {
                PropsSection { groups: story.props.clone() }
            }
            if !story.related.is_empty() {
                RelatedSection { related: story.related }
            }
            PrevNext {
                prev: prev.map(|s| (s.slug, s.name)),
                next: next.map(|s| (s.slug, s.name)),
            }
        }
    }
}

#[component]
fn StoryHead(story: Story) -> Element {
    let cargo = story.cargo_line();
    rsx! {
        header { class: "sc-story__head",
            Breadcrumbs {
                items: Some(vec![
                    BreadcrumbData::new("Componenti"),
                    BreadcrumbData::new(story.category.label()),
                    BreadcrumbData::new(story.name),
                ]),
            }
            div { class: "sc-story__title-row",
                div { class: "sc-icon-tile", Icon { kind: story.icon, size: 22 } }
                div { class: "sc-story__title-col",
                    h1 { class: "sc-title", "{story.name}" }
                    p { class: "sc-lede", "{story.tagline}" }
                }
                div { class: "sc-story__meta",
                    for f in story.features {
                        Badge { key: "{f}", label: f.to_string(), pill: true }
                    }
                    if story.status == Status::Beta {
                        Badge { label: "Beta".to_string(), variant: BadgeVariant::Warning, pill: true }
                    }
                }
            }
            CodeBlock {
                code: cargo,
                language: "toml",
                filename: Some("Cargo.toml".to_string()),
                with_line_numbers: false,
                class: "sc-story__cargo".to_string(),
            }
        }
    }
}

#[component]
fn LivePlayground(playground: Playground) -> Element {
    let knobs = use_signal(|| Knobs::from_controls(&playground.controls));
    let changed = use_signal(|| None::<&'static str>);
    let current = knobs();
    let generated = knobs::generate(&playground, &current, changed());
    let stage_class = match playground.stage {
        Stage::Center => "sc-stage",
        Stage::Start => "sc-stage sc-stage--start",
        Stage::Form => "sc-stage sc-stage--form",
        Stage::Fill => "sc-stage sc-stage--fill",
    };
    let file = format!("{}.rs", playground.component.to_lowercase());
    rsx! {
        section { class: "sc-section", id: "playground",
            div { class: "sc-section__head-row",
                SectionHead { eyebrow: "Playground", title: "Prova dal vivo",
                    desc: Some("Ogni controllo aggiorna il componente e il codice qui sotto.") }
                ThemeSwatches { compact: true }
            }
            div { class: "sc-playground",
                div { class: "sc-playground__stage",
                    div { class: "{stage_class}", {(playground.render)(&current)} }
                    CodeBlock {
                        code: generated.code,
                        language: "rsx",
                        filename: Some(file),
                        highlight_lines: generated.highlight,
                    }
                }
                aside { class: "sc-playground__knobs", aria_label: "Controlli del playground",
                    KnobPanel { controls: playground.controls.clone(), knobs, changed }
                }
            }
        }
    }
}
```

### 4.7 `story/sections.rs`

```rust
use dioxus::prelude::*;
use shadow_design::prelude::*;

use super::registry;
use super::schema::{Example, PropGroup, Recipe, Span};
use crate::Route;

#[component]
pub fn SectionHead(eyebrow: &'static str, title: &'static str, #[props(default)] desc: Option<&'static str>) -> Element {
    rsx! {
        div { class: "sc-section__head",
            span { class: "sc-eyebrow", "{eyebrow}" }
            h2 { class: "sc-section__title", "{title}" }
            if let Some(d) = desc { p { class: "sc-section__desc", "{d}" } }
        }
    }
}

#[component]
pub fn ExamplesSection(examples: Vec<Example>) -> Element {
    rsx! {
        section { class: "sc-section", id: "esempi",
            SectionHead { eyebrow: "Esempi", title: "Varianti e contesto", desc: Some("Ogni esempio è codice reale: aprilo con \"Codice\".") }
            div { class: "sc-examples",
                for e in examples { ExampleCard { key: "{e.id}", example: e } }
            }
        }
    }
}

#[component]
fn ExampleCard(example: Example) -> Element {
    let mut show_code = use_signal(|| false);
    let class = if example.span == Span::Full { "sc-example sc-example--full" } else { "sc-example" };
    rsx! {
        Card {
            class: class.to_string(),
            title: Some(example.title.to_string()),
            subtitle: Some(example.description.to_string()),
            buttons: Some(rsx! {
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    aria_pressed: Some(show_code()),
                    left_icon: Some(rsx! { Icon { kind: IconKind::Code, size: 14 } }),
                    onclick: move |_| show_code.toggle(),
                    "Codice"
                }
            }),
            div { class: "sc-example__stage", {(example.render)()} }
            Collapsible { open: show_code,
                CodeBlock { code: example.code.to_string(), language: "rsx", with_line_numbers: false }
            }
        }
    }
}

#[component]
pub fn NotesSection(recipe: Option<Recipe>, a11y: &'static [&'static str]) -> Element {
    rsx! {
        section { class: "sc-section", id: "ricetta",
            SectionHead { eyebrow: "Lit Glass", title: "Ricetta e accessibilità" }
            div { class: "sc-notes",
                if let Some(r) = recipe {
                    Card { title: Some("Ricetta".to_string()), subtitle: Some(r.recipe.to_string()),
                        dl { class: "sc-dl",
                            dt { "Profondità" } dd { "{r.depth}" }
                            dt { "Token" }
                            dd { class: "sc-dl__chips", for t in r.tokens { Code { key: "{t}", "{t}" } } }
                            dt { "Note" } dd { "{r.notes}" }
                        }
                    }
                }
                if !a11y.is_empty() {
                    Card { title: Some("Accessibilità".to_string()),
                        List {
                            for (i, item) in a11y.iter().enumerate() {
                                ListItem { key: "{i}", icon: Some(rsx! { Icon { kind: IconKind::CheckCircle, size: 14 } }), "{item}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PropsSection(groups: Vec<PropGroup>) -> Element {
    let active = use_signal(|| groups.first().map(|g| g.component.to_string()).unwrap_or_default());
    rsx! {
        section { class: "sc-section", id: "api",
            SectionHead { eyebrow: "API", title: "Props" }
            if groups.len() == 1 {
                PropsTable { group: groups[0].clone() }
            } else {
                Tabs {
                    TabList { aria_label: Some("Componenti documentati".to_string()),
                        for g in groups.iter() {
                            Tab { key: "{g.component}", value: g.component.to_string(), active, "{g.component}" }
                        }
                    }
                    for g in groups.iter() {
                        TabPanel { key: "{g.component}", value: g.component.to_string(), active,
                            PropsTable { group: g.clone() }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PropsTable(group: PropGroup) -> Element {
    rsx! {
        div { class: "sc-props",
            Table { hoverable: false, bordered: true,
                TableHeader { TableRow {
                    TableCell { is_header: true, "Prop" }
                    TableCell { is_header: true, "Tipo" }
                    TableCell { is_header: true, "Default" }
                    TableCell { is_header: true, "Descrizione" }
                } }
                TableBody {
                    for row in group.rows {
                        TableRow { key: "{row.name}",
                            TableCell { class: "sc-props__name", "{row.name}" }
                            TableCell { class: "sc-props__type", "{row.ty}" }
                            TableCell { class: "sc-props__default", "{row.default}" }
                            TableCell { class: "sc-props__desc", "{row.doc}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn RelatedSection(related: &'static [&'static str]) -> Element {
    let nav = use_navigator();
    rsx! {
        section { class: "sc-section", id: "correlati",
            SectionHead { eyebrow: "Correlati", title: "Vedi anche" }
            div { class: "sc-related",
                for slug in related {
                    if let Some(s) = registry::find(slug) {
                        NavLink {
                            key: "{s.slug}",
                            label: s.name.to_string(),
                            description: Some(s.tagline.to_string()),
                            icon: Some(SvgIcon::kind(s.icon)),
                            on_click: move |_| { nav.push(Route::story(s.slug)); },
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PrevNext(prev: Option<(&'static str, &'static str)>, next: Option<(&'static str, &'static str)>) -> Element {
    let nav = use_navigator();
    rsx! {
        nav { class: "sc-prevnext", aria_label: "Storie adiacenti",
            if let Some((slug, name)) = prev {
                Button { variant: ButtonVariant::Secondary,
                    left_icon: Some(rsx! { Icon { kind: IconKind::ChevronLeft, size: 14 } }),
                    onclick: move |_| { nav.push(Route::story(slug)); },
                    "{name}"
                }
            } else { span {} }
            if let Some((slug, name)) = next {
                Button { variant: ButtonVariant::Secondary,
                    right_icon: Some(rsx! { Icon { kind: IconKind::ChevronRight, size: 14 } }),
                    onclick: move |_| { nav.push(Route::story(slug)); },
                    "{name}"
                }
            }
        }
    }
}

/// STATIC layout tile for Grid/Group/Stack/Masonry/Center/AspectRatio demos. Never looks clickable.
#[component]
pub fn DemoCell(#[props(default)] index: Option<usize>, #[props(default)] label: Option<String>, #[props(default)] tall: bool, #[props(default)] accent: bool) -> Element {
    let tall = if tall { "sc-demo-cell--tall" } else { "" };
    let accent = if accent { "sc-demo-cell--accent" } else { "" };
    rsx! {
        div { class: "sc-demo-cell {tall} {accent}",
            if let Some(i) = index { span { class: "sc-demo-cell__index", "{i}" } }
            if let Some(l) = label { span { "{l}" } }
        }
    }
}
```

### 4.8 Chrome: `app/theme_menu.rs` (the rest of `app/` follows the current `layout.rs`, rebuilt on these pieces)

```rust
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::Route;

const PRESETS: [(PrimaryColor, &str); 5] = [
    (PrimaryColor::Violet, "Ultraviolet"),
    (PrimaryColor::Obsidian, "Obsidian"),
    (PrimaryColor::Crimson, "Crimson"),
    (PrimaryColor::Cyan, "Cyan"),
    (PrimaryColor::Emerald, "Emerald"),
];

/// Five accent dots. Each dot is a library `Button` (ghost, icon only) carrying a
/// `.sc-dot` span whose background is the preset hex through an inline style
/// built from `PrimaryColor::hex()` (a runtime value, not a literal).
#[component]
pub fn ThemeSwatches(#[props(default)] compact: bool) -> Element {
    let mut theme = use_theme();
    let current = theme().primary_color;
    let class = if compact { "sc-swatches sc-swatches--compact" } else { "sc-swatches" };
    rsx! {
        div { class: "{class}", role: "group", aria_label: "Colore primario",
            for (color, name) in PRESETS {
                Tooltip { key: "{name}", content: name.to_string(), position: TooltipPosition::Bottom,
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        aria_label: Some(name.to_string()),
                        aria_pressed: Some(current == color),
                        class: if current == color { "sc-swatch sc-swatch--active".to_string() } else { "sc-swatch".to_string() },
                        onclick: {
                            let color = color.clone();
                            move |_| theme.write().apply_primary_color(color.clone())
                        },
                        span { class: "sc-dot", style: "background-color: {color.hex()};" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ThemeControls() -> Element {
    let mut theme = use_theme();
    let mut radius = use_signal(|| radius_key(theme.peek().radius_scale).to_string());
    let mut glow = use_signal(|| glow_key(theme.peek().glow_mode).to_string());
    use_effect(move || {
        let t = theme();
        if radius.peek().as_str() != radius_key(t.radius_scale) { radius.set(radius_key(t.radius_scale).to_string()); }
        if glow.peek().as_str() != glow_key(t.glow_mode) { glow.set(glow_key(t.glow_mode).to_string()); }
    });
    rsx! {
        div { class: "sc-theme-controls",
            div { class: "sc-knob",
                span { class: "sc-eyebrow", "Raggio" }
                SegmentedControl { size: SegmentedControlSize::Sm, full_width: true,
                    options: vec![SegmentOption::new("sharp", "Sharp"), SegmentOption::new("medium", "Medium"), SegmentOption::new("soft", "Soft")],
                    value: radius,
                    on_change: move |v: String| theme.write().apply_radius_scale(match v.as_str() { "sharp" => RadiusScale::Sharp, "soft" => RadiusScale::Soft, _ => RadiusScale::Medium }),
                }
            }
            div { class: "sc-knob",
                span { class: "sc-eyebrow", "Glow" }
                SegmentedControl { size: SegmentedControlSize::Sm, full_width: true,
                    options: vec![SegmentOption::new("off", "Off"), SegmentOption::new("subtle", "Subtle"), SegmentOption::new("expressive", "Expressive")],
                    value: glow,
                    on_change: move |v: String| theme.write().apply_glow_mode(match v.as_str() { "off" => GlowMode::Off, "expressive" => GlowMode::Expressive, _ => GlowMode::Subtle }),
                }
            }
        }
    }
}

/// Header button: current accent dot + "Tema", opens the full controls in a Popover.
#[component]
pub fn ThemeMenuButton() -> Element {
    let opened = use_signal(|| false);
    let nav = use_navigator();
    rsx! {
        Popover { opened, position: PopoverPosition::BottomEnd,
            PopoverTrigger {
                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, aria_label: Some("Apri le opzioni del tema".to_string()),
                    span { class: "sc-dot sc-dot--accent" }
                    span { class: "sc-hide-below-md", "Tema" }
                }
            }
            PopoverDropdown { class: "sc-theme-menu".to_string(),
                ThemeSwatches {}
                ThemeControls {}
                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, full_width: true,
                    left_icon: Some(rsx! { Icon { kind: IconKind::Palette, size: 14 } }),
                    onclick: move |_| { nav.push(Route::ThemeBuilderPage {}); },
                    "Theme Builder"
                }
            }
        }
    }
}

fn radius_key(r: RadiusScale) -> &'static str { match r { RadiusScale::Sharp => "sharp", RadiusScale::Medium => "medium", RadiusScale::Soft => "soft" } }
fn glow_key(g: GlowMode) -> &'static str { match g { GlowMode::Off => "off", GlowMode::Subtle => "subtle", GlowMode::Expressive => "expressive" } }
```

`.sc-dot--accent { background: var(--sd-accent) }` is the only dot that uses a token; the five preset dots use the preset hex from `PrimaryColor::hex()` at runtime (allowed: the string contains `{`, the lint skips it, and it is data, not styling).

`app/layout.rs` keeps the current structure (header, mobile `Drawer`, sidebar `aside`, `main > div.sc-page > Outlet`) with these changes: the toolbar and `.sc-toolrow` are removed; the header right side is `SearchButton` (a library `Button` Secondary Sm with `Search` icon, "Cerca" label hidden below `sm`, and a library `Kbd { "Ctrl K" }`) + `ThemeMenuButton`; the mobile drawer gets a `TextInput` filter above `SidebarNav`; `.sc-page` gets `sc-page--wide` when the route is `Home`.

### 4.9 Worked story: `stories/buttons.rs`

```rust
//! Story: Button (feature `button`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const VARIANTS: [ButtonVariant; 6] = [
    ButtonVariant::Primary, ButtonVariant::Secondary, ButtonVariant::Outline,
    ButtonVariant::Ghost, ButtonVariant::Success, ButtonVariant::Danger,
];
const VARIANT_OPTS: &[ChoiceOption] = &[
    opt("Primary", "ButtonVariant::Primary"), opt("Secondary", "ButtonVariant::Secondary"),
    opt("Outline", "ButtonVariant::Outline"), opt("Ghost", "ButtonVariant::Ghost"),
    opt("Success", "ButtonVariant::Success"), opt("Danger", "ButtonVariant::Danger"),
];
const SIZES: [ButtonSize; 5] = [ButtonSize::Xs, ButtonSize::Sm, ButtonSize::Md, ButtonSize::Lg, ButtonSize::Xl];
const SIZE_OPTS: &[ChoiceOption] = &[
    opt("Xs", "ButtonSize::Xs"), opt("Sm", "ButtonSize::Sm"), opt("Md", "ButtonSize::Md"),
    opt("Lg", "ButtonSize::Lg"), opt("Xl", "ButtonSize::Xl"),
];

fn render(k: &Knobs) -> Element {
    rsx! {
        Button {
            variant: k.pick("variant", &VARIANTS),
            size: k.pick("size", &SIZES),
            loading: k.bool("loading"),
            disabled: k.bool("disabled"),
            full_width: k.bool("full_width"),
            glow: k.bool("glow").then_some(true),
            left_icon: k.bool("left_icon").then(|| rsx! { Icon { kind: IconKind::Sparkles, size: 15 } }),
            right_icon: k.bool("right_icon").then(|| rsx! { Icon { kind: IconKind::ChevronRight, size: 15 } }),
            {k.text("label")}
        }
    }
}

#[component]
fn Variants() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm,
            Button { variant: ButtonVariant::Primary, "Primary" }
            Button { variant: ButtonVariant::Secondary, "Secondary" }
            Button { variant: ButtonVariant::Outline, "Outline" }
            Button { variant: ButtonVariant::Ghost, "Ghost" }
            Button { variant: ButtonVariant::Success, "Success" }
            Button { variant: ButtonVariant::Danger, "Danger" }
        }
    }
}
const VARIANTS_CODE: &str = r#"Group { gap: SpacingToken::Sm,
    Button { variant: ButtonVariant::Primary, "Primary" }
    Button { variant: ButtonVariant::Secondary, "Secondary" }
    Button { variant: ButtonVariant::Outline, "Outline" }
    Button { variant: ButtonVariant::Ghost, "Ghost" }
    Button { variant: ButtonVariant::Success, "Success" }
    Button { variant: ButtonVariant::Danger, "Danger" }
}"#;

#[component]
fn Sizes() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm, align: AlignItems::Center,
            Button { size: ButtonSize::Xs, variant: ButtonVariant::Secondary, "Xs" }
            Button { size: ButtonSize::Sm, variant: ButtonVariant::Secondary, "Sm" }
            Button { size: ButtonSize::Md, variant: ButtonVariant::Secondary, "Md" }
            Button { size: ButtonSize::Lg, variant: ButtonVariant::Secondary, "Lg" }
            Button { size: ButtonSize::Xl, variant: ButtonVariant::Secondary, "Xl" }
        }
    }
}
const SIZES_CODE: &str = r#"Group { gap: SpacingToken::Sm, align: AlignItems::Center,
    Button { size: ButtonSize::Xs, variant: ButtonVariant::Secondary, "Xs" }
    Button { size: ButtonSize::Sm, variant: ButtonVariant::Secondary, "Sm" }
    Button { size: ButtonSize::Md, variant: ButtonVariant::Secondary, "Md" }
    Button { size: ButtonSize::Lg, variant: ButtonVariant::Secondary, "Lg" }
    Button { size: ButtonSize::Xl, variant: ButtonVariant::Secondary, "Xl" }
}"#;

#[component]
fn States() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm,
            Button { loading: true, "Salvataggio" }
            Button { disabled: true, variant: ButtonVariant::Secondary, "Non disponibile" }
            Button { variant: ButtonVariant::Ghost, aria_label: Some("Impostazioni".to_string()),
                Icon { kind: IconKind::Settings, size: 16 }
            }
        }
    }
}
const STATES_CODE: &str = r#"Group { gap: SpacingToken::Sm,
    Button { loading: true, "Salvataggio" }
    Button { disabled: true, variant: ButtonVariant::Secondary, "Non disponibile" }
    Button { variant: ButtonVariant::Ghost, aria_label: Some("Impostazioni".to_string()),
        Icon { kind: IconKind::Settings, size: 16 }
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Nodo edge-07".to_string()),
            subtitle: Some("Kyber-1024 · 4 peer attivi".to_string()),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, "Annulla" }
                    Button { variant: ButtonVariant::Danger, "Scollega" }
                    Button { variant: ButtonVariant::Primary,
                        left_icon: Some(rsx! { Icon { kind: IconKind::Check, size: 15 } }),
                        onclick: move |_| { toast.success("Configurazione salvata"); },
                        "Salva modifiche"
                    }
                }
            }),
            p { class: "sc-muted-text", "Una sola azione primaria per vista: le altre restano secondarie o ghost." }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut toast = use_notification();

Card {
    title: Some("Nodo edge-07".to_string()),
    subtitle: Some("Kyber-1024 · 4 peer attivi".to_string()),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button { variant: ButtonVariant::Ghost, "Annulla" }
            Button { variant: ButtonVariant::Danger, "Scollega" }
            Button { variant: ButtonVariant::Primary,
                left_icon: Some(rsx! { Icon { kind: IconKind::Check, size: 15 } }),
                onclick: move |_| { toast.success("Configurazione salvata"); },
                "Salva modifiche"
            }
        }
    }),
    p { "Una sola azione primaria per vista: le altre restano secondarie o ghost." }
}"#;

const PROPS: &[PropDoc] = &[
    prop("variant", "ButtonVariant", "Primary", "Primary (lit accent), Secondary, Outline, Ghost, Success e Danger (tinta soft)."),
    prop("size", "ButtonSize", "Md", "Altezze da --sd-control-xs a --sd-control-xl."),
    prop("loading", "bool", "false", "Spinner, aria-busy e aria-disabled."),
    prop("loading_label", "Option<String>", "None", "Testo per screen reader durante il caricamento."),
    prop("disabled", "bool", "false", "Opacità --sd-disabled-opacity, nessun hover né glow."),
    prop("full_width", "bool", "false", "Occupa il 100% del contenitore."),
    prop("glow", "Option<bool>", "None", "Forza o rimuove il glow (--sd-glow-sm)."),
    prop("left_icon / right_icon", "Option<Element>", "None", "Icona prima o dopo il testo, aria-hidden."),
    prop("aria_label", "Option<String>", "None", "Obbligatoria per i pulsanti con sola icona."),
    prop("aria_pressed", "Option<bool>", "None", "Per pulsanti toggle."),
    prop("button_type", "Option<String>", "None", "type HTML (submit, reset, button)."),
    prop("onclick", "Option<EventHandler<MouseEvent>>", "None", "Click o attivazione da tastiera."),
    prop("class", "String", "\"\"", "Classi aggiuntive sull'elemento <button>."),
];

pub fn story() -> Story {
    Story {
        slug: "buttons",
        name: "Button",
        category: Category::Form,
        features: &["button"],
        icon: IconKind::Sparkles,
        tagline: "L'unica superficie con il riempimento illuminato: una sola azione primaria per vista.",
        keywords: &["button", "pulsante", "cta", "azione", "loading", "icon button"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Button",
            controls: vec![
                Control::choice("variant", "Variante", VARIANT_OPTS, 0),
                Control::choice("size", "Dimensione", SIZE_OPTS, 2),
                Control::text("label", "Etichetta", "Salva modifiche").emit(Emit::Skip),
                Control::bool("left_icon", "Icona sinistra", true)
                    .emit(Emit::Template("left_icon: Some(rsx! { Icon { kind: IconKind::Sparkles, size: 15 } })")),
                Control::bool("right_icon", "Icona destra", false)
                    .emit(Emit::Template("right_icon: Some(rsx! { Icon { kind: IconKind::ChevronRight, size: 15 } })")),
                Control::bool("loading", "Loading", false),
                Control::bool("disabled", "Disabilitato", false),
                Control::bool("full_width", "Larghezza piena", false),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some("\"{label}\""),
            stage: Stage::Center,
        }),
        examples: vec![
            Example { id: "variants", title: "Varianti", description: "Primary è l'unico con riempimento accento.", code: VARIANTS_CODE, render: || rsx! { Variants {} }, span: Span::Half },
            Example { id: "sizes", title: "Dimensioni", description: "Cinque altezze dai token --sd-control-*.", code: SIZES_CODE, render: || rsx! { Sizes {} }, span: Span::Half },
            Example { id: "states", title: "Stati", description: "Loading, disabilitato e sola icona.", code: STATES_CODE, render: || rsx! { States {} }, span: Span::Half },
            Example { id: "context", title: "In contesto", description: "Barra azioni di una card con una sola primaria.", code: IN_CONTEXT_CODE, render: || rsx! { InContext {} }, span: Span::Full },
        ],
        props: vec![PropGroup { component: "Button", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Raised control (language.md §3.5)",
            depth: "Lit control",
            tokens: &["--sd-control-md", "--sd-accent", "--sd-accent-deep", "--sd-accent-fg", "--sd-accent-border", "--sd-glow-sm", "--sd-radius-md"],
            notes: "Gradiente accento → accent-deep solo su Primary; Danger e Success sono tinte soft, mai piene.",
        }),
        a11y: &[
            "Elemento <button> nativo: attivabile con Invio e Spazio.",
            "loading imposta aria-busy e aria-disabled senza rimuovere il focus.",
            "I pulsanti con sola icona richiedono aria_label.",
            "Focus visibile con --sd-focus-ring (outline 2px, offset 2px).",
        ],
        related: &["close-button", "copy-button", "segmented-controls", "speed-dial"],
    }
}
```

Note the `Example.code` for `InContext` omits the `class: "sc-muted-text"` of the live render on purpose: showcase-only classes never appear in sample code. That is the one allowed difference (§5 item 6).

### 4.10 Compound story: `stories/select.rs`

```rust
//! Story: Select, MultiSelect, Combobox (feature `select`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

fn frameworks() -> Vec<SelectItem> {
    vec![
        SelectItem::new("dioxus", "Dioxus 0.7").with_description("Web, desktop e mobile da un solo codice"),
        SelectItem::new("axum", "Axum").with_description("Web framework su Tokio"),
        SelectItem::new("tauri", "Tauri 2").with_description("App desktop e mobile leggere"),
        SelectItem::new("bevy", "Bevy").with_description("Motore di gioco ECS"),
        SelectItem::new("yew", "Yew").with_description("Non più mantenuto nello stack").disabled(),
    ]
}

/// Stateful demo: owns the signal the playground cannot (render is a plain fn).
#[component]
fn SelectDemo(knobs: Knobs) -> Element {
    let value = use_signal(|| Some("dioxus".to_string()));
    let error = knobs.bool("error").then(|| "Seleziona un framework".to_string());
    rsx! {
        Select {
            data: frameworks(),
            value,
            label: Some(knobs.text("label")),
            description: Some(knobs.text("description")),
            placeholder: Some(knobs.text("placeholder")),
            searchable: knobs.bool("searchable"),
            clearable: knobs.bool("clearable"),
            creatable: knobs.bool("creatable"),
            disabled: knobs.bool("disabled"),
            loading: knobs.bool("loading"),
            error,
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! { SelectDemo { knobs: k.clone() } }
}

#[component]
fn Multi() -> Element {
    let values = use_signal(|| vec!["rust".to_string(), "dioxus".to_string()]);
    rsx! {
        MultiSelect {
            data: vec![
                SelectItem::new("rust", "Rust"), SelectItem::new("dioxus", "Dioxus"), SelectItem::new("wasm", "WebAssembly"),
                SelectItem::new("scss", "SCSS"), SelectItem::new("tokio", "Tokio"),
            ],
            values,
            label: Some("Competenze".to_string()),
            searchable: true,
            clearable: true,
            creatable: true,
            max_selected_display: Some(3),
        }
    }
}
const MULTI_CODE: &str = r#"let values = use_signal(|| vec!["rust".to_string(), "dioxus".to_string()]);

MultiSelect {
    data: vec![
        SelectItem::new("rust", "Rust"), SelectItem::new("dioxus", "Dioxus"), SelectItem::new("wasm", "WebAssembly"),
        SelectItem::new("scss", "SCSS"), SelectItem::new("tokio", "Tokio"),
    ],
    values,
    label: Some("Competenze".to_string()),
    searchable: true,
    clearable: true,
    creatable: true,
    max_selected_display: Some(3),
}"#;

#[component]
fn Combo() -> Element {
    let value = use_signal(|| "Ultraviolet".to_string());
    rsx! {
        Combobox {
            data: vec![
                SelectItem::new("Ultraviolet", "Ultraviolet"), SelectItem::new("Obsidian", "Obsidian"),
                SelectItem::new("Crimson", "Crimson"), SelectItem::new("Cyan", "Cyan"), SelectItem::new("Emerald", "Emerald"),
            ],
            value,
            label: Some("Preset".to_string()),
            description: Some("Digita o scegli un suggerimento".to_string()),
            clearable: true,
        }
    }
}
const COMBO_CODE: &str = r#"let value = use_signal(|| "Ultraviolet".to_string());

Combobox {
    data: vec![
        SelectItem::new("Ultraviolet", "Ultraviolet"), SelectItem::new("Obsidian", "Obsidian"),
        SelectItem::new("Crimson", "Crimson"), SelectItem::new("Cyan", "Cyan"), SelectItem::new("Emerald", "Emerald"),
    ],
    value,
    label: Some("Preset".to_string()),
    description: Some("Digita o scegli un suggerimento".to_string()),
    clearable: true,
}"#;

#[component]
fn Grouped() -> Element {
    let value = use_signal(|| None::<String>);
    rsx! {
        Select {
            data: vec![
                SelectItem::new("arch", "Systems Architect").with_group("Engineering"),
                SelectItem::new("core", "Rust Core Developer").with_group("Engineering"),
                SelectItem::new("ui", "Tactile UI Designer").with_group("Design"),
                SelectItem::new("sec", "Security Auditor").with_group("Security"),
            ],
            value,
            label: Some("Ruolo".to_string()),
            placeholder: Some("Scegli o crea un ruolo".to_string()),
            searchable: true,
            creatable: true,
        }
    }
}
const GROUPED_CODE: &str = r#"let value = use_signal(|| None::<String>);

Select {
    data: vec![
        SelectItem::new("arch", "Systems Architect").with_group("Engineering"),
        SelectItem::new("core", "Rust Core Developer").with_group("Engineering"),
        SelectItem::new("ui", "Tactile UI Designer").with_group("Design"),
        SelectItem::new("sec", "Security Auditor").with_group("Security"),
    ],
    value,
    label: Some("Ruolo".to_string()),
    placeholder: Some("Scegli o crea un ruolo".to_string()),
    searchable: true,
    creatable: true,
}"#;

#[component]
fn InContext() -> Element {
    rsx! { crate::scenes::SettingsForm { compact: true } }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/settings_form.rs: Select e MultiSelect in un Fieldset accanto a Switch, Slider e Button.
Fieldset { legend: "Rete".to_string(),
    Select { data: regions(), value: region, label: Some("Regione".to_string()), searchable: true }
    MultiSelect { data: protocols(), values: protocols_on, label: Some("Protocolli".to_string()), clearable: true }
}"#;

const SELECT_PROPS: &[PropDoc] = &[
    prop("data", "Vec<SelectItem>", "-", "Opzioni: value, label, description, group, disabled."),
    prop("value", "Signal<Option<String>>", "-", "Valore selezionato; None quando vuoto."),
    prop("label / description / placeholder", "Option<String>", "None", "Etichetta, aiuto e segnaposto (placeholder da Locale se assente)."),
    prop("searchable", "bool", "false", "Campo di ricerca nel menu."),
    prop("clearable", "bool", "false", "Pulsante X per azzerare."),
    prop("creatable", "bool", "false", "Crea una voce dal testo cercato (on_create)."),
    prop("disabled / loading", "bool", "false", "Disabilitato; spinner al posto del chevron."),
    prop("error", "Option<String>", "None", "Bordo danger, aria-invalid e messaggio."),
    prop("glow", "Option<bool>", "None", "Glow esplicito sul trigger."),
    prop("on_change", "Option<EventHandler<Option<String>>>", "None", "Cambio selezione (None su clear)."),
    prop("on_create", "Option<EventHandler<String>>", "None", "Nuovo valore creato."),
];
const MULTI_PROPS: &[PropDoc] = &[
    prop("values", "Signal<Vec<String>>", "-", "Valori selezionati."),
    prop("max_selected_display", "Option<usize>", "None", "Chip mostrate prima di \"+N\"."),
    prop("searchable / clearable / creatable", "bool", "false", "Come Select."),
    prop("on_change", "Option<EventHandler<Vec<String>>>", "None", "Aggiunta o rimozione."),
];
const COMBO_PROPS: &[PropDoc] = &[
    prop("value", "Signal<String>", "-", "Testo libero, modificabile."),
    prop("data", "Vec<SelectItem>", "-", "Suggerimenti."),
    prop("clearable / loading / disabled", "bool", "false", "Come Select."),
    prop("on_change", "Option<EventHandler<String>>", "None", "Digitazione o scelta."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("SelectItem::new(value, label)", "fn", "-", "Costruttore."),
    prop(".with_description(..)", "fn", "-", "Riga secondaria."),
    prop(".with_group(..)", "fn", "-", "Intestazione di gruppo."),
    prop(".disabled()", "fn", "-", "Non selezionabile."),
];

pub fn story() -> Story {
    Story {
        slug: "select",
        name: "Select",
        category: Category::Form,
        features: &["select"],
        icon: IconKind::ChevronDown,
        tagline: "Selezione singola, multipla con chip e combobox editabile, con ricerca, gruppi e creazione.",
        keywords: &["select", "multiselect", "combobox", "dropdown", "autocomplete", "tag", "chip"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Select",
            controls: vec![
                Control::text("label", "Etichetta", "Framework").emit(Emit::Template("label: Some(\"{v}\".to_string())")),
                Control::text("description", "Descrizione", "Il framework primario dello stack").emit(Emit::Template("description: Some(\"{v}\".to_string())")),
                Control::text("placeholder", "Placeholder", "Scegli un framework").emit(Emit::Template("placeholder: Some(\"{v}\".to_string())")),
                Control::bool("searchable", "Ricerca", true),
                Control::bool("clearable", "Azzerabile", true),
                Control::bool("creatable", "Creabile", false),
                Control::bool("disabled", "Disabilitato", false),
                Control::bool("loading", "Loading", false),
                Control::bool("error", "Errore", false).emit(Emit::Template("error: Some(\"Seleziona un framework\".to_string())")),
            ],
            render,
            prelude_code: Some("let value = use_signal(|| Some(\"dioxus\".to_string()));\nlet data = vec![\n    SelectItem::new(\"dioxus\", \"Dioxus 0.7\").with_description(\"Web, desktop e mobile\"),\n    SelectItem::new(\"axum\", \"Axum\"),\n];"),
            fixed_code: &["data,", "value,"],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example { id: "multi", title: "MultiSelect", description: "Chip rimovibili, limite di visualizzazione, creazione.", code: MULTI_CODE, render: || rsx! { Multi {} }, span: Span::Half },
            Example { id: "combo", title: "Combobox", description: "Input libero con suggerimenti.", code: COMBO_CODE, render: || rsx! { Combo {} }, span: Span::Half },
            Example { id: "grouped", title: "Gruppi e creazione", description: "Intestazioni di gruppo e nuove voci con Invio.", code: GROUPED_CODE, render: || rsx! { Grouped {} }, span: Span::Half },
            Example { id: "context", title: "In contesto", description: "Il form Impostazioni della scena condivisa.", code: IN_CONTEXT_CODE, render: || rsx! { InContext {} }, span: Span::Full },
        ],
        props: vec![
            PropGroup { component: "Select", rows: SELECT_PROPS },
            PropGroup { component: "MultiSelect", rows: MULTI_PROPS },
            PropGroup { component: "Combobox", rows: COMBO_PROPS },
            PropGroup { component: "SelectItem", rows: ITEM_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Recessed field (§3.6) + floating panel (§3.2) + items (§3.4)",
            depth: "Well (trigger) · Raised (menu)",
            tokens: &["--sd-well", "--sd-border-strong", "--sd-inset-well", "--sd-surface-2", "--sd-elev-2", "--sd-item-h", "--sd-accent-soft"],
            notes: "Il trigger è un campo; il menu è un pannello flottante con voci sd-item e una sola voce selezionata.",
        }),
        a11y: &[
            "Trigger con role=combobox, aria-expanded e aria-controls sul listbox.",
            "Frecce, Home/End, Invio ed Esc gestiti; la ricerca mantiene il focus nel campo.",
            "error imposta aria-invalid e collega il messaggio con aria-describedby.",
            "Le voci disabilitate restano annunciate ma non selezionabili.",
        ],
        related: &["inputs", "chip", "dropdown", "command-palette", "segmented-controls"],
    }
}
```

### 4.11 Home skeleton: `pages/home.rs`

```rust
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::app::theme_menu::ThemeSwatches;
use crate::pages::FOUNDATION;
use crate::scenes::{AuthCard, InboxPane, OpsDashboard, SettingsForm};
use crate::story::registry;
use crate::story::schema::Category;
use crate::story::sections::SectionHead;
use crate::Route;

const PRESETS: [(PrimaryColor, fn() -> Theme); 5] = [
    (PrimaryColor::Violet, Theme::shadow),
    (PrimaryColor::Obsidian, Theme::obsidian),
    (PrimaryColor::Crimson, Theme::crimson),
    (PrimaryColor::Cyan, Theme::cyan),
    (PrimaryColor::Emerald, Theme::emerald),
];

#[component]
pub fn Home() -> Element {
    let nav = use_navigator();
    let scene = use_signal(|| "dashboard".to_string());
    let components = registry::all().len();
    let features = registry::feature_count();
    let install = format!(
        "[dependencies.shadow-design]\nversion = \"{VERSION}\"\ndefault-features = false\nfeatures = [\"button\", \"card\", \"select\", \"theme\"]"
    );

    rsx! {
        div { class: "sc-home",
            // 1. Hero: words left, a working product right.
            section { class: "sc-hero",
                div { class: "sc-hero__copy",
                    span { class: "sc-eyebrow", "Dioxus 0.7 · Lit Glass · v{VERSION}" }
                    h1 { class: "sc-display", "Shadow Design" }
                    p { class: "sc-lede sc-lede--lg",
                        "Vetro nero illuminato dall'alto. Un solo hex ricolora {components} componenti; ogni feature Cargo è opzionale."
                    }
                    Group { gap: SpacingToken::Sm,
                        Button { variant: ButtonVariant::Primary,
                            left_icon: Some(rsx! { Icon { kind: IconKind::Sparkles, size: 15 } }),
                            onclick: move |_| { nav.push(Route::story("buttons")); },
                            "Esplora i componenti"
                        }
                        Button { variant: ButtonVariant::Secondary,
                            left_icon: Some(rsx! { Icon { kind: IconKind::Palette, size: 15 } }),
                            onclick: move |_| { nav.push(Route::ThemeBuilderPage {}); },
                            "Theme Builder"
                        }
                    }
                    div { class: "sc-hero__swatches",
                        ThemeSwatches {}
                        span { class: "sc-hint", "Clicca un colore: tutto lo showcase, compresa la scena qui accanto, cambia all'istante." }
                    }
                }
                div { class: "sc-scene sc-scene--hero", OpsDashboard { compact: false } }
            }

            // 2. Preset wall: five accents at once (nested ThemeProviders).
            section { class: "sc-section",
                SectionHead { eyebrow: "Tema", title: "Un hex, tutto il tema", desc: Some("Cinque preset resi insieme. Ognuno è solo un colore: derivazioni, glow e focus seguono.") }
                ScrollArea { scroll_x: true, scroll_y: false,
                    div { class: "sc-preset-wall",
                        for (preset, build) in PRESETS { PresetTile { key: "{preset.name()}", preset, build } }
                    }
                }
            }

            // 3. Scenes.
            section { class: "sc-section",
                SectionHead { eyebrow: "Scene", title: "Componenti in un prodotto vero", desc: Some("Quattro schermate composte solo con la libreria.") }
                Tabs {
                    TabList { aria_label: Some("Scene".to_string()),
                        Tab { value: "dashboard", active: scene, icon: Some(IconKind::Layers), "Dashboard" }
                        Tab { value: "settings", active: scene, icon: Some(IconKind::Settings), "Impostazioni" }
                        Tab { value: "inbox", active: scene, icon: Some(IconKind::MessageSquare), "Inbox" }
                        Tab { value: "auth", active: scene, icon: Some(IconKind::Lock), "Accesso" }
                    }
                    div { class: "sc-scene",
                        TabPanel { value: "dashboard", active: scene, class: "animate-fade-in", OpsDashboard { compact: false } }
                        TabPanel { value: "settings", active: scene, class: "animate-fade-in", SettingsForm { compact: false } }
                        TabPanel { value: "inbox", active: scene, class: "animate-fade-in", InboxPane { compact: false } }
                        TabPanel { value: "auth", active: scene, class: "animate-fade-in", AuthCard { compact: false } }
                    }
                }
            }

            // 4. Zero bloat: numbers from the registry, not claims.
            section { class: "sc-section",
                SectionHead { eyebrow: "Zero bloat", title: "Compili solo ciò che usi" }
                div { class: "sc-stats",
                    Stat { value: components.to_string(), label: "componenti" }
                    Stat { value: features.to_string(), label: "feature Cargo indipendenti" }
                    Stat { value: "0".to_string(), label: "dipendenze JavaScript" }
                    Stat { value: "3".to_string(), label: "famiglie tipografiche" }
                }
                CodeBlock { code: install, language: "toml", filename: Some("Cargo.toml".to_string()) }
            }

            // 5. Foundations and categories.
            section { class: "sc-section",
                SectionHead { eyebrow: "Fondamenta", title: "Tema, token, tipografia, icone" }
                SimpleGrid { cols: 1, sm: Some(2), lg: Some(4), spacing: GridSpacing::Md,
                    for p in FOUNDATION.iter().skip(1) {
                        Card { key: "{p.name}", title: Some(p.name.to_string()), subtitle: Some(p.blurb.to_string()),
                            onclick: { let r = p.route.clone(); move |_| { nav.push(r.clone()); } },
                            avatar: Some(rsx! { div { class: "sc-icon-tile", Icon { kind: p.icon, size: 18 } } }),
                        }
                    }
                }
            }
            section { class: "sc-section",
                SectionHead { eyebrow: "Catalogo", title: "Otto categorie" }
                SimpleGrid { cols: 1, sm: Some(2), lg: Some(4), spacing: GridSpacing::Md,
                    for cat in Category::ALL.iter().copied().filter(|c| *c != Category::Fondamenta) {
                        {
                            let items = registry::by_category(cat);
                            let first = items.first().map(|s| s.slug).unwrap_or("buttons");
                            rsx! {
                                Card { key: "{cat.label()}", title: Some(cat.label().to_string()),
                                    subtitle: Some(format!("{} componenti · {}", items.len(), cat.blurb())),
                                    onclick: move |_| { nav.push(Route::story(first)); },
                                    avatar: Some(rsx! { div { class: "sc-icon-tile", Icon { kind: cat.icon(), size: 18 } } }),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// One preset rendered under its own ThemeProvider. The "Usa" button writes the GLOBAL theme,
/// which this component reads from the outer context (it is rendered outside the nested provider).
#[component]
fn PresetTile(preset: PrimaryColor, build: fn() -> Theme) -> Element {
    let local = use_signal(build);
    let mut global = use_theme();
    let active = global().primary_color == preset;
    let on = use_signal(|| true);
    let name = preset.name().to_string();
    rsx! {
        ThemeProvider { theme: Some(local),
            Card { class: "sc-preset-tile".to_string(), title: Some(name.clone()), glow: active.then_some(true),
                Stack { gap: SpacingToken::Sm,
                    Group { gap: SpacingToken::Sm, align: AlignItems::Center,
                        Button { size: ButtonSize::Sm, "Salva" }
                        Switch { checked: on }
                        Badge { label: "Attivo".to_string(), variant: BadgeVariant::Primary, pill: true, dot: true }
                    }
                    ProgressBar { value: 64.0 }
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, full_width: true,
                        disabled: active,
                        onclick: {
                            let preset = preset.clone();
                            move |_| global.write().apply_primary_color(preset.clone())
                        },
                        if active { "Preset attivo" } else { "Usa {name}" }
                    }
                }
            }
        }
    }
}

#[component]
fn Stat(value: String, label: &'static str) -> Element {
    rsx! {
        Card { class: "sc-stat".to_string(),
            span { class: "sc-stat__value", "{value}" }
            span { class: "sc-eyebrow", "{label}" }
        }
    }
}
```

Scenes API (phase 1): `pub fn OpsDashboard(compact: bool) -> Element`, same for `SettingsForm`, `InboxPane`, `AuthCard`. `OpsDashboard` data: six `ChartDataPoint`s (Gen..Giu) with series `latenza`, `throughput`, `errori` using `ChartSeries::new(..).with_label(..)` **without** `with_color` (the chart falls back to `--sd-chart-n`); three metric `Card`s ("Latenza media 1.2 ms", "Richieste 4.8M", "Affidabilità 99.99%") each with a `Sparkline { data, trend }`; `DataTable` with columns Nodo / Regione / Stato (`Badge`) / Latenza (tabular mono) / Peer, five rows; a `SegmentedControl` 24h / 7g / 30g; one Primary `Button` "Nuovo nodo".

### 4.12 `showcase.scss` additions

Replace the panel/playground/banner/hero/toolbar blocks of the current file with the following (everything else: shell, sidebar, text helpers, props, demo-cell, builder, stays). Tokens and library mixins only.

```scss
@layer sc-showcase {
  :root {
    --sc-header-h: 3.5rem;
    --sc-sidebar-w: 16rem;
    --sc-sidebar-w-collapsed: 4.25rem;
    --sc-page-w: 72rem;
    --sc-page-w-wide: 80rem;
    --sc-section-gap: var(--sd-space-10);
  }

  .sc-page { width: 100%; max-width: var(--sc-page-w); > * { max-width: none; } }
  .sc-page--wide { max-width: var(--sc-page-w-wide); }

  // ---------------------------------------------------------------- type
  .sc-eyebrow {
    font-family: var(--sd-font-mono);
    font-size: var(--sd-text-2xs);
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--sd-text-subtle);
  }
  .sc-title {
    margin: 0;
    font-family: var(--sd-font-heading);
    font-weight: 700;
    font-size: var(--sd-text-2xl);
    line-height: var(--sd-leading-tight);
    color: var(--sd-text);
    overflow-wrap: anywhere;
  }
  .sc-display {
    @extend .sc-title;
    // Display size derived from the scale (ADR-0006 §3): never a raw px.
    font-size: calc(var(--sd-text-3xl) * 1.5);
    @include bp-up(lg) { font-size: calc(var(--sd-text-3xl) * 1.85); }
  }
  .sc-lede { margin: 0; font-size: var(--sd-text-md); line-height: var(--sd-leading-normal); color: var(--sd-text-muted); max-width: 44rem; }
  .sc-lede--lg { font-size: var(--sd-text-lg); }
  .sc-hint { font-family: var(--sd-font-mono); font-size: var(--sd-text-xs); color: var(--sd-text-subtle); }
  .sc-readout { font-family: var(--sd-font-mono); font-size: var(--sd-text-xs); color: var(--sd-accent-strong); font-variant-numeric: tabular-nums; }

  // ---------------------------------------------------------------- sections
  .sc-story, .sc-home { display: flex; flex-direction: column; gap: var(--sc-section-gap); }
  .sc-section { display: flex; flex-direction: column; gap: var(--sd-space-6); min-width: 0; }
  .sc-section__head { display: flex; flex-direction: column; gap: var(--sd-space-1); }
  .sc-section__head-row { display: flex; flex-wrap: wrap; align-items: flex-end; justify-content: space-between; gap: var(--sd-space-3); }
  .sc-section__title { margin: 0; font-family: var(--sd-font-heading); font-weight: var(--sd-weight-heading); font-size: var(--sd-text-xl); line-height: var(--sd-leading-tight); color: var(--sd-text); }
  .sc-section__desc { margin: 0; font-size: var(--sd-text-sm); color: var(--sd-text-muted); }

  // ---------------------------------------------------------------- story head
  .sc-story__head { display: flex; flex-direction: column; gap: var(--sd-space-4); }
  .sc-story__title-row { display: flex; flex-wrap: wrap; align-items: flex-start; gap: var(--sd-space-4); }
  .sc-story__title-col { display: flex; flex-direction: column; gap: var(--sd-space-2); flex: 1; min-width: 16rem; }
  .sc-story__meta { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sd-space-2); }
  .sc-story__cargo { max-width: 56rem; }
  .sc-icon-tile {
    @include sd-surface(1);
    display: inline-flex; align-items: center; justify-content: center; flex: none;
    width: var(--sd-control-lg); height: var(--sd-control-lg);
    color: var(--sd-accent-strong);
  }

  // ---------------------------------------------------------------- playground
  .sc-playground {
    display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--sd-space-6); align-items: start;
    @include bp-up(lg) { grid-template-columns: minmax(0, 7fr) minmax(0, 5fr); }
  }
  .sc-playground__stage { display: flex; flex-direction: column; gap: var(--sd-space-4); min-width: 0; }
  .sc-playground__knobs { @include sd-surface(1); padding: var(--sd-space-4); min-width: 0; }
  .sc-knobs { display: flex; flex-direction: column; gap: var(--sd-space-4); }
  .sc-knob { display: flex; flex-direction: column; gap: var(--sd-space-2); min-width: 0; }
  .sc-knob__row { display: flex; align-items: baseline; justify-content: space-between; gap: var(--sd-space-2); }

  // Stage: static, flat-lit, never reacts to the pointer. Overflow visible so menus escape.
  .sc-stage {
    @include sd-surface(1);
    display: flex; flex-direction: column; align-items: center; justify-content: center;
    gap: var(--sd-space-4); min-height: 12rem; padding: var(--sd-space-6); min-width: 0; overflow: visible;
    &--start { align-items: stretch; justify-content: flex-start; }
    &--form { align-items: center; > * { width: 100%; max-width: 24rem; } }
    &--fill { align-items: stretch; justify-content: flex-start; min-height: 20rem; padding: var(--sd-space-4); }
  }

  // ---------------------------------------------------------------- examples
  .sc-examples {
    display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--sd-space-4);
    @include bp-up(lg) { grid-template-columns: repeat(2, minmax(0, 1fr)); }
    > * { min-width: 0; }
  }
  .sc-example--full { grid-column: 1 / -1; }
  .sc-example__stage { display: flex; flex-direction: column; gap: var(--sd-space-3); min-height: 6rem; justify-content: center; }

  // ---------------------------------------------------------------- notes, related, prev/next
  .sc-notes { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--sd-space-4); @include bp-up(md) { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
  .sc-dl { margin: 0; display: grid; grid-template-columns: max-content minmax(0, 1fr); gap: var(--sd-space-2) var(--sd-space-4); font-size: var(--sd-text-sm);
    dt { @extend .sc-eyebrow; padding-top: 2px; } dd { margin: 0; color: var(--sd-text-muted); } }
  .sc-dl__chips { display: flex; flex-wrap: wrap; gap: var(--sd-space-1); }
  .sc-related { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--sd-space-2); @include bp-up(md) { grid-template-columns: repeat(2, minmax(0, 1fr)); } @include bp-up(lg) { grid-template-columns: repeat(3, minmax(0, 1fr)); } }
  .sc-prevnext { display: flex; justify-content: space-between; gap: var(--sd-space-3); padding-top: var(--sd-space-6); border-top: 1px solid var(--sd-border-subtle); }

  // ---------------------------------------------------------------- theme menu & swatches
  .sc-swatches { display: inline-flex; align-items: center; gap: var(--sd-space-1); }
  .sc-swatch { padding-inline: var(--sd-space-1); }
  .sc-swatch--active .sc-dot { outline: 2px solid var(--sd-text); outline-offset: 2px; }
  .sc-dot { display: inline-block; width: var(--sd-icon-md); height: var(--sd-icon-md); border-radius: var(--sd-radius-full); border: 1px solid var(--sd-border-strong); flex: none; }
  .sc-dot--accent { background-color: var(--sd-accent); --sd-shadow-glow: var(--sd-glow-sm); @include sd-shadow; }
  .sc-theme-menu { display: flex; flex-direction: column; gap: var(--sd-space-4); width: 18rem; max-width: calc(100vw - var(--sd-space-8)); }
  .sc-theme-controls { display: flex; flex-direction: column; gap: var(--sd-space-3); }

  // ---------------------------------------------------------------- home
  .sc-hero {
    display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--sd-space-8); align-items: center;
    @include bp-up(lg) { grid-template-columns: minmax(0, 5fr) minmax(0, 7fr); }
  }
  .sc-hero__copy { display: flex; flex-direction: column; gap: var(--sd-space-5); }
  .sc-hero__swatches { display: flex; flex-direction: column; gap: var(--sd-space-2); }
  .sc-scene { @include sd-surface(1); padding: var(--sd-space-4); min-width: 0; overflow: visible; &--hero { min-height: 30rem; } }
  .sc-preset-wall { display: grid; grid-auto-flow: column; grid-auto-columns: 16rem; gap: var(--sd-space-4); padding-bottom: var(--sd-space-2);
    @include bp-up(lg) { grid-auto-flow: row; grid-template-columns: repeat(5, minmax(0, 1fr)); grid-auto-columns: auto; } }
  .sc-stats { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: var(--sd-space-4); @include bp-up(md) { grid-template-columns: repeat(4, minmax(0, 1fr)); } }
  .sc-stat { text-align: center; }
  .sc-stat__value { display: block; font-family: var(--sd-font-heading); font-weight: var(--sd-weight-heading); font-size: var(--sd-text-3xl); line-height: var(--sd-leading-tight); color: var(--sd-text); font-variant-numeric: tabular-nums; }
}
```

Removed from `showcase.scss`: `.sc-toolbar*`, `.sc-toolrow`, `.sc-search*`, `.sc-panel*`, `.sc-banner*`, `.sc-control*` (replaced by `.sc-knob*`), `.sc-hero__title`, `.sc-page-header/.sc-page-title/.sc-page-desc` (replaced by `.sc-title/.sc-lede`), `.sc-color-input`, `.sc-kbd` (library `Kbd`). Keep `.sc-props*`, `.sc-demo-cell*`, `.sc-demo-frame`, text helpers, `.sc-stack`, `.sc-actions`, `.sc-card-grid`, builder classes, shell classes.

---

## 5. Migration checklist (per story)

Source: `showcase/legacy/pages/<old>.rs`. Target: `showcase/src/stories/<slug>.rs`. Done when every item holds.

1. **Identity**: `slug` = old route path without `/`; `name` = Rust component name (strip "Component", "System", "(SVG)", "(Mesh)", "& Combobox"...); `category` from §2.1; `features` from the old `FeatureBanner`/`ComponentItem`; `icon` the most specific `IconKind` available; `tagline` one Italian sentence that says what the component is for (no internals like "z-[1000]", no marketing adjectives); `keywords` ≥ 4, lowercase, Italian and English.
2. **Playground**: port the old `Playground` signals to `Control`s. Every old `ChoicePicker` → `Control::choice` with a typed table + options; `ToggleControl`/`CheckControl` → `Control::bool`; sliders → `Control::number`; free text → `Control::text`. Decide `Emit` per control: `Template` for `Option<_>` props and icons, `Skip` for demo-only knobs. Stateful components use the `XDemo { knobs }` pattern (§3.2). Choose the `Stage`. Drop debug readouts ("Stato reattivo Signal", counters) unless they are the point of the component (hooks).
3. **Examples**: move every old "Catalogo / Stati / Varianti" `Card` gallery into `Example`s (`Span::Half`), each with a private `#[component]` and a `const *_CODE`. Add one `In contesto` `Span::Full` example: a scene (`crate::scenes::*`) or an inline composition where the component does a real job. Trivial components (Divider, Burger, CloseButton…) may have only two examples and no playground.
4. **Props**: port every old `PropRow` to `prop(...)`; merge pairs like `left_icon / right_icon`; one `PropGroup` per documented component; check names and types against the signature in `crate/src/<feature>.rs` (the old tables had drift).
5. **Recipe and a11y**: fill `Recipe` from `language.md` (recipe §, depth ladder row, 4–8 tokens) and 3–5 a11y bullets that are true for the implementation (read the component's ARIA code).
6. **Code fidelity**: `Example.code` is the body of its component's `rsx!` (plus preceding signal lines when present), pasted verbatim; the only allowed differences are removed showcase-only classes (`sc-*`) and the removal of `key` attributes.
7. **Hygiene**: no `ShowcaseTabs`, `FeatureBanner`, `PropRow`, `PageHeader`, `ChoicePicker`, `ToggleControl`, `Counter` imports; no hand-rolled `button`/`input`/`a`; no hex and no colour utilities (`python scripts/lint_design.py --showcase`); no `with_color("#…")` on chart series (use token fallback; `sd-lint-allow` is not accepted in stories); Italian copy with correct accents; `status: Status::Stable`.
8. **Related**: 3–6 slugs that exist (`cargo test -p showcase` checks).
9. **Verify**: `SHOWCASE_ALLOW_STUBS=1 cargo test -p showcase`, `cargo clippy -p showcase`, open `/slug` in `dx serve`: knobs move the code, examples open their code, Crimson + Sharp + Glow Off leaves nothing violet or round, 390px has no horizontal scroll.
10. **Delete** `showcase/legacy/pages/<old>.rs`.

What to drop from old pages: duplicated tab panels, static code strings for the playground, "STATO REATTIVO" readouts, English-ish titles, descriptions that leak implementation details, `Card` wrappers around single widgets, `w-full`/`flex` utility soup where `Group`/`Stack`/`SimpleGrid` do the job.

---

## 6. Batches

Files are `showcase/src/stories/<file>`; the legacy source is `showcase/legacy/pages/<same file>` (identical names). Weights are the old page line counts; totals are 4309 / 4295 / 4563.

**Batch A — Azioni & Form + Contenuti & Social (27 files)**
`buttons.rs`, `inputs.rs`, `number_input.rs`, `pin_input.rs`, `textarea.rs`, `select.rs`, `checkbox.rs`, `radio.rs`, `switch.rs`, `slider.rs`, `rating.rs`, `chip.rs`, `color_picker.rs`, `segmented_controls.rs`, `date_picker.rs`, `dropzone.rs`, `fieldset.rs`, `form.rs`, `avatars.rs`, `facepile.rs`, `chat.rs`, `activity_feed.rs`, `attachment_cards.rs`, `document_card.rs`, `media_card.rs`, `rich_text.rs`, `code_highlight.rs`

**Batch B — Layout + Navigazione + Fondamenta story (26 files)**
`app_shell.rs`, `navbar.rs`, `grid.rs`, `group.rs`, `stack.rs`, `center.rs`, `aspect_ratio.rs`, `cards.rs`, `scroll_area.rs`, `dividers.rs`, `resizable.rs`, `masonry.rs`, `accordion.rs`, `spoiler.rs`, `navigation_menu.rs`, `overflow_set.rs`, `nav_link.rs`, `tabs.rs`, `breadcrumbs.rs`, `pagination.rs`, `stepper.rs`, `bottom_navigation.rs`, `menubar.rs`, `burger.rs`, `command_palette.rs`, `animations.rs`

**Batch C — Overlay + Dati & Feedback + Utility (32 files)**
`modals.rs`, `alert_dialog.rs`, `drawer.rs`, `dropdown.rs`, `popover.rs`, `hover_card.rs`, `context_menu.rs`, `tooltip.rs`, `coachmark.rs`, `speed_dial.rs`, `affix.rs`, `keytip.rs`, `tables.rs`, `lists.rs`, `timelines.rs`, `badges.rs`, `alerts.rs`, `notifications.rs`, `progress.rs`, `skeletons.rs`, `trees.rs`, `indicator.rs`, `loading_overlay.rs`, `virtual_list.rs`, `transfer_list.rs`, `carousel.rs`, `charts.rs`, `close_button.rs`, `copy_button.rs`, `highlight.rs`, `marquee_selection.rs`, `reactive_hooks.rs`

Story-specific notes:
- `charts.rs`: no playground; one `Span::Half` example per chart type (Area, Bar, Line, Pie, Radar, Scatter, Sparkline, Gauge, Heatmap) with series colours from the token fallback, plus `OpsDashboard` as `In contesto`. All ten visible at once.
- `reactive_hooks.rs`: no playground; one example per hook (`use_hotkey`, `use_debounced_signal`, `use_clipboard`, `use_local_storage`, `use_media_query`, `use_idle`, `use_window_scroll`, `use_click_outside`, `use_intersection_observer`), each with a live readout. `recipe: None`.
- `animations.rs`: examples re-trigger via a `key` bump on a `Button` click; `recipe` points to language.md §5.
- Overlay stories (`modals`, `drawer`, `alert_dialog`, `command_palette`, `coachmark`): playground `Stage::Fill`, the trigger sits inside a mini composition (toolbar or card), never alone in the stage.
- `select.rs` and `date_picker.rs`: keep `overflow: visible` on the stage; if the menu still renders see-through or clipped, do not patch in the showcase; it is library proposal §8.1/§8.2.

---

## 7. Acceptance criteria

Structural (verifier runs from the workspace root inside `nix develop`):

1. `cargo check --workspace`, `cargo clippy --workspace` (no new warnings), `cargo test -p showcase` **without** `SHOWCASE_ALLOW_STUBS` pass. `python scripts/lint_design.py --showcase` passes.
2. `showcase/src/pages/` contains exactly `mod.rs, home.rs, tokens.rs, theme_builder.rs, typography.rs, icons.rs, not_found.rs`. `showcase/legacy/` does not exist. `showcase/src/components/` contains only `mod.rs` and `clipboard.rs`.
3. `grep -rn "ShowcaseTabs\|FeatureBanner\|PropRow\|PageHeader\|ChoicePicker\|ToggleControl" showcase/src` returns nothing.
4. `grep -rnE "^\s*(button|input|select|textarea|a) \{" showcase/src` returns nothing (chrome and stories use library components; `Link` is the only anchor).
5. `grep -rn "with_color(\"#" showcase/src` returns nothing; `grep -rn "sd-lint-allow" showcase/src` returns nothing.
6. `stories/mod.rs` has 85 `ALL` entries; every entry's `slug` equals an old route path from the previous `Route` enum (list in git history, `main.rs` at commit `5815872`), so every old URL resolves to a story. `/theme-builder`, `/typography`, `/icons`, `/` resolve to pages; `/tokens` is new; `/does-not-exist` renders `NotFound`.
7. Every story: `status == Stable`; a playground or ≥ 2 examples; ≥ 1 `In contesto` example (`Span::Full`) except `reactive_hooks`, `animations`, `burger`, `close_button`, `copy_button`, `highlight`, `keytip`, `marquee_selection`; ≥ 1 `PropGroup`; `recipe.is_some()` except `reactive_hooks`; a11y ≥ 3 bullets.
8. `showcase.scss`: no hex, no `rgb(`, no numeric `z-index`, no raw `px` font sizes; every `.sc-*` class used in `showcase/src` exists in the compiled CSS (`python scripts/check_classes.py`).

Visual (1440×900 and 390×844, default theme unless stated; use the capture scripts from the survey folder as a base):

9. Home above the fold at 1440 shows the title, two CTAs, the five swatches and the live `OpsDashboard` with a rendered chart and table; no tag cloud anywhere on the page.
10. Preset wall: five tiles with five visibly different accents on the same screenshot; clicking "Usa Crimson" recolours the header dot, the sidebar active link, the hero scene and the primary buttons; the Crimson tile becomes "Preset attivo".
11. Story page `/buttons`: toggling "Icona destra" adds the `right_icon:` line to the code block and it is highlighted; switching variant to Ghost changes the live button and the `variant:` line; the Cargo line reads `features = ["button"]` with the real `VERSION`.
12. `/select`: four props tabs (Select, MultiSelect, Combobox, SelectItem); the open menu is not clipped by the stage (if it is see-through, it is logged as library bug §8.1, not a showcase failure).
13. `/charts`: all nine chart types visible without interacting; series colours change with the preset (chart-1 follows the accent).
14. Header at 1440: brand, search button with `Kbd`, theme button; no swatches, no segmented controls in the header. Opening the theme button shows a `Popover` with swatches, Raggio, Glow, Theme Builder.
15. Mobile 390: header is one row ≤ 56px; drawer opens with a search field at top and the active item visible; `scrollWidth == 390` on `/`, `/buttons`, `/select`, `/tables`, `/charts`; the preset wall scrolls horizontally; the playground stacks stage then knobs.
16. Crimson + Sharp + Glow Off on `/`, `/buttons`, `/cards`, `/tables`: no violet pixel in UI chrome (sampled on the active sidebar link, primary button, focused field), no rounded corner above 4px on cards/buttons, no visible glow halo.
17. Theme Builder: the "Auto-save" and "Checkbox di verifica" labels are on one line; preview cards have equal heights in their row.
18. Every page title and card title renders in Kanit, body in Onest, eyebrows and code in Source Code Pro (inspect computed `font-family`).

---

## 8. Library proposals (not implemented here)

1. **Select menu stacking** (`crate/src/select.rs`, `_select.scss`): the open listbox renders see-through over following fields (survey `state-select.png`). Give the menu the raised recipe fill (`--sd-surface-2`) and `z('dropdown')`, or portal it; check `MultiSelect` and `Combobox` too.
2. **DatePicker popup clipping** (`date_picker.rs`): the calendar is clipped by its container; it needs a floating layer with `z('popover')` and no `overflow: hidden` ancestor dependency.
3. **SegmentedControl label truncation**: "Expressive" truncates at `Sm`; items need `min-width: max-content` or the control needs `full_width` to distribute.
4. **Props metadata**: a `#[derive(PropsDoc)]`-style export (or a build-time rustdoc JSON scrape) so the showcase props tables derive from the source (ADR-0006 deferred item).
5. **Display type size**: a `--sd-text-4xl` (36px) token for hero titles would remove the showcase `calc()` derivation.
6. **Router-aware `Breadcrumbs`/`NavLink`**: both accept only `href`; an `on_click`-only mode that renders a `<button>` (NavLink) and a slot for a custom link element (Breadcrumbs) would let docs sites use the Dioxus `Link` without full reloads.
7. **Category icons**: `IconKind` lacks obvious glyphs for Form/Layout/Overlay/Data (e.g. `LayoutGrid`, `MousePointer`, `Table`, `Bell`, `Type`); the sidebar reuses `Terminal`/`Box`/`Layers` today.
8. **Chart series colour API**: `ChartSeries::with_color` takes a hex string; a `with_token(n)` (→ `var(--sd-chart-n)`) would make themed series explicit and lint-clean.
9. **`CodeBlock` language `rsx`**: confirm the tokenizer treats `rsx` like `rust` (keywords, strings, types); the generated samples use it everywhere.
10. **`ThemeProvider` documentation**: nested providers work (display: contents + inline variables); document it as the supported way to preview a theme in isolation, and expose `Theme::for_preset(PrimaryColor) -> Theme` so the preset wall does not need a table of constructor pointers.
