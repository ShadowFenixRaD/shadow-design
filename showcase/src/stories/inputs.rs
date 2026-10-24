//! Story: TextInput (feature `input`). Stateful: the playground goes through `InputDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const TYPES: [&str; 4] = ["text", "email", "search", "url"];
const TYPE_OPTS: &[ChoiceOption] = &[opt("Text", "text"), opt("Email", "email"), opt("Search", "search"), opt("Url", "url")];

const PRELUDE: &str = r#"let value = use_signal(|| "edge-07".to_string());"#;

#[component]
fn InputDemo(knobs: Knobs) -> Element {
    let value = use_signal(|| "edge-07".to_string());
    let input_type = knobs.pick("input_type", &TYPES);
    rsx! {
        TextInput {
            value,
            label: Some(knobs.text("label")).filter(|s| !s.is_empty()),
            placeholder: Some(knobs.text("placeholder")).filter(|s| !s.is_empty()),
            help_text: Some(knobs.text("help_text")).filter(|s| !s.is_empty()),
            input_type: (input_type != "text").then(|| input_type.to_string()),
            left_icon: knobs.bool("left_icon").then(|| IconKind::Server.into()),
            clearable: knobs.bool("clearable"),
            disabled: knobs.bool("disabled"),
            error: knobs.bool("error").then(|| "Nome già in uso nella mesh".to_string()),
            glow: knobs.bool("glow").then_some(true),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        InputDemo { knobs: k.clone() }
    }
}

#[component]
fn Icons() -> Element {
    let query = use_signal(|| "kyber".to_string());
    let secret = use_signal(|| "sk_live_shadowfenix".to_string());
    let mut reveal = use_signal(|| false);
    rsx! {
        Stack { gap: SpacingToken::Md,
            TextInput {
                value: query,
                placeholder: Some("Cerca nodi, peer, commit…".to_string()),
                left_icon: Some(IconKind::Search.into()),
                clearable: true,
            }
            TextInput {
                value: secret,
                label: Some("Chiave API".to_string()),
                input_type: Some(if reveal() { "text" } else { "password" }.to_string()),
                left_icon: Some(IconKind::Key.into()),
                right_icon: Some(if reveal() { IconKind::EyeOff } else { IconKind::Eye }.into()),
                right_icon_label: Some(if reveal() { "Nascondi la chiave" } else { "Mostra la chiave" }.to_string()),
                on_right_icon_click: move |_| reveal.toggle(),
            }
        }
    }
}
const ICONS_CODE: &str = r#"let query = use_signal(|| "kyber".to_string());
let secret = use_signal(|| "sk_live_shadowfenix".to_string());
let mut reveal = use_signal(|| false);

Stack { gap: SpacingToken::Md,
    TextInput {
        value: query,
        placeholder: Some("Cerca nodi, peer, commit…".to_string()),
        left_icon: Some(IconKind::Search.into()),
        clearable: true,
    }
    TextInput {
        value: secret,
        label: Some("Chiave API".to_string()),
        input_type: Some(if reveal() { "text" } else { "password" }.to_string()),
        left_icon: Some(IconKind::Key.into()),
        right_icon: Some(if reveal() { IconKind::EyeOff } else { IconKind::Eye }.into()),
        right_icon_label: Some(if reveal() { "Nascondi la chiave" } else { "Mostra la chiave" }.to_string()),
        on_right_icon_click: move |_| reveal.toggle(),
    }
}"#;

#[component]
fn States() -> Element {
    let host = use_signal(|| "edge-07.mesh.local".to_string());
    let port = use_signal(|| "80800".to_string());
    let region = use_signal(|| "eu-south-1".to_string());
    rsx! {
        Stack { gap: SpacingToken::Md,
            TextInput {
                value: host,
                label: Some("Host".to_string()),
                help_text: Some("Nome DNS interno, risolto dalla mesh.".to_string()),
            }
            TextInput {
                value: port,
                label: Some("Porta".to_string()),
                error: Some("La porta deve essere compresa tra 1 e 65535".to_string()),
            }
            TextInput {
                value: region,
                label: Some("Regione".to_string()),
                disabled: true,
                help_text: Some("Fissata alla creazione del nodo.".to_string()),
            }
        }
    }
}
const STATES_CODE: &str = r#"let host = use_signal(|| "edge-07.mesh.local".to_string());
let port = use_signal(|| "80800".to_string());
let region = use_signal(|| "eu-south-1".to_string());

Stack { gap: SpacingToken::Md,
    TextInput {
        value: host,
        label: Some("Host".to_string()),
        help_text: Some("Nome DNS interno, risolto dalla mesh.".to_string()),
    }
    TextInput {
        value: port,
        label: Some("Porta".to_string()),
        error: Some("La porta deve essere compresa tra 1 e 65535".to_string()),
    }
    TextInput {
        value: region,
        label: Some("Regione".to_string()),
        disabled: true,
        help_text: Some("Fissata alla creazione del nodo.".to_string()),
    }
}"#;

#[component]
fn LiveValidation() -> Element {
    let email = use_signal(String::new);
    let error = match email() {
        e if e.is_empty() => None,
        e if !e.contains('@') || !e.contains('.') => Some("Indirizzo email non valido".to_string()),
        _ => None,
    };
    rsx! {
        TextInput {
            value: email,
            label: Some("Email di notifica".to_string()),
            placeholder: Some("ops@shadowfenix.dev".to_string()),
            input_type: Some("email".to_string()),
            help_text: Some("Ricevi qui gli avvisi di latenza.".to_string()),
            clearable: true,
            error,
        }
    }
}
const LIVE_VALIDATION_CODE: &str = r#"let email = use_signal(String::new);
let error = match email() {
    e if e.is_empty() => None,
    e if !e.contains('@') || !e.contains('.') => Some("Indirizzo email non valido".to_string()),
    _ => None,
};

TextInput {
    value: email,
    label: Some("Email di notifica".to_string()),
    placeholder: Some("ops@shadowfenix.dev".to_string()),
    input_type: Some("email".to_string()),
    help_text: Some("Ricevi qui gli avvisi di latenza.".to_string()),
    clearable: true,
    error,
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::SettingsForm { compact: false }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/settings_form.rs (estratto): TextInput nel Fieldset "Identità".
let name = use_signal(|| "edge-07".to_string());

Fieldset { legend: "Identità".to_string(), description: Some("Come il nodo appare nella mesh.".to_string()),
    Stack { gap: SpacingToken::Md,
        TextInput { value: name, label: Some("Nome del nodo".to_string()), clearable: true }
        Select { data: regions(), value: region, label: Some("Regione".to_string()), searchable: true }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("value", "Signal<String>", "-", "Testo del campo, aggiornato a ogni digitazione."),
    prop("label", "Option<String>", "None", "Etichetta collegata al campo con for/id."),
    prop("placeholder", "Option<String>", "None", "Segnaposto in --sd-text-subtle."),
    prop("help_text", "Option<String>", "None", "Testo di aiuto sotto al campo, collegato con aria-describedby."),
    prop("error", "Option<String>", "None", "Bordo danger, aria-invalid e messaggio sotto al campo."),
    prop("disabled", "bool", "false", "Campo non modificabile, opacità --sd-disabled-opacity."),
    prop("clearable", "bool", "false", "Pulsante di cancellazione quando il campo non è vuoto."),
    prop("glow", "Option<bool>", "None", "Forza o rimuove il glow del campo."),
    prop("left_icon / right_icon", "Option<SvgIcon>", "None", "Icona dentro il campo; accetta IconKind::X.into()."),
    prop("on_left_icon_click / on_right_icon_click", "Option<EventHandler<MouseEvent>>", "None", "Rende l'icona un pulsante (es. mostra password)."),
    prop("left_icon_label / right_icon_label", "Option<String>", "None", "aria-label e title dell'icona interattiva."),
    prop("input_type", "Option<String>", "None", "type HTML: text (default), email, password, search, url."),
    prop("id", "Option<String>", "None", "id del campo; generato se assente."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];

pub fn story() -> Story {
    Story {
        slug: "inputs",
        name: "TextInput",
        category: Category::Form,
        features: &["input"],
        icon: IconKind::Terminal,
        tagline: "Campo di testo con etichetta, icone, pulsante di cancellazione e stato di errore.",
        keywords: &["input", "text input", "campo", "testo", "password", "form", "ricerca"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "TextInput",
            controls: vec![
                Control::text("label", "Etichetta", "Nome del nodo").emit(Emit::Template("label: Some(\"{v}\".to_string())")),
                Control::text("placeholder", "Placeholder", "es. edge-07")
                    .emit(Emit::Template("placeholder: Some(\"{v}\".to_string())")),
                Control::text("help_text", "Aiuto", "Visibile a tutti i peer della mesh.")
                    .emit(Emit::Template("help_text: Some(\"{v}\".to_string())")),
                Control::choice("input_type", "Tipo", TYPE_OPTS, 0).emit(Emit::Template("input_type: Some(\"{v}\".to_string())")),
                Control::bool("left_icon", "Icona sinistra", true)
                    .emit(Emit::Template("left_icon: Some(IconKind::Server.into())")),
                Control::bool("clearable", "Azzerabile", true).emit(Emit::Template("clearable: true")),
                Control::bool("disabled", "Disabilitato", false),
                Control::bool("error", "Errore", false)
                    .emit(Emit::Template("error: Some(\"Nome già in uso nella mesh\".to_string())")),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["value,"],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "icons",
                title: "Icone e azioni",
                description: "Ricerca azzerabile e chiave API con mostra/nascondi.",
                code: ICONS_CODE,
                render: || rsx! { Icons {} },
                span: Span::Half,
            },
            Example {
                id: "states",
                title: "Stati",
                description: "Aiuto, errore e disabilitato.",
                code: STATES_CODE,
                render: || rsx! { States {} },
                span: Span::Half,
            },
            Example {
                id: "validation",
                title: "Validazione live",
                description: "L'errore dipende dal valore digitato.",
                code: LIVE_VALIDATION_CODE,
                render: || rsx! { LiveValidation {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Le impostazioni di un nodo: campi, select e toggle in due Fieldset.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "TextInput", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Recessed field (language.md §3.6)",
            depth: "Well",
            tokens: &["--sd-well", "--sd-border-strong", "--sd-inset-well", "--sd-control-md", "--sd-accent", "--sd-glow-sm", "--sd-danger-border"],
            notes: "Il focus è il bordo accento con --sd-glow-sm, senza outline; l'errore passa a --sd-danger-border.",
        }),
        a11y: &[
            "L'etichetta è un <label for> collegato all'id del campo.",
            "help_text ed error sono collegati con aria-describedby; error imposta aria-invalid.",
            "Le icone interattive sono <button> con aria-label da left_icon_label / right_icon_label.",
            "Il pulsante di cancellazione usa il testo di Locale (clear_text) come aria-label.",
        ],
        related: &["textarea", "number-input", "select", "form", "fieldset"],
    }
}
