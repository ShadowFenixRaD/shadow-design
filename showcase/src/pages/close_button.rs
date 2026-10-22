use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn CloseButtonPage() -> Element {
    let mut selected_size = use_signal(|| CloseButtonSize::Md);
    let mut selected_variant = use_signal(|| CloseButtonVariant::Subtle);
    let mut click_count = use_signal(|| 0usize);

    let props = vec![
        PropRow {
            name: "size",
            r#type: "CloseButtonSize",
            default: "CloseButtonSize::Md",
            description: "Dimensione del pulsante (Sm = 24px, Md = 32px, Lg = 40px)",
        },
        PropRow {
            name: "variant",
            r#type: "CloseButtonVariant",
            default: "CloseButtonVariant::Subtle",
            description: "Variante stilistica (Subtle, Filled, Transparent)",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita il pulsante e blocca le interazioni",
        },
        PropRow {
            name: "aria_label",
            r#type: "String",
            default: "\"Chiudi\"",
            description: "Etichetta di accessibilità per screen reader e tooltip nativo",
        },
    ];

    let code_snippet = r#"CloseButton {
    size: CloseButtonSize::Md,
    variant: CloseButtonVariant::Subtle,
    onclick: move |_| println!("Finestra chiusa"),
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "CloseButton" }
                Text { dimmed: true, size: TextSize::Sm, "Pulsante di chiusura standardizzato con icona 'X' e feedback tattile, coerente su modali, notifiche toast, drawer e schede." }
            }

            FeatureBanner { feature_name: "close_button" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-12 py-16 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center gap-8 shadow-inner min-h-[320px]",
                        div { class: "flex items-center gap-6 flex-wrap justify-center",
                            div { class: "flex items-center gap-2",
                                span { class: "text-xs text-zinc-400 font-mono", "Dimensione:" }
                                button {
                                    r#type: "button",
                                    class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_size.read() == CloseButtonSize::Sm { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| selected_size.set(CloseButtonSize::Sm),
                                    "Sm"
                                }
                                button {
                                    r#type: "button",
                                    class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_size.read() == CloseButtonSize::Md { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| selected_size.set(CloseButtonSize::Md),
                                    "Md"
                                }
                                button {
                                    r#type: "button",
                                    class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_size.read() == CloseButtonSize::Lg { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| selected_size.set(CloseButtonSize::Lg),
                                    "Lg"
                                }
                            }

                            div { class: "flex items-center gap-2",
                                span { class: "text-xs text-zinc-400 font-mono", "Variante:" }
                                button {
                                    r#type: "button",
                                    class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_variant.read() == CloseButtonVariant::Subtle { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| selected_variant.set(CloseButtonVariant::Subtle),
                                    "Subtle"
                                }
                                button {
                                    r#type: "button",
                                    class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_variant.read() == CloseButtonVariant::Filled { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| selected_variant.set(CloseButtonVariant::Filled),
                                    "Filled"
                                }
                                button {
                                    r#type: "button",
                                    class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}", if *selected_variant.read() == CloseButtonVariant::Transparent { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                    onclick: move |_| selected_variant.set(CloseButtonVariant::Transparent),
                                    "Transparent"
                                }
                            }
                        }

                        div { class: "p-6 rounded-2xl bg-white/[0.03] border border-white/[0.08] flex items-center gap-6 shadow-lg",
                            CloseButton {
                                size: *selected_size.read(),
                                variant: *selected_variant.read(),
                                onclick: move |_| click_count += 1,
                            }
                            CloseButton {
                                size: *selected_size.read(),
                                variant: *selected_variant.read(),
                                disabled: true,
                            }
                        }

                        div { class: "flex items-center gap-2 text-xs font-mono text-zinc-400",
                            span { "Click registrati: " }
                            span { class: "text-purple-300 font-bold", "{click_count}" }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
