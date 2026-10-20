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
            name: "href",
            r#type: "Option<String>",
            default: "None",
            description: "URL opzionale di destinazione (renderizza un tag semantico <a>)",
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
            description: "Variante cromatica del badge (Default, Primary, Success, Warning, Danger)",
        },
        PropRow {
            name: "default_opened",
            r#type: "bool",
            default: "false",
            description: "Stato iniziale di apertura per i sottomenu figli",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita le interazioni e applica l'attenuazione visiva",
        },
        PropRow {
            name: "children",
            r#type: "Option<Element>",
            default: "None",
            description: "Sottomenu annidabili collassabili con chevron rotante ed indentazione connettiva",
        },
    ];

    let code_snippet = r##"SidebarMenu {
    title: Some("Infrastruttura Mesh".to_string()),
    collapsible: true,
    default_opened: true,
    NavLink {
        label: "Nodi Mesh P2P".to_string(),
        description: Some("Topology a 12 router".to_string()),
        icon: SvgIcon::Kind(IconKind::Server),
        active: true,
        badge: Some("12".to_string()),
        badge_variant: Some(BadgeVariant::Default),
        default_opened: true,
        // Sottomenu figli
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
    }
}"##
    .to_string();

    rsx! {
        div { class: "max-w-5xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "NavLink & SidebarMenu" }
                Text { dimmed: true, size: TextSize::Sm, "Voci di navigazione per barre laterali con indicatore neon di stato attivo, icone di contesto, badge contatori e sottomenu gerarchici collassabili." }
            }

            FeatureBanner { feature_name: "nav_link" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6",
                        // Toolbar interattiva con flexbox e spaziatura generosa
                        div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-wrap items-center justify-between gap-4 text-xs font-mono",
                            div { class: "flex items-center gap-3 flex-wrap",
                                span { class: "text-zinc-400 uppercase tracking-wider text-[10px]", "Voce Attiva:" }
                                div { class: "flex flex-wrap gap-1.5",
                                    button {
                                        r#type: "button",
                                        class: if active_id() == "dashboard" {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer transition-colors"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] hover:bg-white/[0.08] cursor-pointer transition-colors"
                                        },
                                        onclick: move |_| active_id.set("dashboard".to_string()),
                                        "Dashboard"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if active_id() == "analytics" {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer transition-colors"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] hover:bg-white/[0.08] cursor-pointer transition-colors"
                                        },
                                        onclick: move |_| active_id.set("analytics".to_string()),
                                        "Analitica"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if active_id() == "mesh" {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer transition-colors"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] hover:bg-white/[0.08] cursor-pointer transition-colors"
                                        },
                                        onclick: move |_| active_id.set("mesh".to_string()),
                                        "Nodi Mesh"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if active_id() == "security" {
                                            "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer transition-colors"
                                        } else {
                                            "px-2.5 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] hover:bg-white/[0.08] cursor-pointer transition-colors"
                                        },
                                        onclick: move |_| active_id.set("security".to_string()),
                                        "Sicurezza"
                                    }
                                }
                            }

                            div { class: "flex items-center gap-4 flex-wrap",
                                Checkbox {
                                    checked: with_desc,
                                    label: Some("Mostra Descrizioni".to_string()),
                                    size: CheckboxSize::Sm,
                                }
                                Checkbox {
                                    checked: with_badge,
                                    label: Some("Mostra Badge Contatori".to_string()),
                                    size: CheckboxSize::Sm,
                                }
                                Checkbox {
                                    checked: is_disabled,
                                    label: Some("Disabilita Sicurezza".to_string()),
                                    size: CheckboxSize::Sm,
                                }
                            }
                        }

                        // Preview box: Mockup Barra laterale completa Shadow UI affiancato al pannello info
                        div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-start",
                            div { class: "p-5 rounded-2xl bg-[#060608] border border-white/[0.08] shadow-2xl flex flex-col gap-4 w-full",
                                div { class: "text-[11px] font-mono text-zinc-400 pb-2.5 border-b border-white/[0.06] flex items-center justify-between",
                                    span { class: "font-semibold tracking-wider", "PANNELLO DI NAVIGAZIONE LATERALE" }
                                    span { class: "text-purple-400 font-bold", "NEON ACTIVE" }
                                }

                                div { class: "flex flex-col gap-3 w-full",
                                    SidebarMenu {
                                        title: Some("Generale".to_string()),
                                        collapsible: true,
                                        default_opened: true,
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
                                        collapsible: true,
                                        default_opened: true,
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
                            }

                            // Info panel accanto
                            div { class: "p-6 rounded-2xl bg-[#060608] border border-white/[0.08] space-y-4 text-xs font-mono w-full",
                                div { class: "text-purple-400 font-bold uppercase tracking-wider text-[11px]", "Caratteristiche Visuali & Interazione" }
                                ul { class: "space-y-2.5 text-zinc-300",
                                    li { "• Indicatore al neon sul bordo sinistro con specular glow coordinato." }
                                    li { "• Sottomenu collassabili con linea di giunzione connettiva a sinistra." }
                                    li { "• Azioni di coda allineate orizzontalmente (badge contatore e chevron rotante)." }
                                    li { "• Transizione fluida a 180° per header e 90° per sottomenu al click." }
                                    li { "• Accessibilità semantica con supporto nativo ad `aria-current=\"page\"`." }
                                }
                                div { class: "p-3.5 rounded-xl bg-white/[0.02] border border-white/[0.04] space-y-1.5",
                                    div { class: "text-zinc-500 text-[10px] uppercase tracking-wider", "Stato Reattivo Corrente:" }
                                    div { class: "text-zinc-200 font-semibold", "Voce Selezionata: \"{active_id()}\"" }
                                    div { class: "text-zinc-400 text-[11px]", "Descrizioni: {with_desc()} | Badge: {with_badge()} | Disabilitato: {is_disabled()}" }
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Visual Catalog
            Card {
                title: "Varianti e Combinazioni Grafiche".to_string(),
                subtitle: "Galleria comparativa di tutte le configurazioni del componente NavLink".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                    div { class: "p-3 rounded-xl bg-white/[0.02] border border-white/[0.06] space-y-2",
                        span { class: "text-[10px] font-mono text-zinc-500 uppercase tracking-wider block", "Semplice & Standard" }
                        NavLink {
                            label: "Panoramica Generale".to_string(),
                        }
                        NavLink {
                            label: "Dashboard Attiva".to_string(),
                            active: true,
                        }
                    }

                    div { class: "p-3 rounded-xl bg-white/[0.02] border border-white/[0.06] space-y-2",
                        span { class: "text-[10px] font-mono text-zinc-500 uppercase tracking-wider block", "Con Icona & Descrizione" }
                        NavLink {
                            label: "Crittografia Hardware".to_string(),
                            description: Some("Modulo HSM FIPS 140-3".to_string()),
                            icon: SvgIcon::Kind(IconKind::Lock),
                        }
                    }

                    div { class: "p-3 rounded-xl bg-white/[0.02] border border-white/[0.06] space-y-2",
                        span { class: "text-[10px] font-mono text-zinc-500 uppercase tracking-wider block", "Con Badge Contatore" }
                        NavLink {
                            label: "Notifiche di Sistema".to_string(),
                            icon: SvgIcon::Kind(IconKind::AlertCircle),
                            badge: Some("8 NUOVE".to_string()),
                            badge_variant: Some(BadgeVariant::Danger),
                        }
                    }

                    div { class: "p-3 rounded-xl bg-white/[0.02] border border-white/[0.06] space-y-2",
                        span { class: "text-[10px] font-mono text-zinc-500 uppercase tracking-wider block", "Stato Disabilitato" }
                        NavLink {
                            label: "Accesso Root Limitato".to_string(),
                            description: Some("Privilegi amministrativi richiesti".to_string()),
                            icon: SvgIcon::Kind(IconKind::Shield),
                            disabled: true,
                        }
                    }
                }
            }
        }
    }
}
