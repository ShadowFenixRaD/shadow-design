use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn BurgerPage() -> Element {
    let opened = use_signal(|| false);
    let mut size = use_signal(|| BurgerSize::Md);

    let props = vec![
        PropRow {
            name: "opened",
            r#type: "Signal<bool>",
            default: "-",
            description: "Signal reattivo che controlla lo stato aperto (icona a X) o chiuso (hamburger a 3 righe)",
        },
        PropRow {
            name: "size",
            r#type: "BurgerSize",
            default: "BurgerSize::Md",
            description: "Dimensioni del pulsante (Sm = 18px, Md = 24px, Lg = 32px)",
        },
        PropRow {
            name: "color",
            r#type: "Option<String>",
            default: "None",
            description: "Colore personalizzato per le linee del burger",
        },
        PropRow {
            name: "aria_label",
            r#type: "String",
            default: "\"Toggle navigation\"",
            description: "Etichetta accessibile per screen reader",
        },
    ];

    let code_snippet = r#"let mut opened = use_signal(|| false);

Burger {
    opened: opened,
    size: BurgerSize::Md,
    aria_label: "Menu di navigazione".to_string(),
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Burger" }
                Text { dimmed: true, size: TextSize::Sm, "Pulsante hamburger animato a 3 linee con transizione ottica fluida in icona di chiusura 'X', ideale per trigger di navigazione mobile e sidebar." }
            }

            FeatureBanner { feature_name: "burger" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-12 py-20 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center gap-8 shadow-inner min-h-[300px]",
                        div { class: "flex items-center gap-2 flex-wrap justify-center",
                            span { class: "text-xs text-zinc-400 mr-2 font-mono", "Dimensione:" }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *size.read() == BurgerSize::Sm { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| size.set(BurgerSize::Sm),
                                "Sm (18px)"
                            }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *size.read() == BurgerSize::Md { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| size.set(BurgerSize::Md),
                                "Md (24px)"
                            }
                            button {
                                r#type: "button",
                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *size.read() == BurgerSize::Lg { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                onclick: move |_| size.set(BurgerSize::Lg),
                                "Lg (32px)"
                            }
                        }

                        div { class: "p-6 rounded-2xl bg-white/[0.03] border border-white/[0.08] flex items-center justify-center shadow-lg",
                            Burger {
                                opened: opened,
                                size: *size.read(),
                            }
                        }

                        div { class: "flex items-center gap-2 text-xs font-mono text-zinc-400",
                            span { "Stato corrente: " }
                            span {
                                class: if *opened.read() { "text-purple-300 font-bold" } else { "text-zinc-500" },
                                if *opened.read() { "Aperto (X)" } else { "Chiuso (Hamburger)" }
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
