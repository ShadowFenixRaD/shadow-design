use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn RatingPage() -> Element {
    let score = use_signal(|| 4.0);
    let half_score = use_signal(|| 3.5);
    let flame_score = use_signal(|| 5.0);

    let mut star_count = use_signal(|| 5usize);
    let mut fractions_val = use_signal(|| 1usize);
    let is_readonly = use_signal(|| false);
    let is_disabled = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "value",
            r#type: "Signal<f64>",
            default: "-",
            description: "Signal reattivo bidirezionale collegato al punteggio di valutazione corrente",
        },
        PropRow {
            name: "count",
            r#type: "usize",
            default: "5",
            description: "Numero totale di stelle o icone visualizzate nella scala",
        },
        PropRow {
            name: "fractions",
            r#type: "usize",
            default: "1",
            description: "Frazioni consentite: 1 per stelle intere (1, 2, 3), 2 per mezze stelle (0.5, 1.0, 1.5)",
        },
        PropRow {
            name: "readonly",
            r#type: "bool",
            default: "false",
            description: "Impedisce le modifiche dell'utente visualizzando la valutazione in sola lettura",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita le interazioni opacizzando il componente",
        },
        PropRow {
            name: "icon",
            r#type: "Option<IconKind>",
            default: "None",
            description: "Icona vettoriale personalizzata da utilizzare al posto della classica stella a 5 punte",
        },
        PropRow {
            name: "size",
            r#type: "u32",
            default: "22",
            description: "Dimensione in pixel di ciascuna stella o icona",
        },
    ];

    let code_snippet = r#"let mut score = use_signal(|| 4.0);
let mut half_score = use_signal(|| 3.5);

// Stelle Intere Standard
Rating {
    value: score,
    count: 5,
    fractions: 1,
}

// Mezze Stelle (Precisione 0.5)
Rating {
    value: half_score,
    count: 5,
    fractions: 2,
}

// Icona Personalizzata (Flame)
Rating {
    value: score,
    icon: Some(IconKind::Flame),
    size: 24,
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Rating" }
                Text { dimmed: true, size: TextSize::Sm, "Componente di valutazione a stelle o icone vettoriali con supporto per mezze valutazioni (precisione 0.5), anteprima hover dinamica e navigazione da tastiera (ArrowLeft / ArrowRight)." }
            }

            FeatureBanner { feature_name: "rating" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-start",
                        // Live Preview
                        div { class: "p-6 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-6 shadow-inner",
                            div { class: "flex flex-col gap-2",
                                span { class: "text-xs font-mono text-zinc-400", "Valutazione Standard: [{score}]" }
                                Rating {
                                    value: score,
                                    count: star_count(),
                                    fractions: fractions_val(),
                                    readonly: is_readonly(),
                                    disabled: is_disabled(),
                                }
                            }

                            div { class: "pt-4 border-t border-white/[0.06] flex flex-col gap-2",
                                span { class: "text-xs font-mono text-zinc-400", "Mezze Stelle (Frazioni 0.5): [{half_score}]" }
                                Rating {
                                    value: half_score,
                                    count: 5,
                                    fractions: 2,
                                    readonly: is_readonly(),
                                    disabled: is_disabled(),
                                }
                            }

                            div { class: "pt-4 border-t border-white/[0.06] flex flex-col gap-2",
                                span { class: "text-xs font-mono text-zinc-400", "Icone Custom (Flame): [{flame_score}]" }
                                Rating {
                                    value: flame_score,
                                    count: 5,
                                    icon: Some(IconKind::Flame),
                                    size: 24,
                                    readonly: is_readonly(),
                                    disabled: is_disabled(),
                                }
                            }
                        }

                        // Configurator
                        div { class: "space-y-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Numero Stelle:" }
                                div { class: "flex gap-2",
                                    for c in [5, 7, 10] {
                                        button {
                                            r#type: "button",
                                            class: if star_count() == c {
                                                "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                            } else {
                                                "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                            },
                                            onclick: move |_| star_count.set(c),
                                            "{c} stelle"
                                        }
                                    }
                                }
                            }

                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Frazioni:" }
                                div { class: "flex gap-2",
                                    button {
                                        r#type: "button",
                                        class: if fractions_val() == 1 {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| fractions_val.set(1),
                                        "Intere (1.0)"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if fractions_val() == 2 {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| fractions_val.set(2),
                                        "Mezze (0.5)"
                                    }
                                }
                            }

                            div { class: "pt-2 flex flex-col gap-2 border-t border-white/[0.06]",
                                Checkbox {
                                    checked: is_readonly,
                                    label: "Sola Lettura (Readonly)".to_string(),
                                    size: CheckboxSize::Sm,
                                }
                                Checkbox {
                                    checked: is_disabled,
                                    label: "Disabilitato".to_string(),
                                    size: CheckboxSize::Sm,
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
