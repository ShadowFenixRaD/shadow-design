# Shadow Design

Suite di componenti UI modulari per applicazioni web Dioxus 0.7, progettata per l'organizzazione **shadowfenix** con tema predefinito a sfondo nero assoluto (`0 0 0`) e accenti in viola scuro intenso.

## Struttura del Workspace

Il repository è configurato come workspace Cargo:

```text
shadow-design/
├── Cargo.toml            # Workspace root
├── crate/                # Libreria UI condivisa (crate: shadow-design)
│   ├── Cargo.toml        # Feature flags per ogni singolo componente
│   └── src/
│       ├── lib.rs        # Moduli e prelude
│       ├── button.rs     # Feature "button"
│       ├── card.rs       # Feature "card"
│       ├── input.rs      # Feature "input"
│       ├── badge.rs      # Feature "badge"
│       ├── navbar.rs     # Feature "navbar"
│       ├── modal.rs      # Feature "modal"
│       └── alert.rs      # Feature "alert"
└── showcase/             # Espositore Dioxus per testare i componenti
    ├── Cargo.toml
    ├── Dioxus.toml
    ├── assets/
    └── src/
        └── main.rs       # Router multi-pagina per esplorare ogni componente
```

---

## 📦 Libreria UI (`crate/`)

La libreria esporta componenti isolati e controllati da singole feature Cargo.

### Includere la libreria nei tuoi siti web

Aggiungi `shadow-design` nel `Cargo.toml` del tuo sito, selezionando solo le feature necessarie:

```toml
[dependencies]
shadow-design = { version = "0.1.0", features = ["button", "card", "theme"] }

# Oppure importa l'intera suite:
# shadow-design = { version = "0.1.0", features = ["all"] }
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

### Feature disponibili

| Feature | Componente | Descrizione |
|---|---|---|
| `button` | `Button`, `ButtonVariant`, `ButtonSize` | Pulsanti stilizzati con varianti cromatiche e dimensioni |
| `card` | `Card`, `CardShadow`, `CardVariant` | Contenitori a schede con vari livelli di ombra, header e footer |
| `input` | `TextInput` | Campi di input reattivi per testi e password con gestione errori e icone |
| `checkbox` | `Checkbox`, `CheckboxSize` | Caselle di selezione tattili con stati indeterminato ed errore |
| `badge` | `Badge`, `BadgeVariant` | Etichette, pillole di stato e dot pulsante |
| `navbar` | `Navbar` | Barra di navigazione responsive con brand e link |
| `modal` | `Modal` | Finestre di dialogo con backdrop e controllate da Signal |
| `alert` | `Alert`, `AlertVariant` | Notifiche e callout semantici con icone SVG |
| `notification` | `NotificationProvider`, `use_notification` | Sistema Toast globale |
| `tabs` | `TabList`, `Tab`, `TabPanel` | Navigazione a schede orizzontali |
| `switch` | `Switch` | Interruttori a scatto tattili ad alta precisione |
| `tooltip` | `Tooltip`, `TooltipPosition` | Micro-etichette fluttuanti su hover a 4 direzioni |
| `drawer` | `Drawer`, `DrawerPosition`, `DrawerSize` | Pannelli laterali a scomparsa da sinistra o destra |
| `dropdown` | `Dropdown`, `DropdownItem`, `DropdownPlacement` | Menu a tendina ancorati al trigger |
| `icon` | Native SVG Icons | Icone vettoriali ad alta nitidezza senza dipendenze esterne |
| `typography` | `Heading`, `Text`, `Kbd`, `Code` | Componenti tipografici con scala Geist |
| `animation` | Optical Transitions | Transizioni ottiche e micro-animazioni Dioxus |
| `theme` | `Theme`, `ThemeProvider`, `use_theme` | Gestione centralizzata del tema, 5 preset e personalizzazione dinamica |
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
