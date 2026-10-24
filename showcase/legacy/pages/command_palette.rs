use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{Counter, FeatureBanner, PageHeader, PropRow, ShowcaseTabs, Stage};

#[component]
pub fn CommandPalettePage() -> Element {
    let mut opened = use_signal(|| false);
    let mut last_selected = use_signal(|| "Nessuna azione selezionata".to_string());


    let actions = vec![
        CommandAction::new("home", "Vai alla Home")
            .with_description("Ritorna alla schermata principale di Shadow Design")
            .with_category("Navigazione")
            .with_shortcut("Ctrl+H")
            .with_icon(IconKind::Terminal)
            .with_keywords(["dashboard", "inizio"]),
        CommandAction::new("buttons", "Showcase Pulsanti")
            .with_description("Esplora varianti, glow e dimensioni dei bottoni")
            .with_category("Navigazione")
            .with_shortcut("Ctrl+B")
            .with_icon(IconKind::Box)
            .with_keywords(["button", "cta"]),
        CommandAction::new("theme_dark", "Preset Ultraviolet Shadow")
            .with_description("Attiva il tema viola fluorescente predefinito")
            .with_category("Temi & Aspetto")
            .with_icon(IconKind::Palette)
            .with_keywords(["violet", "dark", "neon"]),
        CommandAction::new("theme_crimson", "Preset Crimson Phoenix")
            .with_description("Attiva la palette cremisi fenice")
            .with_category("Temi & Aspetto")
            .with_icon(IconKind::Flame)
            .with_keywords(["rosso", "red", "crimson"]),
        CommandAction::new("copy_link", "Copia Link Condivisione")
            .with_description("Salva l'URL della documentazione negli appunti")
            .with_category("Azioni")
            .with_shortcut("Ctrl+C")
            .with_icon(IconKind::Share2)
            .with_keywords(["clipboard", "share"]),
        CommandAction::new("settings", "Impostazioni Studio")
            .with_description("Configura layout, font Kanit e scala raggi")
            .with_category("Sistema")
            .with_shortcut("Ctrl+,")
            .with_icon(IconKind::Settings)
            .with_keywords(["preferenze", "config"]),
    ];

    let props = vec![
        PropRow {
            name: "opened",
            r#type: "Signal<bool>",
            default: "-",
            description: "Segnale reattivo che controlla la visualizzazione del modal",
        },
        PropRow {
            name: "actions",
            r#type: "Vec<CommandAction>",
            default: "vec![]",
            description: "Lista delle azioni disponibili con titoli, descrizioni, categorie e icone",
        },
        PropRow {
            name: "placeholder",
            r#type: "String",
            default: "\"Cerca comandi o naviga...\"",
            description: "Testo segnaposto della barra di ricerca rapida",
        },
        PropRow {
            name: "on_select",
            r#type: "Option<EventHandler<CommandAction>>",
            default: "None",
            description: "Callback scatenato alla conferma di un'azione tramite click o Invio",
        },
        PropRow {
            name: "close_on_select",
            r#type: "bool",
            default: "true",
            description: "Se chiudere automaticamente la modale dopo la selezione",
        },
    ];

    let code_snippet = r#"let mut opened = use_signal(|| false);

CommandPalette {
    opened: opened,
    actions: vec![
        CommandAction::new("home", "Vai alla Home")
            .with_category("Navigazione")
            .with_shortcut("Ctrl+H")
            .with_icon(IconKind::Terminal),
        CommandAction::new("theme", "Cambia Tema")
            .with_category("Aspetto")
            .with_icon(IconKind::Palette),
    ],
    on_select: move |action: CommandAction| {
        println!("Eseguo azione: {}", action.title);
    },
}"#
    .to_string();

    let actions_for_palette = actions.clone();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "CommandPalette (Spotlight)".to_string(),
                description: "Modal di ricerca e navigazione universale ad alta efficienza azionabile da tastiera, con raggruppamento per categoria e fuzzy matching.".to_string(),
            }

            FeatureBanner { feature_name: "command_palette" }

            ShowcaseTabs {
                preview: rsx! {
                    Stage {
                        p { class: "sc-muted-text",
                            "Premi Ctrl+K da tastiera o clicca sul pulsante. Naviga con frecce ↑/↓, premi Invio per confermare ed Esc per chiudere."
                        }

                        div { class: "sc-actions",
                            Button {
                                variant: ButtonVariant::Primary,
                                left_icon: rsx! { Icon { kind: IconKind::Search, size: 16 } },
                                onclick: move |_| {
                                    let cur = *opened.read();
                                    opened.set(!cur);
                                },
                                "Apri Command Palette"
                            }
                            Kbd { "Ctrl" }
                            span { class: "sc-subtle-text", "+" }
                            Kbd { "K" }
                        }

                        Counter { label: "Ultima azione eseguita".to_string(), value: last_selected() }

                        CommandPalette {
                            opened: opened,
                            actions: actions_for_palette,
                            on_select: move |action: CommandAction| {
                                last_selected.set(format!("{} (ID: {})", action.title, action.id));
                            },
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
