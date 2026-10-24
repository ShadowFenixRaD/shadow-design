use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn IndicatorPage() -> Element {
    let selected_pos = use_signal(|| IndicatorPosition::TopEnd);
    let selected_color = use_signal(|| IndicatorColor::Violet);
    let processing = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "position",
            r#type: "IndicatorPosition",
            default: "IndicatorPosition::TopEnd",
            description: "Angolo di ancoraggio (TopEnd, TopStart, BottomEnd, BottomStart)",
        },
        PropRow {
            name: "color",
            r#type: "IndicatorColor",
            default: "IndicatorColor::Violet",
            description: "Colore e alone luminoso (Violet, Emerald, Rose, Amber, Cyan)",
        },
        PropRow {
            name: "size",
            r#type: "IndicatorSize",
            default: "IndicatorSize::Md",
            description: "Dimensione del dot o badge (Sm, Md, Lg)",
        },
        PropRow {
            name: "processing",
            r#type: "bool",
            default: "false",
            description: "Attiva l'animazione di onda radar a impulsi indicando elaborazione attiva",
        },
        PropRow {
            name: "label",
            r#type: "Option<String>",
            default: "None",
            description: "Conteggio o etichetta testuale nel badge (es. \"3\" o \"99+\")",
        },
    ];

    let code_snippet = r#"Indicator {
    position: IndicatorPosition::TopEnd,
    color: IndicatorColor::Violet,
    processing: true,
    Avatar {
        name: "Shadow Fenix".to_string(),
        size: AvatarSize::Md,
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Indicator".to_string(),
                description: "Punto di stato o badge contatore ancorato agli angoli di qualsiasi elemento (avatar, pulsanti, icone) con supporto per onda radar pulsante.".to_string(),
            }

            FeatureBanner { feature_name: "indicator" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            div { class: "sc-actions",
                                Indicator {
                                    position: *selected_pos.read(),
                                    color: IndicatorColor::Emerald,
                                    processing: *processing.read(),
                                    Avatar {
                                        name: "Christian Torbidone".to_string(),
                                        size: AvatarSize::Lg,
                                    }
                                }

                                Indicator {
                                    position: *selected_pos.read(),
                                    color: IndicatorColor::Rose,
                                    label: Some("3".to_string()),
                                    Button {
                                        variant: ButtonVariant::Secondary,
                                        "Messaggi"
                                    }
                                }

                                Indicator {
                                    position: *selected_pos.read(),
                                    color: *selected_color.read(),
                                    processing: *processing.read(),
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        left_icon: rsx! { Icon { kind: IconKind::Info, size: 16 } },
                                        "Notifiche"
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Posizione".to_string(),
                                options: vec![
                                    (IndicatorPosition::TopEnd, "TopEnd".to_string()),
                                    (IndicatorPosition::TopStart, "TopStart".to_string()),
                                    (IndicatorPosition::BottomEnd, "BottomEnd".to_string()),
                                    (IndicatorPosition::BottomStart, "BottomStart".to_string()),
                                ],
                                value: selected_pos,
                            }
                            ChoicePicker {
                                label: "Colore".to_string(),
                                options: vec![
                                    (IndicatorColor::Violet, "Violet".to_string()),
                                    (IndicatorColor::Cyan, "Cyan".to_string()),
                                    (IndicatorColor::Amber, "Amber".to_string()),
                                ],
                                value: selected_color,
                            }
                            ToggleControl { label: "Onda Radar".to_string(), checked: processing }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
