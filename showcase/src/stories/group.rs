//! Story: Group (feature `group`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;
use crate::story::sections::DemoCell;

const GAPS: [SpacingToken; 6] =
    [SpacingToken::None, SpacingToken::Xs, SpacingToken::Sm, SpacingToken::Md, SpacingToken::Lg, SpacingToken::Xl];
const GAP_OPTS: &[ChoiceOption] = &[
    opt("None", "SpacingToken::None"),
    opt("Xs", "SpacingToken::Xs"),
    opt("Sm", "SpacingToken::Sm"),
    opt("Md", "SpacingToken::Md"),
    opt("Lg", "SpacingToken::Lg"),
    opt("Xl", "SpacingToken::Xl"),
];
const ALIGNS: [AlignItems; 5] = [AlignItems::Start, AlignItems::Center, AlignItems::End, AlignItems::Stretch, AlignItems::Baseline];
const ALIGN_OPTS: &[ChoiceOption] = &[
    opt("Start", "AlignItems::Start"),
    opt("Center", "AlignItems::Center"),
    opt("End", "AlignItems::End"),
    opt("Stretch", "AlignItems::Stretch"),
    opt("Baseline", "AlignItems::Baseline"),
];
const JUSTIFIES: [JustifyContent; 6] = [
    JustifyContent::Start,
    JustifyContent::Center,
    JustifyContent::End,
    JustifyContent::SpaceBetween,
    JustifyContent::SpaceAround,
    JustifyContent::SpaceEvenly,
];
const JUSTIFY_OPTS: &[ChoiceOption] = &[
    opt("Start", "JustifyContent::Start"),
    opt("Center", "JustifyContent::Center"),
    opt("End", "JustifyContent::End"),
    opt("SpaceBetween", "JustifyContent::SpaceBetween"),
    opt("SpaceAround", "JustifyContent::SpaceAround"),
    opt("SpaceEvenly", "JustifyContent::SpaceEvenly"),
];

const CHILDREN: &str = r#"DemoCell { index: Some(1), label: Some("Alto".to_string()), tall: true, accent: true }
DemoCell { index: Some(2), label: Some("Medio".to_string()) }
DemoCell { index: Some(3), label: Some("Basso".to_string()), short: true }
DemoCell { index: Some(4), label: Some("Medio".to_string()) }"#;

fn render(k: &Knobs) -> Element {
    rsx! {
        Group {
            gap: k.pick("gap", &GAPS),
            align: k.pick("align", &ALIGNS),
            justify: k.pick("justify", &JUSTIFIES),
            wrap: k.bool("wrap"),
            grow: k.bool("grow"),
            DemoCell { index: Some(1), label: Some("Alto".to_string()), tall: true, accent: true }
            DemoCell { index: Some(2), label: Some("Medio".to_string()) }
            DemoCell { index: Some(3), label: Some("Basso".to_string()), short: true }
            DemoCell { index: Some(4), label: Some("Medio".to_string()) }
        }
    }
}

#[component]
fn Toolbar() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button { variant: ButtonVariant::Ghost, "Annulla" }
            Button { variant: ButtonVariant::Secondary, "Salva bozza" }
            Button { variant: ButtonVariant::Primary, "Pubblica" }
        }
    }
}
const TOOLBAR_CODE: &str = r#"Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
    Button { variant: ButtonVariant::Ghost, "Annulla" }
    Button { variant: ButtonVariant::Secondary, "Salva bozza" }
    Button { variant: ButtonVariant::Primary, "Pubblica" }
}"#;

#[component]
fn Between() -> Element {
    rsx! {
        Group { justify: JustifyContent::SpaceBetween,
            Group { gap: SpacingToken::Sm,
                Icon { kind: IconKind::Server, size: 16 }
                Text { "edge-07.fra" }
            }
            Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
        }
    }
}
const BETWEEN_CODE: &str = r#"Group { justify: JustifyContent::SpaceBetween,
    Group { gap: SpacingToken::Sm,
        Icon { kind: IconKind::Server, size: 16 }
        Text { "edge-07.fra" }
    }
    Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
}"#;

#[component]
fn Grow() -> Element {
    rsx! {
        Group { grow: true, gap: SpacingToken::Sm,
            Button { variant: ButtonVariant::Secondary, "Giorno" }
            Button { variant: ButtonVariant::Secondary, "Settimana" }
            Button { variant: ButtonVariant::Secondary, "Mese" }
        }
    }
}
const GROW_CODE: &str = r#"Group { grow: true, gap: SpacingToken::Sm,
    Button { variant: ButtonVariant::Secondary, "Giorno" }
    Button { variant: ButtonVariant::Secondary, "Settimana" }
    Button { variant: ButtonVariant::Secondary, "Mese" }
}"#;

#[component]
fn Wrapping() -> Element {
    rsx! {
        Group { gap: SpacingToken::Xs,
            for tag in ["rust", "dioxus", "wasm", "scss", "tokio", "serde", "axum", "tauri"] {
                Badge { key: "{tag}", label: tag.to_string(), pill: true }
            }
        }
    }
}
const WRAPPING_CODE: &str = r#"Group { gap: SpacingToken::Xs,
    for tag in ["rust", "dioxus", "wasm", "scss", "tokio", "serde", "axum", "tauri"] {
        Badge { label: tag.to_string(), pill: true }
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut toast = use_notification();
    rsx! {
        Card {
            Group { justify: JustifyContent::SpaceBetween,
                Group { gap: SpacingToken::Sm,
                    Avatar { name: Some("Giulia Ferri".to_string()), status: Some(PresenceStatus::Online) }
                    Stack { gap: SpacingToken::None,
                        Text { "Giulia Ferri" }
                        Text { size: TextSize::Sm, dimmed: true, "Rust core · Milano" }
                    }
                }
                Group { gap: SpacingToken::Sm,
                    Badge { label: "Admin".to_string(), variant: BadgeVariant::Primary }
                    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Profilo" }
                    Button {
                        variant: ButtonVariant::Primary,
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            toast.success("Invito inviato a Giulia");
                        },
                        "Invita"
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut toast = use_notification();

Card {
    Group { justify: JustifyContent::SpaceBetween,
        Group { gap: SpacingToken::Sm,
            Avatar { name: Some("Giulia Ferri".to_string()), status: Some(PresenceStatus::Online) }
            Stack { gap: SpacingToken::None,
                Text { "Giulia Ferri" }
                Text { size: TextSize::Sm, dimmed: true, "Rust core · Milano" }
            }
        }
        Group { gap: SpacingToken::Sm,
            Badge { label: "Admin".to_string(), variant: BadgeVariant::Primary }
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Profilo" }
            Button {
                variant: ButtonVariant::Primary,
                size: ButtonSize::Sm,
                onclick: move |_| {
                    toast.success("Invito inviato a Giulia");
                },
                "Invita"
            }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("gap", "SpacingToken", "Md", "Spazio tra gli elementi: None, Xs, Sm, Md, Lg, Xl."),
    prop("align", "AlignItems", "Center", "Allineamento verticale: Start, Center, End, Stretch, Baseline."),
    prop("justify", "JustifyContent", "Start", "Distribuzione orizzontale: Start, Center, End, SpaceBetween, SpaceAround, SpaceEvenly."),
    prop("wrap", "bool", "true", "Manda a capo gli elementi quando lo spazio finisce."),
    prop("grow", "bool", "false", "Ogni figlio si allarga in parti uguali."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];

pub fn story() -> Story {
    Story {
        slug: "group",
        name: "Group",
        category: Category::Layout,
        features: &["group"],
        icon: IconKind::Layers,
        tagline: "Dispone gli elementi in riga con gap a token, allineamento e ritorno a capo.",
        keywords: &["group", "riga", "row", "flex", "toolbar", "allineamento", "gap"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Group",
            controls: vec![
                Control::choice("gap", "Gap", GAP_OPTS, 3),
                Control::choice("align", "Allineamento", ALIGN_OPTS, 1),
                Control::choice("justify", "Distribuzione", JUSTIFY_OPTS, 0),
                Control::bool("wrap", "A capo", true).hint("Visibile su schermi stretti."),
                Control::bool("grow", "Riempi la riga", false),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some(CHILDREN),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "toolbar",
                title: "Barra azioni",
                description: "Azioni allineate a destra, una sola primaria.",
                code: TOOLBAR_CODE,
                render: || rsx! { Toolbar {} },
                span: Span::Half,
            },
            Example {
                id: "between",
                title: "Agli estremi",
                description: "Etichetta a sinistra, stato a destra.",
                code: BETWEEN_CODE,
                render: || rsx! { Between {} },
                span: Span::Half,
            },
            Example {
                id: "grow",
                title: "Larghezze uguali",
                description: "grow divide la riga in parti uguali.",
                code: GROW_CODE,
                render: || rsx! { Grow {} },
                span: Span::Half,
            },
            Example {
                id: "wrap",
                title: "Ritorno a capo",
                description: "Una nuvola di tag che va a capo da sola.",
                code: WRAPPING_CODE,
                render: || rsx! { Wrapping {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Riga di un membro del team: identità a sinistra, azioni a destra.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Group", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Primitiva di layout (nessuna superficie, language.md §2 Void)",
            depth: "Void: dispone, non disegna",
            tokens: &["--sd-space-1", "--sd-space-2", "--sd-space-3", "--sd-space-4", "--sd-space-6"],
            notes: "Gap a token al posto dei margini: i figli non conoscono il proprio vicino.",
        }),
        a11y: &[
            "Contenitore div senza ruolo: annunciato solo attraverso i figli.",
            "justify e wrap non cambiano l'ordine del DOM né quello del focus.",
            "Per una barra di comandi aggiungi role=toolbar e un'etichetta su un contenitore esterno.",
        ],
        related: &["stack", "grid", "center", "buttons"],
    }
}
