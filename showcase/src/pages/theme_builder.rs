//! Theme Builder: edit the global theme live and export it as Rust or JSON (REVAMP §2.4).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::app::theme_menu::{preset_name, ThemeControls, ThemeSwatches};
use crate::components::clipboard::copy_text;
use crate::scenes::SettingsForm;
use crate::story::sections::SectionHead;

pub const STORAGE_KEY: &str = "shadow_design_custom_theme";

pub fn save_theme_to_storage(theme: &Theme) {
    if let Ok(json) = serde_json::to_string(theme) {
        let js = format!("try {{ localStorage.setItem('{}', {:?}); }} catch(e) {{}} return true;", STORAGE_KEY, json);
        let eval = document::eval(&js);
        spawn(async move {
            let _ = eval.await;
        });
    }
}

pub fn clear_theme_from_storage() {
    let js = format!("try {{ localStorage.removeItem('{}'); }} catch(e) {{}} return true;", STORAGE_KEY);
    let eval = document::eval(&js);
    spawn(async move {
        let _ = eval.await;
    });
}

/// A local text signal mirroring one string field of the global theme, both ways.
fn use_theme_text(get: fn(&Theme) -> String, set: fn(&mut Theme, String), on_change: impl Fn() + 'static) -> Signal<String> {
    let mut theme = use_theme();
    let mut local = use_signal(|| get(&theme.peek()));
    // Theme → field (preset applied elsewhere, JSON import, reset).
    use_effect(move || {
        let v = get(&theme());
        if *local.peek() != v {
            local.set(v);
        }
    });
    // Field → theme.
    use_effect(move || {
        let v = local();
        if get(&theme.peek()) != v {
            set(&mut theme.write(), v);
            on_change();
        }
    });
    local
}

fn rust_code(t: &Theme) -> String {
    let primary = match &t.primary_color {
        PrimaryColor::Violet => "PrimaryColor::Violet".to_string(),
        PrimaryColor::Obsidian => "PrimaryColor::Obsidian".to_string(),
        PrimaryColor::Crimson => "PrimaryColor::Crimson".to_string(),
        PrimaryColor::Cyan => "PrimaryColor::Cyan".to_string(),
        PrimaryColor::Emerald => "PrimaryColor::Emerald".to_string(),
        PrimaryColor::Custom { name, hex, css_prefix } => {
            format!("PrimaryColor::custom_with_prefix(\"{name}\", \"{hex}\", \"{}\")", css_prefix.as_deref().unwrap_or("custom"))
        }
    };
    let radius = match t.radius_scale {
        RadiusScale::Sharp => "RadiusScale::Sharp",
        RadiusScale::Medium => "RadiusScale::Medium",
        RadiusScale::Soft => "RadiusScale::Soft",
    };
    let elevation = match t.elevation_style {
        ElevationStyle::NeonGlow => "ElevationStyle::NeonGlow",
        ElevationStyle::Flat => "ElevationStyle::Flat",
    };
    let glow = match t.glow_mode {
        GlowMode::Off => "GlowMode::Off",
        GlowMode::Subtle => "GlowMode::Subtle",
        GlowMode::Expressive => "GlowMode::Expressive",
    };
    let mut overrides = String::new();
    for (comp, ovr) in &t.components {
        let mut chained = String::new();
        if let Some(base) = ovr.base_class.as_deref().filter(|b| !b.is_empty()) {
            chained.push_str(&format!(".with_base_class(\"{base}\")"));
        }
        for (v, c) in &ovr.variant_classes {
            chained.push_str(&format!(".with_variant(\"{v}\", \"{c}\")"));
        }
        for (s, c) in &ovr.size_classes {
            chained.push_str(&format!(".with_size(\"{s}\", \"{c}\")"));
        }
        if let Some(sh) = &ovr.shadow_class {
            chained.push_str(&format!(".with_shadow(\"{sh}\")"));
        }
        if !chained.is_empty() {
            overrides.push_str(&format!("\n        .override_component(\"{comp}\", |c| c{chained})"));
        }
    }
    format!(
        r#"use shadow_design::theme::{{ElevationStyle, GlowMode, PrimaryColor, RadiusScale, Theme, ThemeBuilder}};

/// The custom theme designed in the Theme Builder.
pub fn my_theme() -> Theme {{
    ThemeBuilder::new()
        .name("{name}")
        .primary_color({primary})
        .radius_scale({radius})
        .elevation_style({elevation})
        .glow_mode({glow})
        .surface_0("{s0}")
        .surface_1("{s1}")
        .surface_2("{s2}")
        .surface_3("{s3}"){overrides}
        .build()
}}

// In the app root:
// let theme = use_signal(my_theme);
// rsx! {{ ThemeProvider {{ theme: Some(theme), App {{}} }} }}"#,
        name = t.name,
        s0 = t.surface_0,
        s1 = t.surface_1,
        s2 = t.surface_2,
        s3 = t.surface_3,
    )
}

/// One surface token: a colour field bound to `surface_<n>` of the global theme.
#[component]
fn SurfaceToken(label: &'static str, hint: &'static str, field: u8, on_change: EventHandler<()>) -> Element {
    let mut theme = use_theme();
    let get = move |t: &Theme| match field {
        0 => t.surface_0.clone(),
        1 => t.surface_1.clone(),
        2 => t.surface_2.clone(),
        _ => t.surface_3.clone(),
    };
    let mut value = use_signal(|| get(&theme.peek()));
    use_effect(move || {
        let v = get(&theme());
        if *value.peek() != v {
            value.set(v);
        }
    });
    rsx! {
        ColorInput {
            value,
            label: Some(label.to_string()),
            description: Some(hint.to_string()),
            on_change: move |v: String| {
                if is_valid_hex_color(&v) {
                    {
                        let mut t = theme.write();
                        match field {
                            0 => t.surface_0 = v,
                            1 => t.surface_1 = v,
                            2 => t.surface_2 = v,
                            _ => t.surface_3 = v,
                        }
                    }
                    on_change.call(());
                }
            },
        }
    }
}

/// Extra base class for one component (ComponentOverride, level 2).
#[component]
fn OverrideField(component: &'static str, placeholder: &'static str, on_change: EventHandler<()>) -> Element {
    let mut theme = use_theme();
    let current = move |t: &Theme| t.components.get(component).and_then(|c| c.base_class.clone()).unwrap_or_default();
    let value = use_signal(|| current(&theme.peek()));
    use_effect(move || {
        let v = value();
        if current(&theme.peek()) != v {
            let mut t = theme.write();
            let mut ov = t.components.remove(component).unwrap_or_default();
            ov.base_class = if v.trim().is_empty() { None } else { Some(v) };
            t.components.insert(component.to_string(), ov);
            drop(t);
            on_change.call(());
        }
    });
    rsx! {
        TextInput {
            value,
            label: Some(format!("{component}: classe base aggiuntiva")),
            placeholder: Some(placeholder.to_string()),
            clearable: true,
        }
    }
}

/// The current theme on the preset-wall tile (same content as the home tiles).
#[component]
fn CurrentThemeTile() -> Element {
    let theme = use_theme();
    let on = use_signal(|| true);
    let name = preset_name(&theme().primary_color);
    rsx! {
        Card { class: "sc-preset-tile".to_string(), title: Some(name), subtitle: Some("Tessera del tema attivo".to_string()),
            Stack { gap: SpacingToken::Sm,
                Group { gap: SpacingToken::Sm, align: AlignItems::Center,
                    Button { size: ButtonSize::Sm, "Salva" }
                    Switch { checked: on, label: Some("Relay".to_string()) }
                    Badge { label: "Attivo".to_string(), variant: BadgeVariant::Primary, pill: true, dot: true }
                }
                ProgressBar { value: 64.0 }
            }
        }
    }
}

#[component]
pub fn ThemeBuilderPage() -> Element {
    let mut theme = use_theme();
    let mut toast = use_notification();
    let section = use_signal(|| "palette".to_string());
    let auto_save = use_signal(|| true);
    let apply_change = move || {
        if *auto_save.peek() {
            save_theme_to_storage(&theme.peek());
        }
    };

    let name = use_theme_text(|t| t.name.clone(), |t, v| t.name = v, apply_change);

    let mut custom_hex = use_signal(|| theme.peek().primary_color.hex().to_string());
    let custom_name = use_signal(|| "Custom Accent".to_string());
    let custom_prefix = use_signal(|| "custom".to_string());

    let mut elevation = use_signal(|| elevation_key(theme.peek().elevation_style).to_string());
    use_effect(move || {
        let t = theme();
        if elevation.peek().as_str() != elevation_key(t.elevation_style) {
            elevation.set(elevation_key(t.elevation_style).to_string());
        }
        let hex = t.primary_color.hex().to_string();
        if *custom_hex.peek() != hex {
            custom_hex.set(hex);
        }
    });

    let json_input = use_signal(String::new);
    let mut json_error = use_signal(|| Option::<String>::None);

    let t = theme();
    let generated = rust_code(&t);
    let theme_json = serde_json::to_string_pretty(&t).unwrap_or_default();
    let rust_to_copy = generated.clone();

    rsx! {
        document::Title { "Theme Builder · Shadow Design" }
        div { class: "sc-story",
            header { class: "sc-story__head",
                span { class: "sc-eyebrow", "Fondamenta · feature theme" }
                h1 { class: "sc-title", "Theme Builder" }
                p { class: "sc-lede",
                    "Progetta il tema e collaudalo dal vivo: ogni modifica vale per tutto lo showcase. Esporta il risultato come codice Rust o JSON."
                }
                Group { gap: SpacingToken::Sm,
                    Button {
                        size: ButtonSize::Sm,
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Check, size: 14 }
                        }),
                        onclick: move |_| {
                            save_theme_to_storage(&theme.peek());
                            toast.success("Tema salvato nel browser");
                        },
                        "Salva nel browser"
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Copy, size: 14 }
                        }),
                        onclick: move |_| {
                            let text = rust_to_copy.clone();
                            spawn(async move {
                                if copy_text(&text).await {
                                    toast.info("Codice Rust copiato negli appunti");
                                } else {
                                    toast.danger("Impossibile copiare negli appunti");
                                }
                            });
                        },
                        "Copia codice Rust"
                    }
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::X, size: 14 }
                        }),
                        onclick: move |_| {
                            clear_theme_from_storage();
                            *theme.write() = Theme::shadow();
                            toast.warning("Tema ripristinato: Ultraviolet");
                        },
                        "Ripristina default"
                    }
                }
            }

            section { class: "sc-section",
                SectionHead { eyebrow: "Preset", title: "Punto di partenza", desc: Some("Un preset sostituisce l'intero tema; poi modifica i singoli valori.") }
                Card {
                    div { class: "sc-builder-presets",
                        Group { gap: SpacingToken::Sm,
                            for (label , make) in [
                                ("Shadow", Theme::shadow as fn() -> Theme),
                                ("Obsidian", Theme::obsidian as fn() -> Theme),
                                ("Crimson", Theme::crimson as fn() -> Theme),
                                ("Cyan", Theme::cyan as fn() -> Theme),
                                ("Emerald", Theme::emerald as fn() -> Theme),
                            ]
                            {
                                Button {
                                    key: "{label}",
                                    variant: ButtonVariant::Secondary,
                                    size: ButtonSize::Sm,
                                    onclick: move |_| {
                                        *theme.write() = make();
                                        apply_change();
                                        toast.info(format!("Preset {label} applicato"));
                                    },
                                    "{label}"
                                }
                            }
                        }
                        div { class: "sc-builder-presets__meta",
                            Checkbox {
                                checked: auto_save,
                                label: Some("Auto-save su modifiche (localStorage)".to_string()),
                                class: "sc-wrap-label",
                            }
                            span { class: "sc-hint", "Tema attivo: {t.name}" }
                        }
                    }
                }
            }

            div { class: "sc-builder-grid",
                section { class: "sc-section",
                    SectionHead { eyebrow: "Parametri", title: "Configurazione" }
                    Card {
                        Tabs {
                            TabList { aria_label: Some("Sezioni del builder".to_string()),
                                Tab { value: "palette", active: section, icon: Some(IconKind::Palette), "Palette" }
                                Tab { value: "shape", active: section, icon: Some(IconKind::Box), "Forma" }
                                Tab { value: "overrides", active: section, icon: Some(IconKind::Layers), "Override" }
                                Tab { value: "json", active: section, icon: Some(IconKind::Code), "JSON" }
                            }
                            TabPanel { value: "palette", active: section,
                                div { class: "sc-builder-panel",
                                    TextInput { value: name, label: Some("Nome del tema".to_string()), placeholder: Some("Es. Cyber Amber Studio".to_string()) }
                                    div { class: "sc-knob",
                                        span { class: "sc-eyebrow", "Accento preset" }
                                        ThemeSwatches {}
                                    }
                                    Divider { label: Some("Accento libero".to_string()) }
                                    ColorInput {
                                        value: custom_hex,
                                        label: Some("Colore HEX".to_string()),
                                        description: Some("Qualsiasi hex: accento, derivate e focus seguono.".to_string()),
                                        on_change: move |v: String| {
                                            if is_valid_hex_color(&v) {
                                                theme.write().apply_primary_color(PrimaryColor::custom_with_prefix(custom_name(), v, custom_prefix()));
                                                apply_change();
                                            }
                                        },
                                    }
                                    TextInput { value: custom_name, label: Some("Nome del colore".to_string()) }
                                    TextInput { value: custom_prefix, label: Some("Prefisso CSS".to_string()), help_text: Some("Solo informativo: nessuna classe dipende dal prefisso.".to_string()) }
                                    Divider { label: Some("Superfici".to_string()) }
                                    SurfaceToken { label: "surface_0", hint: "Vuoto, sfondo pagina", field: 0, on_change: move |_| apply_change() }
                                    SurfaceToken { label: "surface_1", hint: "Contenitori", field: 1, on_change: move |_| apply_change() }
                                    SurfaceToken { label: "surface_2", hint: "Layer flottanti", field: 2, on_change: move |_| apply_change() }
                                    SurfaceToken { label: "surface_3", hint: "Hover nei layer, secondario", field: 3, on_change: move |_| apply_change() }
                                }
                            }
                            TabPanel { value: "shape", active: section,
                                div { class: "sc-builder-panel",
                                    ThemeControls {}
                                    div { class: "sc-knob",
                                        span { class: "sc-eyebrow", "Elevazione" }
                                        SegmentedControl {
                                            size: SegmentedControlSize::Sm,
                                            full_width: true,
                                            options: vec![SegmentOption::new("neon", "Glow attivo"), SegmentOption::new("flat", "Flat")],
                                            value: elevation,
                                            on_change: move |v: String| {
                                                theme.write().apply_elevation_style(if v == "flat" { ElevationStyle::Flat } else { ElevationStyle::NeonGlow });
                                                apply_change();
                                            },
                                        }
                                        span { class: "sc-hint", "Flat spegne il glow come GlowMode::Off." }
                                    }
                                }
                            }
                            TabPanel { value: "overrides", active: section,
                                div { class: "sc-builder-panel",
                                    p { class: "sc-muted-text",
                                        "ComponentOverride aggiunge classi tue a un componente. È il modo supportato per personalizzarlo senza toccare la libreria."
                                    }
                                    OverrideField { component: "button", placeholder: "Es. my-btn", on_change: move |_| apply_change() }
                                    OverrideField { component: "card", placeholder: "Es. my-card", on_change: move |_| apply_change() }
                                    OverrideField { component: "badge", placeholder: "Es. my-badge", on_change: move |_| apply_change() }
                                }
                            }
                            TabPanel { value: "json", active: section,
                                div { class: "sc-builder-panel",
                                    Group { gap: SpacingToken::Sm, justify: JustifyContent::SpaceBetween, align: AlignItems::Center,
                                        span { class: "sc-eyebrow", "JSON serializzato" }
                                        CopyButton { value: theme_json.clone(), label: Some("Copia JSON".to_string()) }
                                    }
                                    CodeBlock { code: theme_json.clone(), language: "json" }
                                    Textarea {
                                        value: json_input,
                                        label: Some("Importa da JSON".to_string()),
                                        placeholder: Some("Incolla qui un tema esportato".to_string()),
                                        min_rows: 4,
                                        error: json_error(),
                                    }
                                    Group { gap: SpacingToken::Sm,
                                        Button {
                                            variant: ButtonVariant::Secondary,
                                            onclick: move |_| {
                                                match serde_json::from_str::<Theme>(&json_input()) {
                                                    Ok(loaded) => {
                                                        *theme.write() = loaded;
                                                        apply_change();
                                                        json_error.set(None);
                                                        toast.success("Tema importato");
                                                    }
                                                    Err(err) => {
                                                        json_error.set(Some(format!("JSON non valido: {err}")));
                                                    }
                                                }
                                            },
                                            "Importa"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                section { class: "sc-section",
                    SectionHead { eyebrow: "Anteprima", title: "Il tema dal vivo", desc: Some("Componenti reali sotto il tema corrente.") }
                    div { class: "sc-builder-preview",
                        CurrentThemeTile {}
                        div { class: "sc-scene",
                            SettingsForm { compact: true }
                        }
                    }
                }
            }

            section { class: "sc-section",
                SectionHead { eyebrow: "Esporta", title: "Codice Rust generato", desc: Some("Incolla questa funzione nella tua app Dioxus.") }
                CodeBlock { code: generated, language: "rust", filename: Some("theme.rs".to_string()) }
            }
        }
    }
}

fn elevation_key(e: ElevationStyle) -> &'static str {
    match e {
        ElevationStyle::NeonGlow => "neon",
        ElevationStyle::Flat => "flat",
    }
}
