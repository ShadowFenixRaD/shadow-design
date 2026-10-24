use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PageHeader, PropRow, ShowcaseTabs, Stage};

#[component]
pub fn HoverCardPage() -> Element {
    let props = vec![
        PropRow {
            name: "open_delay",
            r#type: "u32",
            default: "150",
            description: "Ritardo in millisecondi prima dell'apertura del card al passaggio del mouse",
        },
        PropRow {
            name: "close_delay",
            r#type: "u32",
            default: "200",
            description: "Ritardo in millisecondi prima della chiusura quando il cursore esce dall'area",
        },
        PropRow {
            name: "width",
            r#type: "Option<u32>",
            default: "None",
            description: "Larghezza personalizzata in pixel del contenitore di anteprima",
        },
        PropRow {
            name: "position",
            r#type: "HoverCardPosition",
            default: "HoverCardPosition::Bottom",
            description: "Posizione di apertura (Bottom, Top, Left, Right)",
        },
    ];

    let code_snippet = r#"HoverCard {
    open_delay: 150,
    close_delay: 200,
    HoverCardTrigger {
        Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "@shadowfenix" }
    }
    HoverCardDropdown {
        div { class: "sc-actions",
            Avatar {
                name: "Shadow Fenix".to_string(),
                size: AvatarSize::Md,
            }
            div { class: "sc-stack",
                div { class: "sc-title-row",
                    h4 { class: "sc-section-title", "Shadowfenix RaD" }
                    Badge { label: "Core Team".to_string(), variant: BadgeVariant::Primary, pill: true }
                }
                p { class: "sc-muted-text", "Ecosistema di componenti ad alte prestazioni per Dioxus 0.7." }
            }
        }
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "HoverCard".to_string(),
                description: "Scheda di preview contestuale ricca attivata al passaggio del cursore (hover) con ritardi configurabili e persistenza su interazione.".to_string(),
            }

            FeatureBanner { feature_name: "hover_card" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage {
                        p { class: "sc-muted-text",
                            "Passa il cursore sulla menzione sottostante per visualizzare l'anteprima del profilo. Il card rimane interattivo consentendo di cliccare pulsanti o link al suo interno."
                        }

                        div { class: "sc-actions",
                            span { class: "sc-muted-text", "Sviluppato con passione da" }
                            HoverCard {
                                open_delay: 100,
                                close_delay: 200,
                                HoverCardTrigger {
                                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "@shadowfenix" }
                                }
                                HoverCardDropdown {
                                    div { class: "sc-actions",
                                        Avatar {
                                            name: "Shadow Fenix".to_string(),
                                            size: AvatarSize::Md,
                                        }
                                        div { class: "sc-stack",
                                            div { class: "sc-title-row",
                                                h4 { class: "sc-section-title", "Shadowfenix RaD" }
                                                Badge { label: "Verified".to_string(), variant: BadgeVariant::Primary, pill: true }
                                            }
                                            p { class: "sc-muted-text",
                                                "Laboratorio di ricerca e sviluppo specializzato in architetture Rust & Dioxus web/desktop."
                                            }
                                            p { class: "sc-subtle-text sc-mono", "50+ componenti • 100% pure SCSS" }
                                        }
                                    }
                                }
                            }
                            span { class: "sc-muted-text", "per la community Rust." }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
