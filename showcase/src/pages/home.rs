use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut theme = use_theme();
    let custom_accent = use_signal(|| "".to_string());

    rsx! {
        div { class: "max-w-4xl space-y-10",
            // Hero section with shadow aura
            div { class: "relative overflow-hidden rounded-2xl border border-purple-900/60 bg-gradient-to-b from-[#120520] via-[#08030f] to-black p-8 shadow-[0_0_50px_rgba(88,28,135,0.25)]",
                div { class: "relative z-10 space-y-4",
                    div { class: "flex items-center gap-3",
                        span { class: "px-3 py-1 text-xs font-mono font-semibold rounded-full bg-purple-950/80 text-purple-300 border border-purple-800/70 shadow-[0_0_12px_rgba(147,51,234,0.3)]",
                            "🔥 ORGANIZZAZIONE SHADOWFENIX"
                        }
                        span { class: "text-xs text-zinc-500 font-mono", "TEMA: SHADOW & PURPLE" }
                    }
                    h1 { class: "text-4xl font-extrabold tracking-tight text-white",
                        "Shadow Design"
                    }
                    p { class: "text-zinc-300 text-lg leading-relaxed max-w-2xl",
                        "Libreria di componenti UI modulari per Dioxus 0.7 creata per l'ecosistema "
                        strong { class: "text-purple-300 font-semibold", "shadowfenix" }
                        ". Caratterizzata da uno sfondo nero assoluto (0 0 0), accenti in viola scuro intenso, classi pre-fatte e supporto completo per la personalizzazione tramite la nuova feature "
                        strong { class: "text-purple-300 font-mono", "Theme" }
                        "."
                    }
                    div { class: "flex flex-wrap items-center gap-3 pt-2",
                        Button {
                            variant: ButtonVariant::Primary,
                            "Esplora Componenti"
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            "Documentazione Cargo"
                        }
                    }
                }
                // Atmospheric decorative elements
                div { class: "absolute -right-16 -top-16 w-64 h-64 rounded-full bg-purple-600/15 blur-3xl pointer-events-none" }
                div { class: "absolute right-20 -bottom-10 w-48 h-48 rounded-full bg-purple-900/20 blur-2xl pointer-events-none" }
            }

            // Theme Feature Card
            Card {
                title: "🎨 Feature Theme: Classi Pre-fatte e Personalizzazione Totale".to_string(),
                subtitle: "Non occorre riscrivere codice e classi ripetitive in decine di file".to_string(),
                shadow: CardShadow::Lg,
                div { class: "space-y-6",
                    p { class: "text-sm text-zinc-300 leading-relaxed",
                        "Grazie alla feature "
                        code { class: "px-1.5 py-0.5 rounded bg-black text-purple-300 font-mono text-xs border border-purple-900/60", "theme" }
                        ", puoi usare classi pre-fatte centralizzate senza duplicare stili. Tutti i componenti ("
                        code { class: "text-xs font-mono text-zinc-400", "Button, Card, TextInput, Badge, Navbar, Modal, Alert" }
                        ") adottano automaticamente il tema corrente e si aggiornano in tempo reale."
                    }

                    // Playground Interattivo di personalizzazione
                    div { class: "p-5 rounded-xl bg-black border border-purple-950/80 space-y-4 shadow-inner",
                        div { class: "flex items-center justify-between",
                            span { class: "text-xs font-bold text-white uppercase tracking-wider font-mono",
                                "Collaudo Reattività Tema (Live Preview)"
                            }
                            span { class: "text-xs font-mono px-2 py-0.5 rounded bg-purple-950/80 text-purple-300 border border-purple-800/60",
                                "Attivo: {theme().name}"
                            }
                        }

                        div { class: "flex flex-wrap items-center gap-2",
                            span { class: "text-xs text-zinc-400 mr-2", "Preset disponibili:" }
                            Button {
                                size: ButtonSize::Sm,
                                variant: if theme().name.contains("Shadow") { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                                onclick: move |_| *theme.write() = Theme::shadow(),
                                "Shadow (Nero + Viola Scuro)"
                            }
                            Button {
                                size: ButtonSize::Sm,
                                variant: if theme().name.contains("Obsidian") { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                                onclick: move |_| *theme.write() = Theme::obsidian(),
                                "Obsidian Stealth"
                            }
                            Button {
                                size: ButtonSize::Sm,
                                variant: if theme().name.contains("Crimson") { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                                onclick: move |_| *theme.write() = Theme::crimson(),
                                "Crimson Fenix"
                            }
                        }

                        // Form per sovrascrittura personalizzata
                        div { class: "pt-2 border-t border-purple-950/60 flex flex-col sm:flex-row items-stretch sm:items-end gap-3",
                            div { class: "flex-1",
                                TextInput {
                                    value: custom_accent,
                                    label: "Sovrascrivi pulsante primario con classi custom:".to_string(),
                                    placeholder: "es: bg-emerald-600 hover:bg-emerald-500 text-white shadow-lg".to_string(),
                                }
                            }
                            Button {
                                size: ButtonSize::Md,
                                variant: ButtonVariant::Primary,
                                onclick: move |_| {
                                    let val = custom_accent();
                                    if !val.trim().is_empty() {
                                        theme.write().button_primary = val;
                                        theme.write().name = "Personalizzato".to_string();
                                    }
                                },
                                "Applica Sovrascrittura"
                            }
                        }
                    }

                    // Guida all'uso in codice Rust
                    div { class: "space-y-3",
                        p { class: "font-semibold text-white text-sm", "1. Come usare le classi pre-fatte nel tuo codice Rust:" }
                        pre { class: "p-4 rounded-xl bg-black font-mono text-xs text-purple-300 border border-purple-950/80 overflow-x-auto shadow-inner",
                            "use dioxus::prelude::*;\nuse shadow_design::prelude::*;\n\n#[component]\npub fn View() -> Element {{\n    // Accesso diretto alle classi pre-fatte del tema:\n    let theme = use_theme();\n\n    rsx! {{\n        div {{\n            class: \"{{theme().root}}\",\n            header {{ class: \"{{theme().header}}\", \"Mio Header\" }}\n            aside {{ class: \"{{theme().sidebar}}\", \"Mia Sidebar\" }}\n            main {{\n                class: \"{{theme().content}}\",\n                Button {{ variant: ButtonVariant::Primary, \"Pulsante a tema\" }}\n            }}\n        }}\n    }}\n}}"
                        }
                    }

                    // Guida a sovrascrittura e personalizzazione
                    div { class: "space-y-3",
                        p { class: "font-semibold text-white text-sm", "2. Come personalizzare e sovrascrivere il tema globale:" }
                        pre { class: "p-4 rounded-xl bg-black font-mono text-xs text-emerald-300 border border-purple-950/80 overflow-x-auto shadow-inner",
                            "// Creazione di un tema personalizzato con il builder:\nlet mio_tema = Theme::shadow()\n    .with_name(\"Mio Brand\")\n    .with_primary(\"bg-indigo-700 hover:bg-indigo-600 text-white shadow-lg\")\n    .with_card_base(\"bg-[#0a0a10] border border-indigo-950 rounded-2xl\");\n\n// Fornitura a livello root dell'applicazione:\nrsx! {{\n    ThemeProvider {{\n        theme: use_signal(|| mio_tema),\n        App {{}}\n    }}\n}}\n\n// Oppure modifica a runtime da qualsiasi componente:\nlet mut theme = use_theme();\ntheme.write().button_primary = \"bg-purple-600 text-white\".to_string();"
                        }
                    }

                    // Utility CSS Tailwind
                    div { class: "space-y-3",
                        p { class: "font-semibold text-white text-sm", "3. Classi pre-fatte disponibili in Tailwind CSS:" }
                        p { class: "text-xs text-zinc-400",
                            "Se preferisci applicare le classi direttamente nel markup HTML/RSX senza invocare use_theme(), sono definite le seguenti utility:"
                        }
                        div { class: "grid grid-cols-2 sm:grid-cols-3 gap-2 font-mono text-xs",
                            div { class: "p-2 rounded bg-black border border-purple-950/70 text-purple-300", "theme-root" }
                            div { class: "p-2 rounded bg-black border border-purple-950/70 text-purple-300", "theme-header" }
                            div { class: "p-2 rounded bg-black border border-purple-950/70 text-purple-300", "theme-sidebar" }
                            div { class: "p-2 rounded bg-black border border-purple-950/70 text-purple-300", "theme-card" }
                            div { class: "p-2 rounded bg-black border border-purple-950/70 text-purple-300", "theme-surface" }
                            div { class: "p-2 rounded bg-black border border-purple-950/70 text-purple-300", "theme-glow" }
                        }
                    }
                }
            }

            // Architecture Grid
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                Card {
                    title: "📦 crate (shadow-design)".to_string(),
                    shadow: CardShadow::Md,
                    p { class: "text-sm text-zinc-300 mb-3 leading-relaxed",
                        "La libreria UI condivisa di shadowfenix. Ogni componente è isolato e compilabile singolarmente attivando la relativa feature."
                    }
                    div { class: "flex flex-wrap gap-1.5",
                        Badge { label: "button".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "card".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "input".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "badge".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "navbar".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "modal".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "alert".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "theme".to_string(), variant: BadgeVariant::Primary }
                    }
                }

                Card {
                    title: "🖼️ showcase (espositore)".to_string(),
                    shadow: CardShadow::Md,
                    p { class: "text-sm text-zinc-300 mb-3 leading-relaxed",
                        "Applicazione interattiva per visualizzare, verificare e collaudare ogni singolo elemento prima di integrarlo nei progetti web."
                    }
                    div { class: "flex items-center gap-2",
                        Badge { label: "Dioxus 0.7".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "Tailwind CSS v4".to_string(), variant: BadgeVariant::Success }
                        Badge { label: "Reactive Theme".to_string(), variant: BadgeVariant::Primary }
                    }
                }
            }

            // Quick Usage Guide
            Card {
                title: "Come utilizzarlo nei progetti shadowfenix".to_string(),
                shadow: CardShadow::Lg,
                div { class: "space-y-4 text-sm text-zinc-300",
                    p { "Aggiungi la dipendenza selezionando solo i componenti desiderati per un bundle snello:" }
                    pre { class: "p-4 rounded-xl bg-black font-mono text-xs text-purple-300 border border-purple-950/80 overflow-x-auto shadow-inner",
                        "[dependencies]\nshadow-design = {{ path = \"../shadow-design/crate\", features = [\"button\", \"card\", \"theme\"] }}\n\n# Oppure per includere tutta la suite:\n# shadow-design = {{ path = \"../shadow-design/crate\", features = [\"all\"] }}"
                    }
                    p { "Nel tuo codice Rust Dioxus 0.7:" }
                    pre { class: "p-4 rounded-xl bg-black font-mono text-xs text-emerald-300 border border-purple-950/80 overflow-x-auto shadow-inner",
                        "use dioxus::prelude::*;\nuse shadow_design::prelude::*;\n\n#[component]\npub fn View() -> Element {{\n    rsx! {{\n        Card {{\n            title: \"Titolo della Card\".to_string(),\n            Button {{\n                variant: ButtonVariant::Primary,\n                \"Azione Primaria\"\n            }}\n        }}\n    }}\n}}"
                    }
                }
            }
        }
    }
}
