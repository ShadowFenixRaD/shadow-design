//! Story: Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem, CommandBar
//! (feature `menubar`). Stateful playground (`MenubarDemo { knobs }`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const PRELUDE: &str = r#"let mut toast = use_notification();
let mut terminal = use_signal(|| true);"#;

const CHILDREN: &str = r#"MenubarMenu { id: Some("file".to_string()),
    MenubarTrigger { "File" }
    MenubarContent {
        MenubarItem {
            label: Some("Nuovo nodo".to_string()),
            icon: Some(SvgIcon::kind(IconKind::Plus)),
            shortcut: Some("Ctrl+N".to_string()),
            on_click: move |_| { toast.info("Nuovo nodo"); },
        }
        MenubarItem { label: Some("Apri vault".to_string()), icon: Some(SvgIcon::kind(IconKind::Lock)), shortcut: Some("Ctrl+O".to_string()) }
        MenubarSeparator {}
        MenubarItem { label: Some("Esporta certificato".to_string()), icon: Some(SvgIcon::kind(IconKind::Download)) }
    }
}
MenubarMenu { id: Some("edit".to_string()),
    MenubarTrigger { "Modifica" }
    MenubarContent {
        MenubarItem { label: Some("Annulla".to_string()), shortcut: Some("Ctrl+Z".to_string()) }
        MenubarItem { label: Some("Ripeti".to_string()), shortcut: Some("Ctrl+Y".to_string()), disabled: true }
    }
}
MenubarMenu { id: Some("view".to_string()),
    MenubarTrigger { "Vista" }
    MenubarContent {
        MenubarItem {
            label: Some("Terminale".to_string()),
            checked: Some(terminal()),
            on_click: move |_| terminal.toggle(),
        }
    }
}
MenubarMenu { id: Some("help".to_string()),
    MenubarTrigger { disabled: {help_disabled}, "Aiuto" }
    MenubarContent {
        MenubarItem { label: Some("Documentazione".to_string()), icon: Some(SvgIcon::kind(IconKind::ExternalLink)) }
    }
}"#;

#[component]
fn MenubarDemo(knobs: Knobs) -> Element {
    let mut toast = use_notification();
    let mut terminal = use_signal(|| true);
    rsx! {
        Menubar { border: knobs.bool("border"),
            MenubarMenu { id: Some("file".to_string()),
                MenubarTrigger { "File" }
                MenubarContent {
                    MenubarItem {
                        label: Some("Nuovo nodo".to_string()),
                        icon: Some(SvgIcon::kind(IconKind::Plus)),
                        shortcut: Some("Ctrl+N".to_string()),
                        on_click: move |_| {
                            toast.info("Nuovo nodo");
                        },
                    }
                    MenubarItem { label: Some("Apri vault".to_string()), icon: Some(SvgIcon::kind(IconKind::Lock)), shortcut: Some("Ctrl+O".to_string()) }
                    MenubarSeparator {}
                    MenubarItem { label: Some("Esporta certificato".to_string()), icon: Some(SvgIcon::kind(IconKind::Download)) }
                }
            }
            MenubarMenu { id: Some("edit".to_string()),
                MenubarTrigger { "Modifica" }
                MenubarContent {
                    MenubarItem { label: Some("Annulla".to_string()), shortcut: Some("Ctrl+Z".to_string()) }
                    MenubarItem { label: Some("Ripeti".to_string()), shortcut: Some("Ctrl+Y".to_string()), disabled: true }
                }
            }
            MenubarMenu { id: Some("view".to_string()),
                MenubarTrigger { "Vista" }
                MenubarContent {
                    MenubarItem {
                        label: Some("Terminale".to_string()),
                        checked: Some(terminal()),
                        on_click: move |_| terminal.toggle(),
                    }
                }
            }
            MenubarMenu { id: Some("help".to_string()),
                MenubarTrigger { disabled: knobs.bool("help_disabled"), "Aiuto" }
                MenubarContent {
                    MenubarItem { label: Some("Documentazione".to_string()), icon: Some(SvgIcon::kind(IconKind::ExternalLink)) }
                }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        MenubarDemo { knobs: k.clone() }
    }
}

#[component]
fn Checkable() -> Element {
    let mut grid = use_signal(|| true);
    let mut rulers = use_signal(|| false);
    rsx! {
        Menubar {
            MenubarMenu { id: Some("view".to_string()),
                MenubarTrigger { "Vista" }
                MenubarContent {
                    MenubarItem { label: Some("Griglia".to_string()), checked: Some(grid()), on_click: move |_| grid.toggle() }
                    MenubarItem { label: Some("Righelli".to_string()), checked: Some(rulers()), on_click: move |_| rulers.toggle() }
                }
            }
        }
    }
}
const CHECKABLE_CODE: &str = r#"let mut grid = use_signal(|| true);
let mut rulers = use_signal(|| false);

Menubar {
    MenubarMenu { id: Some("view".to_string()),
        MenubarTrigger { "Vista" }
        MenubarContent {
            MenubarItem { label: Some("Griglia".to_string()), checked: Some(grid()), on_click: move |_| grid.toggle() }
            MenubarItem { label: Some("Righelli".to_string()), checked: Some(rulers()), on_click: move |_| rulers.toggle() }
        }
    }
}"#;

#[component]
fn CommandBarDemo() -> Element {
    rsx! {
        CommandBar {
            menubar: Some(rsx! {
                Menubar { border: false,
                    MenubarMenu { id: Some("file".to_string()),
                        MenubarTrigger { "File" }
                        MenubarContent {
                            MenubarItem { label: Some("Salva".to_string()), shortcut: Some("Ctrl+S".to_string()) }
                        }
                    }
                }
            }),
            action: Some(rsx! {
                Button { size: ButtonSize::Sm, "Condividi" }
            }),
        }
    }
}
const COMMAND_BAR_CODE: &str = r#"CommandBar {
    menubar: Some(rsx! {
        Menubar { border: false,
            MenubarMenu { id: Some("file".to_string()),
                MenubarTrigger { "File" }
                MenubarContent {
                    MenubarItem { label: Some("Salva".to_string()), shortcut: Some("Ctrl+S".to_string()) }
                }
            }
        }
    }),
    action: Some(rsx! {
        Button { size: ButtonSize::Sm, "Condividi" }
    }),
}"#;

#[component]
fn InContext() -> Element {
    let mut toast = use_notification();
    let mut numbers = use_signal(|| true);
    rsx! {
        Card { title: Some("mesh.toml".to_string()), subtitle: Some("edge-07 · modificato".to_string()),
            Stack { gap: SpacingToken::Sm,
                CommandBar {
                    menubar: Some(rsx! {
                        Menubar {
                            MenubarMenu { id: Some("file".to_string()),
                                MenubarTrigger { "File" }
                                MenubarContent {
                                    MenubarItem {
                                        label: Some("Salva".to_string()),
                                        shortcut: Some("Ctrl+S".to_string()),
                                        on_click: move |_| {
                                            toast.success("mesh.toml salvato");
                                        },
                                    }
                                    MenubarItem { label: Some("Ripristina".to_string()), icon: Some(SvgIcon::kind(IconKind::Clock)) }
                                }
                            }
                            MenubarMenu { id: Some("view".to_string()),
                                MenubarTrigger { "Vista" }
                                MenubarContent {
                                    MenubarItem {
                                        label: Some("Numeri di riga".to_string()),
                                        checked: Some(numbers()),
                                        on_click: move |_| numbers.toggle(),
                                    }
                                }
                            }
                        }
                    }),
                    action: Some(rsx! {
                        Button {
                            size: ButtonSize::Sm,
                            left_icon: Some(rsx! {
                                Icon { kind: IconKind::Upload, size: 14 }
                            }),
                            onclick: move |_| {
                                toast.success("Configurazione distribuita");
                            },
                            "Distribuisci"
                        }
                    }),
                }
                CodeBlock {
                    code: "[node]\nname = \"edge-07\"\nregion = \"eu-central\"\n\n[crypto]\nkem = \"kyber-1024\"".to_string(),
                    language: "toml",
                    with_line_numbers: numbers(),
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut toast = use_notification();
let mut numbers = use_signal(|| true);

Card { title: Some("mesh.toml".to_string()), subtitle: Some("edge-07 · modificato".to_string()),
    Stack { gap: SpacingToken::Sm,
        CommandBar {
            menubar: Some(rsx! {
                Menubar {
                    MenubarMenu { id: Some("file".to_string()),
                        MenubarTrigger { "File" }
                        MenubarContent {
                            MenubarItem {
                                label: Some("Salva".to_string()),
                                shortcut: Some("Ctrl+S".to_string()),
                                on_click: move |_| {
                                    toast.success("mesh.toml salvato");
                                },
                            }
                            MenubarItem { label: Some("Ripristina".to_string()), icon: Some(SvgIcon::kind(IconKind::Clock)) }
                        }
                    }
                    MenubarMenu { id: Some("view".to_string()),
                        MenubarTrigger { "Vista" }
                        MenubarContent {
                            MenubarItem {
                                label: Some("Numeri di riga".to_string()),
                                checked: Some(numbers()),
                                on_click: move |_| numbers.toggle(),
                            }
                        }
                    }
                }
            }),
            action: Some(rsx! {
                Button {
                    size: ButtonSize::Sm,
                    left_icon: Some(rsx! {
                        Icon { kind: IconKind::Upload, size: 14 }
                    }),
                    onclick: move |_| {
                        toast.success("Configurazione distribuita");
                    },
                    "Distribuisci"
                }
            }),
        }
        CodeBlock {
            code: "[node]\nname = \"edge-07\"\nregion = \"eu-central\"\n\n[crypto]\nkem = \"kyber-1024\"".to_string(),
            language: "toml",
            with_line_numbers: numbers(),
        }
    }
}"#;

const MENUBAR_PROPS: &[PropDoc] = &[
    prop("active_menu", "Option<Signal<Option<String>>>", "None", "Menu aperto (id), controllato dall'esterno."),
    prop("border", "bool", "true", "Bordo e superficie della barra."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
    prop("children", "Element", "-", "MenubarMenu in ordine."),
];
const MENU_PROPS: &[PropDoc] = &[
    prop("MenubarMenu.id", "Option<String>", "None", "Id del menu, usato da active_menu."),
    prop("MenubarTrigger.disabled", "bool", "false", "Trigger non apribile."),
    prop("MenubarContent.class", "String", "\"\"", "Classi del pannello a discesa."),
    prop("MenubarSeparator", "-", "-", "Divisore tra gruppi di voci."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("label", "Option<String>", "None", "Etichetta; in alternativa i children."),
    prop("icon", "Option<SvgIcon>", "None", "Icona prima dell'etichetta."),
    prop("shortcut", "Option<String>", "None", "Scorciatoia allineata a destra (MenubarShortcut)."),
    prop("checked", "Option<bool>", "None", "Rende la voce menuitemcheckbox con spunta."),
    prop("disabled", "bool", "false", "Voce non attivabile."),
    prop("on_click", "Option<EventHandler<MouseEvent>>", "None", "Click; il menu si chiude."),
    prop("class / children", "String / Option<Element>", "\"\" / None", "Classi e contenuto personalizzato."),
];
const BAR_PROPS: &[PropDoc] = &[
    prop("menubar", "Option<Element>", "None", "La Menubar a sinistra."),
    prop("action", "Option<Element>", "None", "Azione principale a destra."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "menubar",
        name: "Menubar",
        category: Category::Navigazione,
        features: &["menubar"],
        icon: IconKind::Menu,
        tagline: "Barra dei menu in stile desktop, con scorciatoie, voci a spunta e separatori.",
        keywords: &["menubar", "barra dei menu", "menu", "desktop", "scorciatoie", "shortcut", "command bar"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Menubar",
            controls: vec![
                Control::bool("border", "Bordo", true),
                Control::bool("help_disabled", "Menu Aiuto disabilitato", false).emit(Emit::Skip),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &[],
            children_code: Some(CHILDREN),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "checkable",
                title: "Voci a spunta",
                description: "checked trasforma la voce in un interruttore.",
                code: CHECKABLE_CODE,
                render: || rsx! { Checkable {} },
                span: Span::Half,
            },
            Example {
                id: "command-bar",
                title: "CommandBar",
                description: "Menu a sinistra, azione principale a destra.",
                code: COMMAND_BAR_CODE,
                render: || rsx! { CommandBarDemo {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Barra di un editor di configurazione: salva, vista e distribuisci.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Menubar", rows: MENUBAR_PROPS },
            PropGroup { component: "MenubarMenu · Trigger · Content", rows: MENU_PROPS },
            PropGroup { component: "MenubarItem", rows: ITEM_PROPS },
            PropGroup { component: "CommandBar", rows: BAR_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Track di item (§3.4) + floating panel (§3.2)",
            depth: "Well (barra) · Raised (menu)",
            tokens: &["--sd-well", "--sd-border", "--sd-item-h", "--sd-accent-soft", "--sd-surface-2", "--sd-elev-2", "--sd-font-mono"],
            notes: "Il trigger aperto prende sd-selected; le scorciatoie sono mono subtle allineate a destra.",
        }),
        a11y: &[
            "La barra ha role=menubar e aria-label dal Locale; i pannelli role=menu.",
            "I trigger sono menuitem con aria-haspopup e aria-expanded; Esc chiude il menu.",
            "Le voci con checked sono menuitemcheckbox con aria-checked.",
            "I separatori hanno role=separator.",
        ],
        related: &["dropdown", "context-menu", "overflow-set", "navigation-menu"],
    }
}
