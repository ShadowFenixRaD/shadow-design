# Changelog — Shadow Design

Tutte le modifiche rilevanti a questo progetto sono documentate in questo file.
Il formato è basato su [Keep a Changelog](https://keepachangelog.com/it/1.0.0/) e aderisce al [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] — 2026-10-09

**Data di Pubblicazione su crates.io**: Venerdì 9 Ottobre 2026, ore 18:05:00 CEST (16:05:00 UTC)  
**Commit di Pubblicazione**: [`dc9678f689f08ca63ebb0d27128681edb7c49869`](https://source.shadowfenix.net/development/shadow-design/commit/dc9678f689f08ca63ebb0d27128681edb7c49869)  
**Commit di Preparazione Metadata**: [`4b5a2993945afccbc2ffcda36b2c332c204cf86e`](https://source.shadowfenix.net/development/shadow-design/commit/4b5a2993945afccbc2ffcda36b2c332c204cf86e)  

Prima release pubblica ufficiale di **Shadow Design** (`shadow-design`) su [crates.io](https://crates.io/crates/shadow-design).

### 🚀 Nuove Funzionalità & Componenti UI (Dioxus 0.7)

- **`Button` (`button`)**:
  - Varianti BEM: `Primary`, `Secondary`, `Outline`, `Ghost`, `Danger`, `Success`.
  - 5 scale dimensionali da `Xs` a `Xl`, supporto per loading spinner SVG nativo, icone sinistra/destra e feedback tattile al click (`tactile-press`).
- **`Card` (`card`)**:
  - Schede contenitore con varianti `Default`, `Elevated`, `Interactive` e `Metric`.
  - 5 livelli di profondità perimetrale (`CardShadow`: `None`, `Sm`, `Md`, `Lg`, `Xl`) e supporto micro-glow.
  - Header integrato con titolo/sottotitolo e slot per footer dedicato.
- **`TextInput` (`input`)**:
  - Binding reattivo bidirezionale con `Signal<String>`.
  - Supporto per icone statiche e cliccabili sia a sinistra che a destra (`on_left_icon_click`, `on_right_icon_click`), stato di errore e pulsante di cancellazione rapida (*clear button*).
- **`Checkbox` (`checkbox`)**:
  - Caselle di spunta tattili a 3 dimensioni (`Sm`, `Md`, `Lg`) con supporto per stato `indeterminate` e stato di errore.
- **`Badge` (`badge`)**:
  - Etichette di stato a contrasto, design rettangolare o a pillola (`pill`), con indicatore a punto pulsante animato (`dot`).
- **`Modal` (`modal`)**:
  - Finestre di dialogo in overlay con animazione di scala all'apertura, backdrop sfocato click-to-close e gestione controllata via `Signal<bool>`.
- **`Navbar` (`navbar`)**:
  - Barra di navigazione responsive con specular highlight superiore da 1px, slot per brand ed azioni.
- **`Alert` (`alert`)**:
  - Banner di notifica semantici (`Info`, `Success`, `Warning`, `Danger`) con icone SVG native incorporate e dismiss manuale.
- **`Notification` (`notification`)**:
  - Sistema Toast globale basato sul Context Dioxus con `NotificationProvider`, hook `use_notification` e dismiss temporizzato/manuale.
- **`Tabs` (`tabs`)**:
  - Navigazione orizzontale a schede in stile Mantine (`TabList`, `Tab`, `TabPanel`).
- **`Switch` (`switch`)**:
  - Interruttori toggle tattili ad alta precisione con etichette, descrizioni e animazione elastica del cursore.
- **`Tooltip` (`tooltip`)**:
  - Micro-etichette contestuali fluttuanti su hover con posizionamento a 4 direzioni (`Top`, `Bottom`, `Left`, `Right`).
- **`Drawer` (`drawer`)**:
  - Pannelli scorrevoli laterali a comparsa da sinistra o destra con backdrop oscurato e gesture di chiusura.
- **`Dropdown` (`dropdown`)**:
  - Menu a tendina contestuali ancorati al trigger con gestione dello stack z-index (`z-[999]` / `z-[1000]`) per prevenire problemi di clipping.
- **`Icon` (`icon`)**:
  - Suite completa di icone SVG native vettoriali ad alta nitidezza (tratto 1.5px), completamente a zero dipendenze esterne.
- **`Typography` (`typography`)**:
  - Componenti tipografici basati sulla famiglia di font Geist: `Heading` (H1-H6), `Text`, tasti da tastiera `Kbd` e blocchi inline `Code`.
- **`Animation` (`animation`)**:
  - Framework ottico di transizioni e micro-animazioni CSS (Fade, Slide, Scale, Pulse Glow).

### 🎨 Design System SCSS Nativo & Asset Pipeline

- **Zero-Tooling Style Integration**:
  - Rimosso qualsiasi legame con TailwindCSS o Node.js; l'intera suite di fogli di stile risiede in puro SCSS all'interno della crate in `crate/scss/`.
  - Esposizione diretta tramite la macro asset di Dioxus 0.7: `pub const STYLES: Asset = asset!("/scss/main.scss");`.
  - Integrazione istantanea nelle applicazioni a valle tramite `document::Link { rel: "stylesheet", href: STYLES }`.
- **Dark Craft Depth a 4 Livelli**:
  - Sistema di superfici graduate da `surface_0` (`#030305`) a `surface_3` (`#181824`).
- **Engine Reattivo del Tema (`theme`)**:
  - Supporto per 5 preset cromatici integrati: *Ultraviolet Shadow*, *Obsidian Titanium*, *Crimson Phoenix*, *Cyber Cyan*, *Emerald Matrix*.
  - Scale di personalizzazione runtime per il raggio di curvatura (`RadiusScale`) e intensità del glow (`GlowMode`).
  - Architettura a zero overhead: supporto per fallback statico trasparente quando la feature `theme` non è abilitata.

### 📦 Architettura del Crate & Metadati

- **Modularità Cargo Feature Flags**:
  - Ogni componente è racchiuso dietro una feature flag Cargo indipendente; abilitazione complessiva tramite `features = ["all"]`.
- **Doppia Licenza Open Source**:
  - `MIT OR Apache-2.0`.
- **Esclusione Crate Applicativi**:
  - Configurata la root del workspace Cargo ed escluso lo showcase dal publishing su crates.io (`publish = false`).
