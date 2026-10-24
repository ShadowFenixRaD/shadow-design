//! Story: Burger (feature `burger`).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const SIZES: [BurgerSize; 3] = [BurgerSize::Sm, BurgerSize::Md, BurgerSize::Lg];
const SIZE_OPTS: &[ChoiceOption] = &[
    opt("Sm", "BurgerSize::Sm"),
    opt("Md", "BurgerSize::Md"),
    opt("Lg", "BurgerSize::Lg"),
];
const COLORS: [Option<&str>; 3] = [None, Some("var(--sd-accent)"), Some("var(--sd-text-muted)")];
const COLOR_OPTS: &[ChoiceOption] = &[
    opt("Testo", "None"),
    opt("Accento", "var(--sd-accent)"),
    opt("Muted", "var(--sd-text-muted)"),
];

const PRELUDE: &str = "let opened = use_signal(|| false);";

/// Stateful demo: the burger owns the `opened` signal.
#[component]
fn BurgerDemo(knobs: Knobs) -> Element {
    let opened = use_signal(|| false);
    rsx! {
        Burger {
            opened,
            size: knobs.pick("size", &SIZES),
            aria_label: knobs.text("aria_label"),
            color: knobs.pick("color", &COLORS).map(str::to_string),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        BurgerDemo { knobs: k.clone() }
    }
}

#[component]
fn Sizes() -> Element {
    let small = use_signal(|| false);
    let medium = use_signal(|| false);
    let large = use_signal(|| false);
    rsx! {
        Group { gap: SpacingToken::Lg, align: AlignItems::Center,
            Burger { opened: small, size: BurgerSize::Sm, aria_label: "Menu piccolo" }
            Burger { opened: medium, size: BurgerSize::Md, aria_label: "Menu medio" }
            Burger { opened: large, size: BurgerSize::Lg, aria_label: "Menu grande" }
        }
    }
}
const SIZES_CODE: &str = r#"let small = use_signal(|| false);
let medium = use_signal(|| false);
let large = use_signal(|| false);

Group { gap: SpacingToken::Lg, align: AlignItems::Center,
    Burger { opened: small, size: BurgerSize::Sm, aria_label: "Menu piccolo" }
    Burger { opened: medium, size: BurgerSize::Md, aria_label: "Menu medio" }
    Burger { opened: large, size: BurgerSize::Lg, aria_label: "Menu grande" }
}"#;

#[component]
fn States() -> Element {
    let closed = use_signal(|| false);
    let open = use_signal(|| true);
    rsx! {
        Group { gap: SpacingToken::Xl, align: AlignItems::Center,
            Stack { gap: SpacingToken::Xs, align: AlignItems::Center,
                Burger { opened: closed, aria_label: "Apri navigazione" }
                Text { size: TextSize::Xs, dimmed: true, "Chiuso" }
            }
            Stack { gap: SpacingToken::Xs, align: AlignItems::Center,
                Burger { opened: open, aria_label: "Chiudi navigazione" }
                Text { size: TextSize::Xs, dimmed: true, "Aperto" }
            }
        }
    }
}
const STATES_CODE: &str = r#"let closed = use_signal(|| false);
let open = use_signal(|| true);

Group { gap: SpacingToken::Xl, align: AlignItems::Center,
    Stack { gap: SpacingToken::Xs, align: AlignItems::Center,
        Burger { opened: closed, aria_label: "Apri navigazione" }
        Text { size: TextSize::Xs, dimmed: true, "Chiuso" }
    }
    Stack { gap: SpacingToken::Xs, align: AlignItems::Center,
        Burger { opened: open, aria_label: "Chiudi navigazione" }
        Text { size: TextSize::Xs, dimmed: true, "Aperto" }
    }
}"#;

#[component]
fn MobileHeader() -> Element {
    let opened = use_signal(|| false);
    rsx! {
        Card {
            Stack { gap: SpacingToken::Md,
                Group { gap: SpacingToken::Sm, align: AlignItems::Center,
                    Burger { opened, aria_label: "Navigazione principale" }
                    Heading { order: HeadingOrder::H4, "Shadowfenix Mesh" }
                }
                if opened() {
                    Stack { gap: SpacingToken::Xs,
                        NavLink { label: "Panoramica".to_string(), icon: Some(SvgIcon::kind(IconKind::Layers)), active: true }
                        NavLink { label: "Nodi".to_string(), icon: Some(SvgIcon::kind(IconKind::Server)), badge: Some("12".to_string()) }
                        NavLink { label: "Sicurezza".to_string(), icon: Some(SvgIcon::kind(IconKind::Shield)) }
                        NavLink { label: "Impostazioni".to_string(), icon: Some(SvgIcon::kind(IconKind::Settings)) }
                    }
                }
            }
        }
    }
}
const MOBILE_HEADER_CODE: &str = r#"let opened = use_signal(|| false);

Card {
    Stack { gap: SpacingToken::Md,
        Group { gap: SpacingToken::Sm, align: AlignItems::Center,
            Burger { opened, aria_label: "Navigazione principale" }
            Heading { order: HeadingOrder::H4, "Shadowfenix Mesh" }
        }
        if opened() {
            Stack { gap: SpacingToken::Xs,
                NavLink { label: "Panoramica".to_string(), icon: Some(SvgIcon::kind(IconKind::Layers)), active: true }
                NavLink { label: "Nodi".to_string(), icon: Some(SvgIcon::kind(IconKind::Server)), badge: Some("12".to_string()) }
                NavLink { label: "Sicurezza".to_string(), icon: Some(SvgIcon::kind(IconKind::Shield)) }
                NavLink { label: "Impostazioni".to_string(), icon: Some(SvgIcon::kind(IconKind::Settings)) }
            }
        }
    }
}"#;

const PROPS: &[PropDoc] = &[
    prop("opened", "Signal<bool>", "-", "Stato aperto (X) o chiuso (tre linee); il click lo inverte."),
    prop("size", "BurgerSize", "Md", "Sm, Md o Lg."),
    prop("aria_label", "String", "\"Toggle navigation\"", "Etichetta per screen reader: sostituiscila con un testo localizzato."),
    prop("color", "Option<String>", "None", "Colore CSS delle linee (--sd-burger-color), ad esempio var(--sd-accent)."),
    prop("onclick", "Option<EventHandler<MouseEvent>>", "None", "Chiamato dopo aver invertito opened."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul <button>."),
];

pub fn story() -> Story {
    Story {
        slug: "burger",
        name: "Burger",
        category: Category::Navigazione,
        features: &["burger"],
        icon: IconKind::Menu,
        tagline: "Pulsante hamburger che apre e chiude la navigazione mobile, con le linee che diventano una X.",
        keywords: &["burger", "hamburger", "menu", "toggle", "navigazione mobile", "mobile nav"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Burger",
            controls: vec![
                Control::choice("size", "Dimensione", SIZE_OPTS, 1),
                Control::text("aria_label", "Etichetta accessibile", "Apri navigazione").emit(Emit::Always),
                Control::choice("color", "Colore linee", COLOR_OPTS, 0)
                    .emit(Emit::Template("color: Some(\"{v}\".to_string())")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["opened,"],
            children_code: None,
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "sizes",
                title: "Dimensioni",
                description: "Sm, Md e Lg, ognuno con il proprio stato.",
                code: SIZES_CODE,
                render: || rsx! { Sizes {} },
                span: Span::Half,
            },
            Example {
                id: "states",
                title: "Chiuso e aperto",
                description: "Le tre linee ruotano in una X quando opened è true.",
                code: STATES_CODE,
                render: || rsx! { States {} },
                span: Span::Half,
            },
            Example {
                id: "mobile-header",
                title: "Testata mobile",
                description: "Il burger mostra e nasconde la navigazione sotto la testata.",
                code: MOBILE_HEADER_CODE,
                render: || rsx! { MobileHeader {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Burger", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Icona interattiva senza materiale: focus ring (language.md §3.8), press e motion (§5)",
            depth: "Void: nessuna superficie, solo le linee",
            tokens: &["--sd-text-muted", "--sd-text", "--sd-radius-full", "--sd-radius-xs", "--sd-dur", "--sd-focus-ring"],
            notes: "Linee pill in --sd-text-muted, --sd-text al passaggio (o --sd-burger-color); press a 0.92 e morph in X con --sd-dur, ridotto con reduced motion.",
        }),
        a11y: &[
            "Elemento <button type=button> nativo: attivabile con Invio e Spazio.",
            "aria-expanded riflette opened, così il lettore annuncia aperto o chiuso.",
            "aria_label dà il nome al pulsante, che non ha testo visibile.",
            "Focus visibile con --sd-focus-ring.",
        ],
        related: &["navbar", "app-shell", "drawer", "nav-link", "bottom-navigation"],
    }
}
