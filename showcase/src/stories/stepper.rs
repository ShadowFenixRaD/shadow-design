//! Story: Stepper, StepperStep, StepperCompleted (feature `stepper`). Stateful playground
//! (`StepperDemo { knobs }`): the active step lives in a signal.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const ORIENTATIONS: [StepperOrientation; 2] = [StepperOrientation::Horizontal, StepperOrientation::Vertical];
const ORIENTATION_OPTS: &[ChoiceOption] =
    &[opt("Orizzontale", "StepperOrientation::Horizontal"), opt("Verticale", "StepperOrientation::Vertical")];
const STATES: [Option<StepState>; 2] = [None, Some(StepState::Error)];
const STATE_OPTS: &[ChoiceOption] = &[opt("Automatico", "None"), opt("Errore", "Some(StepState::Error)")];

const PRELUDE: &str = "let mut active = use_signal(|| 1usize);";

const CHILDREN: &str = r#"StepperStep { step: 0, label: "Account".to_string(), description: Some("Email e password".to_string()),
    Button { size: ButtonSize::Sm, onclick: move |_| active.set(1), "Continua" }
}
StepperStep { step: 1, label: "Nodo".to_string(), description: Some("Nome e regione".to_string()), state: {network_state},
    Button { size: ButtonSize::Sm, onclick: move |_| active.set(2), "Continua" }
}
StepperStep { step: 2, label: "Chiavi".to_string(), description: Some("Kyber-1024".to_string()),
    Button { size: ButtonSize::Sm, onclick: move |_| active.set(3), "Completa" }
}
StepperCompleted { completed_step: Some(3),
    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, onclick: move |_| active.set(0), "Ricomincia" }
}"#;

#[component]
fn StepperDemo(knobs: Knobs) -> Element {
    let mut active = use_signal(|| 1usize);
    rsx! {
        Stepper {
            active: active(),
            on_change: move |s| active.set(s),
            orientation: Some(knobs.pick("orientation", &ORIENTATIONS)),
            allow_click_completed: knobs.bool("allow_click_completed"),
            StepperStep { step: 0, label: "Account".to_string(), description: Some("Email e password".to_string()),
                Button { size: ButtonSize::Sm, onclick: move |_| active.set(1), "Continua" }
            }
            StepperStep {
                step: 1,
                label: "Nodo".to_string(),
                description: Some("Nome e regione".to_string()),
                state: knobs.pick("network_state", &STATES),
                Button { size: ButtonSize::Sm, onclick: move |_| active.set(2), "Continua" }
            }
            StepperStep { step: 2, label: "Chiavi".to_string(), description: Some("Kyber-1024".to_string()),
                Button { size: ButtonSize::Sm, onclick: move |_| active.set(3), "Completa" }
            }
            StepperCompleted { completed_step: Some(3),
                Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, onclick: move |_| active.set(0), "Ricomincia" }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        StepperDemo { knobs: k.clone() }
    }
}

#[component]
fn Icons() -> Element {
    rsx! {
        Stepper { active: 1,
            StepperStep { step: 0, label: "Identità".to_string(), icon: Some(SvgIcon::kind(IconKind::Fingerprint)) }
            StepperStep { step: 1, label: "Rete".to_string(), icon: Some(SvgIcon::kind(IconKind::Wifi)) }
            StepperStep { step: 2, label: "Sicurezza".to_string(), icon: Some(SvgIcon::kind(IconKind::ShieldCheck)) }
        }
    }
}
const ICONS_CODE: &str = r#"Stepper { active: 1,
    StepperStep { step: 0, label: "Identità".to_string(), icon: Some(SvgIcon::kind(IconKind::Fingerprint)) }
    StepperStep { step: 1, label: "Rete".to_string(), icon: Some(SvgIcon::kind(IconKind::Wifi)) }
    StepperStep { step: 2, label: "Sicurezza".to_string(), icon: Some(SvgIcon::kind(IconKind::ShieldCheck)) }
}"#;

#[component]
fn Vertical() -> Element {
    rsx! {
        Stepper { active: 2, orientation: Some(StepperOrientation::Vertical),
            StepperStep { step: 0, label: "Ordine ricevuto".to_string(), description: Some("Lunedì 09:12".to_string()) }
            StepperStep { step: 1, label: "Nodo assemblato".to_string(), description: Some("Martedì 15:40".to_string()) }
            StepperStep { step: 2, label: "In spedizione".to_string(), description: Some("Arrivo previsto giovedì".to_string()) }
            StepperStep { step: 3, label: "Consegnato".to_string() }
        }
    }
}
const VERTICAL_CODE: &str = r#"Stepper { active: 2, orientation: Some(StepperOrientation::Vertical),
    StepperStep { step: 0, label: "Ordine ricevuto".to_string(), description: Some("Lunedì 09:12".to_string()) }
    StepperStep { step: 1, label: "Nodo assemblato".to_string(), description: Some("Martedì 15:40".to_string()) }
    StepperStep { step: 2, label: "In spedizione".to_string(), description: Some("Arrivo previsto giovedì".to_string()) }
    StepperStep { step: 3, label: "Consegnato".to_string() }
}"#;

#[component]
fn InContext() -> Element {
    let mut active = use_signal(|| 0usize);
    let host = use_signal(|| "edge-22".to_string());
    let region = use_signal(|| "eu-south".to_string());
    let pq = use_signal(|| true);
    let mut toast = use_notification();
    rsx! {
        Card { title: Some("Nuovo nodo".to_string()), subtitle: Some("Provisioning in tre passi".to_string()),
            Stack { gap: SpacingToken::Lg,
                Stepper { active: active(), on_change: move |s| active.set(s),
                    StepperStep { step: 0, label: "Identità".to_string(), description: Some("Nome host".to_string()) }
                    StepperStep { step: 1, label: "Rete".to_string(), description: Some("Regione".to_string()) }
                    StepperStep { step: 2, label: "Sicurezza".to_string(), description: Some("Cifratura".to_string()) }
                }
                if active() == 0 {
                    TextInput { value: host, label: Some("Nome host".to_string()) }
                } else if active() == 1 {
                    TextInput { value: region, label: Some("Regione".to_string()) }
                } else if active() == 2 {
                    Switch { checked: pq, label: Some("Handshake post-quantistico (Kyber-1024)".to_string()) }
                } else {
                    Alert { variant: AlertVariant::Success, title: Some("Nodo pronto".to_string()),
                        "{host} è attivo in {region}."
                    }
                }
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button {
                        variant: ButtonVariant::Ghost,
                        disabled: active() == 0,
                        onclick: move |_| active.set(active().saturating_sub(1)),
                        "Indietro"
                    }
                    Button {
                        disabled: active() > 2,
                        onclick: move |_| {
                            if active() == 2 {
                                toast.success("Provisioning completato");
                            }
                            active.set(active() + 1);
                        },
                        if active() == 2 { "Crea nodo" } else { "Avanti" }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut active = use_signal(|| 0usize);
let host = use_signal(|| "edge-22".to_string());
let region = use_signal(|| "eu-south".to_string());
let pq = use_signal(|| true);
let mut toast = use_notification();

Card { title: Some("Nuovo nodo".to_string()), subtitle: Some("Provisioning in tre passi".to_string()),
    Stack { gap: SpacingToken::Lg,
        Stepper { active: active(), on_change: move |s| active.set(s),
            StepperStep { step: 0, label: "Identità".to_string(), description: Some("Nome host".to_string()) }
            StepperStep { step: 1, label: "Rete".to_string(), description: Some("Regione".to_string()) }
            StepperStep { step: 2, label: "Sicurezza".to_string(), description: Some("Cifratura".to_string()) }
        }
        if active() == 0 {
            TextInput { value: host, label: Some("Nome host".to_string()) }
        } else if active() == 1 {
            TextInput { value: region, label: Some("Regione".to_string()) }
        } else if active() == 2 {
            Switch { checked: pq, label: Some("Handshake post-quantistico (Kyber-1024)".to_string()) }
        } else {
            Alert { variant: AlertVariant::Success, title: Some("Nodo pronto".to_string()),
                "{host} è attivo in {region}."
            }
        }
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button {
                variant: ButtonVariant::Ghost,
                disabled: active() == 0,
                onclick: move |_| active.set(active().saturating_sub(1)),
                "Indietro"
            }
            Button {
                disabled: active() > 2,
                onclick: move |_| {
                    if active() == 2 {
                        toast.success("Provisioning completato");
                    }
                    active.set(active() + 1);
                },
                if active() == 2 { "Crea nodo" } else { "Avanti" }
            }
        }
    }
}"#;

const STEPPER_PROPS: &[PropDoc] = &[
    prop("active", "usize", "0", "Indice del passo attivo; i precedenti risultano completati."),
    prop("orientation", "Option<StepperOrientation>", "None (Horizontal)", "Horizontal o Vertical."),
    prop("allow_click_completed", "bool", "true", "I passi completati diventano pulsanti per tornare indietro."),
    prop("on_change", "Option<EventHandler<usize>>", "None", "Passo scelto con un click."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const STEP_PROPS: &[PropDoc] = &[
    prop("step", "usize", "0", "Indice del passo, da 0."),
    prop("label", "String", "-", "Titolo del passo."),
    prop("description", "Option<String>", "None", "Riga secondaria."),
    prop("icon / completed_icon", "Option<SvgIcon>", "None", "Icona al posto del numero, e quella da completato."),
    prop("state", "Option<StepState>", "None", "Forza Upcoming, Active, Completed o Error."),
    prop("disabled", "bool", "false", "Passo non cliccabile."),
    prop("children", "Option<Element>", "None", "Contenuto mostrato quando il passo è attivo."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const COMPLETED_PROPS: &[PropDoc] = &[
    prop("completed_step", "Option<usize>", "None", "Appare quando active raggiunge questo indice."),
    prop("show", "Option<bool>", "None", "Visibilità forzata."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "stepper",
        name: "Stepper",
        category: Category::Navigazione,
        features: &["stepper"],
        icon: IconKind::CheckCircle,
        tagline: "Guida una procedura a passi, orizzontale o verticale, con stato di avanzamento ed errori.",
        keywords: &["stepper", "wizard", "passi", "steps", "procedura", "onboarding", "progress"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Stepper",
            controls: vec![
                Control::choice("orientation", "Orientamento", ORIENTATION_OPTS, 0)
                    .emit(Emit::Template("orientation: Some({v})")),
                Control::bool("allow_click_completed", "Torna ai completati", true),
                Control::choice("network_state", "Stato del passo Nodo", STATE_OPTS, 0).emit(Emit::Skip),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["active: active(),", "on_change: move |s| active.set(s),"],
            children_code: Some(CHILDREN),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "icons",
                title: "Con icone",
                description: "Icone al posto dei numeri.",
                code: ICONS_CODE,
                render: || rsx! { Icons {} },
                span: Span::Half,
            },
            Example {
                id: "vertical",
                title: "Verticale",
                description: "Tracciamento di una spedizione.",
                code: VERTICAL_CODE,
                render: || rsx! { Vertical {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Procedura guidata per registrare un nodo, con Indietro e Avanti.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Stepper", rows: STEPPER_PROPS },
            PropGroup { component: "StepperStep", rows: STEP_PROPS },
            PropGroup { component: "StepperCompleted", rows: COMPLETED_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Item (§3.4) con passo attivo sd-selected (§3.7) e value fill sui completati (§3.11)",
            depth: "Void (linea) · item",
            tokens: &["--sd-accent", "--sd-accent-soft", "--sd-accent-strong", "--sd-border-subtle", "--sd-danger-strong", "--sd-glow-sm"],
            notes: "Un solo passo attivo; i completati usano l'accento pieno, l'errore la tinta danger.",
        }),
        a11y: &[
            "Il passo attivo ha aria-current=step.",
            "I passi cliccabili (attivo e completati) sono <button> nativi; gli altri sono testo statico.",
            "La linea tra i passi è aria-hidden.",
            "Lo stato di errore usa anche un'icona, non solo il colore.",
        ],
        related: &["progress", "tabs", "pagination", "form"],
    }
}
