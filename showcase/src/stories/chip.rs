//! Story: Chip and ChipGroup (feature `chip`). Stateful: the playground goes through `ChipDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const VARIANTS: [ChipVariant; 4] = [ChipVariant::Filled, ChipVariant::Outline, ChipVariant::Light, ChipVariant::Subtle];
const VARIANT_OPTS: &[ChoiceOption] = &[
    opt("Filled", "ChipVariant::Filled"),
    opt("Outline", "ChipVariant::Outline"),
    opt("Light", "ChipVariant::Light"),
    opt("Subtle", "ChipVariant::Subtle"),
];
const SIZES: [ControlSize; 3] = [ControlSize::Sm, ControlSize::Md, ControlSize::Lg];
const SIZE_OPTS: &[ChoiceOption] = &[opt("Sm", "ControlSize::Sm"), opt("Md", "ControlSize::Md"), opt("Lg", "ControlSize::Lg")];

const PRELUDE: &str = "let checked = use_signal(|| true);";

#[component]
fn ChipDemo(knobs: Knobs) -> Element {
    let checked = use_signal(|| true);
    rsx! {
        Chip {
            checked,
            variant: knobs.pick("variant", &VARIANTS),
            size: knobs.pick("size", &SIZES),
            icon: knobs.bool("icon").then_some(IconKind::Flame),
            show_check: knobs.bool("show_check"),
            disabled: knobs.bool("disabled"),
            {knobs.text("label")}
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        ChipDemo { knobs: k.clone() }
    }
}

#[component]
fn Variants() -> Element {
    rsx! {
        Group { gap: SpacingToken::Sm,
            Chip { default_checked: true, variant: ChipVariant::Filled, "Filled" }
            Chip { default_checked: true, variant: ChipVariant::Outline, "Outline" }
            Chip { default_checked: true, variant: ChipVariant::Light, "Light" }
            Chip { default_checked: true, variant: ChipVariant::Subtle, "Subtle" }
            Chip { variant: ChipVariant::Outline, "Spento" }
        }
    }
}
const VARIANTS_CODE: &str = r#"Group { gap: SpacingToken::Sm,
    Chip { default_checked: true, variant: ChipVariant::Filled, "Filled" }
    Chip { default_checked: true, variant: ChipVariant::Outline, "Outline" }
    Chip { default_checked: true, variant: ChipVariant::Light, "Light" }
    Chip { default_checked: true, variant: ChipVariant::Subtle, "Subtle" }
    Chip { variant: ChipVariant::Outline, "Spento" }
}"#;

#[component]
fn Single() -> Element {
    let range = use_signal(|| vec!["24h".to_string()]);
    rsx! {
        ChipGroup { value: range, variant: Some(ChipVariant::Outline),
            Chip { value: "1h", "Ultima ora" }
            Chip { value: "24h", "24 ore" }
            Chip { value: "7d", "7 giorni" }
            Chip { value: "30d", "30 giorni" }
        }
    }
}
const SINGLE_CODE: &str = r#"let range = use_signal(|| vec!["24h".to_string()]);

ChipGroup { value: range, variant: Some(ChipVariant::Outline),
    Chip { value: "1h", "Ultima ora" }
    Chip { value: "24h", "24 ore" }
    Chip { value: "7d", "7 giorni" }
    Chip { value: "30d", "30 giorni" }
}"#;

#[component]
fn Multiple() -> Element {
    let topics = use_signal(|| vec!["security".to_string(), "network".to_string()]);
    rsx! {
        ChipGroup { value: topics, multiple: true, size: Some(ControlSize::Sm),
            Chip { value: "security", icon: Some(IconKind::Shield), "Sicurezza" }
            Chip { value: "network", icon: Some(IconKind::Wifi), "Rete" }
            Chip { value: "storage", icon: Some(IconKind::Server), "Storage" }
            Chip { value: "release", icon: Some(IconKind::Sparkles), "Release" }
            Chip { value: "legacy", icon: Some(IconKind::Clock), disabled: true, "Archivio" }
        }
    }
}
const MULTIPLE_CODE: &str = r#"let topics = use_signal(|| vec!["security".to_string(), "network".to_string()]);

ChipGroup { value: topics, multiple: true, size: Some(ControlSize::Sm),
    Chip { value: "security", icon: Some(IconKind::Shield), "Sicurezza" }
    Chip { value: "network", icon: Some(IconKind::Wifi), "Rete" }
    Chip { value: "storage", icon: Some(IconKind::Server), "Storage" }
    Chip { value: "release", icon: Some(IconKind::Sparkles), "Release" }
    Chip { value: "legacy", icon: Some(IconKind::Clock), disabled: true, "Archivio" }
}"#;

#[component]
fn InContext() -> Element {
    let regions = use_signal(|| vec!["eu".to_string(), "us".to_string()]);
    let peers = [
        ("edge-07", "eu", "Milano", "12 ms"),
        ("edge-11", "eu", "Francoforte", "18 ms"),
        ("relay-02", "us", "New York", "84 ms"),
        ("edge-21", "asia", "Singapore", "162 ms"),
    ];
    rsx! {
        Card {
            title: Some("Peer connessi".to_string()),
            subtitle: Some("Filtra per regione".to_string()),
            Stack { gap: SpacingToken::Md,
                ChipGroup { value: regions, multiple: true, size: Some(ControlSize::Sm),
                    Chip { value: "eu", "Europa" }
                    Chip { value: "us", "America" }
                    Chip { value: "asia", "Asia" }
                }
                List { spacing: ListSpacing::Sm,
                    for (name, region, city, latency) in peers.into_iter().filter(|p| regions().iter().any(|r| r == p.1)) {
                        ListItem { key: "{name}",
                            Group { justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
                                Text { "{name} · {city}" }
                                Badge { label: latency.to_string(), variant: if region == "asia" { BadgeVariant::Warning } else { BadgeVariant::Default } }
                            }
                        }
                    }
                }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let regions = use_signal(|| vec!["eu".to_string(), "us".to_string()]);
let peers = [
    ("edge-07", "eu", "Milano", "12 ms"),
    ("edge-11", "eu", "Francoforte", "18 ms"),
    ("relay-02", "us", "New York", "84 ms"),
    ("edge-21", "asia", "Singapore", "162 ms"),
];

Card {
    title: Some("Peer connessi".to_string()),
    subtitle: Some("Filtra per regione".to_string()),
    Stack { gap: SpacingToken::Md,
        ChipGroup { value: regions, multiple: true, size: Some(ControlSize::Sm),
            Chip { value: "eu", "Europa" }
            Chip { value: "us", "America" }
            Chip { value: "asia", "Asia" }
        }
        List { spacing: ListSpacing::Sm,
            for (name, region, city, latency) in peers.into_iter().filter(|p| regions().iter().any(|r| r == p.1)) {
                ListItem {
                    Group { justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
                        Text { "{name} · {city}" }
                        Badge { label: latency.to_string(), variant: if region == "asia" { BadgeVariant::Warning } else { BadgeVariant::Default } }
                    }
                }
            }
        }
    }
}"#;

const CHIP_PROPS: &[PropDoc] = &[
    prop("children", "Element", "-", "Testo della chip."),
    prop("checked", "Option<Signal<bool>>", "None", "Stato controllato; senza signal la chip gestisce lo stato da sola."),
    prop("default_checked", "bool", "false", "Stato iniziale quando non è controllata."),
    prop("value", "Option<String>", "None", "Valore dentro un ChipGroup."),
    prop("variant", "ChipVariant", "Filled", "Filled, Outline, Light o Subtle."),
    prop("size", "ControlSize", "Md", "Sm, Md o Lg."),
    prop("icon", "Option<IconKind>", "None", "Icona prima del testo."),
    prop("show_check", "bool", "true", "Spunta visibile quando selezionata."),
    prop("disabled", "bool", "false", "Non selezionabile."),
    prop("on_toggle", "Option<EventHandler<bool>>", "None", "Nuovo stato."),
    prop("on_click", "Option<EventHandler<MouseEvent>>", "None", "Click grezzo."),
    prop("class", "Option<String>", "None", "Classi aggiuntive."),
];
const GROUP_PROPS: &[PropDoc] = &[
    prop("value", "Signal<Vec<String>>", "-", "Valori selezionati (uno solo se multiple è false)."),
    prop("multiple", "bool", "false", "Selezione multipla (role=group) o singola (role=radiogroup)."),
    prop("size / variant", "Option<ControlSize> / Option<ChipVariant>", "None", "Applicati a tutte le chip figlie."),
    prop("on_change", "Option<EventHandler<Vec<String>>>", "None", "Nuova selezione."),
    prop("children / class", "Element / Option<String>", "-", "Chip figlie e classi aggiuntive."),
];

pub fn story() -> Story {
    Story {
        slug: "chip",
        name: "Chip",
        category: Category::Form,
        features: &["chip"],
        icon: IconKind::Check,
        tagline: "Opzioni compatte da attivare, da sole o in gruppo, per filtri e preferenze.",
        keywords: &["chip", "filtro", "tag", "toggle", "chip group", "selezione"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Chip",
            controls: vec![
                Control::text("label", "Etichetta", "Solo critici").emit(Emit::Skip),
                Control::choice("variant", "Variante", VARIANT_OPTS, 0),
                Control::choice("size", "Dimensione", SIZE_OPTS, 1),
                Control::bool("icon", "Icona", true).emit(Emit::Template("icon: Some(IconKind::Flame)")),
                Control::bool("show_check", "Spunta", true),
                Control::bool("disabled", "Disabilitato", false),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["checked,"],
            children_code: Some("\"{label}\""),
            stage: Stage::Center,
        }),
        examples: vec![
            Example {
                id: "variants",
                title: "Varianti",
                description: "Quattro stili selezionati e uno spento.",
                code: VARIANTS_CODE,
                render: || rsx! { Variants {} },
                span: Span::Half,
            },
            Example {
                id: "single",
                title: "Scelta singola",
                description: "ChipGroup senza multiple si comporta come un radio.",
                code: SINGLE_CODE,
                render: || rsx! { Single {} },
                span: Span::Half,
            },
            Example {
                id: "multiple",
                title: "Scelta multipla",
                description: "Argomenti con icona; uno disabilitato.",
                code: MULTIPLE_CODE,
                render: || rsx! { Multiple {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Filtri per regione sopra l'elenco dei peer.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Chip", rows: CHIP_PROPS },
            PropGroup { component: "ChipGroup", rows: GROUP_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Item (§3.4) con stato selected (§3.7), forma a pillola",
            depth: "Lit control (selezionata) · flat (spenta)",
            tokens: &["--sd-radius-full", "--sd-accent-soft", "--sd-accent-border", "--sd-accent-strong", "--sd-glow-sm", "--sd-tint-1"],
            notes: "La pillola resta tale con ogni scala di raggio; in un gruppo una sola chip può brillare.",
        }),
        a11y: &[
            "Ogni chip è un <button> con aria-checked e aria-disabled; da sola ha role=checkbox.",
            "ChipGroup singolo: role=radiogroup con chip role=radio; multiplo: role=group con chip role=checkbox.",
            "Invio e Spazio attivano la chip; il focus mostra --sd-focus-ring.",
            "La spunta e l'icona sono decorative.",
        ],
        related: &["checkbox", "segmented-controls", "badges", "select"],
    }
}
