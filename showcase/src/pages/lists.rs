use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{CheckControl, ChoicePicker, ControlRow, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs};

#[component]
pub fn ListsPage() -> Element {
    let list_type = use_signal(|| ListType::Unordered);
    let mut marker = use_signal(|| ListMarker::Disc);
    let spacing = use_signal(|| ListSpacing::Sm);
    let size = use_signal(|| ListSize::Md);
    let mut orientation = use_signal(|| ListOrientation::Vertical);
    let is_horizontal = use_signal(|| false);
    let center = use_signal(|| false);
    let use_custom_icons = use_signal(|| false);

    use_effect(move || {
        if is_horizontal() {
            orientation.set(ListOrientation::Horizontal);
        } else {
            orientation.set(ListOrientation::Vertical);
        }
    });

    let props = vec![
        PropRow {
            name: "list_type",
            r#type: "ListType",
            default: "ListType::Unordered",
            description: "Tipo di lista: Unordered (<ul>) o Ordered (<ol>)",
        },
        PropRow {
            name: "marker",
            r#type: "Option<ListMarker>",
            default: "None (Disc per <ul>, Decimal per <ol>)",
            description:
                "Marcatore: Disc, Circle, Square, Decimal, LowerAlpha, UpperAlpha, None, Custom",
        },
        PropRow {
            name: "spacing",
            r#type: "ListSpacing",
            default: "ListSpacing::Sm",
            description: "Spaziatura verticale/orizzontale tra gli elementi: Xs, Sm, Md, Lg, Xl",
        },
        PropRow {
            name: "size",
            r#type: "ListSize",
            default: "ListSize::Md",
            description: "Scala dimensionale tipografica e icone: Xs, Sm, Md, Lg, Xl",
        },
        PropRow {
            name: "orientation",
            r#type: "ListOrientation",
            default: "ListOrientation::Vertical",
            description: "Orientamento degli elementi: Vertical o Horizontal",
        },
        PropRow {
            name: "center",
            r#type: "bool",
            default: "false",
            description: "Allinea verticalmente al centro icone e marcatori rispetto al testo",
        },
        PropRow {
            name: "icon",
            r#type: "Option<Element>",
            default: "None",
            description: "Icona globale ereditata da tutti i ListItem (quando marker = Custom)",
        },
        PropRow {
            name: "with_padding",
            r#type: "Option<bool>",
            default: "None (true per default, false per Custom/Horizontal)",
            description: "Controlla il rientro sinistro standard della lista",
        },
    ];

    let code_snippet = r#"List {
    list_type: ListType::Unordered,
    marker: Some(ListMarker::Custom),
    spacing: ListSpacing::Sm,
    size: ListSize::Md,
    icon: rsx! { Icon { kind: IconKind::CheckCircle, class: "w-4 h-4 sc-accent-text" } },
    ListItem { "Crittografia End-to-End post-quantica" }
    ListItem { "Protocollo di routing mesh p2p zero-trust" }
    ListItem { "Sincronizzazione decentralizzata dei nodi" }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "List & ListItem".to_string(),
                description: "Elenchi ordinati, non ordinati e con icone vettoriali personalizzate, spaziatura flessibile e supporto a sub-liste annidate.".to_string(),
            }

            FeatureBanner { feature_name: "list" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                            List {
                                list_type: list_type(),
                                marker: Some(marker()),
                                spacing: spacing(),
                                size: size(),
                                orientation: orientation(),
                                center: center(),
                                icon: if use_custom_icons() {
                                    Some(rsx! {
                                        Icon { kind: IconKind::CheckCircle, class: "w-4 h-4 sc-accent-text" }
                                    })
                                } else {
                                    None
                                },
                                ListItem { "Architettura Zero-Trust distribuita" }
                                ListItem { "Verifica crittografica delle firme ED25519" }
                                ListItem { "Routing a maglia dinamica a bassa latenza" }
                                ListItem { "Canali temporanei P2P con cancellazione a zero residui" }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Tipo di lista".to_string(),
                                options: vec![
                                    (ListType::Unordered, "Unordered (ul)".to_string()),
                                    (ListType::Ordered, "Ordered (ol)".to_string()),
                                ],
                                value: list_type,
                                on_change: move |t: ListType| {
                                    marker.set(if t == ListType::Ordered { ListMarker::Decimal } else { ListMarker::Disc });
                                },
                            }
                            ChoicePicker {
                                label: "Stile marcatore".to_string(),
                                options: vec![
                                    (ListMarker::Disc, "Disc".to_string()),
                                    (ListMarker::Circle, "Circle".to_string()),
                                    (ListMarker::Square, "Square".to_string()),
                                    (ListMarker::Decimal, "Decimal".to_string()),
                                    (ListMarker::LowerAlpha, "Lower Alpha".to_string()),
                                    (ListMarker::LowerRoman, "Lower Roman".to_string()),
                                    (ListMarker::None, "Nessuno".to_string()),
                                    (ListMarker::Custom, "Custom".to_string()),
                                ],
                                value: marker,
                            }
                            ChoicePicker {
                                label: "Spaziatura".to_string(),
                                options: vec![
                                    (ListSpacing::Xs, "Xs".to_string()),
                                    (ListSpacing::Sm, "Sm".to_string()),
                                    (ListSpacing::Md, "Md".to_string()),
                                    (ListSpacing::Lg, "Lg".to_string()),
                                    (ListSpacing::Xl, "Xl".to_string()),
                                ],
                                value: spacing,
                            }
                            ChoicePicker {
                                label: "Dimensione testo".to_string(),
                                options: vec![
                                    (ListSize::Xs, "Xs".to_string()),
                                    (ListSize::Sm, "Sm".to_string()),
                                    (ListSize::Md, "Md".to_string()),
                                    (ListSize::Lg, "Lg".to_string()),
                                    (ListSize::Xl, "Xl".to_string()),
                                ],
                                value: size,
                            }
                            hr { class: "sc-control-divider" }
                            ControlRow {
                                CheckControl { label: "Orizzontale".to_string(), checked: is_horizontal }
                                CheckControl { label: "Allinea al centro".to_string(), checked: center }
                                CheckControl { label: "Icone SVG custom".to_string(), checked: use_custom_icons }
                            }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Sezione Sub-liste annidate e varianti d'uso
            div { class: "sc-stack",
                h2 { class: "sc-section-title", "Sub-liste annidate e casi d'uso" }
                p { class: "sc-page-desc", "Le liste supportano la nidificazione automatica con indentazione nativa." }

                div { class: "sc-card-grid",
                    Card { title: "Gerarchia ad albero".to_string(),
                        List {
                            list_type: ListType::Unordered,
                            marker: Some(ListMarker::Disc),
                            ListItem { "Nodo Coordinatore Rete" }
                            ListItem {
                                "Nodi Mesh Periferici"
                                List {
                                    list_type: ListType::Unordered,
                                    marker: Some(ListMarker::Circle),
                                    ListItem { "Peer Alpha (Canale 12)" }
                                    ListItem { "Peer Beta (Canale 14)" }
                                    ListItem {
                                        "Sub-cluster IoT"
                                        List {
                                            list_type: ListType::Unordered,
                                            marker: Some(ListMarker::Square),
                                            ListItem { "Sensore Mesh #1" }
                                            ListItem { "Sensore Mesh #2" }
                                        }
                                    }
                                }
                            }
                            ListItem { "Relay Gateway Criptato" }
                        }
                    }

                    Card { title: "Icone individuali per item".to_string(),
                        List {
                            spacing: ListSpacing::Md,
                            ListItem {
                                icon: rsx! { Icon { kind: IconKind::CheckCircle, class: "w-4 h-4 sc-success-text" } },
                                "Connessione stabilita con successo (TLS 1.3)"
                            }
                            ListItem {
                                icon: rsx! { Icon { kind: IconKind::AlertTriangle, class: "w-4 h-4 sc-warning-text" } },
                                "Latenza rilevata superiore a 45ms"
                            }
                            ListItem {
                                icon: rsx! { Icon { kind: IconKind::Flame, class: "w-4 h-4 sc-danger-text" } },
                                "Nodo segnalato per sovraccarico termico"
                            }
                            ListItem {
                                icon: rsx! { Icon { kind: IconKind::Shield, class: "w-4 h-4 sc-info-text" } },
                                "Firma biometrica convalidata da ShadowID"
                            }
                        }
                    }
                }
            }
        }
    }
}
