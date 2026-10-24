use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn BreadcrumbsPage() -> Element {
    let separator_choice = use_signal(|| 0); // 0: Chevron, 1: Slash, 2: Dot, 3: Arrow
    let max_choice = use_signal(|| 3usize);
    let mut is_expanded = use_signal(|| false);
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
            description: "Soglia massima di elementi visibili. Se superata, gli elementi intermedi vengono compressi nell'ellissi '...'",
        },
        PropRow {
            name: "expanded",
            r#type: "Option<Signal<bool>>",
            default: "None",
            description: "Controllo reattivo bidirezionale dello stato di espansione dell'ellissi",
        },
        PropRow {
            name: "on_toggle_expand",
            r#type: "Option<EventHandler<bool>>",
            default: "None",
            description: "Callback scatenato all'espansione o compressione del percorso",
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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Breadcrumbs".to_string(),
                description: "Navigazione a briciole di pane gerarchica con separatori personalizzabili, icone di contesto e collasso intelligente con ellissi per percorsi lunghi.".to_string(),
            }

            FeatureBanner { feature_name: "breadcrumbs" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            Breadcrumbs {
                                items: if with_icons() { items_data.clone() } else { items_without_icons.clone() },
                                max_items: match max_choice() {
                                    0 => None,
                                    n => Some(n),
                                },
                                expanded: is_expanded,
                                separator_text: match separator_choice() {
                                    1 => Some("/".to_string()),
                                    2 => Some("•".to_string()),
                                    3 => Some("→".to_string()),
                                    _ => None,
                                },
                            }

                            hr { class: "sc-control-divider" }
                            span { class: "sc-control-label", "Composizione Diretta (RSX Manuale)" }
                            Breadcrumbs {
                                BreadcrumbItem { href: "#/", "Home" }
                                li { class: "sd-breadcrumbs__separator", "›" }
                                BreadcrumbItem { href: "#/settings", "Impostazioni" }
                                li { class: "sd-breadcrumbs__separator", "›" }
                                BreadcrumbItem { is_current: true, "Crittografia & Chiavi HSM" }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Separatore".to_string(),
                                options: vec![
                                    (0, "Chevron (›)".to_string()),
                                    (1, "Slash (/)".to_string()),
                                    (2, "Punto (•)".to_string()),
                                    (3, "Freccia (→)".to_string()),
                                ],
                                value: separator_choice,
                            }
                            ChoicePicker {
                                label: "Compressione Max Items".to_string(),
                                options: vec![
                                    (0usize, "Tutti (5)".to_string()),
                                    (4usize, "Max 4".to_string()),
                                    (3usize, "Max 3".to_string()),
                                ],
                                value: max_choice,
                                on_change: move |_| is_expanded.set(false),
                            }
                            ToggleControl { label: "Includi Icone Vettoriali".to_string(), checked: with_icons }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
