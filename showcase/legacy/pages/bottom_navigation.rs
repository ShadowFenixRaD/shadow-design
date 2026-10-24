use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, Counter, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn BottomNavigationPage() -> Element {
    let mut active_tab = use_signal(|| "home".to_string());
    let tab_count = use_signal(|| 4usize);
    let with_safe_area = use_signal(|| true);
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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "BottomNavigation & BottomNavItem".to_string(),
                description: "Barra di navigazione inferiore fissa ottimizzata per interfacce mobile da 3 a 5 destinazioni, con micro-glow attivo, transizioni tattili, gestione safe area e badge di notifica integrati.".to_string(),
            }

            FeatureBanner { feature_name: "bottom_navigation" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            div {
                                class: "sc-demo-frame",
                                style: "position: relative; display: flex; flex-direction: column; width: 100%; max-width: 360px; height: 600px; --sd-safe-area-bottom: 22px;",
                                div { class: "sc-title-row", style: "padding: var(--sd-space-3) var(--sd-space-4);",
                                    span { class: "sc-subtle-text sc-mono", "09:41" }
                                    div { class: "sc-actions",
                                        Icon { kind: IconKind::Wifi, size: 11 }
                                        Icon { kind: IconKind::SignalHigh, size: 11 }
                                    }
                                }

                                div {
                                    class: "sc-stack",
                                    style: "flex: 1; overflow-y: auto; padding: var(--sd-space-4); padding-bottom: 7rem;",
                                    Card {
                                        title: "Nodo Mesh Connesso".to_string(),
                                        subtitle: "Latenza: 14ms • P2P Attivo".to_string(),
                                        span { class: "sc-section-title", "Shadowfenix Node #01" }
                                    }
                                    Card { title: "Destinazione Selezionata".to_string(),
                                        span { class: "sc-accent-text sc-mono", "{active_tab()}" }
                                    }
                                    Alert {
                                        variant: AlertVariant::Info,
                                        "La barra inferiore rimane fissa in basso con supporto nativo alla safe area."
                                    }
                                }

                                BottomNavigation {
                                    position: Some(BottomNavPosition::Absolute),
                                    safe_area: with_safe_area(),
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
                            Counter { label: "Tab attiva".to_string(), value: format!("{} / {} tab", active_tab(), tab_count()) }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Destinazione Attiva".to_string(),
                                options: {
                                    let mut o = vec![
                                        ("home".to_string(), "Home".to_string()),
                                        ("radio".to_string(), "Radio".to_string()),
                                        ("chat".to_string(), "Chat".to_string()),
                                    ];
                                    if tab_count() >= 4 {
                                        o.push(("vault".to_string(), "Vault".to_string()));
                                    }
                                    if tab_count() >= 5 {
                                        o.push(("setup".to_string(), "Setup".to_string()));
                                    }
                                    o
                                },
                                value: active_tab,
                            }
                            ChoicePicker {
                                label: "Numero Destinazioni".to_string(),
                                options: vec![
                                    (3usize, "3 Tab".to_string()),
                                    (4usize, "4 Tab".to_string()),
                                    (5usize, "5 Tab".to_string()),
                                ],
                                value: tab_count,
                            }
                            hr { class: "sc-control-divider" }
                            ToggleControl { label: "Safe Area Inferiore (Home Bar)".to_string(), checked: with_safe_area }
                            ToggleControl { label: "Mostra Badge Notifiche".to_string(), checked: with_badges }
                            ToggleControl { label: "Top Specular Hairline".to_string(), checked: with_border }
                            ToggleControl { label: "Backdrop Blur Glassmorphism".to_string(), checked: with_blur }
                            ToggleControl { label: "Disabilita Destinazione Vault".to_string(), checked: is_vault_disabled }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
