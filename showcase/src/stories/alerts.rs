//! Story: Alert (feature `alert`). Closing it needs a signal, so the playground goes through
//! `AlertDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::scenes::AuthCard;
use crate::story::schema::*;

const VARIANTS: [AlertVariant; 4] = [AlertVariant::Info, AlertVariant::Success, AlertVariant::Warning, AlertVariant::Danger];
const VARIANT_OPTS: &[ChoiceOption] = &[
    opt("Info", "AlertVariant::Info"),
    opt("Success", "AlertVariant::Success"),
    opt("Warning", "AlertVariant::Warning"),
    opt("Danger", "AlertVariant::Danger"),
];

const PRELUDE: &str = "let mut visible = use_signal(|| true);";

#[component]
fn AlertDemo(knobs: Knobs) -> Element {
    let mut visible = use_signal(|| true);
    let title = knobs.text("title");
    let severity = knobs.text("severity_label");
    let closable = knobs.bool("closable");
    rsx! {
        if visible() {
            Alert {
                variant: knobs.pick("variant", &VARIANTS),
                title: (!title.is_empty()).then_some(title),
                on_close: closable.then_some(EventHandler::new(move |_| visible.set(false))),
                live: knobs.bool("live"),
                severity_label: (!severity.is_empty()).then_some(severity),
                glow: knobs.bool("glow").then_some(true),
                "{knobs.text(\"body\")}"
            }
        } else {
            Group { justify: JustifyContent::SpaceBetween,
                Text { size: TextSize::Sm, dimmed: true, "L'avviso è stato chiuso." }
                Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, onclick: move |_| visible.set(true), "Mostra di nuovo" }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        AlertDemo { knobs: k.clone() }
    }
}

#[component]
fn Variants() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Sm,
            Alert { variant: AlertVariant::Info, title: Some("Finestra di manutenzione".to_string()),
                "Sabato dalle 02:00 alle 04:00 i nodi core si riavviano a rotazione."
            }
            Alert { variant: AlertVariant::Success, title: Some("Build completata".to_string()),
                "Workspace compilato senza errori in 54 secondi."
            }
            Alert { variant: AlertVariant::Warning, title: Some("Certificato in scadenza".to_string()),
                "Il certificato di relay-03 scade tra 6 giorni."
            }
            Alert { variant: AlertVariant::Danger, title: Some("Nodo irraggiungibile".to_string()),
                "edge-21 non risponde da 4 minuti."
            }
        }
    }
}
const VARIANTS_CODE: &str = r#"Stack { gap: SpacingToken::Sm,
    Alert { variant: AlertVariant::Info, title: Some("Finestra di manutenzione".to_string()),
        "Sabato dalle 02:00 alle 04:00 i nodi core si riavviano a rotazione."
    }
    Alert { variant: AlertVariant::Success, title: Some("Build completata".to_string()),
        "Workspace compilato senza errori in 54 secondi."
    }
    Alert { variant: AlertVariant::Warning, title: Some("Certificato in scadenza".to_string()),
        "Il certificato di relay-03 scade tra 6 giorni."
    }
    Alert { variant: AlertVariant::Danger, title: Some("Nodo irraggiungibile".to_string()),
        "edge-21 non risponde da 4 minuti."
    }
}"#;

#[component]
fn WithActions() -> Element {
    let mut toast = use_notification();
    rsx! {
        Alert { variant: AlertVariant::Warning, title: Some("Certificato in scadenza".to_string()), live: false,
            Stack { gap: SpacingToken::Sm,
                Text { size: TextSize::Sm, "Il certificato di relay-03 scade tra 6 giorni. Il rinnovo automatico è disattivato." }
                Group { gap: SpacingToken::Sm,
                    Button {
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            toast.success("Rinnovo avviato");
                        },
                        "Rinnova ora"
                    }
                    Button { size: ButtonSize::Sm, variant: ButtonVariant::Ghost, "Ricordamelo domani" }
                }
            }
        }
    }
}
const WITH_ACTIONS_CODE: &str = r#"let mut toast = use_notification();

Alert { variant: AlertVariant::Warning, title: Some("Certificato in scadenza".to_string()), live: false,
    Stack { gap: SpacingToken::Sm,
        Text { size: TextSize::Sm, "Il certificato di relay-03 scade tra 6 giorni. Il rinnovo automatico è disattivato." }
        Group { gap: SpacingToken::Sm,
            Button {
                size: ButtonSize::Sm,
                variant: ButtonVariant::Secondary,
                onclick: move |_| {
                    toast.success("Rinnovo avviato");
                },
                "Rinnova ora"
            }
            Button { size: ButtonSize::Sm, variant: ButtonVariant::Ghost, "Ricordamelo domani" }
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        AuthCard {}
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/auth_card.rs (estratto): l'avviso compare solo dopo un codice sbagliato. Prova 123456.
if wrong() {
    Alert { variant: AlertVariant::Danger, title: Some("Codice non valido".to_string()),
        "Il codice è scaduto o errato. Ne arriva uno nuovo ogni 30 secondi."
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("variant", "AlertVariant", "Info", "Info, Success, Warning, Danger: icona, tinta e ruolo ARIA."),
    prop("title", "Option<String>", "None", "Titolo, collegato con aria-labelledby."),
    prop("on_close", "Option<EventHandler<MouseEvent>>", "None", "Mostra il pulsante X; chi usa l'alert decide come nasconderlo."),
    prop("live", "bool", "true", "Annuncio automatico: role=alert per Warning e Danger, role=status per Info e Success."),
    prop("severity_label", "Option<String>", "None", "Prefisso letto dagli screen reader prima del titolo (es. \"Errore\")."),
    prop("heading_level", "Option<u8>", "None", "Livello del titolo (2-6) per l'ordine dei titoli della pagina."),
    prop("close_label", "Option<String>", "None", "Etichetta del pulsante X (da Locale se assente)."),
    prop("glow", "Option<bool>", "None", "Glow opzionale."),
    prop("id / class", "Option<String> / String", "None / \"\"", "Id e classi aggiuntive."),
    prop("children", "Element", "-", "Corpo dell'avviso: testo o azioni."),
];

pub fn story() -> Story {
    Story {
        slug: "alerts",
        name: "Alert",
        category: Category::Dati,
        features: &["alert"],
        icon: IconKind::AlertCircle,
        tagline: "Avviso in linea che comunica un esito o un rischio con tinta semantica, icona e titolo.",
        keywords: &["alert", "avviso", "callout", "messaggio", "errore", "warning", "banner"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Alert",
            controls: vec![
                Control::choice("variant", "Variante", VARIANT_OPTS, 2).emit(Emit::Always),
                Control::text("title", "Titolo", "Certificato in scadenza").emit(Emit::Template("title: Some(\"{v}\".to_string())")),
                Control::text("body", "Testo", "Il certificato di relay-03 scade tra 6 giorni.").emit(Emit::Skip),
                Control::text("severity_label", "Prefisso SR", "Attenzione").emit(Emit::Template("severity_label: Some(\"{v}\".to_string())")),
                Control::bool("closable", "Chiudibile", true).emit(Emit::Template("on_close: move |_| visible.set(false)")),
                Control::bool("live", "Annuncio live", true),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &[],
            children_code: Some("\"{body}\""),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "variants",
                title: "Varianti",
                description: "Quattro esiti, ognuno con la sua icona.",
                code: VARIANTS_CODE,
                render: || rsx! { Variants {} },
                span: Span::Half,
            },
            Example {
                id: "with-actions",
                title: "Con azioni",
                description: "Il corpo accetta pulsanti; live: false per un avviso statico.",
                code: WITH_ACTIONS_CODE,
                render: || rsx! { WithActions {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Accesso con secondo fattore: l'errore appare sotto il codice.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Alert", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Semantic surface (language.md §3.10)",
            depth: "Flat-lit",
            tokens: &["--sd-warning-soft", "--sd-warning-border", "--sd-warning-strong", "--sd-warning", "--sd-text-muted", "--sd-radius-lg"],
            notes: "Rail di 3px nel colore semantico pieno sul bordo iniziale; titolo e icona strong, corpo muted.",
        }),
        a11y: &[
            "Con live attivo usa role=alert e aria-live=assertive per Warning e Danger, role=status e polite per Info e Success.",
            "Un prefisso nascosto (severity_label) dice la gravità prima del titolo; l'icona è aria-hidden.",
            "Il titolo è un heading di livello configurabile e nomina la regione con aria-labelledby.",
            "Il pulsante X è un CloseButton con etichetta da Locale.",
        ],
        related: &["notifications", "alert-dialog", "badges", "loading-overlay"],
    }
}
