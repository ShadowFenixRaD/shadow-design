use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn ButtonsPage() -> Element {
    let mut click_count = use_signal(|| 0);
    let mut selected_variant = use_signal(|| ButtonVariant::Primary);
    let mut selected_size = use_signal(|| ButtonSize::Md);
    let is_loading = use_signal(|| false);
    let is_disabled = use_signal(|| false);
    let show_left_icon = use_signal(|| true);
    let show_right_icon = use_signal(|| false);
    let is_full_width = use_signal(|| false);
    let enable_glow = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "variant",
            r#type: "ButtonVariant",
            default: "ButtonVariant::Primary",
            description: "Stile cromatico (Primary, Secondary, Outline, Ghost, Danger, Success)",
        },
        PropRow {
            name: "size",
            r#type: "ButtonSize",
            default: "ButtonSize::Md",
            description: "Scala dimensionale (Xs, Sm, Md, Lg, Xl)",
        },
        PropRow {
            name: "loading",
            r#type: "bool",
            default: "false",
            description: "Mostra uno spinner SVG rotante e imposta aria-busy e aria-disabled",
        },
        PropRow {
            name: "loading_label",
            r#type: "Option<String>",
            default: "None",
            description: "Testo opzionale per lettori di schermo durante lo stato di caricamento",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita il pulsante e riduce l'opacità con aria-disabled",
        },
        PropRow {
            name: "full_width",
            r#type: "bool",
            default: "false",
            description: "Espande il pulsante al 100% della larghezza del contenitore",
        },
        PropRow {
            name: "glow",
            r#type: "Option<bool>",
            default: "None",
            description: "Attiva o disattiva esplicitamente il micro-glow perimetrale",
        },
        PropRow {
            name: "left_icon",
            r#type: "Option<Element>",
            default: "None",
            description: "Elemento icona visualizzato prima del testo (con aria-hidden)",
        },
        PropRow {
            name: "right_icon",
            r#type: "Option<Element>",
            default: "None",
            description: "Elemento icona visualizzato dopo il testo (con aria-hidden)",
        },
        PropRow {
            name: "onclick",
            r#type: "Option<EventHandler<MouseEvent>>",
            default: "None",
            description: "Callback scatenata al click o all'attivazione da tastiera",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS aggiuntive applicate all'elemento <button>",
        },
    ];

    let code_snippet = r#"Button {
    variant: ButtonVariant::Primary,
    size: ButtonSize::Md,
    left_icon: rsx! { Icon { kind: IconKind::Sparkles, size: 16 } },
    onclick: move |_| println!("Pulsante premuto!"),
    "Azione Principale"
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Button" }
                Text { dimmed: true, size: TextSize::Sm, "Pulsanti tattili con feedback ottico alla pressione, supporto a icone native, stato loading accessibile e raggio scalabile da tema." }
            }

            FeatureBanner { feature_name: "button" }

            // Configurator Playground Tabs
            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-center",
                        // Preview Box
                        div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center min-h-[180px] shadow-inner w-full",
                            div { class: if is_full_width() { "w-full" } else { "" },
                                Button {
                                    variant: selected_variant(),
                                    size: selected_size(),
                                    loading: is_loading(),
                                    disabled: is_disabled(),
                                    full_width: is_full_width(),
                                    glow: if enable_glow() { Some(true) } else { None },
                                    left_icon: if show_left_icon() {
                                        Some(rsx! { Icon { kind: IconKind::Sparkles, size: 15 } })
                                    } else {
                                        None
                                    },
                                    right_icon: if show_right_icon() {
                                        Some(rsx! { Icon { kind: IconKind::ChevronRight, size: 15 } })
                                    } else {
                                        None
                                    },
                                    onclick: move |_| *click_count.write() += 1,
                                    "Pulsante Interattivo"
                                }
                            }
                            span { class: "text-xs font-mono text-zinc-500 mt-4",
                                "Click registrati: "
                                strong { class: "text-purple-300", "{click_count}" }
                            }
                        }

                        // Controls Box
                        div { class: "space-y-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Variante Cromatica:" }
                                div { class: "flex flex-wrap gap-1.5",
                                    for v in [ButtonVariant::Primary, ButtonVariant::Secondary, ButtonVariant::Outline, ButtonVariant::Ghost, ButtonVariant::Success, ButtonVariant::Danger] {
                                        button {
                                            key: "{v:?}",
                                            r#type: "button",
                                            class: if selected_variant() == v {
                                                "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer shadow-sm"
                                            } else {
                                                "px-2.5 py-1 rounded bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors"
                                            },
                                            onclick: move |_| selected_variant.set(v),
                                            "{v:?}"
                                        }
                                    }
                                }
                            }

                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Dimensione (Size):" }
                                div { class: "flex flex-wrap gap-1.5",
                                    for s in [ButtonSize::Xs, ButtonSize::Sm, ButtonSize::Md, ButtonSize::Lg, ButtonSize::Xl] {
                                        button {
                                            key: "{s:?}",
                                            r#type: "button",
                                            class: if selected_size() == s {
                                                "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer shadow-sm"
                                            } else {
                                                "px-2.5 py-1 rounded bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors"
                                            },
                                            onclick: move |_| selected_size.set(s),
                                            "{s:?}"
                                        }
                                    }
                                }
                            }

                            div { class: "pt-2 border-t border-white/[0.06] grid grid-cols-2 gap-2.5",
                                Checkbox {
                                    checked: is_loading,
                                    label: Some("Stato Loading".to_string()),
                                    size: CheckboxSize::Sm,
                                }
                                Checkbox {
                                    checked: is_disabled,
                                    label: Some("Disabilitato".to_string()),
                                    size: CheckboxSize::Sm,
                                }
                                Checkbox {
                                    checked: show_left_icon,
                                    label: Some("Icona Sinistra".to_string()),
                                    size: CheckboxSize::Sm,
                                }
                                Checkbox {
                                    checked: show_right_icon,
                                    label: Some("Icona Destra".to_string()),
                                    size: CheckboxSize::Sm,
                                }
                                Checkbox {
                                    checked: is_full_width,
                                    label: Some("Larghezza 100%".to_string()),
                                    size: CheckboxSize::Sm,
                                }
                                Checkbox {
                                    checked: enable_glow,
                                    label: Some("Attiva Glow".to_string()),
                                    size: CheckboxSize::Sm,
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Visual Catalog
            Card {
                title: "Catalogo Dimensioni e Varianti".to_string(),
                subtitle: "Tutte le scale dimensionali con feedback tattile".to_string(),
                div { class: "space-y-6",
                    div { class: "flex flex-wrap items-center gap-3",
                        Button { size: ButtonSize::Xs, variant: ButtonVariant::Primary, "Extra Small" }
                        Button { size: ButtonSize::Sm, variant: ButtonVariant::Primary, "Small" }
                        Button { size: ButtonSize::Md, variant: ButtonVariant::Primary, "Medium" }
                        Button { size: ButtonSize::Lg, variant: ButtonVariant::Primary, "Large" }
                        Button { size: ButtonSize::Xl, variant: ButtonVariant::Primary, "Extra Large" }
                    }
                    div { class: "flex flex-wrap items-center gap-3 pt-4 border-t border-white/[0.06]",
                        Button { variant: ButtonVariant::Primary, "Primary" }
                        Button { variant: ButtonVariant::Secondary, "Secondary" }
                        Button { variant: ButtonVariant::Outline, "Outline" }
                        Button { variant: ButtonVariant::Ghost, "Ghost" }
                        Button { variant: ButtonVariant::Success, "Success" }
                        Button { variant: ButtonVariant::Danger, "Danger" }
                    }
                }
            }
        }
    }
}
