//! Story: Navbar (feature `navbar`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const BRAND_CODE: &str = r#"brand: rsx! {
        Group { gap: SpacingToken::Sm,
            Icon { kind: IconKind::Layers, size: 18 }
            Text { "Shadowfenix" }
        }
    },"#;

const CHILDREN: &str = r#"Group { gap: SpacingToken::Xs,
    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Prodotto" }
    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Prezzi" }
    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Documentazione" }
    Button { size: ButtonSize::Sm, "Accedi" }
}"#;

fn render(k: &Knobs) -> Element {
    rsx! {
        Navbar {
            brand: rsx! {
                Group { gap: SpacingToken::Sm,
                    Icon { kind: IconKind::Layers, size: 18 }
                    Text { "Shadowfenix" }
                }
            },
            collapsible: k.bool("collapsible"),
            aria_label: k.text("aria_label"),
            Group { gap: SpacingToken::Xs,
                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Prodotto" }
                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Prezzi" }
                Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Documentazione" }
                Button { size: ButtonSize::Sm, "Accedi" }
            }
        }
    }
}

#[component]
fn WithSearch() -> Element {
    let query = use_signal(String::new);
    rsx! {
        Navbar {
            brand: rsx! {
                Group { gap: SpacingToken::Sm,
                    Icon { kind: IconKind::Terminal, size: 18 }
                    Text { "Console" }
                }
            },
            Group { gap: SpacingToken::Sm, wrap: false,
                TextInput {
                    value: query,
                    placeholder: Some("Cerca nodi".to_string()),
                    left_icon: Some(SvgIcon::kind(IconKind::Search)),
                }
                Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm, status: Some(PresenceStatus::Online) }
            }
        }
    }
}
const WITH_SEARCH_CODE: &str = r#"let query = use_signal(String::new);

Navbar {
    brand: rsx! {
        Group { gap: SpacingToken::Sm,
            Icon { kind: IconKind::Terminal, size: 18 }
            Text { "Console" }
        }
    },
    Group { gap: SpacingToken::Sm, wrap: false,
        TextInput {
            value: query,
            placeholder: Some("Cerca nodi".to_string()),
            left_icon: Some(SvgIcon::kind(IconKind::Search)),
        }
        Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm, status: Some(PresenceStatus::Online) }
    }
}"#;

#[component]
fn Minimal() -> Element {
    rsx! {
        Navbar {
            brand: rsx! {
                Text { "Shadow Design" }
            },
            Badge { label: "v0.5.2".to_string(), pill: true }
        }
    }
}
const MINIMAL_CODE: &str = r#"Navbar {
    brand: rsx! {
        Text { "Shadow Design" }
    },
    Badge { label: "v0.5.2".to_string(), pill: true }
}"#;

#[component]
fn InContext() -> Element {
    let mut toast = use_notification();
    rsx! {
        Stack { gap: SpacingToken::None,
            Navbar {
                collapsible: true,
                brand: rsx! {
                    Group { gap: SpacingToken::Sm,
                        Icon { kind: IconKind::ShieldCheck, size: 18 }
                        Text { "Shadowfenix Mesh" }
                    }
                },
                Group { gap: SpacingToken::Xs,
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Funzioni" }
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Sicurezza" }
                    Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Clienti" }
                    Button {
                        size: ButtonSize::Sm,
                        onclick: move |_| {
                            toast.success("Prova gratuita attivata per 14 giorni");
                        },
                        "Prova gratis"
                    }
                }
            }
            Stack { gap: SpacingToken::Sm, align: AlignItems::Start, class: "p-6",
                Badge { label: "Nuovo · Kyber-1024".to_string(), variant: BadgeVariant::Primary, pill: true }
                Heading { order: HeadingOrder::H3, "La tua rete privata, ovunque." }
                Text { dimmed: true, "Nodi cifrati end-to-end che si sincronizzano anche offline." }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let mut toast = use_notification();

Stack { gap: SpacingToken::None,
    Navbar {
        collapsible: true,
        brand: rsx! {
            Group { gap: SpacingToken::Sm,
                Icon { kind: IconKind::ShieldCheck, size: 18 }
                Text { "Shadowfenix Mesh" }
            }
        },
        Group { gap: SpacingToken::Xs,
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Funzioni" }
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Sicurezza" }
            Button { variant: ButtonVariant::Ghost, size: ButtonSize::Sm, "Clienti" }
            Button {
                size: ButtonSize::Sm,
                onclick: move |_| {
                    toast.success("Prova gratuita attivata per 14 giorni");
                },
                "Prova gratis"
            }
        }
    }
    Stack { gap: SpacingToken::Sm, align: AlignItems::Start, class: "p-6",
        Badge { label: "Nuovo · Kyber-1024".to_string(), variant: BadgeVariant::Primary, pill: true }
        Heading { order: HeadingOrder::H3, "La tua rete privata, ovunque." }
        Text { dimmed: true, "Nodi cifrati end-to-end che si sincronizzano anche offline." }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("brand", "Element", "-", "Marchio a sinistra: logo, nome, versione."),
    prop("children", "Element", "-", "Link e azioni a destra."),
    prop("collapsible", "bool", "false", "Sotto 640px le azioni finiscono in un menu aperto da un pulsante."),
    prop("aria_label", "String", "\"\" (da Locale)", "Nome del landmark; vuoto usa navbar_label."),
    prop("toggle_aria_label", "Option<String>", "None", "Etichetta del pulsante mobile; senza, Apri / Chiudi da Locale."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "navbar",
        name: "Navbar",
        category: Category::Layout,
        features: &["navbar"],
        icon: IconKind::Menu,
        tagline: "Barra superiore con marchio, link e azioni, che su mobile si raccoglie in un menu.",
        keywords: &["navbar", "barra", "header", "navigazione", "topbar", "menu mobile"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Navbar",
            controls: vec![
                Control::bool("collapsible", "Menu mobile", false).hint("Il pulsante appare sotto i 640px."),
                Control::text("aria_label", "Nome del landmark", "")
                    .emit(Emit::Template("aria_label: \"{v}\".to_string()"))
                    .hint("Vuoto: \"Principale\" dal Locale."),
            ],
            render,
            prelude_code: None,
            fixed_code: &[BRAND_CODE],
            children_code: Some(CHILDREN),
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "search",
                title: "Con ricerca",
                description: "Campo di ricerca e avatar dell'utente.",
                code: WITH_SEARCH_CODE,
                render: || rsx! { WithSearch {} },
                span: Span::Half,
            },
            Example {
                id: "minimal",
                title: "Minima",
                description: "Solo marchio e versione.",
                code: MINIMAL_CODE,
                render: || rsx! { Minimal {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Testata di una landing page; a 390px le voci passano nel menu.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Navbar", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Container sd-surface(1) (§3.1); menu mobile come floating panel (§3.2)",
            depth: "Flat-lit (barra) · Raised (menu mobile)",
            tokens: &["--sd-surface-1", "--sd-surface-2", "--sd-border", "--sd-specular", "--sd-elev-2", "--sd-space-4"],
            notes: "Barra sticky con z('header'); il menu mobile entra con sd-pop-in, mai a tutta pagina.",
        }),
        a11y: &[
            "Rende un landmark <nav> con aria-label (da Locale se assente).",
            "Il pulsante mobile ha aria-expanded e aria-controls verso il contenitore delle azioni.",
            "Esc chiude il menu mobile e riporta il focus sul pulsante.",
            "L'etichetta del pulsante passa da \"Apri\" a \"Chiudi\" menu di navigazione.",
        ],
        related: &["app-shell", "burger", "navigation-menu", "menubar"],
    }
}
