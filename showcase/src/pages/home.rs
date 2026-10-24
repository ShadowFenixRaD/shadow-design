use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::nav::all_cargo_features;
use crate::components::CodeBlock;

#[component]
pub fn Home() -> Element {
    let nav = use_navigator();
    let cargo_features = all_cargo_features();

    let install_code = format!(
        r#"# Solo i moduli che usi (zero bloat)
[dependencies.shadow-design]
version = "{VERSION}"
default-features = false
features = ["button", "card", "theme", "icon"]

# Oppure l'intera suite:
# shadow-design = {{ version = "{VERSION}", features = ["all"] }}"#
    );

    let rust_example = r#"use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        // Includi il foglio di stile SCSS nativo
        document::Link { rel: "stylesheet", href: shadow_design::STYLES }

        Card {
            title: "Pannello operativo".to_string(),
            p { "Interfaccia dark mode pronta all'uso." }
            Button {
                variant: ButtonVariant::Primary,
                left_icon: rsx! { Icon { kind: IconKind::Check, size: 15 } },
                "Salva modifiche"
            }
        }
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            section { class: "sc-hero",
                div { class: "sc-actions",
                    Badge {
                        label: "SHADOWFENIX ECOSYSTEM".to_string(),
                        variant: BadgeVariant::Default,
                        pill: true,
                        dot: true,
                    }
                    span { class: "sc-counter", "DIOXUS 0.7 · LIT GLASS" }
                }
                h1 { class: "sc-hero__title", "Shadow Design" }
                p { class: "sc-page-desc",
                    "Libreria di componenti UI modulare ad alte prestazioni per Dioxus 0.7. Un solo linguaggio visivo: vetro nero illuminato dall'alto, superfici a quattro livelli, bordi hairline, un accento che compare solo dove qualcosa è attivo e zero dipendenze esterne."
                }
                div { class: "sc-actions",
                    Button {
                        variant: ButtonVariant::Primary,
                        left_icon: rsx! {
                            Icon { kind: IconKind::Sparkles, size: 15 }
                        },
                        onclick: move |_| {
                            nav.push(crate::Route::ButtonsPage {});
                        },
                        "Esplora i componenti"
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        left_icon: rsx! {
                            Icon { kind: IconKind::Palette, size: 15 }
                        },
                        onclick: move |_| {
                            nav.push(crate::Route::ThemeBuilderPage {});
                        },
                        "Theme Builder"
                    }
                }
            }

            div { class: "sc-card-grid",
                Card {
                    title: "Libreria".to_string(),
                    subtitle: "Zero bloat: architettura Cargo modulare".to_string(),
                    div { class: "sc-stack",
                        p { class: "sc-muted-text",
                            "Ogni componente è dietro una feature flag Cargo indipendente. Compili solo il codice che la tua applicazione usa."
                        }
                        div { class: "sc-actions",
                            for feat in &cargo_features {
                                Badge { key: "{feat}", label: (*feat).to_string(), variant: BadgeVariant::Default }
                            }
                        }
                    }
                }

                Card {
                    title: "Showcase".to_string(),
                    subtitle: "Ambiente di collaudo interattivo".to_string(),
                    div { class: "sc-stack",
                        p { class: "sc-muted-text",
                            "Ogni componente ha un playground in tempo reale, il codice RSX copiabile con un click e la tabella dei parametri API."
                        }
                        div { class: "sc-actions",
                            Badge { label: "Dioxus 0.7".to_string(), variant: BadgeVariant::Success, dot: true }
                            Badge { label: "Kanit + Onest".to_string(), variant: BadgeVariant::Default }
                            Badge { label: "Native SVG".to_string(), variant: BadgeVariant::Default }
                            Badge { label: "SCSS nativo Dioxus".to_string(), variant: BadgeVariant::Default }
                        }
                    }
                }
            }

            Card {
                title: "Guida rapida".to_string(),
                subtitle: "Come aggiungere Shadow Design ai tuoi progetti Dioxus".to_string(),
                div { class: "sc-stack",
                    span { class: "sc-control-label", "1. Configura Cargo.toml" }
                    CodeBlock { code: install_code, language: "toml" }
                    span { class: "sc-control-label", "2. Usa i componenti nel tuo RSX" }
                    CodeBlock { code: rust_example, language: "rust" }
                }
            }
        }
    }
}
