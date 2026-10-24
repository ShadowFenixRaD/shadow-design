//! Story: Divider (feature `divider`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const ORIENTATIONS: [Orientation; 2] = [Orientation::Horizontal, Orientation::Vertical];
const ORIENTATION_OPTS: &[ChoiceOption] =
    &[opt("Orizzontale", "Orientation::Horizontal"), opt("Verticale", "Orientation::Vertical")];

/// The live divider sits between two short texts so its orientation reads at a glance.
fn render(k: &Knobs) -> Element {
    let orientation = k.pick("orientation", &ORIENTATIONS);
    let label = Some(k.text("label")).filter(|l| !l.is_empty());
    rsx! {
        if orientation == Orientation::Vertical {
            Group { class: "h-12", gap: SpacingToken::None,
                Text { size: TextSize::Sm, dimmed: true, "Bozze · 4" }
                Divider { orientation, label }
                Text { size: TextSize::Sm, dimmed: true, "Pubblicati · 18" }
            }
        } else {
            Stack { gap: SpacingToken::None,
                Text { size: TextSize::Sm, dimmed: true, "Ieri, 23:40 · Deploy di edge-03 completato" }
                Divider { orientation, label }
                Text { size: TextSize::Sm, dimmed: true, "Oggi, 08:12 · Rotazione delle chiavi avviata" }
            }
        }
    }
}

#[component]
fn Plain() -> Element {
    rsx! {
        Stack { gap: SpacingToken::None,
            Text { "Profilo" }
            Divider {}
            Text { "Sicurezza" }
            Divider {}
            Text { "Fatturazione" }
        }
    }
}
const PLAIN_CODE: &str = r#"Stack { gap: SpacingToken::None,
    Text { "Profilo" }
    Divider {}
    Text { "Sicurezza" }
    Divider {}
    Text { "Fatturazione" }
}"#;

#[component]
fn Vertical() -> Element {
    rsx! {
        Group { gap: SpacingToken::None,
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Copia" }
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Incolla" }
            Divider { orientation: Orientation::Vertical }
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Annulla" }
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Ripeti" }
        }
    }
}
const VERTICAL_CODE: &str = r#"Group { gap: SpacingToken::None,
    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Copia" }
    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Incolla" }
    Divider { orientation: Orientation::Vertical }
    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Annulla" }
    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Ripeti" }
}"#;

#[component]
fn Labelled() -> Element {
    rsx! {
        Stack { gap: SpacingToken::None,
            Text { size: TextSize::Sm, dimmed: true, "Hai sbloccato il canale cifrato." }
            Divider { label: Some("Messaggi non letti".to_string()) }
            Text { size: TextSize::Sm, "Il nodo edge-07 è tornato online alle 09:14." }
        }
    }
}
const LABELLED_CODE: &str = r#"Stack { gap: SpacingToken::None,
    Text { size: TextSize::Sm, dimmed: true, "Hai sbloccato il canale cifrato." }
    Divider { label: Some("Messaggi non letti".to_string()) }
    Text { size: TextSize::Sm, "Il nodo edge-07 è tornato online alle 09:14." }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::AuthCard { compact: true }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/auth_card.rs (estratto): il Divider con etichetta separa credenziali e secondo fattore.
Checkbox { checked: remember, label: Some("Ricordami su questo dispositivo".to_string()) }
Divider { label: Some("Secondo fattore".to_string()) }
PinInput { value: code, length: 6, numeric_only: true, error: wrong() }"#;

const PROPS: &[PropDoc] = &[
    prop("orientation", "Orientation", "Horizontal", "Horizontal (linea a tutta larghezza) o Vertical (alta quanto il genitore)."),
    prop("label", "Option<String>", "None", "Testo centrato tra due linee, in mono maiuscolo."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul separatore."),
];

pub fn story() -> Story {
    Story {
        slug: "dividers",
        name: "Divider",
        category: Category::Layout,
        features: &["divider"],
        icon: IconKind::Code,
        tagline: "Linea sottile che separa sezioni, gruppi di azioni o momenti di una conversazione.",
        keywords: &["divider", "separatore", "separator", "linea", "hr", "etichetta"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Divider",
            controls: vec![
                Control::choice("orientation", "Orientamento", ORIENTATION_OPTS, 0),
                Control::text("label", "Etichetta", "Oggi")
                    .emit(Emit::Template("label: Some(\"{v}\".to_string())"))
                    .hint("Svuota il campo per una linea semplice."),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: None,
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "plain",
                title: "Semplice",
                description: "Separa voci di una lista di impostazioni.",
                code: PLAIN_CODE,
                render: || rsx! { Plain {} },
                span: Span::Half,
            },
            Example {
                id: "vertical",
                title: "Verticale",
                description: "Divide due gruppi di azioni in una barra.",
                code: VERTICAL_CODE,
                render: || rsx! { Vertical {} },
                span: Span::Half,
            },
            Example {
                id: "labelled",
                title: "Con etichetta",
                description: "Segna un punto preciso di un flusso.",
                code: LABELLED_CODE,
                render: || rsx! { Labelled {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il passaggio al secondo fattore nella scena di accesso.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Divider", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Bordo hairline (language.md §3.1, divisori di header e footer)",
            depth: "Void: una sola linea, nessuna superficie",
            tokens: &["--sd-border", "--sd-space-3", "--sd-space-4", "--sd-font-mono", "--sd-text-2xs", "--sd-text-subtle"],
            notes: "Colore --sd-border, mai accento; l'etichetta segue lo stile delle eyebrow maiuscole.",
        }),
        a11y: &[
            "Rende role=separator, annunciato come separazione tra i contenuti.",
            "L'etichetta è testo reale nel DOM e viene letta insieme al separatore.",
            "Non riceve mai il focus: non interrompe la navigazione da tastiera.",
        ],
        related: &["stack", "group", "cards", "chat"],
    }
}
