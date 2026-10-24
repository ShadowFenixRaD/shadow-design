use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{Counter, FeatureBanner, PageHeader, Stage, PropRow, ShowcaseTabs};

#[component]
pub fn MarqueeSelectionPage() -> Element {
    let mut selected_indices = use_signal(|| vec![0, 1]);
    let mut last_coords = use_signal(|| "In attesa di trascinamento".to_string());

    let props = vec![
        PropRow {
            name: "enabled",
            r#type: "bool",
            default: "true",
            description: "Se true, abilita il trascinamento del riquadro elastico di selezione",
        },
        PropRow {
            name: "on_change",
            r#type: "Option<EventHandler<MarqueeRect>>",
            default: "None",
            description: "Callback scatenato al termine del trascinamento con le coordinate x, y, width, height",
        },
        PropRow {
            name: "on_select_indices",
            r#type: "Option<EventHandler<Vec<usize>>>",
            default: "None",
            description: "Restituisce automaticamente gli indici dei figli con attributo data-marquee-item intersecati dal lasso",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Elementi e card contenute all'interno dell'area di selezione",
        },
    ];

    let items = [
        "mesh_router.rs",
        "peer_protocol.proto",
        "crypto_vault.bin",
        "shadow_config.json",
        "pipeline.yaml",
        "kernel_patch.diff",
    ];

    let code_snippet = r#"MarqueeSelection {
    enabled: true,
    on_change: move |rect: MarqueeRect| {
        println!("Rettangolo calcolato: {:?}", rect);
    },
    on_select_indices: move |indices: Vec<usize>| {
        selected_indices.set(indices);
    },
    div { class: "grid grid-cols-3 gap-3",
        for (i, item) in items.iter().enumerate() {
            div {
                "data-marquee-item": "{i}",
                // Stile selezionato / non selezionato...
            }
        }
    }
}"#
    .to_string();

    let sel_count = selected_indices().len();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "MarqueeSelection".to_string(),
                description: "Riquadro rettangolare a trascinamento elastico (Lasso / Rubber Band) con collision detection integrata per selezionare blocchi di file, card o nodi.".to_string(),
            }

            FeatureBanner { feature_name: "marquee_selection" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage { start: true, wide: true,
                        div { class: "sc-stack w-full",
                            div { class: "sc-title-row",
                                div { class: "sc-actions",
                                    Counter { label: "Elementi selezionati".to_string(), value: sel_count.to_string() }
                                    if sel_count > 0 {
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            size: ButtonSize::Sm,
                                            onclick: move |_| selected_indices.set(vec![]),
                                            "Deseleziona tutto"
                                        }
                                    }
                                }
                                span { class: "sc-mono sc-muted-text", "{last_coords()}" }
                            }

                            div { class: "sc-demo-frame",
                                MarqueeSelection {
                                    enabled: true,
                                    on_change: move |rect: MarqueeRect| {
                                        last_coords.set(format!("x: {:.0}, y: {:.0}, w: {:.0}, h: {:.0}", rect.x, rect.y, rect.width, rect.height));
                                    },
                                    on_select_indices: move |indices: Vec<usize>| {
                                        selected_indices.set(indices);
                                    },
                                    div { class: "grid grid-cols-2 sm:grid-cols-3 gap-3",
                                        for (i, item) in items.iter().enumerate() {
                                            {
                                                let is_sel = selected_indices().contains(&i);
                                                let sel_class = if is_sel { "sd-marquee-item--selected" } else { "" };
                                                rsx! {
                                                    div {
                                                        key: "{i}",
                                                        "data-marquee-item": "{i}",
                                                        class: "sc-demo-cell {sel_class}",
                                                        onclick: move |_| {
                                                            let mut current = selected_indices();
                                                            if current.contains(&i) {
                                                                current.retain(|&x| x != i);
                                                            } else {
                                                                current.push(i);
                                                            }
                                                            selected_indices.set(current);
                                                        },
                                                        span { class: "pointer-events-none", "{item}" }
                                                    }
                                                }
                                            }
                                        }
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
