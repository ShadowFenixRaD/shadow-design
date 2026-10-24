use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

#[component]
pub fn AppShellPage() -> Element {
    let mut nav_collapsed = use_signal(|| false);
    let mut aside_collapsed = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "header",
            r#type: "Option<Element>",
            default: "None",
            description: "Slot superiore fisso o sticky per AppShellHeader (z-index 30)",
        },
        PropRow {
            name: "navbar",
            r#type: "Option<Element>",
            default: "None",
            description:
                "Slot barra laterale sinistra per AppShellNavbar con collasso fluido (z-index 25)",
        },
        PropRow {
            name: "aside",
            r#type: "Option<Element>",
            default: "None",
            description:
                "Slot laterale ausiliario destro per AppShellAside con collasso fluido (z-index 25)",
        },
        PropRow {
            name: "main / children",
            r#type: "Element",
            default: "-",
            description:
                "Area centrale contenente AppShellMain o viste applicative con scrolling autonomo",
        },
        PropRow {
            name: "footer",
            r#type: "Option<Element>",
            default: "None",
            description: "Slot inferiore a larghezza piena per AppShellFooter (z-index 20)",
        },
        PropRow {
            name: "navbar_collapsed",
            r#type: "Option<Signal<bool>>",
            default: "None",
            description: "Signal reattivo per pilotare programmaticamente il collasso di Navbar",
        },
        PropRow {
            name: "aside_collapsed",
            r#type: "Option<Signal<bool>>",
            default: "None",
            description: "Signal reattivo per pilotare programmaticamente il collasso di Aside",
        },
        PropRow {
            name: "fullscreen",
            r#type: "bool",
            default: "false",
            description: "Se true forza altezza 100vh come layout root; se false (default) si adatta fluidamente al container genitore",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS/SCSS addizionali applicate alla radice del layout master",
        },
    ];

    let code_snippet = r#"let mut nav_collapsed = use_signal(|| false);
let mut aside_collapsed = use_signal(|| false);

AppShell {
    navbar_collapsed: nav_collapsed,
    aside_collapsed: aside_collapsed,
    header: rsx! {
        AppShellHeader {
            height: "56px",
            div { class: "flex items-center justify-between w-full px-2",
                div { class: "flex items-center gap-3",
                    Button {
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| nav_collapsed.set(!nav_collapsed()),
                        Icon { kind: IconKind::Menu, size: 16 }
                    }
                    span { class: "sc-section-title", "ShadowStudio" }
                }
                div { class: "flex items-center gap-2",
                    Button {
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Ghost,
                        onclick: move |_| aside_collapsed.set(!aside_collapsed()),
                        Icon { kind: IconKind::Settings, size: 16 }
                    }
                }
            }
        }
    },
    navbar: rsx! {
        AppShellNavbar {
            width: "240px",
            div { class: "sc-stack",
                span { class: "sc-control-label", "Navigazione" }
                Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, full_width: true, "Dashboard" }
                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, full_width: true, "Progetti" }
                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, full_width: true, "Impostazioni" }
            }
        }
    },
    aside: rsx! {
        AppShellAside {
            width: "260px",
            div { class: "sc-stack",
                span { class: "sc-control-label", "Ispettore Attivo" }
                span { class: "sc-subtle-text", "Nessun nodo selezionato" }
            }
        }
    },
    AppShellMain {
        div { class: "sc-stack",
            h1 { class: "sc-section-title", "Contenuto Applicativo" }
            p { class: "sc-muted-text", "Area scrollabile indipendente con supporto viewport flessibile." }
        }
    },
    footer: rsx! {
        AppShellFooter {
            height: "36px",
            div { class: "sc-title-row w-full",
                span { "Ready • v{VERSION}" }
                span { "Shadow Design System" }
            }
        }
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "AppShell".to_string(),
                description: "Architettura e layout master responsive a 5 zone (Header, Navbar, Aside, Main, Footer) con gerarchia z-index standardizzata, hairline specular top e scrolling indipendente.".to_string(),
            }

            FeatureBanner { feature_name: "app_shell" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "sc-stack",
                        div { class: "sc-title-row",
                            div { class: "sc-actions",
                                Button {
                                    size: ButtonSize::Sm,
                                    variant: if nav_collapsed() { ButtonVariant::Secondary } else { ButtonVariant::Primary },
                                    onclick: move |_| nav_collapsed.set(!nav_collapsed()),
                                    left_icon: rsx! { Icon { kind: IconKind::Menu, size: 14 } },
                                    if nav_collapsed() { "Espandi Navbar" } else { "Comprimi Navbar" }
                                }
                                Button {
                                    size: ButtonSize::Sm,
                                    variant: if aside_collapsed() { ButtonVariant::Secondary } else { ButtonVariant::Primary },
                                    onclick: move |_| aside_collapsed.set(!aside_collapsed()),
                                    left_icon: rsx! { Icon { kind: IconKind::Settings, size: 14 } },
                                    if aside_collapsed() { "Espandi Aside" } else { "Comprimi Aside" }
                                }
                            }
                            Badge {
                                label: "Strati: Header Nav Aside Footer Shell".to_string(),
                                variant: BadgeVariant::Primary,
                            }
                        }

                        div { class: "sc-demo-frame", style: "height: 520px;",
                            AppShell {
                                navbar_collapsed: nav_collapsed,
                                aside_collapsed: aside_collapsed,
                                header: rsx! {
                                    AppShellHeader {
                                        height: "56px",
                                        div { class: "sc-title-row w-full h-full",
                                            div { class: "sc-actions",
                                                Button {
                                                    size: ButtonSize::Sm,
                                                    variant: ButtonVariant::Ghost,
                                                    onclick: move |_| nav_collapsed.set(!nav_collapsed()),
                                                    Icon { kind: IconKind::Menu, size: 16 }
                                                }
                                                span { class: "sc-section-title", "ShadowStudio" }
                                            }
                                            div { class: "sc-actions",
                                                Button {
                                                    size: ButtonSize::Xs,
                                                    variant: ButtonVariant::Ghost,
                                                    left_icon: rsx! { Icon { kind: IconKind::Search, size: 13 } },
                                                    "Cerca"
                                                }
                                                Button {
                                                    size: ButtonSize::Sm,
                                                    variant: ButtonVariant::Ghost,
                                                    onclick: move |_| aside_collapsed.set(!aside_collapsed()),
                                                    Icon { kind: IconKind::Settings, size: 16 }
                                                }
                                            }
                                        }
                                    }
                                },
                                navbar: rsx! {
                                    AppShellNavbar {
                                        width: "220px",
                                        div { class: "sc-stack",
                                            span { class: "sc-control-label", "Workspace" }
                                            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, full_width: true, left_icon: rsx! { Icon { kind: IconKind::Box, size: 15 } }, "Dashboard" }
                                            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, full_width: true, left_icon: rsx! { Icon { kind: IconKind::Layers, size: 15 } }, "Progetti" }
                                            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, full_width: true, left_icon: rsx! { Icon { kind: IconKind::Terminal, size: 15 } }, "Console" }
                                            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, full_width: true, left_icon: rsx! { Icon { kind: IconKind::Settings, size: 15 } }, "Impostazioni" }
                                        }
                                    }
                                },
                                aside: rsx! {
                                    AppShellAside {
                                        width: "250px",
                                        div { class: "sc-stack",
                                            div { class: "sc-title-row",
                                                span { class: "sc-control-label", "Ispettore Nodo" }
                                                Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
                                            }
                                            Card { title: "Nodo ID".to_string(),
                                                span { class: "sc-accent-text sc-mono", "node-cluster-07x" }
                                            }
                                            Card { title: "Memoria Allocata".to_string(),
                                                span { class: "sc-mono", "4.2 GB / 16.0 GB" }
                                            }
                                        }
                                    }
                                },
                                footer: rsx! {
                                    AppShellFooter {
                                        height: "36px",
                                        div { class: "sc-title-row w-full h-full",
                                            span { class: "sc-success-text sc-mono", "Tutti i sistemi operativi" }
                                            span { class: "sc-subtle-text sc-mono", "Shadow Design v{VERSION}" }
                                        }
                                    }
                                },
                                AppShellMain {
                                    div { class: "sc-stack",
                                        div { class: "sc-title-row",
                                            div {
                                                h3 { class: "sc-section-title", "Centro di Controllo Cluster" }
                                                p { class: "sc-muted-text", "Monitoraggio dei carichi distribuiti in tempo reale con scrolling fluido." }
                                            }
                                            Button {
                                                size: ButtonSize::Sm,
                                                variant: ButtonVariant::Primary,
                                                left_icon: rsx! { Icon { kind: IconKind::Sparkles, size: 14 } },
                                                "Nuovo Deploy"
                                            }
                                        }

                                        div { class: "sc-card-grid",
                                            Card { title: "Rete Peer-to-Peer".to_string(), subtitle: "+14% rispetto a ieri".to_string(),
                                                span { class: "sc-section-title", "128 Nodi" }
                                            }
                                            Card { title: "Throughput Rete".to_string(), subtitle: "Latenza media: 4.8ms".to_string(),
                                                span { class: "sc-section-title", "1.42 GB/s" }
                                            }
                                        }

                                        Card { title: "Eventi di Sistema Recenti".to_string(),
                                            div { class: "sc-list",
                                                for i in 1..=6 {
                                                    div { key: "{i}", class: "sc-title-row",
                                                        span { class: "sc-muted-text", "Sincronizzazione snapshot cluster #{i}" }
                                                        span { class: "sc-subtle-text sc-mono", "15:4{i}:02" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
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
