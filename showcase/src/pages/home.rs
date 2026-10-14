use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::CodeBlock;
const CARGO_FEATURES: &[&str] = &[
    "button",
    "card",
    "input",
    "select",
    "textarea",
    "checkbox",
    "badge",
    "navbar",
    "modal",
    "alert",
    "theme",
    "icon",
    "typography",
    "notification",
    "tabs",
    "switch",
    "tooltip",
    "drawer",
    "dropdown",
    "animation",
    "avatar",
    "chat",
    "progress",
    "attachment_card",
    "contact_card",
    "segmented_control",
    "divider",
    "tree",
    "skeleton",
];

#[component]
pub fn Home() -> Element {
    let mut theme = use_theme();
    let nav = use_navigator();

    let install_code = r#"[dependencies]
# Importa solo i moduli desiderati con default-features = false (zero bloat):
shadow-design = { version = "0.3.0", default-features = false, features = ["button", "card", "theme", "icon"] }

# Oppure include l'intera suite:
# shadow-design = { version = "0.3.0", features = ["all"] }"#
        .to_string();

    let rust_example = r#"use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        // Includi il foglio di stile SCSS nativo
        document::Link { rel: "stylesheet", href: shadow_design::STYLES }

        Card {
            title: "Pannello Operativo".to_string(),
            variant: CardVariant::Interactive,
            hover_glow: true,
            p { "Interfaccia tactile dark mode pronta all'uso." }
            Button {
                variant: ButtonVariant::Primary,
                left_icon: rsx! { Icon { kind: IconKind::Check, size: 15 } },
                "Salva Modifiche"
            }
        }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-10",
            // Hero Section: Cyber-Industrial Tactile Craft
            div { class: "relative overflow-hidden rounded-2xl border border-white/[0.08] bg-[#07070b] p-8 sm:p-10 shadow-2xl shadow-black/80 specular-top",
                div { class: "relative z-10 space-y-5",
                    div { class: "flex items-center gap-3",
                        Badge {
                            label: "SHADOWFENIX ECOSYSTEM".to_string(),
                            variant: BadgeVariant::Primary,
                            pill: true,
                            dot: true,
                        }
                        span { class: "text-xs text-zinc-500 font-mono", "DIOXUS 0.7 • CRAFT UI" }
                    }
                    div { class: "space-y-2",
                        Heading { order: HeadingOrder::H1, "Shadow Design" }
                        Text { size: TextSize::Lg, dimmed: true,
                            "Libreria di componenti UI modulare ad alte prestazioni per Dioxus 0.7. Architettura del tema in stile Mantine, gerarchia a 4 livelli di superficie scura, finiture hairline a 1px con top specular highlights e zero dipendenze esterne."
                        }
                    }
                    div { class: "flex flex-wrap items-center gap-3 pt-2",
                        Button {
                            variant: ButtonVariant::Primary,
                            left_icon: rsx! { Icon { kind: IconKind::Sparkles, size: 15 } },
                            onclick: move |_| { nav.push(crate::Route::ButtonsPage {}); },
                            "Esplora Componenti"
                        }
                        Button {
                            variant: ButtonVariant::Secondary,
                            left_icon: rsx! { Icon { kind: IconKind::Code, size: 15 } },
                            onclick: move |_| { nav.push(crate::Route::TypographyPage {}); },
                            "Tipografia Geist"
                        }
                    }
                }
            }

            // Mantine-Style Theme Showcase Card
            Card {
                title: "Mantine-Style Theme Engine".to_string(),
                subtitle: "Controllo centralizzato di tonalità colore, raggio di curvatura e micro-glow".to_string(),
                shadow: CardShadow::Lg,
                div { class: "space-y-6",
                    Text { size: TextSize::Sm,
                        "Il motore del tema permette di commutare palette primarie ad alta leggibilità, scalare il raggio di curvatura (Sharp, Medium, Soft) e regolare l'intensità del bagliore ottico su tutti i componenti con architettura pura SCSS nativa."
                    }

                    // Selettore Rapido Preset
                    div { class: "p-5 rounded-xl bg-[#060608] border border-white/[0.14] space-y-4 shadow-inner",
                        div { class: "flex flex-wrap items-center justify-between gap-2",
                            span { class: "text-xs font-mono font-bold text-white uppercase tracking-wider flex items-center gap-1.5",
                                Icon { kind: IconKind::Palette, size: 14 }
                                "Collaudo Reattività Tema (Live Preview)"
                            }
                            span { class: "text-xs font-mono px-2 py-0.5 rounded bg-purple-950/60 text-purple-200 border border-purple-500/30",
                                "Attivo: {theme().name}"
                            }
                        }

                        div { class: "flex flex-wrap items-center gap-2.5",
                            Button {
                                size: ButtonSize::Sm,
                                variant: if theme().primary_color == PrimaryColor::Violet { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                                left_icon: rsx! { span { style: "width: 8px; height: 8px; border-radius: 9999px; display: inline-block; background-color: #a855f7; box-shadow: 0 0 6px #a855f7;" } },
                                onclick: move |_| theme.write().apply_primary_color(PrimaryColor::Violet),
                                "Violet (Default)"
                            }
                            Button {
                                size: ButtonSize::Sm,
                                variant: if theme().primary_color == PrimaryColor::Obsidian { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                                left_icon: rsx! { span { style: "width: 8px; height: 8px; border-radius: 9999px; display: inline-block; background-color: #ffffff; box-shadow: 0 0 6px #ffffff;" } },
                                onclick: move |_| theme.write().apply_primary_color(PrimaryColor::Obsidian),
                                "Obsidian Titanium"
                            }
                            Button {
                                size: ButtonSize::Sm,
                                variant: if theme().primary_color == PrimaryColor::Crimson { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                                left_icon: rsx! { span { style: "width: 8px; height: 8px; border-radius: 9999px; display: inline-block; background-color: #ff1744; box-shadow: 0 0 6px #ff1744;" } },
                                onclick: move |_| theme.write().apply_primary_color(PrimaryColor::Crimson),
                                "Crimson Phoenix"
                            }
                            Button {
                                size: ButtonSize::Sm,
                                variant: if theme().primary_color == PrimaryColor::Cyan { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                                left_icon: rsx! { span { style: "width: 8px; height: 8px; border-radius: 9999px; display: inline-block; background-color: #00f0ff; box-shadow: 0 0 6px #00f0ff;" } },
                                onclick: move |_| theme.write().apply_primary_color(PrimaryColor::Cyan),
                                "Cyber Cyan"
                            }
                            Button {
                                size: ButtonSize::Sm,
                                variant: if theme().primary_color == PrimaryColor::Emerald { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                                left_icon: rsx! { span { style: "width: 8px; height: 8px; border-radius: 9999px; display: inline-block; background-color: #00ff9d; box-shadow: 0 0 6px #00ff9d;" } },
                                onclick: move |_| theme.write().apply_primary_color(PrimaryColor::Emerald),
                                "Emerald Matrix"
                            }
                            Button {
                                size: ButtonSize::Sm,
                                variant: ButtonVariant::Outline,
                                left_icon: rsx! { Icon { kind: IconKind::Settings, size: 13 } },
                                onclick: move |_| { nav.push(crate::Route::ThemeBuilderPage {}); },
                                "Theme Builder Studio"
                            }
                        }
                    }
                }
            }

            // Architecture Grid
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                Card {
                    title: "Libreria (crate/shadow-design)".to_string(),
                    subtitle: "Zero bloat: architettura cargo modulare".to_string(),
                    shadow: CardShadow::Md,
                    p { class: "text-xs text-zinc-300 mb-3 leading-relaxed",
                        "Ogni componente è racchiuso dietro una feature flag Cargo indipendente. Compila esclusivamente il codice che la tua applicazione utilizza."
                    }
                    div { class: "flex flex-wrap gap-1.5",
                        for feat in CARGO_FEATURES {
                            Badge { label: (*feat).to_string(), variant: BadgeVariant::Primary }
                        }
                    }
                }

                Card {
                    title: "Showcase (showcase)".to_string(),
                    subtitle: "Ambiente di collaudo interattivo".to_string(),
                    shadow: CardShadow::Md,
                    p { class: "text-xs text-zinc-300 mb-3 leading-relaxed",
                        "Ogni componente include schede interattive per il collaudo in tempo reale, visualizzatore di codice RSX copiabile con un click e tabella reference dei parametri API."
                    }
                    div { class: "flex flex-wrap gap-2",
                        Badge { label: "Dioxus 0.7".to_string(), variant: BadgeVariant::Success, dot: true }
                        Badge { label: "Geist Fonts".to_string(), variant: BadgeVariant::Default }
                        Badge { label: "Native SVG".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "SCSS Nativo Dioxus".to_string(), variant: BadgeVariant::Default }
                    }
                }
            }

            // Installation & Quick Start
            Card {
                title: "Guida Rapida all'Integrazione".to_string(),
                subtitle: "Come aggiungere Shadow Design ai tuoi progetti Rust Dioxus".to_string(),
                shadow: CardShadow::Lg,
                div { class: "space-y-4",
                    div { class: "space-y-2",
                        span { class: "text-xs font-semibold text-zinc-300", "1. Configura Cargo.toml:" }
                        CodeBlock { code: install_code, language: "toml" }
                    }
                    div { class: "space-y-2 pt-2",
                        span { class: "text-xs font-semibold text-zinc-300", "2. Utilizza i componenti nel tuo codice RSX:" }
                        CodeBlock { code: rust_example, language: "rust" }
                    }
                }
            }
        }
    }
}
