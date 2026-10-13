use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn ProgressPage() -> Element {
    let mut progress_val = use_signal(|| 68.0f32);
    let mut selected_size = use_signal(|| ProgressSize::Md);
    let mut selected_variant = use_signal(|| ProgressVariant::Primary);
    let mut is_indeterminate = use_signal(|| false);
    let mut show_label = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "value",
            r#type: "f32",
            default: "0.0",
            description: "Percentuale di avanzamento da 0.0 a 100.0",
        },
        PropRow {
            name: "size",
            r#type: "ProgressSize",
            default: "ProgressSize::Md",
            description: "Spessore della barra: Sm (4px), Md (8px), Lg (12px)",
        },
        PropRow {
            name: "variant",
            r#type: "ProgressVariant",
            default: "ProgressVariant::Primary",
            description: "Variante cromatica: Primary (neon accent), Success, Warning, Danger",
        },
        PropRow {
            name: "indeterminate",
            r#type: "bool",
            default: "false",
            description: "Attiva l'animazione shimmer continua per caricamenti infiniti o scansioni beacon",
        },
        PropRow {
            name: "show_label",
            r#type: "bool",
            default: "false",
            description: "Mostra automaticamente l'etichetta percentuale (es. '68%')",
        },
    ];

    let code_snippet = r#"ProgressBar {
    value: 68.0,
    size: ProgressSize::Md,
    variant: ProgressVariant::Primary,
    show_label: true,
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Progress & Barra di Avanzamento" }
                Text { dimmed: true, size: TextSize::Sm, "Barra di progressione fluida per upload chunked, trasferimento file P2P, benchmarking latenza e scansioni radio." }
            }

            FeatureBanner { feature_name: "progress" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-center",
                        // Live Preview
                        div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col justify-center min-h-[160px] shadow-inner space-y-4",
                            ProgressBar {
                                value: progress_val(),
                                size: selected_size(),
                                variant: selected_variant(),
                                indeterminate: is_indeterminate(),
                                show_label: show_label(),
                            }
                        }

                        // Configurator
                        div { class: "space-y-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Spessore Barra (Size):" }
                                div { class: "flex gap-2",
                                    for s in [ProgressSize::Sm, ProgressSize::Md, ProgressSize::Lg] {
                                        button {
                                            r#type: "button",
                                            class: if selected_size() == s {
                                                "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                            } else {
                                                "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                            },
                                            onclick: move |_| selected_size.set(s),
                                            match s {
                                                ProgressSize::Sm => "SM (4px)",
                                                ProgressSize::Md => "MD (8px)",
                                                ProgressSize::Lg => "LG (12px)",
                                            }
                                        }
                                    }
                                }
                            }

                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Variante Cromatica:" }
                                div { class: "flex flex-wrap gap-1.5",
                                    for v in [ProgressVariant::Primary, ProgressVariant::Success, ProgressVariant::Warning, ProgressVariant::Danger] {
                                        button {
                                            r#type: "button",
                                            class: if selected_variant() == v {
                                                "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                            } else {
                                                "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                            },
                                            onclick: move |_| selected_variant.set(v),
                                            "{v:?}"
                                        }
                                    }
                                }
                            }

                            div { class: "pt-2 border-t border-white/[0.06] space-y-3",
                                div { class: "space-y-1.5",
                                    div { class: "flex justify-between text-zinc-400 text-[10px]",
                                        span { "VALORE:" }
                                        span { "{progress_val():.0}%" }
                                    }
                                    input {
                                        r#type: "range",
                                        min: "0",
                                        max: "100",
                                        step: "1",
                                        class: "w-full accent-purple-500 cursor-pointer",
                                        value: "{progress_val}",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse::<f32>() {
                                                progress_val.set(v);
                                            }
                                        },
                                    }
                                }

                                div { class: "flex gap-4 pt-1",
                                    label { class: "flex items-center gap-1.5 cursor-pointer text-zinc-300",
                                        input {
                                            r#type: "checkbox",
                                            checked: "{is_indeterminate}",
                                            onchange: move |e| is_indeterminate.set(e.checked()),
                                        }
                                        "Indeterminato (Scan Shimmer)"
                                    }
                                    label { class: "flex items-center gap-1.5 cursor-pointer text-zinc-300",
                                        input {
                                            r#type: "checkbox",
                                            checked: "{show_label}",
                                            onchange: move |e| show_label.set(e.checked()),
                                        }
                                        "Mostra Etichetta %"
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
