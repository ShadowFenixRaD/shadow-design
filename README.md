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
shadow-design = { path = "../shadow-design/crate", features = ["button", "card", "input"] }

# Oppure importa tutti i componenti:
# shadow-design = { path = "../shadow-design/crate", features = ["all"] }
```

### Feature disponibili

| Feature | Componente | Descrizione |
|---|---|---|
| `button` | `Button`, `ButtonVariant`, `ButtonSize` | Pulsanti stilizzati con varianti cromatiche e dimensioni |
| `card` | `Card`, `CardShadow` | Contenitori a schede con vari livelli di ombra, header e footer |
| `input` | `TextInput` | Campi di input reattivi per testi e password con gestione errori |
| `badge` | `Badge`, `BadgeVariant` | Etichette e pillole di stato |
| `navbar` | `Navbar` | Barra di navigazione responsive con brand e link |
| `modal` | `Modal` | Finestre di dialogo con backdrop e controllate da Signal |
| `alert` | `Alert`, `AlertVariant` | Notifiche e banner colorati |
| `theme` | `Theme`, `ThemeProvider`, `use_theme` | Gestione centralizzata del tema, classi pre-fatte e personalizzazione globale dinamica |
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
