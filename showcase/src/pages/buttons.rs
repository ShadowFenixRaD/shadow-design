use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn ButtonsPage() -> Element {
    let mut click_count = use_signal(|| 0);
    let mut selected_variant = use_signal(|| ButtonVariant::Primary);
    let mut selected_size = use_signal(|| ButtonSize::Md);
    let mut is_loading = use_signal(|| false);
    let mut is_disabled = use_signal(|| false);
    let mut show_icon = use_signal(|| true);

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
            description: "Mostra uno spinner SVG rotante e disabilita l'interazione",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita il pulsante e riduce l'opacità",
        },
        PropRow {
            name: "full_width",
            r#type: "bool",
            default: "false",
            description: "Espande il pulsante al 100% della larghezza del contenitore",
        },
        PropRow {
            name: "left_icon",
            r#type: "Option<Element>",
            default: "None",
            description: "Elemento icona visualizzato prima del testo",
        },
        PropRow {
            name: "right_icon",
            r#type: "Option<Element>",
            default: "None",
            description: "Elemento icona visualizzato dopo il testo",
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
                Text { dimmed: true, size: TextSize::Sm, "Pulsanti tattili con feedback ottico alla pressione, supporto a icone native, stato loading e raggio scalabile da tema." }
            }

            FeatureBanner { feature_name: "button" }

            // Configurator Playground Tabs
            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-center",
                        // Preview Box
                        div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center min-h-[180px] shadow-inner",
                            Button {
                                variant: selected_variant(),
                                size: selected_size(),
                                loading: is_loading(),
                                disabled: is_disabled(),
                                left_icon: if show_icon() {
                                    Some(rsx! { Icon { kind: IconKind::Sparkles, size: 15 } })
                                } else {
                                    None
                                },
                                onclick: move |_| *click_count.write() += 1,
                                "Pulsante Interattivo"
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
                                            r#type: "button",
                                            class: if selected_variant() == v {
                                                "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer shadow-sm"
                                            } else {
                                                "px-2 py-1 rounded bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors"
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
                                            r#type: "button",
                                            class: if selected_size() == s {
                                                "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer shadow-sm"
                                            } else {
                                                "px-2 py-1 rounded bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors"
                                            },
                                            onclick: move |_| selected_size.set(s),
                                            "{s:?}"
                                        }
                                    }
                                }
                            }

                            div { class: "pt-2 border-t border-white/[0.06] flex flex-wrap gap-3",
                                label { class: "flex items-center gap-1.5 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{is_loading}",
                                        onchange: move |e| is_loading.set(e.checked()),
                                    }
                                    "Stato Loading"
                                }
                                label { class: "flex items-center gap-1.5 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{is_disabled}",
                                        onchange: move |e| is_disabled.set(e.checked()),
                                    }
                                    "Disabilitato"
                                }
                                label { class: "flex items-center gap-1.5 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{show_icon}",
                                        onchange: move |e| show_icon.set(e.checked()),
                                    }
                                    "Mostra Icona"
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
                title: "Catalogo Completo delle Varianti".to_string(),
                subtitle: "Tutte le opzioni cromatiche predefinite con specular highlights".to_string(),
                div { class: "flex flex-wrap items-center gap-3",
                    Button { variant: ButtonVariant::Primary, "Primary" }
                    Button { variant: ButtonVariant::Secondary, "Secondary" }
                    Button { variant: ButtonVariant::Outline, "Outline" }
                    Button { variant: ButtonVariant::Ghost, "Ghost" }
                    Button { variant: ButtonVariant::Success, "Success" }
                    Button { variant: ButtonVariant::Danger, "Danger" }
                    Button { disabled: true, "Disabled" }
                }
            }

            Card {
                title: "Pulsanti con Icone e Loading State".to_string(),
                subtitle: "Icone vettoriali integrate a 1.5px stroke e spinner animato".to_string(),
                div { class: "flex flex-wrap items-center gap-4",
                    Button {
                        variant: ButtonVariant::Primary,
                        left_icon: rsx! { Icon { kind: IconKind::Check, size: 15 } },
                        "Conferma"
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        right_icon: rsx! { Icon { kind: IconKind::ExternalLink, size: 14 } },
                        "Apri Documentazione"
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        loading: true,
                        "Salvataggio in corso..."
                    }
                    Button {
                        variant: ButtonVariant::Danger,
                        left_icon: rsx! { Icon { kind: IconKind::X, size: 14 } },
                        "Elimina Risorsa"
                    }
                }
            }
        }
    }
}
