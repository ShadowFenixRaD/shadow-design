# Showcase revamp: batch migration guide (phase 2)

For the three engineers who migrate stories in parallel. Binding: ADR-0001…0006, `docs/design/*`, `docs/showcase/REVAMP.md`. This guide only adds what phase 1 decided while building the framework; where it is silent, REVAMP.md applies.

Phase 1 is done: the story engine, the new shell, the home/tokens/typography/icons/theme-builder/404 pages, the four scenes and `showcase.scss` are in place. Every story is registered. `Button` (`stories/buttons.rs`) and `Select` (`stories/select.rs`) are fully migrated and are **the references**: copy their structure.

---

## 1. Where things are

| Path | What | Who edits |
| :-- | :-- | :-- |
| `showcase/src/stories/<file>.rs` | One story per component: `pub fn story() -> Story` plus private `#[component]` helpers | **You, only the files of your batch** |
| `showcase/legacy/pages/<file>.rs` | The old page, same file name as the story (not compiled) | **You delete it** when the story is done |
| `showcase/legacy/components/*.rs` | Old `ShowcaseTabs`, `ChoicePicker`, `PropRow`, layout, nav (not compiled), to read old pages | Nobody (phase 3 deletes `legacy/`) |
| `showcase/src/stories/mod.rs` | The 85 `pub mod` lines and the `ALL` list, in sidebar order | Nobody |
| `showcase/src/story/*` | Schema, knobs + code generator, knob panel, registry + tests, page template, sections | Nobody |
| `showcase/src/scenes/*` | `OpsDashboard`, `SettingsForm`, `InboxPane`, `AuthCard` (`compact: bool`) | Nobody (read-only, compose them) |
| `showcase/src/app/*`, `pages/*`, `components/*`, `main.rs` | Chrome, non-component pages, routing | Nobody |
| `showcase/assets/showcase.scss` | All `.sc-*` classes | Nobody |
| `crate/` | The library | Nobody (library bugs go to REVAMP §8) |

If you need something from a shared file (a new `.sc-*` class, a new `Kind`, a scene variant, a library fix), send a one-line request to the phase-1 owner and keep going with what exists. Do not edit the shared file yourself: three batches run at once and the only thing that keeps them conflict-free is that each one touches its own files.

## 2. The stub contract

Every story file you own currently looks like this:

```rust
//! Story: Tooltip (feature `tooltip`).
//!
//! STUB (phase 1, ADR-0006). Migrate from `showcase/legacy/pages/tooltip.rs` …
use shadow_design::prelude::IconKind;

use crate::story::schema::*;

pub fn story() -> Story {
    Story::stub("tooltip", "Tooltip", Category::Overlay, &["tooltip"], IconKind::Info,
        "Etichetta flottante al passaggio del puntatore, in quattro direzioni.")
}
```

The contract between your file and the rest of the app:

1. The file exposes exactly `pub fn story() -> Story`. Its name, its module name and the file name never change (they are listed in `stories/mod.rs`).
2. `slug` never changes: it is the old URL (`/tooltip`), the route and the registry key. `category` stays as given (REVAMP §2.1); `features` must exist in `crate/Cargo.toml`.
3. `name`, `icon`, `tagline` may be improved (checklist §5 item 1). The stub's name is the Rust component name; keep it unless the story documents a different primary component.
4. Anything else the story needs lives in the same file as private items (`fn`, `const`, `#[component] fn`). Nothing in the file is `pub` except `story()`.
5. A stub renders head + "Storia in migrazione" + prev/next. A migrated story sets `status: Status::Stable` and must pass the registry tests (§5).

## 3. Steps for one story

Do them in order, one story at a time, and commit each finished story on its own.

1. **Read** `showcase/legacy/pages/<file>.rs` and the component's source in `crate/src/<feature>.rs` (or `<feature>/mod.rs`). Old props tables drift: the source signature wins.
2. **Open the reference**: `stories/buttons.rs` for a simple component, `stories/select.rs` for a stateful or compound one.
3. **Rewrite** `stories/<file>.rs` as a full `Story { … }` literal (no `Story::stub`):
   - identity: `slug` unchanged, `name`, `category` unchanged, `features`, `icon`, `tagline` (one Italian sentence, no internals, no marketing), `keywords` (≥ 4, lowercase, Italian and English);
   - `playground` (or none for trivial components, §4.4), `examples`, `props`, `recipe`, `a11y` (≥ 3), `related` (3–6 existing slugs), `status: Status::Stable`.
4. **Playground** (§4): port every old picker/toggle/slider to a `Control`, pick the `Emit` per control, choose the `Stage`, use the `XDemo { knobs }` pattern when the component needs signals.
5. **Examples**: every old "Catalogo / Stati / Varianti" gallery becomes an `Example` (`Span::Half`) with its own private `#[component]` and a `const *_CODE` holding the same RSX. Add one `Span::Full` example titled "In contesto" (a scene or an inline composition doing a real job). Exceptions without "In contesto": `reactive_hooks`, `animations`, `burger`, `close_button`, `copy_button`, `highlight`, `keytip`, `marquee_selection`.
6. **Code fidelity**: each `*_CODE` is the body of its component's `rsx!` (plus the signal lines before it), pasted verbatim. The only allowed differences: removed `sc-*` classes and removed `key` attributes. Easiest way: write the `rsx!` first, then copy it into the raw string and delete the `class: "sc-…"` attributes.
7. **Props**: one `PropGroup` per documented component; merge pairs (`left_icon / right_icon`); check each name and type against the source.
8. **Recipe + a11y**: recipe from `docs/design/language.md` §3 (name the §, depth ladder row, 4–8 tokens, one note). a11y bullets must be true: read the component's ARIA code before writing them.
9. **Verify** (§6). Open `/<slug>` in `dx serve` and check: knobs move the live component and the code, the changed line is highlighted, examples open their code, Crimson + Sharp + Glow Off leaves nothing violet or round, 390px has no horizontal scroll.
10. **Delete** `showcase/legacy/pages/<file>.rs` in the same commit.

Drop from old pages: duplicated tab panels, static code strings, "STATO REATTIVO" readouts and click counters (except in `reactive_hooks`), `Card` wrappers around single widgets, `w-full`/`flex` utility soup (use `Group`/`Stack`/`SimpleGrid`), English-ish titles.

## 4. Engine cheat sheet

Everything below is in `crate::story::schema::*` (one glob import, as in the stubs) unless noted.

### 4.1 Controls and code generation

| Constructor | Knob | Read it in `render` |
| :-- | :-- | :-- |
| `Control::choice(key, label, OPTS, default_index)` | `SegmentedControl` (≤ 5 options) or `Select` | `k.pick(key, &TABLE)` (typed) or `k.index(key)` |
| `Control::bool(key, label, default)` | `Switch` (grouped under "Opzioni") | `k.bool(key)` |
| `Control::text(key, label, default)` | `TextInput` | `k.text(key)` |
| `Control::number(key, label, default, min, max, step, decimals)` | `Slider` with readout | `k.number(key)` |
| `Control::color(key, label, default)` | `ColorInput` | `k.color(key)` |

Chain `.emit(Emit::…)` and `.hint("…")`. How each control appears in the generated code:

| `Emit` | Printed |
| :-- | :-- |
| `IfChanged` (default) | `key: <literal>,` only when the value differs from the default. Use for plain props whose library default equals the knob default (`variant`, `size`, `loading`). |
| `Always` | `key: <literal>,` on every render. |
| `Template("…{v}…")` | The template while the knob is **on**: a `Bool` when true, a `Text`/`Color` when non-empty, a `Choice`/`Number` when changed. Use it for `Option<_>` props and icons (`label: Some("{v}".to_string())`, `left_icon: Some(rsx! { … })`). |
| `Skip` | Never: the knob only shapes the demo (children text, stage width). Put the text in `children_code: Some("\"{label}\"")` instead. |

Phase-1 change to REVAMP §4.2: a `Template` is printed while it is "on", **including at the default**. With the spec's rule, Button's icon (on by default) was visible live but missing from the code; now the code always matches the live component.

`Playground` fields: `component` (block head), `controls`, `render: fn(&Knobs) -> Element`, `prelude_code` (lines before the block: signals, data; must be the same data the demo uses), `fixed_code` (lines always printed first inside the block, e.g. `&["data,", "value,"]`), `children_code`, `stage`.

| `Stage` | Use |
| :-- | :-- |
| `Center` | content-sized: buttons, badges, switches |
| `Start` | full width from the top-left: lists, tables, navigation |
| `Form` | a centred 24rem column: fields, selects, cards |
| `Fill` | full width, ≥ 20rem: layout demos and **every overlay** (modals, drawer, alert dialog, command palette, coachmark), with the trigger inside a mini composition |

### 4.2 Gotchas the compiler or the tests will catch

- **Choice options must be a `const`.** `Control::choice("v", "V", &[opt(..), opt(..)], 0)` does not compile (`opt(..)` results are not promoted to `'static`). Write `const VARIANT_OPTS: &[ChoiceOption] = &[opt("Primary", "ButtonVariant::Primary"), …];` and a matching typed table `const VARIANTS: [ButtonVariant; N] = [...]` in the same order.
- **`render` is a plain `fn`**: no hooks in it. A component that needs a signal goes through a private `#[component] fn XDemo(knobs: Knobs) -> Element` and `fn render(k: &Knobs) -> Element { rsx! { XDemo { knobs: k.clone() } } }` (see `select.rs`).
- **`Example.render` is `fn() -> Element`**: `render: || rsx! { Variants {} }`, with the hooks inside the private component.
- **No prop named `build`** on a private `#[component]` (it collides with the props builder), and **no `fn` pointer props** (`unpredictable_function_pointer_comparisons` warning): pass an index into a `const` table instead.
- **Example ids** must be unique per story; **example code** must not contain `sc-` classes (registry test).

### 4.3 Shared vocabulary you may use

- `crate::scenes::{OpsDashboard, SettingsForm, InboxPane, AuthCard}` with `compact: bool`, for "In contesto" examples. REVAMP §1.3 lists which story uses which scene. A scene's `*_CODE` may be a short excerpt with a `// scenes/<file>.rs (estratto)` comment, as in `select.rs`.
- `crate::story::sections::DemoCell { index, label, tall, short, accent }`: the static dashed tile for Grid/Group/Stack/Masonry/Center/AspectRatio demos. Never a `Button` as a placeholder tile.
- `.sc-*` classes for showcase-only text and layout: `sc-muted-text`, `sc-subtle-text`, `sc-accent-text`, `sc-success-text`, `sc-warning-text`, `sc-danger-text`, `sc-info-text`, `sc-mono`, `sc-num` (tabular mono), `sc-hint`, `sc-eyebrow`, `sc-readout`, `sc-counter`, `sc-stack`, `sc-stack--loose`, `sc-actions`, `sc-title-row`, `sc-card-grid(--3)`, `sc-demo-frame`, `sc-list`. Layout utilities from the library (`flex`, `gap-*`, `w-full`, …) are allowed but `Group`/`Stack`/`SimpleGrid` are preferred. Colour utilities and hex are never allowed.
- `use_notification()` for toasts (the app root provides `NotificationProvider`).

### 4.4 Trivial components

`Divider`, `Burger`, `CloseButton`, `Center`, … may skip the playground and ship two or more examples instead (`playground: None`). They still need props, recipe, a11y, keywords and related.

## 5. Rules (REVAMP §5 and §7, condensed)

- No hand-written interactive elements in a story: no `button {`, `input {`, `select {`, `textarea {`, `a {` with behaviour. Use library components.
- No `space-y-*` / `space-x-*` around overlays (Modal, Drawer, AlertDialog, CommandPalette): use `Stack`/`Group`. Phase 1 also fixed the library (`_utilities.scss` skips the fixed backdrops), but stories must not rely on margin utilities next to overlays.
- No hex anywhere in `showcase/src`, not even as data: the lint rejects any string with a hex colour unless it is a format string. For a colour value use a preset (`PrimaryColor::Cyan.hex().to_string()`) or a token (`var(--sd-chart-3)`). `// sd-lint-allow` is not accepted in the showcase.
- Charts: never `ChartSeries::with_color(..)`; series take `--sd-chart-n` by fallback, so chart-1 follows the accent.
- `select.rs` / `date_picker.rs`: if a menu or calendar renders see-through or clipped, do not patch it in the story; it is library proposal §8.1/§8.2.
- Italian copy with correct accents; identifiers, code and token names in English.

## 6. Verify before each commit

From the repo root:

```bash
nix develop --command cargo check -p showcase
nix develop --command cargo clippy -p showcase          # no warnings
SHOWCASE_ALLOW_STUBS=1 nix develop --command cargo test -p showcase
nix develop --command python scripts/lint_design.py --showcase
nix develop --command python scripts/check_classes.py   # every class you use exists
cd showcase && nix develop --command dx serve            # open /<slug>
```

The pre-commit hook runs the design lint on staged files. `cargo test` checks for every non-stub story: a playground or ≥ 2 examples, ≥ 1 `PropGroup`, ≥ 3 a11y bullets, ≥ 4 keywords, related slugs that exist, unique example ids, no `sc-` in example code, features present in `crate/Cargo.toml`. Commit message: `feat(showcase): migrate <Name> story (ADR-0006)`.

When your whole batch is done, run `cargo test -p showcase` **without** `SHOWCASE_ALLOW_STUBS` filtered to your batch by eye: `no_stub_remains` lists the stubs left, and none of them may be yours.

## 7. Batches

Weights are the old page line counts. File = `showcase/src/stories/<file>`, legacy source = `showcase/legacy/pages/<file>` (same name; `animations.rs` was `pages/animation.rs`).

### Batch A — Azioni & Form + Contenuti & Social (27 files, 25 to migrate)

| File | URL | Name | Legacy lines | State |
| :-- | :-- | :-- | --: | :-- |
| `buttons.rs` | `/buttons` | Button | 199 | done (reference) |
| `inputs.rs` | `/inputs` | TextInput | 162 | stub |
| `number_input.rs` | `/number-input` | NumberInput | 174 | stub |
| `pin_input.rs` | `/pin-input` | PinInput | 239 | stub |
| `textarea.rs` | `/textarea` | Textarea | 247 | stub |
| `select.rs` | `/select` | Select | 410 | done (reference) |
| `checkbox.rs` | `/checkbox` | Checkbox | 155 | stub |
| `radio.rs` | `/radio` | RadioGroup | 173 | stub |
| `switch.rs` | `/switch` | Switch | 103 | stub |
| `slider.rs` | `/slider` | Slider | 152 | stub |
| `rating.rs` | `/rating` | Rating | 168 | stub |
| `chip.rs` | `/chip` | Chip | 210 | stub |
| `color_picker.rs` | `/color-picker` | ColorPicker | 175 | stub |
| `segmented_controls.rs` | `/segmented-controls` | SegmentedControl | 128 | stub |
| `date_picker.rs` | `/date-picker` | DatePicker | 126 | stub |
| `dropzone.rs` | `/dropzone` | Dropzone | 149 | stub |
| `fieldset.rs` | `/fieldset` | Fieldset | 115 | stub |
| `form.rs` | `/form` | Form | 214 | stub |
| `avatars.rs` | `/avatars` | Avatar | 117 | stub |
| `facepile.rs` | `/facepile` | Facepile | 93 | stub |
| `chat.rs` | `/chat` | MessageBubble | 128 | stub |
| `activity_feed.rs` | `/activity-feed` | ActivityFeed | 110 | stub |
| `attachment_cards.rs` | `/attachment-cards` | AttachmentCard | 135 | stub |
| `document_card.rs` | `/document-card` | DocumentCard | 97 | stub |
| `media_card.rs` | `/media-card` | MediaCard | 95 | stub |
| `rich_text.rs` | `/rich-text` | RichTextEditor | 78 | stub |
| `code_highlight.rs` | `/code-highlight` | CodeHighlight | 157 | stub |

Scenes for "In contesto": `SettingsForm` (inputs, switch, slider, radio, checkbox, fieldset, form, color-picker), `AuthCard` (pin-input), `OpsDashboard` (avatars, segmented-controls). The legacy files of `buttons.rs` and `select.rs` are already deleted.

### Batch B — Layout + Navigazione + Fondamenta story (26 files)

| File | URL | Name | Legacy lines | State |
| :-- | :-- | :-- | --: | :-- |
| `app_shell.rs` | `/app-shell` | AppShell | 291 | stub |
| `navbar.rs` | `/navbar` | Navbar | 92 | stub |
| `grid.rs` | `/grid` | Grid | 197 | stub |
| `group.rs` | `/group` | Group | 132 | stub |
| `stack.rs` | `/stack` | Stack | 121 | stub |
| `center.rs` | `/center` | Center | 96 | stub |
| `aspect_ratio.rs` | `/aspect-ratio` | AspectRatio | 111 | stub |
| `cards.rs` | `/cards` | Card | 283 | stub |
| `scroll_area.rs` | `/scroll-area` | ScrollArea | 195 | stub |
| `dividers.rs` | `/dividers` | Divider | 93 | stub |
| `resizable.rs` | `/resizable` | ResizablePanelGroup | 137 | stub |
| `masonry.rs` | `/masonry` | MasonryGrid | 89 | stub |
| `accordion.rs` | `/accordion` | Accordion | 188 | stub |
| `spoiler.rs` | `/spoiler` | Spoiler | 142 | stub |
| `navigation_menu.rs` | `/navigation-menu` | NavigationMenu | 118 | stub |
| `overflow_set.rs` | `/overflow-set` | OverflowSet | 98 | stub |
| `nav_link.rs` | `/nav-link` | NavLink | 269 | stub |
| `tabs.rs` | `/tabs` | Tabs | 145 | stub |
| `breadcrumbs.rs` | `/breadcrumbs` | Breadcrumbs | 167 | stub |
| `pagination.rs` | `/pagination` | Pagination | 176 | stub |
| `stepper.rs` | `/stepper` | Stepper | 242 | stub |
| `bottom_navigation.rs` | `/bottom-navigation` | BottomNavigation | 216 | stub |
| `menubar.rs` | `/menubar` | Menubar | 238 | stub |
| `burger.rs` | `/burger` | Burger | 89 | stub |
| `command_palette.rs` | `/command-palette` | CommandPalette | 150 | stub |
| `animations.rs` | `/animations` | Transition | 110 | stub |

Scenes: `OpsDashboard` (cards), `AuthCard` (dividers), `InboxPane` (tabs, pagination). `animations.rs`: examples re-trigger through a `key` bump on a `Button` click, `recipe` points to language.md §5. `command_palette.rs` is an overlay: `Stage::Fill`.

### Batch C — Overlay + Dati & Feedback + Utility (32 files)

| File | URL | Name | Legacy lines | State |
| :-- | :-- | :-- | --: | :-- |
| `modals.rs` | `/modals` | Modal | 119 | stub |
| `alert_dialog.rs` | `/alert-dialog` | AlertDialog | 140 | stub |
| `drawer.rs` | `/drawer` | Drawer | 138 | stub |
| `dropdown.rs` | `/dropdown` | Dropdown | 203 | stub |
| `popover.rs` | `/popover` | Popover | 151 | stub |
| `hover_card.rs` | `/hover-card` | HoverCard | 111 | stub |
| `context_menu.rs` | `/context-menu` | ContextMenu | 157 | stub |
| `tooltip.rs` | `/tooltip` | Tooltip | 92 | stub |
| `coachmark.rs` | `/coachmark` | Coachmark | 144 | stub |
| `speed_dial.rs` | `/speed-dial` | SpeedDial | 148 | stub |
| `affix.rs` | `/affix` | Affix | 107 | stub |
| `keytip.rs` | `/keytip` | Keytip | 85 | stub |
| `tables.rs` | `/tables` | DataTable | 247 | stub |
| `lists.rs` | `/lists` | List | 242 | stub |
| `timelines.rs` | `/timelines` | Timeline | 172 | stub |
| `badges.rs` | `/badges` | Badge | 141 | stub |
| `alerts.rs` | `/alerts` | Alert | 118 | stub |
| `notifications.rs` | `/notifications` | Notification | 154 | stub |
| `progress.rs` | `/progress` | ProgressBar | 115 | stub |
| `skeletons.rs` | `/skeletons` | Skeleton | 117 | stub |
| `trees.rs` | `/trees` | TreeView | 119 | stub |
| `indicator.rs` | `/indicator` | Indicator | 133 | stub |
| `loading_overlay.rs` | `/loading-overlay` | LoadingOverlay | 104 | stub |
| `virtual_list.rs` | `/virtual-list` | VirtualList | 103 | stub |
| `transfer_list.rs` | `/transfer-list` | TransferList | 90 | stub |
| `carousel.rs` | `/carousel` | Carousel | 166 | stub |
| `charts.rs` | `/charts` | Charts | 309 | stub |
| `close_button.rs` | `/close-button` | CloseButton | 102 | stub |
| `copy_button.rs` | `/copy-button` | CopyButton | 92 | stub |
| `highlight.rs` | `/highlight` | Highlight | 92 | stub |
| `marquee_selection.rs` | `/marquee-selection` | MarqueeSelection | 140 | stub |
| `reactive_hooks.rs` | `/reactive-hooks` | Hooks | 212 | stub |

Scenes: `OpsDashboard` (charts, tables, badges), `InboxPane` (lists, indicator, context-menu, dropdown, skeletons), `AuthCard` (alerts, and modals: open it inside a `Modal`). Overlays (`modals`, `drawer`, `alert_dialog`, `coachmark`): `Stage::Fill`, trigger inside a small toolbar or card, never alone; the legacy `modals.rs` and `drawer.rs` wrap the overlay in `div { class: "space-y-4" }`, which you must not port (§5). `charts.rs`: no playground; one `Span::Half` example per chart type (Area, Bar, Line, Pie, Radar, Scatter, Sparkline, Gauge, Heatmap), all visible at once, plus `OpsDashboard` as "In contesto". `reactive_hooks.rs`: no playground, one example per hook with a live readout, `recipe: None`.
