use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn DrawerPage() -> Element {
    let mut right_open = use_signal(|| false);
    let mut left_open = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "is_open",
            r#type: "Signal<bool>",
            default: "-",
            description: "Signal reattivo che governa l'apertura e chiusura del pannello",
        },
        PropRow {
            name: "title",
            r#type: "Option<String>",
            default: "None",
            description: "Titolo primario dell'intestazione del pannello laterale",
        },
        PropRow {
            name: "position",
            r#type: "DrawerPosition",
            default: "DrawerPosition::Right",
            description: "Direzione di estrazione laterale (Left, Right)",
        },
        PropRow {
            name: "size",
            r#type: "DrawerSize",
            default: "DrawerSize::Md",
            description: "Larghezza del pannello scorrevole (Sm, Md, Lg)",
        },
    ];

    let code_snippet = r#"let mut is_open = use_signal(|| false);

Button {
    variant: ButtonVariant::Primary,
    onclick: move |_| is_open.set(true),
    "Apri Impostazioni"
}

Drawer {
    is_open: is_open,
    title: "Pannello Configurazione".to_string(),
    position: DrawerPosition::Right,
    size: DrawerSize::Md,
    div { class: "space-y-4",
        p { "Contenuto scorrevole con supporto a form e controlli." }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Drawer Component" }
                Text { dimmed: true, size: TextSize::Sm, "Pannello scorrevole laterale con backdrop oscurato, chiusura al click esterno e larghezza scalabile." }
            }

            FeatureBanner { feature_name: "drawer" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] shadow-inner flex flex-wrap items-center justify-center gap-4",
                        Button {
                            variant: ButtonVariant::Primary,
                            left_icon: rsx! { Icon { kind: IconKind::Settings, size: 15 } },
                            onclick: move |_| right_open.set(true),
                            "Apri Drawer Destro (Right)"
                        }
                        Button {
                            variant: ButtonVariant::Secondary,
                            left_icon: rsx! { Icon { kind: IconKind::Menu, size: 15 } },
                            onclick: move |_| left_open.set(true),
                            "Apri Drawer Sinistro (Left)"
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Right Drawer Instance
            Drawer {
                is_open: right_open,
                title: "Impostazioni Sistema".to_string(),
                position: DrawerPosition::Right,
                size: DrawerSize::Md,
                div { class: "space-y-6 text-xs text-zinc-300",
                    p { "Configura i parametri del runtime e della connettività cluster." }
                    div { class: "space-y-4 pt-3 border-t border-white/[0.06]",
                        Switch {
                            checked: use_signal(|| true),
                            label: "Compressione Gzip Automatica".to_string(),
                            description: "Ottimizza il payload di asset statici".to_string(),
                        }
                        Switch {
                            checked: use_signal(|| false),
                            label: "Debug Hot Reload Tracing".to_string(),
                            description: "Stampa logs dettagliati nella console".to_string(),
                        }
                    }
                    div { class: "pt-4 border-t border-white/[0.06] flex justify-end gap-2",
                        Button {
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Primary,
                            onclick: move |_| right_open.set(false),
                            "Chiudi Pannello"
                        }
                    }
                }
            }

            // Left Drawer Instance
            Drawer {
                is_open: left_open,
                title: "Navigazione Rapida".to_string(),
                position: DrawerPosition::Left,
                size: DrawerSize::Sm,
                div { class: "space-y-4 text-xs text-zinc-300",
                    p { "Menu laterale scorrevole ideale per viewport mobile o navigazioni secondarie." }
                    div { class: "flex flex-col gap-1.5 pt-2",
                        Button { variant: ButtonVariant::Ghost, "Dashboard Principale" }
                        Button { variant: ButtonVariant::Ghost, "Metriche Cluster" }
                        Button { variant: ButtonVariant::Ghost, "Documentazione API" }
                    }
                }
            }
        }
    }
}
