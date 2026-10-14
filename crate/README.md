# Shadow Design (`shadow-design`)

Libreria di componenti UI modulare e ad alte prestazioni per **Dioxus 0.7**, costruita su architettura **100% Pure SCSS** nativa, senza dipendenze npm o Tailwind CLI.

---

## 📦 Installazione in Cargo.toml

```toml
[dependencies]
# Importa solo ciò che ti serve con default-features = false (zero bloat):
shadow-design = { version = "0.3.0", default-features = false, features = ["button", "card", "theme", "icon"] }

# Oppure l'intera suite:
# shadow-design = { version = "0.3.0", features = ["all"] }
```

---

## 🎨 Integrazione Diretta Asset SCSS in Dioxus 0.7

`shadow-design` fornisce direttamente il foglio di stile SCSS compilato nativamente dalla pipeline asset di Dioxus. **Non richiede comandi aggiuntivi, nessun watcher e nessun compilatore esterno.**

Nel tuo file `main.rs` (o nel componente radice `App`):

```rust
use dioxus::prelude::*;
use shadow_design::prelude::*;

pub const STYLES: Asset = shadow_design::STYLES;

#[component]
fn App() -> Element {
    rsx! {
        // Inclusione automatica del foglio di stile
        document::Link { rel: "stylesheet", href: STYLES }

        NotificationProvider {
            MyContent {}
        }
    }
}
```

---

## 🧩 Feature Flags Disponibili

| Feature | Componente | Descrizione |
|---|---|---|
| `button` | `Button`, `ButtonVariant`, `ButtonSize` | Pulsanti con varianti BEM, dimensioni, loading spinner |
| `card` | `Card`, `CardShadow`, `CardVariant` | Schede contenitore con varianti, ombre e footer |
| `input` | `TextInput` | Input di testo reattivi con icone cliccabili e clear button |
| `checkbox` | `Checkbox`, `CheckboxSize` | Caselle di spunta tattili con stati indeterminato ed errore |
| `select` | `Select`, `MultiSelect`, `Combobox`, `SelectItem` | Menu di selezione singola, multipla e combobox con filtri |
| `textarea` | `Textarea` | Campi di testo multilinea con autosize reattivo e contatore |
| `badge` | `Badge`, `BadgeVariant` | Etichette, pillole di stato e dot pulsante |
| `navbar` | `Navbar` | Barra di navigazione responsive con specular highlight |
| `modal` | `Modal` | Finestre di dialogo in overlay con backdrop click-to-close |
| `alert` | `Alert`, `AlertVariant` | Riquadri di notifica con icone semantiche e dismiss |
| `notification` | `NotificationProvider`, `use_notification` | Sistema Toast globale a comparsa |
| `tabs` | `TabList`, `Tab`, `TabPanel` | Navigazione a schede orizzontali conformi ad APG |
| `switch` | `Switch` | Interruttori a scatto tattili ad alta precisione |
| `tooltip` | `Tooltip`, `TooltipPosition` | Micro-etichette fluttuanti su hover a 4 direzioni |
| `drawer` | `Drawer`, `DrawerPosition`, `DrawerSize` | Pannelli laterali a scomparsa da sinistra o destra |
| `dropdown` | `Dropdown`, `DropdownItem`, `DropdownPlacement` | Menu a tendina ancorati al trigger |
| `avatar` | `Avatar`, `AvatarSize`, `AvatarShape` | Avatar utente con indicatori di presenza e fallback |
| `chat` | `MessageBubble`, `BubbleDirection`, `DeliveryStatus` | Bolle messaggio con timestamp, stato e badge |
| `progress` | `ProgressBar`, `ProgressVariant`, `ProgressSize` | Barre di progresso determinate e indeterminate con glow |
| `attachment_card` | `AttachmentCard`, `FileKind` | Card allegato con progresso e azioni rapide |
| `contact_card` | `ContactCard` | Schede contatto con avatar e dettagli di stato |
| `segmented_control` | `SegmentedControl`, `SegmentedOption` | Selettori a segmenti tattili fluidi |
| `divider` | `Divider`, `DividerOrientation` | Divisori orizzontali e verticali con etichette opzionali |
| `tree` | `TreeView`, `TreeNode` | Alberi gerarchici espandibili |
| `skeleton` | `Skeleton`, `SkeletonShape` | Segnaposto scheletrati con animazione shimmer |
| `icon` | Native SVG Icons (50 `IconKind`) | Icone vettoriali ad alta nitidezza senza dipendenze esterne |
| `typography` | `Heading`, `Text`, `Kbd`, `Code` | Componenti tipografici con scala Geist |
| `animation` | `Transition`, `AnimationVariant` | Transizioni ottiche e micro-animazioni Dioxus |
| `theme` | `ThemeProvider`, `use_theme`, `Theme`, `ThemeBuilder` | Engine reattivo Mantine con 5 preset, custom HEX e builder |
| `serde` | `Serialize`, `Deserialize` | Serializzazione e deserializzazione JSON per Theme e Overrides |
| `all` | *Tutti i componenti* | Abilita l'intera suite |

---

## 💻 Esempio d'Uso

```rust
use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn MyView() -> Element {
    rsx! {
        Alert {
            title: "Operazione completata".to_string(),
            variant: AlertVariant::Success,
            "Tutti i componenti sono sincronizzati correttamente in puro SCSS!"
        }

        Card {
            title: "Scheda di Esempio".to_string(),
            shadow: CardShadow::Md,
            p { "Contenuto del corpo della scheda." }
            Button {
                variant: ButtonVariant::Primary,
                onclick: move |_| println!("Pulsante premuto!"),
                "Conferma"
            }
        }
    }
}
```

---

## 🎨 Sistema del Tema Mantine (`theme`)

Con la feature `theme` attiva, puoi commutare dinamicamente il colore primario, il raggio di curvatura e il micro-glow su tutti i componenti senza duplicare stili:

- **Preset inclusi**:
  - `Theme::shadow()`: Ultraviolet Shadow (Default viola profondo)
  - `Theme::obsidian()`: Obsidian Titanium (Monocromatico rigoroso)
  - `Theme::crimson()`: Crimson Phoenix (Rosso fenice)
  - `Theme::cyan()`: Cyber Cyan (Ciano elettrico)
  - `Theme::emerald()`: Emerald Matrix (Verde smeraldo)
