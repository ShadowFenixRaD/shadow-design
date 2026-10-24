//! Story: Checkbox (feature `checkbox`). Stateful: the playground goes through `CheckboxDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const SIZES: [CheckboxSize; 3] = [CheckboxSize::Sm, CheckboxSize::Md, CheckboxSize::Lg];
const SIZE_OPTS: &[ChoiceOption] = &[opt("Sm", "CheckboxSize::Sm"), opt("Md", "CheckboxSize::Md"), opt("Lg", "CheckboxSize::Lg")];

const PRELUDE: &str = "let checked = use_signal(|| true);";

#[component]
fn CheckboxDemo(knobs: Knobs) -> Element {
    let checked = use_signal(|| true);
    rsx! {
        Checkbox {
            checked,
            label: Some(knobs.text("label")).filter(|s| !s.is_empty()),
            description: Some(knobs.text("description")).filter(|s| !s.is_empty()),
            size: knobs.pick("size", &SIZES),
            indeterminate: knobs.bool("indeterminate"),
            disabled: knobs.bool("disabled"),
            error: knobs.bool("error").then(|| "Accetta i termini per continuare".to_string()),
            glow: knobs.bool("glow").then_some(true),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        CheckboxDemo { knobs: k.clone() }
    }
}

#[component]
fn Sizes() -> Element {
    let small = use_signal(|| true);
    let medium = use_signal(|| true);
    let large = use_signal(|| false);
    rsx! {
        Stack { gap: SpacingToken::Sm,
            Checkbox { checked: small, size: CheckboxSize::Sm, label: Some("Sm · liste dense".to_string()) }
            Checkbox { checked: medium, size: CheckboxSize::Md, label: Some("Md · form standard".to_string()) }
            Checkbox { checked: large, size: CheckboxSize::Lg, label: Some("Lg · touch".to_string()) }
        }
    }
}
const SIZES_CODE: &str = r#"let small = use_signal(|| true);
let medium = use_signal(|| true);
let large = use_signal(|| false);

Stack { gap: SpacingToken::Sm,
    Checkbox { checked: small, size: CheckboxSize::Sm, label: Some("Sm · liste dense".to_string()) }
    Checkbox { checked: medium, size: CheckboxSize::Md, label: Some("Md · form standard".to_string()) }
    Checkbox { checked: large, size: CheckboxSize::Lg, label: Some("Lg · touch".to_string()) }
}"#;

#[component]
fn States() -> Element {
    let terms = use_signal(|| false);
    let locked = use_signal(|| true);
    let backup = use_signal(|| true);
    rsx! {
        Stack { gap: SpacingToken::Md,
            Checkbox {
                checked: terms,
                label: Some("Accetto i termini di servizio".to_string()),
                error: if terms() { None } else { Some("Obbligatorio per creare il nodo".to_string()) },
            }
            Checkbox {
                checked: locked,
                label: Some("Cifratura end-to-end".to_string()),
                description: Some("Sempre attiva sul piano Team.".to_string()),
                disabled: true,
            }
            Checkbox {
                checked: backup,
                label: Some("Backup giornaliero".to_string()),
                description: Some("Snapshot alle 03:00, conservati 30 giorni.".to_string()),
            }
        }
    }
}
const STATES_CODE: &str = r#"let terms = use_signal(|| false);
let locked = use_signal(|| true);
let backup = use_signal(|| true);

Stack { gap: SpacingToken::Md,
    Checkbox {
        checked: terms,
        label: Some("Accetto i termini di servizio".to_string()),
        error: if terms() { None } else { Some("Obbligatorio per creare il nodo".to_string()) },
    }
    Checkbox {
        checked: locked,
        label: Some("Cifratura end-to-end".to_string()),
        description: Some("Sempre attiva sul piano Team.".to_string()),
        disabled: true,
    }
    Checkbox {
        checked: backup,
        label: Some("Backup giornaliero".to_string()),
        description: Some("Snapshot alle 03:00, conservati 30 giorni.".to_string()),
    }
}"#;

#[component]
fn SelectAll() -> Element {
    let mut all = use_signal(|| false);
    let mut alerts = use_signal(|| true);
    let mut deploys = use_signal(|| false);
    let mut digest = use_signal(|| true);
    use_effect(move || all.set(alerts() && deploys() && digest()));
    let some = alerts() || deploys() || digest();
    rsx! {
        Stack { gap: SpacingToken::Sm,
            Checkbox {
                checked: all,
                label: Some("Tutte le notifiche".to_string()),
                indeterminate: some && !all(),
                on_change: move |v: bool| {
                    alerts.set(v);
                    deploys.set(v);
                    digest.set(v);
                },
            }
            Divider {}
            Checkbox { checked: alerts, label: Some("Avvisi di latenza".to_string()) }
            Checkbox { checked: deploys, label: Some("Deploy completati".to_string()) }
            Checkbox { checked: digest, label: Some("Riepilogo settimanale".to_string()) }
        }
    }
}
const SELECT_ALL_CODE: &str = r#"let mut all = use_signal(|| false);
let mut alerts = use_signal(|| true);
let mut deploys = use_signal(|| false);
let mut digest = use_signal(|| true);
use_effect(move || all.set(alerts() && deploys() && digest()));
let some = alerts() || deploys() || digest();

Stack { gap: SpacingToken::Sm,
    Checkbox {
        checked: all,
        label: Some("Tutte le notifiche".to_string()),
        indeterminate: some && !all(),
        on_change: move |v: bool| {
            alerts.set(v);
            deploys.set(v);
            digest.set(v);
        },
    }
    Divider {}
    Checkbox { checked: alerts, label: Some("Avvisi di latenza".to_string()) }
    Checkbox { checked: deploys, label: Some("Deploy completati".to_string()) }
    Checkbox { checked: digest, label: Some("Riepilogo settimanale".to_string()) }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::SettingsForm { compact: false }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/settings_form.rs (estratto): Checkbox in fondo al Fieldset "Rete", sotto lo Switch.
let telemetry = use_signal(|| false);

Switch {
    checked: relay,
    label: Some("Relay automatico".to_string()),
    description: Some("Inoltra il traffico dei peer dietro NAT.".to_string()),
}
Checkbox { checked: telemetry, label: Some("Invia telemetria anonima".to_string()) }"#;

const PROPS: &[PropDoc] = &[
    prop("checked", "Signal<bool>", "-", "Stato selezionato."),
    prop("label / description", "Option<String>", "None", "Etichetta cliccabile e riga di aiuto sotto."),
    prop("size", "CheckboxSize", "Md", "Sm, Md o Lg (alias di ControlSize)."),
    prop("indeterminate", "bool", "false", "Trattino al posto della spunta e aria-checked=mixed; un click seleziona."),
    prop("error", "Option<String>", "None", "Bordo danger, aria-invalid e messaggio."),
    prop("disabled", "bool", "false", "Non modificabile, opacità --sd-disabled-opacity."),
    prop("glow", "Option<bool>", "None", "Forza o rimuove il glow quando selezionata."),
    prop("on_change", "Option<EventHandler<bool>>", "None", "Nuovo stato dopo il click."),
    prop("id", "Option<String>", "None", "id dell'input; generato se assente."),
    prop("class", "String", "\"\"", "Classi aggiuntive sull'etichetta."),
];

pub fn story() -> Story {
    Story {
        slug: "checkbox",
        name: "Checkbox",
        category: Category::Form,
        features: &["checkbox"],
        icon: IconKind::CheckCircle,
        tagline: "Casella di selezione con descrizione, stato indeterminato ed errore.",
        keywords: &["checkbox", "casella", "spunta", "indeterminato", "seleziona tutto", "consenso"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Checkbox",
            controls: vec![
                Control::text("label", "Etichetta", "Invia telemetria anonima")
                    .emit(Emit::Template("label: Some(\"{v}\".to_string())")),
                Control::text("description", "Descrizione", "Solo metriche di latenza, mai contenuti.")
                    .emit(Emit::Template("description: Some(\"{v}\".to_string())")),
                Control::choice("size", "Dimensione", SIZE_OPTS, 1),
                Control::bool("indeterminate", "Indeterminato", false),
                Control::bool("disabled", "Disabilitato", false),
                Control::bool("error", "Errore", false)
                    .emit(Emit::Template("error: Some(\"Accetta i termini per continuare\".to_string())")),
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
                id: "sizes",
                title: "Dimensioni",
                description: "Etichette allineate su tutte e tre le taglie.",
                code: SIZES_CODE,
                render: || rsx! { Sizes {} },
                span: Span::Half,
            },
            Example {
                id: "states",
                title: "Stati",
                description: "Errore finché non è spuntata, disabilitata, con descrizione.",
                code: STATES_CODE,
                render: || rsx! { States {} },
                span: Span::Half,
            },
            Example {
                id: "select-all",
                title: "Seleziona tutto",
                description: "Il genitore è indeterminato quando solo alcune voci sono attive.",
                code: SELECT_ALL_CODE,
                render: || rsx! { SelectAll {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il consenso alla telemetria nel form Impostazioni.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Checkbox", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Recessed field, toggle (language.md §3.6)",
            depth: "Well (a riposo) · value fill (selezionata)",
            tokens: &["--sd-well", "--sd-border-strong", "--sd-accent", "--sd-accent-fg", "--sd-glow-sm", "--sd-radius-sm"],
            notes: "A riposo è un pozzetto; selezionata prende il riempimento accento con la spunta in --sd-accent-fg.",
        }),
        a11y: &[
            "Input checkbox nativo dentro un <label>: Spazio la attiva, il click sull'etichetta pure.",
            "indeterminate imposta aria-checked=mixed.",
            "description ed error sono collegati con aria-describedby; error imposta aria-invalid.",
            "La casella disegnata è aria-hidden: lo stato lo annuncia l'input nativo.",
        ],
        related: &["switch", "radio", "chip", "form", "transfer-list"],
    }
}
