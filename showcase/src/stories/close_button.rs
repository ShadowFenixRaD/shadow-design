//! Story: CloseButton (feature `close_button`). Stateless: the playground renders it directly.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const SIZES: [CloseButtonSize; 3] = [CloseButtonSize::Sm, CloseButtonSize::Md, CloseButtonSize::Lg];
const SIZE_OPTS: &[ChoiceOption] = &[
    opt("Sm", "CloseButtonSize::Sm"),
    opt("Md", "CloseButtonSize::Md"),
    opt("Lg", "CloseButtonSize::Lg"),
];
const VARIANTS: [CloseButtonVariant; 3] = [CloseButtonVariant::Subtle, CloseButtonVariant::Filled, CloseButtonVariant::Transparent];
const VARIANT_OPTS: &[ChoiceOption] = &[
    opt("Subtle", "CloseButtonVariant::Subtle"),
    opt("Filled", "CloseButtonVariant::Filled"),
    opt("Transparent", "CloseButtonVariant::Transparent"),
];

fn render(k: &Knobs) -> Element {
    let label = k.text("aria_label");
    rsx! {
        CloseButton {
            size: k.pick("size", &SIZES),
            variant: k.pick("variant", &VARIANTS),
            disabled: k.bool("disabled"),
            aria_label: (!label.is_empty()).then_some(label),
        }
    }
}

#[component]
fn Variants() -> Element {
    rsx! {
        Group { gap: SpacingToken::Lg, align: AlignItems::Center,
            CloseButton { size: CloseButtonSize::Sm }
            CloseButton {}
            CloseButton { size: CloseButtonSize::Lg }
            CloseButton { variant: CloseButtonVariant::Filled }
            CloseButton { variant: CloseButtonVariant::Transparent }
            CloseButton { disabled: true }
        }
    }
}
const VARIANTS_CODE: &str = r#"Group { gap: SpacingToken::Lg, align: AlignItems::Center,
    CloseButton { size: CloseButtonSize::Sm }
    CloseButton {}
    CloseButton { size: CloseButtonSize::Lg }
    CloseButton { variant: CloseButtonVariant::Filled }
    CloseButton { variant: CloseButtonVariant::Transparent }
    CloseButton { disabled: true }
}"#;

#[component]
fn Dismiss() -> Element {
    let mut open = use_signal(|| true);
    rsx! {
        if open() {
            Card {
                title: Some("Nuovo nodo rilevato".to_string()),
                subtitle: Some("edge-31 chiede di entrare nel cluster".to_string()),
                buttons: Some(rsx! {
                    CloseButton {
                        size: CloseButtonSize::Sm,
                        aria_label: Some("Ignora la richiesta di edge-31".to_string()),
                        onclick: move |_| open.set(false),
                    }
                }),
                Text { size: TextSize::Sm, dimmed: true, "Approvalo dalla pagina Nodi entro 24 ore." }
            }
        } else {
            Button { variant: ButtonVariant::Ghost, onclick: move |_| open.set(true), "Mostra di nuovo" }
        }
    }
}
const DISMISS_CODE: &str = r#"let mut open = use_signal(|| true);

if open() {
    Card {
        title: Some("Nuovo nodo rilevato".to_string()),
        subtitle: Some("edge-31 chiede di entrare nel cluster".to_string()),
        buttons: Some(rsx! {
            CloseButton {
                size: CloseButtonSize::Sm,
                aria_label: Some("Ignora la richiesta di edge-31".to_string()),
                onclick: move |_| open.set(false),
            }
        }),
        Text { size: TextSize::Sm, dimmed: true, "Approvalo dalla pagina Nodi entro 24 ore." }
    }
} else {
    Button { variant: ButtonVariant::Ghost, onclick: move |_| open.set(true), "Mostra di nuovo" }
}"#;

const PROPS: &[PropDoc] = &[
    prop("size", "CloseButtonSize", "Md", "Sm, Md, Lg: quadrati da --sd-control-xs a --sd-control-md, icona da 14 a 20 px."),
    prop("variant", "CloseButtonVariant", "Subtle", "Subtle (ghost), Filled (secondary), Transparent (solo colore, per i campi)."),
    prop("aria_label", "Option<String>", "None", "Nome accessibile e title; senza, \"Close\" da Locale."),
    prop("disabled", "bool", "false", "Disabilitato: nessun clic, opacità ridotta."),
    prop("onclick", "Option<EventHandler<MouseEvent>>", "None", "Chiamato al clic o con Invio e Spazio."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul <button>."),
];

pub fn story() -> Story {
    Story {
        slug: "close-button",
        name: "CloseButton",
        category: Category::Utility,
        features: &["close_button"],
        icon: IconKind::X,
        tagline: "La X di chiusura con nome accessibile, uguale in finestre, avvisi, card e campi.",
        keywords: &["close", "chiudi", "chiusura", "x", "dismiss", "icon button"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "CloseButton",
            controls: vec![
                Control::choice("size", "Dimensione", SIZE_OPTS, 1),
                Control::choice("variant", "Variante", VARIANT_OPTS, 0),
                Control::text("aria_label", "Nome accessibile", "Chiudi pannello")
                    .emit(Emit::Template("aria_label: Some(\"{v}\".to_string())"))
                    .hint("Vuoto: usa \"Close\" da Locale."),
                Control::bool("disabled", "Disabilitato", false),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: None,
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "variants",
                title: "Dimensioni e varianti",
                description: "Tre misure, tre varianti e lo stato disabilitato.",
                code: VARIANTS_CODE,
                render: || rsx! { Variants {} },
                span: Span::Half,
            },
            Example {
                id: "dismiss",
                title: "Chiudere una card",
                description: "Nello slot buttons, con un nome che dice cosa chiude.",
                code: DISMISS_CODE,
                render: || rsx! { Dismiss {} },
                span: Span::Half,
            },
        ],
        props: vec![PropGroup { component: "CloseButton", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Raised control, variante ghost (language.md §3.5)",
            depth: "Lit control",
            tokens: &["--sd-control-sm", "--sd-tint-1", "--sd-border", "--sd-text-muted", "--sd-radius-md", "--sd-focus-ring"],
            notes: "Ghost di default per non competere con l'azione primaria; Filled solo su superfici affollate, Transparent dentro i campi.",
        }),
        a11y: &[
            "Elemento <button type=button> nativo, attivabile con Invio e Spazio.",
            "aria-label e title coincidono: \"Close\" da Locale o il testo passato in aria_label.",
            "L'icona X è decorativa; il nome dice cosa si chiude quando ci sono più X nella stessa vista.",
            "Focus visibile con --sd-focus-ring come ogni pulsante.",
        ],
        related: &["buttons", "modals", "alerts", "chip"],
    }
}
