//! Story: RadioGroup, Radio and RadioCard (feature `radio`). Stateful: the playground goes
//! through `RadioDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const ORIENTATIONS: [RadioOrientation; 2] = [RadioOrientation::Vertical, RadioOrientation::Horizontal];
const ORIENTATION_OPTS: &[ChoiceOption] =
    &[opt("Verticale", "RadioOrientation::Vertical"), opt("Orizzontale", "RadioOrientation::Horizontal")];

const PRELUDE: &str = r#"let value = use_signal(|| "team".to_string());"#;
const CHILDREN: &str = r#"Radio { value: "starter", label: Some("Starter".to_string()), description: Some("1 nodo, supporto community".to_string()) }
Radio { value: "team", label: Some("Team".to_string()), description: Some("Fino a 128 peer e relay dedicato".to_string()) }
Radio { value: "mesh", label: Some("Mesh".to_string()), description: Some("Cluster multi-regione e SLA 99,99%".to_string()) }"#;

#[component]
fn RadioDemo(knobs: Knobs) -> Element {
    let value = use_signal(|| "team".to_string());
    rsx! {
        RadioGroup {
            value,
            label: Some(knobs.text("label")).filter(|s| !s.is_empty()),
            help_text: Some(knobs.text("help_text")).filter(|s| !s.is_empty()),
            orientation: knobs.pick("orientation", &ORIENTATIONS),
            disabled: knobs.bool("disabled"),
            error: knobs.bool("error").then(|| "Il piano scelto non copre la regione".to_string()),
            Radio { value: "starter", label: Some("Starter".to_string()), description: Some("1 nodo, supporto community".to_string()) }
            Radio { value: "team", label: Some("Team".to_string()), description: Some("Fino a 128 peer e relay dedicato".to_string()) }
            Radio { value: "mesh", label: Some("Mesh".to_string()), description: Some("Cluster multi-regione e SLA 99,99%".to_string()) }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        RadioDemo { knobs: k.clone() }
    }
}

#[component]
fn Cards() -> Element {
    let storage = use_signal(|| "nvme".to_string());
    rsx! {
        RadioGroup { value: storage, label: Some("Storage".to_string()),
            RadioCard {
                value: "nvme",
                title: Some("NVMe locale".to_string()),
                description: Some("Latenza sotto il millisecondo, un solo nodo.".to_string()),
                icon: Some(IconKind::Server),
                badge: Some("Consigliato".to_string()),
            }
            RadioCard {
                value: "replica",
                title: Some("Replicato".to_string()),
                description: Some("Tre copie su regioni diverse.".to_string()),
                icon: Some(IconKind::Layers),
            }
        }
    }
}
const CARDS_CODE: &str = r#"let storage = use_signal(|| "nvme".to_string());

RadioGroup { value: storage, label: Some("Storage".to_string()),
    RadioCard {
        value: "nvme",
        title: Some("NVMe locale".to_string()),
        description: Some("Latenza sotto il millisecondo, un solo nodo.".to_string()),
        icon: Some(IconKind::Server),
        badge: Some("Consigliato".to_string()),
    }
    RadioCard {
        value: "replica",
        title: Some("Replicato".to_string()),
        description: Some("Tre copie su regioni diverse.".to_string()),
        icon: Some(IconKind::Layers),
    }
}"#;

#[component]
fn Horizontal() -> Element {
    let window = use_signal(|| "night".to_string());
    rsx! {
        RadioGroup {
            value: window,
            label: Some("Finestra di aggiornamento".to_string()),
            help_text: Some("Ora locale del nodo.".to_string()),
            orientation: RadioOrientation::Horizontal,
            Radio { value: "now", label: Some("Subito".to_string()) }
            Radio { value: "night", label: Some("Notte (02-05)".to_string()) }
            Radio { value: "weekend", label: Some("Weekend".to_string()), disabled: true }
        }
    }
}
const HORIZONTAL_CODE: &str = r#"let window = use_signal(|| "night".to_string());

RadioGroup {
    value: window,
    label: Some("Finestra di aggiornamento".to_string()),
    help_text: Some("Ora locale del nodo.".to_string()),
    orientation: RadioOrientation::Horizontal,
    Radio { value: "now", label: Some("Subito".to_string()) }
    Radio { value: "night", label: Some("Notte (02-05)".to_string()) }
    Radio { value: "weekend", label: Some("Weekend".to_string()), disabled: true }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::SettingsForm { compact: false }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/settings_form.rs (estratto): la scelta della cifratura nel Fieldset "Rete".
let cipher = use_signal(|| "kyber1024".to_string());

RadioGroup { value: cipher, label: Some("Cifratura".to_string()), orientation: RadioOrientation::Horizontal,
    Radio { value: "kyber768", checked: Some(cipher), label: Some("Kyber-768".to_string()) }
    Radio { value: "kyber1024", checked: Some(cipher), label: Some("Kyber-1024".to_string()) }
}"#;

const GROUP_PROPS: &[PropDoc] = &[
    prop("value", "Signal<String>", "-", "Valore dell'opzione selezionata, condiviso con i figli."),
    prop("name", "Option<String>", "None", "name HTML comune; generato se assente."),
    prop("orientation", "RadioOrientation", "Vertical", "Vertical o Horizontal."),
    prop("label / help_text", "Option<String>", "None", "Titolo del gruppo (aria-labelledby) e aiuto."),
    prop("error", "Option<String>", "None", "Messaggio di errore e aria-invalid sul gruppo."),
    prop("disabled", "bool", "false", "Disabilita tutte le opzioni."),
    prop("children", "Element", "-", "Radio o RadioCard."),
    prop("id / class", "Option<String> / String", "None / \"\"", "id del gruppo e classi aggiuntive."),
];
const RADIO_PROPS: &[PropDoc] = &[
    prop("value", "String", "-", "Valore di questa opzione."),
    prop("checked", "Option<Signal<String>>", "None", "Signal esterno; dentro un RadioGroup non serve."),
    prop("label / description", "Option<String>", "None", "Etichetta e riga di aiuto."),
    prop("disabled", "bool", "false", "Solo questa opzione."),
    prop("on_change", "Option<EventHandler<String>>", "None", "Valore scelto."),
    prop("name / id / class", "Option<String> / String", "None", "Attributi HTML e classi."),
];
const CARD_PROPS: &[PropDoc] = &[
    prop("value", "String", "-", "Valore di questa opzione."),
    prop("title / description", "Option<String>", "None", "Titolo e testo della scheda."),
    prop("icon", "Option<IconKind>", "None", "Icona a sinistra."),
    prop("badge", "Option<String>", "None", "Etichetta in alto a destra (es. Consigliato)."),
    prop("disabled", "bool", "false", "Scheda non selezionabile."),
    prop("children", "Element", "-", "Contenuto extra sotto la descrizione."),
    prop("on_change", "Option<EventHandler<String>>", "None", "Valore scelto."),
];

pub fn story() -> Story {
    Story {
        slug: "radio",
        name: "RadioGroup",
        category: Category::Form,
        features: &["radio"],
        icon: IconKind::Radio,
        tagline: "Una scelta tra poche opzioni visibili, in elenco o come schede selezionabili.",
        keywords: &["radio", "radio group", "opzioni", "scelta singola", "radio card", "piano"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "RadioGroup",
            controls: vec![
                Control::text("label", "Etichetta", "Piano").emit(Emit::Template("label: Some(\"{v}\".to_string())")),
                Control::text("help_text", "Aiuto", "Cambiabile in qualsiasi momento.")
                    .emit(Emit::Template("help_text: Some(\"{v}\".to_string())")),
                Control::choice("orientation", "Orientamento", ORIENTATION_OPTS, 0),
                Control::bool("disabled", "Disabilitato", false),
                Control::bool("error", "Errore", false)
                    .emit(Emit::Template("error: Some(\"Il piano scelto non copre la regione\".to_string())")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["value,"],
            children_code: Some(CHILDREN),
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "cards",
                title: "RadioCard",
                description: "Schede selezionabili con icona e badge.",
                code: CARDS_CODE,
                render: || rsx! { Cards {} },
                span: Span::Half,
            },
            Example {
                id: "horizontal",
                title: "Orizzontale",
                description: "Opzioni brevi in riga, una disabilitata.",
                code: HORIZONTAL_CODE,
                render: || rsx! { Horizontal {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "La cifratura dei peer nel form Impostazioni.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "RadioGroup", rows: GROUP_PROPS },
            PropGroup { component: "Radio", rows: RADIO_PROPS },
            PropGroup { component: "RadioCard", rows: CARD_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Recessed field, toggle (§3.6) · RadioCard: selected (§3.7)",
            depth: "Well (a riposo) · value fill (selezionato)",
            tokens: &["--sd-well", "--sd-border-strong", "--sd-accent", "--sd-accent-fg", "--sd-glow-sm", "--sd-accent-soft", "--sd-accent-border"],
            notes: "Il pallino selezionato è un riempimento accento; la RadioCard selezionata usa sd-selected senza cambiare peso del testo.",
        }),
        a11y: &[
            "Il gruppo ha role=radiogroup con aria-labelledby sull'etichetta.",
            "Ogni Radio è un input radio nativo con name condiviso: le frecce spostano la selezione.",
            "RadioCard ha role=radio e aria-checked.",
            "error imposta aria-invalid sul gruppo.",
        ],
        related: &["checkbox", "segmented-controls", "select", "switch", "fieldset"],
    }
}
