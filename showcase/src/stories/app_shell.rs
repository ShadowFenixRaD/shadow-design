//! Story: AppShell and its five zones (feature `app_shell`). Stateful playground
//! (`ShellDemo { knobs }`): the header burger drives the navbar through a signal.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const PRELUDE: &str = r#"let mut nav_collapsed = use_signal(|| false);
let menu = use_signal(|| true);"#;

const HEADER_CODE: &str = r#"header: Some(rsx! {
        AppShellHeader { height: Some("52px".to_string()),
            Group { gap: SpacingToken::Sm,
                Burger { opened: menu, size: BurgerSize::Sm, onclick: move |_| nav_collapsed.toggle() }
                Text { "Shadowfenix Mesh" }
            }
            Button { size: ButtonSize::Sm, "Nuovo nodo" }
        }
    })"#;
const NAVBAR_CODE: &str = r#"navbar: Some(rsx! {
        AppShellNavbar { width: Some("200px".to_string()),
            Stack { gap: SpacingToken::Xs, class: "p-3",
                NavLink { label: "Panoramica".to_string(), icon: Some(SvgIcon::kind(IconKind::Layers)), active: true }
                NavLink { label: "Nodi".to_string(), icon: Some(SvgIcon::kind(IconKind::Server)), badge: Some("12".to_string()) }
                NavLink { label: "Sicurezza".to_string(), icon: Some(SvgIcon::kind(IconKind::Shield)) }
            }
        }
    })"#;
const ASIDE_CODE: &str = r#"aside: Some(rsx! {
        AppShellAside { width: Some("220px".to_string()),
            Stack { gap: SpacingToken::Sm, class: "p-4",
                Text { "edge-07" }
                Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
                Text { size: TextSize::Sm, dimmed: true, "Latenza 12 ms · 4 peer" }
            }
        }
    })"#;
const FOOTER_CODE: &str = r#"footer: Some(rsx! {
        AppShellFooter {
            Text { size: TextSize::Xs, dimmed: true, "v0.5.2 · tutti i sistemi operativi" }
        }
    })"#;
const CHILDREN: &str = r#"AppShellMain { class: "p-4",
    SimpleGrid { cols: 1, sm: Some(2), spacing: GridSpacing::Sm,
        Card { title: Some("Latenza".to_string()), subtitle: Some("1.2 ms".to_string()) }
        Card { title: Some("Richieste".to_string()), subtitle: Some("4.8M".to_string()) }
    }
}"#;

#[component]
fn ShellDemo(knobs: Knobs) -> Element {
    let mut nav_collapsed = use_signal(|| false);
    let menu = use_signal(|| true);
    rsx! {
        AppShell {
            class: "h-80",
            navbar_collapsed: Some(nav_collapsed),
            header: knobs.bool("header").then(|| rsx! {
                AppShellHeader { height: Some("52px".to_string()),
                    Group { gap: SpacingToken::Sm,
                        Burger { opened: menu, size: BurgerSize::Sm, onclick: move |_| nav_collapsed.toggle() }
                        Text { "Shadowfenix Mesh" }
                    }
                    Button { size: ButtonSize::Sm, "Nuovo nodo" }
                }
            }),
            navbar: knobs.bool("navbar").then(|| rsx! {
                AppShellNavbar { width: Some("200px".to_string()),
                    Stack { gap: SpacingToken::Xs, class: "p-3",
                        NavLink { label: "Panoramica".to_string(), icon: Some(SvgIcon::kind(IconKind::Layers)), active: true }
                        NavLink { label: "Nodi".to_string(), icon: Some(SvgIcon::kind(IconKind::Server)), badge: Some("12".to_string()) }
                        NavLink { label: "Sicurezza".to_string(), icon: Some(SvgIcon::kind(IconKind::Shield)) }
                    }
                }
            }),
            aside: knobs.bool("aside").then(|| rsx! {
                AppShellAside { width: Some("220px".to_string()),
                    Stack { gap: SpacingToken::Sm, class: "p-4",
                        Text { "edge-07" }
                        Badge { label: "Online".to_string(), variant: BadgeVariant::Success, dot: true }
                        Text { size: TextSize::Sm, dimmed: true, "Latenza 12 ms · 4 peer" }
                    }
                }
            }),
            footer: knobs.bool("footer").then(|| rsx! {
                AppShellFooter {
                    Text { size: TextSize::Xs, dimmed: true, "v0.5.2 · tutti i sistemi operativi" }
                }
            }),
            AppShellMain { class: "p-4",
                SimpleGrid { cols: 1, sm: Some(2), spacing: GridSpacing::Sm,
                    Card { title: Some("Latenza".to_string()), subtitle: Some("1.2 ms".to_string()) }
                    Card { title: Some("Richieste".to_string()), subtitle: Some("4.8M".to_string()) }
                }
            }
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        ShellDemo { knobs: k.clone() }
    }
}

#[component]
fn HeaderOnly() -> Element {
    rsx! {
        AppShell {
            class: "h-56",
            header: Some(rsx! {
                AppShellHeader {
                    Text { "Documentazione" }
                    Badge { label: "v0.5".to_string(), pill: true }
                }
            }),
            AppShellMain { class: "p-4",
                Text { size: TextSize::Sm, dimmed: true, "Solo header e contenuto: la forma più semplice, per pagine senza navigazione laterale." }
            }
        }
    }
}
const HEADER_ONLY_CODE: &str = r#"AppShell {
    class: "h-56",
    header: Some(rsx! {
        AppShellHeader {
            Text { "Documentazione" }
            Badge { label: "v0.5".to_string(), pill: true }
        }
    }),
    AppShellMain { class: "p-4",
        Text { size: TextSize::Sm, dimmed: true, "Solo header e contenuto: la forma più semplice, per pagine senza navigazione laterale." }
    }
}"#;

#[component]
fn Inspector() -> Element {
    let mut aside_collapsed = use_signal(|| true);
    rsx! {
        AppShell {
            class: "h-56",
            aside_collapsed: Some(aside_collapsed),
            header: Some(rsx! {
                AppShellHeader {
                    Text { "Nodo edge-07" }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        aria_pressed: Some(!aside_collapsed()),
                        onclick: move |_| aside_collapsed.toggle(),
                        "Dettagli"
                    }
                }
            }),
            aside: Some(rsx! {
                AppShellAside { width: Some("200px".to_string()),
                    Stack { gap: SpacingToken::Xs, class: "p-4",
                        Text { size: TextSize::Sm, "Kyber-1024" }
                        Text { size: TextSize::Sm, dimmed: true, "Chiave ruotata 2 ore fa" }
                    }
                }
            }),
            AppShellMain { class: "p-4",
                Text { size: TextSize::Sm, dimmed: true, "Il pannello destro si apre e si chiude da un Signal." }
            }
        }
    }
}
const INSPECTOR_CODE: &str = r#"let mut aside_collapsed = use_signal(|| true);

AppShell {
    class: "h-56",
    aside_collapsed: Some(aside_collapsed),
    header: Some(rsx! {
        AppShellHeader {
            Text { "Nodo edge-07" }
            Button {
                variant: ButtonVariant::Secondary,
                size: ButtonSize::Sm,
                aria_pressed: Some(!aside_collapsed()),
                onclick: move |_| aside_collapsed.toggle(),
                "Dettagli"
            }
        }
    }),
    aside: Some(rsx! {
        AppShellAside { width: Some("200px".to_string()),
            Stack { gap: SpacingToken::Xs, class: "p-4",
                Text { size: TextSize::Sm, "Kyber-1024" }
                Text { size: TextSize::Sm, dimmed: true, "Chiave ruotata 2 ore fa" }
            }
        }
    }),
    AppShellMain { class: "p-4",
        Text { size: TextSize::Sm, dimmed: true, "Il pannello destro si apre e si chiude da un Signal." }
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut nav_collapsed = use_signal(|| false);
    let menu = use_signal(|| true);
    rsx! {
        AppShell {
            class: "min-h-[460px]",
            navbar_collapsed: Some(nav_collapsed),
            header: Some(rsx! {
                AppShellHeader { height: Some("52px".to_string()),
                    Group { gap: SpacingToken::Sm,
                        Burger { opened: menu, size: BurgerSize::Sm, onclick: move |_| nav_collapsed.toggle() }
                        Text { "Posta operatori" }
                    }
                    Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm, status: Some(PresenceStatus::Online) }
                }
            }),
            navbar: Some(rsx! {
                AppShellNavbar { width: Some("200px".to_string()),
                    Stack { gap: SpacingToken::Xs, class: "p-3",
                        NavLink { label: "In arrivo".to_string(), icon: Some(SvgIcon::kind(IconKind::MessageSquare)), active: true, badge: Some("4".to_string()) }
                        NavLink { label: "Inviati".to_string(), icon: Some(SvgIcon::kind(IconKind::Send)) }
                        NavLink { label: "Archivio".to_string(), icon: Some(SvgIcon::kind(IconKind::Box)) }
                        NavLink { label: "Cestino".to_string(), icon: Some(SvgIcon::kind(IconKind::Trash2)) }
                    }
                }
            }),
            AppShellMain { class: "p-4",
                crate::scenes::InboxPane { compact: true }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut nav_collapsed = use_signal(|| false);
let menu = use_signal(|| true);

AppShell {
    class: "min-h-[460px]",
    navbar_collapsed: Some(nav_collapsed),
    header: Some(rsx! {
        AppShellHeader { height: Some("52px".to_string()),
            Group { gap: SpacingToken::Sm,
                Burger { opened: menu, size: BurgerSize::Sm, onclick: move |_| nav_collapsed.toggle() }
                Text { "Posta operatori" }
            }
            Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm, status: Some(PresenceStatus::Online) }
        }
    }),
    navbar: Some(rsx! {
        AppShellNavbar { width: Some("200px".to_string()),
            Stack { gap: SpacingToken::Xs, class: "p-3",
                NavLink { label: "In arrivo".to_string(), icon: Some(SvgIcon::kind(IconKind::MessageSquare)), active: true, badge: Some("4".to_string()) }
                NavLink { label: "Inviati".to_string(), icon: Some(SvgIcon::kind(IconKind::Send)) }
                NavLink { label: "Archivio".to_string(), icon: Some(SvgIcon::kind(IconKind::Box)) }
                NavLink { label: "Cestino".to_string(), icon: Some(SvgIcon::kind(IconKind::Trash2)) }
            }
        }
    }),
    AppShellMain { class: "p-4",
        InboxPane { compact: true } // scenes/inbox_pane.rs
    }
}"#;

const SHELL_PROPS: &[PropDoc] = &[
    prop("header / navbar / aside / footer", "Option<Element>", "None", "Le quattro zone opzionali attorno al contenuto."),
    prop("children", "Element", "-", "Area centrale (AppShellMain), con scorrimento proprio."),
    prop("navbar_collapsed / aside_collapsed", "Option<Signal<bool>>", "None", "Chiusura controllata dei pannelli laterali; senza, sotto md partono chiusi."),
    prop("fullscreen", "bool", "false", "Altezza 100vh; altrimenti riempie il genitore."),
    prop("class", "String", "\"\"", "Classi aggiuntive (dai un'altezza se non è fullscreen)."),
];
const HEADER_PROPS: &[PropDoc] = &[
    prop("height", "Option<String>", "\"60px\"", "Altezza dell'header."),
    prop("sticky", "bool", "true", "Resta in cima durante lo scorrimento."),
    prop("border", "bool", "true", "Divisore inferiore."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const SIDE_PROPS: &[PropDoc] = &[
    prop("width", "Option<String>", "\"260px\" / \"280px\"", "Larghezza da aperto (navbar / aside)."),
    prop("collapsed", "Option<bool>", "None", "Forza lo stato, ignorando il Signal dell'AppShell."),
    prop("border", "bool", "true", "Divisore verso il contenuto."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const FOOTER_PROPS: &[PropDoc] = &[
    prop("height", "Option<String>", "\"40px\"", "Altezza del footer."),
    prop("border", "bool", "true", "Divisore superiore."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const MAIN_PROPS: &[PropDoc] = &[
    prop("class", "String", "\"\"", "Classi aggiuntive (il padding è a carico tuo)."),
    prop("children", "Element", "-", "Contenuto della vista."),
];

pub fn story() -> Story {
    Story {
        slug: "app-shell",
        name: "AppShell",
        category: Category::Layout,
        features: &["app_shell"],
        icon: IconKind::Box,
        tagline: "Layout applicativo a cinque zone: header, navigazione, contenuto, pannello laterale e footer.",
        keywords: &["app shell", "layout", "sidebar", "header", "footer", "dashboard", "scheletro"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "AppShell",
            controls: vec![
                Control::bool("header", "Header", true).emit(Emit::Template(HEADER_CODE)),
                Control::bool("navbar", "Navbar", true).emit(Emit::Template(NAVBAR_CODE)),
                Control::bool("aside", "Aside", false).emit(Emit::Template(ASIDE_CODE)),
                Control::bool("footer", "Footer", true).emit(Emit::Template(FOOTER_CODE)),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["class: \"h-80\",", "navbar_collapsed: Some(nav_collapsed),"],
            children_code: Some(CHILDREN),
            stage: Stage::Fill,
        }),
        examples: vec![
            Example {
                id: "header-only",
                title: "Header e contenuto",
                description: "La forma minima, senza pannelli.",
                code: HEADER_ONLY_CODE,
                render: || rsx! { HeaderOnly {} },
                span: Span::Half,
            },
            Example {
                id: "inspector",
                title: "Ispettore a destra",
                description: "L'aside si apre da un pulsante dell'header.",
                code: INSPECTOR_CODE,
                render: || rsx! { Inspector {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Una casella di posta con cartelle a sinistra; il burger chiude la navigazione.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "AppShell", rows: SHELL_PROPS },
            PropGroup { component: "AppShellHeader", rows: HEADER_PROPS },
            PropGroup { component: "AppShellNavbar / AppShellAside", rows: SIDE_PROPS },
            PropGroup { component: "AppShellFooter", rows: FOOTER_PROPS },
            PropGroup { component: "AppShellMain", rows: MAIN_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Container sd-surface(1) per le zone (§3.1) + backdrop per i pannelli mobili (§3.3)",
            depth: "Void (corpo) · Flat-lit (zone) · Raised-max (pannelli off-canvas sotto md)",
            tokens: &["--sd-void", "--sd-surface-1", "--sd-border", "--sd-elev-1", "--sd-elev-3", "--sd-scrim", "--sd-backdrop-blur"],
            notes: "Da md in su i pannelli sono agganciati; sotto diventano off-canvas con velo, senza schiacciare il contenuto.",
        }),
        a11y: &[
            "Landmark nativi: header role=banner, main role=main, footer role=contentinfo.",
            "La navbar è role=navigation e l'aside role=complementary, con aria-label da Locale.",
            "Un pannello chiuso è aria-hidden e inert: il suo contenuto esce dal Tab.",
            "Sotto md il velo dietro i pannelli aperti li chiude al click.",
        ],
        related: &["navbar", "nav-link", "burger", "grid", "resizable"],
    }
}
