use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, FeatureBanner, PageHeader, Playground, ToggleControl, PropRow, ShowcaseTabs};

#[component]
pub fn LoadingOverlayPage() -> Element {
    let visible = use_signal(|| true);
    let blur_px = use_signal(|| 4u32);

    let props = vec![
        PropRow {
            name: "visible",
            r#type: "bool",
            default: "-",
            description: "Se true, attiva l'overlay con spinner e blur bloccando l'interazione sul contenitore",
        },
        PropRow {
            name: "blur_px",
            r#type: "u32",
            default: "3",
            description: "Raggio di sfocatura in pixel per l'effetto backdrop-filter",
        },
        PropRow {
            name: "loader_size",
            r#type: "u32",
            default: "32",
            description: "Dimensione dell'icona animata dello spinner in pixel",
        },
        PropRow {
            name: "description",
            r#type: "Option<String>",
            default: "None",
            description: "Testo opzionale visualizzato al di sotto dello spinner di caricamento",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Contenuto isolato o form da proteggere durante il caricamento",
        },
    ];

    let code_snippet = r#"LoadingOverlay {
    visible: is_loading(),
    blur_px: 3,
    description: Some("Elaborazione dati...".to_string()),
    div { class: "p-6",
        p { "Contenuto protetto" }
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "LoadingOverlay".to_string(),
                description: "Strato semitrasparente in overlay con spinner circolare animato e sfocatura ottica per isolare e disabilitare sezioni durante operazioni asincrone.".to_string(),
            }

            FeatureBanner { feature_name: "loading_overlay" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            div { class: "w-full",
                                LoadingOverlay {
                                    visible: visible(),
                                    blur_px: blur_px(),
                                    description: Some("Sincronizzazione archivio...".to_string()),
                                    Card { title: "Pannello Operativo".to_string(),
                                        div { class: "sc-stack",
                                            p { class: "sc-page-desc", "Questo modulo include parametri e pulsanti la cui interazione viene inibita mentre l'overlay è visibile." }
                                            div { class: "sc-actions",
                                                Button { variant: ButtonVariant::Primary, size: ButtonSize::Sm, "Invia Task" }
                                                Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, "Annulla" }
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ToggleControl { label: "Overlay attivo".to_string(), checked: visible }
                            ChoicePicker {
                                label: "Sfocatura (px)".to_string(),
                                options: vec![
                                    (0u32, "0".to_string()),
                                    (2u32, "2".to_string()),
                                    (4u32, "4".to_string()),
                                    (6u32, "6".to_string()),
                                ],
                                value: blur_px,
                            }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
