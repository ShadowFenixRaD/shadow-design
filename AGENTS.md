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
│   └── src/
│       ├── lib.rs           # Entry point libreria, re-export condizionali e prelude
│       ├── theme.rs         # Sistema reattivo del tema, Context e Preset
│       ├── button.rs        # Componente Button, varianti e dimensioni
│       ├── card.rs          # Componente Card con header, footer e ombre
│       ├── input.rs         # Componente TextInput collegato a Signal<String>
│       ├── badge.rs         # Componente Badge con stili pill/rettangolari
│       ├── navbar.rs        # Componente Navbar responsive
│       ├── modal.rs         # Componente Modal controllato da Signal<bool>
│       └── alert.rs         # Componente Alert con icone automatiche
└── showcase/                # Crate applicativo: Espositore componenti
    ├── Cargo.toml           # Manifest dell'app (dipende da shadow-design { all })
    ├── Dioxus.toml          # Configurazione Dioxus CLI (dx)
    ├── tailwind.css         # Configurazione sorgente Tailwind CSS v4 (@utility)
    ├── assets/
    │   ├── main.css         # Reset globale, scrollbar viola scuro, stili custom
    │   ├── tailwind.css     # CSS compilato per il browser
    │   ├── header.svg       # Logo vettoriale Shadowfenix
    │   └── favicon.ico      # Icona dell'applicazione
    └── src/
        ├── main.rs          # App root, caricamento asset e Dioxus Router
        ├── components/
        │   ├── mod.rs
        │   ├── layout.rs    # Shell con Navbar, Sidebar e Theme Switcher live
        │   └── banner.rs    # Banner informativo con comando Cargo feature
        └── pages/
            ├── mod.rs
            ├── home.rs      # Panoramica, hero e test dinamico del Theme
            ├── buttons.rs   # Showcase e playground interattivo di Button
            ├── cards.rs     # Showcase di Card e livelli di ombra
            ├── inputs.rs    # Showcase di TextInput con binding reattivo
            ├── badges.rs    # Showcase di Badge e varianti
            ├── alerts.rs    # Showcase di Alert di sistema
            ├── modals.rs    # Showcase e test di apertura modale
            └── navbar.rs    # Anteprima di Navbar
```

---

## 3. Crate Libreria (`crate` / `shadow-design`)

### Feature Flags Cargo
Il file `crate/Cargo.toml` definisce feature individuali per consentire ai consumatori di importare esclusivamente ciò di cui necessitano:

| Feature Flag | Descrizione | Modulo |
| :--- | :--- | :--- |
| `all` | Abilita l'intera suite di componenti e il tema | Include tutte le feature sottostanti |
| `button` | Pulsanti con varianti, dimensioni e stati interattivi | `crate::button` |
| `card` | Schede contenitore con intestazione, footer e ombre | `crate::card` |
| `input` | Input di testo reattivi collegati a `Signal<String>` | `crate::input` |
| `badge` | Etichette di stato e indicatori a pillola | `crate::badge` |
| `navbar` | Barra di navigazione responsive | `crate::navbar` |
| `modal` | Finestre di dialogo in overlay con backdrop | `crate::modal` |
| `alert` | Riquadri di notifica con icone semantiche | `crate::alert` |
| `theme` | Sistema reattivo per personalizzazione classi UI | `crate::theme` |

### Supporto Duale al Tema (`Theme` vs Fallback statico)
Tutti i componenti in `crate/src/` sono scritti per funzionare in due modalità grazie a `#[cfg]`:
1. **Con feature `theme` attiva**: Leggono le classi Tailwind in modo reattivo invocando `crate::theme::use_theme()`. Quando il tema cambia a runtime, tutti i componenti istanziati si aggiornano immediatamente.
2. **Senza feature `theme` attiva**: Fanno fallback trasparente su classi statiche predefinite hardcoded nei componenti (zero overhead se il sistema a tema non è richiesto).

### Sistema del Tema (`crate/src/theme.rs`)
- **`Theme`**: Struttura dati che raggruppa tutti i token CSS di layout, accenti e classi di ogni componente.
- **Preset inclusi**:
  - `Theme::shadow()` (Default): Nero assoluto + viola intenso.
  - `Theme::obsidian()`: Nero puro + grafite/argento minimalista.
  - `Theme::crimson()`: Nero + rosso cremisi acceso della fenice.
- **Builder pattern**: Metodi come `.with_primary(...)`, `.with_card_base(...)`, `.with_glow(...)` per personalizzare o creare varianti aziendali al volo.
- **Context API Dioxus**: `ThemeProvider` per fornire il tema all'albero di componenti e `use_theme()` per consumarlo o scrivervi (`theme.write()`).

---

## 4. Crate Showcase (`showcase`)

È l'applicazione web interattiva usata per collaudare e mostrare visivamente tutti i componenti.

- **Routing (Dioxus 0.7)**: Gestito tramite l'enum `Route` in `showcase/src/main.rs`.
- **Layout Shell (`showcase/src/components/layout.rs`)**:
  - Fornisce la barra superiore con logo Shadowfenix e selettore in tempo reale del tema (`Shadow`, `Obsidian`, `Crimson`).
  - Barra laterale con navigazione alle pagine dei singoli componenti.
  - Renderizza la pagina corrente tramite `Outlet::<Route> {}`.
- **Tailwind CSS v4 Integration**:
  - `showcase/tailwind.css` fa `@import "tailwindcss";` e include come `@source` sia la cartella `../crate` che `./src`.
  - Definisce utility custom come `@utility theme-root`, `@utility theme-card`, `@utility theme-glow`.

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
```
