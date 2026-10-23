use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn ListsPage() -> Element {
    let mut list_type = use_signal(|| ListType::Unordered);
    let mut marker = use_signal(|| ListMarker::Disc);
    let mut spacing = use_signal(|| ListSpacing::Sm);
    let mut size = use_signal(|| ListSize::Md);
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
    icon: rsx! { Icon { kind: IconKind::CheckCircle, class: "w-4 h-4 text-purple-400" } },
    ListItem { "Crittografia End-to-End post-quantica" }
    ListItem { "Protocollo di routing mesh p2p zero-trust" }
    ListItem { "Sincronizzazione decentralizzata dei nodi" }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "List & ListItem" }
                Text { dimmed: true, size: TextSize::Sm, "Elenchi ordinati, non ordinati e con icone vettoriali personalizzate, spaziatura flessibile e supporto a sub-liste annidate." }
            }

            FeatureBanner { feature_name: "list" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-start",
                        // Live Preview Panel
                        div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] min-h-[300px] shadow-inner flex flex-col justify-center",
                            List {
                                list_type: list_type(),
                                marker: Some(marker()),
                                spacing: spacing(),
                                size: size(),
                                orientation: orientation(),
                                center: center(),
                                icon: if use_custom_icons() {
                                    Some(rsx! {
                                        Icon { kind: IconKind::CheckCircle, class: "w-4 h-4 text-purple-400" }
                                    })
                                } else {
                                    None
                                },
                                ListItem { "Architettura Zero-Trust distribuita" }
                                ListItem { "Verifica crittografica delle firme ED25519" }
                                ListItem { "Routing a maglia dinamica a bassa latenza" }
                                ListItem { "Canali temporanei P2P con cancellazione a zero residui" }
                            }
                        }

                        // Controls Panel
                        div { class: "p-6 rounded-xl bg-zinc-950/60 border border-white/[0.08] space-y-5 text-sm",
                            div { class: "font-semibold text-zinc-200 border-b border-white/[0.06] pb-2", "Configurazione Live" }

                            // List Type
                            div { class: "space-y-1.5",
                                label { class: "text-xs font-mono text-zinc-400", "Tipo di Lista" }
                                div { class: "flex gap-2",
                                    button {
                                        class: format!("px-3 py-1 rounded text-xs border transition-colors {}",
                                            if list_type() == ListType::Unordered { "bg-purple-950/60 border-purple-500/50 text-purple-200 font-semibold" } else { "bg-zinc-900 border-zinc-800 text-zinc-400" }),
                                        onclick: move |_| {
                                            list_type.set(ListType::Unordered);
                                            marker.set(ListMarker::Disc);
                                        },
                                        "Unordered (ul)"
                                    }
                                    button {
                                        class: format!("px-3 py-1 rounded text-xs border transition-colors {}",
                                            if list_type() == ListType::Ordered { "bg-purple-950/60 border-purple-500/50 text-purple-200 font-semibold" } else { "bg-zinc-900 border-zinc-800 text-zinc-400" }),
                                        onclick: move |_| {
                                            list_type.set(ListType::Ordered);
                                            marker.set(ListMarker::Decimal);
                                        },
                                        "Ordered (ol)"
                                    }
                                }
                            }

                            // Marker Type
                            div { class: "space-y-1.5",
                                label { class: "text-xs font-mono text-zinc-400", "Stile Marcatore" }
                                div { class: "grid grid-cols-3 gap-1.5",
                                    for (m, lbl) in [
                                        (ListMarker::Disc, "Disc"),
                                        (ListMarker::Circle, "Circle"),
                                        (ListMarker::Square, "Square"),
                                        (ListMarker::Decimal, "Decimal"),
                                        (ListMarker::LowerAlpha, "Lower Alpha"),
                                        (ListMarker::LowerRoman, "Lower Roman"),
                                        (ListMarker::None, "Nessuno"),
                                        (ListMarker::Custom, "Custom"),
                                    ] {
                                        button {
                                            class: format!("px-2 py-1 rounded text-xs border transition-colors {}",
                                                if marker() == m { "bg-purple-950/60 border-purple-500/50 text-purple-200 font-semibold" } else { "bg-zinc-900 border-zinc-800 text-zinc-400" }),
                                            onclick: move |_| marker.set(m),
                                            "{lbl}"
                                        }
                                    }
                                }
                            }

                            // Spacing
                            div { class: "space-y-1.5",
                                label { class: "text-xs font-mono text-zinc-400", "Spaziatura" }
                                div { class: "flex gap-2",
                                    for (sp, lbl) in [(ListSpacing::Xs, "Xs"), (ListSpacing::Sm, "Sm"), (ListSpacing::Md, "Md"), (ListSpacing::Lg, "Lg"), (ListSpacing::Xl, "Xl")] {
                                        button {
                                            class: format!("px-2.5 py-1 rounded text-xs border transition-colors {}",
                                                if spacing() == sp { "bg-purple-950/60 border-purple-500/50 text-purple-200 font-semibold" } else { "bg-zinc-900 border-zinc-800 text-zinc-400" }),
                                            onclick: move |_| spacing.set(sp),
                                            "{lbl}"
                                        }
                                    }
                                }
                            }

                            // Size
                            div { class: "space-y-1.5",
                                label { class: "text-xs font-mono text-zinc-400", "Dimensione Testo" }
                                div { class: "flex gap-2",
                                    for (sz, lbl) in [(ListSize::Xs, "Xs"), (ListSize::Sm, "Sm"), (ListSize::Md, "Md"), (ListSize::Lg, "Lg"), (ListSize::Xl, "Xl")] {
                                        button {
                                            class: format!("px-2.5 py-1 rounded text-xs border transition-colors {}",
                                                if size() == sz { "bg-purple-950/60 border-purple-500/50 text-purple-200 font-semibold" } else { "bg-zinc-900 border-zinc-800 text-zinc-400" }),
                                            onclick: move |_| size.set(sz),
                                            "{lbl}"
                                        }
                                    }
                                }
                            }

                            // Orientation & Center
                            div { class: "flex gap-4 pt-2 border-t border-white/[0.06]",
                                Checkbox {
                                    checked: is_horizontal,
                                    label: "Orizzontale".to_string(),
                                    size: CheckboxSize::Sm,
                                }

                                Checkbox {
                                    checked: center,
                                    label: "Allinea al Centro".to_string(),
                                    size: CheckboxSize::Sm,
                                }

                                Checkbox {
                                    checked: use_custom_icons,
                                    label: "Icone SVG Custom".to_string(),
                                    size: CheckboxSize::Sm,
                                    on_change: move |c| {
                                        if c {
                                            marker.set(ListMarker::Custom);
                                        }
                                    },
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Sezione Sub-liste annidate e Varianti d'uso
            div { class: "space-y-4 pt-4 border-t border-white/[0.06]",
                Heading { order: HeadingOrder::H3, "Sub-liste Annidate & Casi d'Uso" }
                Text { dimmed: true, size: TextSize::Sm, "Le liste supportano la nidificazione automatica con indentazione nativa." }

                div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                    // Sub-list example
                    div { class: "p-6 rounded-xl bg-[#060608] border border-white/[0.08] space-y-3",
                        div { class: "text-xs font-semibold text-purple-400 font-mono uppercase tracking-wider", "Gerarchia ad Albero" }
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

                    // Individual Custom Item Icons
                    div { class: "p-6 rounded-xl bg-[#060608] border border-white/[0.08] space-y-3",
                        div { class: "text-xs font-semibold text-purple-400 font-mono uppercase tracking-wider", "Icone Individuali per Item" }
                        List {
                            spacing: ListSpacing::Md,
                            ListItem {
                                icon: rsx! { Icon { kind: IconKind::CheckCircle, class: "w-4 h-4 text-emerald-400" } },
                                "Connessione stabilita con successo (TLS 1.3)"
                            }
                            ListItem {
                                icon: rsx! { Icon { kind: IconKind::AlertTriangle, class: "w-4 h-4 text-amber-400" } },
                                "Latenza rilevata superiore a 45ms"
                            }
                            ListItem {
                                icon: rsx! { Icon { kind: IconKind::Flame, class: "w-4 h-4 text-rose-400" } },
                                "Nodo segnalato per sovraccarico termico"
                            }
                            ListItem {
                                icon: rsx! { Icon { kind: IconKind::Shield, class: "w-4 h-4 text-cyan-400" } },
                                "Firma biometrica convalidata da ShadowID"
                            }
                        }
                    }
                }
            }
        }
    }
}
