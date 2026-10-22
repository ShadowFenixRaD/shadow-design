use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn KeytipPage() -> Element {
    let mut visible = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "key_sequence",
            r#type: "String",
            default: "-",
            description: "Sequenza o combinazione di tasti da mostrare nel badge (es. 'Alt+F', 'Ctrl+S')",
        },
        PropRow {
            name: "visible",
            r#type: "Option<bool>",
            default: "None",
            description: "Controllo esplicito della visibilità del keytip (None = sempre visibile se non disabled)",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Se true, disattiva e nasconde il badge keytip",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Elemento interattivo associato alla scorciatoia",
        },
    ];

    let code_snippet = r#"Keytip {
    key_sequence: "Alt+S".to_string(),
    visible: Some(true),
    Button {
        variant: ButtonVariant::Primary,
        "Salva Progetto"
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Keytip" }
                Text { dimmed: true, size: TextSize::Sm, "Micro-etichette di scorciatoia da tastiera in sovrimpressione su comandi e pulsanti (stile Office Ribbon / IntelliJ)." }
            }

            FeatureBanner { feature_name: "keytip" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-6 shadow-inner",
                        div { class: "flex items-center gap-3",
                            button {
                                r#type: "button",
                                class: format!("px-3 py-1.5 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if visible() { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| visible.toggle(),
                                if visible() { "Nascondi Keytips" } else { "Mostra Keytips" }
                            }
                            span { class: "text-xs font-mono text-zinc-400", "Simula l'attivazione della tastiera" }
                        }

                        div { class: "p-6 rounded-lg bg-white/[0.02] border border-white/[0.06] flex items-center gap-6 flex-wrap justify-center",
                            Keytip {
                                key_sequence: "Alt+F".to_string(),
                                visible: Some(visible()),
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    "File"
                                }
                            }

                            Keytip {
                                key_sequence: "Alt+E".to_string(),
                                visible: Some(visible()),
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    "Modifica"
                                }
                            }

                            Keytip {
                                key_sequence: "Ctrl+S".to_string(),
                                visible: Some(visible()),
                                Button {
                                    variant: ButtonVariant::Primary,
                                    "Salva"
                                }
                            }

                            Keytip {
                                key_sequence: "Ctrl+P".to_string(),
                                visible: Some(visible()),
                                Button {
                                    variant: ButtonVariant::Outline,
                                    "Cerca"
                                }
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
