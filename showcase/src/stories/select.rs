//! Story: Select, MultiSelect, Combobox (feature `select`). Reference story for a stateful,
//! compound component (REVAMP §4.10): the playground goes through `SelectDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

fn frameworks() -> Vec<SelectItem> {
    vec![
        SelectItem::new("dioxus", "Dioxus 0.7").with_description("Web, desktop e mobile da un solo codice"),
        SelectItem::new("axum", "Axum").with_description("Web framework su Tokio"),
        SelectItem::new("tauri", "Tauri 2").with_description("App desktop e mobile leggere"),
        SelectItem::new("bevy", "Bevy").with_description("Motore di gioco ECS"),
        SelectItem::new("yew", "Yew").with_description("Non più mantenuto nello stack").disabled(),
    ]
}

/// Printed before the `Select {` block: the same signal and data the live demo uses.
const PRELUDE: &str = r#"let value = use_signal(|| Some("dioxus".to_string()));
let data = vec![
    SelectItem::new("dioxus", "Dioxus 0.7").with_description("Web, desktop e mobile da un solo codice"),
    SelectItem::new("axum", "Axum").with_description("Web framework su Tokio"),
    SelectItem::new("tauri", "Tauri 2").with_description("App desktop e mobile leggere"),
    SelectItem::new("bevy", "Bevy").with_description("Motore di gioco ECS"),
    SelectItem::new("yew", "Yew").with_description("Non più mantenuto nello stack").disabled(),
];"#;

/// Stateful demo: owns the signal the playground cannot (render is a plain fn).
#[component]
fn SelectDemo(knobs: Knobs) -> Element {
    let value = use_signal(|| Some("dioxus".to_string()));
    let error = knobs.bool("error").then(|| "Seleziona un framework".to_string());
    rsx! {
        Select {
            data: frameworks(),
            value,
            label: Some(knobs.text("label")),
            description: Some(knobs.text("description")),
            placeholder: Some(knobs.text("placeholder")),
            searchable: knobs.bool("searchable"),
            clearable: knobs.bool("clearable"),
            creatable: knobs.bool("creatable"),
            disabled: knobs.bool("disabled"),
            loading: knobs.bool("loading"),
            error,
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        SelectDemo { knobs: k.clone() }
    }
}

#[component]
fn Multi() -> Element {
    let values = use_signal(|| vec!["rust".to_string(), "dioxus".to_string()]);
    rsx! {
        MultiSelect {
            data: vec![
                SelectItem::new("rust", "Rust"),
                SelectItem::new("dioxus", "Dioxus"),
                SelectItem::new("wasm", "WebAssembly"),
                SelectItem::new("scss", "SCSS"),
                SelectItem::new("tokio", "Tokio"),
            ],
            values,
            label: Some("Competenze".to_string()),
            searchable: true,
            clearable: true,
            creatable: true,
            max_selected_display: Some(3),
        }
    }
}
const MULTI_CODE: &str = r#"let values = use_signal(|| vec!["rust".to_string(), "dioxus".to_string()]);

MultiSelect {
    data: vec![
        SelectItem::new("rust", "Rust"),
        SelectItem::new("dioxus", "Dioxus"),
        SelectItem::new("wasm", "WebAssembly"),
        SelectItem::new("scss", "SCSS"),
        SelectItem::new("tokio", "Tokio"),
    ],
    values,
    label: Some("Competenze".to_string()),
    searchable: true,
    clearable: true,
    creatable: true,
    max_selected_display: Some(3),
}"#;

#[component]
fn Combo() -> Element {
    let value = use_signal(|| "Ultraviolet".to_string());
    rsx! {
        Combobox {
            data: vec![
                SelectItem::new("Ultraviolet", "Ultraviolet"),
                SelectItem::new("Obsidian", "Obsidian"),
                SelectItem::new("Crimson", "Crimson"),
                SelectItem::new("Cyan", "Cyan"),
                SelectItem::new("Emerald", "Emerald"),
            ],
            value,
            label: Some("Preset".to_string()),
            description: Some("Digita o scegli un suggerimento".to_string()),
            clearable: true,
        }
    }
}
const COMBO_CODE: &str = r#"let value = use_signal(|| "Ultraviolet".to_string());

Combobox {
    data: vec![
        SelectItem::new("Ultraviolet", "Ultraviolet"),
        SelectItem::new("Obsidian", "Obsidian"),
        SelectItem::new("Crimson", "Crimson"),
        SelectItem::new("Cyan", "Cyan"),
        SelectItem::new("Emerald", "Emerald"),
    ],
    value,
    label: Some("Preset".to_string()),
    description: Some("Digita o scegli un suggerimento".to_string()),
    clearable: true,
}"#;

#[component]
fn Grouped() -> Element {
    let value = use_signal(|| None::<String>);
    rsx! {
        Select {
            data: vec![
                SelectItem::new("arch", "Systems Architect").with_group("Engineering"),
                SelectItem::new("core", "Rust Core Developer").with_group("Engineering"),
                SelectItem::new("ui", "Tactile UI Designer").with_group("Design"),
                SelectItem::new("sec", "Security Auditor").with_group("Security"),
            ],
            value,
            label: Some("Ruolo".to_string()),
            placeholder: Some("Scegli o crea un ruolo".to_string()),
            searchable: true,
            creatable: true,
        }
    }
}
const GROUPED_CODE: &str = r#"let value = use_signal(|| None::<String>);

Select {
    data: vec![
        SelectItem::new("arch", "Systems Architect").with_group("Engineering"),
        SelectItem::new("core", "Rust Core Developer").with_group("Engineering"),
        SelectItem::new("ui", "Tactile UI Designer").with_group("Design"),
        SelectItem::new("sec", "Security Auditor").with_group("Security"),
    ],
    value,
    label: Some("Ruolo".to_string()),
    placeholder: Some("Scegli o crea un ruolo".to_string()),
    searchable: true,
    creatable: true,
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::SettingsForm { compact: true }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/settings_form.rs (estratto): Select e MultiSelect nei Fieldset, accanto a Switch, Slider e Button.
Fieldset { legend: "Identità".to_string(),
    Select { data: regions(), value: region, label: Some("Regione".to_string()), searchable: true }
}
Fieldset { legend: "Rete".to_string(),
    MultiSelect { data: protocols(), values: protocols_on, label: Some("Protocolli".to_string()), clearable: true }
}"#;

const SELECT_PROPS: &[PropDoc] = &[
    prop("data", "Vec<SelectItem>", "-", "Opzioni: value, label, description, group, disabled."),
    prop("value", "Signal<Option<String>>", "-", "Valore selezionato; None quando vuoto."),
    prop("label / description / placeholder", "Option<String>", "None", "Etichetta, aiuto e segnaposto (placeholder da Locale se assente)."),
    prop("search_placeholder / nothing_found", "Option<String>", "None", "Testi del campo di ricerca e dello stato vuoto (da Locale se assenti)."),
    prop("searchable", "bool", "false", "Campo di ricerca nel menu."),
    prop("clearable", "bool", "false", "Pulsante X per azzerare."),
    prop("creatable", "bool", "false", "Crea una voce dal testo cercato (on_create)."),
    prop("disabled / loading", "bool", "false", "Disabilitato; spinner al posto del chevron."),
    prop("error", "Option<String>", "None", "Bordo danger, aria-invalid e messaggio."),
    prop("glow", "Option<bool>", "None", "Glow esplicito sul trigger."),
    prop("on_change", "Option<EventHandler<Option<String>>>", "None", "Cambio selezione (None su clear)."),
    prop("on_create", "Option<EventHandler<String>>", "None", "Nuovo valore creato."),
];
const MULTI_PROPS: &[PropDoc] = &[
    prop("values", "Signal<Vec<String>>", "-", "Valori selezionati."),
    prop("max_selected_display", "Option<usize>", "None", "Chip mostrate prima di \"+N\"."),
    prop("searchable / clearable / creatable", "bool", "false", "Come Select."),
    prop("on_change", "Option<EventHandler<Vec<String>>>", "None", "Aggiunta o rimozione."),
];
const COMBO_PROPS: &[PropDoc] = &[
    prop("value", "Signal<String>", "-", "Testo libero, modificabile."),
    prop("data", "Vec<SelectItem>", "-", "Suggerimenti."),
    prop("clearable / loading / disabled", "bool", "false", "Come Select."),
    prop("on_change", "Option<EventHandler<String>>", "None", "Digitazione o scelta."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("SelectItem::new(value, label)", "fn", "-", "Costruttore."),
    prop(".with_description(..)", "fn", "-", "Riga secondaria."),
    prop(".with_group(..)", "fn", "-", "Intestazione di gruppo."),
    prop(".disabled()", "fn", "-", "Non selezionabile."),
];

pub fn story() -> Story {
    Story {
        slug: "select",
        name: "Select",
        category: Category::Form,
        features: &["select"],
        icon: IconKind::ChevronDown,
        tagline: "Selezione singola, multipla con chip e combobox editabile, con ricerca, gruppi e creazione.",
        keywords: &["select", "multiselect", "combobox", "dropdown", "autocomplete", "tag", "chip"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Select",
            controls: vec![
                Control::text("label", "Etichetta", "Framework").emit(Emit::Template("label: Some(\"{v}\".to_string())")),
                Control::text("description", "Descrizione", "Il framework primario dello stack")
                    .emit(Emit::Template("description: Some(\"{v}\".to_string())")),
                Control::text("placeholder", "Placeholder", "Scegli un framework")
                    .emit(Emit::Template("placeholder: Some(\"{v}\".to_string())")),
                Control::bool("searchable", "Ricerca", true),
                Control::bool("clearable", "Azzerabile", true),
                Control::bool("creatable", "Creabile", false),
                Control::bool("disabled", "Disabilitato", false),
                Control::bool("loading", "Loading", false),
                Control::bool("error", "Errore", false).emit(Emit::Template("error: Some(\"Seleziona un framework\".to_string())")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["data,", "value,"],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "multi",
                title: "MultiSelect",
                description: "Chip rimovibili, limite di visualizzazione, creazione.",
                code: MULTI_CODE,
                render: || rsx! { Multi {} },
                span: Span::Half,
            },
            Example {
                id: "combo",
                title: "Combobox",
                description: "Input libero con suggerimenti.",
                code: COMBO_CODE,
                render: || rsx! { Combo {} },
                span: Span::Half,
            },
            Example {
                id: "grouped",
                title: "Gruppi e creazione",
                description: "Intestazioni di gruppo e nuove voci con Invio.",
                code: GROUPED_CODE,
                render: || rsx! { Grouped {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il form Impostazioni della scena condivisa.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Select", rows: SELECT_PROPS },
            PropGroup { component: "MultiSelect", rows: MULTI_PROPS },
            PropGroup { component: "Combobox", rows: COMBO_PROPS },
            PropGroup { component: "SelectItem", rows: ITEM_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Recessed field (§3.6) + floating panel (§3.2) + items (§3.4)",
            depth: "Well (trigger) · Raised (menu)",
            tokens: &["--sd-well", "--sd-border-strong", "--sd-inset-well", "--sd-surface-2", "--sd-elev-2", "--sd-item-h", "--sd-accent-soft"],
            notes: "Il trigger è un campo; il menu è un pannello flottante con voci sd-item e una sola voce selezionata.",
        }),
        a11y: &[
            "Trigger con role=combobox, aria-expanded e aria-controls sul listbox.",
            "Frecce, Home/End, Invio ed Esc gestiti; la ricerca mantiene il focus nel campo.",
            "error imposta aria-invalid e collega il messaggio con aria-describedby.",
            "Le voci disabilitate restano annunciate ma non selezionabili.",
        ],
        related: &["inputs", "chip", "dropdown", "command-palette", "segmented-controls"],
    }
}
