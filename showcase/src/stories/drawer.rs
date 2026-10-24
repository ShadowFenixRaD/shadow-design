//! Story: Drawer (feature `drawer`). Overlay story: the trigger sits in a node toolbar,
//! the playground goes through `DrawerDemo { knobs }` because the panel owns signals.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const POSITIONS: [DrawerPosition; 2] = [DrawerPosition::Left, DrawerPosition::Right];
const POSITION_OPTS: &[ChoiceOption] = &[opt("Left", "DrawerPosition::Left"), opt("Right", "DrawerPosition::Right")];
const SIZES: [DrawerSize; 3] = [DrawerSize::Sm, DrawerSize::Md, DrawerSize::Lg];
const SIZE_OPTS: &[ChoiceOption] = &[opt("Sm", "DrawerSize::Sm"), opt("Md", "DrawerSize::Md"), opt("Lg", "DrawerSize::Lg")];

const PRELUDE: &str = r#"let mut is_open = use_signal(|| false);
let gzip = use_signal(|| true);
let tracing = use_signal(|| false);"#;

const BODY_CODE: &str = r#"Stack { gap: SpacingToken::Lg,
    Text { dimmed: true, "Parametri del runtime per edge-07. Le modifiche valgono dal prossimo riavvio." }
    Switch { checked: gzip, label: Some("Compressione gzip".to_string()), description: Some("Riduce il payload degli asset statici".to_string()) }
    Switch { checked: tracing, label: Some("Tracing di debug".to_string()), description: Some("Log dettagliati nella console del nodo".to_string()) }
    Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
        Button { variant: ButtonVariant::Ghost, onclick: move |_| is_open.set(false), "Annulla" }
        Button { onclick: move |_| is_open.set(false), "Salva" }
    }
}"#;

#[component]
fn DrawerDemo(knobs: Knobs) -> Element {
    let mut is_open = use_signal(|| false);
    let gzip = use_signal(|| true);
    let tracing = use_signal(|| false);
    let title = knobs.text("title");
    rsx! {
        Card {
            title: Some("edge-07".to_string()),
            subtitle: Some("Milano · 42 peer · 0.9 ms".to_string()),
            buttons: Some(rsx! {
                Group { gap: SpacingToken::Sm,
                    Badge { label: "Online".to_string(), variant: BadgeVariant::Success, pill: true, dot: true }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Settings, size: 14 }
                        }),
                        onclick: move |_| is_open.set(true),
                        "Configura"
                    }
                }
            }),
            Text { size: TextSize::Sm, dimmed: true, "Il pannello di configurazione si apre dal bordo scelto, sopra la pagina." }
        }
        Drawer {
            is_open,
            title: (!title.is_empty()).then_some(title),
            position: knobs.pick("position", &POSITIONS),
            size: knobs.pick("size", &SIZES),
            close_on_escape: knobs.bool("close_on_escape"),
            close_on_backdrop: knobs.bool("close_on_backdrop"),
            glow: knobs.bool("glow").then_some(true),
            Stack { gap: SpacingToken::Lg,
                Text { dimmed: true, "Parametri del runtime per edge-07. Le modifiche valgono dal prossimo riavvio." }
                Switch { checked: gzip, label: Some("Compressione gzip".to_string()), description: Some("Riduce il payload degli asset statici".to_string()) }
                Switch { checked: tracing, label: Some("Tracing di debug".to_string()), description: Some("Log dettagliati nella console del nodo".to_string()) }
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, onclick: move |_| is_open.set(false), "Annulla" }
                    Button { onclick: move |_| is_open.set(false), "Salva" }
                }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        DrawerDemo { knobs: k.clone() }
    }
}

#[component]
fn NavLeft() -> Element {
    let mut is_open = use_signal(|| false);
    let mut active = use_signal(|| "nodi");
    rsx! {
        Group {
            Button {
                variant: ButtonVariant::Ghost,
                aria_label: Some("Apri la navigazione".to_string()),
                onclick: move |_| is_open.set(true),
                Icon { kind: IconKind::Menu, size: 18 }
            }
        }
        Drawer {
            is_open,
            title: Some("Shadowfenix Mesh".to_string()),
            position: DrawerPosition::Left,
            size: DrawerSize::Sm,
            Stack { gap: SpacingToken::Xs,
                NavLink {
                    label: "Panoramica".to_string(),
                    icon: Some(SvgIcon::kind(IconKind::Layers)),
                    active: active() == "panoramica",
                    on_click: move |_| active.set("panoramica"),
                }
                NavLink {
                    label: "Nodi".to_string(),
                    icon: Some(SvgIcon::kind(IconKind::Server)),
                    badge: Some("24".to_string()),
                    active: active() == "nodi",
                    on_click: move |_| active.set("nodi"),
                }
                NavLink {
                    label: "Sicurezza".to_string(),
                    icon: Some(SvgIcon::kind(IconKind::Shield)),
                    active: active() == "sicurezza",
                    on_click: move |_| active.set("sicurezza"),
                }
            }
        }
    }
}
const NAV_LEFT_CODE: &str = r#"let mut is_open = use_signal(|| false);
let mut active = use_signal(|| "nodi");

Group {
    Button {
        variant: ButtonVariant::Ghost,
        aria_label: Some("Apri la navigazione".to_string()),
        onclick: move |_| is_open.set(true),
        Icon { kind: IconKind::Menu, size: 18 }
    }
}
Drawer {
    is_open,
    title: Some("Shadowfenix Mesh".to_string()),
    position: DrawerPosition::Left,
    size: DrawerSize::Sm,
    Stack { gap: SpacingToken::Xs,
        NavLink {
            label: "Panoramica".to_string(),
            icon: Some(SvgIcon::kind(IconKind::Layers)),
            active: active() == "panoramica",
            on_click: move |_| active.set("panoramica"),
        }
        NavLink {
            label: "Nodi".to_string(),
            icon: Some(SvgIcon::kind(IconKind::Server)),
            badge: Some("24".to_string()),
            active: active() == "nodi",
            on_click: move |_| active.set("nodi"),
        }
        NavLink {
            label: "Sicurezza".to_string(),
            icon: Some(SvgIcon::kind(IconKind::Shield)),
            active: active() == "sicurezza",
            on_click: move |_| active.set("sicurezza"),
        }
    }
}"#;

#[component]
fn Filters() -> Element {
    let mut is_open = use_signal(|| false);
    let degraded = use_signal(|| true);
    let offline = use_signal(|| false);
    let latency = use_signal(|| 5.0);
    let mut toast = use_notification();
    rsx! {
        Group {
            Button {
                variant: ButtonVariant::Secondary,
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Search, size: 15 }
                }),
                onclick: move |_| is_open.set(true),
                "Filtri"
            }
        }
        Drawer { is_open, title: Some("Filtra i nodi".to_string()),
            Stack { gap: SpacingToken::Lg,
                Checkbox { checked: degraded, label: Some("Includi i nodi degradati".to_string()) }
                Checkbox { checked: offline, label: Some("Includi i nodi offline".to_string()) }
                Slider { value: latency, min: 0.0, max: 10.0, step: 0.5, label: Some("Latenza massima (ms)".to_string()) }
                Button {
                    full_width: true,
                    onclick: move |_| {
                        is_open.set(false);
                        toast.info("Filtri applicati");
                    },
                    "Applica filtri"
                }
            }
        }
    }
}
const FILTERS_CODE: &str = r#"let mut is_open = use_signal(|| false);
let degraded = use_signal(|| true);
let offline = use_signal(|| false);
let latency = use_signal(|| 5.0);
let mut toast = use_notification();

Group {
    Button {
        variant: ButtonVariant::Secondary,
        left_icon: Some(rsx! {
            Icon { kind: IconKind::Search, size: 15 }
        }),
        onclick: move |_| is_open.set(true),
        "Filtri"
    }
}
Drawer { is_open, title: Some("Filtra i nodi".to_string()),
    Stack { gap: SpacingToken::Lg,
        Checkbox { checked: degraded, label: Some("Includi i nodi degradati".to_string()) }
        Checkbox { checked: offline, label: Some("Includi i nodi offline".to_string()) }
        Slider { value: latency, min: 0.0, max: 10.0, step: 0.5, label: Some("Latenza massima (ms)".to_string()) }
        Button {
            full_width: true,
            onclick: move |_| {
                is_open.set(false);
                toast.info("Filtri applicati");
            },
            "Applica filtri"
        }
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut is_open = use_signal(|| false);
    rsx! {
        Alert {
            variant: AlertVariant::Warning,
            title: Some("Latenza elevata su relay-03".to_string()),
            Group { gap: SpacingToken::Md, justify: JustifyContent::SpaceBetween,
                Text { size: TextSize::Sm, "Picco a 3.8 ms dalle 08:10 sulla tratta verso Parigi." }
                Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, onclick: move |_| is_open.set(true), "Apri l'incidente" }
            }
        }
        Drawer { is_open, title: Some("Incidente INC-2041".to_string()), size: DrawerSize::Lg,
            Stack { gap: SpacingToken::Lg,
                Group { gap: SpacingToken::Sm,
                    Badge { label: "Aperto".to_string(), variant: BadgeVariant::Warning, pill: true, dot: true }
                    Badge { label: "relay-03".to_string(), variant: BadgeVariant::Outline, pill: true }
                }
                Timeline { active: 1,
                    TimelineItem { title: Some("Allarme latenza".to_string()), time: Some("08:10".to_string()),
                        "La p95 supera la soglia di 3 ms."
                    }
                    TimelineItem { title: Some("Presa in carico".to_string()), time: Some("08:14".to_string()),
                        "Marco Bellini analizza la tratta verso Parigi."
                    }
                    TimelineItem { title: Some("Reindirizzamento".to_string()), time: Some("in attesa".to_string()),
                        "Il traffico passerà da edge-12."
                    }
                }
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, onclick: move |_| is_open.set(false), "Chiudi" }
                    Button { onclick: move |_| is_open.set(false), "Reindirizza ora" }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut is_open = use_signal(|| false);

Alert {
    variant: AlertVariant::Warning,
    title: Some("Latenza elevata su relay-03".to_string()),
    Group { gap: SpacingToken::Md, justify: JustifyContent::SpaceBetween,
        Text { size: TextSize::Sm, "Picco a 3.8 ms dalle 08:10 sulla tratta verso Parigi." }
        Button { variant: ButtonVariant::Secondary, size: ButtonSize::Sm, onclick: move |_| is_open.set(true), "Apri l'incidente" }
    }
}
Drawer { is_open, title: Some("Incidente INC-2041".to_string()), size: DrawerSize::Lg,
    Stack { gap: SpacingToken::Lg,
        Group { gap: SpacingToken::Sm,
            Badge { label: "Aperto".to_string(), variant: BadgeVariant::Warning, pill: true, dot: true }
            Badge { label: "relay-03".to_string(), variant: BadgeVariant::Outline, pill: true }
        }
        Timeline { active: 1,
            TimelineItem { title: Some("Allarme latenza".to_string()), time: Some("08:10".to_string()),
                "La p95 supera la soglia di 3 ms."
            }
            TimelineItem { title: Some("Presa in carico".to_string()), time: Some("08:14".to_string()),
                "Marco Bellini analizza la tratta verso Parigi."
            }
            TimelineItem { title: Some("Reindirizzamento".to_string()), time: Some("in attesa".to_string()),
                "Il traffico passerà da edge-12."
            }
        }
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button { variant: ButtonVariant::Ghost, onclick: move |_| is_open.set(false), "Chiudi" }
            Button { onclick: move |_| is_open.set(false), "Reindirizza ora" }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("is_open", "Signal<bool>", "-", "Apre e chiude il pannello; X, Esc e backdrop lo riportano a false."),
    prop("title", "Option<String>", "None", "Titolo nell'intestazione, collegato con aria-labelledby."),
    prop("aria_label", "Option<String>", "None", "Nome accessibile quando non c'è un titolo visibile."),
    prop("position", "DrawerPosition", "Right", "Bordo da cui entra il pannello: Left o Right."),
    prop("size", "DrawerSize", "Md", "Larghezza del pannello: Sm, Md, Lg."),
    prop("close_on_escape", "bool", "true", "Esc chiude il pannello."),
    prop("close_on_backdrop", "bool", "true", "Il clic sul backdrop chiude il pannello."),
    prop("close_label", "Option<String>", "None", "Etichetta del pulsante X (da Locale se assente)."),
    prop("glow", "Option<bool>", "None", "Glow perimetrale opzionale."),
    prop("id", "Option<String>", "None", "Id del pannello; generato se assente."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul pannello."),
    prop("children", "Element", "-", "Corpo scorrevole del pannello."),
];

pub fn story() -> Story {
    Story {
        slug: "drawer",
        name: "Drawer",
        category: Category::Overlay,
        features: &["drawer"],
        icon: IconKind::Menu,
        tagline: "Pannello che entra dal bordo dello schermo per navigazione, filtri o dettagli senza lasciare la pagina.",
        keywords: &["drawer", "pannello laterale", "side panel", "sheet", "offcanvas", "overlay"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Drawer",
            controls: vec![
                Control::text("title", "Titolo", "Configurazione nodo").emit(Emit::Template("title: Some(\"{v}\".to_string())")),
                Control::choice("position", "Posizione", POSITION_OPTS, 1),
                Control::choice("size", "Dimensione", SIZE_OPTS, 1),
                Control::bool("close_on_escape", "Chiudi con Esc", true),
                Control::bool("close_on_backdrop", "Chiudi dal backdrop", true),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["is_open,"],
            children_code: Some(BODY_CODE),
            stage: Stage::Fill,
        }),
        examples: vec![
            Example {
                id: "nav-left",
                title: "Navigazione a sinistra",
                description: "Menu mobile: Left, Sm, voci NavLink.",
                code: NAV_LEFT_CODE,
                render: || rsx! { NavLeft {} },
                span: Span::Half,
            },
            Example {
                id: "filters",
                title: "Filtri",
                description: "Campi di filtro con una sola azione primaria in fondo.",
                code: FILTERS_CODE,
                render: || rsx! { Filters {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il dettaglio di un incidente si apre da un avviso, con la sua cronologia.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Drawer", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Floating panel (language.md §3.2) + backdrop (§3.3)",
            depth: "Raised-max (edge-attached)",
            tokens: &["--sd-surface-2", "--sd-border-strong", "--sd-elev-3", "--sd-radius-xl", "--sd-scrim", "--sd-backdrop-blur", "--sd-dur-slow"],
            notes: "Attaccato al bordo: raggio solo sul lato libero; entra con sd-slide-in-<lato> a --sd-dur-slow.",
        }),
        a11y: &[
            "Pannello con role=dialog e aria-modal=true, nominato dal titolo (aria-labelledby) o da aria_label.",
            "All'apertura il focus va al pannello e Tab / Maiusc+Tab restano intrappolati al suo interno.",
            "Esc chiude il pannello finché close_on_escape è attivo; lo scroll della pagina resta bloccato mentre è aperto.",
            "Il pulsante X è un CloseButton con etichetta da Locale (close_label).",
        ],
        related: &["modals", "app-shell", "navbar", "nav-link"],
    }
}
