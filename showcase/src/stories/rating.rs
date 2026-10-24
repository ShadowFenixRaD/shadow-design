//! Story: Rating (feature `rating`). Stateful: the playground goes through `RatingDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const COUNTS: [usize; 3] = [5, 7, 10];
const COUNT_OPTS: &[ChoiceOption] = &[opt("5", "5"), opt("7", "7"), opt("10", "10")];
const FRACTIONS: [usize; 2] = [1, 2];
const FRACTION_OPTS: &[ChoiceOption] = &[opt("Intere", "1"), opt("Mezze", "2")];

const PRELUDE: &str = "let value = use_signal(|| 4.0);";

#[component]
fn RatingDemo(knobs: Knobs) -> Element {
    let value = use_signal(|| 4.0);
    rsx! {
        Rating {
            value,
            label: Some("Valutazione del servizio".to_string()),
            count: knobs.pick("count", &COUNTS),
            fractions: knobs.pick("fractions", &FRACTIONS),
            size: knobs.number("size") as u32,
            icon: knobs.bool("icon").then_some(IconKind::Flame),
            readonly: knobs.bool("readonly"),
            disabled: knobs.bool("disabled"),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        RatingDemo { knobs: k.clone() }
    }
}

#[component]
fn Summary() -> Element {
    let average = use_signal(|| 4.5);
    rsx! {
        Group { gap: SpacingToken::Md, align: AlignItems::Center,
            Heading { order: HeadingOrder::H3, "4,5" }
            Stack { gap: SpacingToken::Xs,
                Rating { value: average, fractions: 2, readonly: true, size: 18, label: Some("Media 4,5 su 5".to_string()) }
                Text { size: TextSize::Sm, dimmed: true, "Su 1.284 recensioni verificate" }
            }
        }
    }
}
const SUMMARY_CODE: &str = r#"let average = use_signal(|| 4.5);

Group { gap: SpacingToken::Md, align: AlignItems::Center,
    Heading { order: HeadingOrder::H3, "4,5" }
    Stack { gap: SpacingToken::Xs,
        Rating { value: average, fractions: 2, readonly: true, size: 18, label: Some("Media 4,5 su 5".to_string()) }
        Text { size: TextSize::Sm, dimmed: true, "Su 1.284 recensioni verificate" }
    }
}"#;

#[component]
fn Icons() -> Element {
    let heat = use_signal(|| 3.0);
    let security = use_signal(|| 4.0);
    rsx! {
        Stack { gap: SpacingToken::Md,
            Rating { value: heat, icon: Some(IconKind::Flame), size: 24, label: Some("Priorità".to_string()) }
            Rating { value: security, icon: Some(IconKind::Shield), size: 24, label: Some("Livello di sicurezza".to_string()) }
        }
    }
}
const ICONS_CODE: &str = r#"let heat = use_signal(|| 3.0);
let security = use_signal(|| 4.0);

Stack { gap: SpacingToken::Md,
    Rating { value: heat, icon: Some(IconKind::Flame), size: 24, label: Some("Priorità".to_string()) }
    Rating { value: security, icon: Some(IconKind::Shield), size: 24, label: Some("Livello di sicurezza".to_string()) }
}"#;

#[component]
fn InContext() -> Element {
    let score = use_signal(|| 0.0);
    let comment = use_signal(String::new);
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Com'è andata con il supporto?".to_string()),
            subtitle: Some("Ticket #SF-2291 · chiuso da Marco".to_string()),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, "Più tardi" }
                    Button {
                        disabled: score() == 0.0,
                        onclick: move |_| {
                            toast.success("Grazie per il feedback");
                        },
                        "Invia valutazione"
                    }
                }
            }),
            Stack { gap: SpacingToken::Md,
                Rating { value: score, size: 28, label: Some("Valutazione del supporto".to_string()) }
                Textarea {
                    value: comment,
                    label: Some("Commento (facoltativo)".to_string()),
                    placeholder: Some("Cosa possiamo migliorare?".to_string()),
                    min_rows: 2,
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let score = use_signal(|| 0.0);
let comment = use_signal(String::new);
let mut toast = use_notification();

Card {
    title: Some("Com'è andata con il supporto?".to_string()),
    subtitle: Some("Ticket #SF-2291 · chiuso da Marco".to_string()),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button { variant: ButtonVariant::Ghost, "Più tardi" }
            Button {
                disabled: score() == 0.0,
                onclick: move |_| {
                    toast.success("Grazie per il feedback");
                },
                "Invia valutazione"
            }
        }
    }),
    Stack { gap: SpacingToken::Md,
        Rating { value: score, size: 28, label: Some("Valutazione del supporto".to_string()) }
        Textarea {
            value: comment,
            label: Some("Commento (facoltativo)".to_string()),
            placeholder: Some("Cosa possiamo migliorare?".to_string()),
            min_rows: 2,
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("value", "Signal<f64>", "-", "Valutazione corrente (0 = nessuna)."),
    prop("count", "usize", "5", "Numero di simboli."),
    prop("fractions", "usize", "1", "1 per valori interi, 2 per mezze valutazioni."),
    prop("icon", "Option<IconKind>", "None", "Simbolo al posto della stella (es. Flame, Shield)."),
    prop("size", "u32", "22", "Dimensione del simbolo in px."),
    prop("readonly", "bool", "false", "Solo visualizzazione, nessun hover."),
    prop("disabled", "bool", "false", "Inattivo, opacità --sd-disabled-opacity."),
    prop("label", "Option<String>", "None", "aria-label del controllo (da Locale se assente)."),
    prop("on_change", "Option<EventHandler<f64>>", "None", "Nuovo valore."),
    prop("id / class", "Option<String> / String", "None / \"\"", "id e classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "rating",
        name: "Rating",
        category: Category::Form,
        features: &["rating"],
        icon: IconKind::Flame,
        tagline: "Valutazione a stelle o con un simbolo a scelta, intera o a metà.",
        keywords: &["rating", "valutazione", "stelle", "recensione", "feedback", "punteggio"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Rating",
            controls: vec![
                Control::choice("count", "Simboli", COUNT_OPTS, 0),
                Control::choice("fractions", "Frazioni", FRACTION_OPTS, 0),
                Control::number("size", "Dimensione", 22.0, 14.0, 40.0, 2.0, 0),
                Control::bool("icon", "Icona Flame", false).emit(Emit::Template("icon: Some(IconKind::Flame)")),
                Control::bool("readonly", "Sola lettura", false),
                Control::bool("disabled", "Disabilitato", false),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["value,", "label: Some(\"Valutazione del servizio\".to_string()),"],
            children_code: None,
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "summary",
                title: "Media in sola lettura",
                description: "Mezze stelle per il riepilogo delle recensioni.",
                code: SUMMARY_CODE,
                render: || rsx! { Summary {} },
                span: Span::Half,
            },
            Example {
                id: "icons",
                title: "Simboli personalizzati",
                description: "Qualsiasi IconKind al posto della stella.",
                code: ICONS_CODE,
                render: || rsx! { Icons {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Feedback dopo la chiusura di un ticket: l'invio si attiva dopo il voto.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Rating", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Value fill (language.md §3.11)",
            depth: "Value fill su void",
            tokens: &["--sd-accent", "--sd-glow-sm", "--sd-text-subtle", "--sd-disabled-opacity", "--sd-focus-ring"],
            notes: "I simboli attivi sono riempimenti accento; quelli vuoti restano contorni subtle.",
        }),
        a11y: &[
            "Il controllo ha role=slider con aria-valuenow, aria-valuemin=0 e aria-valuemax=count.",
            "Frecce destra/su e sinistra/giù cambiano il valore di un passo (0,5 con fractions: 2).",
            "readonly e disabled sono annunciati con aria-readonly e aria-disabled.",
            "Senza label usa il testo di Locale (rating_label).",
        ],
        related: &["slider", "chip", "segmented-controls", "media-card"],
    }
}
