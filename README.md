# Shadow Design

Suite di componenti UI modulari per applicazioni web Dioxus 0.7, progettata per l'organizzazione **shadowfenix** con tema predefinito a sfondo nero assoluto (`0 0 0`) e accenti in viola scuro intenso.

## Struttura del Workspace

Il repository è configurato come workspace Cargo:

```text
shadow-design/
├── Cargo.toml            # Workspace root
├── crate/                # Libreria UI condivisa (crate: shadow-design)
│   ├── Cargo.toml        # Feature flags per ogni singolo componente
│   ├── scss/             # Stili Pure SCSS esportati come Asset
│   └── src/              # Componenti Rust per Dioxus 0.7
└── showcase/             # Espositore Dioxus interattivo
    ├── Cargo.toml
    ├── Dioxus.toml
    ├── assets/
    └── src/
```

---

## 📦 Libreria UI (`crate/`)

La libreria esporta componenti isolati e controllati da singole feature Cargo.

### Includere la libreria nei tuoi siti web

Aggiungi `shadow-design` nel `Cargo.toml` del tuo sito, specificando `default-features = false` per includere solo i componenti necessari (zero bloat):

```toml
[dependencies]
# Importa solo ciò che ti serve:
shadow-design = { version = "0.4.0", default-features = false, features = ["button", "card", "theme", "icon"] }

# Oppure importa l'intera suite:
# shadow-design = { version = "0.4.0", features = ["all"] }
```

### 🎨 Integrazione Asset SCSS (`STYLES`)

Includi direttamente il master stylesheet nativo fornito dalla crate senza bisogno di npm o Tailwind CLI:

```rust
use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLES }
        // Contenuto UI
    }
}
```

### 🎭 Sistema Temi Mantine-Style & ThemeBuilder

Shadow Design supporta la personalizzazione totale dell'interfaccia a runtime con 5 preset cromatici integrati, `ElevationStyle` multilivello e preset `Theme::cartoon()` neo-brutalista:

```rust
use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
fn App() -> Element {
    let custom_theme = ThemeBuilder::new()
        .primary_color(PrimaryColor::custom("Amber Glow", "#f59e0b"))
        .radius_scale(RadiusScale::Medium)
        .glow_mode(GlowMode::Subtle)
        .elevation_style(ElevationStyle::NeonGlow)
        .override_component("card", |c| c.with_base_class("border-amber-500/30"))
        .build();

    rsx! {
        document::Link { rel: "stylesheet", href: STYLES }
        ThemeProvider {
            theme: custom_theme,
            NotificationProvider {
                MainContent {}
            }
        }
    }
}
```

### Feature disponibili

| Feature | Componente | Descrizione |
|---|---|---|
| `button` | `Button`, `ButtonVariant`, `ButtonSize` | Pulsanti stilizzati con varianti cromatiche e dimensioni |
| `card` | `Card`, `CardShadow`, `CardVariant` | Contenitori a schede con vari livelli di ombra, header e footer |
| `input` | `TextInput` | Campi di input reattivi per testi e password con gestione errori e icone |
| `checkbox` | `Checkbox`, `CheckboxSize` | Caselle di selezione tattili con stati indeterminato ed errore |
| `select` | `Select`, `MultiSelect`, `Combobox`, `SelectItem` | Menu a discesa singoli/multipli e combobox con ricerca e filtri |
| `textarea` | `Textarea` | Campi di testo multilinea con autosize reattivo e contatore |
| `badge` | `Badge`, `BadgeVariant` | Etichette, pillole di stato e dot pulsante |
| `navbar` | `Navbar` | Barra di navigazione responsive con brand e link |
| `modal` | `Modal` | Finestre di dialogo con backdrop e controllate da Signal |
| `alert` | `Alert`, `AlertVariant` | Notifiche e callout semantici con icone SVG |
| `notification` | `NotificationProvider`, `use_notification` | Sistema Toast globale |
| `tabs` | `TabList`, `Tab`, `TabPanel` | Navigazione a schede orizzontali conformi ad APG |
| `switch` | `Switch` | Interruttori a scatto tattili ad alta precisione |
| `tooltip` | `Tooltip`, `TooltipPosition` | Micro-etichette fluttuanti su hover a 4 direzioni |
| `drawer` | `Drawer`, `DrawerPosition`, `DrawerSize` | Pannelli laterali a scomparsa da sinistra o destra |
| `dropdown` | `Dropdown`, `DropdownItem`, `DropdownPlacement` | Menu a tendina ancorati al trigger |
| `popover` | `Popover`, `PopoverTrigger`, `PopoverDropdown` | Pannello fluttuante ancorato con 12 allineamenti e freccia |
| `hover_card` | `HoverCard`, `HoverCardTrigger`, `HoverCardDropdown` | Scheda di preview contestuale ricca attivata su hover |
| `context_menu` | `ContextMenu`, `ContextMenuTrigger`, `ContextMenuContent` | Menu a tendina richiamato da click destro del mouse |
| `alert_dialog` | `AlertDialog`, `ConfirmDialog` | Finestra di dialogo bloccante per conferme critiche e distruttive |
| `coachmark` | `Coachmark`, `TeachingBubble` | Bolla di guida onboarding passo-passo |
| `speed_dial` | `SpeedDial`, `SpeedDialAction`, `Fab` | Pulsante circolare FAB primario con espansione radiale/lineare |
| `affix` | `Affix`, `BackToTop`, `AffixPosition` | Contenitore fisso in overlay e trigger scroll BackToTop |
| `icon` | Native SVG Icons (51 `IconKind`) | Icone vettoriali ad alta nitidezza senza dipendenze esterne |
| `typography` | `Heading`, `Text`, `Kbd`, `Code` | Componenti tipografici con scala Geist |
| `animation` | `Transition`, `AnimationVariant` | Transizioni ottiche e micro-animazioni Dioxus |
| `theme` | `Theme`, `ThemeProvider`, `use_theme`, `ThemeBuilder` | Engine reattivo Mantine con 5 preset cromatici, custom HEX e builder |
| `serde` | `Serialize`, `Deserialize` | Serializzazione e deserializzazione JSON di Theme, Preset e Overrides |
| `all` | *Tutti i componenti* | Abilita l'intera suite |

---

## 🖼️ Espositore (`showcase/`)

L'espositore è un'applicazione web Dioxus che mostra una pagina dedicata per ogni componente UI, con anteprima dal vivo, controlli interattivi ed esempi di codice.

### Avviare l'Espositore

Dalla cartella `showcase`:
```bash
cd showcase
dx serve
```

Oppure direttamente dalla root del workspace:
```bash
dx serve -p showcase
```
