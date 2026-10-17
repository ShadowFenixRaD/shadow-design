use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn BottomNavigationPage() -> Element {
    let mut active_tab = use_signal(|| "home".to_string());
    let mut tab_count = use_signal(|| 4usize);
    let with_badges = use_signal(|| true);
    let with_border = use_signal(|| true);
    let with_blur = use_signal(|| true);
    let is_vault_disabled = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "position",
            r#type: "Option<BottomNavPosition>",
            default: "BottomNavPosition::Fixed",
            description: "Posizionamento della barra: Fixed (viewport mobile), Absolute (contenitore relativo/mockup) o Relative",
        },
        PropRow {
            name: "safe_area",
            r#type: "bool",
            default: "true",
            description: "Applica automaticamente padding-bottom per l'home indicator di iOS e safe area mobile",
        },
        PropRow {
            name: "border",
            r#type: "bool",
            default: "true",
            description: "Include il bordo superiore con top specular hairline semi-trasparente (1px white/10)",
        },
        PropRow {
            name: "blur",
            r#type: "bool",
            default: "true",
            description: "Abilita l'effetto glassmorphism con backdrop-filter blur a 16px",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Destinazioni di navigazione (da 3 a 5 elementi BottomNavItem)",
        },
    ];

    let code_snippet = r##"BottomNavigation {
    position: Some(BottomNavPosition::Fixed),
    safe_area: true,
    border: true,
    blur: true,
    BottomNavItem {
        label: "Home".to_string(),
        icon: SvgIcon::Kind(IconKind::Box),
        active: active_tab() == "home",
        on_click: move |_| active_tab.set("home".to_string()),
    }
    BottomNavItem {
        label: "Radio".to_string(),
        icon: SvgIcon::Kind(IconKind::Radio),
        active: active_tab() == "radio",
        on_click: move |_| active_tab.set("radio".to_string()),
    }
    BottomNavItem {
        label: "Chat".to_string(),
        icon: SvgIcon::Kind(IconKind::MessageSquare),
        active: active_tab() == "chat",
        badge: Some("3".to_string()),
        badge_variant: Some(BadgeVariant::Danger),
        on_click: move |_| active_tab.set("chat".to_string()),
    }
    BottomNavItem {
        label: "Vault".to_string(),
        icon: SvgIcon::Kind(IconKind::Lock),
        active: active_tab() == "vault",
        on_click: move |_| active_tab.set("vault".to_string()),
    }
}"##
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "BottomNavigation & BottomNavItem" }
                Text { dimmed: true, size: TextSize::Sm, "Barra di navigazione inferiore fissa ottimizzata per interfacce mobile da 3 a 5 destinazioni, con micro-glow attivo, transizioni tattili, gestione safe area e badge di notifica integrati." }
            }

            FeatureBanner { feature_name: "bottom_navigation" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6",
                        // Toolbar Controlli
                        div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] grid grid-cols-1 md:grid-cols-3 gap-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Destinazione Attiva:" }
                                div { class: "grid grid-cols-2 gap-1.5",
                                    button {
                                        r#type: "button",
                                        class: if active_tab() == "home" {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| active_tab.set("home".to_string()),
                                        "Home"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if active_tab() == "radio" {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| active_tab.set("radio".to_string()),
                                        "Radio"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if active_tab() == "chat" {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| active_tab.set("chat".to_string()),
                                        "Chat"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if active_tab() == "vault" {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| active_tab.set("vault".to_string()),
                                        "Vault"
                                    }
                                }
                            }

                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Numero Destinazioni (3-5):" }
                                div { class: "flex space-x-2",
                                    button {
                                        r#type: "button",
                                        class: if tab_count() == 3 {
                                            "flex-1 px-2 py-1.5 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "flex-1 px-2 py-1.5 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| tab_count.set(3),
                                        "3 Tab"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if tab_count() == 4 {
                                            "flex-1 px-2 py-1.5 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "flex-1 px-2 py-1.5 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| tab_count.set(4),
                                        "4 Tab"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if tab_count() == 5 {
                                            "flex-1 px-2 py-1.5 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "flex-1 px-2 py-1.5 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| tab_count.set(5),
                                        "5 Tab"
                                    }
                                }
                            }

                            div { class: "flex flex-col justify-center space-y-2 pt-2 md:pt-0 border-t md:border-l border-white/[0.06] md:pl-4",
                                Checkbox {
                                    checked: with_badges,
                                    label: Some("Mostra Badge Notifiche".to_string()),
                                }
                                Checkbox {
                                    checked: with_blur,
                                    label: Some("Backdrop Blur Glassmorphism".to_string()),
                                }
                                Checkbox {
                                    checked: is_vault_disabled,
                                    label: Some("Disabilita Destinazione Vault".to_string()),
                                }
                            }
                        }

                        // Preview Box: Mockup Telefono Mobile Cyberpunk Shadow
                        div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-center",
                            div { class: "flex justify-center",
                                div {
                                    class: "relative bg-black border border-white/10 shadow-2xl overflow-hidden flex flex-col justify-between",
                                    style: "width: 310px; height: 520px; border-radius: 36px;",
                                // Top Bar con Dynamic Island
                                div { class: "relative z-10 pt-3 px-6 flex items-center justify-between text-[11px] font-mono text-zinc-400",
                                    span { "09:41" }
                                    div { class: "w-16 h-3.5 bg-zinc-900 rounded-full" }
                                    div { class: "flex items-center space-x-2",
                                        Icon { kind: IconKind::Wifi, size: 11 }
                                        Icon { kind: IconKind::SignalHigh, size: 11 }
                                    }
                                }

                                // Contenuto Schermo Centrale Reattivo
                                div { class: "p-4 space-y-3 flex-1 overflow-y-auto font-mono text-xs",
                                    div { class: "p-3 rounded-lg bg-white/[0.04] border border-white/[0.06]",
                                        div { class: "text-[10px] text-purple-400 uppercase tracking-widest", "Nodo Mesh Connesso" }
                                        div { class: "text-zinc-100 font-bold mt-0.5", "Shadowfenix Node #01" }
                                        div { class: "text-[11px] text-zinc-500 mt-1", "Latenza: 14ms • P2P Attivo" }
                                    }

                                        div { class: "p-3 rounded-lg bg-white/[0.02] border border-white/[0.04]",
                                            div { class: "text-[10px] text-zinc-400 uppercase", "Destinazione Selezionata:" }
                                            div { class: "text-purple-300 font-bold text-sm uppercase mt-0.5", "{active_tab()}" }
                                        }

                                        div { class: "p-2.5 rounded border border-purple-500/20 bg-purple-950/20 text-[11px] text-purple-200",
                                            "La barra inferiore rimane fissa in basso con supporto nativo alla safe area."
                                        }
                                    }

                                    // Barra di Navigazione Mobile Ancorata
                                    BottomNavigation {
                                        position: Some(BottomNavPosition::Absolute),
                                        safe_area: true,
                                        border: with_border(),
                                        blur: with_blur(),
                                        BottomNavItem {
                                            label: "Home".to_string(),
                                            icon: SvgIcon::Kind(IconKind::Box),
                                            active: active_tab() == "home",
                                            on_click: move |_| active_tab.set("home".to_string()),
                                        }
                                        BottomNavItem {
                                            label: "Radio".to_string(),
                                            icon: SvgIcon::Kind(IconKind::Radio),
                                            active: active_tab() == "radio",
                                            on_click: move |_| active_tab.set("radio".to_string()),
                                        }
                                        BottomNavItem {
                                            label: "Chat".to_string(),
                                            icon: SvgIcon::Kind(IconKind::MessageSquare),
                                            active: active_tab() == "chat",
                                            badge: if with_badges() { Some("3".to_string()) } else { None },
                                            badge_variant: Some(BadgeVariant::Danger),
                                            on_click: move |_| active_tab.set("chat".to_string()),
                                        }
                                        if tab_count() >= 4 {
                                            BottomNavItem {
                                                label: "Vault".to_string(),
                                                icon: SvgIcon::Kind(IconKind::Lock),
                                                active: active_tab() == "vault",
                                                disabled: is_vault_disabled(),
                                                badge: if with_badges() { Some("•".to_string()) } else { None },
                                                badge_variant: Some(BadgeVariant::Success),
                                                on_click: move |_| active_tab.set("vault".to_string()),
                                            }
                                        }
                                        if tab_count() >= 5 {
                                            BottomNavItem {
                                                label: "Setup".to_string(),
                                                icon: SvgIcon::Kind(IconKind::Settings),
                                                active: active_tab() == "setup",
                                                on_click: move |_| active_tab.set("setup".to_string()),
                                            }
                                        }
                                    }
                                }
                            }

                            // Pannello Specifiche Tecniche
                            div { class: "p-6 rounded-xl bg-[#060608] border border-white/[0.06] space-y-4 text-xs font-mono",
                                div { class: "text-purple-400 font-bold uppercase tracking-wider text-[11px]", "Caratteristiche Mobile UX" }
                                ul { class: "space-y-2 text-zinc-300",
                                    li { "• Layout equispaziato flessibile per 3, 4 o 5 destinazioni." }
                                    li { "• Micro-glow al neon viola e pillola luminosa sotto l'icona attiva." }
                                    li { "• Feedback aptico/tattile fluido su click e press (:active scale 0.94)." }
                                    li { "• Supporto nativo safe area: padding-bottom: env(safe-area-inset-bottom, 0px)." }
                                    li { "• Top specular hairline (border-t border-white/10) e glassmorphism." }
                                }
                                div { class: "p-3 rounded bg-white/[0.02] border border-white/[0.04]",
                                    div { class: "text-zinc-500 text-[10px]", "Stato Reattivo:" }
                                    div { class: "text-zinc-200 mt-1", "Tab Attiva: \"{active_tab()}\" • Totale Tab: {tab_count()}" }
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
