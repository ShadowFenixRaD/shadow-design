//! Story: NumberInput (feature `number_input`). Stateful: the playground goes through `NumberDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const STEPS: [f64; 4] = [0.5, 1.0, 5.0, 10.0];
const STEP_OPTS: &[ChoiceOption] = &[opt("0.5", "0.5"), opt("1", "1.0"), opt("5", "5.0"), opt("10", "10.0")];
const PRECISIONS: [usize; 3] = [0, 1, 2];
const PRECISION_OPTS: &[ChoiceOption] = &[opt("0", "0"), opt("1", "1"), opt("2", "2")];

const PRELUDE: &str = "let value = use_signal(|| 64.0);";

#[component]
fn NumberDemo(knobs: Knobs) -> Element {
    let value = use_signal(|| 64.0);
    rsx! {
        NumberInput {
            value,
            min: Some(0.0),
            max: Some(256.0),
            label: Some(knobs.text("label")).filter(|s| !s.is_empty()),
            step: knobs.pick("step", &STEPS),
            precision: knobs.pick("precision", &PRECISIONS),
            prefix: Some(knobs.text("prefix")).filter(|s| !s.is_empty()),
            suffix: Some(knobs.text("suffix")).filter(|s| !s.is_empty()),
            help_text: Some(knobs.text("help_text")).filter(|s| !s.is_empty()),
            disabled: knobs.bool("disabled"),
            error: knobs.bool("error").then(|| "Supera la banda del piano".to_string()),
            glow: knobs.bool("glow").then_some(true),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        NumberDemo { knobs: k.clone() }
    }
}

#[component]
fn Affixes() -> Element {
    let price = use_signal(|| 12.5);
    let cpu = use_signal(|| 75.0);
    rsx! {
        Stack { gap: SpacingToken::Md,
            NumberInput {
                value: price,
                label: Some("Prezzo per nodo".to_string()),
                prefix: Some("€".to_string()),
                suffix: Some("/mese".to_string()),
                step: 0.5,
                precision: 2,
                min: Some(0.0),
            }
            NumberInput {
                value: cpu,
                label: Some("Soglia CPU".to_string()),
                suffix: Some("%".to_string()),
                step: 5.0,
                min: Some(0.0),
                max: Some(100.0),
            }
        }
    }
}
const AFFIXES_CODE: &str = r#"let price = use_signal(|| 12.5);
let cpu = use_signal(|| 75.0);

Stack { gap: SpacingToken::Md,
    NumberInput {
        value: price,
        label: Some("Prezzo per nodo".to_string()),
        prefix: Some("€".to_string()),
        suffix: Some("/mese".to_string()),
        step: 0.5,
        precision: 2,
        min: Some(0.0),
    }
    NumberInput {
        value: cpu,
        label: Some("Soglia CPU".to_string()),
        suffix: Some("%".to_string()),
        step: 5.0,
        min: Some(0.0),
        max: Some(100.0),
    }
}"#;

#[component]
fn States() -> Element {
    let ttl = use_signal(|| 3600.0);
    let peers = use_signal(|| 140.0);
    let shards = use_signal(|| 4.0);
    rsx! {
        Stack { gap: SpacingToken::Md,
            NumberInput {
                value: ttl,
                label: Some("TTL sessione".to_string()),
                suffix: Some("s".to_string()),
                step: 60.0,
                min: Some(60.0),
                help_text: Some("Frecce su e giù per passi da 60 secondi.".to_string()),
            }
            NumberInput {
                value: peers,
                label: Some("Peer massimi".to_string()),
                max: Some(128.0),
                error: Some("Il piano Team ammette al massimo 128 peer".to_string()),
            }
            NumberInput { value: shards, label: Some("Shard".to_string()), disabled: true }
        }
    }
}
const STATES_CODE: &str = r#"let ttl = use_signal(|| 3600.0);
let peers = use_signal(|| 140.0);
let shards = use_signal(|| 4.0);

Stack { gap: SpacingToken::Md,
    NumberInput {
        value: ttl,
        label: Some("TTL sessione".to_string()),
        suffix: Some("s".to_string()),
        step: 60.0,
        min: Some(60.0),
        help_text: Some("Frecce su e giù per passi da 60 secondi.".to_string()),
    }
    NumberInput {
        value: peers,
        label: Some("Peer massimi".to_string()),
        max: Some(128.0),
        error: Some("Il piano Team ammette al massimo 128 peer".to_string()),
    }
    NumberInput { value: shards, label: Some("Shard".to_string()), disabled: true }
}"#;

#[component]
fn InContext() -> Element {
    let replicas = use_signal(|| 3.0);
    let vcpu = use_signal(|| 2.0);
    let budget = use_signal(|| 120.0);
    let mut toast = use_notification();
    let cost = replicas() * vcpu() * 9.5;
    let over = cost > budget();
    rsx! {
        Card {
            title: Some("Scala il cluster edge-eu".to_string()),
            subtitle: Some("Le repliche partono entro 30 secondi".to_string()),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
                    Badge { label: format!("Stima € {cost:.2} / mese"), variant: if over { BadgeVariant::Danger } else { BadgeVariant::Success } }
                    Button {
                        disabled: over,
                        onclick: move |_| {
                            toast.success("Cluster scalato");
                        },
                        "Applica"
                    }
                }
            }),
            SimpleGrid { cols: 1, sm: Some(3), spacing: GridSpacing::Md,
                NumberInput { value: replicas, label: Some("Repliche".to_string()), min: Some(1.0), max: Some(12.0) }
                NumberInput { value: vcpu, label: Some("vCPU per replica".to_string()), min: Some(1.0), max: Some(16.0) }
                NumberInput {
                    value: budget,
                    label: Some("Budget".to_string()),
                    prefix: Some("€".to_string()),
                    step: 10.0,
                    min: Some(0.0),
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let replicas = use_signal(|| 3.0);
let vcpu = use_signal(|| 2.0);
let budget = use_signal(|| 120.0);
let mut toast = use_notification();
let cost = replicas() * vcpu() * 9.5;
let over = cost > budget();

Card {
    title: Some("Scala il cluster edge-eu".to_string()),
    subtitle: Some("Le repliche partono entro 30 secondi".to_string()),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
            Badge { label: format!("Stima € {cost:.2} / mese"), variant: if over { BadgeVariant::Danger } else { BadgeVariant::Success } }
            Button {
                disabled: over,
                onclick: move |_| {
                    toast.success("Cluster scalato");
                },
                "Applica"
            }
        }
    }),
    SimpleGrid { cols: 1, sm: Some(3), spacing: GridSpacing::Md,
        NumberInput { value: replicas, label: Some("Repliche".to_string()), min: Some(1.0), max: Some(12.0) }
        NumberInput { value: vcpu, label: Some("vCPU per replica".to_string()), min: Some(1.0), max: Some(16.0) }
        NumberInput {
            value: budget,
            label: Some("Budget".to_string()),
            prefix: Some("€".to_string()),
            step: 10.0,
            min: Some(0.0),
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("value", "Signal<f64>", "-", "Valore corrente, già arrotondato e limitato."),
    prop("step", "f64", "1.0", "Passo dei pulsanti e delle frecce su/giù."),
    prop("min / max", "Option<f64>", "None", "Limiti; i pulsanti si disattivano al limite."),
    prop("precision", "usize", "0", "Cifre decimali mostrate e salvate."),
    prop("prefix / suffix", "Option<String>", "None", "Unità prima o dopo il numero (€, %, s)."),
    prop("label / placeholder", "Option<String>", "None", "Etichetta collegata e segnaposto."),
    prop("help_text", "Option<String>", "None", "Testo di aiuto collegato con aria-describedby."),
    prop("error", "Option<String>", "None", "Bordo danger, aria-invalid e messaggio."),
    prop("disabled", "bool", "false", "Campo e pulsanti disattivati."),
    prop("glow", "Option<bool>", "None", "Forza o rimuove il glow del campo."),
    prop("on_change", "Option<EventHandler<f64>>", "None", "Nuovo valore dopo limiti e arrotondamento."),
    prop("id", "Option<String>", "None", "id del campo; generato se assente."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];

pub fn story() -> Story {
    Story {
        slug: "number-input",
        name: "NumberInput",
        category: Category::Form,
        features: &["number_input"],
        icon: IconKind::Plus,
        tagline: "Campo numerico con pulsanti più e meno, limiti, decimali e unità di misura.",
        keywords: &["number input", "numero", "spinbutton", "stepper", "quantità", "prezzo"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "NumberInput",
            controls: vec![
                Control::text("label", "Etichetta", "Banda massima").emit(Emit::Template("label: Some(\"{v}\".to_string())")),
                Control::choice("step", "Passo", STEP_OPTS, 1),
                Control::choice("precision", "Decimali", PRECISION_OPTS, 0),
                Control::text("prefix", "Prefisso", "").emit(Emit::Template("prefix: Some(\"{v}\".to_string())")),
                Control::text("suffix", "Suffisso", "Mbit/s").emit(Emit::Template("suffix: Some(\"{v}\".to_string())")),
                Control::text("help_text", "Aiuto", "").emit(Emit::Template("help_text: Some(\"{v}\".to_string())")),
                Control::bool("disabled", "Disabilitato", false),
                Control::bool("error", "Errore", false)
                    .emit(Emit::Template("error: Some(\"Supera la banda del piano\".to_string())")),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["value,", "min: Some(0.0),", "max: Some(256.0),"],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "affixes",
                title: "Prefisso e suffisso",
                description: "Valuta con due decimali e percentuale limitata.",
                code: AFFIXES_CODE,
                render: || rsx! { Affixes {} },
                span: Span::Half,
            },
            Example {
                id: "states",
                title: "Stati",
                description: "Aiuto, errore di limite e disabilitato.",
                code: STATES_CODE,
                render: || rsx! { States {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Dimensionare un cluster con stima del costo e azione bloccata oltre il budget.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "NumberInput", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Recessed field (language.md §3.6)",
            depth: "Well",
            tokens: &["--sd-well", "--sd-border-strong", "--sd-inset-well", "--sd-control-md", "--sd-accent", "--sd-glow-sm", "--sd-text-subtle"],
            notes: "Prefisso e suffisso sono testo subtle dentro il pozzetto; i pulsanti più/meno sono parte del campo, non pulsanti illuminati.",
        }),
        a11y: &[
            "Il campo ha role=spinbutton con aria-valuenow, aria-valuemin e aria-valuemax.",
            "Freccia su e giù cambiano il valore di step; i limiti sono rispettati anche da tastiera.",
            "I pulsanti più e meno hanno aria-label da Locale (number_increment / number_decrement).",
            "error imposta aria-invalid e collega il messaggio con aria-describedby.",
        ],
        related: &["inputs", "slider", "pin-input", "form"],
    }
}
