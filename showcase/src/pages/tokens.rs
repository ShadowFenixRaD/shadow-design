//! Tokens: every `--sd-*` group of docs/design/tokens.md rendered live (ADR-0006 §5).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::app::theme_menu::{glow_key, preset_name, radius_key, ThemeSwatches};
use crate::story::sections::SectionHead;

/// A token group: head, live body, the token names in a code block.
#[component]
fn TokenGroup(eyebrow: &'static str, title: &'static str, desc: &'static str, code: String, children: Element) -> Element {
    rsx! {
        section { class: "sc-section",
            SectionHead { eyebrow, title, desc: Some(desc) }
            {children}
            CodeBlock { code, language: "css", with_line_numbers: false }
        }
    }
}

/// Static colour chip: a swatch painted with the token itself, its name and resolved value.
#[component]
fn TokenChip(token: &'static str, value: String) -> Element {
    rsx! {
        div { class: "sc-token-chip",
            span { class: "sc-token-chip__swatch", style: "background: var({token});" }
            span { class: "sc-token-chip__name", "{token}" }
            span { class: "sc-hint", "{value}" }
        }
    }
}

/// A copy of the global theme with one radius scale or glow mode applied, so a row can show
/// the three values side by side. It follows the global accent.
#[component]
fn ScopedTheme(#[props(default)] radius: Option<RadiusScale>, #[props(default)] glow: Option<GlowMode>, children: Element) -> Element {
    let global = use_theme();
    let make = move |base: &Theme| {
        let mut t = base.clone();
        if let Some(r) = radius {
            t.apply_radius_scale(r);
        }
        if let Some(g) = glow {
            t.apply_glow_mode(g);
        }
        t
    };
    let mut local = use_signal(|| make(&global.peek()));
    use_effect(move || {
        let next = make(&global());
        if *local.peek() != next {
            local.set(next);
        }
    });
    rsx! {
        ThemeProvider { theme: Some(local), {children} }
    }
}

#[component]
fn RadiusTile(scale: RadiusScale, label: &'static str) -> Element {
    let mut theme = use_theme();
    let value = use_signal(|| "edge-07".to_string());
    let active = theme().radius_scale == scale;
    rsx! {
        Card {
            class: "sc-scope-tile".to_string(),
            title: Some(label.to_string()),
            subtitle: Some(format!("--sd-radius-* · {}", scale.name())),
            glow: active.then_some(true),
            Stack { gap: SpacingToken::Sm,
                TextInput { value, label: Some("Nodo".to_string()) }
                Group { gap: SpacingToken::Sm, align: AlignItems::Center,
                    Button { size: ButtonSize::Sm, variant: ButtonVariant::Secondary, "Azione" }
                    Badge { label: "Online".to_string(), variant: BadgeVariant::Success, pill: true, dot: true }
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    full_width: true,
                    disabled: active,
                    onclick: move |_| theme.write().apply_radius_scale(scale),
                    if active {
                        "In uso"
                    } else {
                        "Applica"
                    }
                }
            }
        }
    }
}

#[component]
fn GlowTile(mode: GlowMode, label: &'static str, strength: &'static str) -> Element {
    let mut theme = use_theme();
    let value = use_signal(|| "Campo a fuoco".to_string());
    let tab = use_signal(|| "b".to_string());
    let active = theme().glow_mode == mode;
    rsx! {
        Card {
            class: "sc-scope-tile".to_string(),
            title: Some(label.to_string()),
            subtitle: Some(format!("--sd-glow-strength: {strength}")),
            Stack { gap: SpacingToken::Sm,
                Button { size: ButtonSize::Sm, "Primaria" }
                TextInput { value, glow: Some(true) }
                SegmentedControl {
                    size: SegmentedControlSize::Sm,
                    full_width: true,
                    options: vec![SegmentOption::new("a", "Uno"), SegmentOption::new("b", "Due"), SegmentOption::new("c", "Tre")],
                    value: tab,
                }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    full_width: true,
                    disabled: active,
                    onclick: move |_| theme.write().apply_glow_mode(mode),
                    if active {
                        "In uso"
                    } else {
                        "Applica"
                    }
                }
            }
        }
    }
}

const SURFACES: [(&str, &str); 5] = [
    ("--sd-void", "Pagina"),
    ("--sd-well", "Campi e tracce"),
    ("--sd-surface-1", "Contenitori"),
    ("--sd-surface-2", "Layer flottanti"),
    ("--sd-surface-3", "Hover nei layer"),
];

const SIZES: [(&str, &str, ButtonSize); 5] = [
    ("--sd-control-xs", "26px", ButtonSize::Xs),
    ("--sd-control-sm", "32px", ButtonSize::Sm),
    ("--sd-control-md", "40px", ButtonSize::Md),
    ("--sd-control-lg", "48px", ButtonSize::Lg),
    ("--sd-control-xl", "54px", ButtonSize::Xl),
];

const TYPE_SCALE: [(&str, &str); 8] = [
    ("--sd-text-3xl", "28px"),
    ("--sd-text-2xl", "22px"),
    ("--sd-text-xl", "18px"),
    ("--sd-text-lg", "16px"),
    ("--sd-text-md", "14px"),
    ("--sd-text-sm", "13px"),
    ("--sd-text-xs", "12px"),
    ("--sd-text-2xs", "11px"),
];

#[component]
pub fn TokensPage() -> Element {
    let theme = use_theme();
    let t = theme();
    let accent = t.primary_color.accent_tokens();
    let accent_name = preset_name(&t.primary_color);
    let radius_now = radius_key(t.radius_scale);
    let glow_now = glow_key(t.glow_mode);

    rsx! {
        document::Title { "Tokens · Shadow Design" }
        div { class: "sc-story",
            header { class: "sc-story__head",
                span { class: "sc-eyebrow", "Fondamenta" }
                h1 { class: "sc-title", "Tokens" }
                p { class: "sc-lede",
                    "Ogni valore della libreria è una variabile --sd-* scritta dal ThemeProvider. Qui sono rese dal vivo: cambia accento, raggio o glow e guarda la pagina seguire."
                }
            }

            TokenGroup {
                eyebrow: "Accento",
                title: "Un hex, sette derivate",
                desc: "Calcolate in theme.rs da un solo colore. Accento attivo: vedi le tessere qui sotto.",
                code: "--sd-accent  --sd-accent-rgb  --sd-accent-strong  --sd-accent-deep\n--sd-accent-soft  --sd-accent-border  --sd-accent-fg  --sd-focus-ring".to_string(),
                div { class: "sc-section__head-row",
                    span { class: "sc-hint", "Accento attivo: {accent_name}" }
                    ThemeSwatches { compact: true }
                }
                div { class: "sc-token-grid",
                    TokenChip { token: "--sd-accent", value: accent.accent.clone() }
                    TokenChip { token: "--sd-accent-strong", value: accent.strong.clone() }
                    TokenChip { token: "--sd-accent-deep", value: accent.deep.clone() }
                    TokenChip { token: "--sd-accent-soft", value: accent.soft.clone() }
                    TokenChip { token: "--sd-accent-border", value: accent.border.clone() }
                    TokenChip { token: "--sd-accent-fg", value: accent.fg.clone() }
                }
            }

            TokenGroup {
                eyebrow: "Superfici",
                title: "Profondità per ruolo",
                desc: "Il vuoto è la pagina; ogni livello sopra è un ruolo diverso (language.md §2).",
                code: "--sd-void  --sd-well  --sd-surface-1  --sd-surface-2  --sd-surface-3\n--sd-border-subtle  --sd-border  --sd-border-strong  --sd-border-hover".to_string(),
                div { class: "sc-nest",
                    SurfaceBox { level: 0 }
                }
            }

            TokenGroup {
                eyebrow: "Semantici",
                title: "Stati fissi",
                desc: "Successo, avviso, errore e info non seguono l'accento: tinta soft, bordo, testo forte.",
                code: "--sd-success  --sd-warning  --sd-danger  --sd-info\n--sd-<sem>-strong  --sd-<sem>-soft  --sd-<sem>-soft-hover  --sd-<sem>-border".to_string(),
                div { class: "sc-token-grid sc-token-grid--2",
                    Alert { variant: AlertVariant::Success, title: Some("Success".to_string()), "Deploy completato su 12 nodi." }
                    Alert { variant: AlertVariant::Warning, title: Some("Warning".to_string()), "Latenza sopra soglia su relay-03." }
                    Alert { variant: AlertVariant::Danger, title: Some("Danger".to_string()), "Chiave revocata: sessione chiusa." }
                    Alert { variant: AlertVariant::Info, title: Some("Info".to_string()), "Manutenzione programmata alle 02:00." }
                }
            }

            TokenGroup {
                eyebrow: "Ombre",
                title: "Elevazione",
                desc: "Specular in alto, ombra per ruolo; sd-shadow le compone senza sovrascriverle.",
                code: "--sd-specular  --sd-specular-strong  --sd-inset-well\n--sd-elev-1  --sd-elev-2  --sd-elev-3".to_string(),
                div { class: "sc-token-grid sc-token-grid--3",
                    Card { title: Some("Sm".to_string()), subtitle: Some("CardShadow::Sm".to_string()), shadow: CardShadow::Sm,
                        p { class: "sc-muted-text", "Contenitori in pagina." }
                    }
                    Card { title: Some("Md".to_string()), subtitle: Some("CardShadow::Md".to_string()), shadow: CardShadow::Md,
                        p { class: "sc-muted-text", "Pannelli in evidenza." }
                    }
                    Card { title: Some("Lg".to_string()), subtitle: Some("CardShadow::Lg".to_string()), shadow: CardShadow::Lg,
                        p { class: "sc-muted-text", "Layer flottanti." }
                    }
                }
            }

            TokenGroup {
                eyebrow: "Raggio",
                title: "Sharp, Medium, Soft",
                desc: "Tre scale dello stesso tema, una accanto all'altra. Le pillole restano pillole.",
                code: format!(
                    "--sd-radius-xs  --sd-radius-sm  --sd-radius-md  --sd-radius-lg  --sd-radius-xl  --sd-radius-full\n/* scala attiva: {radius_now} */"
                ),
                div { class: "sc-token-grid sc-token-grid--3",
                    ScopedTheme { radius: Some(RadiusScale::Sharp),
                        RadiusTile { scale: RadiusScale::Sharp, label: "Sharp" }
                    }
                    ScopedTheme { radius: Some(RadiusScale::Medium),
                        RadiusTile { scale: RadiusScale::Medium, label: "Medium" }
                    }
                    ScopedTheme { radius: Some(RadiusScale::Soft),
                        RadiusTile { scale: RadiusScale::Soft, label: "Soft" }
                    }
                }
            }

            TokenGroup {
                eyebrow: "Glow",
                title: "Off, Subtle, Expressive",
                desc: "Solo focus, selezione, primaria e riempimenti di valore: l'intensità è un solo token.",
                code: format!("--sd-glow-strength  --sd-glow-sm  --sd-glow-md  --sd-glow-danger\n/* modalità attiva: {glow_now} */"),
                div { class: "sc-token-grid sc-token-grid--3",
                    ScopedTheme { glow: Some(GlowMode::Off),
                        GlowTile { mode: GlowMode::Off, label: "Off", strength: "0" }
                    }
                    ScopedTheme { glow: Some(GlowMode::Subtle),
                        GlowTile { mode: GlowMode::Subtle, label: "Subtle", strength: "0.6" }
                    }
                    ScopedTheme { glow: Some(GlowMode::Expressive),
                        GlowTile { mode: GlowMode::Expressive, label: "Expressive", strength: "1" }
                    }
                }
            }

            TokenGroup {
                eyebrow: "Dimensioni",
                title: "Altezze dei controlli",
                desc: "Cinque altezze per pulsanti, campi e select; md è il default.",
                code: "--sd-control-xs / sm / md / lg / xl\n--sd-item-h-sm  --sd-item-h  --sd-item-h-lg\n--sd-space-1 … --sd-space-10  --sd-icon-sm / md / lg".to_string(),
                Card {
                    div { class: "sc-ruler",
                        for (token , px , size) in SIZES {
                            div { key: "{token}", class: "sc-ruler__row",
                                span { class: "sc-ruler__bar", style: "height: var({token});" }
                                Button { size, variant: ButtonVariant::Secondary, "{px}" }
                                span { class: "sc-hint", "{token}" }
                            }
                        }
                    }
                }
            }

            TokenGroup {
                eyebrow: "Tipografia",
                title: "Scala dei testi",
                desc: "Otto passi; Kanit per i titoli, Onest per il resto, Source Code Pro per codice ed etichette.",
                code: "--sd-font-heading  --sd-font-body  --sd-font-mono  --sd-weight-heading\n--sd-text-2xs … --sd-text-3xl  --sd-leading-tight  --sd-leading-normal".to_string(),
                Card {
                    div { class: "sc-type-scale",
                        for (token , px) in TYPE_SCALE {
                            div { key: "{token}", class: "sc-type-scale__row",
                                span { class: "sc-hint", "{token} · {px}" }
                                span { class: "sc-type-scale__sample", style: "font-size: var({token});", "Vetro nero illuminato dall'alto" }
                            }
                        }
                    }
                }
            }

            TokenGroup {
                eyebrow: "Movimento",
                title: "Durate e curva",
                desc: "Hover e colore in --sd-dur-fast, ingressi in --sd-dur, cassetti in --sd-dur-slow; tutto rispetta reduced motion.",
                code: "--sd-ease: cubic-bezier(.16, 1, .3, 1)\n--sd-dur-fast: 120ms   /* hover, colore */\n--sd-dur: 180ms        /* pop-in, fade */\n--sd-dur-slow: 280ms   /* drawer, accordion */".to_string(),
                Group { gap: SpacingToken::Sm,
                    Button { variant: ButtonVariant::Primary, "Passa qui" }
                    Button { variant: ButtonVariant::Secondary, "Passa qui" }
                    Button { variant: ButtonVariant::Outline, "Passa qui" }
                }
            }
        }
    }
}

/// Five nested boxes, one per surface level.
#[component]
fn SurfaceBox(level: usize) -> Element {
    let Some((token, role)) = SURFACES.get(level).copied() else {
        return rsx! {};
    };
    rsx! {
        div { class: "sc-nest__box", style: "background: var({token});",
            span { class: "sc-nest__label",
                span { class: "sc-token-chip__name", "{token}" }
                span { class: "sc-hint", "{role}" }
            }
            SurfaceBox { level: level + 1 }
        }
    }
}
