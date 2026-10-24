use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{Stage, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Tabs Component".to_string(),
                description: "Navigazione a schede orizzontali stile Mantine con supporto a icone native, badge di notifica e rendering condizionale fluido.".to_string(),
            }

            FeatureBanner { feature_name: "tabs" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage { start: true, wide: true,
                        div { class: "sc-stack w-full",
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

                            TabPanel { value: "account".to_string(), active: active_tab,
                                div { class: "sc-stack",
                                    h3 { class: "sc-section-title", "Configurazione Profilo Utente" }
                                    p { class: "sc-page-desc", "Gestisci le tue credenziali e le preferenze di interfaccia grafica." }
                                    div { class: "sc-actions",
                                        Badge { label: "Ruolo: Developer Pro".to_string(), variant: BadgeVariant::Primary, pill: true }
                                        Badge { label: "Attivo".to_string(), variant: BadgeVariant::Success, dot: true }
                                    }
                                }
                            }

                            TabPanel { value: "security".to_string(), active: active_tab,
                                div { class: "sc-stack",
                                    h3 { class: "sc-section-title", "Parametri di Protezione e Accesso" }
                                    p { class: "sc-page-desc", "L'autenticazione a due fattori con chiave di sicurezza hardware è raccomandata." }
                                    div { class: "sc-actions",
                                        Button {
                                            size: ButtonSize::Sm,
                                            variant: ButtonVariant::Primary,
                                            left_icon: rsx! { Icon { kind: IconKind::Check, size: 14 } },
                                            "Verifica Sessioni Attive"
                                        }
                                    }
                                }
                            }

                            TabPanel { value: "api".to_string(), active: active_tab,
                                div { class: "sc-stack",
                                    h3 { class: "sc-section-title", "Token di Accesso al Cluster" }
                                    p { class: "sc-page-desc", "Utilizza questo token nelle tue pipeline CI/CD o da linea di comando." }
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
