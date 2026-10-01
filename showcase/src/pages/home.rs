use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "max-w-4xl space-y-8",
            div { class: "space-y-3",
                h1 { class: "text-3xl font-extrabold tracking-tight text-white", "Shadow Design" }
                p { class: "text-zinc-400 text-lg leading-relaxed",
                    "Libreria di componenti UI modulari per Dioxus 0.7, progettata per essere condivisa tra tutti i tuoi siti web con supporto per feature-flagging per ogni singolo componente."
                }
            }

            // Architecture Grid
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 pt-2",
                Card {
                    title: "📦 crate (shadow-design)".to_string(),
                    shadow: CardShadow::Md,
                    p { class: "text-sm text-zinc-300 mb-3",
                        "La libreria UI vera e propria. Include tutti i componenti isolati e compilabili singolarmente con le relative feature."
                    }
                    div { class: "flex flex-wrap gap-1.5",
                        Badge { label: "button".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "card".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "input".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "badge".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "navbar".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "modal".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "alert".to_string(), variant: BadgeVariant::Primary }
                    }
                }

                Card {
                    title: "🖼️ showcase (espositore)".to_string(),
                    shadow: CardShadow::Md,
                    p { class: "text-sm text-zinc-300 mb-3",
                        "Questa applicazione Dioxus funzionante da catalogo ed espositore interattivo per testare e visualizzare ogni componente prima di usarlo."
                    }
                    div { class: "flex items-center gap-2",
                        Badge { label: "Dioxus Router".to_string(), variant: BadgeVariant::Success }
                        Badge { label: "Tailwind CSS".to_string(), variant: BadgeVariant::Success }
                    }
                }
            }

            // Quick Usage Guide
            Card {
                title: "Come utilizzarlo nei tuoi siti web".to_string(),
                shadow: CardShadow::Lg,
                div { class: "space-y-4 text-sm text-zinc-300",
                    p { "Nei tuoi progetti Dioxus esterni, aggiungi la dipendenza selezionando solo i componenti che desideri:" }
                    pre { class: "p-4 rounded-xl bg-zinc-950 font-mono text-xs text-blue-300 border border-zinc-800 overflow-x-auto",
                        "[dependencies]\nshadow-design = {{ path = \"../shadow-design/crate\", features = [\"button\", \"card\"] }}\n\n# Oppure per includere tutto:\n# shadow-design = {{ path = \"../shadow-design/crate\", features = [\"all\"] }}"
                    }
                    p { "Nel tuo codice Rust Dioxus 0.7:" }
                    pre { class: "p-4 rounded-xl bg-zinc-950 font-mono text-xs text-emerald-300 border border-zinc-800 overflow-x-auto",
                        "use dioxus::prelude::*;\nuse shadow_design::prelude::*;\n\n#[component]\npub fn View() -> Element {{\n    rsx! {{\n        Card {{\n            title: \"Titolo della Card\".to_string(),\n            Button {{\n                variant: ButtonVariant::Primary,\n                \"Clicca Qui\"\n            }}\n        }}\n    }}\n}}"
                    }
                }
            }

            // Tailwind Config Card
            Card {
                title: "Configurazione Tailwind CSS (Fondamentale)".to_string(),
                shadow: CardShadow::Lg,
                div { class: "space-y-4 text-sm text-zinc-300",
                    p {
                        "Poiché le classi CSS risiedono nei file sorgente Rust della crate, devi indicare al compilatore Tailwind dove trovarle."
                    }
                    div { class: "space-y-2",
                        p { class: "font-semibold text-white", "1. Se usi la crate in locale (path dependency):" }
                        pre { class: "p-3 rounded-lg bg-zinc-950 font-mono text-xs text-amber-300 border border-zinc-800",
                            "@import \"tailwindcss\";\n@source \"../shadow-design/crate\";"
                        }
                    }
                    div { class: "space-y-2",
                        p { class: "font-semibold text-white", "2. Se scarichi la crate tramite Cargo (Git o Crates.io):" }
                        pre { class: "p-3 rounded-lg bg-zinc-950 font-mono text-xs text-amber-300 border border-zinc-800",
                            "@import \"tailwindcss\";\n@source \"~/.cargo/registry/src/**/shadow-design*\";\n@source \"~/.cargo/git/checkouts/**/shadow-design*\";"
                        }
                    }
                }
            }
        }
    }
}
