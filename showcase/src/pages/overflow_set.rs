use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn OverflowSetPage() -> Element {
    let mut last_action = use_signal(|| "Nessuna".to_string());
    let mut limit = use_signal(|| 4usize);

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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "OverflowSet / ResponsiveToolbar" }
                Text { dimmed: true, size: TextSize::Sm, "Barra degli strumenti intelligente che garantisce zero overflow visivo spostando automaticamente le azioni in eccesso in una tendina Altre azioni (•••)." }
            }

            FeatureBanner { feature_name: "overflow_set" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "flex flex-col gap-6 w-full p-6 rounded-2xl bg-zinc-950/70 border border-white/[0.08]",
                        div { class: "flex items-center gap-3 text-xs font-mono text-zinc-400",
                            span { "Pulsanti visibili:" }
                            for val in [2, 3, 4, 5] {
                                {
                                    let is_active = limit() == val;
                                    let btn_cls = if is_active {
                                        "bg-purple-600 text-white border-purple-500 font-bold"
                                    } else {
                                        "bg-white/[0.03] border-white/10 text-zinc-300 hover:bg-white/[0.08]"
                                    };
                                    rsx! {
                                        button {
                                            key: "{val}",
                                            r#type: "button",
                                            class: "px-2.5 py-1 rounded-md border {btn_cls} transition-colors cursor-pointer",
                                            onclick: move |_| limit.set(val),
                                            "{val}"
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "p-4 rounded-xl bg-black/50 border border-white/[0.06] flex items-center justify-between",
                            OverflowSet {
                                items: toolbar_actions,
                                max_visible: Some(limit()),
                                on_action: move |id: String| {
                                    last_action.set(id);
                                },
                            }
                        }

                        div { class: "p-3 rounded-xl bg-white/[0.02] border border-white/[0.06] text-xs font-mono text-zinc-400 flex items-center justify-between",
                            span { "Ultima azione eseguita:" }
                            span { class: "text-purple-400 font-bold", "{last_action()}" }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
