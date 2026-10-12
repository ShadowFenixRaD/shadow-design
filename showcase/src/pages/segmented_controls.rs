use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn SegmentedControlsPage() -> Element {
    let storage_val = use_signal(|| "sqlite".to_string());
    let crypto_val = use_signal(|| "pq".to_string());
    let mut selected_size = use_signal(|| SegmentedControlSize::Md);
    let mut is_full_width = use_signal(|| false);
    let mut is_disabled = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "options",
            r#type: "Vec<SegmentOption>",
            default: "-",
            description: "Elenco delle opzioni selezionabili con chiave, etichetta e icona opzionale",
        },
        PropRow {
            name: "value",
            r#type: "Signal<String>",
            default: "-",
            description: "Signal reattivo bidirezionale collegato al valore dell'opzione selezionata",
        },
        PropRow {
            name: "size",
            r#type: "SegmentedControlSize",
            default: "SegmentedControlSize::Md",
            description: "Dimensione del controllo (Sm, Md, Lg)",
        },
        PropRow {
            name: "full_width",
            r#type: "bool",
            default: "false",
            description: "Espande il controllo per occupare l'intera larghezza disponibile del contenitore",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita tutte le interazioni sul selettore",
        },
    ];

    let code_snippet = r#"let mut storage_mode = use_signal(|| "sqlite".to_string());

SegmentedControl {
    value: storage_mode,
    options: vec![
        SegmentOption::new("ram", "RAM").with_icon(IconKind::Terminal.into()),
        SegmentOption::new("sqlite", "SQLite").with_icon(IconKind::Server.into()),
        SegmentOption::new("encrypted", "Cifrato").with_icon(IconKind::Lock.into()),
    ],
}"#
    .to_string();

    let storage_options = vec![
        SegmentOption::new("ram", "RAM Volatile").with_icon(IconKind::Terminal.into()),
        SegmentOption::new("sqlite", "SQLite").with_icon(IconKind::Server.into()),
        SegmentOption::new("encrypted", "Cifrato PQ").with_icon(IconKind::Lock.into()),
    ];

    let crypto_options = vec![
        SegmentOption::new("auto", "Auto").with_icon(IconKind::Sparkles.into()),
        SegmentOption::new("pq", "Post-Quantum").with_icon(IconKind::ShieldCheck.into()),
        SegmentOption::new("classic", "Classica").with_icon(IconKind::Key.into()),
        SegmentOption::new("compat", "Compat").with_icon(IconKind::Radio.into()),
    ];

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "SegmentedControl" }
                Text { dimmed: true, size: TextSize::Sm, "Selettore orizzontale a segmenti tattile ad alta precisione con animazione scorrevole e micro-glow del tema attivo." }
            }

            FeatureBanner { feature_name: "segmented_control" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-center",
                        // Live Preview
                        div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center gap-6 min-h-[200px] shadow-inner",
                            div { class: "w-full flex flex-col items-center gap-2",
                                span { class: "text-[11px] font-mono text-zinc-400", "Modalità Storage: [{storage_val}]" }
                                SegmentedControl {
                                    options: storage_options,
                                    value: storage_val,
                                    size: selected_size(),
                                    full_width: is_full_width(),
                                    disabled: is_disabled(),
                                }
                            }

                            div { class: "w-full flex flex-col items-center gap-2 pt-2 border-t border-white/[0.06]",
                                span { class: "text-[11px] font-mono text-zinc-400", "Suite Crittografica: [{crypto_val}]" }
                                SegmentedControl {
                                    options: crypto_options,
                                    value: crypto_val,
                                    size: selected_size(),
                                    full_width: is_full_width(),
                                    disabled: is_disabled(),
                                }
                            }
                        }

                        // Configurator
                        div { class: "space-y-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Dimensione SegmentedControl:" }
                                div { class: "flex gap-2",
                                    for s in [SegmentedControlSize::Sm, SegmentedControlSize::Md, SegmentedControlSize::Lg] {
                                        button {
                                            r#type: "button",
                                            class: if selected_size() == s {
                                                "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                            } else {
                                                "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                            },
                                            onclick: move |_| selected_size.set(s),
                                            match s {
                                                SegmentedControlSize::Sm => "SM",
                                                SegmentedControlSize::Md => "MD",
                                                SegmentedControlSize::Lg => "LG",
                                            }
                                        }
                                    }
                                }
                            }

                            div { class: "pt-2 border-t border-white/[0.06] space-y-2.5",
                                label { class: "flex items-center gap-2 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{is_full_width}",
                                        onchange: move |e| is_full_width.set(e.checked()),
                                    }
                                    "Larghezza Piena (Full Width)"
                                }
                                label { class: "flex items-center gap-2 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{is_disabled}",
                                        onchange: move |e| is_disabled.set(e.checked()),
                                    }
                                    "Disabilitato (Disabled)"
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
