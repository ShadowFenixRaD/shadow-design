//! Story: CommandPalette and CommandAction (feature `command_palette`). Overlay story:
//! `Stage::Fill`, the trigger sits inside a small console toolbar (REVAMP §6).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

fn console_actions() -> Vec<CommandAction> {
    vec![
        CommandAction::new("edge-07", "Apri edge-07")
            .with_description("Francoforte · 12 ms · 4 peer")
            .with_category("Nodi")
            .with_icon(IconKind::Server)
            .with_keywords(["frankfurt", "nodo"]),
        CommandAction::new("core-02", "Apri core-02")
            .with_description("Milano · 8 ms · 9 peer")
            .with_category("Nodi")
            .with_icon(IconKind::Server)
            .with_keywords(["milano", "nodo"]),
        CommandAction::new("rotate", "Ruota le chiavi Kyber")
            .with_description("Genera e distribuisce nuove chiavi su tutta la mesh")
            .with_category("Sicurezza")
            .with_shortcut("Ctrl+Shift+K")
            .with_icon(IconKind::Key)
            .with_keywords(["kyber", "chiavi", "rotate"]),
        CommandAction::new("audit", "Apri il registro di audit")
            .with_category("Sicurezza")
            .with_icon(IconKind::ShieldCheck)
            .with_keywords(["log", "audit"]),
        CommandAction::new("share", "Condividi il report")
            .with_description("Copia il link del report settimanale")
            .with_category("Azioni")
            .with_shortcut("Ctrl+L")
            .with_icon(IconKind::Share2),
        CommandAction::new("settings", "Impostazioni")
            .with_category("Sistema")
            .with_shortcut("Ctrl+,")
            .with_icon(IconKind::Settings)
            .with_keywords(["preferenze", "config"]),
    ]
}

/// Printed before the `CommandPalette {` block: the same signals and data the live demo uses.
const PRELUDE: &str = r#"let mut opened = use_signal(|| false);
let mut toast = use_notification();
let actions = vec![
    CommandAction::new("edge-07", "Apri edge-07")
        .with_description("Francoforte · 12 ms · 4 peer")
        .with_category("Nodi")
        .with_icon(IconKind::Server)
        .with_keywords(["frankfurt", "nodo"]),
    CommandAction::new("core-02", "Apri core-02")
        .with_description("Milano · 8 ms · 9 peer")
        .with_category("Nodi")
        .with_icon(IconKind::Server)
        .with_keywords(["milano", "nodo"]),
    CommandAction::new("rotate", "Ruota le chiavi Kyber")
        .with_description("Genera e distribuisce nuove chiavi su tutta la mesh")
        .with_category("Sicurezza")
        .with_shortcut("Ctrl+Shift+K")
        .with_icon(IconKind::Key)
        .with_keywords(["kyber", "chiavi", "rotate"]),
    CommandAction::new("audit", "Apri il registro di audit")
        .with_category("Sicurezza")
        .with_icon(IconKind::ShieldCheck)
        .with_keywords(["log", "audit"]),
    CommandAction::new("share", "Condividi il report")
        .with_description("Copia il link del report settimanale")
        .with_category("Azioni")
        .with_shortcut("Ctrl+L")
        .with_icon(IconKind::Share2),
    CommandAction::new("settings", "Impostazioni")
        .with_category("Sistema")
        .with_shortcut("Ctrl+,")
        .with_icon(IconKind::Settings)
        .with_keywords(["preferenze", "config"]),
];

// Trigger nella barra: Button { onclick: move |_| opened.set(true), "Cerca comandi" }"#;

/// Stateful demo: a console toolbar whose search button opens the palette.
#[component]
fn PaletteDemo(knobs: Knobs) -> Element {
    let mut opened = use_signal(|| false);
    let mut toast = use_notification();
    rsx! {
        Card {
            Group { justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
                Stack { gap: SpacingToken::Xs,
                    Heading { order: HeadingOrder::H4, "Shadowfenix Mesh" }
                    Text { size: TextSize::Xs, dimmed: true, "24 nodi · 3 regioni" }
                }
                Button {
                    variant: ButtonVariant::Secondary,
                    left_icon: Some(rsx! {
                        Icon { kind: IconKind::Search, size: 15 }
                    }),
                    onclick: move |_| opened.set(true),
                    "Cerca comandi"
                }
            }
        }
        CommandPalette {
            opened,
            actions: console_actions(),
            placeholder: knobs.text("placeholder"),
            close_on_select: knobs.bool("close_on_select"),
            on_select: move |action: CommandAction| {
                toast.success(format!("Eseguito: {}", action.title));
            },
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        PaletteDemo { knobs: k.clone() }
    }
}

#[component]
fn ThemeActions() -> Element {
    let mut opened = use_signal(|| false);
    let mut theme = use_theme();
    rsx! {
        Group { gap: SpacingToken::Sm, align: AlignItems::Center,
            Button {
                variant: ButtonVariant::Secondary,
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Palette, size: 15 }
                }),
                onclick: move |_| opened.set(true),
                "Cambia accento"
            }
            Text { size: TextSize::Xs, dimmed: true, "Scegli un preset: cambia tutta la pagina." }
        }
        CommandPalette {
            opened,
            placeholder: "Cerca un preset",
            actions: vec![
                CommandAction::new("violet", "Ultraviolet").with_category("Tema").with_icon(IconKind::Sparkles),
                CommandAction::new("crimson", "Crimson").with_category("Tema").with_icon(IconKind::Flame),
                CommandAction::new("cyan", "Cyan").with_category("Tema").with_icon(IconKind::Radio),
                CommandAction::new("emerald", "Emerald").with_category("Tema").with_icon(IconKind::ShieldCheck),
            ],
            on_select: move |action: CommandAction| {
                let color = match action.id.as_str() {
                    "crimson" => PrimaryColor::Crimson,
                    "cyan" => PrimaryColor::Cyan,
                    "emerald" => PrimaryColor::Emerald,
                    _ => PrimaryColor::Violet,
                };
                theme.write().apply_primary_color(color);
            },
        }
    }
}
const THEME_ACTIONS_CODE: &str = r#"let mut opened = use_signal(|| false);
let mut theme = use_theme();

Group { gap: SpacingToken::Sm, align: AlignItems::Center,
    Button {
        variant: ButtonVariant::Secondary,
        left_icon: Some(rsx! {
            Icon { kind: IconKind::Palette, size: 15 }
        }),
        onclick: move |_| opened.set(true),
        "Cambia accento"
    }
    Text { size: TextSize::Xs, dimmed: true, "Scegli un preset: cambia tutta la pagina." }
}
CommandPalette {
    opened,
    placeholder: "Cerca un preset",
    actions: vec![
        CommandAction::new("violet", "Ultraviolet").with_category("Tema").with_icon(IconKind::Sparkles),
        CommandAction::new("crimson", "Crimson").with_category("Tema").with_icon(IconKind::Flame),
        CommandAction::new("cyan", "Cyan").with_category("Tema").with_icon(IconKind::Radio),
        CommandAction::new("emerald", "Emerald").with_category("Tema").with_icon(IconKind::ShieldCheck),
    ],
    on_select: move |action: CommandAction| {
        let color = match action.id.as_str() {
            "crimson" => PrimaryColor::Crimson,
            "cyan" => PrimaryColor::Cyan,
            "emerald" => PrimaryColor::Emerald,
            _ => PrimaryColor::Violet,
        };
        theme.write().apply_primary_color(color);
    },
}"#;

#[component]
fn MultiPick() -> Element {
    let mut opened = use_signal(|| false);
    let mut toast = use_notification();
    rsx! {
        Group { gap: SpacingToken::Sm, align: AlignItems::Center,
            Button {
                variant: ButtonVariant::Secondary,
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Server, size: 15 }
                }),
                onclick: move |_| opened.set(true),
                "Aggiungi nodi al gruppo"
            }
            Text { size: TextSize::Xs, dimmed: true, "Resta aperta: Esc per chiudere." }
        }
        CommandPalette {
            opened,
            close_on_select: false,
            placeholder: "Cerca un nodo",
            actions: vec![
                CommandAction::new("edge-07", "edge-07").with_description("Francoforte").with_icon(IconKind::Server),
                CommandAction::new("edge-11", "edge-11").with_description("Parigi").with_icon(IconKind::Server),
                CommandAction::new("core-02", "core-02").with_description("Milano").with_icon(IconKind::Server),
                CommandAction::new("relay-04", "relay-04").with_description("Madrid").with_icon(IconKind::Radio),
            ],
            on_select: move |action: CommandAction| {
                toast.info(format!("{} aggiunto al gruppo", action.title));
            },
        }
    }
}
const MULTI_PICK_CODE: &str = r#"let mut opened = use_signal(|| false);
let mut toast = use_notification();

Group { gap: SpacingToken::Sm, align: AlignItems::Center,
    Button {
        variant: ButtonVariant::Secondary,
        left_icon: Some(rsx! {
            Icon { kind: IconKind::Server, size: 15 }
        }),
        onclick: move |_| opened.set(true),
        "Aggiungi nodi al gruppo"
    }
    Text { size: TextSize::Xs, dimmed: true, "Resta aperta: Esc per chiudere." }
}
CommandPalette {
    opened,
    close_on_select: false,
    placeholder: "Cerca un nodo",
    actions: vec![
        CommandAction::new("edge-07", "edge-07").with_description("Francoforte").with_icon(IconKind::Server),
        CommandAction::new("edge-11", "edge-11").with_description("Parigi").with_icon(IconKind::Server),
        CommandAction::new("core-02", "core-02").with_description("Milano").with_icon(IconKind::Server),
        CommandAction::new("relay-04", "relay-04").with_description("Madrid").with_icon(IconKind::Radio),
    ],
    on_select: move |action: CommandAction| {
        toast.info(format!("{} aggiunto al gruppo", action.title));
    },
}"#;

#[component]
fn InContext() -> Element {
    let mut opened = use_signal(|| false);
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Nodi della regione EU".to_string()),
            subtitle: Some("Aggiornato 2 minuti fa".to_string()),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button {
                        variant: ButtonVariant::Ghost,
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Search, size: 15 }
                        }),
                        onclick: move |_| opened.set(true),
                        "Azioni rapide"
                    }
                    Button { variant: ButtonVariant::Primary, "Distribuisci" }
                }
            }),
            Stack { gap: SpacingToken::Sm,
                Group { justify: JustifyContent::SpaceBetween,
                    Text { "edge-07 · Francoforte" }
                    Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
                }
                Group { justify: JustifyContent::SpaceBetween,
                    Text { "core-02 · Milano" }
                    Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
                }
                Group { justify: JustifyContent::SpaceBetween,
                    Text { "relay-04 · Madrid" }
                    Badge { label: "Degradato".to_string(), variant: BadgeVariant::Warning, dot: true }
                }
            }
        }
        CommandPalette {
            opened,
            placeholder: "Cosa vuoi fare con la regione EU?",
            actions: vec![
                CommandAction::new("restart", "Riavvia relay-04")
                    .with_description("Latenza oltre 120 ms da 10 minuti")
                    .with_category("Suggerite")
                    .with_icon(IconKind::Radio),
                CommandAction::new("drain", "Svuota edge-07")
                    .with_description("Sposta i peer sugli altri nodi")
                    .with_category("Nodi")
                    .with_icon(IconKind::Server),
                CommandAction::new("rotate", "Ruota le chiavi della regione")
                    .with_category("Sicurezza")
                    .with_shortcut("Ctrl+Shift+K")
                    .with_icon(IconKind::Key),
                CommandAction::new("export", "Esporta metriche CSV")
                    .with_category("Azioni")
                    .with_icon(IconKind::Download),
            ],
            on_select: move |action: CommandAction| {
                toast.success(format!("Avviato: {}", action.title));
            },
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut opened = use_signal(|| false);
let mut toast = use_notification();

Card {
    title: Some("Nodi della regione EU".to_string()),
    subtitle: Some("Aggiornato 2 minuti fa".to_string()),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button {
                variant: ButtonVariant::Ghost,
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Search, size: 15 }
                }),
                onclick: move |_| opened.set(true),
                "Azioni rapide"
            }
            Button { variant: ButtonVariant::Primary, "Distribuisci" }
        }
    }),
    Stack { gap: SpacingToken::Sm,
        Group { justify: JustifyContent::SpaceBetween,
            Text { "edge-07 · Francoforte" }
            Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
        }
        Group { justify: JustifyContent::SpaceBetween,
            Text { "core-02 · Milano" }
            Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
        }
        Group { justify: JustifyContent::SpaceBetween,
            Text { "relay-04 · Madrid" }
            Badge { label: "Degradato".to_string(), variant: BadgeVariant::Warning, dot: true }
        }
    }
}
CommandPalette {
    opened,
    placeholder: "Cosa vuoi fare con la regione EU?",
    actions: vec![
        CommandAction::new("restart", "Riavvia relay-04")
            .with_description("Latenza oltre 120 ms da 10 minuti")
            .with_category("Suggerite")
            .with_icon(IconKind::Radio),
        CommandAction::new("drain", "Svuota edge-07")
            .with_description("Sposta i peer sugli altri nodi")
            .with_category("Nodi")
            .with_icon(IconKind::Server),
        CommandAction::new("rotate", "Ruota le chiavi della regione")
            .with_category("Sicurezza")
            .with_shortcut("Ctrl+Shift+K")
            .with_icon(IconKind::Key),
        CommandAction::new("export", "Esporta metriche CSV")
            .with_category("Azioni")
            .with_icon(IconKind::Download),
    ],
    on_select: move |action: CommandAction| {
        toast.success(format!("Avviato: {}", action.title));
    },
}"#;

const PALETTE_PROPS: &[PropDoc] = &[
    prop("opened", "Signal<bool>", "-", "Apre e chiude la palette; Esc, click sullo sfondo e selezione la chiudono."),
    prop("actions", "Vec<CommandAction>", "vec![]", "Azioni filtrate per titolo, descrizione, categoria e parole chiave."),
    prop("placeholder", "String", "\"\"", "Segnaposto del campo; vuoto usa Locale command_placeholder."),
    prop("on_select", "Option<EventHandler<CommandAction>>", "None", "Azione confermata con click o Invio."),
    prop("close_on_select", "bool", "true", "Chiude la palette e azzera la ricerca dopo la selezione."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul pannello."),
];
const ACTION_PROPS: &[PropDoc] = &[
    prop("CommandAction::new(id, title)", "fn", "-", "Costruttore: id univoco e titolo visibile."),
    prop(".with_description(..)", "fn", "-", "Riga secondaria sotto il titolo."),
    prop(".with_category(..)", "fn", "-", "Intestazione di gruppo; le azioni consecutive con la stessa categoria sono raggruppate."),
    prop(".with_shortcut(..)", "fn", "-", "Scorciatoia mostrata come Kbd (solo visiva)."),
    prop(".with_icon(IconKind)", "fn", "-", "Icona della voce."),
    prop(".with_keywords([..])", "fn", "-", "Parole chiave aggiuntive per la ricerca."),
];

pub fn story() -> Story {
    Story {
        slug: "command-palette",
        name: "CommandPalette",
        category: Category::Navigazione,
        features: &["command_palette"],
        icon: IconKind::Search,
        tagline: "Ricerca universale di comandi e destinazioni, raggruppati per categoria e guidati da tastiera.",
        keywords: &["command palette", "palette comandi", "spotlight", "ricerca", "search", "ctrl+k", "launcher"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "CommandPalette",
            controls: vec![
                Control::text("placeholder", "Placeholder", "Cerca nodi, comandi o impostazioni")
                    .emit(Emit::Template("placeholder: \"{v}\""))
                    .hint("Vuoto: testo da Locale."),
                Control::bool("close_on_select", "Chiudi alla selezione", true),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &[
                "opened,",
                "actions,",
                "on_select: move |action: CommandAction| {",
                "    toast.success(format!(\"Eseguito: {}\", action.title));",
                "},",
            ],
            children_code: None,
            stage: Stage::Fill,
        }),
        examples: vec![
            Example {
                id: "theme",
                title: "Azioni di tema",
                description: "on_select applica un preset: la palette ricolora tutta la pagina.",
                code: THEME_ACTIONS_CODE,
                render: || rsx! { ThemeActions {} },
                span: Span::Half,
            },
            Example {
                id: "multi",
                title: "Selezione ripetuta",
                description: "close_on_select false: si aggiungono più nodi senza riaprire.",
                code: MULTI_PICK_CODE,
                render: || rsx! { MultiPick {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Azioni rapide su una card di regione, con suggerimenti in testa.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "CommandPalette", rows: PALETTE_PROPS },
            PropGroup { component: "CommandAction", rows: ACTION_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Backdrop (language.md §3.3) + floating panel (§3.2) + items (§3.4)",
            depth: "Raised: pannello flottante sopra lo scrim",
            tokens: &["--sd-scrim", "--sd-backdrop-blur", "--sd-surface-2", "--sd-elev-2", "--sd-tint-1", "--sd-item-h-lg", "--sd-accent-soft", "--sd-text-subtle"],
            notes: "Campo di ricerca senza pozzo nella testata tinta; una sola voce attiva con sd-selected(row) e intestazioni di gruppo in mono 2xs.",
        }),
        a11y: &[
            "Il pannello ha role=dialog e aria-modal=true; il campo riceve il focus all'apertura.",
            "Frecce su e giù spostano la voce attiva, Invio la conferma, Esc chiude.",
            "La lista ha role=listbox e ogni voce role=option con aria-selected sulla voce attiva.",
            "Segnaposto, stato vuoto e legenda tastiera vengono da Locale.",
        ],
        related: &["modals", "select", "navbar", "keytip", "dropdown"],
    }
}
