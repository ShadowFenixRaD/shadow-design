//! Story: use_form (feature `form`). A hook, not a component: no playground, three examples.
//! Each example shows its model and validator next to the component body.
use std::collections::HashMap;

use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

#[derive(Clone, PartialEq, Default)]
struct Invite {
    email: String,
}

fn validate_invite(v: &Invite) -> HashMap<String, String> {
    let mut errors = HashMap::new();
    if !v.email.contains('@') || !v.email.contains('.') {
        errors.insert("email".to_string(), "Inserisci un indirizzo email valido".to_string());
    }
    errors
}

#[component]
fn SubmitValidation() -> Element {
    let form = use_form(Invite::default(), Some(validate_invite));
    let email = use_signal(String::new);
    let mut toast = use_notification();
    rsx! {
        Stack { gap: SpacingToken::Md,
            TextInput {
                value: email,
                label: Some("Invita un operatore".to_string()),
                placeholder: Some("nome@shadowfenix.dev".to_string()),
                error: form.get_error("email"),
            }
            Group { justify: JustifyContent::End,
                Button {
                    onclick: move |_| {
                        form.set_values(Invite { email: email() });
                        form.submit(|v| {
                            toast.success(format!("Invito inviato a {}", v.email));
                        });
                    },
                    "Invia invito"
                }
            }
        }
    }
}
const SUBMIT_CODE: &str = r#"#[derive(Clone, PartialEq, Default)]
struct Invite {
    email: String,
}

fn validate_invite(v: &Invite) -> HashMap<String, String> {
    let mut errors = HashMap::new();
    if !v.email.contains('@') || !v.email.contains('.') {
        errors.insert("email".to_string(), "Inserisci un indirizzo email valido".to_string());
    }
    errors
}

let form = use_form(Invite::default(), Some(validate_invite));
let email = use_signal(String::new);
let mut toast = use_notification();

Stack { gap: SpacingToken::Md,
    TextInput {
        value: email,
        label: Some("Invita un operatore".to_string()),
        placeholder: Some("nome@shadowfenix.dev".to_string()),
        error: form.get_error("email"),
    }
    Group { justify: JustifyContent::End,
        Button {
            onclick: move |_| {
                form.set_values(Invite { email: email() });
                form.submit(|v| {
                    toast.success(format!("Invito inviato a {}", v.email));
                });
            },
            "Invia invito"
        }
    }
}"#;

#[derive(Clone, PartialEq)]
struct Profile {
    name: String,
}

#[component]
fn DirtyReset() -> Element {
    let form = use_form(Profile { name: "edge-07".to_string() }, None::<fn(&Profile) -> HashMap<String, String>>);
    let mut name = use_signal(|| "edge-07".to_string());
    use_effect(move || {
        let v = name();
        if form.read().name != v {
            form.with_mut(|p| p.name = v);
        }
    });
    rsx! {
        Stack { gap: SpacingToken::Md,
            TextInput { value: name, label: Some("Nome del nodo".to_string()) }
            Group { justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
                if (form.is_dirty)() {
                    Badge { label: "Modifiche non salvate".to_string(), variant: BadgeVariant::Warning, dot: true }
                } else {
                    Badge { label: "Salvato".to_string(), variant: BadgeVariant::Success, dot: true }
                }
                Button {
                    variant: ButtonVariant::Secondary,
                    disabled: !(form.is_dirty)(),
                    onclick: move |_| {
                        form.reset();
                        name.set(form.read().name);
                    },
                    "Ripristina"
                }
            }
        }
    }
}
const DIRTY_CODE: &str = r#"#[derive(Clone, PartialEq)]
struct Profile {
    name: String,
}

let form = use_form(Profile { name: "edge-07".to_string() }, None::<fn(&Profile) -> HashMap<String, String>>);
let mut name = use_signal(|| "edge-07".to_string());
use_effect(move || {
    let v = name();
    if form.read().name != v {
        form.with_mut(|p| p.name = v);
    }
});

Stack { gap: SpacingToken::Md,
    TextInput { value: name, label: Some("Nome del nodo".to_string()) }
    Group { justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
        if (form.is_dirty)() {
            Badge { label: "Modifiche non salvate".to_string(), variant: BadgeVariant::Warning, dot: true }
        } else {
            Badge { label: "Salvato".to_string(), variant: BadgeVariant::Success, dot: true }
        }
        Button {
            variant: ButtonVariant::Secondary,
            disabled: !(form.is_dirty)(),
            onclick: move |_| {
                form.reset();
                name.set(form.read().name);
            },
            "Ripristina"
        }
    }
}"#;

#[derive(Clone, PartialEq, Default)]
struct Operator {
    name: String,
    email: String,
    role: Option<String>,
    terms: bool,
}

fn validate_operator(v: &Operator) -> HashMap<String, String> {
    let mut errors = HashMap::new();
    if v.name.trim().len() < 3 {
        errors.insert("name".to_string(), "Almeno 3 caratteri".to_string());
    }
    if !v.email.contains('@') {
        errors.insert("email".to_string(), "Email non valida".to_string());
    }
    if v.role.is_none() {
        errors.insert("role".to_string(), "Scegli un ruolo".to_string());
    }
    if !v.terms {
        errors.insert("terms".to_string(), "Obbligatorio".to_string());
    }
    errors
}

#[component]
fn InContext() -> Element {
    let form = use_form(Operator::default(), Some(validate_operator));
    let name = use_signal(String::new);
    let email = use_signal(String::new);
    let role = use_signal(|| None::<String>);
    let terms = use_signal(|| false);
    let mut toast = use_notification();
    let errors = form.errors.read().len();
    rsx! {
        Card {
            title: Some("Nuovo operatore".to_string()),
            subtitle: Some("Accesso alla console Shadowfenix Mesh".to_string()),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
                    if errors > 0 {
                        Badge { label: format!("{errors} campi da correggere"), variant: BadgeVariant::Danger }
                    } else {
                        span {}
                    }
                    Button {
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Check, size: 15 }
                        }),
                        onclick: move |_| {
                            form.set_values(Operator { name: name(), email: email(), role: role(), terms: terms() });
                            form.submit(|v| {
                                toast.success(format!("{} aggiunto alla mesh", v.name));
                            });
                        },
                        "Crea operatore"
                    }
                }
            }),
            Stack { gap: SpacingToken::Md,
                SimpleGrid { cols: 1, sm: Some(2), spacing: GridSpacing::Md,
                    TextInput { value: name, label: Some("Nome".to_string()), error: form.get_error("name") }
                    TextInput {
                        value: email,
                        label: Some("Email".to_string()),
                        input_type: Some("email".to_string()),
                        error: form.get_error("email"),
                    }
                }
                Select {
                    data: vec![
                        SelectItem::new("admin", "Amministratore"),
                        SelectItem::new("ops", "Operatore di rete"),
                        SelectItem::new("audit", "Revisore (sola lettura)"),
                    ],
                    value: role,
                    label: Some("Ruolo".to_string()),
                    error: form.get_error("role"),
                }
                Checkbox {
                    checked: terms,
                    label: Some("Ho letto la policy di accesso".to_string()),
                    error: form.get_error("terms"),
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"#[derive(Clone, PartialEq, Default)]
struct Operator {
    name: String,
    email: String,
    role: Option<String>,
    terms: bool,
}

fn validate_operator(v: &Operator) -> HashMap<String, String> {
    let mut errors = HashMap::new();
    if v.name.trim().len() < 3 {
        errors.insert("name".to_string(), "Almeno 3 caratteri".to_string());
    }
    if !v.email.contains('@') {
        errors.insert("email".to_string(), "Email non valida".to_string());
    }
    if v.role.is_none() {
        errors.insert("role".to_string(), "Scegli un ruolo".to_string());
    }
    if !v.terms {
        errors.insert("terms".to_string(), "Obbligatorio".to_string());
    }
    errors
}

let form = use_form(Operator::default(), Some(validate_operator));
let name = use_signal(String::new);
let email = use_signal(String::new);
let role = use_signal(|| None::<String>);
let terms = use_signal(|| false);
let mut toast = use_notification();
let errors = form.errors.read().len();

Card {
    title: Some("Nuovo operatore".to_string()),
    subtitle: Some("Accesso alla console Shadowfenix Mesh".to_string()),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
            if errors > 0 {
                Badge { label: format!("{errors} campi da correggere"), variant: BadgeVariant::Danger }
            } else {
                span {}
            }
            Button {
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Check, size: 15 }
                }),
                onclick: move |_| {
                    form.set_values(Operator { name: name(), email: email(), role: role(), terms: terms() });
                    form.submit(|v| {
                        toast.success(format!("{} aggiunto alla mesh", v.name));
                    });
                },
                "Crea operatore"
            }
        }
    }),
    Stack { gap: SpacingToken::Md,
        SimpleGrid { cols: 1, sm: Some(2), spacing: GridSpacing::Md,
            TextInput { value: name, label: Some("Nome".to_string()), error: form.get_error("name") }
            TextInput {
                value: email,
                label: Some("Email".to_string()),
                input_type: Some("email".to_string()),
                error: form.get_error("email"),
            }
        }
        Select {
            data: vec![
                SelectItem::new("admin", "Amministratore"),
                SelectItem::new("ops", "Operatore di rete"),
                SelectItem::new("audit", "Revisore (sola lettura)"),
            ],
            value: role,
            label: Some("Ruolo".to_string()),
            error: form.get_error("role"),
        }
        Checkbox {
            checked: terms,
            label: Some("Ho letto la policy di accesso".to_string()),
            error: form.get_error("terms"),
        }
    }
}"#;

const HOOK_PROPS: &[PropDoc] = &[
    prop("use_form(initial, validate)", "fn(T, Option<impl Fn(&T) -> HashMap<String, String>>) -> UseForm<T>", "-", "Crea lo stato del form; T: Clone + 'static."),
    prop("validate", "Option<impl Fn(&T) -> HashMap<String, String>>", "None", "Restituisce campo → messaggio; vuota se tutto è valido."),
];
const STATE_PROPS: &[PropDoc] = &[
    prop("values", "Signal<T>", "-", "Modello corrente."),
    prop("errors", "Signal<HashMap<String, String>>", "{}", "Errori dell'ultima validazione."),
    prop("touched", "Signal<HashMap<String, bool>>", "{}", "Campi toccati (set_touched)."),
    prop("is_dirty", "Signal<bool>", "false", "true dopo set_values o with_mut, false dopo reset."),
    prop(".read() / .set_values(v) / .with_mut(f)", "fn", "-", "Legge o aggiorna il modello (le scritture lo marcano dirty)."),
    prop(".get_error(field) / .set_error / .clear_error / .clear_errors", "fn", "-", "Errori per campo; get_error va diretto nella prop error dei campi."),
    prop(".validate() / .is_valid()", "fn -> bool", "-", "Esegue il validatore e aggiorna errors / legge lo stato."),
    prop(".submit(on_valid)", "fn(FnOnce(T)) -> bool", "-", "Valida e, se valido, chiama on_valid con il modello."),
    prop(".reset()", "fn", "-", "Torna ai valori iniziali e azzera errori, touched e dirty."),
];

pub fn story() -> Story {
    Story {
        slug: "form",
        name: "Form",
        category: Category::Form,
        features: &["form"],
        icon: IconKind::CheckCircle,
        tagline: "Stato e validazione dei moduli con use_form: un modello tipizzato e un errore per campo.",
        keywords: &["form", "use_form", "validazione", "modulo", "errori", "submit"],
        status: Status::Stable,
        playground: None,
        examples: vec![
            Example {
                id: "submit",
                title: "Validazione al submit",
                description: "submit esegue il validatore; l'errore arriva al campo con get_error.",
                code: SUBMIT_CODE,
                render: || rsx! { SubmitValidation {} },
                span: Span::Half,
            },
            Example {
                id: "dirty",
                title: "Modifiche e ripristino",
                description: "is_dirty segnala le modifiche, reset torna ai valori iniziali.",
                code: DIRTY_CODE,
                render: || rsx! { DirtyReset {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Creazione di un operatore: quattro regole, errori per campo e conteggio nel piè di pagina.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "use_form", rows: HOOK_PROPS },
            PropGroup { component: "UseForm<T>", rows: STATE_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Recessed field, stato di errore (language.md §3.6)",
            depth: "Well",
            tokens: &["--sd-danger-border", "--sd-danger", "--sd-glow-danger", "--sd-danger-strong", "--sd-text-xs"],
            notes: "use_form non disegna nulla: gli errori passano alla prop error di ogni campo, che usa il trattamento danger della ricetta.",
        }),
        a11y: &[
            "Gli errori arrivano ai campi tramite error: ogni campo imposta aria-invalid e collega il messaggio con aria-describedby.",
            "Il submit è un Button nativo: si attiva con Invio e Spazio.",
            "Le regole restano in un'unica funzione: lo stesso messaggio è mostrato e annunciato.",
        ],
        related: &["fieldset", "inputs", "select", "checkbox", "buttons"],
    }
}
