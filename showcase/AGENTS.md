# Showcase — Istruzioni Agenti

Questo crate fa parte del workspace **Shadow Design**.

Per l'architettura completa del workspace, la struttura di tutti i componenti, le regole di linting Dioxus 0.7 e le convenzioni di sviluppo, fai riferimento al file principale:
👉 **[AGENTS.md](../AGENTS.md)** nella root del repository.

### Note specifiche per Showcase
- **Esecuzione dev server**: `dx serve` eseguito all'interno di questa directory (`showcase/`). Integrazione asset SCSS automatica e diretta.
- **Pagine & Router**: Tutte le route sono censite nell'enum `Route` in [showcase/src/main.rs](file:///c:/Users/KillerBoss/Documents/shadow-design/showcase/src/main.rs).
- **Stili SCSS**: Gli stili risiedono in `crate/scss/` e vengono inclusi tramite `shadow_design::STYLES`. Nessun comando o compilatore esterno richiesto.
