//! Story: Card (feature `card`). Stateful playground (`CardDemo { knobs }`): the clickable
//! variant needs the notification hook.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const VARIANTS: [CardVariant; 3] = [CardVariant::Default, CardVariant::Elevated, CardVariant::Metric];
const VARIANT_OPTS: &[ChoiceOption] = &[
    opt("Default", "CardVariant::Default"),
    opt("Elevated", "CardVariant::Elevated"),
    opt("Metric", "CardVariant::Metric"),
];
const SHADOWS: [CardShadow; 5] = [CardShadow::None, CardShadow::Sm, CardShadow::Md, CardShadow::Lg, CardShadow::Xl];
const SHADOW_OPTS: &[ChoiceOption] = &[
    opt("None", "CardShadow::None"),
    opt("Sm", "CardShadow::Sm"),
    opt("Md", "CardShadow::Md"),
    opt("Lg", "CardShadow::Lg"),
    opt("Xl", "CardShadow::Xl"),
];

const PRELUDE: &str = "let mut toast = use_notification();";

#[component]
fn CardDemo(knobs: Knobs) -> Element {
    let mut toast = use_notification();
    let title = Some(knobs.text("title")).filter(|t| !t.is_empty());
    let subtitle = Some(knobs.text("subtitle")).filter(|t| !t.is_empty());
    let onclick = knobs.bool("onclick").then(|| {
        EventHandler::new(move |_: MouseEvent| {
            toast.info("Apertura di Cluster Mesh #07");
        })
    });
    rsx! {
        Card {
            variant: knobs.pick("variant", &VARIANTS),
            shadow: knobs.pick("shadow", &SHADOWS),
            title,
            subtitle,
            avatar: knobs.bool("avatar").then(|| rsx! {
                Avatar { name: Some("Giulia Ferri".to_string()), status: Some(PresenceStatus::Online) }
            }),
            progress: knobs.bool("progress").then_some(68.0),
            buttons: knobs.bool("buttons").then(|| rsx! {
                Badge { label: "Live".to_string(), variant: BadgeVariant::Success, dot: true }
            }),
            footer: knobs.bool("footer").then(|| rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Ispeziona" }
                    Button { size: ButtonSize::Sm, "Sincronizza" }
                }
            }),
            glow: knobs.bool("glow").then_some(true),
            onclick,
            Text { size: TextSize::Sm, dimmed: true, "4 nodi attivi su Kyber-1024. Ultimo handshake 2 minuti fa." }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        CardDemo { knobs: k.clone() }
    }
}

#[component]
fn Variants() -> Element {
    rsx! {
        SimpleGrid { cols: 1, sm: Some(3), spacing: GridSpacing::Sm,
            Card { title: Some("Default".to_string()), subtitle: Some("Flat-lit".to_string()),
                Text { size: TextSize::Sm, dimmed: true, "Il contenitore di base." }
            }
            Card {
                variant: CardVariant::Elevated,
                title: Some("Elevated".to_string()),
                subtitle: Some("Superficie 2".to_string()),
                Text { size: TextSize::Sm, dimmed: true, "Un gradino sopra la pagina." }
            }
            Card {
                variant: CardVariant::Metric,
                title: Some("Metric".to_string()),
                subtitle: Some("Latenza media".to_string()),
                Text { size: TextSize::Xl, "1.2 ms" }
            }
        }
    }
}
const VARIANTS_CODE: &str = r#"SimpleGrid { cols: 1, sm: Some(3), spacing: GridSpacing::Sm,
    Card { title: Some("Default".to_string()), subtitle: Some("Flat-lit".to_string()),
        Text { size: TextSize::Sm, dimmed: true, "Il contenitore di base." }
    }
    Card {
        variant: CardVariant::Elevated,
        title: Some("Elevated".to_string()),
        subtitle: Some("Superficie 2".to_string()),
        Text { size: TextSize::Sm, dimmed: true, "Un gradino sopra la pagina." }
    }
    Card {
        variant: CardVariant::Metric,
        title: Some("Metric".to_string()),
        subtitle: Some("Latenza media".to_string()),
        Text { size: TextSize::Xl, "1.2 ms" }
    }
}"#;

#[component]
fn Clickable() -> Element {
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Rapporto settimanale".to_string()),
            subtitle: Some("Generato lunedì alle 06:00".to_string()),
            onclick: move |_| {
                toast.info("Apertura del rapporto");
            },
            Text { size: TextSize::Sm, dimmed: true, "Tutta la card è cliccabile e risponde a Invio e Spazio." }
        }
    }
}
const CLICKABLE_CODE: &str = r#"let mut toast = use_notification();

Card {
    title: Some("Rapporto settimanale".to_string()),
    subtitle: Some("Generato lunedì alle 06:00".to_string()),
    onclick: move |_| {
        toast.info("Apertura del rapporto");
    },
    Text { size: TextSize::Sm, dimmed: true, "Tutta la card è cliccabile e risponde a Invio e Spazio." }
}"#;

#[component]
fn Progress() -> Element {
    rsx! {
        Card {
            title: Some("Migrazione del database".to_string()),
            subtitle: Some("Shard 3 di 5 · stima 12 minuti".to_string()),
            progress: Some(64.0),
            buttons: Some(rsx! {
                Badge { label: "In corso".to_string(), variant: BadgeVariant::Primary, dot: true }
            }),
            Text { size: TextSize::Sm, dimmed: true, "Le scritture restano attive durante la copia." }
        }
    }
}
const PROGRESS_CODE: &str = r#"Card {
    title: Some("Migrazione del database".to_string()),
    subtitle: Some("Shard 3 di 5 · stima 12 minuti".to_string()),
    progress: Some(64.0),
    buttons: Some(rsx! {
        Badge { label: "In corso".to_string(), variant: BadgeVariant::Primary, dot: true }
    }),
    Text { size: TextSize::Sm, dimmed: true, "Le scritture restano attive durante la copia." }
}"#;

#[component]
fn WithAvatar() -> Element {
    rsx! {
        Card {
            avatar: Some(rsx! {
                Avatar { name: Some("Marco Lenzi".to_string()), status: Some(PresenceStatus::Busy) }
            }),
            title: Some("Marco Lenzi".to_string()),
            subtitle: Some("Security auditor · in riunione".to_string()),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Profilo" }
                    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Scrivi" }
                }
            }),
            Text { size: TextSize::Sm, dimmed: true, "Sta revisionando la rotazione delle chiavi di edge-03." }
        }
    }
}
const WITH_AVATAR_CODE: &str = r#"Card {
    avatar: Some(rsx! {
        Avatar { name: Some("Marco Lenzi".to_string()), status: Some(PresenceStatus::Busy) }
    }),
    title: Some("Marco Lenzi".to_string()),
    subtitle: Some("Security auditor · in riunione".to_string()),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Profilo" }
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Scrivi" }
        }
    }),
    Text { size: TextSize::Sm, dimmed: true, "Sta revisionando la rotazione delle chiavi di edge-03." }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::OpsDashboard { compact: true }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/ops_dashboard.rs (estratto): tre card metriche e la card del grafico.
SimpleGrid { cols: 1, sm: metric_cols, spacing: GridSpacing::Sm,
    Metric {
        label: "Latenza media",
        value: "1.2 ms",
        delta: "−8% rispetto a ieri",
        data: vec![1.6, 1.5, 1.7, 1.4, 1.3, 1.25, 1.2],
        trend: TrendDirection::Positive,
    }
    // Richieste, Affidabilità…
}
Card { title: Some("Traffico per nodo".to_string()), subtitle: Some("Latenza, throughput ed errori".to_string()),
    buttons: Some(rsx! {
        Badge { label: "Live".to_string(), variant: BadgeVariant::Success, pill: true, dot: true }
    }),
    AreaChart { data, series, stacked: false, with_dots: false, height: 180 }
}"#;

const PROPS: &[PropDoc] = &[
    prop("title / subtitle", "Option<String>", "None", "Intestazione in Kanit e riga secondaria muted."),
    prop("avatar", "Option<Element>", "None", "Elemento a sinistra del titolo."),
    prop("buttons", "Option<Element>", "None", "Slot azioni o stato sotto il corpo."),
    prop("footer", "Option<Element>", "None", "Piè con tinta --sd-tint-1 e divisore."),
    prop("variant", "CardVariant", "Default", "Default, Elevated (superficie 2), Interactive, Metric."),
    prop("shadow", "CardShadow", "Md", "Elevazione: None, Sm, Md, Lg, Xl."),
    prop("progress", "Option<f32>", "None", "ProgressBar Sm in fondo alla card (0-100)."),
    prop("image / image_alt / image_height", "Option<String>", "None", "Immagine a tutta larghezza in fondo, con testo alternativo e altezza."),
    prop("onclick / on_click", "Option<EventHandler<MouseEvent>>", "None", "Rende la card attivabile: role=button, tabindex 0, Invio e Spazio."),
    prop("href", "Option<String>", "None", "Il titolo diventa un link esteso a tutta la card."),
    prop("glow", "Option<bool>", "None", "Glow opzionale --sd-glow-md (una card per gruppo)."),
    prop("hover_glow", "bool", "false", "Glow al passaggio, se glow non è impostato."),
    prop("overflow_hidden", "bool", "false", "Ritaglia il contenuto ai bordi (attenzione ai menu figli)."),
    prop("id / class", "Option<String> / String", "None / \"\"", "Id del contenitore e classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "cards",
        name: "Card",
        category: Category::Layout,
        features: &["card"],
        icon: IconKind::Box,
        tagline: "Contenitore di vetro con titolo, avatar, azioni, avanzamento e footer.",
        keywords: &["card", "scheda", "contenitore", "pannello", "panel", "metric", "tile"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Card",
            controls: vec![
                Control::choice("variant", "Variante", VARIANT_OPTS, 0),
                Control::choice("shadow", "Ombra", SHADOW_OPTS, 2),
                Control::text("title", "Titolo", "Cluster Mesh #07").emit(Emit::Template("title: Some(\"{v}\".to_string())")),
                Control::text("subtitle", "Sottotitolo", "Kyber-1024 · 4 nodi attivi")
                    .emit(Emit::Template("subtitle: Some(\"{v}\".to_string())")),
                Control::bool("avatar", "Avatar", false).emit(Emit::Template(
                    "avatar: Some(rsx! { Avatar { name: Some(\"Giulia Ferri\".to_string()), status: Some(PresenceStatus::Online) } })",
                )),
                Control::bool("buttons", "Stato (buttons)", true).emit(Emit::Template(
                    "buttons: Some(rsx! { Badge { label: \"Live\".to_string(), variant: BadgeVariant::Success, dot: true } })",
                )),
                Control::bool("progress", "Avanzamento", false).emit(Emit::Template("progress: Some(68.0)")),
                Control::bool("footer", "Footer", true).emit(Emit::Template(
                    "footer: Some(rsx! { Group { gap: SpacingToken::Sm, justify: JustifyContent::End, Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, \"Ispeziona\" } Button { size: ButtonSize::Sm, \"Sincronizza\" } } })",
                )),
                Control::bool("onclick", "Cliccabile", false)
                    .emit(Emit::Template("onclick: move |_| { toast.info(\"Apertura di Cluster Mesh #07\"); }")),
                Control::bool("glow", "Glow", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &[],
            children_code: Some(
                "Text { size: TextSize::Sm, dimmed: true, \"4 nodi attivi su Kyber-1024. Ultimo handshake 2 minuti fa.\" }",
            ),
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "variants",
                title: "Varianti",
                description: "Default, Elevated e Metric affiancate.",
                code: VARIANTS_CODE,
                render: || rsx! { Variants {} },
                span: Span::Full,
            },
            Example {
                id: "clickable",
                title: "Cliccabile",
                description: "onclick rende interattiva tutta la superficie.",
                code: CLICKABLE_CODE,
                render: || rsx! { Clickable {} },
                span: Span::Half,
            },
            Example {
                id: "progress",
                title: "Avanzamento",
                description: "Una barra di progresso agganciata al fondo.",
                code: PROGRESS_CODE,
                render: || rsx! { Progress {} },
                span: Span::Half,
            },
            Example {
                id: "avatar",
                title: "Avatar e footer",
                description: "Scheda persona con azioni nel piè.",
                code: WITH_AVATAR_CODE,
                render: || rsx! { WithAvatar {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Metriche, grafico e tabella del cruscotto operativo.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Card", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Container: sd-surface(1) (language.md §3.1)",
            depth: "Flat-lit (Elevated: Raised)",
            tokens: &["--sd-surface-1", "--sd-border", "--sd-specular", "--sd-elev-1", "--sd-radius-lg", "--sd-tint-1", "--sd-border-hover"],
            notes: "Statica a riposo; con onclick o href diventa interactive container: bordo hover ed elev-2, mai gradiente.",
        }),
        a11y: &[
            "Il titolo è un h3 e, se la card è cliccabile, ne diventa il nome accessibile (aria-labelledby).",
            "Con onclick la card ha role=button, tabindex 0 e si attiva con Invio e Spazio.",
            "Con href il titolo è un vero link <a> esteso a tutta la superficie.",
            "I controlli interni di una card cliccabile devono fermare la propagazione del click.",
        ],
        related: &["grid", "badges", "avatars", "progress", "media-card"],
    }
}
