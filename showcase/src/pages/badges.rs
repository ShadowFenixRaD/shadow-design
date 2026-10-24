use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, ControlGroup, ControlRow, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn BadgesPage() -> Element {
    let selected_variant = use_signal(|| BadgeVariant::Primary);
    let is_pill = use_signal(|| true);
    let has_dot = use_signal(|| true);
    let enable_glow = use_signal(|| false);
    let badge_text = use_signal(|| "Online".to_string());

    let props = vec![
        PropRow {
            name: "label",
            r#type: "String",
            default: "-",
            description: "Testo visualizzato all'interno del badge",
        },
        PropRow {
            name: "variant",
            r#type: "BadgeVariant",
            default: "BadgeVariant::Default",
            description: "Variante semantica (Default, Primary, Success, Warning, Danger, Outline)",
        },
        PropRow {
            name: "pill",
            r#type: "bool",
            default: "false",
            description: "Rende i bordi completamente arrotondati (rounded-full)",
        },
        PropRow {
            name: "dot",
            r#type: "bool",
            default: "false",
            description: "Mostra un indicatore circolare con micro-animazione pulsante",
        },
        PropRow {
            name: "glow",
            r#type: "Option<bool>",
            default: "None",
            description: "Bagliore semantico coordinato con il colore della variante",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS aggiuntive applicate all'elemento <span>",
        },
    ];

    let code_snippet = r#"Badge {
    label: "Attivo".to_string(),
    variant: BadgeVariant::Success,
    pill: true,
    dot: true,
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Badge".to_string(),
                description: "Etichette di stato e indicatori a pillola con supporto a micro-pulsazioni e forme rettangolari o arrotondate.".to_string(),
            }

            FeatureBanner { feature_name: "badge" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            Badge {
                                label: badge_text(),
                                variant: selected_variant(),
                                pill: is_pill(),
                                dot: has_dot(),
                                glow: if enable_glow() { Some(true) } else { None },
                            }
                        },
                        controls: rsx! {
                            ControlGroup { label: "Testo Etichetta".to_string(),
                                TextInput {
                                    value: badge_text,
                                    placeholder: "Testo badge...".to_string(),
                                }
                            }
                            ChoicePicker {
                                label: "Variante Semantica".to_string(),
                                options: vec![
                                    (BadgeVariant::Default, "Default".to_string()),
                                    (BadgeVariant::Primary, "Primary".to_string()),
                                    (BadgeVariant::Success, "Success".to_string()),
                                    (BadgeVariant::Warning, "Warning".to_string()),
                                    (BadgeVariant::Danger, "Danger".to_string()),
                                    (BadgeVariant::Outline, "Outline".to_string()),
                                ],
                                value: selected_variant,
                            }
                            hr { class: "sc-control-divider" }
                            ControlRow {
                                ToggleControl { label: "Forma a Pillola".to_string(), checked: is_pill }
                                ToggleControl { label: "Dot Pulsante".to_string(), checked: has_dot }
                                ToggleControl { label: "Micro-Glow Semantico".to_string(), checked: enable_glow }
                            }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Visual Catalog
            Card {
                title: "Varianti Semantiche e Forme".to_string(),
                subtitle: "Anteprima comparativa di tutti gli stati e combinazioni grafiche".to_string(),
                div { class: "sc-stack",
                    div { class: "sc-actions",
                        Badge { label: "Default".to_string(), variant: BadgeVariant::Default }
                        Badge { label: "Primary".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "Success".to_string(), variant: BadgeVariant::Success }
                        Badge { label: "Warning".to_string(), variant: BadgeVariant::Warning }
                        Badge { label: "Danger".to_string(), variant: BadgeVariant::Danger }
                        Badge { label: "Outline".to_string(), variant: BadgeVariant::Outline }
                    }
                    hr { class: "sc-control-divider" }
                    div { class: "sc-actions",
                        Badge { label: "Offline".to_string(), variant: BadgeVariant::Default, pill: true, dot: true }
                        Badge { label: "Processing".to_string(), variant: BadgeVariant::Primary, pill: true, dot: true }
                        Badge { label: "Operational".to_string(), variant: BadgeVariant::Success, pill: true, dot: true }
                        Badge { label: "Degraded".to_string(), variant: BadgeVariant::Warning, pill: true, dot: true }
                        Badge { label: "Critical".to_string(), variant: BadgeVariant::Danger, pill: true, dot: true }
                    }
                }
            }
        }
    }
}
