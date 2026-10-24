//! Story: NavigationMenu (feature `navigation_menu`). Data-driven component: no knobs would
//! change the code meaningfully, so it ships examples only (BATCH_GUIDE §4.4).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

#[component]
fn MegaMenu() -> Element {
    let mut toast = use_notification();
    let items = vec![
        NavigationMenuItem::new(
            "Prodotto",
            vec![
                NavigationMenuSection {
                    heading: Some("Piattaforma".to_string()),
                    links: vec![
                        NavigationLinkItem::new("Mesh Gateway")
                            .with_description("Instradamento cifrato tra i nodi")
                            .with_icon(IconKind::Radio)
                            .with_badge("Nuovo"),
                        NavigationLinkItem::new("Telemetria")
                            .with_description("Metriche e grafici in tempo reale")
                            .with_icon(IconKind::Signal),
                    ],
                },
                NavigationMenuSection {
                    heading: Some("Sicurezza".to_string()),
                    links: vec![
                        NavigationLinkItem::new("Accesso zero trust")
                            .with_description("Verifica continua di identità e permessi")
                            .with_icon(IconKind::ShieldCheck),
                        NavigationLinkItem::new("Gestione chiavi")
                            .with_description("Rotazione automatica ogni 24 ore")
                            .with_icon(IconKind::Key),
                    ],
                },
            ],
        )
        .with_preview(NavigationPreviewCard {
            title: "Shadowfenix 0.5".to_string(),
            description: "Il nuovo motore dei temi: un hex ricolora tutta l'interfaccia.".to_string(),
            cta_label: Some("Leggi le novità".to_string()),
            image_url: None,
            on_cta_click: Some(EventHandler::new(move |_| {
                toast.info("Apertura delle note di rilascio");
            })),
        }),
        NavigationMenuItem::new(
            "Risorse",
            vec![NavigationMenuSection {
                heading: None,
                links: vec![
                    NavigationLinkItem::new("Documentazione").with_icon(IconKind::Code),
                    NavigationLinkItem::new("Changelog").with_icon(IconKind::Clock),
                    NavigationLinkItem::new("Stato del servizio").with_icon(IconKind::Server),
                ],
            }],
        ),
    ];
    rsx! {
        NavigationMenu {
            items,
            on_select: move |link: NavigationLinkItem| {
                toast.info(format!("Vai a {}", link.title));
            },
        }
    }
}
const MEGA_MENU_CODE: &str = r#"let mut toast = use_notification();
let items = vec![
    NavigationMenuItem::new(
        "Prodotto",
        vec![
            NavigationMenuSection {
                heading: Some("Piattaforma".to_string()),
                links: vec![
                    NavigationLinkItem::new("Mesh Gateway")
                        .with_description("Instradamento cifrato tra i nodi")
                        .with_icon(IconKind::Radio)
                        .with_badge("Nuovo"),
                    NavigationLinkItem::new("Telemetria")
                        .with_description("Metriche e grafici in tempo reale")
                        .with_icon(IconKind::Signal),
                ],
            },
            NavigationMenuSection {
                heading: Some("Sicurezza".to_string()),
                links: vec![
                    NavigationLinkItem::new("Accesso zero trust")
                        .with_description("Verifica continua di identità e permessi")
                        .with_icon(IconKind::ShieldCheck),
                    NavigationLinkItem::new("Gestione chiavi")
                        .with_description("Rotazione automatica ogni 24 ore")
                        .with_icon(IconKind::Key),
                ],
            },
        ],
    )
    .with_preview(NavigationPreviewCard {
        title: "Shadowfenix 0.5".to_string(),
        description: "Il nuovo motore dei temi: un hex ricolora tutta l'interfaccia.".to_string(),
        cta_label: Some("Leggi le novità".to_string()),
        image_url: None,
        on_cta_click: Some(EventHandler::new(move |_| {
            toast.info("Apertura delle note di rilascio");
        })),
    }),
    NavigationMenuItem::new(
        "Risorse",
        vec![NavigationMenuSection {
            heading: None,
            links: vec![
                NavigationLinkItem::new("Documentazione").with_icon(IconKind::Code),
                NavigationLinkItem::new("Changelog").with_icon(IconKind::Clock),
                NavigationLinkItem::new("Stato del servizio").with_icon(IconKind::Server),
            ],
        }],
    ),
];

NavigationMenu {
    items,
    on_select: move |link: NavigationLinkItem| {
        toast.info(format!("Vai a {}", link.title));
    },
}"#;

#[component]
fn Simple() -> Element {
    rsx! {
        NavigationMenu {
            items: vec![
                NavigationMenuItem::new(
                    "Team",
                    vec![NavigationMenuSection {
                        heading: None,
                        links: vec![
                            NavigationLinkItem::new("Membri").with_description("12 persone in 3 regioni"),
                            NavigationLinkItem::new("Ruoli").with_description("Admin, operatori, revisori"),
                        ],
                    }],
                ),
                NavigationMenuItem::new(
                    "Fatturazione",
                    vec![NavigationMenuSection {
                        heading: None,
                        links: vec![NavigationLinkItem::new("Piano").with_badge("Team"), NavigationLinkItem::new("Fatture")],
                    }],
                ),
            ],
        }
    }
}
const SIMPLE_CODE: &str = r#"NavigationMenu {
    items: vec![
        NavigationMenuItem::new(
            "Team",
            vec![NavigationMenuSection {
                heading: None,
                links: vec![
                    NavigationLinkItem::new("Membri").with_description("12 persone in 3 regioni"),
                    NavigationLinkItem::new("Ruoli").with_description("Admin, operatori, revisori"),
                ],
            }],
        ),
        NavigationMenuItem::new(
            "Fatturazione",
            vec![NavigationMenuSection {
                heading: None,
                links: vec![NavigationLinkItem::new("Piano").with_badge("Team"), NavigationLinkItem::new("Fatture")],
            }],
        ),
    ],
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        Navbar {
            brand: rsx! {
                Group { gap: SpacingToken::Sm,
                    Icon { kind: IconKind::Layers, size: 18 }
                    Text { "Shadowfenix" }
                }
            },
            Group { gap: SpacingToken::Sm,
                MegaMenu {}
                Button { size: ButtonSize::Sm, "Accedi" }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"// MegaMenu è il NavigationMenu dell'esempio "Mega-menu".
Navbar {
    brand: rsx! {
        Group { gap: SpacingToken::Sm,
            Icon { kind: IconKind::Layers, size: 18 }
            Text { "Shadowfenix" }
        }
    },
    Group { gap: SpacingToken::Sm,
        MegaMenu {}
        Button { size: ButtonSize::Sm, "Accedi" }
    }
}"#;

const MENU_PROPS: &[PropDoc] = &[
    prop("items", "Vec<NavigationMenuItem>", "-", "Voci della barra, ognuna con le sue sezioni."),
    prop("on_select", "Option<EventHandler<NavigationLinkItem>>", "None", "Link scelto; il pannello si chiude."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const ITEM_PROPS: &[PropDoc] = &[
    prop("NavigationMenuItem::new(label, sections)", "fn", "-", "Voce della barra con le sue colonne."),
    prop(".with_preview(NavigationPreviewCard)", "fn", "-", "Scheda in evidenza a destra del pannello."),
    prop("NavigationMenuSection { heading, links }", "struct", "-", "Colonna con titolo opzionale e link."),
];
const LINK_PROPS: &[PropDoc] = &[
    prop("NavigationLinkItem::new(title)", "fn", "-", "Costruttore."),
    prop(".with_description(..)", "fn", "-", "Riga secondaria."),
    prop(".with_icon(IconKind)", "fn", "-", "Icona a sinistra."),
    prop(".with_badge(..)", "fn", "-", "Etichetta accanto al titolo."),
    prop(".with_href(..)", "fn", "-", "Destinazione, passata a on_select."),
];
const PREVIEW_PROPS: &[PropDoc] = &[
    prop("title / description", "String", "-", "Titolo e testo della scheda."),
    prop("cta_label", "Option<String>", "None", "Pulsante della scheda."),
    prop("image_url", "Option<String>", "None", "Immagine in alto."),
    prop("on_cta_click", "Option<EventHandler<()>>", "None", "Click sul pulsante."),
];

pub fn story() -> Story {
    Story {
        slug: "navigation-menu",
        name: "NavigationMenu",
        category: Category::Navigazione,
        features: &["navigation_menu"],
        icon: IconKind::Menu,
        tagline: "Mega-menu per siti e prodotti con molte sezioni: colonne di link e una scheda in evidenza.",
        keywords: &["navigation menu", "mega menu", "menu", "navigazione", "header", "link"],
        status: Status::Stable,
        playground: None,
        examples: vec![
            Example {
                id: "mega",
                title: "Mega-menu",
                description: "Due colonne di link con icone e una scheda in evidenza.",
                code: MEGA_MENU_CODE,
                render: || rsx! { MegaMenu {} },
                span: Span::Full,
            },
            Example {
                id: "simple",
                title: "Semplice",
                description: "Una sola colonna, senza titoli né icone.",
                code: SIMPLE_CODE,
                render: || rsx! { Simple {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Il mega-menu dentro la Navbar di un sito.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "NavigationMenu", rows: MENU_PROPS },
            PropGroup { component: "NavigationMenuItem", rows: ITEM_PROPS },
            PropGroup { component: "NavigationLinkItem", rows: LINK_PROPS },
            PropGroup { component: "NavigationPreviewCard", rows: PREVIEW_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Track di item (§3.4) + floating panel (§3.2)",
            depth: "Well (barra) · Raised (pannello)",
            tokens: &["--sd-well", "--sd-item-h", "--sd-accent-soft", "--sd-surface-2", "--sd-elev-2", "--sd-radius-lg", "--sd-space-4"],
            notes: "Il trigger aperto prende sd-selected; il pannello entra con sd-pop-in e le voci sono sd-item.",
        }),
        a11y: &[
            "I trigger sono <button> nella <nav>, con aria-haspopup e aria-expanded.",
            "Esc chiude il pannello aperto; un click sul velo lo chiude.",
            "Ogni link è un <button> raggiungibile con Tab e attivabile con Invio o Spazio.",
            "Il titolo \"In evidenza\" della scheda viene dal Locale.",
        ],
        related: &["navbar", "menubar", "dropdown", "nav-link"],
    }
}
