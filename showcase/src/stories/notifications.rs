//! Story: Notification (feature `notification`). The toast queue lives in the app root
//! (`NotificationProvider`); the playground goes through `NotifyDemo { knobs }` because
//! `use_notification()` is a hook.
use std::time::Duration;

use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::scenes::SettingsForm;
use crate::story::schema::*;

const VARIANTS: [NotificationVariant; 4] = [
    NotificationVariant::Info,
    NotificationVariant::Success,
    NotificationVariant::Warning,
    NotificationVariant::Danger,
];
const VARIANT_OPTS: &[ChoiceOption] = &[
    opt("Info", "NotificationVariant::Info"),
    opt("Success", "NotificationVariant::Success"),
    opt("Warning", "NotificationVariant::Warning"),
    opt("Danger", "NotificationVariant::Danger"),
];
const DURATIONS: [Option<u64>; 3] = [Some(6), Some(12), None];
const DURATION_OPTS: &[ChoiceOption] = &[
    opt("6 s", "Some(Duration::from_secs(6))"),
    opt("12 s", "Some(Duration::from_secs(12))"),
    opt("Persistente", "None"),
];

const PRELUDE: &str = "let mut toast = use_notification();";
const CHILDREN: &str = r#"onclick: move |_| {
    toast.show_with_duration(
        "{title}",
        "{message}",
        {variant},
        {duration},
    );
},
"Mostra notifica""#;

/// Stateful demo: the hook needs a component scope.
#[component]
fn NotifyDemo(knobs: Knobs) -> Element {
    let mut toast = use_notification();
    let title = knobs.text("title");
    let message = knobs.text("message");
    let variant = knobs.pick("variant", &VARIANTS);
    let duration = knobs.pick("duration", &DURATIONS).map(Duration::from_secs);
    rsx! {
        Button {
            onclick: move |_| {
                toast.show_with_duration(title.clone(), message.clone(), variant, duration);
            },
            "Mostra notifica"
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        NotifyDemo { knobs: k.clone() }
    }
}

#[component]
fn Shortcuts() -> Element {
    let mut toast = use_notification();
    rsx! {
        Group { gap: SpacingToken::Sm,
            Button {
                variant: ButtonVariant::Secondary,
                onclick: move |_| {
                    toast.info("Nuova versione del design system disponibile.");
                },
                "Info"
            }
            Button {
                variant: ButtonVariant::Success,
                onclick: move |_| {
                    toast.success("Workspace compilato senza errori.");
                },
                "Success"
            }
            Button {
                variant: ButtonVariant::Secondary,
                onclick: move |_| {
                    toast.warning("Latenza oltre 200 ms verso relay-03.");
                },
                "Warning"
            }
            Button {
                variant: ButtonVariant::Danger,
                onclick: move |_| {
                    toast.danger("Impossibile raggiungere edge-21.");
                },
                "Danger"
            }
        }
    }
}
const SHORTCUTS_CODE: &str = r#"let mut toast = use_notification();

Group { gap: SpacingToken::Sm,
    Button {
        variant: ButtonVariant::Secondary,
        onclick: move |_| {
            toast.info("Nuova versione del design system disponibile.");
        },
        "Info"
    }
    Button {
        variant: ButtonVariant::Success,
        onclick: move |_| {
            toast.success("Workspace compilato senza errori.");
        },
        "Success"
    }
    Button {
        variant: ButtonVariant::Secondary,
        onclick: move |_| {
            toast.warning("Latenza oltre 200 ms verso relay-03.");
        },
        "Warning"
    }
    Button {
        variant: ButtonVariant::Danger,
        onclick: move |_| {
            toast.danger("Impossibile raggiungere edge-21.");
        },
        "Danger"
    }
}"#;

#[component]
fn Queue() -> Element {
    let mut toast = use_notification();
    let mut pending = use_signal(|| None::<usize>);
    rsx! {
        Group { gap: SpacingToken::Sm,
            Button {
                disabled: pending().is_some(),
                onclick: move |_| {
                    let id = toast.show_with_duration(
                        "Migrazione in corso",
                        "Copia dei volumi da edge-07 a edge-22.",
                        NotificationVariant::Info,
                        None,
                    );
                    pending.set(Some(id));
                },
                "Avvia migrazione"
            }
            Button {
                variant: ButtonVariant::Secondary,
                disabled: pending().is_none(),
                onclick: move |_| {
                    if let Some(id) = pending.take() {
                        toast.dismiss(id);
                        toast.success("Migrazione completata.");
                    }
                },
                "Completa"
            }
            Button {
                variant: ButtonVariant::Ghost,
                onclick: move |_| {
                    toast.clear();
                    pending.set(None);
                },
                "Svuota tutto"
            }
        }
    }
}
const QUEUE_CODE: &str = r#"let mut toast = use_notification();
let mut pending = use_signal(|| None::<usize>);

Group { gap: SpacingToken::Sm,
    Button {
        disabled: pending().is_some(),
        onclick: move |_| {
            let id = toast.show_with_duration(
                "Migrazione in corso",
                "Copia dei volumi da edge-07 a edge-22.",
                NotificationVariant::Info,
                None,
            );
            pending.set(Some(id));
        },
        "Avvia migrazione"
    }
    Button {
        variant: ButtonVariant::Secondary,
        disabled: pending().is_none(),
        onclick: move |_| {
            if let Some(id) = pending.take() {
                toast.dismiss(id);
                toast.success("Migrazione completata.");
            }
        },
        "Completa"
    }
    Button {
        variant: ButtonVariant::Ghost,
        onclick: move |_| {
            toast.clear();
            pending.set(None);
        },
        "Svuota tutto"
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        SettingsForm { compact: true }
    }
}
const IN_CONTEXT_CODE: &str = r#"// main.rs: un solo provider alla radice dell'app.
NotificationProvider { aria_label: Some("Notifiche".to_string()),
    Router::<Route> {}
}

// scenes/settings_form.rs (estratto): il salvataggio conferma con un toast.
let mut toast = use_notification();
Button {
    onclick: move |_| {
        toast.success("Impostazioni salvate");
    },
    "Salva impostazioni"
}"#;

const PROVIDER_PROPS: &[PropDoc] = &[
    prop("aria_label", "Option<String>", "None", "Nome della regione dei toast (\"Notifications\" se assente)."),
    prop("children", "Element", "-", "L'albero dell'app: monta il contenitore dei toast dopo i figli."),
];
const HANDLE_PROPS: &[PropDoc] = &[
    prop("use_notification()", "fn() -> NotificationHandle", "-", "Handle Copy; va in panic senza un NotificationProvider antenato."),
    prop("try_use_notification()", "fn() -> Option<NotificationHandle>", "-", "Variante che non va in panic."),
    prop("info / success / warning", "fn(&mut self, impl Into<String>) -> usize", "-", "Toast con titolo predefinito, chiuso dopo 6 secondi."),
    prop("danger", "fn(&mut self, impl Into<String>) -> usize", "-", "Toast di errore persistente: si chiude solo a mano."),
    prop("show / show_with_title", "fn(&mut self, title, message, NotificationVariant) -> usize", "-", "Titolo e messaggio; 6 secondi, persistente se Danger."),
    prop("show_with_duration", "fn(&mut self, title, message, NotificationVariant, Option<Duration>) -> usize", "-", "Durata esplicita; None lo rende persistente."),
    prop("dismiss", "fn(&mut self, usize)", "-", "Chiude il toast con l'id restituito da show_*."),
    prop("clear", "fn(&mut self)", "-", "Svuota la coda."),
];
const VARIANT_PROPS: &[PropDoc] = &[
    prop("Info", "NotificationVariant", "default", "Icona Info, regione polite."),
    prop("Success", "NotificationVariant", "-", "Icona Check, regione polite."),
    prop("Warning", "NotificationVariant", "-", "Icona AlertTriangle, regione polite."),
    prop("Danger", "NotificationVariant", "-", "Icona AlertCircle, regione assertive."),
];

pub fn story() -> Story {
    Story {
        slug: "notifications",
        name: "Notification",
        category: Category::Dati,
        features: &["notification"],
        icon: IconKind::Info,
        tagline: "Toast globali per confermare un'azione o segnalare un errore, da qualsiasi componente.",
        keywords: &["notification", "notifica", "toast", "snackbar", "messaggio", "feedback"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Button",
            controls: vec![
                Control::choice("variant", "Variante", VARIANT_OPTS, 1).emit(Emit::Skip),
                Control::text("title", "Titolo", "Snapshot creato").emit(Emit::Skip),
                Control::text("message", "Messaggio", "Il backup del cluster è salvato su tre regioni.").emit(Emit::Skip),
                Control::choice("duration", "Durata", DURATION_OPTS, 0)
                    .emit(Emit::Skip)
                    .hint("Persistente: resta finché non viene chiuso."),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &[],
            children_code: Some(CHILDREN),
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "shortcuts",
                title: "Scorciatoie",
                description: "info, success, warning e danger; solo danger resta finché non lo chiudi.",
                code: SHORTCUTS_CODE,
                render: || rsx! { Shortcuts {} },
                span: Span::Half,
            },
            Example {
                id: "queue",
                title: "Coda e chiusura",
                description: "show_* restituisce un id: chiudi un toast persistente quando il lavoro finisce.",
                code: QUEUE_CODE,
                render: || rsx! { Queue {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il form Impostazioni conferma il salvataggio con un toast.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "NotificationProvider", rows: PROVIDER_PROPS },
            PropGroup { component: "NotificationHandle", rows: HANDLE_PROPS },
            PropGroup { component: "NotificationVariant", rows: VARIANT_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Semantic surface (language.md §3.10) su floating panel (§3.2)",
            depth: "Raised",
            tokens: &["--sd-surface-2", "--sd-border-strong", "--sd-elev-2", "--sd-success-soft", "--sd-success-border", "--sd-success-strong", "--sd-radius-lg"],
            notes: "Pannello flottante con rail semantico di 3px; entra con sd-pop-in e si ferma al passaggio del puntatore o al focus.",
        }),
        a11y: &[
            "Il contenitore è una regione con nome; Info, Success e Warning vanno in una pila role=status con aria-live=polite.",
            "Danger va in una pila separata role=alert con aria-live=assertive ed è persistente.",
            "Ogni toast inizia con un prefisso nascosto (Info, Success, Warning, Error); l'icona è aria-hidden.",
            "Il timer si ferma con il puntatore sopra o con il focus dentro il toast.",
            "Il pulsante di chiusura è un <button> con aria-label da Locale.",
        ],
        related: &["alerts", "alert-dialog", "progress", "loading-overlay"],
    }
}
