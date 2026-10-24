use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PageHeader, PropRow, ShowcaseTabs, Stage};

#[component]
pub fn DrawerPage() -> Element {
    let mut right_open = use_signal(|| false);
    let mut left_open = use_signal(|| false);
    let gzip_enabled = use_signal(|| true);
    let debug_tracing = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "is_open",
            r#type: "Signal<bool>",
            default: "-",
            description: "Signal reattivo che governa l'apertura e chiusura del pannello",
        },
        PropRow {
            name: "title",
            r#type: "Option<String>",
            default: "None",
            description: "Titolo primario dell'intestazione del pannello laterale",
        },
        PropRow {
            name: "position",
            r#type: "DrawerPosition",
            default: "DrawerPosition::Right",
            description: "Direzione di estrazione laterale (Left, Right)",
        },
        PropRow {
            name: "size",
            r#type: "DrawerSize",
            default: "DrawerSize::Md",
            description: "Larghezza del pannello scorrevole (Sm, Md, Lg)",
        },
    ];

    let code_snippet = r#"let mut is_open = use_signal(|| false);

Button {
    variant: ButtonVariant::Primary,
    onclick: move |_| is_open.set(true),
    "Apri Impostazioni"
}

Drawer {
    is_open: is_open,
    title: "Pannello Configurazione".to_string(),
    position: DrawerPosition::Right,
    size: DrawerSize::Md,
    div { class: "space-y-4",
        p { "Contenuto scorrevole con supporto a form e controlli." }
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Drawer Component".to_string(),
                description: "Pannello scorrevole laterale con backdrop oscurato, chiusura al click esterno e larghezza scalabile.".to_string(),
            }

            FeatureBanner { feature_name: "drawer" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage {
                        Button {
                            variant: ButtonVariant::Primary,
                            left_icon: rsx! { Icon { kind: IconKind::Settings, size: 15 } },
                            onclick: move |_| right_open.set(true),
                            "Apri Drawer Destro (Right)"
                        }
                        Button {
                            variant: ButtonVariant::Secondary,
                            left_icon: rsx! { Icon { kind: IconKind::Menu, size: 15 } },
                            onclick: move |_| left_open.set(true),
                            "Apri Drawer Sinistro (Left)"
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Right Drawer Instance
            Drawer {
                is_open: right_open,
                title: "Impostazioni Sistema".to_string(),
                position: DrawerPosition::Right,
                size: DrawerSize::Md,
                div { class: "sc-stack",
                    p { class: "sc-muted-text", "Configura i parametri del runtime e della connettività cluster." }
                    hr { class: "sc-control-divider" }
                    div { class: "sc-stack",
                        Switch {
                            checked: gzip_enabled,
                            label: "Compressione Gzip Automatica".to_string(),
                            description: "Ottimizza il payload di asset statici".to_string(),
                        }
                        Switch {
                            checked: debug_tracing,
                            label: "Debug Hot Reload Tracing".to_string(),
                            description: "Stampa logs dettagliati nella console".to_string(),
                        }
                    }
                    div { class: "sc-actions",
                        Button {
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Primary,
                            onclick: move |_| right_open.set(false),
                            "Chiudi Pannello"
                        }
                    }
                }
            }

            // Left Drawer Instance
            Drawer {
                is_open: left_open,
                title: "Navigazione Rapida".to_string(),
                position: DrawerPosition::Left,
                size: DrawerSize::Sm,
                div { class: "sc-stack",
                    p { class: "sc-muted-text", "Menu laterale scorrevole ideale per viewport mobile o navigazioni secondarie." }
                    div { class: "sc-stack",
                        Button { variant: ButtonVariant::Ghost, "Dashboard Principale" }
                        Button { variant: ButtonVariant::Ghost, "Metriche Cluster" }
                        Button { variant: ButtonVariant::Ghost, "Documentazione API" }
                    }
                }
            }
        }
    }
}
