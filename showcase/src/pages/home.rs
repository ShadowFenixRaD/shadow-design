//! Home: a live product scene, the preset wall, the four scenes and numbers from the registry
//! (ADR-0006 §5, REVAMP §1.1–1.3).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::app::theme_menu::{preset_name, ThemeSwatches};
use crate::pages::FOUNDATION;
use crate::scenes::{AuthCard, InboxPane, OpsDashboard, SettingsForm};
use crate::story::registry;
use crate::story::schema::Category;
use crate::story::sections::SectionHead;
use crate::Route;

const PRESETS: [(PrimaryColor, fn() -> Theme); 5] = [
    (PrimaryColor::Violet, Theme::shadow),
    (PrimaryColor::Obsidian, Theme::obsidian),
    (PrimaryColor::Crimson, Theme::crimson),
    (PrimaryColor::Cyan, Theme::cyan),
    (PrimaryColor::Emerald, Theme::emerald),
];

#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();
    let scene = use_signal(|| "settings".to_string());
    // Below md the hero scene drops the table's least important columns (REVAMP §1.6).
    let narrow = use_media_query("(max-width: 767px)");
    let components = registry::all().len();
    let features = registry::feature_count();
    let install = format!(
        "[dependencies.shadow-design]\nversion = \"{VERSION}\"\ndefault-features = false\nfeatures = [\"button\", \"card\", \"select\", \"theme\"]"
    );

    rsx! {
        document::Title { "Shadow Design · Componenti Dioxus in Lit Glass" }
        div { class: "sc-home",
            // 1. Hero: words left, a working product right.
            section { class: "sc-hero",
                div { class: "sc-hero__copy",
                    span { class: "sc-eyebrow", "Dioxus 0.7 · Lit Glass · v{VERSION}" }
                    h1 { class: "sc-display", "Shadow Design" }
                    p { class: "sc-lede sc-lede--lg",
                        "Vetro nero illuminato dall'alto. Un solo hex ricolora {components} componenti; ogni feature Cargo è opzionale."
                    }
                    Group { gap: SpacingToken::Sm,
                        Button {
                            left_icon: Some(rsx! {
                                Icon { kind: IconKind::Sparkles, size: 15 }
                            }),
                            onclick: move |_| {
                                navigator.push(Route::story("buttons"));
                            },
                            "Esplora i componenti"
                        }
                        Button {
                            variant: ButtonVariant::Secondary,
                            left_icon: Some(rsx! {
                                Icon { kind: IconKind::Palette, size: 15 }
                            }),
                            onclick: move |_| {
                                navigator.push(Route::ThemeBuilderPage {});
                            },
                            "Theme Builder"
                        }
                    }
                    div { class: "sc-hero__swatches",
                        ThemeSwatches {}
                        span { class: "sc-hint",
                            "Clicca un colore: tutto lo showcase, compresa la scena qui accanto, cambia all'istante."
                        }
                    }
                }
                div { class: "sc-scene sc-scene--hero",
                    OpsDashboard { compact: narrow() }
                }
            }

            // 2. Preset wall: five accents at once (nested ThemeProviders).
            section { class: "sc-section",
                SectionHead {
                    eyebrow: "Tema",
                    title: "Un hex, tutto il tema",
                    desc: Some("Cinque preset resi insieme. Ognuno è solo un colore: derivazioni, glow e focus seguono."),
                }
                ScrollArea { scroll_x: true, scroll_y: false,
                    div { class: "sc-preset-wall",
                        for index in 0..PRESETS.len() {
                            PresetTile { key: "{index}", index }
                        }
                    }
                }
            }

            // 3. Scenes.
            section { class: "sc-section",
                SectionHead {
                    eyebrow: "Scene",
                    title: "Componenti in un prodotto vero",
                    desc: Some("La dashboard qui sopra e altre tre schermate, composte solo con la libreria."),
                }
                Tabs {
                    TabList { aria_label: Some("Scene".to_string()),
                        Tab { value: "settings", active: scene, icon: Some(IconKind::Settings), "Impostazioni" }
                        Tab { value: "inbox", active: scene, icon: Some(IconKind::MessageSquare), "Inbox" }
                        Tab { value: "auth", active: scene, icon: Some(IconKind::Lock), "Accesso" }
                    }
                    div { class: "sc-scene",
                        TabPanel { value: "settings", active: scene, class: "animate-fade-in",
                            SettingsForm { compact: narrow() }
                        }
                        TabPanel { value: "inbox", active: scene, class: "animate-fade-in",
                            InboxPane { compact: narrow() }
                        }
                        TabPanel { value: "auth", active: scene, class: "animate-fade-in",
                            AuthCard { compact: narrow() }
                        }
                    }
                }
            }

            // 4. Zero bloat: numbers from the registry, not claims.
            section { class: "sc-section",
                SectionHead {
                    eyebrow: "Zero bloat",
                    title: "Compili solo ciò che usi",
                    desc: Some("Ogni componente è dietro una feature Cargo indipendente; lo SCSS si compila con asset!(), senza Node."),
                }
                div { class: "sc-stats",
                    Stat { value: components.to_string(), label: "componenti" }
                    Stat { value: features.to_string(), label: "feature Cargo indipendenti" }
                    Stat { value: "0".to_string(), label: "dipendenze JavaScript" }
                    Stat { value: "3".to_string(), label: "famiglie tipografiche" }
                }
                CodeBlock { code: install, language: "toml", filename: Some("Cargo.toml".to_string()) }
            }

            // 5. Foundations and categories.
            section { class: "sc-section",
                SectionHead { eyebrow: "Fondamenta", title: "Tema, token, tipografia, icone" }
                SimpleGrid { cols: 1, sm: Some(2), lg: Some(4), spacing: GridSpacing::Md,
                    for p in FOUNDATION.iter().skip(1) {
                        Card {
                            key: "{p.name}",
                            title: Some(p.name.to_string()),
                            subtitle: Some(p.blurb.to_string()),
                            onclick: {
                                let r = p.route.clone();
                                move |_| {
                                    navigator.push(r.clone());
                                }
                            },
                            avatar: Some(rsx! {
                                div { class: "sc-icon-tile",
                                    Icon { kind: p.icon, size: 18 }
                                }
                            }),
                        }
                    }
                }
            }
            section { class: "sc-section",
                SectionHead { eyebrow: "Catalogo", title: "Otto categorie" }
                SimpleGrid { cols: 1, sm: Some(2), lg: Some(4), spacing: GridSpacing::Md,
                    for cat in Category::ALL {
                        {
                            let items = registry::by_category(cat);
                            let first = items.first().map(|s| s.slug).unwrap_or("buttons");
                            let count = if cat == Category::Fondamenta { items.len() + FOUNDATION.len() } else { items.len() };
                            let noun = if cat == Category::Fondamenta { "voci" } else { "componenti" };
                            rsx! {
                                Card {
                                    key: "{cat.label()}",
                                    title: Some(cat.label().to_string()),
                                    subtitle: Some(format!("{count} {noun} · {}", cat.blurb())),
                                    onclick: move |_| {
                                        if cat == Category::Fondamenta {
                                            navigator.push(Route::TokensPage {});
                                        } else {
                                            navigator.push(Route::story(first));
                                        }
                                    },
                                    avatar: Some(rsx! {
                                        div { class: "sc-icon-tile",
                                            Icon { kind: cat.icon(), size: 18 }
                                        }
                                    }),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// One preset rendered under its own ThemeProvider. The "Usa" button writes the GLOBAL theme,
/// which this component reads from the outer context (it sits outside the nested provider).
#[component]
fn PresetTile(index: usize) -> Element {
    let (preset, make) = PRESETS[index].clone();
    let mut local = use_signal(make);
    let mut global = use_theme();
    // The tile keeps its own accent but follows the global radius and glow, so the wall
    // answers the theme menu too.
    use_effect(move || {
        let g = global();
        let l = local.peek().clone();
        if l.radius_scale != g.radius_scale || l.glow_mode != g.glow_mode {
            let mut w = local.write();
            w.apply_radius_scale(g.radius_scale);
            w.apply_glow_mode(g.glow_mode);
        }
    });
    let active = global().primary_color == preset;
    let on = use_signal(|| true);
    let name = preset_name(&preset);
    rsx! {
        ThemeProvider { theme: Some(local),
            Card {
                class: "sc-preset-tile".to_string(),
                title: Some(name.clone()),
                glow: active.then_some(true),
                Stack { gap: SpacingToken::Sm,
                    Group { gap: SpacingToken::Sm, align: AlignItems::Center,
                        Button { size: ButtonSize::Sm, "Salva" }
                        Switch { checked: on, label: Some("Relay".to_string()) }
                        Badge { label: "Attivo".to_string(), variant: BadgeVariant::Primary, pill: true, dot: true }
                    }
                    ProgressBar { value: 64.0 }
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        full_width: true,
                        disabled: active,
                        onclick: {
                            let preset = preset.clone();
                            move |_| global.write().apply_primary_color(preset.clone())
                        },
                        if active {
                            "Preset attivo"
                        } else {
                            "Usa {name}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Stat(value: String, label: &'static str) -> Element {
    rsx! {
        Card { class: "sc-stat".to_string(),
            span { class: "sc-stat__value", "{value}" }
            span { class: "sc-eyebrow", "{label}" }
        }
    }
}
