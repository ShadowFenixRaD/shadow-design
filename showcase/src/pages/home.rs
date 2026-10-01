use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn Home() -> Element {
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
                        ". Caratterizzata da uno sfondo nero assoluto (0 0 0), accenti in viola scuro intenso e supporto al feature-flagging per ogni singolo componente."
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

            // Theme Palette Specification Card
            Card {
                title: "Identità Visiva: Shadow & Shadowfenix".to_string(),
                subtitle: "Specifiche cromatiche del tema predefinito".to_string(),
                shadow: CardShadow::Lg,
                div { class: "space-y-4",
                    p { class: "text-sm text-zinc-300 leading-relaxed",
                        "La palette rispecchia l'oscurità profonda e il fuoco mistico della Fenice delle Ombre:"
                    }
                    div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4 pt-1",
                        // Nero Assoluto
                        div { class: "p-4 rounded-xl bg-black border border-purple-950/70 flex flex-col gap-2 shadow-inner",
                            div { class: "w-full h-10 rounded-lg bg-black border border-zinc-800 flex items-center justify-center text-xs font-mono text-zinc-600",
                                "RGB (0, 0, 0)"
                            }
                            span { class: "text-xs font-bold text-white uppercase tracking-wider", "Nero Assoluto" }
                            span { class: "text-[11px] text-zinc-400", "Sfondo principale di default, senza gradazioni chiare." }
                        }
                        // Viola Ombra Scuro
                        div { class: "p-4 rounded-xl bg-[#09080e] border border-purple-900/50 flex flex-col gap-2",
                            div { class: "w-full h-10 rounded-lg bg-[#581c87] border border-purple-800/60 shadow-[0_0_15px_rgba(88,28,135,0.4)]" }
                            span { class: "text-xs font-bold text-purple-300 uppercase tracking-wider", "Viola Ombra Profondo" }
                            span { class: "text-[11px] text-zinc-400", "Colore secondario per bordi, ombre e superfici." }
                        }
                        // Viola Acceso Intenso
                        div { class: "p-4 rounded-xl bg-[#09080e] border border-purple-900/50 flex flex-col gap-2",
                            div { class: "w-full h-10 rounded-lg bg-[#7e22ce] border border-purple-600/70 shadow-[0_0_15px_rgba(126,34,206,0.5)]" }
                            span { class: "text-xs font-bold text-purple-200 uppercase tracking-wider", "Viola Acceso Scuro" }
                            span { class: "text-[11px] text-zinc-400", "Pulsanti primari, focus, pillole e azioni chiave." }
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
                        "[dependencies]\nshadow-design = {{ path = \"../shadow-design/crate\", features = [\"button\", \"card\"] }}\n\n# Oppure per includere tutta la suite:\n# shadow-design = {{ path = \"../shadow-design/crate\", features = [\"all\"] }}"
                    }
                    p { "Nel tuo codice Rust Dioxus 0.7:" }
                    pre { class: "p-4 rounded-xl bg-black font-mono text-xs text-emerald-300 border border-purple-950/80 overflow-x-auto shadow-inner",
                        "use dioxus::prelude::*;\nuse shadow_design::prelude::*;\n\n#[component]\npub fn View() -> Element {{\n    rsx! {{\n        Card {{\n            title: \"Titolo della Card\".to_string(),\n            Button {{\n                variant: ButtonVariant::Primary,\n                \"Azione Primaria\"\n            }}\n        }}\n    }}\n}}"
                    }
                }
            }

            // Tailwind Config Card
            Card {
                title: "Integrazione Tailwind CSS v4".to_string(),
                shadow: CardShadow::Lg,
                div { class: "space-y-4 text-sm text-zinc-300",
                    p {
                        "Per compilare le classi Tailwind utilizzate dai componenti della crate:"
                    }
                    div { class: "space-y-2",
                        p { class: "font-semibold text-white", "Nel tuo tailwind.css:" }
                        pre { class: "p-3 rounded-lg bg-black font-mono text-xs text-purple-300 border border-purple-950/80",
                            "@import \"tailwindcss\";\n@source \"../shadow-design/crate\";"
                        }
                    }
                }
            }
        }
    }
}
