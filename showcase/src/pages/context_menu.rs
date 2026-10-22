use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn ContextMenuPage() -> Element {
    let mut show_grid = use_signal(|| true);
    let mut show_minimap = use_signal(|| false);
    let mut last_action = use_signal(|| "Nessuna azione selezionata".to_string());

    let props = vec![
        PropRow {
            name: "ContextMenuTrigger",
            r#type: "Component",
            default: "-",
            description: "Area interattiva che intercetta il click destro del mouse alle coordinate del puntatore",
        },
        PropRow {
            name: "ContextMenuContent",
            r#type: "Component",
            default: "-",
            description: "Pannello menu ancorato a coordinate fisse con z-index 1000 e chiusura automatica",
        },
        PropRow {
            name: "ContextMenuItem",
            r#type: "Component",
            default: "-",
            description: "Voce d'azione con supporto per icone, scorciatoie da tastiera e variante distruttiva",
        },
        PropRow {
            name: "ContextMenuCheckboxItem",
            r#type: "Component",
            default: "-",
            description: "Voce con casella di controllo booleana e spunta visuale WAI-ARIA",
        },
        PropRow {
            name: "ContextMenuSeparator",
            r#type: "Component",
            default: "-",
            description: "Separatore visivo sottile orizzontale tra gruppi di comandi",
        },
    ];

    let code_snippet = r#"ContextMenu {
    ContextMenuTrigger {
        div { class: "p-12 border border-dashed rounded-xl text-center",
            "Fai click destro qui dentro"
        }
    }
    ContextMenuContent {
        ContextMenuItem {
            label: "Copia".to_string(),
            icon: Some(IconKind::Copy),
            shortcut: Some("Ctrl+C".to_string()),
            onclick: move |_| println!("Copia"),
        }
        ContextMenuItem {
            label: "Taglia".to_string(),
            shortcut: Some("Ctrl+X".to_string()),
        }
        ContextMenuSeparator {}
        ContextMenuCheckboxItem {
            label: "Mostra griglia".to_string(),
            checked: show_grid(),
            onchange: move |val| show_grid.set(val),
        }
        ContextMenuSeparator {}
        ContextMenuItem {
            label: "Elimina".to_string(),
            icon: Some(IconKind::Trash2),
            danger: true,
            shortcut: Some("Del".to_string()),
        }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "ContextMenu" }
                Text { dimmed: true, size: TextSize::Sm, "Menu a tendina contestuale richiamato dal click destro del mouse alle coordinate precise del puntatore, con supporto per scorciatoie, checkbox e azioni distruttive." }
            }

            FeatureBanner { feature_name: "context_menu" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center gap-6 shadow-inner",
                        ContextMenu {
                            ContextMenuTrigger {
                                div { class: "w-full max-w-lg p-16 rounded-2xl border-2 border-dashed border-white/[0.12] hover:border-purple-500/40 bg-white/[0.01] hover:bg-white/[0.03] transition-all flex flex-col items-center justify-center gap-3 cursor-context-menu text-center group",
                                    div { class: "p-3 rounded-full bg-white/[0.04] text-purple-400 group-hover:scale-110 transition-transform",
                                        Icon { kind: IconKind::Terminal, size: 24 }
                                    }
                                    h4 { class: "font-semibold text-sm text-zinc-100", "Area di Test Menu Contestuale" }
                                    p { class: "text-xs text-zinc-400 max-w-xs",
                                        "Fai click con il tasto destro del mouse ovunque all'interno di questo riquadro per aprire il menu Shadow."
                                    }
                                }
                            }
                            ContextMenuContent {
                                ContextMenuItem {
                                    label: "Copia elemento".to_string(),
                                    icon: Some(IconKind::Copy),
                                    shortcut: Some("Ctrl+C".to_string()),
                                    onclick: move |_| last_action.set("Azione: Copia elemento".to_string()),
                                }
                                ContextMenuItem {
                                    label: "Incolla".to_string(),
                                    shortcut: Some("Ctrl+V".to_string()),
                                    onclick: move |_| last_action.set("Azione: Incolla".to_string()),
                                }
                                ContextMenuSeparator {}
                                ContextMenuCheckboxItem {
                                    label: "Mostra griglia isometrica".to_string(),
                                    checked: *show_grid.read(),
                                    shortcut: Some("Ctrl+G".to_string()),
                                    onchange: move |v| {
                                        show_grid.set(v);
                                        last_action.set(format!("Griglia: {}", if v { "Abilitata" } else { "Disabilitata" }));
                                    },
                                }
                                ContextMenuCheckboxItem {
                                    label: "Minimappa codice".to_string(),
                                    checked: *show_minimap.read(),
                                    onchange: move |v| {
                                        show_minimap.set(v);
                                        last_action.set(format!("Minimappa: {}", if v { "Abilitata" } else { "Disabilitata" }));
                                    },
                                }
                                ContextMenuSeparator {}
                                ContextMenuItem {
                                    label: "Impostazioni avanzate".to_string(),
                                    icon: Some(IconKind::Settings),
                                    onclick: move |_| last_action.set("Azione: Impostazioni".to_string()),
                                }
                                ContextMenuItem {
                                    label: "Elimina elemento".to_string(),
                                    icon: Some(IconKind::Trash2),
                                    danger: true,
                                    shortcut: Some("Del".to_string()),
                                    onclick: move |_| last_action.set("Azione: Elimina elemento permanente".to_string()),
                                }
                            }
                        }

                        div { class: "p-3 rounded-lg bg-white/[0.02] border border-white/[0.06] flex items-center gap-2 text-xs font-mono text-zinc-400",
                            Icon { kind: IconKind::CheckCircle, size: 14 }
                            span { "Ultima azione: " }
                            span { class: "text-purple-300 font-semibold", "{last_action}" }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
