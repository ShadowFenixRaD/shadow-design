# AGENTS.md — Shadow Design Workspace Guide

Istruzioni operative, architettura e standard di sviluppo per agenti AI e sviluppatori che lavorano su **Shadow Design**.

---

## 1. Panoramica del Progetto

**Shadow Design** è una libreria di componenti UI modulare e ad alte prestazioni progettata per **Dioxus 0.7** (applicazioni Web, Desktop e Mobile), nata all'interno dell'ecosistema **Shadowfenix**.

### Filosofia Estetica
- **Sfondo**: Nero assoluto (`#000000`, `bg-black`).
- **Accenti Cromatici**: Viola scuro intenso (`purple-950`, `purple-900`, `purple-800`), con preset alternativi Obsidian (monocromatico) e Crimson (fenice rossa).
- **Dettagli**: Micro-glow (`shadow-[0_0_20px_rgba(107,33,168,0.3)]`), bordi semi-trasparenti sottili e contrasti elevati.
- **Modularità**: Architettura *zero-bloat* dove ogni componente è vincolato a una feature flag Cargo indipendente.

---

## 2. Struttura del Workspace Cargo

Il progetto è configurato come un Cargo workspace con due membri principali:

```text
shadow-design/
├── Cargo.toml               # Workspace root (resolver = "2", Dioxus 0.7.1)
├── Cargo.lock               # Dipendenze bloccate del workspace
├── clippy.toml              # Regole di linting per Dioxus 0.7 / Generational Box
├── AGENTS.md                # Questo file d'istruzioni
├── crate/                   # Crate libreria: shadow-design
│   ├── Cargo.toml           # Manifest della libreria con feature flags
│   ├── scss/                # Sistema di stile nativo Pure SCSS
│   │   ├── main.scss        # Entrypoint SCSS master esportato come Asset
│   │   ├── _variables.scss  # Token e scale cromatiche
│   │   ├── _mixins.scss     # Mixin tattili, specular, depth e ambient glow
│   │   ├── _reset.scss      # Reset CSS preflight (@layer sd-reset)
│   │   ├── _utilities.scss  # Utilità universali (@layer sd-utilities)
│   │   ├── _theme_presets.scss # Stili dei 5 temi e scale di personalizzazione
│   │   └── components/      # Moduli SCSS dedicati per ogni componente
│   └── src/
│       ├── lib.rs           # Entry point libreria, re-export condizionali e prelude
│       ├── a11y.rs          # Helper di accessibilità ARIA e ID univoci
│       ├── alert.rs         # Componente Alert con icone automatiche
│       ├── animation.rs     # Transizioni ottiche e micro-animazioni
│       ├── badge.rs         # Componente Badge con stili pill/rettangolari
│       ├── button.rs        # Componente Button, varianti e dimensioni
│       ├── card.rs          # Componente Card con header, footer e ombre
│       ├── checkbox.rs      # Caselle di selezione tattili (Sm/Md/Lg)
│       ├── compat.rs        # Layer di retrocompatibilità
│       ├── drawer.rs        # Pannelli laterali a scomparsa da sx/dx
│       ├── dropdown.rs      # Menu a tendina contestuali ancorati
│       ├── glow.rs          # Risolutore del micro-glow
│       ├── icon.rs          # Modulo icone native SVG (50 IconKind)
│       ├── input.rs         # Componente TextInput collegato a Signal<String>
│       ├── modal.rs         # Finestre di dialogo in overlay con backdrop
│       ├── navbar.rs        # Componente Navbar responsive
│       ├── notification.rs  # Sistema Toast globale con NotificationProvider
│       ├── select.rs        # Select, MultiSelect e Combobox filtrabili
│       ├── switch.rs        # Interruttori a scatto tattili ad alta precisione
│       ├── tabs.rs          # Navigazione a schede orizzontali stile Mantine
│       ├── textarea.rs      # Campi di testo multilinea con autosize
│       ├── theme.rs         # Sistema reattivo del tema, Context e Preset
│       ├── tooltip.rs       # Micro-etichette fluttuanti su hover a 4 direzioni
│       ├── typography.rs    # Componenti tipografici basati su Geist
│       └── util.rs          # Utility class builder cx!
└── showcase/                # Crate applicativo: Espositore componenti
    ├── Cargo.toml           # Manifest dell'app (dipende da shadow-design { all })
    ├── Dioxus.toml          # Configurazione Dioxus CLI (dx)
    ├── assets/
    │   ├── critical.css     # CSS critico di boot (dark background immediato)
    │   ├── header.svg       # Logo vettoriale Shadowfenix
    │   └── favicon.ico      # Icona dell'applicazione
    └── src/
        ├── main.rs          # App root, caricamento asset e Dioxus Router
        ├── components/
        │   ├── banner.rs    # Banner informativo con comando Cargo feature
        │   ├── clipboard.rs # Helper asincrono copy-to-clipboard con fallback
        │   ├── code_block.rs# Visualizzatore codice con syntax highlighting
        │   ├── highlight.rs # Tokenizer sintattico per Rust/RSX
        │   ├── layout.rs    # Shell con Navbar, Sidebar responsive e Theme Switcher
        │   ├── mod.rs       # Moduli componenti showcase
        │   ├── nav.rs       # Registry componenti e SearchPalette (Ctrl+K)
        │   ├── props_table.rs# Tabella documentazione props delle API
        │   └── showcase_tabs.rs # Tabs Playground / Codice RSX / API Props
        └── pages/
            ├── mod.rs
            ├── alerts.rs    # Showcase di Alert
            ├── animation.rs # Playground interattivo Transition
            ├── badges.rs    # Showcase di Badge e varianti
            ├── buttons.rs   # Showcase e playground interattivo di Button
            ├── cards.rs     # Showcase di Card e livelli di ombra
            ├── checkbox.rs  # Showcase di Checkbox tattile
            ├── drawer.rs    # Showcase di Drawer laterale
            ├── dropdown.rs  # Showcase di Dropdown contestuale
            ├── home.rs      # Panoramica, hero e test dinamico del Theme
            ├── icons.rs     # Griglia ricercabile di tutti i 50 IconKind
            ├── inputs.rs    # Showcase di TextInput con binding reattivo
            ├── modals.rs    # Showcase e test di apertura modale
            ├── navbar.rs    # Anteprima di Navbar
            ├── notifications.rs # Showcase del Toast Provider
            ├── select.rs    # Showcase di Select, MultiSelect e Combobox
            ├── switch.rs    # Showcase di Switch tattile
            ├── tabs.rs      # Showcase di Tabs orizzontali
            ├── textarea.rs  # Showcase di Textarea autosize
            ├── theme_builder.rs # Studio visuale Theme Builder
            ├── tooltip.rs   # Showcase di Tooltip su hover
            └── typography.rs# Showcase della scala tipografica Geist
```

---

## 3. Crate Libreria (`crate` / `shadow-design`)

### Feature Flags Cargo
Il file `crate/Cargo.toml` definisce feature individuali per consentire ai consumatori di importare esclusivamente ciò di cui necessitano:

| Feature Flag | Descrizione | Modulo |
| :--- | :--- | :--- |
| `all` | Abilita l'intera suite di componenti e il tema | Include tutte le feature sottostanti |
| `button` | Pulsanti con varianti, dimensioni, loading spinner e stati tattili | `crate::button` |
| `card` | Schede contenitore con varianti (Interactive, Metric, Elevated) e ombre | `crate::card` |
| `input` | Input di testo reattivi con icone statiche/cliccabili (`on_left_icon_click`, `on_right_icon_click`), clear button e Signal<String> | `crate::input` |
| `badge` | Etichette di stato, indicatori a pillola e dot animato | `crate::badge` |
| `navbar` | Barra di navigazione responsive con specular top highlight | `crate::navbar` |
| `modal` | Finestre di dialogo in overlay con backdrop click-to-close | `crate::modal` |
| `alert` | Riquadri di notifica con icone SVG native e dismiss manuale | `crate::alert` |
| `theme` | Engine reattivo in stile Mantine (PrimaryColor, RadiusScale, GlowMode) | `crate::theme` |
| `icon` | Modulo icone vettoriali native SVG (stroke 1.5px, zero dipendenze) | `crate::icon` |
| `typography` | Componenti tipografici basati su Geist (Heading, Text, Kbd, Code) | `crate::typography` |
| `notification` | Sistema Toast globale (NotificationProvider, use_notification, dismiss) | `crate::notification` |
| `tabs` | Navigazione a schede orizzontali stile Mantine (TabList, Tab, TabPanel) | `crate::tabs` |
| `switch` | Interruttori a scatto tattili ad alta precisione con descrizioni | `crate::switch` |
| `tooltip` | Micro-etichette fluttuanti su hover a 4 direzioni (Top, Bottom, Left, Right) | `crate::tooltip` |
| `drawer` | Pannelli laterali a scomparsa da sinistra o destra con backdrop | `crate::drawer` |
| `dropdown` | Menu a tendina contestuali ancorati al trigger con dismiss automatico | `crate::dropdown` |
| `checkbox` | Caselle di selezione tattili (Sm/Md/Lg, indeterminate, error, glow) | `crate::checkbox` |
| `select` | Menu a discesa Select, MultiSelect e Combobox ricercabili | `crate::select` |
| `textarea` | Campi di testo multilinea con autosize reattivo e contatore | `crate::textarea` |
| `animation` | Sistema di transizioni ottiche e micro-animazioni Dioxus (Fade, Slide, Scale, Pulse) | `crate::animation` |
| `serde` | Serializzazione e deserializzazione JSON di Theme, Preset e Overrides | Dipendenza `serde` opzionale |

### Gerarchia Z-Index Standardizzata
Per prevenire clipping e sovrapposizioni errate (es. Dropdown clippato da Card o nascosto sotto altri elementi):
- `Card` e `ShowcaseTabs` non usano `overflow-hidden` forzato sull'intero contenitore per permettere ai menu fluttuanti di estendersi all'esterno.
- `Dropdown` supporta `DropdownPlacement` (`BottomRight`, `BottomLeft`, `TopRight`, `TopLeft`): quando posizionato vicino al fondo di una card, il preset `Top` apre il menu verso l'alto evitando di fuoriuscire dal layout.
- Quando aperto, il Dropdown riceve priorità assoluta: Backdrop `z-[995]`, Container `z-[999]`, Menu `z-[1000]`.
- Scala Z-index universale:
  - Header / Sticky: `z-30`
  - Drawer: Backdrop `z-[65]`, Panel `z-[70]`
  - Modal: Backdrop `z-[75]`, Dialog `z-[80]` / `z-[85]`
  - Notifications: Container `z-[90]`
  - Tooltips: Bubble `z-[100]`
  - Select Menu: Container `z-[999]`, Menu `z-[1000]`
  - Dropdown Attivo: Container `z-[999]`, Menu `z-[1000]`

### Architettura 100% SCSS & Supporto Nativo Dioxus `asset!()` nella Crate
TailwindCSS è stato completamente rimosso dal progetto. L'intera suite di stili è ora scritta in puro SCSS e risiede direttamente all'interno della crate libreria in `crate/scss/`, esposta nativamente tramite la macro di Dioxus in [crate/src/lib.rs](crate/src/lib.rs):
```rust
pub const STYLES: Asset = asset!("/scss/main.scss");
```
E consumata direttamente dalle applicazioni (come lo showcase) tramite:
```rust
document::Link { rel: "stylesheet", href: shadow_design::STYLES }
```
- `_variables.scss`: Token per colori, surface 4-livelli, font Geist, raggi di curvatura.
- `_zindex.scss`: Mappa `$z-layers` e funzione `z($layer)` per un layer management matematicamente affidabile.
- `_mixins.scss`: Mixin per specular highlights, ambient glow, micro-scrollbar, tactile press.
- `_reset.scss`: CSS Preflight Reset totale che azzera i default del browser per `button`, `input`, `textarea`, `select`, eliminando alla radice il problema dei bottoni con background bianco o grigio nativo del browser.
- `_animations.scss`: Keyframes per toast in/out, modal scale-in, dropdown unroll, drawer slides, indeterminate slide, spin, pulse glow e classi `.animate-*`.
- `_utilities.scss`: Motore di utilità SCSS universale (Flexbox, Grid, Sizing, Spacing, Typography, Colors, Borders, Shadows).
- `_theme_presets.scss`: Presets e classi cromatiche per i 5 temi (Violet, Obsidian, Crimson, Cyan, Emerald), anelli di selezione (`ring-2`), scale attive e glow matrix dinamica.
- `components/_*.scss`: Moduli specifici per ciascun componente UI, inclusi `.sd-btn`, `.sd-card`, `.sd-input`, `.sd-checkbox`, `.sd-modal`, `.sd-drawer`, `.sd-dropdown`, `.sd-toast`, `.sd-alert`, `.sd-badge`, `.sd-navbar`, `.sd-switch`, `.sd-tabs`, `.sd-tooltip`, `.sd-heading`/`.sd-text`.
- `main.scss`: Entrypoint master all'interno di `crate/scss/` che assembla e compila l'intera suite.

### Supporto Duale al Tema (`Theme` vs Fallback statico)
Tutti i componenti in `crate/src/` sono scritti per funzionare in due modalità grazie a `#[cfg]`:
1. **Con feature `theme` attiva**: Leggono le classi SCSS in modo reattivo invocando `crate::theme::use_theme()`. Quando il tema cambia a runtime, tutti i componenti istanziati si aggiornano immediatamente.
2. **Senza feature `theme` attiva**: Fanno fallback trasparente su classi statiche predefinite hardcoded nei componenti (zero overhead se il sistema a tema non è richiesto).

### Sistema del Tema in stile Mantine (`crate/src/theme.rs`)
- **`Theme`**: Struttura dati che raggruppa tutti i token CSS di layout, superfici a 4 livelli, accenti e classi di ogni componente.
- **Preset inclusi**:
  - `Theme::shadow()` (Default): Ultraviolet Shadow (Nero profondo + accenti viola e specular highlights).
  - `Theme::obsidian()`: Obsidian Titanium (Monocromatico rigoroso argento e titanio).
  - `Theme::crimson()`: Crimson Phoenix (Nero profondo con fiamme cremisi fenice).
  - `Theme::cyan()`: Cyber Cyan (Ciano elettrico ad alto contrasto).
  - `Theme::emerald()`: Emerald Matrix (Verde smeraldo ad alta nitidezza).
- **Scale di Personalizzazione**:
  - `PrimaryColor`: Violet, Obsidian, Crimson, Cyan, Emerald.
  - `RadiusScale`: Sharp (2px), Medium (8px), Soft (14px).
  - `GlowMode`: Subtle, Expressive, Off.
- **Context API Dioxus**: `ThemeProvider` per fornire il tema all'albero di componenti e `use_theme()` per consumarlo o scrivervi (`theme.write()`).

---

## 4. Crate Showcase (`showcase`)

È l'applicazione web interattiva usata per collaudare e mostrare visivamente tutti i componenti.

- **Routing (Dioxus 0.7)**: Gestito tramite l'enum `Route` in `showcase/src/main.rs`.
- **Layout Shell (`showcase/src/components/layout.rs`)**:
  - Fornisce la barra superiore con logo Shadowfenix e toolbar interattiva Mantine (palette swatches, radius selector, glow toggle).
  - Barra laterale con navigazione alle pagine dei singoli componenti e icone SVG native.
  - Renderizza la pagina corrente tramite `Outlet::<Route> {}`.
- **ComponentShowcase a 3 Schede (`ShowcaseTabs`)**:
  - **Playground**: Controlli interattivi live (varianti, dimensioni, stati, switch).
  - **Codice RSX**: Snippet di codice copiabile negli appunti con feedback "Copiato!".
  - **API Props**: Tabella dettagliata dei parametri (Nome, Tipo Rust, Default, Descrizione).

---

## 5. Regole e Convenzioni per lo Sviluppo

### Dioxus 0.7 API Standards
- **No API obsolete**: `cx`, `Scope`, `use_state` non esistono più in Dioxus 0.7.
- **Stato reattivo**:
  - Usa `use_signal(|| init_val)`.
  - Lettura: `sig()` oppure `*sig.read()`.
  - Scrittura: `*sig.write() = new_val` oppure `sig.with_mut(|v| ...)`.
- **Proprietà dei Componenti**:
  - Devono implementare `Clone` e `PartialEq`.
  - Usa tipi posseduti (`String`, `Option<String>`, `Signal<T>`), non riferimenti (`&str`, `&[T]`).
  - Per eventi, usa `Option<EventHandler<MouseEvent>>` o simili.
- **Sintassi RSX**:
  - Cicli con `for item in items { ... }`.
  - Condizionali diretti con `if condition { ... }`.
  - Attributi condizionali e stringhe formattate con `{base} {extra}`.

### Regole di Concorrenza e Linting (`clippy.toml`)
Il repository include configurazioni specifiche per evitare deadlock e panic nei thread asincroni Dioxus:
- **NON trattenere mai riferimenti o lock attraverso punti di `.await`**:
  - `generational_box::GenerationalRef`
  - `generational_box::GenerationalRefMut`
  - `dioxus_signals::WriteLock`
- Se devi eseguire un `.await`, rilascia sempre prima il lock dello Signal o della box generazionale (ad esempio clonando il valore prima dell'await o chiudendo il blocco di scope del lock).

### Linee Guida per Aggiungere Nuovi Componenti
Quando si aggiunge un nuovo componente a `shadow-design`:
1. Crea il file `crate/src/<componente>.rs`.
2. Aggiungi la feature in `crate/Cargo.toml` (sia come feature isolata, sia come dipendenza di `all`).
3. In `crate/src/lib.rs`, esponi il modulo con `#[cfg(feature = "...")]` e includilo in `pub mod prelude`.
4. Nel componente, gestisci sia il supporto reattivo a `crate::theme::use_theme()` sia il fallback statico senza feature `theme`.
5. Se il componente richiede stili nel tema, aggiorna `Theme` in `crate/src/theme.rs` con i campi corrispondenti e i metodi builder.
6. Aggiungi la pagina dimostrativa corrispondente in `showcase/src/pages/` e registrala nel Router e nella Sidebar.

---

## 6. Comandi Utili per il Workflow

Esegui sempre i comandi dalla root del workspace o specificando il package target:

```bash
# Verifica rapida della compilazione dell'intero workspace
cargo check --workspace

# Analisi statica con linter Clippy (rispetta clippy.toml)
cargo clippy --workspace

# Esecuzione dei test
cargo test --workspace

# Avvio del server di sviluppo dello Showcase (con hot-reload)
cd showcase
dx serve

# Verifica che tutte le classi RSX utilizzate esistano nel CSS compilato
python scripts/check_classes.py

```
