use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs,
};

#[component]
pub fn CenterPage() -> Element {
    let inline = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "inline",
            r#type: "bool",
            default: "false",
            description: "Se impostato a true, utilizza display: inline-flex anziché display: flex.",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS aggiuntive applicate al contenitore Center.",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Elementi figli posizionati ed allineati esattamente al centro.",
        },
    ];

    let code_snippet = r#"Center {
    class: "sc-demo-frame h-48 w-full",
    div { class: "flex flex-col items-center gap-2",
        Icon { kind: IconKind::Check, size: 28 }
        span { class: "sc-section-title", "Allineato al centro" }
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Center".to_string(),
                description: "Contenitore utility per centrare istantaneamente il contenuto sia in orizzontale che in verticale senza codice CSS boilerplate, con supporto per layout a blocco o inline.".to_string(),
            }

            FeatureBanner { feature_name: "center" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            if inline() {
                                div { class: "sc-demo-frame text-center",
                                    span { class: "sc-muted-text", "Testo prima dell'elemento " }
                                    Center { inline: true, class: "gap-2",
                                        Icon { kind: IconKind::Layers, size: 16 }
                                        span { class: "sc-accent-text sc-mono", "Inline Centered" }
                                    }
                                    span { class: "sc-muted-text", " Testo dopo l'elemento" }
                                }
                            } else {
                                Center { class: "sc-demo-frame h-56 w-full",
                                    div { class: "sc-stack items-center text-center",
                                        Icon { kind: IconKind::Check, size: 28 }
                                        span { class: "sc-section-title sc-mono", "Perfettamente Centrato" }
                                        span { class: "sc-subtle-text sc-mono", "display: flex; justify-center; items-center;" }
                                        div { class: "sc-actions",
                                            Badge { label: "H-Center".to_string(), variant: BadgeVariant::Primary }
                                            Badge { label: "V-Center".to_string(), variant: BadgeVariant::Success }
                                        }
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Modalità Display".to_string(),
                                options: vec![
                                    (false, "Block (display: flex)".to_string()),
                                    (true, "Inline (display: inline-flex)".to_string()),
                                ],
                                value: inline,
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
