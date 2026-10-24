//! Story: Switch (feature `switch`). Stateful: the playground goes through `SwitchDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const PRELUDE: &str = "let checked = use_signal(|| true);";

#[component]
fn SwitchDemo(knobs: Knobs) -> Element {
    let checked = use_signal(|| true);
    rsx! {
        Switch {
            checked,
            label: Some(knobs.text("label")).filter(|s| !s.is_empty()),
            description: Some(knobs.text("description")).filter(|s| !s.is_empty()),
            disabled: knobs.bool("disabled"),
            glow: knobs.bool("glow").then_some(true),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        SwitchDemo { knobs: k.clone() }
    }
}

#[component]
fn Preferences() -> Element {
    let push = use_signal(|| true);
    let email = use_signal(|| false);
    let quiet = use_signal(|| true);
    rsx! {
        Stack { gap: SpacingToken::Md,
            Switch {
                checked: push,
                label: Some("Notifiche push".to_string()),
                description: Some("Avvisi immediati su latenza e peer offline.".to_string()),
            }
            Switch {
                checked: email,
                label: Some("Riepilogo via email".to_string()),
                description: Some("Un messaggio al giorno alle 08:00.".to_string()),
            }
            Switch {
                checked: quiet,
                label: Some("Non disturbare di notte".to_string()),
                description: Some("Silenzia tutto tra le 22 e le 7.".to_string()),
            }
        }
    }
}
const PREFERENCES_CODE: &str = r#"let push = use_signal(|| true);
let email = use_signal(|| false);
let quiet = use_signal(|| true);

Stack { gap: SpacingToken::Md,
    Switch {
        checked: push,
        label: Some("Notifiche push".to_string()),
        description: Some("Avvisi immediati su latenza e peer offline.".to_string()),
    }
    Switch {
        checked: email,
        label: Some("Riepilogo via email".to_string()),
        description: Some("Un messaggio al giorno alle 08:00.".to_string()),
    }
    Switch {
        checked: quiet,
        label: Some("Non disturbare di notte".to_string()),
        description: Some("Silenzia tutto tra le 22 e le 7.".to_string()),
    }
}"#;

#[component]
fn States() -> Element {
    let on = use_signal(|| true);
    let off = use_signal(|| false);
    let locked_on = use_signal(|| true);
    let locked_off = use_signal(|| false);
    rsx! {
        SimpleGrid { cols: 2, spacing: GridSpacing::Md,
            Switch { checked: on, label: Some("Attivo".to_string()) }
            Switch { checked: off, label: Some("Spento".to_string()) }
            Switch { checked: locked_on, label: Some("Bloccato attivo".to_string()), disabled: true }
            Switch { checked: locked_off, label: Some("Bloccato spento".to_string()), disabled: true }
        }
    }
}
const STATES_CODE: &str = r#"let on = use_signal(|| true);
let off = use_signal(|| false);
let locked_on = use_signal(|| true);
let locked_off = use_signal(|| false);

SimpleGrid { cols: 2, spacing: GridSpacing::Md,
    Switch { checked: on, label: Some("Attivo".to_string()) }
    Switch { checked: off, label: Some("Spento".to_string()) }
    Switch { checked: locked_on, label: Some("Bloccato attivo".to_string()), disabled: true }
    Switch { checked: locked_off, label: Some("Bloccato spento".to_string()), disabled: true }
}"#;

#[component]
fn Maintenance() -> Element {
    let maintenance = use_signal(|| false);
    let mut toast = use_notification();
    rsx! {
        Stack { gap: SpacingToken::Md,
            Switch {
                checked: maintenance,
                label: Some("Modalità manutenzione".to_string()),
                description: Some("I nuovi peer vengono rifiutati.".to_string()),
                on_change: move |on: bool| {
                    if on {
                        toast.warning("Manutenzione attivata su edge-07");
                    } else {
                        toast.success("edge-07 di nuovo in servizio");
                    }
                },
            }
            if maintenance() {
                Alert { variant: AlertVariant::Warning, title: Some("Nodo in manutenzione".to_string()),
                    "Le sessioni aperte restano attive fino alla scadenza."
                }
            }
        }
    }
}
const MAINTENANCE_CODE: &str = r#"let maintenance = use_signal(|| false);
let mut toast = use_notification();

Stack { gap: SpacingToken::Md,
    Switch {
        checked: maintenance,
        label: Some("Modalità manutenzione".to_string()),
        description: Some("I nuovi peer vengono rifiutati.".to_string()),
        on_change: move |on: bool| {
            if on {
                toast.warning("Manutenzione attivata su edge-07");
            } else {
                toast.success("edge-07 di nuovo in servizio");
            }
        },
    }
    if maintenance() {
        Alert { variant: AlertVariant::Warning, title: Some("Nodo in manutenzione".to_string()),
            "Le sessioni aperte restano attive fino alla scadenza."
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::SettingsForm { compact: false }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/settings_form.rs (estratto): il relay automatico nel Fieldset "Rete".
let relay = use_signal(|| true);

Switch {
    checked: relay,
    label: Some("Relay automatico".to_string()),
    description: Some("Inoltra il traffico dei peer dietro NAT.".to_string()),
}"#;

const PROPS: &[PropDoc] = &[
    prop("checked", "Signal<bool>", "-", "Acceso o spento."),
    prop("label / description", "Option<String>", "None", "Etichetta cliccabile e riga di aiuto."),
    prop("disabled", "bool", "false", "Non modificabile, opacità --sd-disabled-opacity."),
    prop("glow", "Option<bool>", "None", "Forza o rimuove il glow quando acceso."),
    prop("on_change", "Option<EventHandler<bool>>", "None", "Nuovo stato dopo il click."),
    prop("id", "Option<String>", "None", "id del pulsante; generato se assente."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];

pub fn story() -> Story {
    Story {
        slug: "switch",
        name: "Switch",
        category: Category::Form,
        features: &["switch"],
        icon: IconKind::Settings,
        tagline: "Interruttore per un'impostazione che ha effetto subito, senza conferma.",
        keywords: &["switch", "toggle", "interruttore", "impostazione", "on off", "preferenze"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Switch",
            controls: vec![
                Control::text("label", "Etichetta", "Relay automatico").emit(Emit::Template("label: Some(\"{v}\".to_string())")),
                Control::text("description", "Descrizione", "Inoltra il traffico dei peer dietro NAT.")
                    .emit(Emit::Template("description: Some(\"{v}\".to_string())")),
                Control::bool("disabled", "Disabilitato", false),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["checked,"],
            children_code: None,
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "preferences",
                title: "Preferenze",
                description: "Un elenco di impostazioni con descrizione.",
                code: PREFERENCES_CODE,
                render: || rsx! { Preferences {} },
                span: Span::Half,
            },
            Example {
                id: "states",
                title: "Stati",
                description: "Acceso, spento e le due versioni disabilitate.",
                code: STATES_CODE,
                render: || rsx! { States {} },
                span: Span::Half,
            },
            Example {
                id: "maintenance",
                title: "Effetto immediato",
                description: "on_change mostra un toast e un avviso legato allo stato.",
                code: MAINTENANCE_CODE,
                render: || rsx! { Maintenance {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il relay automatico nel form Impostazioni.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Switch", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Recessed field, toggle (§3.6) + value fill (§3.11)",
            depth: "Well (spento) · value fill (acceso)",
            tokens: &["--sd-well", "--sd-border-strong", "--sd-accent", "--sd-glow-sm", "--sd-radius-full", "--sd-dur-fast"],
            notes: "La forma a pillola resta tale con ogni scala di raggio; acceso è l'unico stato con accento e glow.",
        }),
        a11y: &[
            "Elemento <button> con role=switch e aria-checked.",
            "Etichetta e descrizione collegate con aria-labelledby e aria-describedby.",
            "Invio e Spazio cambiano lo stato; il focus mostra --sd-focus-ring.",
            "Il pallino disegnato è aria-hidden.",
        ],
        related: &["checkbox", "segmented-controls", "radio", "form"],
    }
}
