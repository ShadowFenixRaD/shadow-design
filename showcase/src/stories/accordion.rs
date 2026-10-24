//! Story: Accordion and Collapsible (feature `accordion`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const VARIANTS: [AccordionVariant; 3] = [AccordionVariant::Default, AccordionVariant::Contained, AccordionVariant::Separated];
const VARIANT_OPTS: &[ChoiceOption] = &[
    opt("Default", "AccordionVariant::Default"),
    opt("Contained", "AccordionVariant::Contained"),
    opt("Separated", "AccordionVariant::Separated"),
];

const CHILDREN: &str = r#"AccordionItem { value: "sicurezza".to_string(),
    AccordionControl { icon: Some(IconKind::Shield), "Come vengono cifrati i dati?" }
    AccordionPanel { "Ogni sessione negozia una chiave Kyber-1024; i dati a riposo usano AES-256-GCM." }
}
AccordionItem { value: "rete".to_string(),
    AccordionControl { icon: Some(IconKind::Wifi), "Posso usare la mesh offline?" }
    AccordionPanel { "Sì: i nodi vicini si sincronizzano via Bluetooth e Wi-Fi diretto finché la rete torna." }
}
AccordionItem { value: "chiavi".to_string(),
    AccordionControl { icon: Some(IconKind::Key), "Ogni quanto ruotano le chiavi?" }
    AccordionPanel { "Ogni 24 ore, oppure subito dopo la revoca di un dispositivo." }
}"#;

fn render(k: &Knobs) -> Element {
    rsx! {
        Accordion {
            default_value: Some(vec!["sicurezza".to_string()]),
            variant: k.pick("variant", &VARIANTS),
            multiple: k.bool("multiple"),
            AccordionItem { value: "sicurezza".to_string(),
                AccordionControl { icon: Some(IconKind::Shield), "Come vengono cifrati i dati?" }
                AccordionPanel { "Ogni sessione negozia una chiave Kyber-1024; i dati a riposo usano AES-256-GCM." }
            }
            AccordionItem { value: "rete".to_string(),
                AccordionControl { icon: Some(IconKind::Wifi), "Posso usare la mesh offline?" }
                AccordionPanel { "Sì: i nodi vicini si sincronizzano via Bluetooth e Wi-Fi diretto finché la rete torna." }
            }
            AccordionItem { value: "chiavi".to_string(),
                AccordionControl { icon: Some(IconKind::Key), "Ogni quanto ruotano le chiavi?" }
                AccordionPanel { "Ogni 24 ore, oppure subito dopo la revoca di un dispositivo." }
            }
        }
    }
}

#[component]
fn Controlled() -> Element {
    let mut open = use_signal(|| vec!["piano".to_string()]);
    rsx! {
        Stack { gap: SpacingToken::Sm,
            Group { gap: SpacingToken::Xs,
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| open.set(vec!["piano".to_string(), "fattura".to_string()]),
                    "Espandi tutto"
                }
                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| open.set(Vec::new()), "Chiudi tutto" }
            }
            Accordion { value: Some(open), multiple: true, variant: AccordionVariant::Contained,
                AccordionItem { value: "piano".to_string(),
                    AccordionControl { "Piano Team" }
                    AccordionPanel { "12 postazioni, 3 regioni, supporto entro 4 ore." }
                }
                AccordionItem { value: "fattura".to_string(),
                    AccordionControl { "Fatturazione" }
                    AccordionPanel { "Mensile, prossimo addebito il 1° novembre." }
                }
            }
        }
    }
}
const CONTROLLED_CODE: &str = r#"let mut open = use_signal(|| vec!["piano".to_string()]);

Stack { gap: SpacingToken::Sm,
    Group { gap: SpacingToken::Xs,
        Button {
            variant: ButtonVariant::Ghost,
            size: ButtonSize::Sm,
            onclick: move |_| open.set(vec!["piano".to_string(), "fattura".to_string()]),
            "Espandi tutto"
        }
        Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, onclick: move |_| open.set(Vec::new()), "Chiudi tutto" }
    }
    Accordion { value: Some(open), multiple: true, variant: AccordionVariant::Contained,
        AccordionItem { value: "piano".to_string(),
            AccordionControl { "Piano Team" }
            AccordionPanel { "12 postazioni, 3 regioni, supporto entro 4 ore." }
        }
        AccordionItem { value: "fattura".to_string(),
            AccordionControl { "Fatturazione" }
            AccordionPanel { "Mensile, prossimo addebito il 1° novembre." }
        }
    }
}"#;

#[component]
fn Disabled() -> Element {
    rsx! {
        Accordion { variant: AccordionVariant::Separated,
            AccordionItem { value: "base".to_string(),
                AccordionControl { "Impostazioni di base" }
                AccordionPanel { "Nome del nodo, regione e fuso orario." }
            }
            AccordionItem { value: "avanzate".to_string(), disabled: true,
                AccordionControl { "Impostazioni avanzate (solo admin)" }
                AccordionPanel { "Parametri del kernel di rete." }
            }
        }
    }
}
const DISABLED_CODE: &str = r#"Accordion { variant: AccordionVariant::Separated,
    AccordionItem { value: "base".to_string(),
        AccordionControl { "Impostazioni di base" }
        AccordionPanel { "Nome del nodo, regione e fuso orario." }
    }
    AccordionItem { value: "avanzate".to_string(), disabled: true,
        AccordionControl { "Impostazioni avanzate (solo admin)" }
        AccordionPanel { "Parametri del kernel di rete." }
    }
}"#;

#[component]
fn CollapsibleDemo() -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Stack { gap: SpacingToken::Sm,
            Button {
                variant: ButtonVariant::Secondary,
                size: ButtonSize::Sm,
                aria_pressed: Some(open()),
                right_icon: Some(rsx! {
                    Icon { kind: if open() { IconKind::ChevronUp } else { IconKind::ChevronDown }, size: 14 }
                }),
                onclick: move |_| open.toggle(),
                "Dettagli tecnici"
            }
            Collapsible { open,
                Text { size: TextSize::Sm, dimmed: true, "Build 0.5.2 · rustc 1.83 · wasm32-unknown-unknown · 412 KB gzip." }
            }
        }
    }
}
const COLLAPSIBLE_CODE: &str = r#"let mut open = use_signal(|| false);

Stack { gap: SpacingToken::Sm,
    Button {
        variant: ButtonVariant::Secondary,
        size: ButtonSize::Sm,
        aria_pressed: Some(open()),
        right_icon: Some(rsx! {
            Icon { kind: if open() { IconKind::ChevronUp } else { IconKind::ChevronDown }, size: 14 }
        }),
        onclick: move |_| open.toggle(),
        "Dettagli tecnici"
    }
    Collapsible { open,
        Text { size: TextSize::Sm, dimmed: true, "Build 0.5.2 · rustc 1.83 · wasm32-unknown-unknown · 412 KB gzip." }
    }
}"#;

#[component]
fn InContext() -> Element {
    let alerts = use_signal(|| true);
    let digest = use_signal(|| false);
    let name = use_signal(|| "edge-07".to_string());
    rsx! {
        Card { title: Some("Impostazioni del nodo".to_string()), subtitle: Some("edge-07 · eu-central".to_string()),
            Accordion { default_value: Some(vec!["generale".to_string()]), variant: AccordionVariant::Contained,
                AccordionItem { value: "generale".to_string(),
                    AccordionControl { icon: Some(IconKind::Server), "Generale" }
                    AccordionPanel {
                        TextInput { value: name, label: Some("Nome host".to_string()) }
                    }
                }
                AccordionItem { value: "notifiche".to_string(),
                    AccordionControl { icon: Some(IconKind::MessageSquare), "Notifiche" }
                    AccordionPanel {
                        Stack { gap: SpacingToken::Sm,
                            Switch { checked: alerts, label: Some("Allarmi in tempo reale".to_string()) }
                            Switch { checked: digest, label: Some("Riepilogo giornaliero".to_string()) }
                        }
                    }
                }
                AccordionItem { value: "sicurezza".to_string(),
                    AccordionControl { icon: Some(IconKind::Lock), "Sicurezza" }
                    AccordionPanel {
                        Alert { variant: AlertVariant::Warning, title: Some("Chiavi in scadenza".to_string()),
                            "La chiave di firma scade tra 3 giorni."
                        }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let alerts = use_signal(|| true);
let digest = use_signal(|| false);
let name = use_signal(|| "edge-07".to_string());

Card { title: Some("Impostazioni del nodo".to_string()), subtitle: Some("edge-07 · eu-central".to_string()),
    Accordion { default_value: Some(vec!["generale".to_string()]), variant: AccordionVariant::Contained,
        AccordionItem { value: "generale".to_string(),
            AccordionControl { icon: Some(IconKind::Server), "Generale" }
            AccordionPanel {
                TextInput { value: name, label: Some("Nome host".to_string()) }
            }
        }
        AccordionItem { value: "notifiche".to_string(),
            AccordionControl { icon: Some(IconKind::MessageSquare), "Notifiche" }
            AccordionPanel {
                Stack { gap: SpacingToken::Sm,
                    Switch { checked: alerts, label: Some("Allarmi in tempo reale".to_string()) }
                    Switch { checked: digest, label: Some("Riepilogo giornaliero".to_string()) }
                }
            }
        }
        AccordionItem { value: "sicurezza".to_string(),
            AccordionControl { icon: Some(IconKind::Lock), "Sicurezza" }
            AccordionPanel {
                Alert { variant: AlertVariant::Warning, title: Some("Chiavi in scadenza".to_string()),
                    "La chiave di firma scade tra 3 giorni."
                }
            }
        }
    }
}"#;

const ACCORDION_PROPS: &[PropDoc] = &[
    prop("variant", "AccordionVariant", "Default", "Default (separatori), Contained (un solo riquadro), Separated (card distinte)."),
    prop("multiple", "bool", "false", "Più pannelli aperti insieme."),
    prop("value", "Option<Signal<Vec<String>>>", "None", "Pannelli aperti, controllati dall'esterno."),
    prop("default_value", "Option<Vec<String>>", "None", "Pannelli aperti all'avvio (non controllato)."),
    prop("id / class", "Option<String> / String", "None / \"\"", "Id base per gli id ARIA e classi aggiuntive."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("value", "String", "-", "Chiave univoca della voce."),
    prop("disabled", "bool", "false", "Voce non apribile, saltata dalle frecce."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const CONTROL_PROPS: &[PropDoc] = &[
    prop("icon", "Option<IconKind>", "None", "Icona prima dell'etichetta."),
    prop("chevron", "Option<Element>", "None", "Sostituisce la freccia di apertura."),
    prop("disabled", "bool", "false", "Disabilita il solo controllo."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const PANEL_PROPS: &[PropDoc] = &[
    prop("class", "String", "\"\"", "Classi aggiuntive sulla regione."),
    prop("children", "Element", "-", "Contenuto del pannello."),
];
const COLLAPSIBLE_PROPS: &[PropDoc] = &[
    prop("open", "Signal<bool>", "-", "Apre e chiude il contenuto con una transizione di altezza."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "accordion",
        name: "Accordion",
        category: Category::Layout,
        features: &["accordion"],
        icon: IconKind::ChevronDown,
        tagline: "Sezioni che si aprono e si chiudono per mostrare un argomento alla volta.",
        keywords: &["accordion", "fisarmonica", "faq", "collapsible", "espandi", "sezioni"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Accordion",
            controls: vec![
                Control::choice("variant", "Variante", VARIANT_OPTS, 0),
                Control::bool("multiple", "Più pannelli aperti", false),
            ],
            render,
            prelude_code: None,
            fixed_code: &["default_value: Some(vec![\"sicurezza\".to_string()]),"],
            children_code: Some(CHILDREN),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "controlled",
                title: "Controllato",
                description: "Un Signal apre e chiude tutto dall'esterno.",
                code: CONTROLLED_CODE,
                render: || rsx! { Controlled {} },
                span: Span::Half,
            },
            Example {
                id: "disabled",
                title: "Voce disabilitata",
                description: "Separated, con una sezione riservata.",
                code: DISABLED_CODE,
                render: || rsx! { Disabled {} },
                span: Span::Half,
            },
            Example {
                id: "collapsible",
                title: "Collapsible",
                description: "La stessa animazione, per un solo blocco.",
                code: COLLAPSIBLE_CODE,
                render: || rsx! { CollapsibleDemo {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Impostazioni raggruppate per argomento in una card.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Accordion", rows: ACCORDION_PROPS },
            PropGroup { component: "AccordionItem", rows: ITEM_PROPS },
            PropGroup { component: "AccordionControl", rows: CONTROL_PROPS },
            PropGroup { component: "AccordionPanel", rows: PANEL_PROPS },
            PropGroup { component: "Collapsible", rows: COLLAPSIBLE_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Container sd-surface(1) (§3.1) con controlli item (§3.4)",
            depth: "Flat-lit",
            tokens: &["--sd-surface-1", "--sd-border", "--sd-border-subtle", "--sd-tint-1", "--sd-radius-lg", "--sd-text-muted", "--sd-dur"],
            notes: "Il controllo aperto non si accende di accento: cambia solo la freccia e il colore del testo.",
        }),
        a11y: &[
            "Ogni controllo è un <button> con aria-expanded e aria-controls verso il proprio pannello.",
            "Il pannello ha role=region, aria-labelledby verso il controllo, e inert quando è chiuso.",
            "Frecce su e giù, Home e End spostano il focus tra i controlli abilitati.",
            "Le voci disabilitate usano disabled e aria-disabled.",
        ],
        related: &["spoiler", "tabs", "cards", "fieldset"],
    }
}
