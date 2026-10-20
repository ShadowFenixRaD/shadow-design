use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

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
            description: "Mostra la linea sottile inferiore con specular hairline (border-b border-white/8)",
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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Menubar & CommandBar" }
                Text { dimmed: true, size: TextSize::Sm, "Barra dei menu orizzontale in stile applicazione desktop ed IDE. Supporta transizioni rapide su hover una volta aperto un menu, shortcut da tastiera allineate a destra, voci di controllo con spunta, separatori e z-index elevato." }
            }

            FeatureBanner { feature_name: "menubar" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6",
                        // Box Desktop Window Mockup
                        div { class: "rounded-xl bg-[#060608] border border-white/[0.08] shadow-2xl overflow-hidden",
                            // Title bar simulata dell'applicazione desktop
                            div { class: "px-4 py-2.5 bg-black/60 border-b border-white/[0.06] flex items-center justify-between text-xs font-mono select-none",
                                div { class: "flex items-center space-x-2",
                                    span { class: "w-3 h-3 rounded-full inline-block", style: "background-color: rgba(239, 68, 68, 0.8);" }
                                    span { class: "w-3 h-3 rounded-full inline-block", style: "background-color: rgba(234, 179, 8, 0.8);" }
                                    span { class: "w-3 h-3 rounded-full inline-block", style: "background-color: rgba(34, 197, 94, 0.8);" }
                                    span { class: "text-zinc-400 pl-2 font-bold", "Shadow IDE — Mesh Station v0.3" }
                                }
                                div { class: "flex items-center space-x-2 text-[11px] text-zinc-500",
                                    span { class: "w-2 h-2 rounded-full bg-purple-500 inline-block", style: "box-shadow: 0 0 6px #a855f7;" }
                                    span { "ML-KEM ACTIVE" }
                                }
                            }

                            // Barra dei menu principale al di sotto della barra del titolo e sopra la barra di ricerca comandi
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

                            // Barra di ricerca comandi posizionata subito al di sotto
                            div { class: "px-4 py-2 bg-black/60 border-b border-white/[0.06] flex items-center justify-between",
                                div { class: "px-2.5 py-1 rounded bg-white/[0.04] border border-white/[0.06] text-[11px] font-mono text-zinc-400 flex items-center space-x-2 cursor-pointer hover:bg-white/[0.06] transition-colors",
                                    Icon { kind: IconKind::Search, size: 12 }
                                    span { "Cerca Comandi..." }
                                    span { class: "px-1.5 py-0.5 rounded bg-white/[0.06] text-[10px] text-zinc-300", "Ctrl+K" }
                                }
                                div { class: "flex items-center space-x-3 text-[11px] font-mono text-zinc-500",
                                    span { "Station ID: mesh-0492" }
                                    span { class: "text-zinc-600", "•" }
                                    span { class: "text-emerald-400 flex items-center space-x-1",
                                        span { class: "w-1.5 h-1.5 rounded-full bg-emerald-400 inline-block" }
                                        span { "ONLINE" }
                                    }
                                }
                            }

                            // Area di lavoro mockup dell'IDE
                            div { class: "p-6 min-h-[160px] bg-black space-y-4 font-mono text-xs",
                                div { class: "p-4 rounded-lg bg-white/[0.02] border border-white/[0.06] space-y-2",
                                    div { class: "text-zinc-500 uppercase tracking-wider text-[10px]", "Console di Feedback Interattivo:" }
                                    div { class: "text-purple-300 font-bold", "Azione Ricevuta: \"{last_action()}\"" }
                                }

                                div { class: "grid grid-cols-1 md:grid-cols-3 gap-3 text-[11px]",
                                    div { class: "p-3 rounded bg-white/[0.02] border border-white/[0.04] flex items-center justify-between",
                                        span { class: "text-zinc-400", "Terminale:" }
                                        span { class: if show_terminal() { "text-purple-400 font-bold" } else { "text-zinc-600" }, if show_terminal() { "ATTIVO" } else { "NASCOSTO" } }
                                    }
                                    div { class: "p-3 rounded bg-white/[0.02] border border-white/[0.04] flex items-center justify-between",
                                        span { class: "text-zinc-400", "Metriche Mesh:" }
                                        span { class: if show_metrics() { "text-purple-400 font-bold" } else { "text-zinc-600" }, if show_metrics() { "ATTIVO" } else { "NASCOSTO" } }
                                    }
                                    div { class: "p-3 rounded bg-white/[0.02] border border-white/[0.04] flex items-center justify-between",
                                        span { class: "text-zinc-400", "Minimappa:" }
                                        span { class: if show_minimap() { "text-purple-400 font-bold" } else { "text-zinc-600" }, if show_minimap() { "ATTIVO" } else { "NASCOSTO" } }
                                    }
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
