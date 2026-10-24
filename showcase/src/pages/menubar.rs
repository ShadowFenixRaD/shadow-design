use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{Counter, FeatureBanner, PageHeader, Stage, PropRow, ShowcaseTabs};

#[component]
pub fn MenubarPage() -> Element {
    let mut last_action = use_signal(|| "Nessuna azione eseguita".to_string());
    let mut show_terminal = use_signal(|| true);
    let mut show_metrics = use_signal(|| true);
    let mut show_minimap = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "border",
            r#type: "bool",
            default: "true",
            description: "Mostra la linea sottile inferiore con specular hairline",
        },
        PropRow {
            name: "children",
            r#type: "Element",
            default: "-",
            description: "Elenco dei componenti MenubarMenu contenenti MenubarTrigger e MenubarContent",
        },
    ];

    let code_snippet = r##"Menubar {
    border: true,
    MenubarMenu {
        id: Some("file".to_string()),
        MenubarTrigger { "File" }
        MenubarContent {
            MenubarItem {
                label: Some("Nuovo File".to_string()),
                shortcut: Some("Ctrl+N".to_string()),
                icon: Some(SvgIcon::Kind(IconKind::Code)),
                on_click: move |_| println!("Nuovo file!"),
            }
            MenubarItem {
                label: Some("Salva".to_string()),
                shortcut: Some("Ctrl+S".to_string()),
                on_click: move |_| println!("Salva!"),
            }
            MenubarSeparator {}
            MenubarItem {
                label: Some("Esci".to_string()),
                shortcut: Some("Alt+F4".to_string()),
            }
        }
    }
    MenubarMenu {
        id: Some("view".to_string()),
        MenubarTrigger { "Visualizza" }
        MenubarContent {
            MenubarItem {
                label: Some("Terminale".to_string()),
                checked: Some(true),
                shortcut: Some("Ctrl+`".to_string()),
            }
        }
    }
}"##
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Menubar & CommandBar".to_string(),
                description: "Barra dei menu orizzontale in stile applicazione desktop ed IDE. Supporta transizioni rapide su hover una volta aperto un menu, shortcut da tastiera allineate a destra, voci di controllo con spunta, separatori e z-index elevato.".to_string(),
            }

            FeatureBanner { feature_name: "menubar" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage { start: true, wide: true,
                        div { class: "sc-stack w-full",
                    Menubar {
                        border: true,

                        // Menu File
                        MenubarMenu {
                            id: Some("file".to_string()),
                            MenubarTrigger { "File" }
                            MenubarContent {
                                MenubarItem {
                                    label: Some("Nuovo Nodo Mesh".to_string()),
                                    shortcut: Some("Ctrl+N".to_string()),
                                    icon: Some(SvgIcon::Kind(IconKind::Radio)),
                                    on_click: move |_| last_action.set("File -> Nuovo Nodo Mesh".to_string()),
                                }
                                MenubarItem {
                                    label: Some("Apri Vault Crittografico".to_string()),
                                    shortcut: Some("Ctrl+O".to_string()),
                                    icon: Some(SvgIcon::Kind(IconKind::Lock)),
                                    on_click: move |_| last_action.set("File -> Apri Vault Crittografico".to_string()),
                                }
                                MenubarItem {
                                    label: Some("Salva Configurazione".to_string()),
                                    shortcut: Some("Ctrl+S".to_string()),
                                    on_click: move |_| last_action.set("File -> Salva Configurazione (Ctrl+S)".to_string()),
                                }
                                MenubarSeparator {}
                                MenubarItem {
                                    label: Some("Esporta Certificato X.509".to_string()),
                                    on_click: move |_| last_action.set("File -> Esporta Certificato".to_string()),
                                }
                                MenubarSeparator {}
                                MenubarItem {
                                    label: Some("Chiudi Sessione".to_string()),
                                    shortcut: Some("Alt+F4".to_string()),
                                    on_click: move |_| last_action.set("File -> Chiudi Sessione".to_string()),
                                }
                            }
                        }

                        // Menu Modifica
                        MenubarMenu {
                            id: Some("edit".to_string()),
                            MenubarTrigger { "Modifica" }
                            MenubarContent {
                                MenubarItem {
                                    label: Some("Annulla".to_string()),
                                    shortcut: Some("Ctrl+Z".to_string()),
                                    on_click: move |_| last_action.set("Modifica -> Annulla".to_string()),
                                }
                                MenubarItem {
                                    label: Some("Ripeti".to_string()),
                                    shortcut: Some("Ctrl+Y".to_string()),
                                    on_click: move |_| last_action.set("Modifica -> Ripeti".to_string()),
                                }
                                MenubarSeparator {}
                                MenubarItem {
                                    label: Some("Taglia Chiave".to_string()),
                                    shortcut: Some("Ctrl+X".to_string()),
                                    on_click: move |_| last_action.set("Modifica -> Taglia".to_string()),
                                }
                                MenubarItem {
                                    label: Some("Copia Impronta SHA-256".to_string()),
                                    shortcut: Some("Ctrl+C".to_string()),
                                    on_click: move |_| last_action.set("Modifica -> Copia Impronta".to_string()),
                                }
                                MenubarItem {
                                    label: Some("Incolla".to_string()),
                                    shortcut: Some("Ctrl+V".to_string()),
                                    on_click: move |_| last_action.set("Modifica -> Incolla".to_string()),
                                }
                            }
                        }

                        // Menu Visualizza con Checkbox states
                        MenubarMenu {
                            id: Some("view".to_string()),
                            MenubarTrigger { "Visualizza" }
                            MenubarContent {
                                MenubarItem {
                                    label: Some("Terminale di Debug".to_string()),
                                    checked: Some(show_terminal()),
                                    shortcut: Some("Ctrl+`".to_string()),
                                    on_click: move |_| {
                                        let next = !show_terminal();
                                        show_terminal.set(next);
                                        last_action.set(format!("Visualizza -> Toggle Terminale ({next})"));
                                    },
                                }
                                MenubarItem {
                                    label: Some("Metriche Latenza Rete".to_string()),
                                    checked: Some(show_metrics()),
                                    on_click: move |_| {
                                        let next = !show_metrics();
                                        show_metrics.set(next);
                                        last_action.set(format!("Visualizza -> Toggle Metriche ({next})"));
                                    },
                                }
                                MenubarItem {
                                    label: Some("Minimappa Codice".to_string()),
                                    checked: Some(show_minimap()),
                                    on_click: move |_| {
                                        let next = !show_minimap();
                                        show_minimap.set(next);
                                        last_action.set(format!("Visualizza -> Toggle Minimappa ({next})"));
                                    },
                                }
                            }
                        }

                        // Menu Sicurezza
                        MenubarMenu {
                            id: Some("security".to_string()),
                            MenubarTrigger { "Sicurezza" }
                            MenubarContent {
                                MenubarItem {
                                    label: Some("Genera Coppia Chiavi ML-KEM".to_string()),
                                    icon: Some(SvgIcon::Kind(IconKind::Key)),
                                    on_click: move |_| last_action.set("Sicurezza -> Genera Chiavi ML-KEM".to_string()),
                                }
                                MenubarItem {
                                    label: Some("Verifica Integrità HSM".to_string()),
                                    icon: Some(SvgIcon::Kind(IconKind::ShieldCheck)),
                                    on_click: move |_| last_action.set("Sicurezza -> Verifica HSM".to_string()),
                                }
                                MenubarSeparator {}
                                MenubarItem {
                                    label: Some("Revoca Autorizzazione Peer".to_string()),
                                    disabled: true,
                                    on_click: move |_| last_action.set("Sicurezza -> Revoca".to_string()),
                                }
                            }
                        }
                    }

                            div { class: "sc-actions",
                                Counter { label: "Azione ricevuta".to_string(), value: last_action() }
                            }
                            div { class: "sc-actions",
                                Badge {
                                    label: if show_terminal() { "Terminale: attivo".to_string() } else { "Terminale: nascosto".to_string() },
                                    variant: if show_terminal() { BadgeVariant::Primary } else { BadgeVariant::Default },
                                }
                                Badge {
                                    label: if show_metrics() { "Metriche mesh: attive".to_string() } else { "Metriche mesh: nascoste".to_string() },
                                    variant: if show_metrics() { BadgeVariant::Primary } else { BadgeVariant::Default },
                                }
                                Badge {
                                    label: if show_minimap() { "Minimappa: attiva".to_string() } else { "Minimappa: nascosta".to_string() },
                                    variant: if show_minimap() { BadgeVariant::Primary } else { BadgeVariant::Default },
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
