//! Story: VirtualList (feature `virtual_list`). `render_item` is a `Callback`, so the
//! playground goes through `VirtualDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const COUNTS: [usize; 3] = [1_000, 10_000, 100_000];
const COUNT_OPTS: &[ChoiceOption] = &[opt("1.000", "1_000"), opt("10.000", "10_000"), opt("100.000", "100_000")];

/// Sample log lines: level and message. Row `i` shows `LOG[i % LOG.len()]`.
const LOG: [(&str, &str); 6] = [
    ("info", "edge-07: handshake completato"),
    ("info", "relay-mi: 412 pacchetti instradati"),
    ("warn", "relay-fra: latenza 212 ms"),
    ("info", "vault-01: snapshot salvato"),
    ("error", "edge-21: peer non raggiungibile"),
    ("info", "edge-22: chiave ruotata"),
];

fn level_badge(level: &str) -> BadgeVariant {
    match level {
        "warn" => BadgeVariant::Warning,
        "error" => BadgeVariant::Danger,
        _ => BadgeVariant::Default,
    }
}

const PRELUDE: &str = r#"const LOG: [(&str, &str); 6] = [
    ("info", "edge-07: handshake completato"),
    ("info", "relay-mi: 412 pacchetti instradati"),
    ("warn", "relay-fra: latenza 212 ms"),
    ("info", "vault-01: snapshot salvato"),
    ("error", "edge-21: peer non raggiungibile"),
    ("info", "edge-22: chiave ruotata"),
];
fn level_badge(level: &str) -> BadgeVariant {
    match level {
        "warn" => BadgeVariant::Warning,
        "error" => BadgeVariant::Danger,
        _ => BadgeVariant::Default,
    }
}"#;

const RENDER_ITEM: &[&str] = &[
    "render_item: move |idx: usize| {",
    "    let (level, message) = LOG[idx % LOG.len()];",
    "    rsx! {",
    "        Group { gap: SpacingToken::Md, wrap: false, class: \"w-full\",",
    "            span { \"{idx + 1:06}\" }",
    "            Badge { label: level.to_string(), variant: level_badge(level) }",
    "            span { \"{message}\" }",
    "        }",
    "    }",
    "},",
];

#[component]
fn VirtualDemo(knobs: Knobs) -> Element {
    rsx! {
        VirtualList {
            item_count: knobs.pick("item_count", &COUNTS),
            item_height: knobs.number("item_height") as u32,
            height: knobs.number("height") as u32,
            overscan: knobs.number("overscan") as usize,
            render_item: move |idx: usize| {
                let (level, message) = LOG[idx % LOG.len()];
                rsx! {
                    Group { gap: SpacingToken::Md, wrap: false, class: "w-full",
                        span { class: "sc-num sc-subtle-text", "{idx + 1:06}" }
                        Badge { label: level.to_string(), variant: level_badge(level) }
                        span { class: "sc-muted-text", "{message}" }
                    }
                }
            },
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        VirtualDemo { knobs: k.clone() }
    }
}

#[component]
fn Compact() -> Element {
    rsx! {
        VirtualList {
            item_count: 100_000,
            item_height: 32,
            height: 256,
            render_item: move |idx: usize| rsx! {
                span { class: "sc-num", "Blocco {idx + 1} · {(idx * 7919) % 100_000:05} conferme" }
            },
        }
    }
}
const COMPACT_CODE: &str = r#"VirtualList {
    item_count: 100_000,
    item_height: 32,
    height: 256,
    render_item: move |idx: usize| rsx! {
        span { "Blocco {idx + 1} · {(idx * 7919) % 100_000:05} conferme" }
    },
}"#;

const PEOPLE: [&str; 6] = ["Giulia Ferri", "Marco Riva", "Elena Sarti", "Luca Neri", "Anna Conti", "Paolo Gallo"];

#[component]
fn RichRows() -> Element {
    let mut toast = use_notification();
    rsx! {
        VirtualList {
            item_count: 5_000,
            item_height: 56,
            height: 256,
            render_item: move |idx: usize| {
                let name = PEOPLE[idx % PEOPLE.len()];
                rsx! {
                    Group { gap: SpacingToken::Md, wrap: false, justify: JustifyContent::SpaceBetween, class: "w-full",
                        Group { gap: SpacingToken::Sm, wrap: false,
                            Avatar { name: Some(name.to_string()), size: AvatarSize::Sm }
                            span { "{name} · utente {idx + 1}" }
                        }
                        Button {
                            variant: ButtonVariant::Ghost,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                toast.info(format!("Profilo {} aperto", idx + 1));
                            },
                            "Apri"
                        }
                    }
                }
            },
        }
    }
}
const RICH_ROWS_CODE: &str = r#"let mut toast = use_notification();

VirtualList {
    item_count: 5_000,
    item_height: 56,
    height: 256,
    render_item: move |idx: usize| {
        let name = PEOPLE[idx % PEOPLE.len()];
        rsx! {
            Group { gap: SpacingToken::Md, wrap: false, justify: JustifyContent::SpaceBetween, class: "w-full",
                Group { gap: SpacingToken::Sm, wrap: false,
                    Avatar { name: Some(name.to_string()), size: AvatarSize::Sm }
                    span { "{name} · utente {idx + 1}" }
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    onclick: move |_| {
                        toast.info(format!("Profilo {} aperto", idx + 1));
                    },
                    "Apri"
                }
            }
        }
    },
}"#;

#[component]
fn InContext() -> Element {
    let level = use_signal(|| "all".to_string());
    let rows: Vec<usize> = (0..50_000)
        .filter(|i| match level().as_str() {
            "warn" => LOG[i % LOG.len()].0 != "info",
            "error" => LOG[i % LOG.len()].0 == "error",
            _ => true,
        })
        .collect();
    let count = rows.len();
    rsx! {
        Card {
            title: Some("Registro eventi".to_string()),
            subtitle: Some(format!("{count} righe · solo quelle visibili sono nel DOM")),
            buttons: Some(rsx! {
                SegmentedControl {
                    options: vec![
                        SegmentOption::new("all", "Tutti"),
                        SegmentOption::new("warn", "Avvisi"),
                        SegmentOption::new("error", "Errori"),
                    ],
                    value: level,
                    size: SegmentedControlSize::Sm,
                }
            }),
            VirtualList {
                item_count: count,
                item_height: 44,
                height: 320,
                render_item: move |idx: usize| {
                    let line = rows[idx];
                    let (lvl, message) = LOG[line % LOG.len()];
                    rsx! {
                        Group { gap: SpacingToken::Md, wrap: false, class: "w-full",
                            span { class: "sc-num sc-subtle-text", "{line + 1:06}" }
                            Badge { label: lvl.to_string(), variant: level_badge(lvl) }
                            span { class: "sc-muted-text", "{message}" }
                        }
                    }
                },
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let level = use_signal(|| "all".to_string());
let rows: Vec<usize> = (0..50_000)
    .filter(|i| match level().as_str() {
        "warn" => LOG[i % LOG.len()].0 != "info",
        "error" => LOG[i % LOG.len()].0 == "error",
        _ => true,
    })
    .collect();
let count = rows.len();

Card {
    title: Some("Registro eventi".to_string()),
    subtitle: Some(format!("{count} righe · solo quelle visibili sono nel DOM")),
    buttons: Some(rsx! {
        SegmentedControl {
            options: vec![
                SegmentOption::new("all", "Tutti"),
                SegmentOption::new("warn", "Avvisi"),
                SegmentOption::new("error", "Errori"),
            ],
            value: level,
            size: SegmentedControlSize::Sm,
        }
    }),
    VirtualList {
        item_count: count,
        item_height: 44,
        height: 320,
        render_item: move |idx: usize| {
            let line = rows[idx];
            let (lvl, message) = LOG[line % LOG.len()];
            rsx! {
                Group { gap: SpacingToken::Md, wrap: false, class: "w-full",
                    span { "{line + 1:06}" }
                    Badge { label: lvl.to_string(), variant: level_badge(lvl) }
                    span { "{message}" }
                }
            }
        },
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("item_count", "usize", "-", "Numero totale di righe; nel DOM finiscono solo quelle visibili più l'overscan."),
    prop("render_item", "Callback<usize, Element>", "-", "Restituisce il contenuto della riga dato il suo indice assoluto."),
    prop("item_height", "u32", "48", "Altezza fissa di ogni riga in px: serve a calcolare la finestra visibile."),
    prop("height", "u32", "360", "Altezza del viewport scorrevole in px."),
    prop("overscan", "usize", "4", "Righe extra sopra e sotto la finestra, per non vedere vuoti nello scorrimento veloce."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul contenitore."),
];

pub fn story() -> Story {
    Story {
        slug: "virtual-list",
        name: "VirtualList",
        category: Category::Dati,
        features: &["virtual_list"],
        icon: IconKind::Layers,
        tagline: "Scorre decine di migliaia di righe tenendo nel DOM solo quelle visibili.",
        keywords: &["virtual list", "virtualizzazione", "lista lunga", "scroll", "log", "performance", "windowing"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "VirtualList",
            controls: vec![
                Control::choice("item_count", "Righe", COUNT_OPTS, 1).emit(Emit::Always),
                Control::number("item_height", "Altezza riga (px)", 48.0, 32.0, 64.0, 4.0, 0),
                Control::number("height", "Viewport (px)", 360.0, 200.0, 480.0, 20.0, 0),
                Control::number("overscan", "Overscan", 4.0, 0.0, 12.0, 1.0, 0)
                    .hint("Righe renderizzate fuori vista: con 0 lo scorrimento veloce mostra vuoti."),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: RENDER_ITEM,
            children_code: None,
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "compact",
                title: "Centomila righe",
                description: "Righe da 32 px: il costo resta quello di una decina di nodi.",
                code: COMPACT_CODE,
                render: || rsx! { Compact {} },
                span: Span::Half,
            },
            Example {
                id: "rich-rows",
                title: "Righe con componenti",
                description: "Ogni riga può ospitare avatar e pulsanti della libreria.",
                code: RICH_ROWS_CODE,
                render: || rsx! { RichRows {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Un registro di 50.000 eventi filtrato per livello: item_count segue il filtro.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "VirtualList", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Container (language.md §3.1) con scrollbar sottile (§3.2)",
            depth: "Flat-lit",
            tokens: &["--sd-surface-1", "--sd-border", "--sd-border-subtle", "--sd-radius-lg", "--sd-space-4", "--sd-specular"],
            notes: "Le righe sono statiche, separate da un filo --sd-border-subtle: decide il contenuto se una riga è interattiva.",
        }),
        a11y: &[
            "Nel DOM ci sono solo le righe visibili: screen reader e ricerca del browser non vedono il resto.",
            "Il contenitore non ha ruolo né tabindex: per lo scorrimento da tastiera metti elementi focalizzabili nelle righe.",
            "Le righe hanno altezza fissa: un contenuto più alto di item_height viene tagliato, non va a capo.",
            "Per elenchi brevi (meno di qualche centinaio di voci) preferisci List, che resta tutta accessibile.",
        ],
        related: &["lists", "tables", "scroll-area", "trees"],
    }
}
