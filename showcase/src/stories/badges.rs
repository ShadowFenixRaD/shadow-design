//! Story: Badge (feature `badge`). Stateless: the playground renders straight from the knobs.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::scenes::OpsDashboard;
use crate::story::schema::*;

const VARIANTS: [BadgeVariant; 6] = [
    BadgeVariant::Default,
    BadgeVariant::Primary,
    BadgeVariant::Success,
    BadgeVariant::Warning,
    BadgeVariant::Danger,
    BadgeVariant::Outline,
];
const VARIANT_OPTS: &[ChoiceOption] = &[
    opt("Default", "BadgeVariant::Default"),
    opt("Primary", "BadgeVariant::Primary"),
    opt("Success", "BadgeVariant::Success"),
    opt("Warning", "BadgeVariant::Warning"),
    opt("Danger", "BadgeVariant::Danger"),
    opt("Outline", "BadgeVariant::Outline"),
];

fn render(k: &Knobs) -> Element {
    rsx! {
        Badge {
            label: k.text("label"),
            variant: k.pick("variant", &VARIANTS),
            pill: k.bool("pill"),
            dot: k.bool("dot"),
            glow: k.bool("glow").then_some(true),
        }
    }
}

#[component]
fn Variants() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm,
            Badge { label: "Default".to_string() }
            Badge { label: "Primary".to_string(), variant: BadgeVariant::Primary }
            Badge { label: "Success".to_string(), variant: BadgeVariant::Success }
            Badge { label: "Warning".to_string(), variant: BadgeVariant::Warning }
            Badge { label: "Danger".to_string(), variant: BadgeVariant::Danger }
            Badge { label: "Outline".to_string(), variant: BadgeVariant::Outline }
        }
    }
}
const VARIANTS_CODE: &str = r#"Group { gap: SpacingToken::Sm,
    Badge { label: "Default".to_string() }
    Badge { label: "Primary".to_string(), variant: BadgeVariant::Primary }
    Badge { label: "Success".to_string(), variant: BadgeVariant::Success }
    Badge { label: "Warning".to_string(), variant: BadgeVariant::Warning }
    Badge { label: "Danger".to_string(), variant: BadgeVariant::Danger }
    Badge { label: "Outline".to_string(), variant: BadgeVariant::Outline }
}"#;

#[component]
fn StatusPills() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm,
            Badge { label: "Online".to_string(), variant: BadgeVariant::Success, pill: true, dot: true }
            Badge { label: "Degradato".to_string(), variant: BadgeVariant::Warning, pill: true, dot: true }
            Badge { label: "Offline".to_string(), variant: BadgeVariant::Danger, pill: true, dot: true }
            Badge { label: "In manutenzione".to_string(), pill: true, dot: true }
        }
    }
}
const STATUS_CODE: &str = r#"Group { gap: SpacingToken::Sm,
    Badge { label: "Online".to_string(), variant: BadgeVariant::Success, pill: true, dot: true }
    Badge { label: "Degradato".to_string(), variant: BadgeVariant::Warning, pill: true, dot: true }
    Badge { label: "Offline".to_string(), variant: BadgeVariant::Danger, pill: true, dot: true }
    Badge { label: "In manutenzione".to_string(), pill: true, dot: true }
}"#;

#[component]
fn Inline() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Md,
            Group { gap: SpacingToken::Sm,
                Heading { order: HeadingOrder::H4, "Runtime edge" }
                Badge { label: "Beta".to_string(), variant: BadgeVariant::Primary, pill: true }
            }
            Group { gap: SpacingToken::Sm,
                Text { size: TextSize::Sm, "Richieste di revisione" }
                Badge { label: "12".to_string(), variant: BadgeVariant::Outline, pill: true }
            }
            Group { gap: SpacingToken::Sm,
                Code { "v0.5.0" }
                Badge { label: "Ultima".to_string(), variant: BadgeVariant::Success }
            }
        }
    }
}
const INLINE_CODE: &str = r#"Stack { gap: SpacingToken::Md,
    Group { gap: SpacingToken::Sm,
        Heading { order: HeadingOrder::H4, "Runtime edge" }
        Badge { label: "Beta".to_string(), variant: BadgeVariant::Primary, pill: true }
    }
    Group { gap: SpacingToken::Sm,
        Text { size: TextSize::Sm, "Richieste di revisione" }
        Badge { label: "12".to_string(), variant: BadgeVariant::Outline, pill: true }
    }
    Group { gap: SpacingToken::Sm,
        Code { "v0.5.0" }
        Badge { label: "Ultima".to_string(), variant: BadgeVariant::Success }
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        OpsDashboard {}
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/ops_dashboard.rs (estratto): stato dei nodi in tabella e indicatore Live sul grafico.
DataTableColumn::new("status", "Stato", |r: &NodeRow| {
    let status = r.status;
    rsx! {
        Badge { label: status.to_string(), variant: status_badge(status), pill: true, dot: true }
    }
})

Badge { label: "Live".to_string(), variant: BadgeVariant::Success, pill: true, dot: true }"#;

const PROPS: &[PropDoc] = &[
    prop("label", "String", "-", "Testo del badge, breve."),
    prop("variant", "BadgeVariant", "Default", "Default, Primary, Success, Warning, Danger (tinte soft) e Outline."),
    prop("pill", "bool", "false", "Forma a capsula (--sd-radius-full), indipendente dalla scala dei raggi."),
    prop("dot", "bool", "false", "Pallino di stato con alone pulsante a sinistra del testo."),
    prop("glow", "Option<bool>", "None", "Glow opzionale."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "badges",
        name: "Badge",
        category: Category::Dati,
        features: &["badge"],
        icon: IconKind::CheckCircle,
        tagline: "Etichetta breve per stati, conteggi e categorie, con tinte semantiche e pallino di stato.",
        keywords: &["badge", "etichetta", "tag", "stato", "status", "pill", "contatore"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Badge",
            controls: vec![
                Control::text("label", "Etichetta", "Online").emit(Emit::Template("label: \"{v}\".to_string()")),
                Control::choice("variant", "Variante", VARIANT_OPTS, 2).emit(Emit::Always),
                Control::bool("pill", "Pill", true).emit(Emit::Template("pill: true")),
                Control::bool("dot", "Pallino", true).emit(Emit::Template("dot: true")),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: None,
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "variants",
                title: "Varianti",
                description: "Tinte soft semantiche, Primary per l'accento, Outline neutro.",
                code: VARIANTS_CODE,
                render: || rsx! { Variants {} },
                span: Span::Half,
            },
            Example {
                id: "status",
                title: "Stato",
                description: "Pill con pallino per lo stato dei servizi.",
                code: STATUS_CODE,
                render: || rsx! { StatusPills {} },
                span: Span::Half,
            },
            Example {
                id: "inline",
                title: "Accanto al testo",
                description: "Titoli, conteggi e versioni.",
                code: INLINE_CODE,
                render: || rsx! { Inline {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Stato dei nodi e indicatore Live nella dashboard operativa.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Badge", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Semantic surface (language.md §3.10)",
            depth: "Segno piccolo sulla superficie che lo contiene",
            tokens: &["--sd-success-soft", "--sd-success-border", "--sd-success-strong", "--sd-accent-soft", "--sd-accent-strong", "--sd-radius-full", "--sd-text-2xs"],
            notes: "Mai un riempimento pieno: tinta soft, bordo e testo strong; il colore pieno resta solo sul pallino.",
        }),
        a11y: &[
            "Elemento <span> con testo: lo stato è sempre scritto, il colore non è l'unico segnale.",
            "Il pallino è un elemento vuoto, ignorato dagli screen reader.",
            "L'alone del pallino è un'animazione decorativa, rimossa con prefers-reduced-motion.",
            "I conteggi vanno accompagnati da un'etichetta che dica cosa si conta.",
        ],
        related: &["indicator", "chip", "alerts", "avatars"],
    }
}
