use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{ChoicePicker, Counter, Playground, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn OverflowSetPage() -> Element {
    let mut last_action = use_signal(|| "Nessuna".to_string());
    let limit = use_signal(|| 4usize);

    let toolbar_actions = vec![
        ToolbarAction::new("new", "Nuovo File").with_icon(IconKind::Plus),
        ToolbarAction::new("edit", "Modifica").with_icon(IconKind::Sparkles),
        ToolbarAction::new("copy", "Copia").with_icon(IconKind::Copy),
        ToolbarAction::new("share", "Condividi").with_icon(IconKind::Share2),
        ToolbarAction::new("download", "Download").with_icon(IconKind::Download),
        ToolbarAction::new("trash", "Elimina").with_icon(IconKind::Trash2).with_danger(true),
    ];

    let props = vec![
        PropRow {
            name: "items",
            r#type: "Vec<ToolbarAction>",
            default: "-",
            description: "Elenco completo delle azioni disponibili nella barra degli strumenti",
        },
        PropRow {
            name: "max_visible",
            r#type: "Option<usize>",
            default: "Some(4)",
            description: "Numero massimo di pulsanti visibili prima di compattare in tendina",
        },
        PropRow {
            name: "overflow_button_label",
            r#type: "String",
            default: "\"Altre azioni\"",
            description: "Tooltip e accessibilità per il pulsante con tre puntini",
        },
        PropRow {
            name: "on_action",
            r#type: "Option<EventHandler<String>>",
            default: "None",
            description: "Callback invocata con l'ID dell'azione cliccata (visibile o nel menu)",
        },
    ];

    let code_snippet = r#"ResponsiveToolbar {
    items: toolbar_actions,
    max_visible: Some(4),
    on_action: move |id| println!("Azione: {id}"),
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "OverflowSet / ResponsiveToolbar".to_string(),
                description: "Barra degli strumenti intelligente che garantisce zero overflow visivo spostando automaticamente le azioni in eccesso in una tendina Altre azioni (•••).".to_string(),
            }

            FeatureBanner { feature_name: "overflow_set" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            div { class: "sc-stack w-full",
                                OverflowSet {
                                    items: toolbar_actions,
                                    max_visible: Some(limit()),
                                    on_action: move |id: String| {
                                        last_action.set(id);
                                    },
                                }
                                Counter { label: "Ultima azione eseguita".to_string(), value: last_action() }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Pulsanti visibili".to_string(),
                                options: vec![
                                    (2usize, "2".to_string()),
                                    (3usize, "3".to_string()),
                                    (4usize, "4".to_string()),
                                    (5usize, "5".to_string()),
                                ],
                                value: limit,
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
