//! Story: PinInput (feature `pin_input`). Stateful: the playground goes through `PinDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const LENGTHS: [usize; 3] = [4, 6, 8];
const LENGTH_OPTS: &[ChoiceOption] = &[opt("4", "4"), opt("6", "6"), opt("8", "8")];
const SIZES: [ControlSize; 3] = [ControlSize::Sm, ControlSize::Md, ControlSize::Lg];
const SIZE_OPTS: &[ChoiceOption] = &[opt("Sm", "ControlSize::Sm"), opt("Md", "ControlSize::Md"), opt("Lg", "ControlSize::Lg")];

const PRELUDE: &str = "let value = use_signal(String::new);";

#[component]
fn PinDemo(knobs: Knobs) -> Element {
    let value = use_signal(String::new);
    rsx! {
        PinInput {
            value,
            aria_label: Some("Codice di verifica".to_string()),
            length: knobs.pick("length", &LENGTHS),
            size: knobs.pick("size", &SIZES),
            numeric_only: knobs.bool("numeric_only"),
            mask: knobs.bool("mask"),
            placeholder: Some(knobs.text("placeholder")).filter(|s| !s.is_empty()),
            error: knobs.bool("error"),
            disabled: knobs.bool("disabled"),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        PinDemo { knobs: k.clone() }
    }
}

#[component]
fn Sizes() -> Element {
    let small = use_signal(|| "12".to_string());
    let medium = use_signal(|| "1234".to_string());
    let large = use_signal(String::new);
    rsx! {
        Stack { gap: SpacingToken::Md, align: AlignItems::Center,
            PinInput { value: small, size: ControlSize::Sm, numeric_only: true }
            PinInput { value: medium, size: ControlSize::Md, numeric_only: true }
            PinInput { value: large, size: ControlSize::Lg, numeric_only: true }
        }
    }
}
const SIZES_CODE: &str = r#"let small = use_signal(|| "12".to_string());
let medium = use_signal(|| "1234".to_string());
let large = use_signal(String::new);

Stack { gap: SpacingToken::Md, align: AlignItems::Center,
    PinInput { value: small, size: ControlSize::Sm, numeric_only: true }
    PinInput { value: medium, size: ControlSize::Md, numeric_only: true }
    PinInput { value: large, size: ControlSize::Lg, numeric_only: true }
}"#;

#[component]
fn Modes() -> Element {
    let pin = use_signal(|| "4821".to_string());
    let voucher = use_signal(|| "SF7".to_string());
    rsx! {
        Stack { gap: SpacingToken::Lg, align: AlignItems::Center,
            PinInput {
                value: pin,
                length: 4,
                mask: true,
                numeric_only: true,
                aria_label: Some("PIN di sblocco".to_string()),
            }
            PinInput {
                value: voucher,
                length: 5,
                placeholder: Some("•".to_string()),
                aria_label: Some("Codice voucher".to_string()),
            }
        }
    }
}
const MODES_CODE: &str = r#"let pin = use_signal(|| "4821".to_string());
let voucher = use_signal(|| "SF7".to_string());

Stack { gap: SpacingToken::Lg, align: AlignItems::Center,
    PinInput {
        value: pin,
        length: 4,
        mask: true,
        numeric_only: true,
        aria_label: Some("PIN di sblocco".to_string()),
    }
    PinInput {
        value: voucher,
        length: 5,
        placeholder: Some("•".to_string()),
        aria_label: Some("Codice voucher".to_string()),
    }
}"#;

#[component]
fn Complete() -> Element {
    let code = use_signal(String::new);
    let mut wrong = use_signal(|| false);
    let mut toast = use_notification();
    rsx! {
        Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
            PinInput {
                value: code,
                length: 6,
                numeric_only: true,
                error: wrong(),
                aria_label: Some("Codice a sei cifre".to_string()),
                on_change: move |_| wrong.set(false),
                on_complete: move |v: String| {
                    if v == "424242" {
                        toast.success("Dispositivo associato");
                    } else {
                        wrong.set(true);
                    }
                },
            }
            Text { size: TextSize::Sm, dimmed: true, "Il codice corretto è 424242." }
        }
    }
}
const COMPLETE_CODE: &str = r#"let code = use_signal(String::new);
let mut wrong = use_signal(|| false);
let mut toast = use_notification();

Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
    PinInput {
        value: code,
        length: 6,
        numeric_only: true,
        error: wrong(),
        aria_label: Some("Codice a sei cifre".to_string()),
        on_change: move |_| wrong.set(false),
        on_complete: move |v: String| {
            if v == "424242" {
                toast.success("Dispositivo associato");
            } else {
                wrong.set(true);
            }
        },
    }
    Text { size: TextSize::Sm, dimmed: true, "Il codice corretto è 424242." }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::AuthCard { compact: false }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/auth_card.rs (estratto): il secondo fattore dopo email e password.
Divider { label: Some("Secondo fattore".to_string()) }
PinInput {
    value: code,
    length: 6,
    numeric_only: true,
    error: wrong(),
    aria_label: Some("Codice a sei cifre".to_string()),
    on_change: move |_| wrong.set(false),
    on_complete: move |v: String| {
        if v == DEMO_CODE {
            toast.success("Accesso verificato");
        } else {
            wrong.set(true);
        }
    },
}"#;

const PROPS: &[PropDoc] = &[
    prop("value", "Signal<String>", "-", "Codice completo, già filtrato e troncato a length."),
    prop("length", "usize", "4", "Numero di celle (tipicamente 4 o 6)."),
    prop("numeric_only", "bool", "false", "Accetta solo cifre 0-9 e apre il tastierino numerico."),
    prop("mask", "bool", "false", "Nasconde i caratteri come una password."),
    prop("placeholder", "Option<String>", "None", "Carattere mostrato nelle celle vuote (es. •)."),
    prop("size", "ControlSize", "Md", "Sm, Md o Lg."),
    prop("gap", "Option<String>", "None", "Distanza tra le celle (valore CSS, es. var(--sd-space-3))."),
    prop("error", "bool", "false", "Bordo danger e aria-invalid su ogni cella."),
    prop("disabled", "bool", "false", "Celle non modificabili."),
    prop("auto_focus", "bool", "false", "Mette il focus sulla prima cella al montaggio."),
    prop("aria_label", "Option<String>", "None", "Nome del gruppo di celle per gli screen reader."),
    prop("on_change", "Option<EventHandler<String>>", "None", "Ogni modifica del codice."),
    prop("on_complete", "Option<EventHandler<String>>", "None", "Chiamato quando tutte le celle sono piene."),
    prop("class", "Option<String>", "None", "Classi aggiuntive sul contenitore."),
];

pub fn story() -> Story {
    Story {
        slug: "pin-input",
        name: "PinInput",
        category: Category::Form,
        features: &["pin_input"],
        icon: IconKind::Key,
        tagline: "Codici PIN e OTP a celle separate, con avanzamento automatico e incolla su tutte le celle.",
        keywords: &["pin", "otp", "2fa", "codice", "verifica", "one-time password"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "PinInput",
            controls: vec![
                Control::choice("length", "Lunghezza", LENGTH_OPTS, 1).emit(Emit::Always),
                Control::choice("size", "Dimensione", SIZE_OPTS, 1),
                Control::bool("numeric_only", "Solo cifre", true).emit(Emit::Template("numeric_only: true")),
                Control::bool("mask", "Maschera", false),
                Control::text("placeholder", "Segnaposto", "").emit(Emit::Template("placeholder: Some(\"{v}\".to_string())")),
                Control::bool("error", "Errore", false),
                Control::bool("disabled", "Disabilitato", false),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["value,", "aria_label: Some(\"Codice di verifica\".to_string()),"],
            children_code: None,
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "sizes",
                title: "Dimensioni",
                description: "Sm, Md e Lg condividono l'altezza dei controlli.",
                code: SIZES_CODE,
                render: || rsx! { Sizes {} },
                span: Span::Half,
            },
            Example {
                id: "modes",
                title: "Maschera e alfanumerico",
                description: "PIN nascosto e codice voucher con segnaposto.",
                code: MODES_CODE,
                render: || rsx! { Modes {} },
                span: Span::Half,
            },
            Example {
                id: "complete",
                title: "Verifica al completamento",
                description: "on_complete controlla il codice; on_change azzera l'errore.",
                code: COMPLETE_CODE,
                render: || rsx! { Complete {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Accesso con secondo fattore: prova 424242 o un codice errato.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "PinInput", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Recessed field (language.md §3.6), una cella per carattere",
            depth: "Well",
            tokens: &["--sd-well", "--sd-border-strong", "--sd-inset-well", "--sd-radius-md", "--sd-accent", "--sd-glow-sm", "--sd-danger-border"],
            notes: "Ogni cella è un pozzetto; solo la cella con il focus prende bordo accento e glow.",
        }),
        a11y: &[
            "Il contenitore ha role=group con aria-label (da Locale se assente).",
            "Ogni cella ha un aria-label \"cifra n di N\" da Locale (pin_digit_label).",
            "Backspace torna alla cella precedente; incollare un codice lo distribuisce su tutte le celle.",
            "error imposta aria-invalid su ogni cella.",
        ],
        related: &["inputs", "number-input", "form", "alerts"],
    }
}
