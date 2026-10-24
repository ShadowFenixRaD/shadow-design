//! Story: Stack (feature `stack`).
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
const ALIGNS: [AlignItems; 4] = [AlignItems::Start, AlignItems::Center, AlignItems::End, AlignItems::Stretch];
const ALIGN_OPTS: &[ChoiceOption] = &[
    opt("Start", "AlignItems::Start"),
    opt("Center", "AlignItems::Center"),
    opt("End", "AlignItems::End"),
    opt("Stretch", "AlignItems::Stretch"),
];
const JUSTIFIES: [JustifyContent; 5] = [
    JustifyContent::Start,
    JustifyContent::Center,
    JustifyContent::End,
    JustifyContent::SpaceBetween,
    JustifyContent::SpaceEvenly,
];
const JUSTIFY_OPTS: &[ChoiceOption] = &[
    opt("Start", "JustifyContent::Start"),
    opt("Center", "JustifyContent::Center"),
    opt("End", "JustifyContent::End"),
    opt("Between", "JustifyContent::SpaceBetween"),
    opt("Evenly", "JustifyContent::SpaceEvenly"),
];

const CHILDREN: &str = r#"DemoCell { index: Some(1), label: Some("Intestazione".to_string()), accent: true }
DemoCell { index: Some(2), label: Some("Contenuto".to_string()) }
DemoCell { index: Some(3), label: Some("Azioni".to_string()) }"#;

fn render(k: &Knobs) -> Element {
    rsx! {
        Stack {
            class: "h-64",
            gap: k.pick("gap", &GAPS),
            align: k.pick("align", &ALIGNS),
            justify: k.pick("justify", &JUSTIFIES),
            DemoCell { index: Some(1), label: Some("Intestazione".to_string()), accent: true }
            DemoCell { index: Some(2), label: Some("Contenuto".to_string()) }
            DemoCell { index: Some(3), label: Some("Azioni".to_string()) }
        }
    }
}

#[component]
fn Gaps() -> Element {
    rsx! {
        SimpleGrid { cols: 3, spacing: GridSpacing::Md,
            Stack { gap: SpacingToken::Xs,
                DemoCell { label: Some("Xs".to_string()), short: true }
                DemoCell { short: true }
                DemoCell { short: true }
            }
            Stack { gap: SpacingToken::Md,
                DemoCell { label: Some("Md".to_string()), short: true }
                DemoCell { short: true }
                DemoCell { short: true }
            }
            Stack { gap: SpacingToken::Xl,
                DemoCell { label: Some("Xl".to_string()), short: true }
                DemoCell { short: true }
                DemoCell { short: true }
            }
        }
    }
}
const GAPS_CODE: &str = r#"SimpleGrid { cols: 3, spacing: GridSpacing::Md,
    Stack { gap: SpacingToken::Xs,
        DemoCell { label: Some("Xs".to_string()), short: true }
        DemoCell { short: true }
        DemoCell { short: true }
    }
    Stack { gap: SpacingToken::Md,
        DemoCell { label: Some("Md".to_string()), short: true }
        DemoCell { short: true }
        DemoCell { short: true }
    }
    Stack { gap: SpacingToken::Xl,
        DemoCell { label: Some("Xl".to_string()), short: true }
        DemoCell { short: true }
        DemoCell { short: true }
    }
}"#;

#[component]
fn Centered() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
            Avatar { name: Some("Marco Lenzi".to_string()), size: AvatarSize::Lg }
            Text { "Marco Lenzi" }
            Text { size: TextSize::Sm, dimmed: true, "Security auditor" }
            Badge { label: "Revisore".to_string(), variant: BadgeVariant::Primary, pill: true }
        }
    }
}
const CENTERED_CODE: &str = r#"Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
    Avatar { name: Some("Marco Lenzi".to_string()), size: AvatarSize::Lg }
    Text { "Marco Lenzi" }
    Text { size: TextSize::Sm, dimmed: true, "Security auditor" }
    Badge { label: "Revisore".to_string(), variant: BadgeVariant::Primary, pill: true }
}"#;

#[component]
fn InContext() -> Element {
    let name = use_signal(|| "edge-07".to_string());
    let region = use_signal(|| "Francoforte".to_string());
    let auto = use_signal(|| true);
    rsx! {
        Card { title: Some("Nuovo nodo".to_string()), subtitle: Some("Registra un nodo nella mesh".to_string()),
            Stack { gap: SpacingToken::Md,
                TextInput { value: name, label: Some("Nome host".to_string()) }
                TextInput { value: region, label: Some("Regione".to_string()) }
                Switch { checked: auto, label: Some("Aggiornamenti automatici".to_string()) }
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, "Annulla" }
                    Button { variant: ButtonVariant::Primary, "Registra" }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let name = use_signal(|| "edge-07".to_string());
let region = use_signal(|| "Francoforte".to_string());
let auto = use_signal(|| true);

Card { title: Some("Nuovo nodo".to_string()), subtitle: Some("Registra un nodo nella mesh".to_string()),
    Stack { gap: SpacingToken::Md,
        TextInput { value: name, label: Some("Nome host".to_string()) }
        TextInput { value: region, label: Some("Regione".to_string()) }
        Switch { checked: auto, label: Some("Aggiornamenti automatici".to_string()) }
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button { variant: ButtonVariant::Ghost, "Annulla" }
            Button { variant: ButtonVariant::Primary, "Registra" }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("gap", "SpacingToken", "Md", "Spazio verticale tra gli elementi: None, Xs, Sm, Md, Lg, Xl."),
    prop("align", "AlignItems", "Stretch", "Allineamento orizzontale: Start, Center, End, Stretch, Baseline."),
    prop("justify", "JustifyContent", "Start", "Distribuzione verticale quando lo Stack ha un'altezza."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];

pub fn story() -> Story {
    Story {
        slug: "stack",
        name: "Stack",
        category: Category::Layout,
        features: &["stack"],
        icon: IconKind::Layers,
        tagline: "Impila gli elementi in colonna con una spaziatura uniforme a token.",
        keywords: &["stack", "colonna", "column", "verticale", "flex", "spaziatura", "gap"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Stack",
            controls: vec![
                Control::choice("gap", "Gap", GAP_OPTS, 3),
                Control::choice("align", "Allineamento", ALIGN_OPTS, 3),
                Control::choice("justify", "Distribuzione", JUSTIFY_OPTS, 0)
                    .hint("Agisce solo se lo Stack ha un'altezza: qui h-64."),
            ],
            render,
            prelude_code: None,
            fixed_code: &["class: \"h-64\","],
            children_code: Some(CHILDREN),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "gaps",
                title: "Spaziature",
                description: "Lo stesso Stack con gap Xs, Md e Xl.",
                code: GAPS_CODE,
                render: || rsx! { Gaps {} },
                span: Span::Half,
            },
            Example {
                id: "centered",
                title: "Centrato",
                description: "Scheda profilo con align Center.",
                code: CENTERED_CODE,
                render: || rsx! { Centered {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il ritmo verticale di un form dentro una card.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Stack", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Primitiva di layout (nessuna superficie, language.md §2 Void)",
            depth: "Void: dispone, non disegna",
            tokens: &["--sd-space-1", "--sd-space-2", "--sd-space-4", "--sd-space-6", "--sd-space-8"],
            notes: "Il ritmo verticale viene dal gap, mai da margini sui figli.",
        }),
        a11y: &[
            "Contenitore div senza ruolo: l'ordine di lettura è quello del sorgente.",
            "align e justify cambiano solo la posizione visiva, non l'ordine del focus.",
            "Nessuno stile sui figli: le etichette dei campi restano associate ai rispettivi input.",
        ],
        related: &["group", "grid", "center", "fieldset"],
    }
}
