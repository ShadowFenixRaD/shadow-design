//! Story: Button (feature `button`). Reference story for a simple component (REVAMP §4.9).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const VARIANTS: [ButtonVariant; 6] = [
    ButtonVariant::Primary,
    ButtonVariant::Secondary,
    ButtonVariant::Outline,
    ButtonVariant::Ghost,
    ButtonVariant::Success,
    ButtonVariant::Danger,
];
const VARIANT_OPTS: &[ChoiceOption] = &[
    opt("Primary", "ButtonVariant::Primary"),
    opt("Secondary", "ButtonVariant::Secondary"),
    opt("Outline", "ButtonVariant::Outline"),
    opt("Ghost", "ButtonVariant::Ghost"),
    opt("Success", "ButtonVariant::Success"),
    opt("Danger", "ButtonVariant::Danger"),
];
const SIZES: [ButtonSize; 5] = [ButtonSize::Xs, ButtonSize::Sm, ButtonSize::Md, ButtonSize::Lg, ButtonSize::Xl];
const SIZE_OPTS: &[ChoiceOption] = &[
    opt("Xs", "ButtonSize::Xs"),
    opt("Sm", "ButtonSize::Sm"),
    opt("Md", "ButtonSize::Md"),
    opt("Lg", "ButtonSize::Lg"),
    opt("Xl", "ButtonSize::Xl"),
];

fn render(k: &Knobs) -> Element {
    rsx! {
        Button {
            variant: k.pick("variant", &VARIANTS),
            size: k.pick("size", &SIZES),
            loading: k.bool("loading"),
            disabled: k.bool("disabled"),
            full_width: k.bool("full_width"),
            glow: k.bool("glow").then_some(true),
            left_icon: k.bool("left_icon").then(|| rsx! {
                Icon { kind: IconKind::Sparkles, size: 15 }
            }),
            right_icon: k.bool("right_icon").then(|| rsx! {
                Icon { kind: IconKind::ChevronRight, size: 15 }
            }),
            {k.text("label")}
        }
    }
}

#[component]
fn Variants() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm,
            Button { variant: ButtonVariant::Primary, "Primary" }
            Button { variant: ButtonVariant::Secondary, "Secondary" }
            Button { variant: ButtonVariant::Outline, "Outline" }
            Button { variant: ButtonVariant::Ghost, "Ghost" }
            Button { variant: ButtonVariant::Success, "Success" }
            Button { variant: ButtonVariant::Danger, "Danger" }
        }
    }
}
const VARIANTS_CODE: &str = r#"Group { gap: SpacingToken::Sm,
    Button { variant: ButtonVariant::Primary, "Primary" }
    Button { variant: ButtonVariant::Secondary, "Secondary" }
    Button { variant: ButtonVariant::Outline, "Outline" }
    Button { variant: ButtonVariant::Ghost, "Ghost" }
    Button { variant: ButtonVariant::Success, "Success" }
    Button { variant: ButtonVariant::Danger, "Danger" }
}"#;

#[component]
fn Sizes() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm, align: AlignItems::Center,
            Button { size: ButtonSize::Xs, variant: ButtonVariant::Secondary, "Xs" }
            Button { size: ButtonSize::Sm, variant: ButtonVariant::Secondary, "Sm" }
            Button { size: ButtonSize::Md, variant: ButtonVariant::Secondary, "Md" }
            Button { size: ButtonSize::Lg, variant: ButtonVariant::Secondary, "Lg" }
            Button { size: ButtonSize::Xl, variant: ButtonVariant::Secondary, "Xl" }
        }
    }
}
const SIZES_CODE: &str = r#"Group { gap: SpacingToken::Sm, align: AlignItems::Center,
    Button { size: ButtonSize::Xs, variant: ButtonVariant::Secondary, "Xs" }
    Button { size: ButtonSize::Sm, variant: ButtonVariant::Secondary, "Sm" }
    Button { size: ButtonSize::Md, variant: ButtonVariant::Secondary, "Md" }
    Button { size: ButtonSize::Lg, variant: ButtonVariant::Secondary, "Lg" }
    Button { size: ButtonSize::Xl, variant: ButtonVariant::Secondary, "Xl" }
}"#;

#[component]
fn States() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm,
            Button { loading: true, "Salvataggio" }
            Button { disabled: true, variant: ButtonVariant::Secondary, "Non disponibile" }
            Button { variant: ButtonVariant::Ghost, aria_label: Some("Impostazioni".to_string()),
                Icon { kind: IconKind::Settings, size: 16 }
            }
        }
    }
}
const STATES_CODE: &str = r#"Group { gap: SpacingToken::Sm,
    Button { loading: true, "Salvataggio" }
    Button { disabled: true, variant: ButtonVariant::Secondary, "Non disponibile" }
    Button { variant: ButtonVariant::Ghost, aria_label: Some("Impostazioni".to_string()),
        Icon { kind: IconKind::Settings, size: 16 }
    }
}"#;

#[component]
fn InContext() -> Element {
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Nodo edge-07".to_string()),
            subtitle: Some("Kyber-1024 · 4 peer attivi".to_string()),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, "Annulla" }
                    Button { variant: ButtonVariant::Danger, "Scollega" }
                    Button {
                        variant: ButtonVariant::Primary,
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Check, size: 15 }
                        }),
                        onclick: move |_| {
                            toast.success("Configurazione salvata");
                        },
                        "Salva modifiche"
                    }
                }
            }),
            p { class: "sc-muted-text", "Una sola azione primaria per vista: le altre restano secondarie o ghost." }
        }
    }
}
// The live render adds the showcase-only `class: "sc-muted-text"` on the paragraph; sample code
// never carries sc-* classes (REVAMP §5 item 6).
const IN_CONTEXT_CODE: &str = r#"let mut toast = use_notification();

Card {
    title: Some("Nodo edge-07".to_string()),
    subtitle: Some("Kyber-1024 · 4 peer attivi".to_string()),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button { variant: ButtonVariant::Ghost, "Annulla" }
            Button { variant: ButtonVariant::Danger, "Scollega" }
            Button {
                variant: ButtonVariant::Primary,
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Check, size: 15 }
                }),
                onclick: move |_| {
                    toast.success("Configurazione salvata");
                },
                "Salva modifiche"
            }
        }
    }),
    p { "Una sola azione primaria per vista: le altre restano secondarie o ghost." }
}"#;

const PROPS: &[PropDoc] = &[
    prop("variant", "ButtonVariant", "Primary", "Primary (lit accent), Secondary, Outline, Ghost, Success e Danger (tinta soft)."),
    prop("size", "ButtonSize", "Md", "Altezze da --sd-control-xs a --sd-control-xl."),
    prop("loading", "bool", "false", "Spinner, aria-busy e aria-disabled."),
    prop("loading_label", "Option<String>", "None", "Testo per screen reader durante il caricamento."),
    prop("disabled", "bool", "false", "Opacità --sd-disabled-opacity, nessun hover né glow."),
    prop("full_width", "bool", "false", "Occupa il 100% del contenitore."),
    prop("glow", "Option<bool>", "None", "Forza o rimuove il glow (--sd-glow-sm)."),
    prop("left_icon / right_icon", "Option<Element>", "None", "Icona prima o dopo il testo, aria-hidden."),
    prop("aria_label", "Option<String>", "None", "Obbligatoria per i pulsanti con sola icona."),
    prop("aria_pressed", "Option<bool>", "None", "Per pulsanti toggle."),
    prop("button_type", "Option<String>", "None", "type HTML (submit, reset, button)."),
    prop("onclick", "Option<EventHandler<MouseEvent>>", "None", "Click o attivazione da tastiera."),
    prop("class", "String", "\"\"", "Classi aggiuntive sull'elemento <button>."),
];

pub fn story() -> Story {
    Story {
        slug: "buttons",
        name: "Button",
        category: Category::Form,
        features: &["button"],
        icon: IconKind::Sparkles,
        tagline: "L'unica superficie con il riempimento illuminato: una sola azione primaria per vista.",
        keywords: &["button", "pulsante", "cta", "azione", "loading", "icon button"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Button",
            controls: vec![
                Control::choice("variant", "Variante", VARIANT_OPTS, 0),
                Control::choice("size", "Dimensione", SIZE_OPTS, 2),
                Control::text("label", "Etichetta", "Salva modifiche").emit(Emit::Skip),
                Control::bool("left_icon", "Icona sinistra", true)
                    .emit(Emit::Template("left_icon: Some(rsx! { Icon { kind: IconKind::Sparkles, size: 15 } })")),
                Control::bool("right_icon", "Icona destra", false)
                    .emit(Emit::Template("right_icon: Some(rsx! { Icon { kind: IconKind::ChevronRight, size: 15 } })")),
                Control::bool("loading", "Loading", false),
                Control::bool("disabled", "Disabilitato", false),
                Control::bool("full_width", "Larghezza piena", false),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: None,
            fixed_code: &[],
            children_code: Some("\"{label}\""),
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "variants",
                title: "Varianti",
                description: "Primary è l'unico con riempimento accento.",
                code: VARIANTS_CODE,
                render: || rsx! { Variants {} },
                span: Span::Half,
            },
            Example {
                id: "sizes",
                title: "Dimensioni",
                description: "Cinque altezze dai token --sd-control-*.",
                code: SIZES_CODE,
                render: || rsx! { Sizes {} },
                span: Span::Half,
            },
            Example {
                id: "states",
                title: "Stati",
                description: "Loading, disabilitato e sola icona.",
                code: STATES_CODE,
                render: || rsx! { States {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Barra azioni di una card con una sola primaria.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![PropGroup { component: "Button", rows: PROPS }],
        recipe: Some(Recipe {
            recipe: "Raised control (language.md §3.5)",
            depth: "Lit control",
            tokens: &["--sd-control-md", "--sd-accent", "--sd-accent-deep", "--sd-accent-fg", "--sd-accent-border", "--sd-glow-sm", "--sd-radius-md"],
            notes: "Gradiente accento → accent-deep solo su Primary; Danger e Success sono tinte soft, mai piene.",
        }),
        a11y: &[
            "Elemento <button> nativo: attivabile con Invio e Spazio.",
            "loading imposta aria-busy e aria-disabled senza rimuovere il focus.",
            "I pulsanti con sola icona richiedono aria_label.",
            "Focus visibile con --sd-focus-ring (outline 2px, offset 2px).",
        ],
        related: &["close-button", "copy-button", "segmented-controls", "speed-dial"],
    }
}
