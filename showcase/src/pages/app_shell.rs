use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

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
                    span { class: "font-bold text-white text-sm", "ShadowStudio" }
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
            div { class: "p-4 space-y-2",
                div { class: "text-xs font-semibold text-zinc-400 uppercase", "Navigazione" }
                div { class: "p-2 rounded bg-white/10 text-white text-sm cursor-pointer", "Dashboard" }
                div { class: "p-2 rounded hover:bg-white/[0.06] text-zinc-400 hover:text-white text-sm cursor-pointer", "Progetti" }
                div { class: "p-2 rounded hover:bg-white/[0.06] text-zinc-400 hover:text-white text-sm cursor-pointer", "Impostazioni" }
            }
        }
    },
    aside: rsx! {
        AppShellAside {
            width: "260px",
            div { class: "p-4 space-y-3",
                div { class: "text-xs font-semibold text-zinc-400 uppercase", "Ispettore Attivo" }
                div { class: "text-xs text-zinc-500", "Nessun nodo selezionato" }
            }
        }
    },
    AppShellMain {
        div { class: "p-6 space-y-4",
            h1 { class: "text-xl font-bold text-white", "Contenuto Applicativo" }
            p { class: "text-sm text-zinc-400", "Area scrollabile indipendente con supporto viewport flessibile." }
        }
    },
    footer: rsx! {
        AppShellFooter {
            height: "36px",
            div { class: "flex items-center justify-between w-full px-4 text-xs text-zinc-500 font-mono",
                span { "Ready • v{VERSION}" }
                span { "Shadow Design System" }
            }
        }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-5xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "AppShell" }
                Text { dimmed: true, size: TextSize::Sm, "Architettura e layout master responsive a 5 zone (Header, Navbar, Aside, Main, Footer) con gerarchia z-index standardizzata, hairline specular top e scrolling indipendente." }
            }

            FeatureBanner { feature_name: "app-shell" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "flex flex-col gap-4 w-full",
                        div { class: "flex flex-wrap items-center justify-between gap-3 p-3 rounded-lg bg-[#060608] border border-white/[0.08]",
                            div { class: "flex items-center gap-2",
                                Button {
                                    size: ButtonSize::Sm,
                                    variant: if nav_collapsed() { ButtonVariant::Outline } else { ButtonVariant::Primary },
                                    onclick: move |_| nav_collapsed.set(!nav_collapsed()),
                                    left_icon: rsx! { Icon { kind: IconKind::Menu, size: 14 } },
                                    if nav_collapsed() { "Espandi Navbar" } else { "Comprimi Navbar" }
                                }
                                Button {
                                    size: ButtonSize::Sm,
                                    variant: if aside_collapsed() { ButtonVariant::Outline } else { ButtonVariant::Primary },
                                    onclick: move |_| aside_collapsed.set(!aside_collapsed()),
                                    left_icon: rsx! { Icon { kind: IconKind::Settings, size: 14 } },
                                    if aside_collapsed() { "Espandi Aside" } else { "Comprimi Aside" }
                                }
                            }
                            div { class: "flex items-center gap-2",
                                Badge {
                                    label: "Z-Layers: Header(30) Nav(25) Aside(25) Footer(20) Shell(10)".to_string(),
                                    variant: BadgeVariant::Primary,
                                }
                            }
                        }

                        div {
                            class: "w-full rounded-xl border border-white/[0.08] overflow-hidden shadow-2xl",
                            style: "height: 520px;",
                            AppShell {
                                navbar_collapsed: nav_collapsed,
                                aside_collapsed: aside_collapsed,
                                header: rsx! {
                                    AppShellHeader {
                                        height: "56px",
                                        div { class: "flex items-center justify-between w-full h-full",
                                            div { class: "flex items-center gap-3",
                                                Button {
                                                    size: ButtonSize::Sm,
                                                    variant: ButtonVariant::Ghost,
                                                    onclick: move |_| nav_collapsed.set(!nav_collapsed()),
                                                    Icon { kind: IconKind::Menu, size: 16 }
                                                }
                                                div { class: "flex items-center gap-2",
                                                    div { class: "w-7 h-7 rounded-lg bg-purple-600/30 border border-purple-500/40 flex items-center justify-center text-purple-300",
                                                        Icon { kind: IconKind::Layers, size: 16 }
                                                    }
                                                    span { class: "font-bold text-white text-sm tracking-tight", "ShadowStudio" }
                                                }
                                            }
                                            div { class: "flex items-center gap-2",
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
                                        div { class: "p-4 space-y-4",
                                            div { class: "text-xs font-semibold text-zinc-400 uppercase tracking-wider", "Workspace" }
                                            div { class: "space-y-1",
                                                div { class: "flex items-center gap-2 px-3 py-2 rounded-lg bg-white/10 text-white text-sm font-medium cursor-pointer transition-colors",
                                                    Icon { kind: IconKind::Box, size: 15 }
                                                    span { "Dashboard" }
                                                }
                                                div { class: "flex items-center gap-2 px-3 py-2 rounded-lg text-zinc-400 hover:text-white hover:bg-white/[0.06] text-sm font-medium cursor-pointer transition-colors",
                                                    Icon { kind: IconKind::Layers, size: 15 }
                                                    span { "Progetti" }
                                                }
                                                div { class: "flex items-center gap-2 px-3 py-2 rounded-lg text-zinc-400 hover:text-white hover:bg-white/[0.06] text-sm font-medium cursor-pointer transition-colors",
                                                    Icon { kind: IconKind::Terminal, size: 15 }
                                                    span { "Console" }
                                                }
                                                div { class: "flex items-center gap-2 px-3 py-2 rounded-lg text-zinc-400 hover:text-white hover:bg-white/[0.06] text-sm font-medium cursor-pointer transition-colors",
                                                    Icon { kind: IconKind::Settings, size: 15 }
                                                    span { "Impostazioni" }
                                                }
                                            }
                                        }
                                    }
                                },
                                aside: rsx! {
                                    AppShellAside {
                                        width: "250px",
                                        div { class: "p-4 space-y-4",
                                            div { class: "flex items-center justify-between",
                                                span { class: "text-xs font-semibold text-zinc-400 uppercase tracking-wider", "Ispettore Nodo" }
                                                Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
                                            }
                                            div { class: "p-3 rounded-lg bg-[#060608] border border-white/[0.08] space-y-2",
                                                div { class: "text-xs text-zinc-400", "Nodo ID" }
                                                div { class: "text-xs font-mono text-purple-300", "node-cluster-07x" }
                                            }
                                            div { class: "p-3 rounded-lg bg-[#060608] border border-white/[0.08] space-y-2",
                                                div { class: "text-xs text-zinc-400", "Memoria Allocata" }
                                                div { class: "text-xs font-mono text-zinc-200", "4.2 GB / 16.0 GB" }
                                            }
                                        }
                                    }
                                },
                                footer: rsx! {
                                    AppShellFooter {
                                        height: "36px",
                                        div { class: "flex items-center justify-between w-full h-full text-xs text-zinc-500 font-mono",
                                            div { class: "flex items-center gap-2",
                                                span { class: "w-1.5 h-1.5 rounded-full bg-emerald-400" }
                                                span { "Tutti i sistemi operativi" }
                                            }
                                            span { "Shadow Design v{VERSION}" }
                                        }
                                    }
                                },
                                AppShellMain {
                                    div { class: "p-6 space-y-6",
                                        div { class: "flex items-center justify-between",
                                            div {
                                                h3 { class: "text-lg font-bold text-white tracking-tight", "Centro di Controllo Cluster" }
                                                p { class: "text-xs text-zinc-400", "Monitoraggio dei carichi distribuiti in tempo reale con scrolling fluido." }
                                            }
                                            Button {
                                                size: ButtonSize::Sm,
                                                variant: ButtonVariant::Primary,
                                                left_icon: rsx! { Icon { kind: IconKind::Sparkles, size: 14 } },
                                                "Nuovo Deploy"
                                            }
                                        }

                                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                                            div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] space-y-2",
                                                div { class: "flex items-center justify-between",
                                                    span { class: "text-xs text-zinc-400", "Rete Peer-to-Peer" }
                                                    Icon { kind: IconKind::Server, size: 16 }
                                                }
                                                div { class: "text-2xl font-bold text-white", "128 Nodi" }
                                                div { class: "text-xs text-emerald-400", "+14% rispetto a ieri" }
                                            }
                                            div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] space-y-2",
                                                div { class: "flex items-center justify-between",
                                                    span { class: "text-xs text-zinc-400", "Throughput Rete" }
                                                    Icon { kind: IconKind::Wifi, size: 16 }
                                                }
                                                div { class: "text-2xl font-bold text-white", "1.42 GB/s" }
                                                div { class: "text-xs text-purple-400", "Latenza media: 4.8ms" }
                                            }
                                        }

                                        div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] space-y-3",
                                            h4 { class: "text-sm font-semibold text-white", "Eventi di Sistema Recenti" }
                                            for i in 1..=6 {
                                                div { key: "{i}", class: "flex items-center justify-between py-2 border-b border-white/[0.08] text-xs",
                                                    div { class: "flex items-center gap-2",
                                                        span { class: "w-2 h-2 rounded-full bg-purple-500" }
                                                        span { class: "text-zinc-200", "Sincronizzazione snapshot cluster #{i}" }
                                                    }
                                                    span { class: "font-mono text-zinc-500", "15:4{i}:02" }
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
