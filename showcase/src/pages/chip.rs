use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn ChipPage() -> Element {
    // Standalone chip signals
    let notify_chip = use_signal(|| true);
    let dark_mode_chip = use_signal(|| false);

    // Group signals
    let single_selected = use_signal(|| vec!["rust".to_string()]);
    let multi_selected = use_signal(|| vec!["frontend".to_string(), "backend".to_string()]);

    // Playground state
    let play_checked = use_signal(|| true);
    let mut play_variant = use_signal(|| ChipVariant::Filled);
    let mut play_size = use_signal(|| ControlSize::Md);
    let play_disabled = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "checked",
            r#type: "Option<Signal<bool>>",
            default: "None",
            description: "Signal booleano reattivo per l'uso standalone controllato",
        },
        PropRow {
            name: "value",
            r#type: "Option<String>",
            default: "None",
            description: "Identificatore univoco del chip all'interno di un ChipGroup",
        },
        PropRow {
            name: "variant",
            r#type: "ChipVariant",
            default: "ChipVariant::Filled",
            description: "Variante visiva del componente: Filled, Outline, Light o Subtle",
        },
        PropRow {
            name: "size",
            r#type: "ControlSize",
            default: "ControlSize::Md",
            description: "Dimensione del chip: Sm (26px), Md (32px), Lg (38px)",
        },
        PropRow {
            name: "icon",
            r#type: "Option<IconKind>",
            default: "None",
            description: "Icona vettoriale personalizzata visualizzata prima dell'etichetta",
        },
        PropRow {
            name: "show_check",
            r#type: "bool",
            default: "true",
            description: "Mostra automaticamente l'icona di spunta con micro-animazione quando selezionato",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita le interazioni del chip e ne sfuma l'opacità",
        },
        PropRow {
            name: "on_toggle",
            r#type: "Option<EventHandler<bool>>",
            default: "None",
            description: "Callback invocato ad ogni variazione dello stato di selezione",
        },
    ];

    let code_snippet = r##"// ChipGroup a selezione multipla
let mut tags = use_signal(|| vec!["rust".to_string(), "dioxus".to_string()]);

ChipGroup {
    value: tags,
    multiple: true,
    variant: Some(ChipVariant::Filled),
    size: Some(ControlSize::Md),

    Chip { value: "rust".to_string(), "Rust" }
    Chip { value: "dioxus".to_string(), "Dioxus" }
    Chip { value: "web".to_string(), "WebAssembly" }
}

// Chip Standalone
let mut auto_save = use_signal(|| true);

Chip {
    checked: auto_save,
    variant: ChipVariant::Outline,
    icon: Some(IconKind::Sparkles),
    "Salvataggio Automatico"
}"##
        .to_string();

    let single_sel_str = single_selected.read().join(", ");
    let multi_sel_str = multi_selected.read().join(", ");

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Chip & ChipGroup" }
                Text { dimmed: true, size: TextSize::Sm,
                    "Elementi interattivi compatti a capsula per filtri, tag, etichette e selezioni categoriche. "
                    "Supportano micro-glow neon coerente, animazione di spunta, varianti visive (Filled, Outline, Light, Subtle) "
                    "e raggruppamento singolo o multiplo con ChipGroup."
                }
            }

            FeatureBanner { feature_name: "chip" }

            ShowcaseTabs {
                props: props,
                code: code_snippet,
                preview: rsx! {
                    div { class: "space-y-8",
                        // Live Playground
                        Card {
                            div { class: "p-6 space-y-6",
                                Heading { order: HeadingOrder::H3, "Live Playground" }

                                div { class: "flex flex-wrap gap-4 items-center p-4 bg-zinc-950/60 rounded-lg border border-white/5",
                                    div { class: "flex items-center gap-1.5",
                                        span { class: "text-xs text-zinc-400 mr-2", "Variante:" }
                                        for (v, name) in [
                                            (ChipVariant::Filled, "Filled"),
                                            (ChipVariant::Outline, "Outline"),
                                            (ChipVariant::Light, "Light"),
                                            (ChipVariant::Subtle, "Subtle"),
                                        ] {
                                            button {
                                                r#type: "button",
                                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}",
                                                    if *play_variant.read() == v { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                                onclick: move |_| play_variant.set(v),
                                                "{name}"
                                            }
                                        }
                                    }

                                    div { class: "flex items-center gap-1.5",
                                        span { class: "text-xs text-zinc-400 mr-2", "Dimensione:" }
                                        for (s, name) in [
                                            (ControlSize::Sm, "Sm"),
                                            (ControlSize::Md, "Md"),
                                            (ControlSize::Lg, "Lg"),
                                        ] {
                                            button {
                                                r#type: "button",
                                                class: format!("px-2.5 py-1 text-xs font-mono rounded-lg border transition-all cursor-pointer {}",
                                                    if *play_size.read() == s { "bg-purple-950/80 text-purple-200 border-purple-500/50" } else { "bg-white/[0.04] text-zinc-400 border-white/[0.06]" }),
                                                onclick: move |_| play_size.set(s),
                                                "{name}"
                                            }
                                        }
                                    }

                                    Checkbox {
                                        checked: play_disabled,
                                        label: "Disabilitato".to_string(),
                                        size: CheckboxSize::Sm,
                                    }
                                }

                                div { class: "flex flex-col items-center justify-center p-8 bg-black/40 rounded-xl border border-white/10 gap-4",
                                    div { class: "flex flex-wrap items-center gap-3",
                                        Chip {
                                            checked: play_checked,
                                            variant: *play_variant.read(),
                                            size: *play_size.read(),
                                            disabled: *play_disabled.read(),
                                            icon: Some(IconKind::Sparkles),
                                            "Chip Interattivo"
                                        }
                                    }

                                    div { class: "text-xs text-zinc-400 font-mono mt-2",
                                        "Stato selezionato: "
                                        span { class: "text-purple-400 font-bold", "{play_checked}" }
                                    }
                                }
                            }
                        }

                        // Showcase Group Cases
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                            // Single Select Group
                            Card {
                                div { class: "p-6 space-y-4",
                                    Heading { order: HeadingOrder::H4, "ChipGroup (Selezione Singola)" }
                                    Text { dimmed: true, size: TextSize::Sm,
                                        "Consente la selezione esclusiva di un unico elemento alla volta (radio-style)."
                                    }
                                    div { class: "py-2",
                                        ChipGroup {
                                            value: single_selected,
                                            multiple: false,
                                            variant: Some(ChipVariant::Filled),
                                            Chip { value: "rust".to_string(), "Rust" }
                                            Chip { value: "typescript".to_string(), "TypeScript" }
                                            Chip { value: "python".to_string(), "Python" }
                                            Chip { value: "go".to_string(), "Go" }
                                        }
                                    }
                                    div { class: "text-xs text-zinc-500 font-mono",
                                        "Selezionato: [{single_sel_str}]"
                                    }
                                }
                            }

                            // Multi Select Group
                            Card {
                                div { class: "p-6 space-y-4",
                                    Heading { order: HeadingOrder::H4, "ChipGroup (Selezione Multipla)" }
                                    Text { dimmed: true, size: TextSize::Sm,
                                        "Ideale per filtri di ricerca e preferenze con toggle indipendente."
                                    }
                                    div { class: "py-2",
                                        ChipGroup {
                                            value: multi_selected,
                                            multiple: true,
                                            variant: Some(ChipVariant::Outline),
                                            Chip { value: "frontend".to_string(), icon: Some(IconKind::Terminal), "Frontend" }
                                            Chip { value: "backend".to_string(), icon: Some(IconKind::Box), "Backend" }
                                            Chip { value: "security".to_string(), icon: Some(IconKind::Lock), "Security" }
                                            Chip { value: "ai".to_string(), icon: Some(IconKind::Sparkles), "AI / ML" }
                                        }
                                    }
                                    div { class: "text-xs text-zinc-500 font-mono",
                                        "Selezionati: [{multi_sel_str}]"
                                    }
                                }
                            }

                            // Style Variants Matrix
                            Card {
                                div { class: "p-6 space-y-4",
                                    Heading { order: HeadingOrder::H4, "Varianti di Stile" }
                                    div { class: "flex flex-wrap gap-2.5 pt-2",
                                        Chip { default_checked: true, variant: ChipVariant::Filled, "Filled" }
                                        Chip { default_checked: true, variant: ChipVariant::Outline, "Outline" }
                                        Chip { default_checked: true, variant: ChipVariant::Light, "Light" }
                                        Chip { default_checked: true, variant: ChipVariant::Subtle, "Subtle" }
                                    }
                                }
                            }

                            // Standalone Toggles & Sizing
                            Card {
                                div { class: "p-6 space-y-4",
                                    Heading { order: HeadingOrder::H4, "Scala Dimensionale & Standalone" }
                                    div { class: "flex flex-wrap items-center gap-3 pt-2",
                                        Chip { checked: notify_chip, size: ControlSize::Sm, icon: Some(IconKind::Flame), "Notifiche (Sm)" }
                                        Chip { checked: dark_mode_chip, size: ControlSize::Md, icon: Some(IconKind::Moon), "Dark Mode (Md)" }
                                        Chip { default_checked: true, size: ControlSize::Lg, icon: Some(IconKind::Shield), "Protetto (Lg)" }
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
