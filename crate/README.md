# Shadow Design

Libreria di componenti UI modulari per Dioxus 0.7 per l'ecosistema **shadowfenix**, caratterizzata dal tema a sfondo nero assoluto (`0 0 0`) ed accenti viola intenso.

## 📦 Installazione in Cargo.toml

Aggiungi `shadow-design` al `Cargo.toml` del tuo sito web abilitando solo le feature dei componenti necessari:

```toml
[dependencies]
shadow-design = { path = "../shadow-design/crate", features = ["button", "card", "alert"] }

# Oppure importa tutti i componenti:
# shadow-design = { path = "../shadow-design/crate", features = ["all"] }
```

---

## 🎨 Configurazione Tailwind CSS per l'End-User

Poiché i componenti `shadow-design` utilizzano utility class Tailwind all'interno dei file Rust della crate, il compilatore Tailwind del tuo progetto deve essere informato di dove trovare tali classi, altrimenti non verranno generate nel file CSS finale.

### Caso 1: Progetto Locale / Monorepo (Path Dependency)

Se utilizzi una path dependency nel tuo `Cargo.toml`:

#### Con Tailwind v4 (consigliato su Dioxus 0.7+)
Nel file `tailwind.css` della tua applicazione, aggiungi la direttiva `@source`:

```css
@import "tailwindcss";

/* Percorso relativo alla cartella crate di shadow-design */
@source "../shadow-design/crate";
```

#### Con Tailwind v3 (tailwind.config.js)
Nel tuo `tailwind.config.js`, estendi la lista dei file in `content`:

```javascript
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{rs,html}",
    "../shadow-design/crate/src/**/*.rs",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

---

### Caso 2: Crate remota (da Crates.io o Git Repository)

Se importi la crate da un repository Git o da Crates.io:
```toml
[dependencies]
# Da Git:
shadow-design = { git = "https://github.com/tuo-utente/shadow-design", features = ["all"] }

# O da Crates.io:
# shadow-design = { version = "0.1", features = ["all"] }
```

Cargo memorizza i sorgenti delle dipendenze nella cache globale (`~/.cargo/`). Per consentire a Tailwind di scansionare i sorgenti della crate:

#### Con Tailwind v4
Aggiungi la scansione della cache di Cargo nel tuo `tailwind.css`:

```css
@import "tailwindcss";

/* Scansione dipendenza scaricata da Cargo (crates.io o git) */
@source "~/.cargo/registry/src/**/shadow-design*";
@source "~/.cargo/git/checkouts/**/shadow-design*";
```

#### Con Tailwind v3 (tailwind.config.js)
Nel tuo `tailwind.config.js`:

```javascript
const path = require("path");
const os = require("os");

module.exports = {
  content: [
    "./src/**/*.{rs,html}",
    // Scansione sorgenti scaricati da Cargo
    path.join(os.homedir(), ".cargo/registry/src/**/shadow-design*/**/*.rs"),
    path.join(os.homedir(), ".cargo/git/checkouts/**/shadow-design*/**/*.rs"),
  ],
};
```

---

## 🧩 Feature Disponibili

| Feature | Componente | Descrizione |
|---|---|---|
| `button` | `Button`, `ButtonVariant`, `ButtonSize` | Pulsanti con varianti, dimensioni e stati |
| `card` | `Card`, `CardShadow` | Contenitori a schede con ombre, header e footer |
| `input` | `TextInput` | Campi di input reattivi per testi e password con gestione errori |
| `badge` | `Badge`, `BadgeVariant` | Etichette e pillole di stato |
| `navbar` | `Navbar` | Barra di navigazione responsive con brand e link |
| `modal` | `Modal` | Finestre di dialogo sovrapposte con backdrop controllate da Signal |
| `alert` | `Alert`, `AlertVariant` | Notifiche cromatiche con bordi accentati e icone semantiche |
| `all` | *Tutti i componenti* | Abilita l'intera suite |

---

## 💻 Esempio d'Uso in Dioxus 0.7

```rust
use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn MyView() -> Element {
    rsx! {
        Alert {
            title: "Operazione completata".to_string(),
            variant: AlertVariant::Success,
            "Tutti i componenti sono sincronizzati correttamente!"
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

## 🎨 Feature `theme`: Classi Pre-fatte e Personalizzazione Globale

Abilitando la feature `theme`, hai a disposizione il gestore del tema centralizzato:

```toml
[dependencies]
shadow-design = { path = "../shadow-design/crate", features = ["button", "card", "theme"] }
```

### 1. Utilizzo diretto delle classi pre-fatte:

```rust
use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn Layout() -> Element {
    let theme = use_theme();

    rsx! {
        div { class: "{theme().root}",
            header { class: "{theme().header}", "Header" }
            aside { class: "{theme().sidebar}", "Sidebar" }
            main { class: "{theme().content}",
                Button { variant: ButtonVariant::Primary, "Pulsante Dinamico" }
            }
        }
    }
}
```

### 2. Personalizzare o sovrascrivere il tema globale:

```rust
// Con il ThemeProvider a livello radice:
let my_theme = Theme::shadow()
    .with_primary("bg-indigo-700 hover:bg-indigo-600 text-white shadow-lg")
    .with_card_base("bg-zinc-950 border border-indigo-900 rounded-2xl");

rsx! {
    ThemeProvider { theme: use_signal(|| my_theme),
        App {}
    }
}

// Oppure modificando il Signal<Theme> da qualsiasi punto dell'albero:
let mut theme = use_theme();
*theme.write() = Theme::obsidian(); // cambia al preset Obsidian
theme.write().button_primary = "bg-purple-600 text-white".to_string(); // sovrascrittura mirata
```

