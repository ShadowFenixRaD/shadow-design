//! Story: BottomNavigation and BottomNavItem (feature `bottom_navigation`). Stateful playground
//! (`BottomNavDemo { knobs }`): the active item lives in a signal.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const BADGES: [Option<&str>; 3] = [None, Some("3"), Some("99+")];
const BADGE_OPTS: &[ChoiceOption] = &[opt("Nessuno", "None"), opt("3", "Some(\"3\".to_string())"), opt("99+", "Some(\"99+\".to_string())")];

const PRELUDE: &str = r#"let mut active = use_signal(|| "home".to_string());"#;

const CHILDREN: &str = r#"BottomNavItem {
    label: "Home".to_string(),
    icon: SvgIcon::kind(IconKind::Box),
    active: active() == "home",
    on_click: move |_| active.set("home".to_string()),
}
BottomNavItem {
    label: "Messaggi".to_string(),
    icon: SvgIcon::kind(IconKind::MessageSquare),
    badge: {badge},
    badge_variant: Some(BadgeVariant::Danger),
    active: active() == "chat",
    on_click: move |_| active.set("chat".to_string()),
}
BottomNavItem {
    label: "Cerca".to_string(),
    icon: SvgIcon::kind(IconKind::Search),
    active: active() == "search",
    on_click: move |_| active.set("search".to_string()),
}
BottomNavItem {
    label: "Profilo".to_string(),
    icon: SvgIcon::kind(IconKind::Fingerprint),
    active: active() == "profile",
    on_click: move |_| active.set("profile".to_string()),
}"#;

#[component]
fn BottomNavDemo(knobs: Knobs) -> Element {
    let mut active = use_signal(|| "home".to_string());
    rsx! {
        BottomNavigation {
            position: Some(BottomNavPosition::Relative),
            border: knobs.bool("border"),
            blur: knobs.bool("blur"),
            safe_area: knobs.bool("safe_area"),
            BottomNavItem {
                label: "Home".to_string(),
                icon: SvgIcon::kind(IconKind::Box),
                active: active() == "home",
                on_click: move |_| active.set("home".to_string()),
            }
            BottomNavItem {
                label: "Messaggi".to_string(),
                icon: SvgIcon::kind(IconKind::MessageSquare),
                badge: knobs.pick("badge", &BADGES).map(str::to_string),
                badge_variant: Some(BadgeVariant::Danger),
                active: active() == "chat",
                on_click: move |_| active.set("chat".to_string()),
            }
            BottomNavItem {
                label: "Cerca".to_string(),
                icon: SvgIcon::kind(IconKind::Search),
                active: active() == "search",
                on_click: move |_| active.set("search".to_string()),
            }
            BottomNavItem {
                label: "Profilo".to_string(),
                icon: SvgIcon::kind(IconKind::Fingerprint),
                active: active() == "profile",
                on_click: move |_| active.set("profile".to_string()),
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        BottomNavDemo { knobs: k.clone() }
    }
}

#[component]
fn ThreeItems() -> Element {
    rsx! {
        BottomNavigation { position: Some(BottomNavPosition::Relative),
            BottomNavItem { label: "Oggi".to_string(), icon: SvgIcon::kind(IconKind::Calendar), active: true }
            BottomNavItem { label: "Attività".to_string(), icon: SvgIcon::kind(IconKind::Clock) }
            BottomNavItem { label: "Impostazioni".to_string(), icon: SvgIcon::kind(IconKind::Settings), disabled: true }
        }
    }
}
const THREE_ITEMS_CODE: &str = r#"BottomNavigation { position: Some(BottomNavPosition::Relative),
    BottomNavItem { label: "Oggi".to_string(), icon: SvgIcon::kind(IconKind::Calendar), active: true }
    BottomNavItem { label: "Attività".to_string(), icon: SvgIcon::kind(IconKind::Clock) }
    BottomNavItem { label: "Impostazioni".to_string(), icon: SvgIcon::kind(IconKind::Settings), disabled: true }
}"#;

#[component]
fn Flat() -> Element {
    rsx! {
        BottomNavigation { position: Some(BottomNavPosition::Relative), border: false, blur: false,
            BottomNavItem { label: "Nodi".to_string(), icon: SvgIcon::kind(IconKind::Server), active: true }
            BottomNavItem { label: "Allarmi".to_string(), icon: SvgIcon::kind(IconKind::AlertTriangle), badge: Some("2".to_string()) }
        }
    }
}
const FLAT_CODE: &str = r#"BottomNavigation { position: Some(BottomNavPosition::Relative), border: false, blur: false,
    BottomNavItem { label: "Nodi".to_string(), icon: SvgIcon::kind(IconKind::Server), active: true }
    BottomNavItem { label: "Allarmi".to_string(), icon: SvgIcon::kind(IconKind::AlertTriangle), badge: Some("2".to_string()) }
}"#;

#[component]
fn InContext() -> Element {
    let mut active = use_signal(|| "chat".to_string());
    rsx! {
        div { class: "sc-demo-frame max-w-sm mx-auto",
            Stack { gap: SpacingToken::Sm,
                Group { justify: JustifyContent::SpaceBetween,
                    Text { "Messaggi" }
                    Badge { label: "2 nuovi".to_string(), variant: BadgeVariant::Primary, pill: true }
                }
                ScrollArea { height: Some("13rem".to_string()), fade_edges: true,
                    List { spacing: ListSpacing::Sm,
                        for (name , text) in [
                            ("Giulia Ferri", "Il nodo edge-07 è tornato online."),
                            ("Marco Lenzi", "Ho caricato il rapporto di audit."),
                            ("Sara Conti", "Possiamo ruotare le chiavi stasera?"),
                            ("Luca Neri", "Backup completato su eu-south."),
                        ]
                        {
                            ListItem { key: "{name}",
                                icon: Some(rsx! {
                                    Avatar { name: Some(name.to_string()), size: AvatarSize::Sm }
                                }),
                                Stack { gap: SpacingToken::None,
                                    Text { size: TextSize::Sm, "{name}" }
                                    Text { size: TextSize::Xs, dimmed: true, "{text}" }
                                }
                            }
                        }
                    }
                }
                BottomNavigation { position: Some(BottomNavPosition::Relative),
                    BottomNavItem {
                        label: "Home".to_string(),
                        icon: SvgIcon::kind(IconKind::Box),
                        active: active() == "home",
                        on_click: move |_| active.set("home".to_string()),
                    }
                    BottomNavItem {
                        label: "Messaggi".to_string(),
                        icon: SvgIcon::kind(IconKind::MessageSquare),
                        badge: Some("2".to_string()),
                        badge_variant: Some(BadgeVariant::Danger),
                        active: active() == "chat",
                        on_click: move |_| active.set("chat".to_string()),
                    }
                    BottomNavItem {
                        label: "Nodi".to_string(),
                        icon: SvgIcon::kind(IconKind::Server),
                        active: active() == "nodes",
                        on_click: move |_| active.set("nodes".to_string()),
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut active = use_signal(|| "chat".to_string());

div { class: "max-w-sm mx-auto",
    Stack { gap: SpacingToken::Sm,
        Group { justify: JustifyContent::SpaceBetween,
            Text { "Messaggi" }
            Badge { label: "2 nuovi".to_string(), variant: BadgeVariant::Primary, pill: true }
        }
        ScrollArea { height: Some("13rem".to_string()), fade_edges: true,
            List { spacing: ListSpacing::Sm,
                for (name , text) in [
                    ("Giulia Ferri", "Il nodo edge-07 è tornato online."),
                    ("Marco Lenzi", "Ho caricato il rapporto di audit."),
                    ("Sara Conti", "Possiamo ruotare le chiavi stasera?"),
                    ("Luca Neri", "Backup completato su eu-south."),
                ]
                {
                    ListItem {
                        icon: Some(rsx! {
                            Avatar { name: Some(name.to_string()), size: AvatarSize::Sm }
                        }),
                        Stack { gap: SpacingToken::None,
                            Text { size: TextSize::Sm, "{name}" }
                            Text { size: TextSize::Xs, dimmed: true, "{text}" }
                        }
                    }
                }
            }
        }
        BottomNavigation { position: Some(BottomNavPosition::Relative),
            BottomNavItem {
                label: "Home".to_string(),
                icon: SvgIcon::kind(IconKind::Box),
                active: active() == "home",
                on_click: move |_| active.set("home".to_string()),
            }
            BottomNavItem {
                label: "Messaggi".to_string(),
                icon: SvgIcon::kind(IconKind::MessageSquare),
                badge: Some("2".to_string()),
                badge_variant: Some(BadgeVariant::Danger),
                active: active() == "chat",
                on_click: move |_| active.set("chat".to_string()),
            }
            BottomNavItem {
                label: "Nodi".to_string(),
                icon: SvgIcon::kind(IconKind::Server),
                active: active() == "nodes",
                on_click: move |_| active.set("nodes".to_string()),
            }
        }
    }
}"#;

const NAV_PROPS: &[PropDoc] = &[
    prop("position", "Option<BottomNavPosition>", "None (Fixed)", "Fixed (in fondo alla viewport), Absolute (al genitore) o Relative (nel flusso)."),
    prop("safe_area", "bool", "true", "Padding per l'area sicura dei telefoni (env(safe-area-inset-bottom))."),
    prop("border", "bool", "true", "Divisore superiore."),
    prop("blur", "bool", "true", "Sfondo sfocato sotto la barra."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
    prop("children", "Element", "-", "Da tre a cinque BottomNavItem."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("label", "String", "-", "Etichetta sotto l'icona."),
    prop("icon", "SvgIcon", "-", "Icona della destinazione."),
    prop("active", "bool", "false", "Destinazione corrente: sd-selected e aria-current=page."),
    prop("badge / badge_variant", "Option<String> / Option<BadgeVariant>", "None", "Contatore sull'icona e sua variante."),
    prop("href", "Option<String>", "None", "Rende un <a>; senza href è un <button>."),
    prop("on_click", "Option<EventHandler<MouseEvent>>", "None", "Click sulla voce."),
    prop("disabled", "bool", "false", "Voce non attivabile."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "bottom-navigation",
        name: "BottomNavigation",
        category: Category::Navigazione,
        features: &["bottom_navigation"],
        icon: IconKind::Menu,
        tagline: "Barra inferiore per app mobili con tre-cinque destinazioni principali.",
        keywords: &["bottom navigation", "barra inferiore", "tab bar", "mobile", "navigazione", "app"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "BottomNavigation",
            controls: vec![
                Control::choice("badge", "Badge su Messaggi", BADGE_OPTS, 1).emit(Emit::Skip),
                Control::bool("border", "Divisore", true),
                Control::bool("blur", "Sfocatura", true),
                Control::bool("safe_area", "Area sicura", true),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["position: Some(BottomNavPosition::Relative),"],
            children_code: Some(CHILDREN),
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "three",
                title: "Tre destinazioni",
                description: "Con una voce disabilitata.",
                code: THREE_ITEMS_CODE,
                render: || rsx! { ThreeItems {} },
                span: Span::Half,
            },
            Example {
                id: "flat",
                title: "Senza bordo né sfocatura",
                description: "Per barre su sfondo pieno.",
                code: FLAT_CODE,
                render: || rsx! { Flat {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Schermata messaggi di un'app mobile.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "BottomNavigation", rows: NAV_PROPS },
            PropGroup { component: "BottomNavItem", rows: ITEM_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Container (§3.1) con item selezionato sd-selected (§3.7)",
            depth: "Flat-lit (barra) · item",
            tokens: &["--sd-surface-1", "--sd-border", "--sd-backdrop-blur", "--sd-accent-soft", "--sd-accent-strong", "--sd-text-muted", "--sd-glow-sm"],
            notes: "Una sola destinazione attiva; il badge danger segnala, non decora.",
        }),
        a11y: &[
            "Rende un <nav> con aria-label \"Navigazione inferiore principale\" dal Locale.",
            "La voce attiva ha aria-current=page.",
            "Ogni voce è un <a> o un <button> nativo, con l'etichetta sempre visibile.",
            "Le voci disabilitate con href hanno aria-disabled e tabindex -1.",
        ],
        related: &["tabs", "nav-link", "app-shell", "navbar"],
    }
}
