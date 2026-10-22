use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn FieldsetPage() -> Element {
    let mut variant = use_signal(|| FieldsetVariant::Default);
    let mut disabled = use_signal(|| false);
    let node_name = use_signal(|| "node-shadow-01".to_string());
    let proto = use_signal(|| "tls-v1.3-strict".to_string());

    let props = vec![
        PropRow {
            name: "legend",
            r#type: "String",
            default: "-",
            description: "Titolo primario inserito nell'elemento <legend>",
        },
        PropRow {
            name: "description",
            r#type: "Option<String>",
            default: "None",
            description: "Testo opzionale visualizzato direttamente sotto la legenda",
        },
        PropRow {
            name: "variant",
            r#type: "FieldsetVariant",
            default: "FieldsetVariant::Default",
            description: "Stile grafico del contenitore (Default, Filled, Ungrouped)",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Se true, disabilita il contenitore e a cascata tutti i controlli nativi contenuti",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Controlli o campi form contenuti nel gruppo logico",
        },
    ];

    let code_snippet = r#"Fieldset {
    legend: "Credenziali Crittografiche".to_string(),
    description: Some("Chiavi pubbliche e token di sicurezza.".to_string()),
    variant: FieldsetVariant::Default,
    disabled: false,
    div { class: "space-y-3",
        TextInput { placeholder: "Chiave SSH Ed25519" }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Fieldset" }
                Text { dimmed: true, size: TextSize::Sm, "Raggruppa semanticamente controlli correlati all'interno di un modulo con legenda integrata e supporto allo stato disabled a cascata." }
            }

            FeatureBanner { feature_name: "fieldset" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-6 shadow-inner",
                        div { class: "flex items-center gap-3 flex-wrap",
                            span { class: "text-xs text-zinc-400 font-mono", "Variante:" }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if variant() == FieldsetVariant::Default { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| variant.set(FieldsetVariant::Default),
                                "Default"
                            }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if variant() == FieldsetVariant::Filled { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| variant.set(FieldsetVariant::Filled),
                                "Filled"
                            }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if variant() == FieldsetVariant::Ungrouped { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| variant.set(FieldsetVariant::Ungrouped),
                                "Ungrouped"
                            }
                            div { class: "ml-auto",
                                button {
                                    r#type: "button",
                                    class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if disabled() { "bg-rose-950/80 text-rose-200 border-rose-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| disabled.toggle(),
                                    if disabled() { "Disabilitato" } else { "Abilitato" }
                                }
                            }
                        }

                        div { class: "max-w-md mx-auto w-full",
                            Fieldset {
                                legend: "Cluster Security Profile".to_string(),
                                description: Some("Definisci i parametri di cifratura per il nodo periferico.".to_string()),
                                variant: variant(),
                                disabled: disabled(),
                                div { class: "space-y-4 pt-2",
                                    TextInput {
                                        value: node_name,
                                        placeholder: "Hostname del nodo",
                                    }
                                    TextInput {
                                        value: proto,
                                        placeholder: "Protocollo",
                                    }
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        size: ButtonSize::Sm,
                                        "Salva Configurazione"
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
