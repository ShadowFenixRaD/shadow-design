# Changelog — Shadow Design

Tutte le modifiche rilevanti a questo progetto sono documentate in questo file.
Il formato è basato su [Keep a Changelog](https://keepachangelog.com/it/1.0.0/) e aderisce al [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

---

## [0.3.0] — 2026-10-14

Release maggiore che introduce 11 nuovi componenti UI e feature modulari, una suite completa di selezione (`select`), campi di testo multilinea con autosize (`textarea`), miglioramenti architetturali alla navigazione dello Showcase con sidebar comprimibile inline a sinistra, armonizzazione cromatica dei bagliori ottici (`glow`) per variante di bottone, e una vasta serie di rifiniture all'accessibilità, alla tipografia e al layout.

### 🚀 Nuove Funzionalità & Componenti UI

- **Suite di Selezione Avanzata (`select`)**:
  - `Select`: Menu a discesa a selezione singola con supporto a descrizioni e raggruppamenti.
  - `MultiSelect`: Selezione multipla con badge/tag rimovibili ed eliminazione rapida.
  - `Combobox`: Campo di testo combinato con ricerca live, filtri reattivi e navigazione tastiera.
  - `SelectItem`: Struttura dati builder con `.with_description()`, `.with_group()` e `.disabled()`.
- **Campi di Testo Multilinea (`textarea`)**:
  - `Textarea`: Input multilinea con dimensionamento automatico in base alle righe digitate, contatore di caratteri reattivo e stati di validazione.
- **Componente Avatar (`avatar`)**:
  - Avatar per profili utente e feed P2P con 5 dimensioni (`Xs` - `Xl`), forme rotonde o squadrate, indicatore di presenza/stato e fallback automatico con iniziali.
- **Componente Chat & MessageBubble (`chat`)**:
  - Bolle di messaggio asimmetriche (`Incoming` / `Outgoing`) con badge crittografico (es. PQ), timestamp, tracking dello stato di consegna (`DeliveryStatus`) e design card arrotondato a 18px privo di artefatti perimetrali.
- **Barre di Progresso (`progress`)**:
  - Indicatori di avanzamento lineari determinati e indeterminati con supporto a percentuali, glow temizzato e varianti di colore.
- **Schede Allegati (`attachment_card`)**:
  - Card per file e download/upload con barra di progresso integrata, icone di tipo file e pulsanti di azione rapida.
- **Schede Contatto (`contact_card`)**:
  - Card per nodi e profili con avatar integrato, stato di connessione e azioni rapide.
- **Controllo a Segmenti (`segmented_control`)**:
  - Selettore a segmenti a scatto tattile con animazione di transizione, supporto `full_width`, icone e gestione responsive.
- **Divisore Tipografico (`divider`)**:
  - Separatori orizzontali e verticali con supporto per etichette centrali e bordo singolo pulito.
- **Visualizzatore ad Albero (`tree`)**:
  - Struttura gerarchica ad albero con nodi espandibili/collassabili, icone contestuali e gestione selezioni.
- **Segnaposto Scheletrati (`skeleton`)**:
  - Placeholder di caricamento per il layout (testo, cerchi per avatar, blocchi card) con animazione shimmer fluida.
- **Accessibilità & Navigazione Tastiera**:
  - Piena conformità W3C APG per schede orizzontali (`tabs`) con roving tabindex e tasti freccia.
  - Search Palette globale accessibile con scorciatoia rapida `Ctrl+K`.

### 🎨 Design System SCSS & Showcase

- **Navigazione Showcase Inline Comprimibile**:
  - Sostituito l'overlay drawer con una sidebar nativa inline a sinistra, comprimibile fluidamente da 256px (`w-64`) a 64px (`w-16`) in modalità rail iconica con tooltip informativi su hover e adattamento dinamico del viewport.
- **Armonizzazione Ottica Glow Bottoni**:
  - I bagliori ottici dei pulsanti (`--glow`) sono ora strettamente coordinati alla variante cromatica: rosso rubino per `Danger`, verde smeraldo per `Success`, tinta coordinata al preset attivo per `Primary`, e bagliori neutri per varianti secondarie.
- **Showcase Feature Badges**:
  - Visualizzazione dinamica delle 29 feature flag Cargo modulari nella home page dello showcase.

### 🐛 Correzioni & Ottimizzazioni

- **MessageBubble Layout**:
  - Risolto l'artefatto per cui il wrapper esterno a tutta larghezza ereditava bordi e sfondi della bolla; il flex container esterno è ora rigorosamente non stilizzato e l'aspetto card è confinato alla sola bolla.
- **Pulsanti di Caricamento (Loading States)**:
  - Rimossa la stringa di testo "Loading...", mantenendo esclusivamente lo spinner SVG animato al centro del pulsante.
- **Checkbox Visibilità Native**:
  - Eliminata la visualizzazione concorrente del checkbox blu predefinito del browser tramite la classe `.sd-visually-hidden`.
- **Divider Verticale**:
  - Eliminato il raddoppio delle linee verticali tra contenitore ed elemento linea.
- **SegmentedControl Clipping**:
  - Risolti i problemi di traboccamento e ritaglio delle etichette/icone nelle card in modalità `full_width`.
- **AttachmentCard Spacing**:
  - Aggiunto padding superiore alla barra di avanzamento quando la descrizione inferiore è omessa.
- **Animazioni Showcase**:
  - Risolto il mancato avvio delle animazioni nello showcase garantendo l'esecuzione anche in presenza di impostazioni OS `prefers-reduced-motion` per la classe `.sd-motion-demo`.

---

## [0.2.0] — 2026-10-11

Nuova versione che introduce il motore di temi multi-livello con risoluzione a cascata a 3 livelli, supporto completo a Serde (`serde`) e persistenza `localStorage`, lo studio interattivo **Theme Builder**, correzioni strutturali ai layer CSS Cascade, e isolamento rigoroso delle feature flags.

### 🚀 Nuove Funzionalità

- **Motore di Temi Multi-Livello (Livello 1 / 2 / 3)**:
  - Supporto per tonalità colore arbitrarie con `PrimaryColor::Custom { name, hex, css_prefix }` e derivazione automatica dello slug CSS.
  - Nuovo enum `ElevationStyle` con 4 stili di profondità:
    - `NeonGlow`: bagliore ottico perimetrale al neon.
    - `HardBlock`: ombre solide fumettistiche/manga ad alto impatto per interfacce stile Cartoon.
    - `SoftElevated`: ombreggiature diffuse tradizionali.
    - `Flat`: grafica minimalista priva di ombre.
  - Supporto per `ComponentOverride` a Livello 2: personalizzazione di classi base, varianti, ombre e modificatori per singolo componente.
  - Nuovo `ThemeBuilder` fluido e chainable per comporre temi in modo programmatico:
    `ThemeBuilder::new().primary_color(...).elevation_style(...).override_component(...).build()`.
  - Iniezione dinamica di CSS Custom Properties a runtime tramite `Theme::to_css_variables()`.
- **Serializzazione e Deserializzazione Serde (`serde`)**:
  - Aggiunto `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]` per `Theme`, `PrimaryColor`, `RadiusScale`, `GlowMode`, `ElevationStyle` e `ComponentOverride`.
- **Interactive Theme Builder Studio (`/theme-builder`)**:
  - Pagina applicativa dedicata nello showcase per la creazione, anteprima e collaudo live di temi personalizzati.
  - 4 schede di configurazione: *Palette & Superfici* (con color picker nativo HTML), *Forme & Elevazione*, *Override di Livello 2* e *Import/Export JSON*.
  - Preset a selezione rapida (Ultraviolet, Obsidian, Crimson, Cyan, Emerald e il nuovo *Cartoon Pop*).
  - Persistenza automatica su `localStorage` resistente ai riavvii del browser e propagazione immediata a tutte le pagine dello showcase.
  - Generatore istantaneo di codice Rust `ThemeBuilder` copiabile negli appunti.

### 🐛 Correzioni & Miglioramenti

- **Riorganizzazione Livelli CSS Cascade Layers**:
  - Formalizzato l'ordine gerarchico dei livelli: `@layer sd-reset, sd-base, sd-components, sd-utilities, sd-theme;`.
  - Racchiuso `_reset.scss` in `@layer sd-reset`: risolto il problema critico per cui gli stili unlayered del reset azzeravano il padding e i bordi di `.sd-btn`.
- **Spaziature Bottoni e Icone**:
  - Ripristinato e calibrato il padding orizzontale su tutte le dimensioni (Xs-Xl) con `box-sizing: border-box`.
  - Spaziatura tra icone e testo delegata interamente a `gap: 6px/8px`, eliminando sovrapposizioni e margini non sincronizzati.
- **Bordi e Finiture delle Card**:
  - Bordo netto e nitido su tutti e 4 i lati (`border: 1px solid rgba(255, 255, 255, 0.18)`).
  - Concatenazione corretta di top specular hairline (`inset 0 1px 0 rgba(255, 255, 255, 0.12)`) con le ombre esterne (`0 4px 24px rgba(0, 0, 0, 0.55)`).
  - Linee divisorie nitide da 1px per header e footer (`rgba(255, 255, 255, 0.14)`).
  - Corretti i valori effettivi di `RadiusScale` in `_theme_presets.scss` (Sharp: `2px`, Medium: `8px`, Soft: `14px`).
- **Eliminazione Warning Dioxus**:
  - Rimosso `document::Style { "{theme().to_css_variables()}" }` reattivo (che causava `Changing props of Style {} is not supported`).
  - Sostituito con iniezione asincrona reattiva via `document::eval` in `<style id="sd-dynamic-theme-vars">`.
- **Isolamento Feature Flags Cargo**:
  - Dichiarata correttamente la dipendenza interna `icon` per i componenti che espongono o usano icone nei loro tipi o markup (`input`, `checkbox`, `modal`, `alert`, `notification`, `tabs`, `drawer`, `dropdown`).
  - Verificato con successo che ogni singola feature compila isolatamente senza errori.

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
