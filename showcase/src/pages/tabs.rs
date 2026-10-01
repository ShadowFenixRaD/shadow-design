use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn TabsPage() -> Element {
    let active_tab = use_signal(|| "account".to_string());

    let props = vec![
        PropRow {
            name: "TabList",
            r#type: "Component",
            default: "-",
            description: "Contenitore orizzontale per i pulsanti delle schede con sfondo compatto",
        },
        PropRow {
            name: "Tab",
            r#type: "Component",
            default: "-",
            description: "Singolo pulsante di selezione scheda con supporto a icona e badge",
        },
        PropRow {
            name: "value",
            r#type: "String",
            default: "-",
            description: "Identificatore univoco della scheda (es. \"account\", \"security\")",
        },
        PropRow {
            name: "active",
            r#type: "Signal<String>",
            default: "-",
            description: "Signal reattivo bidirezionale che memorizza l'identificatore attivo",
        },
        PropRow {
            name: "TabPanel",
            r#type: "Component",
            default: "-",
            description: "Pannello di contenuto visualizzato solo quando corrisponde al tab attivo",
        },
    ];

    let code_snippet = r#"let mut active_tab = use_signal(|| "account".to_string());

TabList {
    Tab {
        value: "account".to_string(),
        active: active_tab,
        icon: Some(IconKind::Terminal),
        "Profilo"
    }
    Tab {
        value: "security".to_string(),
        active: active_tab,
        icon: Some(IconKind::Eye),
        badge: Some("2FA".to_string()),
        "Sicurezza"
    }
}

TabPanel { value: "account".to_string(), active: active_tab,
    p { "Dettagli dell'account utente." }
}
TabPanel { value: "security".to_string(), active: active_tab,
    p { "Impostazioni di autenticazione a due fattori." }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Tabs Component" }
                Text { dimmed: true, size: TextSize::Sm, "Navigazione a schede orizzontali stile Mantine con supporto a icone native, badge di notifica e rendering condizionale fluido." }
            }

            FeatureBanner { feature_name: "tabs" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-6 rounded-xl bg-[#09090d] border border-white/[0.08] shadow-xl space-y-6",
                        TabList {
                            Tab {
                                value: "account".to_string(),
                                active: active_tab,
                                icon: Some(IconKind::Terminal),
                                "Account"
                            }
                            Tab {
                                value: "security".to_string(),
                                active: active_tab,
                                icon: Some(IconKind::Eye),
                                badge: Some("2FA".to_string()),
                                "Sicurezza"
                            }
                            Tab {
                                value: "api".to_string(),
                                active: active_tab,
                                icon: Some(IconKind::Code),
                                "Chiavi API"
                            }
                        }

                        div { class: "p-5 rounded-xl bg-[#060608] border border-white/[0.06] shadow-inner",
                            TabPanel { value: "account".to_string(), active: active_tab,
                                div { class: "space-y-3",
                                    h4 { class: "text-sm font-semibold text-white", "Configurazione Profilo Utente" }
                                    p { class: "text-xs text-zinc-400", "Gestisci le tue credenziali e le preferenze di interfaccia grafica." }
                                    div { class: "flex items-center gap-2 pt-2",
                                        Badge { label: "Ruolo: Developer Pro".to_string(), variant: BadgeVariant::Primary, pill: true }
                                        Badge { label: "Attivo".to_string(), variant: BadgeVariant::Success, dot: true }
                                    }
                                }
                            }

                            TabPanel { value: "security".to_string(), active: active_tab,
                                div { class: "space-y-3",
                                    h4 { class: "text-sm font-semibold text-white", "Parametri di Protezione e Accesso" }
                                    p { class: "text-xs text-zinc-400", "L'autenticazione a due fattori con chiave di sicurezza hardware è raccomandata." }
                                    Button {
                                        size: ButtonSize::Sm,
                                        variant: ButtonVariant::Primary,
                                        left_icon: rsx! { Icon { kind: IconKind::Check, size: 14 } },
                                        "Verifica Sessioni Attive"
                                    }
                                }
                            }

                            TabPanel { value: "api".to_string(), active: active_tab,
                                div { class: "space-y-3",
                                    h4 { class: "text-sm font-semibold text-white", "Token di Accesso al Cluster" }
                                    p { class: "text-xs text-zinc-400", "Utilizza questo token nelle tue pipeline CI/CD o da linea di comando." }
                                    Code { "sh_live_948fbc923a10488cd723" }
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
