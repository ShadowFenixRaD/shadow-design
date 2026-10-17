use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn NavLinkPage() -> Element {
    let mut active_id = use_signal(|| "dashboard".to_string());
    let with_desc = use_signal(|| true);
    let with_badge = use_signal(|| true);
    let is_disabled = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "label",
            r#type: "String",
            default: "-",
            description: "Etichetta principale del link di navigazione",
        },
        PropRow {
            name: "description",
            r#type: "Option<String>",
            default: "None",
            description: "Descrizione secondaria sottostante visualizzata con opacità attenuata",
        },
        PropRow {
            name: "icon",
            r#type: "Option<SvgIcon>",
            default: "None",
            description: "Icona vettoriale visualizzata a sinistra con highlight se attivo",
        },
        PropRow {
            name: "active",
            r#type: "bool",
            default: "false",
            description: "Stato attivo: applica l'indicatore neon sul bordo sinistro, glow e contrasto",
        },
        PropRow {
            name: "badge",
            r#type: "Option<String>",
            default: "None",
            description: "Badge contatore o pillola di stato allineata a destra",
        },
        PropRow {
            name: "badge_variant",
            r#type: "Option<BadgeVariant>",
            default: "BadgeVariant::Default",
            description: "Variante cromatica del badge (Default, Success, Warning, Danger)",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "None",
            description: "Sottomenu annidabili collassabili con chevron rotante ed indentazione",
        },
    ];

    let code_snippet = r##"SidebarMenu {
    title: Some("Infrastruttura".to_string()),
    collapsible: true,
    NavLink {
        label: "Nodi Mesh".to_string(),
        description: Some("Throughput p99".to_string()),
        icon: SvgIcon::Kind(IconKind::Server),
        active: true,
        badge: Some("12 ATTIVI".to_string()),
        badge_variant: Some(BadgeVariant::Success),
        // Sottomenu figli
        NavLink {
            label: "Cluster EU-North".to_string(),
            href: Some("#/cluster-eu".to_string()),
        }
    }
}"##
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "NavLink & SidebarMenu" }
                Text { dimmed: true, size: TextSize::Sm, "Voci di navigazione per barre laterali con indicatore neon di stato attivo, icone di contesto, badge contatori e sottomenu gerarchici collassabili." }
            }

            FeatureBanner { feature_name: "nav_link" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6",
                        // Toolbar interattiva
                        div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] grid grid-cols-1 md:grid-cols-3 gap-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Voce Attiva:" }
                                div { class: "grid grid-cols-2 gap-1.5",
                                    button {
                                        r#type: "button",
                                        class: if active_id() == "dashboard" {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| active_id.set("dashboard".to_string()),
                                        "Dashboard"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if active_id() == "analytics" {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| active_id.set("analytics".to_string()),
                                        "Analitica"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if active_id() == "mesh" {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| active_id.set("mesh".to_string()),
                                        "Nodi Mesh"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if active_id() == "security" {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| active_id.set("security".to_string()),
                                        "Sicurezza"
                                    }
                                }
                            }

                            div { class: "flex flex-col justify-center space-y-2 pt-2 md:pt-0 border-t md:border-l border-white/[0.06] md:pl-4",
                                Checkbox {
                                    checked: with_desc,
                                    label: Some("Mostra Descrizioni".to_string()),
                                }
                                Checkbox {
                                    checked: with_badge,
                                    label: Some("Mostra Badge Contatori".to_string()),
                                }
                            }

                            div { class: "flex items-center pt-2 md:pt-0 border-t md:border-l border-white/[0.06] md:pl-4",
                                Checkbox {
                                    checked: is_disabled,
                                    label: Some("Disabilita Voce Sicurezza".to_string()),
                                }
                            }
                        }

                        // Preview box: Barra laterale completa Shadow UI
                        div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6 items-start",
                            div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] shadow-2xl space-y-4",
                                div { class: "text-[11px] font-mono text-zinc-400 pb-2 border-b border-white/[0.06] flex items-center justify-between",
                                    span { "PANNELLO DI NAVIGAZIONE LATERALE" }
                                    span { class: "text-purple-400 font-bold", "NEON ACTIVE" }
                                }

                                SidebarMenu {
                                    title: Some("Generale".to_string()),
                                    NavLink {
                                        label: "Dashboard".to_string(),
                                        description: if with_desc() { Some("Panoramica nodi e cluster".to_string()) } else { None },
                                        icon: SvgIcon::Kind(IconKind::Box),
                                        active: active_id() == "dashboard",
                                        on_click: move |_| active_id.set("dashboard".to_string()),
                                    }
                                    NavLink {
                                        label: "Analitica Rete".to_string(),
                                        description: if with_desc() { Some("Latenza & Throughput live".to_string()) } else { None },
                                        icon: SvgIcon::Kind(IconKind::SignalHigh),
                                        active: active_id() == "analytics",
                                        badge: if with_badge() { Some("LIVE".to_string()) } else { None },
                                        badge_variant: Some(BadgeVariant::Success),
                                        on_click: move |_| active_id.set("analytics".to_string()),
                                    }
                                }

                                SidebarMenu {
                                    title: Some("Infrastruttura Mesh".to_string()),
                                    collapsible: true,
                                    default_opened: true,
                                    NavLink {
                                        label: "Nodi Mesh P2P".to_string(),
                                        description: if with_desc() { Some("Topology a 12 router".to_string()) } else { None },
                                        icon: SvgIcon::Kind(IconKind::Server),
                                        active: active_id() == "mesh",
                                        badge: if with_badge() { Some("12".to_string()) } else { None },
                                        badge_variant: Some(BadgeVariant::Default),
                                        default_opened: true,
                                        on_click: move |_| active_id.set("mesh".to_string()),
                                        // Sottomenu annidati
                                        NavLink {
                                            label: "Cluster EU-North #01".to_string(),
                                            badge: Some("ONLINE".to_string()),
                                            badge_variant: Some(BadgeVariant::Success),
                                        }
                                        NavLink {
                                            label: "Cluster US-East #02".to_string(),
                                            badge: Some("SYNC".to_string()),
                                            badge_variant: Some(BadgeVariant::Warning),
                                        }
                                        NavLink {
                                            label: "Gateway Edge HSM".to_string(),
                                        }
                                    }
                                    NavLink {
                                        label: "Canali Radio P2P".to_string(),
                                        icon: SvgIcon::Kind(IconKind::Radio),
                                        badge: if with_badge() { Some("NEW".to_string()) } else { None },
                                        badge_variant: Some(BadgeVariant::Warning),
                                    }
                                }

                                SidebarMenu {
                                    title: Some("Sicurezza & Accesso".to_string()),
                                    NavLink {
                                        label: "Crittografia HSM".to_string(),
                                        description: if with_desc() { Some("Post-Quantum ML-KEM".to_string()) } else { None },
                                        icon: SvgIcon::Kind(IconKind::Lock),
                                        active: active_id() == "security",
                                        disabled: is_disabled(),
                                        on_click: move |_| active_id.set("security".to_string()),
                                    }
                                    NavLink {
                                        label: "Audit Log & Eventi".to_string(),
                                        icon: SvgIcon::Kind(IconKind::Terminal),
                                    }
                                }
                            }

                            // Info panel accanto
                            div { class: "p-6 rounded-xl bg-[#060608] border border-white/[0.06] space-y-4 text-xs font-mono",
                                div { class: "text-purple-400 font-bold uppercase tracking-wider text-[11px]", "Caratteristiche Visuali & Interazione" }
                                ul { class: "space-y-2 text-zinc-300",
                                    li { "• Indicatore al neon sul bordo sinistro con specular glow coordinato." }
                                    li { "• Sottomenu collassabili con linea di giunzione connettiva a sinistra." }
                                    li { "• Transizione fluida del chevron al click." }
                                    li { "• Accessibilità semantica con supporto `aria-current=\"page\"`." }
                                }
                                div { class: "p-3 rounded bg-white/[0.02] border border-white/[0.04]",
                                    div { class: "text-zinc-500 text-[10px]", "Stato Reattivo Corrente:" }
                                    div { class: "text-zinc-200 mt-1", "Voce Selezionata: \"{active_id()}\"" }
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
