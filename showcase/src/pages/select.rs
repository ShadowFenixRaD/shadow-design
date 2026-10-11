use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn SelectPage() -> Element {
    // Stato per Select singolo standard
    let selected_framework = use_signal(|| Some("dioxus".to_string()));

    // Stato per Select con ricerca e opzioni raggruppate
    let selected_role = use_signal(|| None::<String>);

    // Stato per MultiSelect con chip
    let selected_skills = use_signal(|| vec!["rust".to_string(), "dioxus".to_string()]);

    // Stato per Combobox autocomplete
    let combobox_val = use_signal(|| "Neon".to_string());

    let frameworks_data = vec![
        SelectItem::new("dioxus", "Dioxus 0.7")
            .with_description("Framework reattivo Rust Fullstack Wasm & Desktop"),
        SelectItem::new("actix", "Actix Web")
            .with_description("Server HTTP ad altissime prestazioni per Rust"),
        SelectItem::new("axum", "Axum")
            .with_description("Web framework ergonomico basato sull'ecosistema Tokio"),
        SelectItem::new("tauri", "Tauri 2.0")
            .with_description("Costruzione di app desktop e mobile sicure e snelle"),
        SelectItem::new("yew", "Yew")
            .with_description("Framework web basato su componenti Rust").disabled(),
    ];

    let grouped_roles_data = vec![
        SelectItem::new("lead_arch", "Lead Systems Architect")
            .with_description("Definizione di architetture distribuite")
            .with_group("Engineering & R&D"),
        SelectItem::new("core_rust", "Rust Core Developer")
            .with_description("Sviluppo di crate grafiche e motori asincroni")
            .with_group("Engineering & R&D"),
        SelectItem::new("ui_craft", "UI/UX Tactile Designer")
            .with_description("Creazione di micro-interazioni ed estetica dark")
            .with_group("Design & Aesthetics"),
        SelectItem::new("sec_audit", "Security & Cryptography Auditor")
            .with_description("Verifica crittografica e conformità zero-trust")
            .with_group("Security & Ops"),
    ];

    let skills_data = vec![
        SelectItem::new("rust", "Rust"),
        SelectItem::new("dioxus", "Dioxus"),
        SelectItem::new("wasm", "WebAssembly"),
        SelectItem::new("scss", "SCSS BEM"),
        SelectItem::new("tokio", "Tokio Async"),
        SelectItem::new("crypto", "Cryptography"),
        SelectItem::new("ui_motion", "UI Motion"),
    ];

    let themes_data = vec![
        SelectItem::new("Ultraviolet", "Ultraviolet Shadow"),
        SelectItem::new("Obsidian", "Obsidian Titanium"),
        SelectItem::new("Crimson", "Crimson Phoenix"),
        SelectItem::new("CyberCyan", "Cyber Cyan"),
        SelectItem::new("EmeraldMatrix", "Emerald Matrix"),
        SelectItem::new("CartoonPop", "Cartoon Pop"),
    ];

    let props = vec![
        PropRow {
            name: "data",
            r#type: "Vec<SelectItem>",
            default: "-",
            description: "Collezione di voci selezionabili (supporta value, label, description, group e disabled)",
        },
        PropRow {
            name: "value",
            r#type: "Signal<Option<String>>",
            default: "-",
            description: "Signal reattivo Dioxus 0.7 per il valore selezionato nel Select singolo",
        },
        PropRow {
            name: "values",
            r#type: "Signal<Vec<String>>",
            default: "-",
            description: "Signal reattivo Dioxus 0.7 per l'elenco dei valori selezionati in MultiSelect",
        },
        PropRow {
            name: "placeholder",
            r#type: "Option<String>",
            default: "\"Seleziona un'opzione...\"",
            description: "Testo segnaposto informativo visibile quando non è selezionata alcuna voce",
        },
        PropRow {
            name: "searchable",
            r#type: "bool",
            default: "false",
            description: "Abilita la barra di ricerca live sticky con filtraggio fuzzy istantaneo",
        },
        PropRow {
            name: "clearable",
            r#type: "bool",
            default: "false",
            description: "Mostra un pulsante di cancellazione rapida (X) per azzerare la selezione",
        },
        PropRow {
            name: "creatable",
            r#type: "bool",
            default: "false",
            description: "Consente all'utente di creare dinamicamente voci custom non presenti nei dati iniziali",
        },
        PropRow {
            name: "loading",
            r#type: "bool",
            default: "false",
            description: "Sostituisce l'indicatore a freccia con uno spinner circolare animato",
        },
        PropRow {
            name: "max_selected_display",
            r#type: "Option<usize>",
            default: "None",
            description: "Limite massimo di chip mostrati in MultiSelect prima di riassumere con badge +N",
        },
        PropRow {
            name: "on_change",
            r#type: "Option<EventHandler<...>>",
            default: "None",
            description: "Callback invocata reattivamente ad ogni modifica di selezione",
        },
        PropRow {
            name: "on_create",
            r#type: "Option<EventHandler<String>>",
            default: "None",
            description: "Callback invocata alla creazione di un nuovo valore utente",
        },
    ];

    let code_snippet = r#"let mut selected = use_signal(|| Some("dioxus".to_string()));
let mut multi_selected = use_signal(|| vec!["rust".to_string(), "dioxus".to_string()]);

let data = vec![
    SelectItem::new("dioxus", "Dioxus 0.7")
        .with_description("Framework reattivo Rust Fullstack Wasm"),
    SelectItem::new("actix", "Actix Web")
        .with_description("Server HTTP ad alte prestazioni"),
];

// 1. Select singolo con ricerca e clearable
Select {
    data: data.clone(),
    value: selected,
    label: Some("Framework Primario".to_string()),
    searchable: true,
    clearable: true,
    glow: Some(true),
}

// 2. MultiSelect con chip dismissabili e limite visivo
MultiSelect {
    data: data,
    values: multi_selected,
    label: Some("Competenze Tecniche".to_string()),
    searchable: true,
    clearable: true,
    creatable: true,
    max_selected_display: Some(3),
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Select & MultiSelect" }
                Text { dimmed: true, size: TextSize::Sm, "Selettori a discesa singoli e multipli ad alte prestazioni con ricerca live, chip dismissabili, raggruppamento opzioni, creazione dinamica di valori (creatable) e completamento automatico Combobox." }
            }

            FeatureBanner { feature_name: "select" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-start",
                        // Demo Controlli
                        div { class: "space-y-6",
                            // 1. Select Singolo
                            Select {
                                data: frameworks_data,
                                value: selected_framework,
                                label: Some("Framework di Riferimento".to_string()),
                                description: Some("Seleziona il framework primario per il tuo stack tecnologico".to_string()),
                                placeholder: Some("Seleziona un framework...".to_string()),
                                searchable: true,
                                clearable: true,
                            }

                            // 2. Select Raggruppato con Creatable
                            Select {
                                data: grouped_roles_data,
                                value: selected_role,
                                label: Some("Ruolo Operativo (Con Categorie & Creatable)".to_string()),
                                description: Some("Le voci sono organizzate per dipartimento. Digita per cercare o creare un ruolo custom.".to_string()),
                                placeholder: Some("Scegli un ruolo o digitane uno nuovo...".to_string()),
                                searchable: true,
                                clearable: true,
                                creatable: true,
                            }

                            // 3. MultiSelect con Chip e Limite
                            MultiSelect {
                                data: skills_data,
                                values: selected_skills,
                                label: Some("Competenze Tecniche (MultiSelect)".to_string()),
                                description: Some("Seleziona più tag o creane di nuovi premendo invio".to_string()),
                                placeholder: Some("Aggiungi competenze...".to_string()),
                                searchable: true,
                                clearable: true,
                                creatable: true,
                                max_selected_display: Some(4),
                            }

                            // 4. Combobox Autocomplete
                            Combobox {
                                data: themes_data,
                                value: combobox_val,
                                label: Some("Combobox (Input Autocomplete)".to_string()),
                                description: Some("Digita liberamente o clicca la freccia a destra per vedere i suggerimenti".to_string()),
                                placeholder: Some("Digita il nome di un preset...".to_string()),
                                clearable: true,
                            }
                        }

                        // Telemetria Stato Reattivo
                        div { class: "p-5 rounded-xl bg-[#060608] border border-white/[0.08] space-y-4 font-mono text-xs shadow-inner",
                            p { class: "text-zinc-400 font-semibold uppercase text-[10px] tracking-wider", "Stato Reattivo Signal:" }
                            div { class: "p-3 rounded-lg bg-black/60 border border-white/[0.04] space-y-2.5",
                                div { class: "text-purple-300",
                                    span { class: "text-zinc-500", "selected_framework = " }
                                    if let Some(v) = selected_framework() {
                                        "Some(\"{v}\")"
                                    } else {
                                        "None"
                                    }
                                }
                                div { class: "text-cyan-300",
                                    span { class: "text-zinc-500", "selected_role = " }
                                    if let Some(v) = selected_role() {
                                        "Some(\"{v}\")"
                                    } else {
                                        "None"
                                    }
                                }
                                div { class: "text-emerald-300",
                                    span { class: "text-zinc-500", "selected_skills = " }
                                    "{selected_skills():?}"
                                }
                                div { class: "text-amber-300",
                                    span { class: "text-zinc-500", "combobox_val = " }
                                    "\"{combobox_val}\""
                                }
                            }
                            span { class: "text-[11px] text-zinc-500 block leading-normal",
                                "La selezione o cancellazione di opzioni, sia singole che multiple, si riflette all'istante nei rispettivi Signal<Option<String>> e Signal<Vec<String>>."
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Sezione con Varianti ed Error States
            Card {
                title: "Stati Particolari: Disabilitato, Errore e Loading".to_string(),
                subtitle: "Supporto integrato a feedback visivo e stati transitori".to_string(),
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-6 pt-2",
                    Select {
                        data: vec![SelectItem::new("1", "Opzione")],
                        value: use_signal(|| Some("1".to_string())),
                        label: Some("Stato Disabilitato".to_string()),
                        disabled: true,
                    }

                    Select {
                        data: vec![SelectItem::new("1", "Opzione")],
                        value: use_signal(|| None::<String>),
                        label: Some("Stato di Errore".to_string()),
                        error: Some("Selezione obbligatoria".to_string()),
                    }

                    Select {
                        data: vec![SelectItem::new("1", "Opzione")],
                        value: use_signal(|| None::<String>),
                        label: Some("Caricamento Asincrono".to_string()),
                        loading: true,
                    }
                }
            }
        }
    }
}
