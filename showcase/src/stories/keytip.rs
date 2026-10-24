//! Story: Keytip (feature `keytip`). Stateless badge over a control: the playground renders it
//! straight from the knobs.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const CHILDREN_CODE: &str = r#"Button { variant: ButtonVariant::Secondary, "{label}" }"#;

fn render(k: &Knobs) -> Element {
    rsx! {
        Keytip {
            key_sequence: k.text("key_sequence"),
            visible: k.bool("hidden").then_some(false),
            disabled: k.bool("disabled"),
            Button { variant: ButtonVariant::Secondary, "{k.text(\"label\")}" }
        }
    }
}

#[component]
fn Toolbar() -> Element {
    let keyboard = use_signal(|| true);
    rsx! {
        Stack { gap: SpacingToken::Lg,
            Group { gap: SpacingToken::Md,
                Keytip { key_sequence: "F".to_string(), visible: Some(keyboard()),
                    Button { variant: ButtonVariant::Ghost, "File" }
                }
                Keytip { key_sequence: "M".to_string(), visible: Some(keyboard()),
                    Button { variant: ButtonVariant::Ghost, "Modifica" }
                }
                Keytip { key_sequence: "V".to_string(), visible: Some(keyboard()),
                    Button { variant: ButtonVariant::Ghost, "Vista" }
                }
                Keytip { key_sequence: "Ctrl+S".to_string(), visible: Some(keyboard()),
                    Button { "Salva" }
                }
            }
            Switch { checked: keyboard, label: Some("Modalità tastiera".to_string()), description: Some("Di solito si attiva tenendo premuto Alt".to_string()) }
        }
    }
}
const TOOLBAR_CODE: &str = r#"let keyboard = use_signal(|| true);

Stack { gap: SpacingToken::Lg,
    Group { gap: SpacingToken::Md,
        Keytip { key_sequence: "F".to_string(), visible: Some(keyboard()),
            Button { variant: ButtonVariant::Ghost, "File" }
        }
        Keytip { key_sequence: "M".to_string(), visible: Some(keyboard()),
            Button { variant: ButtonVariant::Ghost, "Modifica" }
        }
        Keytip { key_sequence: "V".to_string(), visible: Some(keyboard()),
            Button { variant: ButtonVariant::Ghost, "Vista" }
        }
        Keytip { key_sequence: "Ctrl+S".to_string(), visible: Some(keyboard()),
            Button { "Salva" }
        }
    }
    Switch { checked: keyboard, label: Some("Modalità tastiera".to_string()), description: Some("Di solito si attiva tenendo premuto Alt".to_string()) }
}"#;

#[component]
fn Disabled() -> Element {
    rsx! {
        Group { gap: SpacingToken::Md, justify: JustifyContent::Center,
            Keytip { key_sequence: "Ctrl+Z".to_string(),
                Button { variant: ButtonVariant::Secondary, "Annulla" }
            }
            Keytip { key_sequence: "Ctrl+Y".to_string(), disabled: true,
                Button { variant: ButtonVariant::Secondary, disabled: true, "Ripeti" }
            }
        }
    }
}
const DISABLED_CODE: &str = r#"Group { gap: SpacingToken::Md, justify: JustifyContent::Center,
    Keytip { key_sequence: "Ctrl+Z".to_string(),
        Button { variant: ButtonVariant::Secondary, "Annulla" }
    }
    Keytip { key_sequence: "Ctrl+Y".to_string(), disabled: true,
        Button { variant: ButtonVariant::Secondary, disabled: true, "Ripeti" }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("key_sequence", "String", "-", "Tasti da mostrare nel badge (F, Ctrl+S, Alt+M...)."),
    prop("visible", "Option<bool>", "None", "Some(false) nasconde il badge; None lo mostra."),
    prop("disabled", "bool", "false", "Nasconde il badge, per esempio quando il comando non è disponibile."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul wrapper."),
    prop("children", "Element", "-", "Il controllo a cui si riferisce la scorciatoia."),
];

pub fn story() -> Story {
    Story {
        slug: "keytip",
        name: "Keytip",
        category: Category::Overlay,
        features: &["keytip"],
        icon: IconKind::Terminal,
        tagline: "Badge con la scorciatoia da tastiera sopra il comando a cui si riferisce.",
        keywords: &["keytip", "scorciatoia", "shortcut", "tastiera", "keyboard", "accesskey"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Keytip",
            controls: vec![
                Control::text("key_sequence", "Tasti", "Ctrl+S").emit(Emit::Template("key_sequence: \"{v}\".to_string()")),
                Control::text("label", "Comando", "Salva").emit(Emit::Skip),
                Control::bool("hidden", "Nascosto", false).emit(Emit::Template("visible: Some(false)")),
                Control::bool("disabled", "Disabilitato", false),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some(CHILDREN_CODE),
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "toolbar",
                title: "Modalità tastiera",
                description: "I keytip compaiono tutti insieme quando l'utente chiede le scorciatoie.",
                code: TOOLBAR_CODE,
                render: || rsx! { Toolbar {} },
                span: Span::Half,
            },
            Example {
                id: "disabled",
                title: "Comando non disponibile",
                description: "disabled nasconde il badge insieme al comando.",
                code: DISABLED_CODE,
                render: || rsx! { Disabled {} },
                span: Span::Half,
            },
        ],
        props: vec![PropGroup { component: "Keytip", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Floating panel in miniatura (language.md §3.2) + testo mono (§3.12)",
            depth: "Raised",
            tokens: &["--sd-surface-2", "--sd-border-strong", "--sd-font-mono", "--sd-text-2xs", "--sd-radius-xs", "--sd-dur-fast"],
            notes: "Badge mono sovrapposto all'angolo del comando, entra con sd-zoom-in e non reagisce al puntatore.",
        }),
        a11y: &[
            "Il badge è solo visivo: non registra il tasto. Collega la scorciatoia con use_hotkey o con un gestore onkeydown.",
            "Il testo del badge è letto dagli screen reader dopo il controllo; ripeti la scorciatoia nell'etichetta se è importante.",
            "Il controllo avvolto resta invariato: focus, ruolo e nome accessibile non cambiano.",
        ],
        related: &["tooltip", "menubar", "command-palette", "reactive-hooks"],
    }
}
