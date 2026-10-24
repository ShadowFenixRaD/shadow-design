//! Story: Popover (feature `popover`). Compound component (`Popover` + `PopoverTrigger` +
//! `PopoverDropdown`) with an `opened` signal: the playground goes through `PopoverDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const POSITIONS: [PopoverPosition; 12] = [
    PopoverPosition::Top,
    PopoverPosition::TopStart,
    PopoverPosition::TopEnd,
    PopoverPosition::Bottom,
    PopoverPosition::BottomStart,
    PopoverPosition::BottomEnd,
    PopoverPosition::Left,
    PopoverPosition::LeftStart,
    PopoverPosition::LeftEnd,
    PopoverPosition::Right,
    PopoverPosition::RightStart,
    PopoverPosition::RightEnd,
];
const POSITION_OPTS: &[ChoiceOption] = &[
    opt("Top", "PopoverPosition::Top"),
    opt("TopStart", "PopoverPosition::TopStart"),
    opt("TopEnd", "PopoverPosition::TopEnd"),
    opt("Bottom", "PopoverPosition::Bottom"),
    opt("BottomStart", "PopoverPosition::BottomStart"),
    opt("BottomEnd", "PopoverPosition::BottomEnd"),
    opt("Left", "PopoverPosition::Left"),
    opt("LeftStart", "PopoverPosition::LeftStart"),
    opt("LeftEnd", "PopoverPosition::LeftEnd"),
    opt("Right", "PopoverPosition::Right"),
    opt("RightStart", "PopoverPosition::RightStart"),
    opt("RightEnd", "PopoverPosition::RightEnd"),
];

const PRELUDE: &str = r#"let mut opened = use_signal(|| false);
let email = use_signal(String::new);
let mut toast = use_notification();"#;

const CHILDREN_CODE: &str = r#"PopoverTrigger {
    Button {
        variant: ButtonVariant::Secondary,
        left_icon: Some(rsx! { Icon { kind: IconKind::Share2, size: 15 } }),
        "Condividi"
    }
}
PopoverDropdown {
    Stack { gap: SpacingToken::Sm,
        Text { size: TextSize::Sm, "Invita per email" }
        TextInput { value: email, placeholder: Some("nome@shadowfenix.dev".to_string()) }
        Group { justify: JustifyContent::End,
            Button {
                size: ButtonSize::Sm,
                onclick: move |_| {
                    opened.set(false);
                    toast.success("Invito inviato");
                },
                "Invita"
            }
        }
    }
}"#;

#[component]
fn PopoverDemo(knobs: Knobs) -> Element {
    let mut opened = use_signal(|| false);
    let email = use_signal(String::new);
    let mut toast = use_notification();
    // close_on_click_outside and glow are read once when the popover mounts: remount on change.
    let remount = format!("{}-{}", knobs.bool("close_on_click_outside"), knobs.bool("glow"));
    rsx! {
        Group { gap: SpacingToken::Md, justify: JustifyContent::SpaceBetween,
            Stack { gap: SpacingToken::None, align: AlignItems::Start,
                Text { size: TextSize::Sm, "Runbook incidenti" }
                Text { size: TextSize::Xs, dimmed: true, "Condiviso con 3 persone" }
            }
            for key in [remount] {
                Popover {
                    key: "{key}",
                    opened,
                    position: knobs.pick("position", &POSITIONS),
                    with_arrow: knobs.bool("with_arrow"),
                    close_on_escape: knobs.bool("close_on_escape"),
                    close_on_click_outside: knobs.bool("close_on_click_outside"),
                    glow: knobs.bool("glow").then_some(true),
                    PopoverTrigger {
                        Button {
                            variant: ButtonVariant::Secondary,
                            left_icon: Some(rsx! {
                                Icon { kind: IconKind::Share2, size: 15 }
                            }),
                            "Condividi"
                        }
                    }
                    PopoverDropdown {
                        Stack { gap: SpacingToken::Sm,
                            Text { size: TextSize::Sm, "Invita per email" }
                            TextInput { value: email, placeholder: Some("nome@shadowfenix.dev".to_string()) }
                            Group { justify: JustifyContent::End,
                                Button {
                                    size: ButtonSize::Sm,
                                    onclick: move |_| {
                                        opened.set(false);
                                        toast.success("Invito inviato");
                                    },
                                    "Invita"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        PopoverDemo { knobs: k.clone() }
    }
}

#[component]
fn Positions() -> Element {
    let top = use_signal(|| false);
    let right = use_signal(|| false);
    let bottom = use_signal(|| false);
    let left = use_signal(|| false);
    rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::Center,
            Popover { opened: top, position: PopoverPosition::Top,
                PopoverTrigger {
                    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Top" }
                }
                PopoverDropdown {
                    Text { size: TextSize::Sm, "Sopra il trigger" }
                }
            }
            Popover { opened: right, position: PopoverPosition::Right,
                PopoverTrigger {
                    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Right" }
                }
                PopoverDropdown {
                    Text { size: TextSize::Sm, "A destra" }
                }
            }
            Popover { opened: bottom, position: PopoverPosition::Bottom,
                PopoverTrigger {
                    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Bottom" }
                }
                PopoverDropdown {
                    Text { size: TextSize::Sm, "Sotto il trigger" }
                }
            }
            Popover { opened: left, position: PopoverPosition::Left,
                PopoverTrigger {
                    Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Left" }
                }
                PopoverDropdown {
                    Text { size: TextSize::Sm, "A sinistra" }
                }
            }
        }
    }
}
const POSITIONS_CODE: &str = r#"let top = use_signal(|| false);
let right = use_signal(|| false);
let bottom = use_signal(|| false);
let left = use_signal(|| false);

Group { gap: SpacingToken::Sm, justify: JustifyContent::Center,
    Popover { opened: top, position: PopoverPosition::Top,
        PopoverTrigger {
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Top" }
        }
        PopoverDropdown {
            Text { size: TextSize::Sm, "Sopra il trigger" }
        }
    }
    Popover { opened: right, position: PopoverPosition::Right,
        PopoverTrigger {
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Right" }
        }
        PopoverDropdown {
            Text { size: TextSize::Sm, "A destra" }
        }
    }
    Popover { opened: bottom, position: PopoverPosition::Bottom,
        PopoverTrigger {
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Bottom" }
        }
        PopoverDropdown {
            Text { size: TextSize::Sm, "Sotto il trigger" }
        }
    }
    Popover { opened: left, position: PopoverPosition::Left,
        PopoverTrigger {
            Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, "Left" }
        }
        PopoverDropdown {
            Text { size: TextSize::Sm, "A sinistra" }
        }
    }
}"#;

#[component]
fn RangeFilter() -> Element {
    let opened = use_signal(|| false);
    let range = use_signal(|| "7g".to_string());
    let errors = use_signal(|| true);
    rsx! {
        Group {
            Popover { opened, position: PopoverPosition::BottomStart, with_arrow: false,
                PopoverTrigger {
                    Button {
                        variant: ButtonVariant::Secondary,
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Calendar, size: 15 }
                        }),
                        "Ultimi {range}"
                    }
                }
                PopoverDropdown {
                    Stack { gap: SpacingToken::Md,
                        SegmentedControl {
                            options: vec![
                                SegmentOption::new("24h", "24h"),
                                SegmentOption::new("7g", "7g"),
                                SegmentOption::new("30g", "30g"),
                            ],
                            value: range,
                            full_width: true,
                        }
                        Switch { checked: errors, label: Some("Mostra solo gli errori".to_string()) }
                    }
                }
            }
        }
    }
}
const RANGE_FILTER_CODE: &str = r#"let opened = use_signal(|| false);
let range = use_signal(|| "7g".to_string());
let errors = use_signal(|| true);

Group {
    Popover { opened, position: PopoverPosition::BottomStart, with_arrow: false,
        PopoverTrigger {
            Button {
                variant: ButtonVariant::Secondary,
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Calendar, size: 15 }
                }),
                "Ultimi {range}"
            }
        }
        PopoverDropdown {
            Stack { gap: SpacingToken::Md,
                SegmentedControl {
                    options: vec![
                        SegmentOption::new("24h", "24h"),
                        SegmentOption::new("7g", "7g"),
                        SegmentOption::new("30g", "30g"),
                    ],
                    value: range,
                    full_width: true,
                }
                Switch { checked: errors, label: Some("Mostra solo gli errori".to_string()) }
            }
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut opened = use_signal(|| false);
    let threshold = use_signal(|| 3.0);
    let paging = use_signal(|| true);
    let mut toast = use_notification();
    let current = threshold();
    rsx! {
        Card {
            title: Some("Latenza p95".to_string()),
            subtitle: Some("Cluster eu-west · ultime 24 ore".to_string()),
            buttons: Some(rsx! {
                Popover { opened, position: PopoverPosition::BottomEnd,
                    PopoverTrigger {
                        Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, aria_label: Some("Soglie di allarme".to_string()),
                            Icon { kind: IconKind::Settings, size: 16 }
                        }
                    }
                    PopoverDropdown {
                        Stack { gap: SpacingToken::Md,
                            Text { size: TextSize::Sm, "Soglie di allarme" }
                            Slider { value: threshold, min: 1.0, max: 10.0, step: 0.5, label: Some("Allarme sopra (ms)".to_string()) }
                            Switch { checked: paging, label: Some("Chiama il reperibile".to_string()) }
                            Button {
                                size: ButtonSize::Sm,
                                full_width: true,
                                onclick: move |_| {
                                    opened.set(false);
                                    toast.success(format!("Soglia impostata a {:.1} ms", threshold()));
                                },
                                "Salva soglie"
                            }
                        }
                    }
                }
            }),
            Group { gap: SpacingToken::Lg, justify: JustifyContent::SpaceBetween,
                Stack { gap: SpacingToken::None, align: AlignItems::Start,
                    Heading { order: HeadingOrder::H3, "1.2 ms" }
                    Text { size: TextSize::Xs, dimmed: true, "Soglia attuale {current:.1} ms" }
                }
                Sparkline { data: vec![1.6, 1.5, 1.7, 1.4, 1.3, 1.25, 1.2], trend: TrendDirection::Positive, width: 160, height: 40 }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut opened = use_signal(|| false);
let threshold = use_signal(|| 3.0);
let paging = use_signal(|| true);
let mut toast = use_notification();
let current = threshold();

Card {
    title: Some("Latenza p95".to_string()),
    subtitle: Some("Cluster eu-west · ultime 24 ore".to_string()),
    buttons: Some(rsx! {
        Popover { opened, position: PopoverPosition::BottomEnd,
            PopoverTrigger {
                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, aria_label: Some("Soglie di allarme".to_string()),
                    Icon { kind: IconKind::Settings, size: 16 }
                }
            }
            PopoverDropdown {
                Stack { gap: SpacingToken::Md,
                    Text { size: TextSize::Sm, "Soglie di allarme" }
                    Slider { value: threshold, min: 1.0, max: 10.0, step: 0.5, label: Some("Allarme sopra (ms)".to_string()) }
                    Switch { checked: paging, label: Some("Chiama il reperibile".to_string()) }
                    Button {
                        size: ButtonSize::Sm,
                        full_width: true,
                        onclick: move |_| {
                            opened.set(false);
                            toast.success(format!("Soglia impostata a {:.1} ms", threshold()));
                        },
                        "Salva soglie"
                    }
                }
            }
        }
    }),
    Group { gap: SpacingToken::Lg, justify: JustifyContent::SpaceBetween,
        Stack { gap: SpacingToken::None, align: AlignItems::Start,
            Heading { order: HeadingOrder::H3, "1.2 ms" }
            Text { size: TextSize::Xs, dimmed: true, "Soglia attuale {current:.1} ms" }
        }
        Sparkline { data: vec![1.6, 1.5, 1.7, 1.4, 1.3, 1.25, 1.2], trend: TrendDirection::Positive, width: 160, height: 40 }
    }
}"#;

const POPOVER_PROPS: &[PropDoc] = &[
    prop("opened", "Signal<bool>", "-", "Visibilità; il trigger la alterna, Esc e il clic fuori la azzerano."),
    prop("position", "PopoverPosition", "Bottom", "Dodici posizioni: Top, Bottom, Left, Right, ciascuna con Start ed End."),
    prop("with_arrow", "bool", "true", "Freccia verso il trigger."),
    prop("close_on_escape", "bool", "true", "Esc chiude il pannello."),
    prop("close_on_click_outside", "bool", "true", "Un clic fuori dal pannello lo chiude."),
    prop("glow", "Option<bool>", "None", "Glow opzionale sul pannello."),
    prop("id", "Option<String>", "None", "Id del contenitore."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
    prop("children", "Element", "-", "Un PopoverTrigger e un PopoverDropdown."),
];
const TRIGGER_PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "Elemento cliccabile (Button, Avatar...): riceve aria-haspopup e aria-expanded."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul wrapper."),
];
const DROPDOWN_PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "Contenuto del pannello: testo, campi, azioni."),
    prop("glow", "Option<bool>", "None", "Sovrascrive il glow del Popover."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul pannello."),
];

pub fn story() -> Story {
    Story {
        slug: "popover",
        name: "Popover",
        category: Category::Overlay,
        features: &["popover"],
        icon: IconKind::Layers,
        tagline: "Pannello flottante ancorato a un trigger, per piccoli form e dettagli senza lasciare la vista.",
        keywords: &["popover", "pannello", "floating", "flottante", "anchor", "dialog"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Popover",
            controls: vec![
                Control::choice("position", "Posizione", POSITION_OPTS, 3),
                Control::bool("with_arrow", "Freccia", true),
                Control::bool("close_on_escape", "Chiudi con Esc", true),
                Control::bool("close_on_click_outside", "Chiudi al clic fuori", true),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["opened,"],
            children_code: Some(CHILDREN_CODE),
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "positions",
                title: "Posizioni",
                description: "Quattro lati; ogni lato ha anche le varianti Start ed End.",
                code: POSITIONS_CODE,
                render: || rsx! { Positions {} },
                span: Span::Half,
            },
            Example {
                id: "range-filter",
                title: "Filtro senza freccia",
                description: "Controlli compatti allineati al bordo del trigger.",
                code: RANGE_FILTER_CODE,
                render: || rsx! { RangeFilter {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Le soglie di allarme di una metrica, modificate sul posto.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Popover", rows: POPOVER_PROPS },
            PropGroup { component: "PopoverTrigger", rows: TRIGGER_PROPS },
            PropGroup { component: "PopoverDropdown", rows: DROPDOWN_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Floating panel (language.md §3.2)",
            depth: "Raised",
            tokens: &["--sd-surface-2", "--sd-border-strong", "--sd-elev-2", "--sd-specular", "--sd-radius-lg", "--sd-space-4"],
            notes: "Padding --sd-space-4 come i dialoghi; entra con sd-pop-in e non ha backdrop visibile.",
        }),
        a11y: &[
            "Il trigger porta aria-haspopup=dialog e aria-expanded sincronizzato con lo stato.",
            "Il pannello ha role=dialog e resta nel flusso del DOM accanto al trigger.",
            "Esc chiude il pannello finché close_on_escape è attivo.",
            "Un trigger a sola icona richiede aria_label sul Button.",
        ],
        related: &["hover-card", "tooltip", "dropdown", "modals"],
    }
}
