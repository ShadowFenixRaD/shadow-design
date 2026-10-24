//! Story: Carousel and CarouselSlide (feature `carousel`). The active slide lives in a signal,
//! so the playground goes through `CarouselDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const PRELUDE: &str = "let slide = use_signal(|| 0usize);";

/// Children of the playground block, printed verbatim (the live render adds no sc-* class).
const SLIDES_CODE: &str = r#"CarouselSlide {
    Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
        Badge { label: "Novità 1 di 3".to_string(), variant: BadgeVariant::Primary, pill: true }
        Heading { order: HeadingOrder::H4, "Un hex, tutto il tema" }
        Text { dimmed: true, "Scegli un colore primario: bottoni, focus, grafici e glow lo seguono." }
    }
}
CarouselSlide {
    Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
        Badge { label: "Novità 2 di 3".to_string(), variant: BadgeVariant::Primary, pill: true }
        Heading { order: HeadingOrder::H4, "Una feature per componente" }
        Text { dimmed: true, "Compili solo ciò che usi: ogni componente è dietro un flag Cargo." }
    }
}
CarouselSlide {
    Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
        Badge { label: "Novità 3 di 3".to_string(), variant: BadgeVariant::Primary, pill: true }
        Heading { order: HeadingOrder::H4, "Nessun tool a valle" }
        Text { dimmed: true, "Niente Node né npm: lo SCSS passa dalla pipeline asset di Dioxus." }
    }
}"#;

#[component]
fn CarouselDemo(knobs: Knobs) -> Element {
    let slide = use_signal(|| 0usize);
    rsx! {
        Carousel {
            slide_count: 3,
            active_index: Some(slide),
            r#loop: knobs.bool("r#loop"),
            with_controls: knobs.bool("with_controls"),
            with_indicators: knobs.bool("with_indicators"),
            padded: knobs.bool("padded"),
            CarouselSlide {
                Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
                    Badge { label: "Novità 1 di 3".to_string(), variant: BadgeVariant::Primary, pill: true }
                    Heading { order: HeadingOrder::H4, "Un hex, tutto il tema" }
                    Text { dimmed: true, "Scegli un colore primario: bottoni, focus, grafici e glow lo seguono." }
                }
            }
            CarouselSlide {
                Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
                    Badge { label: "Novità 2 di 3".to_string(), variant: BadgeVariant::Primary, pill: true }
                    Heading { order: HeadingOrder::H4, "Una feature per componente" }
                    Text { dimmed: true, "Compili solo ciò che usi: ogni componente è dietro un flag Cargo." }
                }
            }
            CarouselSlide {
                Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
                    Badge { label: "Novità 3 di 3".to_string(), variant: BadgeVariant::Primary, pill: true }
                    Heading { order: HeadingOrder::H4, "Nessun tool a valle" }
                    Text { dimmed: true, "Niente Node né npm: lo SCSS passa dalla pipeline asset di Dioxus." }
                }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        CarouselDemo { knobs: k.clone() }
    }
}

fn hours() -> Vec<ChartDataPoint> {
    [("00", 18.0, 120.0), ("04", 16.0, 80.0), ("08", 24.0, 310.0), ("12", 31.0, 420.0), ("16", 27.0, 380.0), ("20", 21.0, 240.0)]
        .iter()
        .map(|(h, lat, req)| ChartDataPoint::new(*h).with_value("latenza", *lat).with_value("richieste", *req))
        .collect()
}

#[component]
fn Charts() -> Element {
    rsx! {
        Carousel { slide_count: 2,
            CarouselSlide {
                LineChart {
                    data: hours(),
                    series: vec![ChartSeries::new("latenza").with_label("Latenza (ms)")],
                    with_legend: false,
                    height: 220,
                }
            }
            CarouselSlide {
                BarChart {
                    data: hours(),
                    series: vec![ChartSeries::new("richieste").with_label("Richieste/s")],
                    with_legend: false,
                    height: 220,
                }
            }
        }
    }
}
const CHARTS_CODE: &str = r#"Carousel { slide_count: 2,
    CarouselSlide {
        LineChart {
            data: hours(),
            series: vec![ChartSeries::new("latenza").with_label("Latenza (ms)")],
            with_legend: false,
            height: 220,
        }
    }
    CarouselSlide {
        BarChart {
            data: hours(),
            series: vec![ChartSeries::new("richieste").with_label("Richieste/s")],
            with_legend: false,
            height: 220,
        }
    }
}"#;

#[component]
fn NoLoop() -> Element {
    rsx! {
        Carousel { slide_count: 3, r#loop: false, padded: true,
            CarouselSlide {
                Stack { gap: SpacingToken::Xs,
                    Text { size: TextSize::Sm, dimmed: true, "v0.5.0 · 24 ottobre" }
                    Text { "Showcase basato sulle story: codice generato dai controlli." }
                }
            }
            CarouselSlide {
                Stack { gap: SpacingToken::Xs,
                    Text { size: TextSize::Sm, dimmed: true, "v0.4.2 · 2 ottobre" }
                    Text { "Tre ruoli tipografici: Kanit, Onest e Source Code Pro." }
                }
            }
            CarouselSlide {
                Stack { gap: SpacingToken::Xs,
                    Text { size: TextSize::Sm, dimmed: true, "v0.4.0 · 12 settembre" }
                    Text { "Lit Glass: un solo materiale per tutti i componenti." }
                }
            }
        }
    }
}
const NO_LOOP_CODE: &str = r#"Carousel { slide_count: 3, r#loop: false, padded: true,
    CarouselSlide {
        Stack { gap: SpacingToken::Xs,
            Text { size: TextSize::Sm, dimmed: true, "v0.5.0 · 24 ottobre" }
            Text { "Showcase basato sulle story: codice generato dai controlli." }
        }
    }
    CarouselSlide {
        Stack { gap: SpacingToken::Xs,
            Text { size: TextSize::Sm, dimmed: true, "v0.4.2 · 2 ottobre" }
            Text { "Tre ruoli tipografici: Kanit, Onest e Source Code Pro." }
        }
    }
    CarouselSlide {
        Stack { gap: SpacingToken::Xs,
            Text { size: TextSize::Sm, dimmed: true, "v0.4.0 · 12 settembre" }
            Text { "Lit Glass: un solo materiale per tutti i componenti." }
        }
    }
}"#;

/// Onboarding steps: icon, title, body.
const STEPS: [(IconKind, &str, &str); 3] = [
    (IconKind::Server, "Collega il primo nodo", "Installa l'agente e incolla il token: il nodo compare nella dashboard in pochi secondi."),
    (IconKind::ShieldCheck, "Proteggi le chiavi", "Attiva la rotazione automatica: le chiavi Kyber-1024 cambiano ogni 24 ore."),
    (IconKind::Send, "Invita il team", "Assegna i ruoli: gli operatori vedono i log, solo gli admin gestiscono gli utenti."),
];

#[component]
fn InContext() -> Element {
    let mut step = use_signal(|| 0usize);
    let mut toast = use_notification();
    let last = STEPS.len() - 1;
    rsx! {
        Stack { gap: SpacingToken::Md,
            Carousel { slide_count: STEPS.len(), active_index: Some(step), r#loop: false, with_controls: false, padded: true,
                for (icon, title, body) in STEPS {
                    CarouselSlide {
                        Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
                            Icon { kind: icon, size: 28 }
                            Heading { order: HeadingOrder::H4, "{title}" }
                            Text { dimmed: true, "{body}" }
                        }
                    }
                }
            }
            Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween,
                span { class: "sc-hint", "Passo {step() + 1} di {STEPS.len()}" }
                Group { gap: SpacingToken::Sm,
                    Button {
                        variant: ButtonVariant::Ghost,
                        disabled: step() == 0,
                        onclick: move |_| step -= 1,
                        "Indietro"
                    }
                    if step() < last {
                        Button { onclick: move |_| step += 1, "Avanti" }
                    } else {
                        Button {
                            onclick: move |_| {
                                toast.success("Configurazione iniziale completata");
                            },
                            "Inizia"
                        }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut step = use_signal(|| 0usize);
let mut toast = use_notification();
let last = STEPS.len() - 1;

Stack { gap: SpacingToken::Md,
    Carousel { slide_count: STEPS.len(), active_index: Some(step), r#loop: false, with_controls: false, padded: true,
        for (icon, title, body) in STEPS {
            CarouselSlide {
                Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
                    Icon { kind: icon, size: 28 }
                    Heading { order: HeadingOrder::H4, "{title}" }
                    Text { dimmed: true, "{body}" }
                }
            }
        }
    }
    Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween,
        span { "Passo {step() + 1} di {STEPS.len()}" }
        Group { gap: SpacingToken::Sm,
            Button {
                variant: ButtonVariant::Ghost,
                disabled: step() == 0,
                onclick: move |_| step -= 1,
                "Indietro"
            }
            if step() < last {
                Button { onclick: move |_| step += 1, "Avanti" }
            } else {
                Button {
                    onclick: move |_| {
                        toast.success("Configurazione iniziale completata");
                    },
                    "Inizia"
                }
            }
        }
    }
}"#;

const CAROUSEL_PROPS: &[PropDoc] = &[
    prop("slide_count", "usize", "-", "Numero di CarouselSlide figlie: serve a frecce e indicatori."),
    prop("active_index", "Option<Signal<usize>>", "None", "Indice della slide visibile; passalo per pilotare il carosello da fuori."),
    prop("r#loop", "bool", "true", "Dall'ultima slide si torna alla prima e viceversa; false disabilita le frecce agli estremi."),
    prop("with_controls", "bool", "true", "Frecce laterali, visibili al passaggio del puntatore o con il focus dentro."),
    prop("with_indicators", "bool", "true", "Punti in basso, uno per slide; il punto attivo si allarga in accento."),
    prop("padded", "bool", "false", "Margine interno per testo e card; false per grafici e immagini a filo."),
    prop("children", "Element", "-", "Le CarouselSlide."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];
const SLIDE_PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "Contenuto della slide, a tutta larghezza."),
    prop("class", "String", "\"\"", "Classi aggiuntive sulla slide."),
];

pub fn story() -> Story {
    Story {
        slug: "carousel",
        name: "Carousel",
        category: Category::Dati,
        features: &["carousel"],
        icon: IconKind::ChevronRight,
        tagline: "Mostra una slide alla volta, con frecce, indicatori e tastiera, per tour, novità e grafici alternati.",
        keywords: &["carousel", "carosello", "slider", "slide", "galleria", "onboarding", "tour"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Carousel",
            controls: vec![
                Control::bool("padded", "Margine interno", true).emit(Emit::Always),
                Control::bool("r#loop", "Loop", true),
                Control::bool("with_controls", "Frecce", true),
                Control::bool("with_indicators", "Indicatori", true),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["slide_count: 3,", "active_index: Some(slide),"],
            children_code: Some(SLIDES_CODE),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "charts",
                title: "Grafici a filo",
                description: "padded: false lascia ai grafici tutta la larghezza.",
                code: CHARTS_CODE,
                render: || rsx! { Charts {} },
                span: Span::Half,
            },
            Example {
                id: "no-loop",
                title: "Senza loop",
                description: "Agli estremi la freccia si disabilita: utile per sequenze con un inizio e una fine.",
                code: NO_LOOP_CODE,
                render: || rsx! { NoLoop {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Un tour di benvenuto guidato da pulsanti esterni sullo stesso signal.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Carousel", rows: CAROUSEL_PROPS },
            PropGroup { component: "CarouselSlide", rows: SLIDE_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Container (language.md §3.1) + raised control (§3.5) + track (§3.4)",
            depth: "Flat-lit (contenitore) · Lit control (frecce)",
            tokens: &["--sd-surface-1", "--sd-border", "--sd-control-sm", "--sd-well", "--sd-accent", "--sd-glow-sm", "--sd-dur-slow"],
            notes: "Le frecce sono bottoni secondary tondi; gli indicatori stanno in una track e solo il punto attivo usa l'accento.",
        }),
        a11y: &[
            "Contenitore con role=region, aria-roledescription=carousel e aria-label da Locale; è focalizzabile.",
            "Con il focus sul carosello, Freccia sinistra e destra cambiano slide.",
            "Frecce e punti sono <button> con aria-label da Locale (\"Vai alla slide N\"); il punto attivo ha aria-selected.",
            "Frecce e indicatori compaiono anche con il focus da tastiera, non solo al passaggio del puntatore.",
        ],
        related: &["cards", "stepper", "tabs", "charts"],
    }
}
