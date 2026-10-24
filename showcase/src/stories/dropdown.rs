//! Story: Dropdown (feature `dropdown`). The menu owns an `is_open` signal, so the playground
//! goes through `DropdownDemo { knobs }`; the trigger sits in a card header.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::scenes::InboxPane;
use crate::story::schema::*;

const PLACEMENTS: [DropdownPlacement; 4] = [
    DropdownPlacement::BottomRight,
    DropdownPlacement::BottomLeft,
    DropdownPlacement::TopRight,
    DropdownPlacement::TopLeft,
];
const PLACEMENT_OPTS: &[ChoiceOption] = &[
    opt("BottomRight", "DropdownPlacement::BottomRight"),
    opt("BottomLeft", "DropdownPlacement::BottomLeft"),
    opt("TopRight", "DropdownPlacement::TopRight"),
    opt("TopLeft", "DropdownPlacement::TopLeft"),
];

const PRELUDE: &str = r#"let is_open = use_signal(|| false);
let mut toast = use_notification();"#;

const TRIGGER_CODE: &[&str] = &[
    "is_open,",
    "trigger: rsx! {",
    "    Button {",
    "        variant: ButtonVariant::Secondary,",
    "        size: ButtonSize::Sm,",
    "        right_icon: Some(rsx! { Icon { kind: IconKind::ChevronDown, size: 14 } }),",
    "        \"Azioni\"",
    "    }",
    "},",
];

const ITEMS_CODE: &str = r#"DropdownItem { icon: Some(IconKind::Terminal), onclick: move |_| { toast.info("Console aperta"); }, "Apri la console" }
DropdownItem { icon: Some(IconKind::Download), onclick: move |_| { toast.info("Log scaricati"); }, "Scarica i log" }
DropdownItem { icon: Some(IconKind::Copy), onclick: move |_| { toast.info("ID copiato"); }, "Copia l'ID del deploy" }
DropdownDivider {}
DropdownItem { icon: Some(IconKind::XCircle), danger: true, onclick: move |_| { toast.danger("Rollback avviato"); }, "Rollback" }"#;

#[component]
fn DropdownDemo(knobs: Knobs) -> Element {
    let is_open = use_signal(|| false);
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Deploy 418".to_string()),
            subtitle: Some("edge-22 · 3 regioni · 12 minuti fa".to_string()),
            buttons: Some(rsx! {
                Dropdown {
                    is_open,
                    trigger: rsx! {
                        Button {
                            variant: ButtonVariant::Secondary,
                            size: ButtonSize::Sm,
                            right_icon: Some(rsx! {
                                Icon { kind: IconKind::ChevronDown, size: 14 }
                            }),
                            "Azioni"
                        }
                    },
                    placement: knobs.pick("placement", &PLACEMENTS),
                    close_on_click: knobs.bool("close_on_click"),
                    glow: knobs.bool("glow").then_some(true),
                    DropdownItem {
                        icon: Some(IconKind::Terminal),
                        onclick: move |_| {
                            toast.info("Console aperta");
                        },
                        "Apri la console"
                    }
                    DropdownItem {
                        icon: Some(IconKind::Download),
                        onclick: move |_| {
                            toast.info("Log scaricati");
                        },
                        "Scarica i log"
                    }
                    DropdownItem {
                        icon: Some(IconKind::Copy),
                        onclick: move |_| {
                            toast.info("ID copiato");
                        },
                        "Copia l'ID del deploy"
                    }
                    DropdownDivider {}
                    DropdownItem {
                        icon: Some(IconKind::XCircle),
                        danger: true,
                        onclick: move |_| {
                            toast.danger("Rollback avviato");
                        },
                        "Rollback"
                    }
                }
            }),
            Group { gap: SpacingToken::Sm,
                Badge { label: "Riuscito".to_string(), variant: BadgeVariant::Success, pill: true, dot: true }
                Badge { label: "a91f3c2".to_string(), variant: BadgeVariant::Outline, pill: true }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        DropdownDemo { knobs: k.clone() }
    }
}

#[component]
fn Upward() -> Element {
    let is_open = use_signal(|| false);
    rsx! {
        Group { justify: JustifyContent::SpaceBetween,
            Text { size: TextSize::Sm, dimmed: true, "Piè di pagina della sidebar" }
            Dropdown {
                is_open,
                placement: DropdownPlacement::TopRight,
                trigger: rsx! {
                    Button {
                        variant: ButtonVariant::Ghost,
                        right_icon: Some(rsx! {
                            Icon { kind: IconKind::ChevronUp, size: 14 }
                        }),
                        Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Xs }
                        "Giulia"
                    }
                },
                DropdownItem { icon: Some(IconKind::Settings), "Preferenze" }
                DropdownItem { icon: Some(IconKind::Key), "Chiavi personali" }
                DropdownDivider {}
                DropdownItem { icon: Some(IconKind::Lock), "Esci" }
            }
        }
    }
}
const UPWARD_CODE: &str = r#"let is_open = use_signal(|| false);

Group { justify: JustifyContent::SpaceBetween,
    Text { size: TextSize::Sm, dimmed: true, "Piè di pagina della sidebar" }
    Dropdown {
        is_open,
        placement: DropdownPlacement::TopRight,
        trigger: rsx! {
            Button {
                variant: ButtonVariant::Ghost,
                right_icon: Some(rsx! {
                    Icon { kind: IconKind::ChevronUp, size: 14 }
                }),
                Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Xs }
                "Giulia"
            }
        },
        DropdownItem { icon: Some(IconKind::Settings), "Preferenze" }
        DropdownItem { icon: Some(IconKind::Key), "Chiavi personali" }
        DropdownDivider {}
        DropdownItem { icon: Some(IconKind::Lock), "Esci" }
    }
}"#;

#[component]
fn RowMenu() -> Element {
    let is_open = use_signal(|| false);
    rsx! {
        Group { justify: JustifyContent::SpaceBetween,
            Stack { gap: SpacingToken::None, align: AlignItems::Start,
                Text { size: TextSize::Sm, "pipeline-telemetria.toml" }
                Text { size: TextSize::Xs, dimmed: true, "Sola lettura · modificato ieri" }
            }
            Dropdown {
                is_open,
                trigger: rsx! {
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, aria_label: Some("Altre azioni".to_string()),
                        Icon { kind: IconKind::MoreHorizontal, size: 16 }
                    }
                },
                DropdownItem { icon: Some(IconKind::Eye), "Apri" }
                DropdownItem { icon: Some(IconKind::Copy), "Duplica" }
                DropdownItem { icon: Some(IconKind::Upload), disabled: true, "Sostituisci" }
                DropdownDivider {}
                DropdownItem { icon: Some(IconKind::Trash2), danger: true, "Elimina" }
            }
        }
    }
}
const ROW_MENU_CODE: &str = r#"let is_open = use_signal(|| false);

Group { justify: JustifyContent::SpaceBetween,
    Stack { gap: SpacingToken::None, align: AlignItems::Start,
        Text { size: TextSize::Sm, "pipeline-telemetria.toml" }
        Text { size: TextSize::Xs, dimmed: true, "Sola lettura · modificato ieri" }
    }
    Dropdown {
        is_open,
        trigger: rsx! {
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, aria_label: Some("Altre azioni".to_string()),
                Icon { kind: IconKind::MoreHorizontal, size: 16 }
            }
        },
        DropdownItem { icon: Some(IconKind::Eye), "Apri" }
        DropdownItem { icon: Some(IconKind::Copy), "Duplica" }
        DropdownItem { icon: Some(IconKind::Upload), disabled: true, "Sostituisci" }
        DropdownDivider {}
        DropdownItem { icon: Some(IconKind::Trash2), danger: true, "Elimina" }
    }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        InboxPane {}
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/inbox_pane.rs (estratto): il filtro dell'inbox, con una voce danger in fondo.
Dropdown {
    is_open: filter_open,
    placement: DropdownPlacement::BottomRight,
    trigger: rsx! {
        Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Filtra" }
    },
    DropdownItem { icon: Some(IconKind::Clock), "Più recenti" }
    DropdownItem { icon: Some(IconKind::AlertTriangle), "Solo incidenti" }
    DropdownDivider {}
    DropdownItem { icon: Some(IconKind::Trash2), danger: true, "Archivia i letti" }
}"#;

const DROPDOWN_PROPS: &[PropDoc] = &[
    prop("is_open", "Signal<bool>", "-", "Visibilità del menu; il trigger la alterna."),
    prop("trigger", "Element", "-", "Elemento che apre il menu, di solito un Button."),
    prop("placement", "DropdownPlacement", "BottomRight", "BottomRight, BottomLeft, TopRight, TopLeft: Top* apre verso l'alto vicino al fondo."),
    prop("close_on_click", "bool", "true", "Chiude il menu dopo il clic su una voce."),
    prop("glow", "Option<bool>", "None", "Glow opzionale sul pannello."),
    prop("id", "Option<String>", "None", "Id del contenitore; generato se assente."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
    prop("children", "Element", "-", "Voci DropdownItem e DropdownDivider."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("icon", "Option<IconKind>", "None", "Icona iniziale, aria-hidden."),
    prop("danger", "bool", "false", "Voce distruttiva: testo --sd-danger-strong."),
    prop("disabled", "bool", "false", "Voce non attivabile, annunciata con aria-disabled."),
    prop("onclick", "Option<EventHandler<MouseEvent>>", "None", "Azione della voce."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
    prop("children", "Element", "-", "Etichetta."),
];
const DIVIDER_PROPS: &[PropDoc] = &[prop("DropdownDivider {}", "-", "-", "Separatore tra gruppi di voci, role=separator.")];

pub fn story() -> Story {
    Story {
        slug: "dropdown",
        name: "Dropdown",
        category: Category::Overlay,
        features: &["dropdown"],
        icon: IconKind::ChevronDown,
        tagline: "Menu di azioni ancorato a un pulsante, che si apre verso il basso o verso l'alto.",
        keywords: &["dropdown", "menu", "azioni", "actions", "tendina", "overflow menu"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Dropdown",
            controls: vec![
                Control::choice("placement", "Posizione", PLACEMENT_OPTS, 0),
                Control::bool("close_on_click", "Chiudi al clic", true),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: TRIGGER_CODE,
            children_code: Some(ITEMS_CODE),
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "upward",
                title: "Verso l'alto",
                description: "TopRight per un menu account in fondo a una colonna.",
                code: UPWARD_CODE,
                render: || rsx! { Upward {} },
                span: Span::Half,
            },
            Example {
                id: "row-menu",
                title: "Menu di riga",
                description: "Trigger a sola icona, voce disabilitata e voce danger.",
                code: ROW_MENU_CODE,
                render: || rsx! { RowMenu {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il filtro dell'inbox operatori.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Dropdown", rows: DROPDOWN_PROPS },
            PropGroup { component: "DropdownItem", rows: ITEM_PROPS },
            PropGroup { component: "DropdownDivider", rows: DIVIDER_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Floating panel (language.md §3.2) + items (§3.4)",
            depth: "Raised",
            tokens: &["--sd-surface-2", "--sd-border-strong", "--sd-elev-2", "--sd-radius-lg", "--sd-item-h", "--sd-radius-sm", "--sd-danger-strong"],
            notes: "Padding del menu --sd-space-2; le voci usano un raggio più piccolo del pannello (sm dentro lg).",
        }),
        a11y: &[
            "Il menu ha role=menu ed è etichettato dal trigger con aria-labelledby.",
            "Le voci sono pulsanti nativi con role=menuitem; quelle disabilitate portano aria-disabled.",
            "Esc chiude il menu e riporta il focus sul trigger; Tab o un clic fuori lo chiudono.",
            "I trigger a sola icona richiedono aria_label sul Button.",
        ],
        related: &["context-menu", "popover", "menubar", "select", "speed-dial"],
    }
}
