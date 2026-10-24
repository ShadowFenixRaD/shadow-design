use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{Stage, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn TooltipPage() -> Element {
    let props = vec![
        PropRow {
            name: "content",
            r#type: "String",
            default: "-",
            description: "Testo visualizzato all'interno del fumetto fluttuante del tooltip",
        },
        PropRow {
            name: "position",
            r#type: "TooltipPosition",
            default: "TooltipPosition::Top",
            description:
                "Posizione di ancoraggio rispetto all'elemento figlio (Top, Bottom, Left, Right)",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Elemento target che attiva il tooltip al passaggio del cursore",
        },
    ];

    let code_snippet = r#"Tooltip {
    content: "Copia comando negli appunti".to_string(),
    position: TooltipPosition::Top,
    Button {
        variant: ButtonVariant::Secondary,
        left_icon: rsx! { Icon { kind: IconKind::Copy, size: 14 } },
        "Copia"
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Tooltip Component".to_string(),
                description: "Micro-etichette fluttuanti con animazione di dissolvenza, posizionamento a 4 direzioni e rifinitura specular-top.".to_string(),
            }

            FeatureBanner { feature_name: "tooltip" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage {
                        Tooltip {
                            content: "Tooltip ancorato in alto (Top)".to_string(),
                            position: TooltipPosition::Top,
                            Button {
                                variant: ButtonVariant::Secondary,
                                "Hover Top"
                            }
                        }
                        Tooltip {
                            content: "Tooltip ancorato in basso (Bottom)".to_string(),
                            position: TooltipPosition::Bottom,
                            Button {
                                variant: ButtonVariant::Secondary,
                                "Hover Bottom"
                            }
                        }
                        Tooltip {
                            content: "Tooltip ancorato a sinistra (Left)".to_string(),
                            position: TooltipPosition::Left,
                            Button {
                                variant: ButtonVariant::Secondary,
                                "Hover Left"
                            }
                        }
                        Tooltip {
                            content: "Tooltip ancorato a destra (Right)".to_string(),
                            position: TooltipPosition::Right,
                            Button {
                                variant: ButtonVariant::Secondary,
                                "Hover Right"
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
