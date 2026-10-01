use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn ModalsPage() -> Element {
    let mut is_modal_open = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "is_open",
            r#type: "Signal<bool>",
            default: "-",
            description: "Signal reattivo che controlla l'apertura e chiusura del dialogo",
        },
        PropRow {
            name: "title",
            r#type: "Option<String>",
            default: "None",
            description: "Titolo primario visualizzato nell'intestazione della finestra modale",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi Tailwind personalizzate per il corpo del dialog",
        },
    ];

    let code_snippet = r#"let mut is_open = use_signal(|| false);

Button {
    variant: ButtonVariant::Primary,
    onclick: move |_| is_open.set(true),
    "Apri Dialog"
}

Modal {
    is_open: is_open,
    title: "Conferma Operazione".to_string(),
    div { class: "space-y-4",
        p { "Sei sicuro di voler procedere con l'eliminazione?" }
        div { class: "flex justify-end gap-2",
            Button {
                variant: ButtonVariant::Secondary,
                onclick: move |_| is_open.set(false),
                "Annulla"
            }
            Button {
                variant: ButtonVariant::Danger,
                onclick: move |_| is_open.set(false),
                "Elimina"
            }
        }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Modal" }
                Text { dimmed: true, size: TextSize::Sm, "Finestre di dialogo in overlay con backdrop oscurato e sfocato (backdrop-blur), chiusura su click esterno e icona X vettoriale." }
            }

            FeatureBanner { feature_name: "modal" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center gap-4 text-center shadow-inner",
                        p { class: "text-xs text-zinc-400 max-w-sm",
                            "Clicca sul pulsante sottostante per aprire la finestra di dialogo. Puoi chiuderla cliccando sulla X, premendo il pulsante 'Chiudi' o cliccando sul backdrop esterno."
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            left_icon: rsx! { Icon { kind: IconKind::Eye, size: 15 } },
                            onclick: move |_| is_modal_open.set(true),
                            "Apri Finestra Modale"
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            Modal {
                is_open: is_modal_open,
                title: "Finestra di Dialogo Shadow Design".to_string(),
                div { class: "space-y-4 text-xs sm:text-sm text-zinc-300",
                    p { class: "leading-relaxed",
                        "Questa finestra modale utilizza l'architettura a 4 livelli con elevazione "
                        strong { class: "text-purple-300 font-mono", "Surface Elevated (Level 2)" }
                        ", rifinitura con top specular highlight a 1px e chiusura intelligente con blocco della propagazione degli eventi."
                    }
                    div { class: "p-3 rounded-lg bg-[#060608] border border-white/[0.06] flex items-center gap-2 text-xs font-mono text-zinc-400",
                        Icon { kind: IconKind::Sparkles, size: 14 }
                        span { "Colore e raggio si adattano in tempo reale al tema attivo." }
                    }
                    div { class: "flex justify-end gap-2.5 pt-3 border-t border-white/[0.06]",
                        Button {
                            variant: ButtonVariant::Secondary,
                            size: ButtonSize::Sm,
                            onclick: move |_| is_modal_open.set(false),
                            "Annulla"
                        }
                        Button {
                            variant: ButtonVariant::Primary,
                            size: ButtonSize::Sm,
                            onclick: move |_| is_modal_open.set(false),
                            "Conferma Azione"
                        }
                    }
                }
            }
        }
    }
}
