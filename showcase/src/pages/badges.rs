use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn BadgesPage() -> Element {
    let mut selected_variant = use_signal(|| BadgeVariant::Primary);
    let is_pill = use_signal(|| true);
    let has_dot = use_signal(|| true);
    let enable_glow = use_signal(|| false);
    let badge_text = use_signal(|| "Online".to_string());

    let props = vec![
        PropRow {
            name: "label",
            r#type: "String",
            default: "-",
            description: "Testo visualizzato all'interno del badge",
        },
        PropRow {
            name: "variant",
            r#type: "BadgeVariant",
            default: "BadgeVariant::Default",
            description: "Variante semantica (Default, Primary, Success, Warning, Danger, Outline)",
        },
        PropRow {
            name: "pill",
            r#type: "bool",
            default: "false",
            description: "Rende i bordi completamente arrotondati (rounded-full)",
        },
        PropRow {
            name: "dot",
            r#type: "bool",
            default: "false",
            description: "Mostra un indicatore circolare con micro-animazione pulsante",
        },
        PropRow {
            name: "glow",
            r#type: "Option<bool>",
            default: "None",
            description: "Bagliore semantico coordinato con il colore della variante",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS aggiuntive applicate all'elemento <span>",
        },
    ];

    let code_snippet = r#"Badge {
    label: "Attivo".to_string(),
    variant: BadgeVariant::Success,
    pill: true,
    dot: true,
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Badge" }
                Text { dimmed: true, size: TextSize::Sm, "Etichette di stato e indicatori a pillola con supporto a micro-pulsazioni e forme rettangolari o arrotondate." }
            }

            FeatureBanner { feature_name: "badge" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-center",
                        // Live Preview
                        div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex items-center justify-center min-h-[140px] shadow-inner",
                            Badge {
                                label: badge_text(),
                                variant: selected_variant(),
                                pill: is_pill(),
                                dot: has_dot(),
                                glow: if enable_glow() { Some(true) } else { None },
                            }
                        }

                        // Configurator
                        div { class: "space-y-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Testo Etichetta:" }
                                TextInput {
                                    value: badge_text,
                                    placeholder: "Testo badge...".to_string(),
                                }
                            }

                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Variante Semantica:" }
                                div { class: "flex flex-wrap gap-1.5",
                                    for v in [BadgeVariant::Default, BadgeVariant::Primary, BadgeVariant::Success, BadgeVariant::Warning, BadgeVariant::Danger, BadgeVariant::Outline] {
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

                            div { class: "grid grid-cols-2 gap-3 pt-2 border-t border-white/[0.06]",
                                Checkbox {
                                    checked: is_pill,
                                    label: Some("Forma a Pillola".to_string()),
                                    size: CheckboxSize::Sm,
                                }
                                Checkbox {
                                    checked: has_dot,
                                    label: Some("Dot Pulsante".to_string()),
                                    size: CheckboxSize::Sm,
                                }
                                Checkbox {
                                    checked: enable_glow,
                                    label: Some("Micro-Glow Semantico".to_string()),
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
                title: "Varianti Semantiche e Forme".to_string(),
                subtitle: "Anteprima comparativa di tutti gli stati e combinazioni grafiche".to_string(),
                div { class: "space-y-4",
                    div { class: "flex flex-wrap items-center gap-2.5",
                        Badge { label: "Default".to_string(), variant: BadgeVariant::Default }
                        Badge { label: "Primary".to_string(), variant: BadgeVariant::Primary }
                        Badge { label: "Success".to_string(), variant: BadgeVariant::Success }
                        Badge { label: "Warning".to_string(), variant: BadgeVariant::Warning }
                        Badge { label: "Danger".to_string(), variant: BadgeVariant::Danger }
                        Badge { label: "Outline".to_string(), variant: BadgeVariant::Outline }
                    }
                    div { class: "flex flex-wrap items-center gap-2.5 pt-3 border-t border-white/[0.06]",
                        Badge { label: "Offline".to_string(), variant: BadgeVariant::Default, pill: true, dot: true }
                        Badge { label: "Processing".to_string(), variant: BadgeVariant::Primary, pill: true, dot: true }
                        Badge { label: "Operational".to_string(), variant: BadgeVariant::Success, pill: true, dot: true }
                        Badge { label: "Degraded".to_string(), variant: BadgeVariant::Warning, pill: true, dot: true }
                        Badge { label: "Critical".to_string(), variant: BadgeVariant::Danger, pill: true, dot: true }
                    }
                }
            }
        }
    }
}
