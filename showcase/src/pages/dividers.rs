use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn DividersPage() -> Element {
    let mut orientation = use_signal(|| Orientation::Horizontal);
    let mut with_label = use_signal(|| true);
    let mut label_text = use_signal(|| "Oggi • Messaggi Crittografati".to_string());

    let props = vec![
        PropRow {
            name: "orientation",
            r#type: "Orientation",
            default: "Orientation::Horizontal",
            description: "Orientamento del separatore: Horizontal o Vertical",
        },
        PropRow {
            name: "label",
            r#type: "Option<String>",
            default: "None",
            description: "Etichetta testuale opzionale posizionata al centro del divisore (es. 'Oggi', 'Ieri')",
        },
    ];

    let code_snippet = r#"Divider {
    orientation: Orientation::Horizontal,
    label: Some("Oggi • 12 Ottobre".to_string()),
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Divider & Separatore" }
                Text { dimmed: true, size: TextSize::Sm, "Separatore semitrasparente hairline per suddividere sezioni, blocchi temporali o date all'interno di chat e form." }
            }

            FeatureBanner { feature_name: "divider" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-center",
                        // Live Preview
                        div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex items-center justify-center min-h-[160px] shadow-inner",
                            if orientation() == Orientation::Horizontal {
                                div { class: "w-full space-y-3",
                                    div { class: "text-xs text-zinc-400 font-mono", "Messaggio precedente (Ieri 23:40)" }
                                    Divider {
                                        orientation: Orientation::Horizontal,
                                        label: if with_label() { Some(label_text()) } else { None },
                                    }
                                    div { class: "text-xs text-zinc-400 font-mono", "Messaggio recente (Oggi 14:28)" }
                                }
                            } else {
                                div { class: "flex items-center h-28 gap-4 text-xs font-mono text-zinc-400",
                                    span { "Pannello Sinistro" }
                                    Divider {
                                        orientation: Orientation::Vertical,
                                        label: if with_label() { Some("OR".to_string()) } else { None },
                                    }
                                    span { "Pannello Destro" }
                                }
                            }
                        }

                        // Configurator
                        div { class: "space-y-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Orientamento:" }
                                div { class: "flex gap-2",
                                    button {
                                        r#type: "button",
                                        class: if orientation() == Orientation::Horizontal {
                                            "px-3 py-1.5 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-3 py-1.5 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| orientation.set(Orientation::Horizontal),
                                        "Orizzontale (Horizontal)"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if orientation() == Orientation::Vertical {
                                            "px-3 py-1.5 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-3 py-1.5 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| orientation.set(Orientation::Vertical),
                                        "Verticale (Vertical)"
                                    }
                                }
                            }

                            div { class: "pt-2 border-t border-white/[0.06] space-y-2.5",
                                label { class: "flex items-center gap-2 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{with_label}",
                                        onchange: move |e| with_label.set(e.checked()),
                                    }
                                    "Mostra Etichetta Testuale Centrata"
                                }

                                if with_label() {
                                    div { class: "space-y-1",
                                        span { class: "text-zinc-400 block text-[10px]", "Testo Etichetta:" }
                                        input {
                                            r#type: "text",
                                            class: "w-full px-2.5 py-1.5 rounded bg-black/60 border border-white/[0.1] text-zinc-200 focus:outline-none focus:border-purple-500",
                                            value: "{label_text}",
                                            oninput: move |e| label_text.set(e.value()),
                                        }
                                    }
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
