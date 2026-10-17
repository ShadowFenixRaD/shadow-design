use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn BreadcrumbsPage() -> Element {
    let mut separator_choice = use_signal(|| 0); // 0: Chevron, 1: Slash, 2: Dot, 3: Arrow
    let mut max_items_choice = use_signal(|| Some(3));
    let with_icons = use_signal(|| true);

    let items_data = vec![
        BreadcrumbData::new("Root")
            .with_href("#/dashboard")
            .with_icon(SvgIcon::Kind(IconKind::Box)),
        BreadcrumbData::new("Infrastruttura")
            .with_href("#/infrastructure")
            .with_icon(SvgIcon::Kind(IconKind::Server)),
        BreadcrumbData::new("Nodi Mesh P2P")
            .with_href("#/mesh-nodes")
            .with_icon(SvgIcon::Kind(IconKind::Wifi)),
        BreadcrumbData::new("Sicurezza & Cifratura")
            .with_href("#/security")
            .with_icon(SvgIcon::Kind(IconKind::ShieldCheck)),
        BreadcrumbData::new("Audit Log #42").with_icon(SvgIcon::Kind(IconKind::Lock)),
    ];

    let items_without_icons = items_data
        .iter()
        .map(|it| {
            let mut b = BreadcrumbData::new(&it.label);
            if let Some(ref h) = it.href {
                b = b.with_href(h);
            }
            b
        })
        .collect::<Vec<_>>();

    let props = vec![
        PropRow {
            name: "items",
            r#type: "Option<Vec<BreadcrumbData>>",
            default: "None",
            description: "Lista dichiarativa degli elementi di navigazione (supporta ellissi automatica)",
        },
        PropRow {
            name: "max_items",
            r#type: "Option<usize>",
            default: "None",
            description: "Soglia massima di elementi visibili oltre la quale il percorso viene compresso con ellissi '...'",
        },
        PropRow {
            name: "items_before_collapse",
            r#type: "usize",
            default: "1",
            description: "Numero di elementi da preservare all'inizio prima dell'ellissi",
        },
        PropRow {
            name: "items_after_collapse",
            r#type: "usize",
            default: "1",
            description: "Numero di elementi da preservare alla fine dopo l'ellissi",
        },
        PropRow {
            name: "separator / separator_text",
            r#type: "Option<Element> / Option<String>",
            default: "IconKind::ChevronRight",
            description: "Separatore grafico personalizzato (es. chevron SVG nativo, slash '/', '•')",
        },
    ];

    let code_snippet = r#"Breadcrumbs {
    max_items: Some(3),
    items: vec![
        BreadcrumbData::new("Root").with_href("/"),
        BreadcrumbData::new("Cluster").with_href("/cluster"),
        BreadcrumbData::new("Mesh P2P").with_href("/mesh"),
        BreadcrumbData::new("Audit Log #42"),
    ],
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Breadcrumbs" }
                Text { dimmed: true, size: TextSize::Sm, "Navigazione a briciole di pane gerarchica con separatori personalizzabili, icone di contesto e collasso intelligente con ellissi per percorsi lunghi." }
            }

            FeatureBanner { feature_name: "breadcrumbs" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6",
                        // Toolbar di controllo interattivo
                        div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] grid grid-cols-1 md:grid-cols-3 gap-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Separatore:" }
                                div { class: "grid grid-cols-2 gap-1.5",
                                    button {
                                        r#type: "button",
                                        class: if separator_choice() == 0 {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| separator_choice.set(0),
                                        "Chevron (›)"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if separator_choice() == 1 {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| separator_choice.set(1),
                                        "Slash (/)"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if separator_choice() == 2 {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| separator_choice.set(2),
                                        "Dot (•)"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if separator_choice() == 3 {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| separator_choice.set(3),
                                        "Freccia (→)"
                                    }
                                }
                            }

                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Compressione Max Items:" }
                                div { class: "flex gap-2",
                                    button {
                                        r#type: "button",
                                        class: if max_items_choice().is_none() {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| max_items_choice.set(None),
                                        "Tutti (5)"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if max_items_choice() == Some(3) {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| max_items_choice.set(Some(3)),
                                        "Max 3 (con ...)"
                                    }
                                }
                            }

                            div { class: "flex items-center pt-2 md:pt-0 border-t md:border-t-0 md:border-l border-white/[0.06] md:pl-4",
                                Checkbox {
                                    checked: with_icons,
                                    label: Some("Includi Icone Vettoriali".to_string()),
                                }
                            }
                        }

                        // Preview box principale
                        div { class: "p-8 rounded-xl bg-[#030304] border border-white/[0.08] shadow-2xl space-y-6",
                            div { class: "text-[11px] font-mono text-zinc-400 mb-2 flex items-center justify-between",
                                span { "NAVIGATORE GERARCHICO REATTIVO" }
                                span { class: "text-purple-400/80", "Sep: {separator_choice()} | Max: {max_items_choice():?}" }
                            }

                            div { class: "p-4 rounded-lg bg-zinc-950/80 border border-white/[0.06]",
                                Breadcrumbs {
                                    items: if with_icons() { items_data.clone() } else { items_without_icons.clone() },
                                    max_items: max_items_choice(),
                                    separator_text: match separator_choice() {
                                        1 => Some("/".to_string()),
                                        2 => Some("•".to_string()),
                                        3 => Some("→".to_string()),
                                        _ => None,
                                    },
                                }
                            }

                            // Dimostrazione compositiva diretta (BreadcrumbItem manuali)
                            div { class: "pt-4 border-t border-white/[0.06] space-y-3",
                                div { class: "text-xs font-semibold text-zinc-400 uppercase tracking-wider text-[10px]", "Composizione Diretta (RSX Manuale):" }
                                Breadcrumbs {
                                    ol { class: "sd-breadcrumbs__list",
                                        BreadcrumbItem { href: "#/", "Home" }
                                        li { class: "sd-breadcrumbs__separator", "›" }
                                        BreadcrumbItem { href: "#/settings", "Impostazioni" }
                                        li { class: "sd-breadcrumbs__separator", "›" }
                                        BreadcrumbItem { is_current: true, "Crittografia & Chiavi HSM" }
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
