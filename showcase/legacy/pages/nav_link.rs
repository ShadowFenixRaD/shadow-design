use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{CheckControl, ControlRow, Counter, FeatureBanner, OptionPicker, PageHeader, Playground, PropRow, ShowcaseTabs};

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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "NavLink & SidebarMenu".to_string(),
                description: "Voci di navigazione per barre laterali con indicatore neon di stato attivo, icone di contesto, badge contatori e sottomenu gerarchici collassabili.".to_string(),
            }

            FeatureBanner { feature_name: "nav_link" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
                        div { class: "sc-stack w-full",
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
                        },
                        controls: rsx! {
                            OptionPicker {
                                label: "Voce attiva".to_string(),
                                options: vec![
                                    ("dashboard".to_string(), "Dashboard".to_string()),
                                    ("analytics".to_string(), "Analitica".to_string()),
                                    ("mesh".to_string(), "Nodi Mesh".to_string()),
                                    ("security".to_string(), "Sicurezza".to_string()),
                                ],
                                value: active_id,
                            }
                            hr { class: "sc-control-divider" }
                            ControlRow {
                                CheckControl { label: "Mostra descrizioni".to_string(), checked: with_desc }
                                CheckControl { label: "Mostra badge contatori".to_string(), checked: with_badge }
                                CheckControl { label: "Disabilita sicurezza".to_string(), checked: is_disabled }
                            }
                            hr { class: "sc-control-divider" }
                            Counter { label: "Voce selezionata".to_string(), value: active_id() }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Visual Catalog
            Card {
                title: "Varianti e Combinazioni Grafiche".to_string(),
                subtitle: "Galleria comparativa di tutte le configurazioni del componente NavLink".to_string(),
                div { class: "sc-card-grid",
                    div { class: "sc-stack",
                        span { class: "sc-control-label", "Semplice & Standard" }
                        NavLink {
                            label: "Panoramica Generale".to_string(),
                        }
                        NavLink {
                            label: "Dashboard Attiva".to_string(),
                            active: true,
                        }
                    }

                    div { class: "sc-stack",
                        span { class: "sc-control-label", "Con Icona & Descrizione" }
                        NavLink {
                            label: "Crittografia Hardware".to_string(),
                            description: Some("Modulo HSM FIPS 140-3".to_string()),
                            icon: SvgIcon::Kind(IconKind::Lock),
                        }
                    }

                    div { class: "sc-stack",
                        span { class: "sc-control-label", "Con Badge Contatore" }
                        NavLink {
                            label: "Notifiche di Sistema".to_string(),
                            icon: SvgIcon::Kind(IconKind::AlertCircle),
                            badge: Some("8 NUOVE".to_string()),
                            badge_variant: Some(BadgeVariant::Danger),
                        }
                    }

                    div { class: "sc-stack",
                        span { class: "sc-control-label", "Stato Disabilitato" }
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
