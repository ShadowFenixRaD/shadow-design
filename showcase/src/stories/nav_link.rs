//! Story: NavLink and SidebarMenu (feature `nav_link`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const BADGE_VARIANTS: [Option<BadgeVariant>; 3] = [None, Some(BadgeVariant::Primary), Some(BadgeVariant::Danger)];
const BADGE_VARIANT_OPTS: &[ChoiceOption] = &[
    opt("Default", "None"),
    opt("Primary", "Some(BadgeVariant::Primary)"),
    opt("Danger", "Some(BadgeVariant::Danger)"),
];

fn render(k: &Knobs) -> Element {
    rsx! {
        NavLink {
            label: k.text("label"),
            description: Some(k.text("description")).filter(|d| !d.is_empty()),
            icon: k.bool("icon").then(|| SvgIcon::kind(IconKind::Server)),
            badge: Some(k.text("badge")).filter(|b| !b.is_empty()),
            badge_variant: k.pick("badge_variant", &BADGE_VARIANTS),
            active: k.bool("active"),
            disabled: k.bool("disabled"),
        }
    }
}

#[component]
fn Nested() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Xs,
            NavLink { label: "Panoramica".to_string(), icon: Some(SvgIcon::kind(IconKind::Layers)) }
            NavLink {
                label: "Sicurezza".to_string(),
                icon: Some(SvgIcon::kind(IconKind::Shield)),
                default_opened: true,
                NavLink { label: "Chiavi".to_string(), active: true }
                NavLink { label: "Policy".to_string() }
                NavLink { label: "Audit".to_string(), badge: Some("2".to_string()), badge_variant: Some(BadgeVariant::Danger) }
            }
            NavLink { label: "Fatturazione".to_string(), icon: Some(SvgIcon::kind(IconKind::Key)), disabled: true }
        }
    }
}
const NESTED_CODE: &str = r#"Stack { gap: SpacingToken::Xs,
    NavLink { label: "Panoramica".to_string(), icon: Some(SvgIcon::kind(IconKind::Layers)) }
    NavLink {
        label: "Sicurezza".to_string(),
        icon: Some(SvgIcon::kind(IconKind::Shield)),
        default_opened: true,
        NavLink { label: "Chiavi".to_string(), active: true }
        NavLink { label: "Policy".to_string() }
        NavLink { label: "Audit".to_string(), badge: Some("2".to_string()), badge_variant: Some(BadgeVariant::Danger) }
    }
    NavLink { label: "Fatturazione".to_string(), icon: Some(SvgIcon::kind(IconKind::Key)), disabled: true }
}"#;

#[component]
fn Groups() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Sm,
            SidebarMenu { title: Some("Rete".to_string()),
                NavLink { label: "Nodi".to_string(), icon: Some(SvgIcon::kind(IconKind::Server)), active: true }
                NavLink { label: "Peer".to_string(), icon: Some(SvgIcon::kind(IconKind::Share2)) }
            }
            SidebarMenu { title: Some("Archivio".to_string()), collapsible: true, default_opened: false,
                NavLink { label: "Backup".to_string(), icon: Some(SvgIcon::kind(IconKind::Download)) }
                NavLink { label: "Log".to_string(), icon: Some(SvgIcon::kind(IconKind::Terminal)) }
            }
        }
    }
}
const GROUPS_CODE: &str = r#"Stack { gap: SpacingToken::Sm,
    SidebarMenu { title: Some("Rete".to_string()),
        NavLink { label: "Nodi".to_string(), icon: Some(SvgIcon::kind(IconKind::Server)), active: true }
        NavLink { label: "Peer".to_string(), icon: Some(SvgIcon::kind(IconKind::Share2)) }
    }
    SidebarMenu { title: Some("Archivio".to_string()), collapsible: true, default_opened: false,
        NavLink { label: "Backup".to_string(), icon: Some(SvgIcon::kind(IconKind::Download)) }
        NavLink { label: "Log".to_string(), icon: Some(SvgIcon::kind(IconKind::Terminal)) }
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut section = use_signal(|| "profilo".to_string());
    let (title, body) = match section().as_str() {
        "sicurezza" => ("Sicurezza", "Autenticazione a due fattori attiva, ultima rotazione delle chiavi 2 ore fa."),
        "notifiche" => ("Notifiche", "Allarmi in tempo reale via email e push; riepilogo giornaliero alle 08:00."),
        _ => ("Profilo", "Giulia Ferri, Rust core, fuso orario Europa/Roma."),
    };
    rsx! {
        Grid { gutter: GridSpacing::Md,
            GridCol { span: 12, md: 4,
                SidebarMenu { title: Some("Impostazioni".to_string()),
                    NavLink {
                        label: "Profilo".to_string(),
                        icon: Some(SvgIcon::kind(IconKind::Fingerprint)),
                        active: section() == "profilo",
                        on_click: move |_| section.set("profilo".to_string()),
                    }
                    NavLink {
                        label: "Sicurezza".to_string(),
                        description: Some("2FA e chiavi".to_string()),
                        icon: Some(SvgIcon::kind(IconKind::Lock)),
                        active: section() == "sicurezza",
                        on_click: move |_| section.set("sicurezza".to_string()),
                    }
                    NavLink {
                        label: "Notifiche".to_string(),
                        icon: Some(SvgIcon::kind(IconKind::MessageSquare)),
                        badge: Some("3".to_string()),
                        badge_variant: Some(BadgeVariant::Primary),
                        active: section() == "notifiche",
                        on_click: move |_| section.set("notifiche".to_string()),
                    }
                }
            }
            GridCol { span: 12, md: 8,
                Card { title: Some(title.to_string()),
                    Text { size: TextSize::Sm, dimmed: true, "{body}" }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut section = use_signal(|| "profilo".to_string());
let (title, body) = match section().as_str() {
    "sicurezza" => ("Sicurezza", "Autenticazione a due fattori attiva, ultima rotazione delle chiavi 2 ore fa."),
    "notifiche" => ("Notifiche", "Allarmi in tempo reale via email e push; riepilogo giornaliero alle 08:00."),
    _ => ("Profilo", "Giulia Ferri, Rust core, fuso orario Europa/Roma."),
};

Grid { gutter: GridSpacing::Md,
    GridCol { span: 12, md: 4,
        SidebarMenu { title: Some("Impostazioni".to_string()),
            NavLink {
                label: "Profilo".to_string(),
                icon: Some(SvgIcon::kind(IconKind::Fingerprint)),
                active: section() == "profilo",
                on_click: move |_| section.set("profilo".to_string()),
            }
            NavLink {
                label: "Sicurezza".to_string(),
                description: Some("2FA e chiavi".to_string()),
                icon: Some(SvgIcon::kind(IconKind::Lock)),
                active: section() == "sicurezza",
                on_click: move |_| section.set("sicurezza".to_string()),
            }
            NavLink {
                label: "Notifiche".to_string(),
                icon: Some(SvgIcon::kind(IconKind::MessageSquare)),
                badge: Some("3".to_string()),
                badge_variant: Some(BadgeVariant::Primary),
                active: section() == "notifiche",
                on_click: move |_| section.set("notifiche".to_string()),
            }
        }
    }
    GridCol { span: 12, md: 8,
        Card { title: Some(title.to_string()),
            Text { size: TextSize::Sm, dimmed: true, "{body}" }
        }
    }
}"#;

const LINK_PROPS: &[PropDoc] = &[
    prop("label", "String", "-", "Etichetta della voce."),
    prop("description", "Option<String>", "None", "Riga secondaria sotto l'etichetta."),
    prop("icon", "Option<SvgIcon>", "None", "Icona a sinistra."),
    prop("active", "bool", "false", "Voce corrente: sd-selected e aria-current=page."),
    prop("href", "Option<String>", "None", "Rende un <a>; senza href è un <button>."),
    prop("on_click", "Option<EventHandler<MouseEvent>>", "None", "Click sulla voce."),
    prop("badge / badge_variant", "Option<String> / Option<BadgeVariant>", "None", "Contatore a destra e sua variante."),
    prop("opened / default_opened", "Option<Signal<bool>> / bool", "None / false", "Apertura dei figli, controllata o iniziale."),
    prop("disabled", "bool", "false", "Voce non attivabile."),
    prop("children", "Option<Element>", "None", "NavLink annidati, mostrati sotto la voce."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const MENU_PROPS: &[PropDoc] = &[
    prop("title", "Option<String>", "None", "Intestazione del gruppo in mono maiuscolo."),
    prop("collapsible", "bool", "false", "L'intestazione apre e chiude il gruppo."),
    prop("opened / default_opened", "Option<Signal<bool>> / bool", "None / true", "Apertura controllata o iniziale."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "nav-link",
        name: "NavLink",
        category: Category::Navigazione,
        features: &["nav_link"],
        icon: IconKind::Menu,
        tagline: "Voce di navigazione laterale con stato attivo, badge e sottovoci.",
        keywords: &["nav link", "sidebar", "voce di menu", "navigazione laterale", "link attivo", "sottomenu"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "NavLink",
            controls: vec![
                Control::text("label", "Etichetta", "Nodi").emit(Emit::Template("label: \"{v}\".to_string()")),
                Control::text("description", "Descrizione", "12 attivi in 3 regioni")
                    .emit(Emit::Template("description: Some(\"{v}\".to_string())")),
                Control::text("badge", "Badge", "4").emit(Emit::Template("badge: Some(\"{v}\".to_string())")),
                Control::choice("badge_variant", "Variante badge", BADGE_VARIANT_OPTS, 0)
                    .emit(Emit::Template("badge_variant: {v}")),
                Control::bool("icon", "Icona", true).emit(Emit::Template("icon: Some(SvgIcon::kind(IconKind::Server))")),
                Control::bool("active", "Attiva", true),
                Control::bool("disabled", "Disabilitata", false),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "nested",
                title: "Sottovoci",
                description: "Una voce che si apre sui figli, con una voce disabilitata.",
                code: NESTED_CODE,
                render: || rsx! { Nested {} },
                span: Span::Half,
            },
            Example {
                id: "groups",
                title: "SidebarMenu",
                description: "Gruppi con titolo, uno richiudibile.",
                code: GROUPS_CODE,
                render: || rsx! { Groups {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Menu delle impostazioni che cambia il pannello a destra.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "NavLink", rows: LINK_PROPS },
            PropGroup { component: "SidebarMenu", rows: MENU_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Item (§3.4) con selezione sd-selected (§3.7)",
            depth: "Item su superficie del contenitore",
            tokens: &["--sd-item-h", "--sd-radius-sm", "--sd-tint-1", "--sd-text-muted", "--sd-accent-soft", "--sd-accent-strong", "--sd-glow-sm"],
            notes: "Una sola voce attiva per gruppo; i figli rientrano di un passo e usano lo stesso item.",
        }),
        a11y: &[
            "Rende un <a> con href, altrimenti un <button> nativo.",
            "La voce attiva ha aria-current=page.",
            "Una voce disabilitata con href ha aria-disabled e tabindex -1; senza href usa disabled.",
            "Il badge è testo reale letto insieme all'etichetta.",
        ],
        related: &["app-shell", "breadcrumbs", "tabs", "bottom-navigation"],
    }
}
