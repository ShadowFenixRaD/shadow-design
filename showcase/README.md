# Shadow Design — Showcase App

Espositore interattivo per la libreria di componenti UI **Shadow Design** basata su Dioxus 0.7.

## 🚀 Sviluppo Locale (Integrazione Diretta Dioxus 0.7)

L'applicazione utilizza l'integrazione diretta degli asset di Dioxus 0.7 per compilare nativamente il master SCSS della libreria (`shadow_design::STYLES`), **senza bisogno di comandi aggiuntivi, senza npm e senza Tailwind**:

```bash
dx serve
```

Per eseguire su piattaforma desktop:
```bash
dx serve --platform desktop
```

## 🎨 Architettura Stili

- **Zero-Tailwind**: TailwindCSS è stato rimosso e disabilitato al 100%.
- **Pure SCSS**: L'intera suite di fogli di stile è modulare e risiede in `crate/scss/`, compilata direttamente dal pipeline asset di Dioxus (`asset!("/scss/main.scss")`).
- **Mantine Theme Engine**: Supporto reattivo per palette di colori primari (Violet, Obsidian, Crimson, Cyan, Emerald), scale di raggio di curvatura e micro-glow ottico.
