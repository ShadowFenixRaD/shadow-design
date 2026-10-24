use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, Counter, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

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
    let play_variant = use_signal(|| ChipVariant::Filled);
    let play_size = use_signal(|| ControlSize::Md);
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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Chip & ChipGroup".to_string(),
                description: "Elementi interattivi compatti a capsula per filtri, tag, etichette e selezioni categoriche. Supportano animazione di spunta, varianti visive (Filled, Outline, Light, Subtle) e raggruppamento singolo o multiplo con ChipGroup.".to_string(),
            }

            FeatureBanner { feature_name: "chip" }

            ShowcaseTabs {
                props: props,
                code: code_snippet,
                preview: rsx! {
                    div { class: "sc-stack sc-stack--loose",
                        Playground {
                            stage: rsx! {
                                Chip {
                                    checked: play_checked,
                                    variant: *play_variant.read(),
                                    size: *play_size.read(),
                                    disabled: *play_disabled.read(),
                                    icon: Some(IconKind::Sparkles),
                                    "Chip Interattivo"
                                }
                                Counter { label: "Stato selezionato".to_string(), value: play_checked().to_string() }
                            },
                            controls: rsx! {
                                ChoicePicker {
                                    label: "Variante".to_string(),
                                    options: vec![
                                        (ChipVariant::Filled, "Filled".to_string()),
                                        (ChipVariant::Outline, "Outline".to_string()),
                                        (ChipVariant::Light, "Light".to_string()),
                                        (ChipVariant::Subtle, "Subtle".to_string()),
                                    ],
                                    value: play_variant,
                                }
                                ChoicePicker {
                                    label: "Dimensione".to_string(),
                                    options: vec![
                                        (ControlSize::Sm, "Sm".to_string()),
                                        (ControlSize::Md, "Md".to_string()),
                                        (ControlSize::Lg, "Lg".to_string()),
                                    ],
                                    value: play_size,
                                }
                                ToggleControl { label: "Disabilitato".to_string(), checked: play_disabled }
                            },
                        }

                        div { class: "sc-card-grid",
                            Card {
                                title: "ChipGroup (Selezione Singola)".to_string(),
                                subtitle: "Consente la selezione esclusiva di un unico elemento alla volta (radio-style).".to_string(),
                                div { class: "sc-stack",
                                    ChipGroup {
                                        value: single_selected,
                                        multiple: false,
                                        variant: Some(ChipVariant::Filled),
                                        Chip { value: "rust".to_string(), "Rust" }
                                        Chip { value: "typescript".to_string(), "TypeScript" }
                                        Chip { value: "python".to_string(), "Python" }
                                        Chip { value: "go".to_string(), "Go" }
                                    }
                                    span { class: "sc-subtle-text sc-mono", "Selezionato: [{single_sel_str}]" }
                                }
                            }

                            Card {
                                title: "ChipGroup (Selezione Multipla)".to_string(),
                                subtitle: "Ideale per filtri di ricerca e preferenze con toggle indipendente.".to_string(),
                                div { class: "sc-stack",
                                    ChipGroup {
                                        value: multi_selected,
                                        multiple: true,
                                        variant: Some(ChipVariant::Outline),
                                        Chip { value: "frontend".to_string(), icon: Some(IconKind::Terminal), "Frontend" }
                                        Chip { value: "backend".to_string(), icon: Some(IconKind::Box), "Backend" }
                                        Chip { value: "security".to_string(), icon: Some(IconKind::Lock), "Security" }
                                        Chip { value: "ai".to_string(), icon: Some(IconKind::Sparkles), "AI / ML" }
                                    }
                                    span { class: "sc-subtle-text sc-mono", "Selezionati: [{multi_sel_str}]" }
                                }
                            }

                            Card { title: "Varianti di Stile".to_string(),
                                div { class: "sc-actions",
                                    Chip { default_checked: true, variant: ChipVariant::Filled, "Filled" }
                                    Chip { default_checked: true, variant: ChipVariant::Outline, "Outline" }
                                    Chip { default_checked: true, variant: ChipVariant::Light, "Light" }
                                    Chip { default_checked: true, variant: ChipVariant::Subtle, "Subtle" }
                                }
                            }

                            Card { title: "Scala Dimensionale & Standalone".to_string(),
                                div { class: "sc-actions",
                                    Chip { checked: notify_chip, size: ControlSize::Sm, icon: Some(IconKind::Flame), "Notifiche (Sm)" }
                                    Chip { checked: dark_mode_chip, size: ControlSize::Md, icon: Some(IconKind::Moon), "Dark Mode (Md)" }
                                    Chip { default_checked: true, size: ControlSize::Lg, icon: Some(IconKind::Shield), "Protetto (Lg)" }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
