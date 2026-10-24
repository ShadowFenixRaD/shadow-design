//! Story: ResizablePanelGroup, ResizablePanel, ResizableHandle (feature `resizable`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const DIRECTIONS: [ResizableDirection; 2] = [ResizableDirection::Horizontal, ResizableDirection::Vertical];
const DIRECTION_OPTS: &[ChoiceOption] =
    &[opt("Orizzontale", "ResizableDirection::Horizontal"), opt("Verticale", "ResizableDirection::Vertical")];

const CHILDREN: &str = r#"ResizablePanel { default_size: 35.0, min_size: 20.0, max_size: 70.0,
    List { spacing: ListSpacing::Sm,
        ListItem { "main.rs" }
        ListItem { "theme.rs" }
        ListItem { "routes.rs" }
    }
}
ResizableHandle { with_handle: {with_handle}, disabled: {disabled} }
ResizablePanel { default_size: 65.0, min_size: 30.0,
    CodeBlock { code: "fn main() {\n    dioxus::launch(App);\n}".to_string(), with_line_numbers: false }
}"#;

fn render(k: &Knobs) -> Element {
    rsx! {
        ResizablePanelGroup { direction: k.pick("direction", &DIRECTIONS),
            ResizablePanel { default_size: 35.0, min_size: 20.0, max_size: 70.0,
                List { spacing: ListSpacing::Sm,
                    ListItem { "main.rs" }
                    ListItem { "theme.rs" }
                    ListItem { "routes.rs" }
                }
            }
            ResizableHandle { with_handle: k.bool("with_handle"), disabled: k.bool("disabled") }
            ResizablePanel { default_size: 65.0, min_size: 30.0,
                CodeBlock { code: "fn main() {\n    dioxus::launch(App);\n}".to_string(), with_line_numbers: false }
            }
        }
    }
}

#[component]
fn Vertical() -> Element {
    rsx! {
        div { class: "sc-demo-frame h-64",
            ResizablePanelGroup { direction: ResizableDirection::Vertical,
                ResizablePanel { default_size: 60.0,
                    Text { size: TextSize::Sm, "Anteprima" }
                }
                ResizableHandle {}
                ResizablePanel { default_size: 40.0,
                    Text { size: TextSize::Sm, dimmed: true, "Console" }
                }
            }
        }
    }
}
const VERTICAL_CODE: &str = r#"div { class: "h-64",
    ResizablePanelGroup { direction: ResizableDirection::Vertical,
        ResizablePanel { default_size: 60.0,
            Text { size: TextSize::Sm, "Anteprima" }
        }
        ResizableHandle {}
        ResizablePanel { default_size: 40.0,
            Text { size: TextSize::Sm, dimmed: true, "Console" }
        }
    }
}"#;

#[component]
fn ThreePanels() -> Element {
    rsx! {
        div { class: "sc-demo-frame h-48",
            ResizablePanelGroup {
                ResizablePanel { default_size: 25.0, min_size: 15.0,
                    Text { size: TextSize::Sm, "Cartelle" }
                }
                ResizableHandle { with_handle: false }
                ResizablePanel { default_size: 45.0,
                    Text { size: TextSize::Sm, "Messaggi" }
                }
                ResizableHandle { with_handle: false }
                ResizablePanel { default_size: 30.0, min_size: 20.0,
                    Text { size: TextSize::Sm, "Lettura" }
                }
            }
        }
    }
}
const THREE_PANELS_CODE: &str = r#"div { class: "h-48",
    ResizablePanelGroup {
        ResizablePanel { default_size: 25.0, min_size: 15.0,
            Text { size: TextSize::Sm, "Cartelle" }
        }
        ResizableHandle { with_handle: false }
        ResizablePanel { default_size: 45.0,
            Text { size: TextSize::Sm, "Messaggi" }
        }
        ResizableHandle { with_handle: false }
        ResizablePanel { default_size: 30.0, min_size: 20.0,
            Text { size: TextSize::Sm, "Lettura" }
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        Card { title: Some("Editor di configurazione".to_string()), subtitle: Some("mesh.toml · edge-07".to_string()),
            div { class: "h-80",
                ResizablePanelGroup {
                    ResizablePanel { default_size: 30.0, min_size: 20.0, max_size: 50.0,
                        Stack { gap: SpacingToken::Sm,
                            span { class: "sc-eyebrow", "File" }
                            List { spacing: ListSpacing::Sm,
                                ListItem { icon: Some(rsx! { Icon { kind: IconKind::Code, size: 14 } }), "mesh.toml" }
                                ListItem { icon: Some(rsx! { Icon { kind: IconKind::Key, size: 14 } }), "keys.toml" }
                                ListItem { icon: Some(rsx! { Icon { kind: IconKind::Shield, size: 14 } }), "policy.toml" }
                            }
                        }
                    }
                    ResizableHandle {}
                    ResizablePanel { default_size: 70.0, min_size: 40.0,
                        CodeBlock {
                            code: "[node]\nname = \"edge-07\"\nregion = \"eu-central\"\n\n[crypto]\nkem = \"kyber-1024\"\nrotate_hours = 24".to_string(),
                            language: "toml",
                            filename: Some("mesh.toml".to_string()),
                        }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"Card { title: Some("Editor di configurazione".to_string()), subtitle: Some("mesh.toml · edge-07".to_string()),
    div { class: "h-80",
        ResizablePanelGroup {
            ResizablePanel { default_size: 30.0, min_size: 20.0, max_size: 50.0,
                Stack { gap: SpacingToken::Sm,
                    span { "File" }
                    List { spacing: ListSpacing::Sm,
                        ListItem { icon: Some(rsx! { Icon { kind: IconKind::Code, size: 14 } }), "mesh.toml" }
                        ListItem { icon: Some(rsx! { Icon { kind: IconKind::Key, size: 14 } }), "keys.toml" }
                        ListItem { icon: Some(rsx! { Icon { kind: IconKind::Shield, size: 14 } }), "policy.toml" }
                    }
                }
            }
            ResizableHandle {}
            ResizablePanel { default_size: 70.0, min_size: 40.0,
                CodeBlock {
                    code: "[node]\nname = \"edge-07\"\nregion = \"eu-central\"\n\n[crypto]\nkem = \"kyber-1024\"\nrotate_hours = 24".to_string(),
                    language: "toml",
                    filename: Some("mesh.toml".to_string()),
                }
            }
        }
    }
}"#;

const GROUP_PROPS: &[PropDoc] = &[
    prop("direction", "ResizableDirection", "Horizontal", "Horizontal (pannelli affiancati) o Vertical (in colonna)."),
    prop("on_resize", "Option<EventHandler<f64>>", "None", "Percentuale del primo pannello durante il trascinamento."),
    prop("class", "String", "\"\"", "Classi aggiuntive; il gruppo riempie l'altezza del genitore."),
    prop("children", "Element", "-", "ResizablePanel alternati a ResizableHandle."),
];
const PANEL_PROPS: &[PropDoc] = &[
    prop("default_size", "f64", "50.0", "Dimensione iniziale in percentuale."),
    prop("min_size", "f64", "10.0", "Percentuale minima durante il trascinamento."),
    prop("max_size", "f64", "90.0", "Percentuale massima durante il trascinamento."),
    prop("class", "String", "\"\"", "Classi aggiuntive; il pannello scorre se il contenuto eccede."),
];
const HANDLE_PROPS: &[PropDoc] = &[
    prop("with_handle", "bool", "true", "Mostra la presa con l'icona GripVertical."),
    prop("disabled", "bool", "false", "Blocca il trascinamento e toglie la maniglia dal Tab."),
    prop("on_drag", "Option<EventHandler<f64>>", "None", "Callback di trascinamento della singola maniglia."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "resizable",
        name: "ResizablePanelGroup",
        category: Category::Layout,
        features: &["resizable"],
        icon: IconKind::GripVertical,
        tagline: "Pannelli affiancati o impilati che l'utente ridimensiona trascinando la maniglia.",
        keywords: &["resizable", "ridimensionabile", "split", "pannelli", "splitter", "panes"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "ResizablePanelGroup",
            controls: vec![
                Control::choice("direction", "Direzione", DIRECTION_OPTS, 0),
                Control::bool("with_handle", "Presa visibile", true).emit(Emit::Skip),
                Control::bool("disabled", "Maniglia bloccata", false).emit(Emit::Skip),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some(CHILDREN),
            stage: Stage::Fill,
        }),
        examples: vec![
            Example {
                id: "vertical",
                title: "Verticale",
                description: "Anteprima sopra, console sotto.",
                code: VERTICAL_CODE,
                render: || rsx! { Vertical {} },
                span: Span::Half,
            },
            Example {
                id: "three",
                title: "Tre pannelli",
                description: "Due maniglie sottili senza presa.",
                code: THREE_PANELS_CODE,
                render: || rsx! { ThreePanels {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Albero dei file e sorgente in un editor di configurazione.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "ResizablePanelGroup", rows: GROUP_PROPS },
            PropGroup { component: "ResizablePanel", rows: PANEL_PROPS },
            PropGroup { component: "ResizableHandle", rows: HANDLE_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Container (language.md §3.1) con maniglia raised control (§3.5)",
            depth: "Flat-lit (pannelli) · Lit control (presa)",
            tokens: &["--sd-border", "--sd-border-strong", "--sd-accent", "--sd-surface-2", "--sd-radius-sm", "--sd-space-6"],
            notes: "La linea della maniglia diventa accento solo durante il trascinamento; i pannelli restano statici.",
        }),
        a11y: &[
            "Ogni maniglia ha role=separator ed è raggiungibile con Tab (tabindex 0, -1 se disabled).",
            "disabled imposta aria-disabled=true e blocca il trascinamento.",
            "Il ridimensionamento usa i pointer events: funziona con mouse, touch e penna.",
            "Le frecce da tastiera non ridimensionano ancora: offri un'alternativa (preset di layout) se serve.",
        ],
        related: &["app-shell", "scroll-area", "grid", "trees"],
    }
}
