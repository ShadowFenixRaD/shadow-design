//! Story: Coachmark and TeachingBubble (feature `coachmark`). The bubble renders in the flow of
//! the page next to the element it explains; the tour state lives in `CoachmarkDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const STEPS: [(&str, &str, &str); 3] = [
    ("Aggiungi un nodo", "Passo 1 · Provisioning", "Nuovo nodo avvia il provisioning in una regione libera del cluster."),
    ("Filtra la vista", "Passo 2 · Filtri", "Mostra solo i nodi degradati o offline quando devi intervenire."),
    ("Esporta il report", "Passo 3 · Report", "Il CSV contiene latenza, peer e stato di ogni nodo nella finestra scelta."),
];
const TARGETS: [&str; 3] = ["Nuovo nodo", "Filtri", "Esporta"];

const PRELUDE: &str = r#"const STEPS: [(&str, &str, &str); 3] = [
    ("Aggiungi un nodo", "Passo 1 · Provisioning", "Nuovo nodo avvia il provisioning in una regione libera del cluster."),
    ("Filtra la vista", "Passo 2 · Filtri", "Mostra solo i nodi degradati o offline quando devi intervenire."),
    ("Esporta il report", "Passo 3 · Report", "Il CSV contiene latenza, peer e stato di ogni nodo nella finestra scelta."),
];
let mut step = use_signal(|| 1usize);
let mut active = use_signal(|| true);
let (title, headline, body) = STEPS[step() - 1];"#;

const FIXED: &[&str] = &[
    "step: step(),",
    "total_steps: 3,",
    "title: title.to_string(),",
    "on_next: move |_| if step() < 3 { step += 1 } else { active.set(false) },",
    "on_prev: move |_| step -= 1,",
];

#[component]
fn CoachmarkDemo(knobs: Knobs) -> Element {
    let mut step = use_signal(|| 1usize);
    let mut active = use_signal(|| true);
    let (title, headline, body) = STEPS[step() - 1];
    let current = if active() { TARGETS[step() - 1] } else { "" };
    rsx! {
        Stack { gap: SpacingToken::Lg,
            Group { gap: SpacingToken::Sm,
                for target in TARGETS {
                    Button {
                        key: "{target}",
                        variant: if target == "Nuovo nodo" { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                        size: ButtonSize::Sm,
                        glow: (target == current).then_some(true),
                        "{target}"
                    }
                }
            }
            if active() {
                Coachmark {
                    step: step(),
                    total_steps: 3,
                    title: title.to_string(),
                    on_next: move |_| if step() < 3 { step += 1 } else { active.set(false) },
                    on_prev: move |_| step -= 1,
                    headline: knobs.bool("headline").then(|| headline.to_string()),
                    on_dismiss: knobs.bool("dismissable").then_some(EventHandler::new(move |_| active.set(false))),
                    glow: knobs.bool("glow").then_some(true),
                    Text { size: TextSize::Sm, dimmed: true, "{body}" }
                }
            } else {
                Group {
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Sparkles, size: 14 }
                        }),
                        onclick: move |_| {
                            step.set(1);
                            active.set(true);
                        },
                        "Riavvia il tour"
                    }
                }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        CoachmarkDemo { knobs: k.clone() }
    }
}

#[component]
fn Bubble() -> Element {
    let mut open = use_signal(|| true);
    rsx! {
        if open() {
            TeachingBubble {
                step: 1,
                total_steps: 1,
                title: "Scorciatoia disponibile".to_string(),
                on_next: move |_| open.set(false),
                on_dismiss: move |_| open.set(false),
                Group { gap: SpacingToken::Xs,
                    Text { size: TextSize::Sm, dimmed: true, "Premi" }
                    Kbd { "Ctrl" }
                    Kbd { "K" }
                    Text { size: TextSize::Sm, dimmed: true, "per cercare un nodo." }
                }
            }
        } else {
            Group {
                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| open.set(true), "Mostra di nuovo" }
            }
        }
    }
}
const BUBBLE_CODE: &str = r#"let mut open = use_signal(|| true);

if open() {
    TeachingBubble {
        step: 1,
        total_steps: 1,
        title: "Scorciatoia disponibile".to_string(),
        on_next: move |_| open.set(false),
        on_dismiss: move |_| open.set(false),
        Group { gap: SpacingToken::Xs,
            Text { size: TextSize::Sm, dimmed: true, "Premi" }
            Kbd { "Ctrl" }
            Kbd { "K" }
            Text { size: TextSize::Sm, dimmed: true, "per cercare un nodo." }
        }
    }
} else {
    Group {
        Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| open.set(true), "Mostra di nuovo" }
    }
}"#;

#[component]
fn LastStep() -> Element {
    let mut toast = use_notification();
    rsx! {
        Coachmark {
            step: 3,
            total_steps: 3,
            title: "Tutto pronto".to_string(),
            headline: Some("Passo 3 · Fine".to_string()),
            on_prev: move |_| {
                toast.info("Torna al passo 2");
            },
            on_next: move |_| {
                toast.success("Tour completato");
            },
            Text { size: TextSize::Sm, dimmed: true, "All'ultimo passo il pulsante principale chiude il tour." }
        }
    }
}
const LAST_STEP_CODE: &str = r#"let mut toast = use_notification();

Coachmark {
    step: 3,
    total_steps: 3,
    title: "Tutto pronto".to_string(),
    headline: Some("Passo 3 · Fine".to_string()),
    on_prev: move |_| {
        toast.info("Torna al passo 2");
    },
    on_next: move |_| {
        toast.success("Tour completato");
    },
    Text { size: TextSize::Sm, dimmed: true, "All'ultimo passo il pulsante principale chiude il tour." }
}"#;

#[component]
fn InContext() -> Element {
    let mut tip = use_signal(|| true);
    let mut toast = use_notification();
    rsx! {
        SimpleGrid { cols: 1, md: Some(2), spacing: GridSpacing::Md,
            Card {
                title: Some("Report settimanale".to_string()),
                subtitle: Some("Cluster eu-west · 24 nodi".to_string()),
                buttons: Some(rsx! {
                    Badge { label: "Nuovo".to_string(), variant: BadgeVariant::Primary, pill: true }
                }),
                footer: Some(rsx! {
                    Group { justify: JustifyContent::End,
                        Button {
                            glow: tip().then_some(true),
                            left_icon: Some(rsx! {
                                Icon { kind: IconKind::Download, size: 15 }
                            }),
                            onclick: move |_| {
                                tip.set(false);
                                toast.success("Report esportato");
                            },
                            "Esporta CSV"
                        }
                    }
                }),
                Text { size: TextSize::Sm, dimmed: true, "Latenza media 1.2 ms, un incidente aperto, 4.8M richieste." }
            }
            if tip() {
                TeachingBubble {
                    step: 1,
                    total_steps: 1,
                    title: "Esporta in CSV".to_string(),
                    headline: Some("Novità della 0.5".to_string()),
                    on_next: move |_| tip.set(false),
                    on_dismiss: move |_| tip.set(false),
                    Text { size: TextSize::Sm, dimmed: true, "Il report ora si scarica in CSV con una riga per nodo." }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut tip = use_signal(|| true);
let mut toast = use_notification();

SimpleGrid { cols: 1, md: Some(2), spacing: GridSpacing::Md,
    Card {
        title: Some("Report settimanale".to_string()),
        subtitle: Some("Cluster eu-west · 24 nodi".to_string()),
        buttons: Some(rsx! {
            Badge { label: "Nuovo".to_string(), variant: BadgeVariant::Primary, pill: true }
        }),
        footer: Some(rsx! {
            Group { justify: JustifyContent::End,
                Button {
                    glow: tip().then_some(true),
                    left_icon: Some(rsx! {
                        Icon { kind: IconKind::Download, size: 15 }
                    }),
                    onclick: move |_| {
                        tip.set(false);
                        toast.success("Report esportato");
                    },
                    "Esporta CSV"
                }
            }
        }),
        Text { size: TextSize::Sm, dimmed: true, "Latenza media 1.2 ms, un incidente aperto, 4.8M richieste." }
    }
    if tip() {
        TeachingBubble {
            step: 1,
            total_steps: 1,
            title: "Esporta in CSV".to_string(),
            headline: Some("Novità della 0.5".to_string()),
            on_next: move |_| tip.set(false),
            on_dismiss: move |_| tip.set(false),
            Text { size: TextSize::Sm, dimmed: true, "Il report ora si scarica in CSV con una riga per nodo." }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("step / total_steps", "usize", "-", "Passo corrente (da 1) e numero di passi: badge e puntini."),
    prop("title", "String", "-", "Titolo del passo."),
    prop("headline", "Option<String>", "None", "Sottotitolo sotto il titolo."),
    prop("on_next", "Option<EventHandler<()>>", "None", "Pulsante Avanti; all'ultimo passo diventa Completa."),
    prop("on_prev", "Option<EventHandler<()>>", "None", "Pulsante Indietro, nascosto al primo passo."),
    prop("on_dismiss", "Option<EventHandler<()>>", "None", "Mostra la X di chiusura."),
    prop("glow", "Option<bool>", "None", "Glow opzionale sulla bolla."),
    prop("target_id / position", "Option<String> / CoachmarkPosition", "None / Bottom", "Riservate: oggi la bolla resta nel flusso, va collocata accanto al bersaglio."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
    prop("children", "Element", "-", "Spiegazione del passo."),
];
const BUBBLE_PROPS: &[PropDoc] = &[prop("…", "-", "-", "Alias di Coachmark con le stesse prop, per suggerimenti di un solo passo.")];

pub fn story() -> Story {
    Story {
        slug: "coachmark",
        name: "Coachmark",
        category: Category::Overlay,
        features: &["coachmark"],
        icon: IconKind::Sparkles,
        tagline: "Bolla di guida che accompagna l'utente passo dopo passo accanto alle funzioni da scoprire.",
        keywords: &["coachmark", "tour", "onboarding", "teaching bubble", "guida", "suggerimento"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Coachmark",
            controls: vec![
                Control::bool("headline", "Sottotitolo", true).emit(Emit::Template("headline: Some(headline.to_string())")),
                Control::bool("dismissable", "Chiudibile", true).emit(Emit::Template("on_dismiss: move |_| active.set(false)")),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: FIXED,
            children_code: Some("Text { size: TextSize::Sm, dimmed: true, \"{body}\" }"),
            stage: Stage::Fill,
        }),
        examples: vec![
            Example {
                id: "bubble",
                title: "TeachingBubble",
                description: "Un solo passo: il pulsante principale chiude la bolla.",
                code: BUBBLE_CODE,
                render: || rsx! { Bubble {} },
                span: Span::Half,
            },
            Example {
                id: "last-step",
                title: "Ultimo passo",
                description: "Indietro, puntini di avanzamento e Completa.",
                code: LAST_STEP_CODE,
                render: || rsx! { LastStep {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Annuncio di una funzione nuova accanto alla card che la contiene.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Coachmark", rows: PROPS },
            PropGroup { component: "TeachingBubble", rows: BUBBLE_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Floating panel (language.md §3.2)",
            depth: "Raised",
            tokens: &["--sd-surface-2", "--sd-border-strong", "--sd-elev-2", "--sd-radius-lg", "--sd-accent", "--sd-space-4"],
            notes: "Il puntino attivo è un value fill (§3.11); l'unica azione Primary è Avanti / Completa.",
        }),
        a11y: &[
            "Titolo (h4), sottotitolo e testo sono contenuto normale, letti nell'ordine del DOM accanto all'elemento spiegato.",
            "Avanti, Indietro e chiudi sono pulsanti nativi; i testi vengono da Locale (forward, back, finish, coachmark_close).",
            "Il passo corrente è scritto per esteso nel badge (Passo 1 di 3 con il Locale italiano), non solo nei puntini.",
            "La bolla non intrappola il focus: l'utente può ignorarla e continuare a usare la pagina.",
        ],
        related: &["popover", "tooltip", "stepper", "keytip"],
    }
}
