//! Story: Tabs, TabList, Tab, TabPanel (feature `tabs`). Stateful playground (`TabsDemo { knobs }`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const BADGES: [Option<&str>; 3] = [None, Some("3"), Some("12")];
const BADGE_OPTS: &[ChoiceOption] = &[opt("Nessuno", "None"), opt("3", "Some(\"3\".to_string())"), opt("12", "Some(\"12\".to_string())")];

const PRELUDE: &str = r#"let active = use_signal(|| "panoramica".to_string());"#;

const CHILDREN: &str = r#"TabList { aria_label: Some("{aria_label}".to_string()),
    Tab { value: "panoramica", active, icon: Some(IconKind::Layers), "Panoramica" }
    Tab { value: "nodi", active, icon: Some(IconKind::Server), badge: {badge}, "Nodi" }
    Tab { value: "log", active, icon: Some(IconKind::Terminal), disabled: {disabled}, "Log" }
}
TabPanel { value: "panoramica", active,
    Text { size: TextSize::Sm, dimmed: true, "12 nodi, latenza media 18 ms, nessun incidente aperto." }
}
TabPanel { value: "nodi", active,
    Text { size: TextSize::Sm, dimmed: true, "edge-01, edge-03, edge-07 e altri 9 nodi in tre regioni." }
}
TabPanel { value: "log", active,
    Text { size: TextSize::Sm, dimmed: true, "09:41 · rotazione chiavi completata." }
}"#;

#[component]
fn TabsDemo(knobs: Knobs) -> Element {
    let active = use_signal(|| "panoramica".to_string());
    let badge = knobs.pick("badge", &BADGES).map(str::to_string);
    rsx! {
        Tabs {
            TabList { aria_label: Some(knobs.text("aria_label")),
                Tab { value: "panoramica", active, icon: Some(IconKind::Layers), "Panoramica" }
                Tab { value: "nodi", active, icon: Some(IconKind::Server), badge, "Nodi" }
                Tab { value: "log", active, icon: Some(IconKind::Terminal), disabled: knobs.bool("disabled"), "Log" }
            }
            TabPanel { value: "panoramica", active,
                Text { size: TextSize::Sm, dimmed: true, "12 nodi, latenza media 18 ms, nessun incidente aperto." }
            }
            TabPanel { value: "nodi", active,
                Text { size: TextSize::Sm, dimmed: true, "edge-01, edge-03, edge-07 e altri 9 nodi in tre regioni." }
            }
            TabPanel { value: "log", active,
                Text { size: TextSize::Sm, dimmed: true, "09:41 · rotazione chiavi completata." }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        TabsDemo { knobs: k.clone() }
    }
}

#[component]
fn Plain() -> Element {
    let active = use_signal(|| "mensile".to_string());
    rsx! {
        Tabs {
            TabList { aria_label: Some("Periodo di fatturazione".to_string()),
                Tab { value: "mensile", active, "Mensile" }
                Tab { value: "annuale", active, "Annuale" }
            }
            TabPanel { value: "mensile", active,
                Text { "29 € al mese, disdici quando vuoi." }
            }
            TabPanel { value: "annuale", active,
                Text { "290 € all'anno: due mesi gratis." }
            }
        }
    }
}
const PLAIN_CODE: &str = r#"let active = use_signal(|| "mensile".to_string());

Tabs {
    TabList { aria_label: Some("Periodo di fatturazione".to_string()),
        Tab { value: "mensile", active, "Mensile" }
        Tab { value: "annuale", active, "Annuale" }
    }
    TabPanel { value: "mensile", active,
        Text { "29 € al mese, disdici quando vuoi." }
    }
    TabPanel { value: "annuale", active,
        Text { "290 € all'anno: due mesi gratis." }
    }
}"#;

#[component]
fn KeepMounted() -> Element {
    let active = use_signal(|| "bozza".to_string());
    let draft = use_signal(|| "Rilascio di venerdì: ".to_string());
    rsx! {
        Tabs {
            TabList { aria_label: Some("Editor".to_string()),
                Tab { value: "bozza", active, "Bozza" }
                Tab { value: "anteprima", active, "Anteprima" }
            }
            TabPanel { value: "bozza", active, keep_mounted: true,
                TextInput { value: draft, label: Some("Titolo".to_string()) }
            }
            TabPanel { value: "anteprima", active, keep_mounted: true,
                Text { "{draft}" }
            }
        }
    }
}
const KEEP_MOUNTED_CODE: &str = r#"let active = use_signal(|| "bozza".to_string());
let draft = use_signal(|| "Rilascio di venerdì: ".to_string());

Tabs {
    TabList { aria_label: Some("Editor".to_string()),
        Tab { value: "bozza", active, "Bozza" }
        Tab { value: "anteprima", active, "Anteprima" }
    }
    TabPanel { value: "bozza", active, keep_mounted: true,
        TextInput { value: draft, label: Some("Titolo".to_string()) }
    }
    TabPanel { value: "anteprima", active, keep_mounted: true,
        Text { "{draft}" }
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::InboxPane { compact: true }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/inbox_pane.rs (estratto): le cartelle filtrano la lista dei messaggi.
let tab = use_signal(|| "all".to_string());

Tabs {
    TabList { aria_label: Some("Cartelle".to_string()),
        Tab { value: "all", active: tab, "Tutti" }
        Tab { value: "unread", active: tab, badge: Some(unread), "Non letti" }
        Tab { value: "mentions", active: tab, "Menzioni" }
    }
}"#;

const TABS_PROPS: &[PropDoc] = &[
    prop("id", "Option<String>", "None", "Id base: collega tab e pannelli (aria-controls, aria-labelledby)."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
    prop("children", "Element", "-", "Un TabList e i TabPanel."),
];
const LIST_PROPS: &[PropDoc] = &[
    prop("aria_label", "Option<String>", "None", "Nome della tablist per i lettori di schermo."),
    prop("id / class", "Option<String> / String", "None / \"\"", "Id e classi aggiuntive."),
];
const TAB_PROPS: &[PropDoc] = &[
    prop("value", "String", "-", "Chiave della scheda, uguale a quella del pannello."),
    prop("active", "Signal<String>", "-", "Scheda attiva, condivisa da tab e pannelli."),
    prop("icon", "Option<IconKind>", "None", "Icona prima dell'etichetta."),
    prop("badge", "Option<String>", "None", "Contatore dopo l'etichetta."),
    prop("disabled", "bool", "false", "Scheda non selezionabile, saltata dalle frecce."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const PANEL_PROPS: &[PropDoc] = &[
    prop("value", "String", "-", "Chiave della scheda che lo mostra."),
    prop("active", "Signal<String>", "-", "Lo stesso Signal dei Tab."),
    prop("keep_mounted", "bool", "false", "Nasconde invece di smontare: conserva lo stato dei figli."),
    prop("id / class", "Option<String> / String", "None / \"\"", "Id e classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "tabs",
        name: "Tabs",
        category: Category::Navigazione,
        features: &["tabs"],
        icon: IconKind::Layers,
        tagline: "Schede per alternare pannelli di contenuto nella stessa vista.",
        keywords: &["tabs", "schede", "tablist", "pannelli", "linguette", "tab panel"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Tabs",
            controls: vec![
                Control::text("aria_label", "Nome della tablist", "Sezioni del nodo").emit(Emit::Skip),
                Control::choice("badge", "Badge su Nodi", BADGE_OPTS, 1).emit(Emit::Skip),
                Control::bool("disabled", "Log disabilitato", false).emit(Emit::Skip),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &[],
            children_code: Some(CHILDREN),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "plain",
                title: "Solo testo",
                description: "Due schede senza icone.",
                code: PLAIN_CODE,
                render: || rsx! { Plain {} },
                span: Span::Half,
            },
            Example {
                id: "keep-mounted",
                title: "Pannelli persistenti",
                description: "keep_mounted conserva lo stato del campo tra un cambio e l'altro.",
                code: KEEP_MOUNTED_CODE,
                render: || rsx! { KeepMounted {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Le cartelle della casella operatori filtrano i messaggi.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Tabs", rows: TABS_PROPS },
            PropGroup { component: "TabList", rows: LIST_PROPS },
            PropGroup { component: "Tab", rows: TAB_PROPS },
            PropGroup { component: "TabPanel", rows: PANEL_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Track di item (§3.4) con selezione sd-selected (§3.7)",
            depth: "Well (track) · item selezionato con glow-sm",
            tokens: &["--sd-well", "--sd-border", "--sd-item-h", "--sd-accent-soft", "--sd-accent-border", "--sd-accent-strong", "--sd-glow-sm"],
            notes: "Una sola scheda selezionata per gruppo; il peso del testo non cambia.",
        }),
        a11y: &[
            "Pattern ARIA tabs: role=tablist con aria-orientation, role=tab con aria-selected e aria-controls.",
            "Frecce sinistra e destra, Home ed End spostano focus e selezione tra le schede abilitate.",
            "Ogni pannello è role=tabpanel, aria-labelledby verso la sua scheda e raggiungibile con Tab.",
            "Le schede disabilitate hanno aria-disabled e vengono saltate dalle frecce.",
        ],
        related: &["segmented-controls", "accordion", "nav-link", "bottom-navigation"],
    }
}
