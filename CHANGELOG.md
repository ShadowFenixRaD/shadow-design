# Changelog — Shadow Design

Tutte le modifiche rilevanti a questo progetto sono documentate in questo file.
Il formato è basato su [Keep a Changelog](https://keepachangelog.com/it/1.0.0/) e aderisce al [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### ⚠️ Breaking — fondamenta Lit Glass (ADR-0002, ADR-0003, ADR-0004)
- Rimossi `Theme::cartoon()`, `ElevationStyle::HardBlock` e `ElevationStyle::SoftElevated` (anche come varianti serde). `ElevationStyle::Flat` ora azzera il glow.
- Il tema scrive solo variabili CSS: `ThemeProvider` applica `sd-theme sd-radius-* sd-glow-*` e le variabili accento/superfici inline; niente più `<style>` globale `:root`, `sd-theme-{prefix}`, `data-sd-theme` né classi per-preset (`*-crimson`, `*-cyan`, `*--primary-custom`, …).
- Accento derivato da un solo hex (`--sd-accent-strong/-deep/-soft/-border/-fg`, `--sd-focus-ring`); Obsidian è `#f4f4f5`, Cyan `#00e5ff`. `PrimaryColor::custom_with_prefix` funziona con qualsiasi prefisso.
- `RadiusScale::sm()/md()/lg()` restituiscono `""` (nuovo `RadiusScale::class()`); i campi `border*`/`glow` di `Theme` sono vuoti, `text_accent*` è `sd-text-accent`, i campi `glow_*` sono classi generiche.
- `resolve_elevation_class(c, Some(true))` restituisce sempre la classe `--glow` del componente; con `None` restituisce `""` (il glow di default vive nelle ricette SCSS). Il glow Subtle vale ora 0.6.
- Nuovi `_tokens.scss` (token `--sd-*`), mixin Lit Glass (`sd-shadow`, `sd-surface`, `sd-lit-fill`, `sd-field`, `sd-item`, `sd-selected`, …) e keyframe `sd-*`; rimosso `_preset_cartoon.scss`. z-index: aggiunti `command-palette` 88, `hover-card` 990, `keytip` 1005; `tooltip` passa a 1010.

---

## [0.5.0] — 2026-10-24

Massiva release architetturale che porta Shadow Design a uno standard enterprise per Dioxus 0.7, introducendo **8 nuovi Epic completi** con oltre 35 componenti aggiuntivi: layout primitives responsive, overlay fluttuanti avanzati, micro-interazioni tattili, engine reattivo di hooks e form validation, estensioni complesse di tooling, suite completa di data visualization SVG nativa, rich content e navigazione collaborativa di livello avanzato.

### 🚀 Nuove Funzionalità & Componenti UI

#### 📐 Layout Primitives & Contenitori (Epic #9)
- **Grid & SimpleGrid (`grid`)**: Griglia responsive a 12 colonne con breakpoint mobili/desktop, gap tokenizzato e subcomponente `GridCol`. `SimpleGrid` per griglie auto-adattive a colonne uguali.
- **Group (`group`)**: Primitiva layout Flexbox orizzontale con wrapping configurabile, allineamento, giustificazione ed espansione controllata (`grow`).
- **Stack (`stack`)**: Primitiva Flexbox verticale per impilare componenti con spaziatura tokenizzata e allineamento semantico.
- **AspectRatio (`aspect_ratio`)**: Contenitore a rapporto di forma bloccato per prevenire Cumulative Layout Shift (CLS) con preset standard (16:9, 4:3, 1:1, 21:9, 9:16).
- **Center (`center`)**: Utility container per il centraggio bidirezionale assoluto in modalità block o inline.

#### 🪟 Floating Overlays & Menu Contestuali (Epic #7)
- **Popover (`popover`)**: Pannello fluttuante ancorato al trigger con 12 allineamenti geometrici, freccia integrata, dismiss su click esterno ed Escape (`Popover`, `PopoverTrigger`, `PopoverDropdown`).
- **HoverCard (`hover_card`)**: Card di anteprima ricca su hover con delay di apertura/chiusura configurabili e persistenza su interazione del contenuto.
- **ContextMenu (`context_menu`)**: Menu contestuale azionato da click destro del mouse alle coordinate precise del puntatore, con scorciatoie da tastiera e checkbox/radio items.
- **AlertDialog & ConfirmDialog (`alert_dialog`)**: Dialoghi bloccanti con focus trapping e backdrop per azioni distruttive irreversibili o conferme critiche.
- **Coachmark & TeachingBubble (`coachmark`)**: Guide passo-passo ancorate a specifici elementi DOM per onboarding utente con indicatori di avanzamento e navigazione interattiva.
- **SpeedDial & FAB (`speed_dial`)**: Floating Action Button ad espansione radiale/lineare a 4 direzioni per raggruppare azioni veloci secondarie.
- **Affix & BackToTop (`affix`)**: Contenitori fluttuanti in overlay agganciati alle coordinate dello schermo con attivazione a soglia di scroll e pulsante fluido `BackToTop`.

#### ✨ Micro-Interazioni Tattili (Epic #13)
- **Burger (`burger`)**: Icona animata a tre linee con transizione fluida verso lo stato Close/X per menu mobili.
- **CloseButton (`close_button`)**: Pulsante universale di chiusura accessibile con varianti cromatiche e feedback tattile.
- **CopyButton (`copy_button`)**: Pulsante di copia rapida negli appunti con feedback visivo temporizzato integrato.
- **Indicator (`indicator`)**: Badge di notifica sovrapposto agli elementi con posizionamento a 9 quadranti, processing glow pulse e dot compatto.
- **Highlight (`highlight`)**: Evidenziatore di testo reattivo per query di ricerca con styling tematico.
- **LoadingOverlay (`loading_overlay`)**: Velatura di caricamento con spinner e blur backdrop per sezioni asincrone.
- **Fieldset (`fieldset`)**: Raggruppamento form semantico con bordo coordinato e legenda personalizzabile.
- **MarqueeSelection (`marquee_selection`)**: Rettangolo di selezione elastico su coordinate del cursore con pointer capture e collision detection.
- **Keytip (`keytip`)**: Suggerimenti visivi delle scorciatoie da tastiera fluttuanti per navigazione rapida senza mouse.

#### ⚡ Reactive Utilities & Form Engine (Epic #14)
- **Form & UseForm (`form`)**: Engine completo di form validation con validatori composabili (Required, Email, MinLength, MaxLength, Pattern, Custom), stato reattivo e gestione dinamica degli errori.
- **Reactive Hooks (`hooks`)**:
  - `use_clipboard`: Gestione asincrona degli appunti con auto-reset temporizzato affidabile.
  - `use_debounced_signal`: Debounce atomico thread-safe ad alte prestazioni per input ad alta frequenza.
  - `use_hotkey`: Ascolto scorciatoie da tastiera globali o locali con modifier keys.
  - `use_idle`: Rilevamento inattività utente con timeout configurabile.
  - `use_intersection_observer`: Lazy loading e trigger di visibilità viewport.
  - `use_local_storage`: Persistenza reattiva sincronizzata nello storage del browser.
  - `use_media_query`: Ascolto dinamico delle media query CSS responsive.
  - `use_window_scroll`: Coordinate di scroll window reattive e controllo programmatico fluido.
  - `use_click_outside`: Rilevamento click al di fuori dei confini di un elemento.

#### 🛠️ Estensioni Complesse & Tooling (Epic #8)
- **CommandPalette (`command_palette`)**: Palette di comando in overlay (Ctrl+K) con ricerca fuzzy, raggruppamento comandi, icone e navigazione globale da tastiera.
- **DatePicker & CalendarView (`date_picker`)**: Calendario completo con navigazione mese/anno, supporto anni bisestili, range selection e localizzazione.
- **Dropzone (`dropzone`)**: Area di upload file drag-and-drop con anteprime, validazione mime-type e indicatore di caricamento.
- **Resizable (`resizable`)**: Splitter di pannelli ridimensionabili orizzontali e verticali con pointer drag engine e proporzioni flessibili.
- **Carousel (`carousel`)**: Slider multimediale a scorrimento con transizioni animate, controlli di navigazione e indicatori dot.
- **VirtualList (`virtual_list`)**: Virtualizzazione di liste massive ad alto rendimento con rendering esclusivo degli elementi visibili nel viewport.
- **CodeHighlight (`code_highlight`)**: Visualizzatore di codice sorgente con tokenizer sintattico multi-linguaggio e numerazione righe.

#### 📊 Native SVG Charts Suite (Epic #10)
- **Chart Primitives (`chart`)**: `ChartContainer`, `ChartTooltip` dinamico con hit-testing, `ChartLegend` e griglie responsive `ChartGrid`.
- **AreaChart & LineChart**: Curve lineari e spline con riempimento a gradiente neon e supporto multi-serie.
- **BarChart**: Grafici a barre orizzontali e verticali con barre raggruppate o sovrapposte.
- **PieChart & DonutChart**: Grafici a torta e ciambella con calcoli trigonometrici nativi degli archi SVG.
- **RadarChart**: Grafici radar per benchmarking multidimensionale e analisi competenze.
- **ScatterChart & BubbleChart**: Distribuzione a coordinate cartesiane con scaling dinamico del raggio dei punti.
- **Sparkline**: Micro-grafici compatti per dashboard metriche e card di telemetria.
- **GaugeChart & RadialBar**: Indicatori a tachimetro e barre radiali per avanzamento percentuale e soglie.
- **Heatmap & CalendarHeatmap**: Mappe di calore matriciali e griglia di attivita annuale stile GitHub.

#### 📰 Rich Content & Media (Epic #11)
- **RichTextEditor (`rich_text`)**: Editor di testo ricco WYSIWYG con conversione bidirezionale Markdown/HTML, barra degli strumenti contestuale e scorciatoie di formattazione.
- **DocumentCard (`document_card`)**: Card interattiva per allegati e documenti con anteprime, badge di formato file e azioni rapide.
- **MasonryGrid (`masonry`)**: Layout a mosaico asimmetrico dinamico a colonne indipendenti con Lightbox modale integrato per immagini.
- **MediaCard (`media_card`)**: Scheda multimediale ad aspect-ratio fisso con overlay di testo e hover lift tattile.

#### 🧭 Navigazione Avanzata & Tooling Collaborativo (Epic #12)
- **NavigationMenu (`navigation_menu`)**: Menu di navigazione orizzontale di livello enterprise con mega-menu a comparsa multi-colonna e card in anteprima.
- **OverflowSet & ResponsiveToolbar (`overflow_set`)**: Toolbar intelligente che rileva la larghezza del contenitore e sposta automaticamente le azioni eccedenti in un dropdown "Altro".
- **TransferList (`transfer_list`)**: Selettore a doppia colonna per lo spostamento di elementi tra liste (Sorgente/Destinazione) con ricerca e azioni bulk.
- **Facepile (`facepile`)**: Cluster di avatar sovrapposti con indicatori di stato/presenza e contatore overflow (+N).
- **ActivityFeed (`activity_feed`)**: Timeline collaborativa ricca per eventi, audit log, commenti e risposte inline.

### 🎨 Design System SCSS & Polish
- Completamento dell integrazione SCSS con moduli BEM dedicati per ogni nuovo componente in `crate/scss/components/`.
- Refactoring globale dei selettori e layering Z-index standardizzato in `_zindex.scss`.
- Nuove icone SVG native in `IconKind` (Plus, Search, Calendar, Upload, Chart, ecc.).
- Centralizzazione della costante `VERSION` reattiva tramite macro `env!("CARGO_PKG_VERSION")`.

### 🐛 Correzioni & Ottimizzazioni
- **Resizable**: Risolto il panic di corruzione degli hook slot Dioxus rimuovendo chiamate annidate a `use_unique_id` all interno di `use_signal`.
- **CommandPalette**: Resa globale la navigazione da tastiera tramite event listener nativo su `window`.
- **Hooks**: Eliminati cicli infiniti di re-render montando i listener DOM una sola volta con `use_hook` e timer browser.
- **Audit Statico & WAI-ARIA**: Allineati attributi di accessibilità ARIA (`role`, `aria-expanded`, `aria-selected`, `aria-invalid`) su tutti i controlli form e overlay.


---

## [0.4.0] — 2026-10-20

Release monumentale che estende radicalmente la libreria introducendo **20 nuovi componenti UI e feature modulari** ad alte prestazioni per Dioxus 0.7, una suite avanzata per form, controlli continui, visualizzazione dati, layout master a 5 zone, navigazione ricca e una profonda riorganizzazione semantica dello Showcase.

### 🚀 Nuove Funzionalità & Componenti UI

- **AppShell (`app_shell`)**:
  - Layout master responsive a 5 zone (`Header`, `Navbar`, `Aside`, `Main`, `Footer`) con supporto al collasso fluido, layering z-index rigoroso e specular highlight perimetrale.
- **ColorPicker & ColorInput (`color_picker`)**:
  - Selettore cromatico avanzato con gradiente 2D saturazione/luminosità, slider Hue e Alpha, parser/formatter Hex a 3/6/8 cifre, controlli RGB e palette di swatches predefinite.
- **PinInput & OTP (`pin_input`)**:
  - Caselle di inserimento sequenziale tattile per codici PIN, OTP e 2FA con auto-avanzamento su digitazione, supporto al mascheramento (password), gestione cancellazione e filtri numerico/alfanumerico.
- **NumberInput (`number_input`)**:
  - Input numerico con stepper tattile a bottoni su/giù, precisione decimale personalizzabile, step configurabile e rispetto dei limiti `min`/`max`.
- **Radio & RadioGroup (`radio`)**:
  - Selezione esclusiva con orientamento orizzontale o verticale, propagazione reattiva dello stato `disabled` tramite contesto, varianti cromatiche e supporto a `RadioCard`.
- **Slider & RangeSlider (`slider`)**:
  - Cursore graduato a trascinamento continuo con supporto per step/tacche (`marks`), indicatore percentuale, snap ai valori e tooltip fluttuante integrato su hover/trascinamento.
- **Rating (`rating`)**:
  - Componente di valutazione a stelle o icone con supporto per punteggi frazionari a mezze stelle, anteprima visiva su hover, selezione cliccabile e modalità sola lettura.
- **Chip & ChipGroup (`chip`)**:
  - Capsule interattive per filtri dinamici, tag e badge a selezione singola o multipla (`multiple`), con checkbox visivo integrato e stati tattili.
- **Accordion & Collapsible (`accordion`)**:
  - Pannelli espandibili a fisarmonica conformi alle specifiche WAI-ARIA, con supporto per espansione singola o multipla e reattività live su contesto condiviso.
- **Spoiler (`spoiler`)**:
  - Contenitore a comparsa progressiva con dissolvenza inferiore e pulsante tattile "Mostra di più / Mostra meno" per testi o elementi estesi.
- **Table & DataTable (`table`)**:
  - Tabelle dati semantiche con intestazioni personalizzate, hover di riga, paginazione nativa e comparatori di ordinamento colonne lato client (`ascending`/`descending`).
- **Timeline (`timeline`)**:
  - Cronologia e linea guida temporale per eventi, audit log e tracciamento stati con bullet cromatici e linee di collegamento solide o tratteggiate.
- **List & ListItem (`list`)**:
  - Elenchi ordinati (`Ordered`), non ordinati (`Unordered`) o con icone personalizzate (`Icon`), spaziatura variabile ed estetica uniforme.
- **ScrollArea (`scroll_area`)**:
  - Contenitore a scorrimento bidirezionale con micro-scrollbar custom Shadow a scomparsa automatica (`auto-hide`), fade-edges a gradiente e contenimento dell'overscroll.
- **Breadcrumbs (`breadcrumbs`)**:
  - Percorsi di navigazione a briciole di pane con separatori grafici configurabili, ellissi collassabile automatica (`max_items`) e reset di stile su liste semantiche.
- **Pagination (`pagination`)**:
  - Barra di controllo paginazione completa con calcolo automatico dei salti ellittici, frecce di navigazione e gestione dinamica della pagina corrente.
- **NavLink & SidebarMenu (`nav_link`)**:
  - Elementi di navigazione laterale con indicatore neon attivo, supporto ad azioni di coda (`trailing actions`), badge informativi e sottomenu gerarchici collassabili.
- **BottomNavigation (`bottom_navigation`)**:
  - Barra di navigazione mobile fissa inferiore con gestione della safe area per smartphone, micro-glow e pillola attiva centrata.
- **Stepper (`stepper`)**:
  - Guida procedurale guidata a passi (wizard) con orientamento orizzontale e verticale, nodi numerati/iconici, linee connettive e schermata finale di completamento (`StepperCompleted`).
- **Menubar & CommandBar (`menubar`)**:
  - Barra dei menu orizzontale in stile applicazione desktop con tendine fluttuanti ancorate, visualizzazione shortcut di tastiera e separatori.

### 🎨 Riorganizzazione Showcase & Search Palette

- **Allineamento Totale (47 Componenti)**: Sincronizzazione 1:1 tra router dell'applicazione, barra laterale (`SidebarView`) e tavolozza di ricerca rapida `Ctrl+K` (`SearchPalette`).
- **7 Macro-Categorie Semantiche**:
  1. *Generale*: Panoramica, Theme Builder, Typography, Icons, Animation.
  2. *Layout & Struttura*: AppShell, Navbar, Card, ScrollArea, Divider, Accordion, Spoiler.
  3. *Navigazione*: NavLink, Tabs, Breadcrumbs, Pagination, Stepper, BottomNavigation, Menubar.
  4. *Inserimento & Form*: Button, TextInput, NumberInput, PinInput / OTP, Textarea, Select & Combobox, Checkbox, Radio & RadioGroup, Switch, Slider & RangeSlider, Rating, Chip & ChipGroup, ColorPicker & ColorInput, SegmentedControl.
  5. *Dati & Feedback*: Table, List, Timeline, Badge, Alert, Notifications, Progress, Skeleton.
  6. *Overlay & Finestre*: Modal, Drawer, Dropdown, Tooltip.
  7. *Chat & P2P Mesh*: Avatar, Chat Bubble, AttachmentCard, TreeView (Mesh).

### 🐛 Correzioni & Rifiniture Architetturali

- **Breadcrumbs**: Risolto il calcolo dell'ellissi e forzato il layout flex orizzontale azzerando il default `list-style` del browser.
- **Pagination**: Allineamento orizzontale nativo dei pulsanti numerici e di salto.
- **ScrollArea**: Aggiunto contenimento `overscroll-behavior`, contrasto aumentato per la scrollbar thumb e reset dello stato di scorrimento al cambio demo.
- **Radio & Accordion**: Risolta la propagazione reattiva dello stato `disabled` e dell'espansione dei pannelli tramite segnali contestuali.
- **Slider**: Refactoring per riutilizzare internamente il componente nativo `Tooltip`.
- **Packaging Crate**: Inclusione di `build.rs` all'interno della distribuzione per garantire la corretta notifica di rigenerazione SCSS (`cargo:rerun-if-changed=scss`).

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
