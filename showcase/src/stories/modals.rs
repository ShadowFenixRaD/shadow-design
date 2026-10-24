//! Story: Modal (feature `modal`). Overlay story: the trigger lives inside a small card,
//! the playground goes through `ModalDemo { knobs }` because the dialog owns a signal.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::scenes::AuthCard;
use crate::story::schema::*;

const SIZES: [ModalSize; 5] = [ModalSize::Sm, ModalSize::Md, ModalSize::Lg, ModalSize::Xl, ModalSize::Xxl];
const SIZE_OPTS: &[ChoiceOption] = &[
    opt("Sm", "ModalSize::Sm"),
    opt("Md", "ModalSize::Md"),
    opt("Lg", "ModalSize::Lg"),
    opt("Xl", "ModalSize::Xl"),
    opt("Xxl", "ModalSize::Xxl"),
];

const PRELUDE: &str = r#"let mut is_open = use_signal(|| false);
let name = use_signal(|| "mesh-prod-2026".to_string());
let mut toast = use_notification();"#;

const BODY_CODE: &str = r#"Stack { gap: SpacingToken::Md,
    Text { dimmed: true, "La chiave attuale resta valida per 24 ore, poi ogni richiesta firmata con essa viene rifiutata." }
    TextInput { value: name, label: Some("Nome della nuova chiave".to_string()) }
    Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
        Button { variant: ButtonVariant::Ghost, onclick: move |_| is_open.set(false), "Annulla" }
        Button {
            onclick: move |_| {
                is_open.set(false);
                toast.success("Nuova chiave generata");
            },
            "Ruota chiave"
        }
    }
}"#;

/// Stateful demo: the card with the trigger plus the dialog it opens.
#[component]
fn ModalDemo(knobs: Knobs) -> Element {
    let mut is_open = use_signal(|| false);
    let name = use_signal(|| "mesh-prod-2026".to_string());
    let mut toast = use_notification();
    let title = knobs.text("title");
    rsx! {
        Card {
            title: Some("Chiavi API".to_string()),
            subtitle: Some("Progetto mesh-prod · 2 chiavi attive".to_string()),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, "Esporta elenco" }
                    Button {
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Key, size: 15 }
                        }),
                        onclick: move |_| is_open.set(true),
                        "Ruota chiave"
                    }
                }
            }),
            Stack { gap: SpacingToken::Sm,
                Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween,
                    Code { "sk_live_…a91f" }
                    Badge { label: "Attiva".to_string(), variant: BadgeVariant::Success, pill: true, dot: true }
                }
                Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween,
                    Code { "sk_test_…07c2" }
                    Badge { label: "Test".to_string(), pill: true }
                }
            }
        }
        Modal {
            is_open,
            title: (!title.is_empty()).then_some(title),
            size: Some(knobs.pick("size", &SIZES)),
            close_on_escape: knobs.bool("close_on_escape"),
            close_on_backdrop: knobs.bool("close_on_backdrop"),
            glow: knobs.bool("glow").then_some(true),
            Stack { gap: SpacingToken::Md,
                Text { dimmed: true, "La chiave attuale resta valida per 24 ore, poi ogni richiesta firmata con essa viene rifiutata." }
                TextInput { value: name, label: Some("Nome della nuova chiave".to_string()) }
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, onclick: move |_| is_open.set(false), "Annulla" }
                    Button {
                        onclick: move |_| {
                            is_open.set(false);
                            toast.success("Nuova chiave generata");
                        },
                        "Ruota chiave"
                    }
                }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        ModalDemo { knobs: k.clone() }
    }
}

#[component]
fn NoTitle() -> Element {
    let mut is_open = use_signal(|| false);
    rsx! {
        Group {
            Button { variant: ButtonVariant::Secondary, onclick: move |_| is_open.set(true), "Vedi riepilogo" }
        }
        Modal {
            is_open,
            aria_label: Some("Deploy completato".to_string()),
            size: Some(ModalSize::Sm),
            Stack { gap: SpacingToken::Md, align: AlignItems::Center,
                Icon { kind: IconKind::CheckCircle, size: 32 }
                Heading { order: HeadingOrder::H3, "Deploy completato" }
                Text { dimmed: true, "edge-22 è in linea su tre regioni. Il canary resta attivo per un'ora." }
                Button { full_width: true, onclick: move |_| is_open.set(false), "Chiudi" }
            }
        }
    }
}
const NO_TITLE_CODE: &str = r#"let mut is_open = use_signal(|| false);

Group {
    Button { variant: ButtonVariant::Secondary, onclick: move |_| is_open.set(true), "Vedi riepilogo" }
}
Modal {
    is_open,
    aria_label: Some("Deploy completato".to_string()),
    size: Some(ModalSize::Sm),
    Stack { gap: SpacingToken::Md, align: AlignItems::Center,
        Icon { kind: IconKind::CheckCircle, size: 32 }
        Heading { order: HeadingOrder::H3, "Deploy completato" }
        Text { dimmed: true, "edge-22 è in linea su tre regioni. Il canary resta attivo per un'ora." }
        Button { full_width: true, onclick: move |_| is_open.set(false), "Chiudi" }
    }
}"#;

#[component]
fn Blocking() -> Element {
    let mut is_open = use_signal(|| false);
    rsx! {
        Group {
            Button { variant: ButtonVariant::Secondary, onclick: move |_| is_open.set(true), "Avvia migrazione" }
        }
        Modal {
            is_open,
            title: Some("Migrazione dello schema".to_string()),
            close_on_escape: false,
            close_on_backdrop: false,
            Stack { gap: SpacingToken::Md,
                Text { dimmed: true, "Tre tabelle su cinque sono già convertite. Esc e il clic fuori non chiudono la finestra." }
                ProgressBar { value: 60.0, show_label: true }
                Group { justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Secondary, onclick: move |_| is_open.set(false), "Continua in background" }
                }
            }
        }
    }
}
const BLOCKING_CODE: &str = r#"let mut is_open = use_signal(|| false);

Group {
    Button { variant: ButtonVariant::Secondary, onclick: move |_| is_open.set(true), "Avvia migrazione" }
}
Modal {
    is_open,
    title: Some("Migrazione dello schema".to_string()),
    close_on_escape: false,
    close_on_backdrop: false,
    Stack { gap: SpacingToken::Md,
        Text { dimmed: true, "Tre tabelle su cinque sono già convertite. Esc e il clic fuori non chiudono la finestra." }
        ProgressBar { value: 60.0, show_label: true }
        Group { justify: JustifyContent::End,
            Button { variant: ButtonVariant::Secondary, onclick: move |_| is_open.set(false), "Continua in background" }
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut is_open = use_signal(|| false);
    rsx! {
        Alert { variant: AlertVariant::Warning, title: Some("Sessione scaduta".to_string()),
            Group { gap: SpacingToken::Md, justify: JustifyContent::SpaceBetween,
                Text { size: TextSize::Sm, "Per salvare la configurazione del nodo devi accedere di nuovo." }
                Button { size: ButtonSize::Sm, onclick: move |_| is_open.set(true), "Accedi di nuovo" }
            }
        }
        Modal { is_open, aria_label: Some("Accesso".to_string()), size: Some(ModalSize::Md),
            AuthCard { compact: true }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut is_open = use_signal(|| false);

Alert { variant: AlertVariant::Warning, title: Some("Sessione scaduta".to_string()),
    Group { gap: SpacingToken::Md, justify: JustifyContent::SpaceBetween,
        Text { size: TextSize::Sm, "Per salvare la configurazione del nodo devi accedere di nuovo." }
        Button { size: ButtonSize::Sm, onclick: move |_| is_open.set(true), "Accedi di nuovo" }
    }
}
Modal { is_open, aria_label: Some("Accesso".to_string()), size: Some(ModalSize::Md),
    // scenes/auth_card.rs: Card con TextInput, PinInput, Checkbox, Divider e Alert.
    AuthCard { compact: true }
}"#;

const PROPS: &[PropDoc] = &[
    prop("is_open", "Signal<bool>", "-", "Apre e chiude il dialogo; il modal lo riporta a false quando si chiude da solo."),
    prop("title", "Option<String>", "None", "Titolo nell'intestazione, collegato con aria-labelledby."),
    prop("aria_label", "Option<String>", "None", "Nome accessibile quando non c'è un titolo visibile."),
    prop("size", "Option<ModalSize>", "Md", "Larghezza: Sm, Md, Lg, Xl, Xxl."),
    prop("close_on_escape", "bool", "true", "Esc chiude il dialogo."),
    prop("close_on_backdrop", "bool", "true", "Il clic sul backdrop chiude il dialogo."),
    prop("close_label", "Option<String>", "None", "Etichetta del pulsante X (da Locale se assente)."),
    prop("glow", "Option<bool>", "None", "Glow perimetrale --sd-glow-md opzionale."),
    prop("id", "Option<String>", "None", "Id del dialogo; generato se assente."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul pannello."),
    prop("children", "Element", "-", "Corpo del dialogo, azioni comprese."),
];

pub fn story() -> Story {
    Story {
        slug: "modals",
        name: "Modal",
        category: Category::Overlay,
        features: &["modal"],
        icon: IconKind::Eye,
        tagline: "Finestra di dialogo sopra la pagina per un compito breve che richiede attenzione.",
        keywords: &["modal", "dialog", "finestra", "overlay", "popup", "dialogo"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Modal",
            controls: vec![
                Control::text("title", "Titolo", "Ruota la chiave API").emit(Emit::Template("title: Some(\"{v}\".to_string())")),
                Control::choice("size", "Dimensione", SIZE_OPTS, 1).emit(Emit::Template("size: Some({v})")),
                Control::bool("close_on_escape", "Chiudi con Esc", true),
                Control::bool("close_on_backdrop", "Chiudi dal backdrop", true),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["is_open,"],
            children_code: Some(BODY_CODE),
            stage: Stage::Fill,
        }),
        examples: vec![
            Example {
                id: "no-title",
                title: "Senza titolo",
                description: "Conferma compatta: nome accessibile da aria_label.",
                code: NO_TITLE_CODE,
                render: || rsx! { NoTitle {} },
                span: Span::Half,
            },
            Example {
                id: "blocking",
                title: "Bloccante",
                description: "Esc e backdrop disattivati durante un'operazione lunga.",
                code: BLOCKING_CODE,
                render: || rsx! { Blocking {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Sessione scaduta: il form di accesso si apre in un modal.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Modal", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Floating panel (language.md §3.2) + backdrop (§3.3)",
            depth: "Raised-max",
            tokens: &["--sd-surface-2", "--sd-border-strong", "--sd-elev-3", "--sd-radius-xl", "--sd-scrim", "--sd-backdrop-blur", "--sd-space-4"],
            notes: "Backdrop identico a drawer, alert dialog e command palette; il pannello entra con sd-pop-in.",
        }),
        a11y: &[
            "Pannello con role=dialog e aria-modal=true, nominato dal titolo (aria-labelledby) o da aria_label.",
            "All'apertura il focus va al dialogo e Tab / Maiusc+Tab restano intrappolati al suo interno.",
            "Esc chiude il dialogo finché close_on_escape è attivo.",
            "Lo scroll della pagina è bloccato mentre il modal è aperto.",
            "Il pulsante X è un CloseButton con etichetta da Locale (close_label).",
        ],
        related: &["alert-dialog", "drawer", "popover", "command-palette"],
    }
}
