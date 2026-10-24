//! Story: Fieldset (feature `fieldset`). Stateful children: the playground goes through `FieldsetDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const VARIANTS: [FieldsetVariant; 3] = [FieldsetVariant::Default, FieldsetVariant::Filled, FieldsetVariant::Ungrouped];
const VARIANT_OPTS: &[ChoiceOption] = &[
    opt("Default", "FieldsetVariant::Default"),
    opt("Filled", "FieldsetVariant::Filled"),
    opt("Ungrouped", "FieldsetVariant::Ungrouped"),
];

const PRELUDE: &str = r#"let host = use_signal(|| "edge-07.mesh.local".to_string());
let port = use_signal(|| 443.0);"#;
const CHILDREN: &str = r#"Stack { gap: SpacingToken::Md,
    TextInput { value: host, label: Some("Host".to_string()) }
    NumberInput { value: port, label: Some("Porta".to_string()), min: Some(1.0), max: Some(65535.0) }
}"#;

#[component]
fn FieldsetDemo(knobs: Knobs) -> Element {
    let host = use_signal(|| "edge-07.mesh.local".to_string());
    let port = use_signal(|| 443.0);
    rsx! {
        Fieldset {
            legend: knobs.text("legend"),
            description: Some(knobs.text("description")).filter(|s| !s.is_empty()),
            variant: knobs.pick("variant", &VARIANTS),
            disabled: knobs.bool("disabled"),
            Stack { gap: SpacingToken::Md,
                TextInput { value: host, label: Some("Host".to_string()) }
                NumberInput { value: port, label: Some("Porta".to_string()), min: Some(1.0), max: Some(65535.0) }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        FieldsetDemo { knobs: k.clone() }
    }
}

#[component]
fn Variants() -> Element {
    let a = use_signal(|| "Giulia Ferri".to_string());
    let b = use_signal(|| "Giulia Ferri".to_string());
    let c = use_signal(|| "Giulia Ferri".to_string());
    rsx! {
        Stack { gap: SpacingToken::Md,
            Fieldset { legend: "Default".to_string(),
                TextInput { value: a, label: Some("Referente".to_string()) }
            }
            Fieldset { legend: "Filled".to_string(), variant: FieldsetVariant::Filled,
                TextInput { value: b, label: Some("Referente".to_string()) }
            }
            Fieldset { legend: "Ungrouped".to_string(), variant: FieldsetVariant::Ungrouped,
                TextInput { value: c, label: Some("Referente".to_string()) }
            }
        }
    }
}
const VARIANTS_CODE: &str = r#"let a = use_signal(|| "Giulia Ferri".to_string());
let b = use_signal(|| "Giulia Ferri".to_string());
let c = use_signal(|| "Giulia Ferri".to_string());

Stack { gap: SpacingToken::Md,
    Fieldset { legend: "Default".to_string(),
        TextInput { value: a, label: Some("Referente".to_string()) }
    }
    Fieldset { legend: "Filled".to_string(), variant: FieldsetVariant::Filled,
        TextInput { value: b, label: Some("Referente".to_string()) }
    }
    Fieldset { legend: "Ungrouped".to_string(), variant: FieldsetVariant::Ungrouped,
        TextInput { value: c, label: Some("Referente".to_string()) }
    }
}"#;

#[component]
fn LockedSection() -> Element {
    let custom = use_signal(|| false);
    let dns = use_signal(|| "10.0.0.53".to_string());
    let mtu = use_signal(|| 1420.0);
    rsx! {
        Stack { gap: SpacingToken::Md,
            Switch { checked: custom, label: Some("Rete personalizzata".to_string()) }
            Fieldset {
                legend: "Parametri avanzati".to_string(),
                description: Some("Attiva la rete personalizzata per modificarli.".to_string()),
                disabled: !custom(),
                Stack { gap: SpacingToken::Md,
                    TextInput { value: dns, label: Some("DNS".to_string()) }
                    NumberInput { value: mtu, label: Some("MTU".to_string()), min: Some(1280.0), max: Some(9000.0) }
                }
            }
        }
    }
}
const LOCKED_SECTION_CODE: &str = r#"let custom = use_signal(|| false);
let dns = use_signal(|| "10.0.0.53".to_string());
let mtu = use_signal(|| 1420.0);

Stack { gap: SpacingToken::Md,
    Switch { checked: custom, label: Some("Rete personalizzata".to_string()) }
    Fieldset {
        legend: "Parametri avanzati".to_string(),
        description: Some("Attiva la rete personalizzata per modificarli.".to_string()),
        disabled: !custom(),
        Stack { gap: SpacingToken::Md,
            TextInput { value: dns, label: Some("DNS".to_string()) }
            NumberInput { value: mtu, label: Some("MTU".to_string()), min: Some(1280.0), max: Some(9000.0) }
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::SettingsForm { compact: false }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/settings_form.rs (estratto): due Fieldset affiancati dividono identità e rete.
Fieldset { legend: "Identità".to_string(), description: Some("Come il nodo appare nella mesh.".to_string()),
    Stack { gap: SpacingToken::Md,
        TextInput { value: name, label: Some("Nome del nodo".to_string()), clearable: true }
        Select { data: regions(), value: region, label: Some("Regione".to_string()), searchable: true }
    }
}
Fieldset { legend: "Rete".to_string(), description: Some("Trasporti e cifratura dei peer.".to_string()),
    Stack { gap: SpacingToken::Md,
        MultiSelect { data: protocols(), values: protocols_on, label: Some("Protocolli".to_string()), clearable: true }
        Switch { checked: relay, label: Some("Relay automatico".to_string()) }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("legend", "String", "-", "Titolo del gruppo, reso come <legend>."),
    prop("description", "Option<String>", "None", "Riga di aiuto sotto la legenda."),
    prop("variant", "FieldsetVariant", "Default", "Default (contenitore), Filled (tinta) o Ungrouped (solo legenda)."),
    prop("disabled", "bool", "false", "Disabilita nativamente tutti i campi contenuti."),
    prop("children", "Element", "-", "Campi del gruppo."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "fieldset",
        name: "Fieldset",
        category: Category::Form,
        features: &["fieldset"],
        icon: IconKind::Box,
        tagline: "Raggruppa i campi correlati di un modulo sotto una legenda, anche per disabilitarli insieme.",
        keywords: &["fieldset", "gruppo", "legenda", "sezione", "form", "legend"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Fieldset",
            controls: vec![
                Control::text("legend", "Legenda", "Endpoint").emit(Emit::Template("legend: \"{v}\".to_string()")),
                Control::text("description", "Descrizione", "Dove il nodo accetta connessioni.")
                    .emit(Emit::Template("description: Some(\"{v}\".to_string())")),
                Control::choice("variant", "Variante", VARIANT_OPTS, 0),
                Control::bool("disabled", "Disabilitato", false),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &[],
            children_code: Some(CHILDREN),
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "variants",
                title: "Varianti",
                description: "Contenitore, tinta o sola legenda.",
                code: VARIANTS_CODE,
                render: || rsx! { Variants {} },
                span: Span::Half,
            },
            Example {
                id: "disabled",
                title: "Sezione bloccata",
                description: "disabled spegne tutti i campi del gruppo con un solo flag.",
                code: LOCKED_SECTION_CODE,
                render: || rsx! { LockedSection {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il form Impostazioni diviso in Identità e Rete.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Fieldset", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Container (language.md §3.1)",
            depth: "Flat-lit (Default) · tint (Filled) · nessuno (Ungrouped)",
            tokens: &["--sd-surface-1", "--sd-border", "--sd-radius-lg", "--sd-tint-1", "--sd-text-sm", "--sd-text-subtle"],
            notes: "È un contenitore statico: niente hover né glow; la legenda segue lo stile delle etichette dei campi.",
        }),
        a11y: &[
            "Elemento <fieldset> nativo con <legend>: gli screen reader annunciano la legenda entrando nel gruppo.",
            "disabled è l'attributo nativo: disabilita ogni campo contenuto, anche da tastiera.",
            "La descrizione è un paragrafo dentro il fieldset, letta in sequenza.",
        ],
        related: &["form", "inputs", "cards", "switch"],
    }
}
