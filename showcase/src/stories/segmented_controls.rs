//! Story: SegmentedControl (feature `segmented_control`). Stateful: the playground goes through
//! `SegmentedDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const SIZES: [SegmentedControlSize; 3] = [SegmentedControlSize::Sm, SegmentedControlSize::Md, SegmentedControlSize::Lg];
const SIZE_OPTS: &[ChoiceOption] = &[
    opt("Sm", "SegmentedControlSize::Sm"),
    opt("Md", "SegmentedControlSize::Md"),
    opt("Lg", "SegmentedControlSize::Lg"),
];

const PRELUDE: &str = r#"let value = use_signal(|| "24h".to_string());
let options = vec![
    SegmentOption::new("1h", "1 ora"),
    SegmentOption::new("24h", "24 ore"),
    SegmentOption::new("7d", "7 giorni"),
    SegmentOption::new("30d", "30 giorni"),
];"#;

fn ranges() -> Vec<SegmentOption> {
    vec![
        SegmentOption::new("1h", "1 ora"),
        SegmentOption::new("24h", "24 ore"),
        SegmentOption::new("7d", "7 giorni"),
        SegmentOption::new("30d", "30 giorni"),
    ]
}

#[component]
fn SegmentedDemo(knobs: Knobs) -> Element {
    let value = use_signal(|| "24h".to_string());
    rsx! {
        SegmentedControl {
            options: ranges(),
            value,
            size: knobs.pick("size", &SIZES),
            full_width: knobs.bool("full_width"),
            disabled: knobs.bool("disabled"),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        SegmentedDemo { knobs: k.clone() }
    }
}

#[component]
fn WithIcons() -> Element {
    let view = use_signal(|| "list".to_string());
    rsx! {
        SegmentedControl {
            options: vec![
                SegmentOption::new("list", "Elenco").with_icon(IconKind::Menu.into()),
                SegmentOption::new("grid", "Griglia").with_icon(IconKind::Box.into()),
                SegmentOption::new("code", "Codice").with_icon(IconKind::Code.into()),
            ],
            value: view,
        }
    }
}
const WITH_ICONS_CODE: &str = r#"let view = use_signal(|| "list".to_string());

SegmentedControl {
    options: vec![
        SegmentOption::new("list", "Elenco").with_icon(IconKind::Menu.into()),
        SegmentOption::new("grid", "Griglia").with_icon(IconKind::Box.into()),
        SegmentOption::new("code", "Codice").with_icon(IconKind::Code.into()),
    ],
    value: view,
}"#;

#[component]
fn Sizes() -> Element {
    let small = use_signal(|| "eu".to_string());
    let large = use_signal(|| "pro".to_string());
    rsx! {
        Stack { gap: SpacingToken::Md, align: AlignItems::Start,
            SegmentedControl {
                options: vec![
                    SegmentOption::new("eu", "EU"),
                    SegmentOption::new("us", "US"),
                    SegmentOption::new("asia", "Asia").with_disabled(true),
                ],
                value: small,
                size: SegmentedControlSize::Sm,
            }
            SegmentedControl {
                options: vec![SegmentOption::new("free", "Free"), SegmentOption::new("pro", "Pro"), SegmentOption::new("team", "Team")],
                value: large,
                size: SegmentedControlSize::Lg,
            }
        }
    }
}
const SIZES_CODE: &str = r#"let small = use_signal(|| "eu".to_string());
let large = use_signal(|| "pro".to_string());

Stack { gap: SpacingToken::Md, align: AlignItems::Start,
    SegmentedControl {
        options: vec![
            SegmentOption::new("eu", "EU"),
            SegmentOption::new("us", "US"),
            SegmentOption::new("asia", "Asia").with_disabled(true),
        ],
        value: small,
        size: SegmentedControlSize::Sm,
    }
    SegmentedControl {
        options: vec![SegmentOption::new("free", "Free"), SegmentOption::new("pro", "Pro"), SegmentOption::new("team", "Team")],
        value: large,
        size: SegmentedControlSize::Lg,
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::OpsDashboard { compact: false }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/ops_dashboard.rs (estratto): l'intervallo del grafico nella testata della dashboard.
let range = use_signal(|| "7g".to_string());
let data = traffic(&range());

SegmentedControl {
    size: SegmentedControlSize::Sm,
    options: vec![
        SegmentOption::new("24h", "24h"),
        SegmentOption::new("7g", "7g"),
        SegmentOption::new("30g", "30g"),
    ],
    value: range,
}"#;

const PROPS: &[PropDoc] = &[
    prop("options", "Vec<SegmentOption>", "-", "Segmenti: SegmentOption::new(value, label)."),
    prop("value", "Signal<String>", "-", "Valore del segmento selezionato."),
    prop("size", "SegmentedControlSize", "Md", "Sm, Md o Lg."),
    prop("full_width", "bool", "false", "Distribuisce i segmenti su tutta la larghezza."),
    prop("disabled", "bool", "false", "Disabilita tutti i segmenti."),
    prop("on_change", "Option<EventHandler<String>>", "None", "Nuovo valore."),
    prop("class", "String", "\"\"", "Classi aggiuntive sulla traccia."),
];
const OPTION_PROPS: &[PropDoc] = &[
    prop("SegmentOption::new(value, label)", "fn", "-", "Costruttore."),
    prop(".with_icon(icon)", "fn(SvgIcon)", "-", "Icona prima dell'etichetta (IconKind::X.into())."),
    prop(".with_disabled(bool)", "fn", "-", "Segmento non selezionabile."),
];

pub fn story() -> Story {
    Story {
        slug: "segmented-controls",
        name: "SegmentedControl",
        category: Category::Form,
        features: &["segmented_control"],
        icon: IconKind::Layers,
        tagline: "Scelta esclusiva tra due e cinque opzioni brevi, tutte visibili in una traccia.",
        keywords: &["segmented control", "segmenti", "toggle group", "intervallo", "vista", "tab"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "SegmentedControl",
            controls: vec![
                Control::choice("size", "Dimensione", SIZE_OPTS, 1),
                Control::bool("full_width", "Larghezza piena", false),
                Control::bool("disabled", "Disabilitato", false),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["options,", "value,"],
            children_code: None,
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "icons",
                title: "Con icone",
                description: "Cambio di vista con icona e testo.",
                code: WITH_ICONS_CODE,
                render: || rsx! { WithIcons {} },
                span: Span::Half,
            },
            Example {
                id: "sizes",
                title: "Dimensioni e segmento disabilitato",
                description: "Sm per le barre strumenti, Lg per le scelte principali.",
                code: SIZES_CODE,
                render: || rsx! { Sizes {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "L'intervallo del traffico nella dashboard operativa.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "SegmentedControl", rows: PROPS },
            PropGroup { component: "SegmentOption", rows: OPTION_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Items in a track (§3.4) + selected (§3.7)",
            depth: "Well (traccia) · item selezionato",
            tokens: &["--sd-well", "--sd-border", "--sd-radius-md", "--sd-radius-sm", "--sd-item-h", "--sd-accent-soft", "--sd-glow-sm"],
            notes: "La traccia è un pozzetto; i segmenti sono item un passo di raggio più piccoli, uno solo selezionato.",
        }),
        a11y: &[
            "La traccia ha role=radiogroup; ogni segmento è un <button> con role=radio e aria-checked.",
            "Tab raggiunge i segmenti, Invio e Spazio selezionano.",
            "I segmenti disabilitati usano l'attributo disabled nativo.",
            "Le icone sono decorative: l'etichetta testuale resta sempre visibile.",
        ],
        related: &["tabs", "radio", "chip", "buttons"],
    }
}
