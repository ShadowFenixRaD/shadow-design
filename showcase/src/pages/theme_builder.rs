use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::clipboard::copy_text;
use crate::components::{ChoicePicker, ControlGroup, CheckControl, PageHeader, ToggleControl};
use crate::components::CodeBlock;

pub const STORAGE_KEY: &str = "shadow_design_custom_theme";

fn is_valid_hex(hex: &str) -> bool {
    let s = hex.trim();
    if !s.starts_with('#') {
        return false;
    }
    let rest = &s[1..];
    (rest.len() == 3 || rest.len() == 6 || rest.len() == 8)
        && rest.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn save_theme_to_storage(theme: &Theme) {
    if let Ok(json) = serde_json::to_string(theme) {
        let js = format!(
            "try {{ localStorage.setItem('{}', {:?}); }} catch(e) {{}} return true;",
            STORAGE_KEY, json
        );
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

/// Local signal mirroring one field of the global theme (kept in sync both ways by the caller).
fn use_theme_field<T: Clone + PartialEq + 'static>(get: fn(&Theme) -> T) -> Signal<T> {
    let theme = use_theme();
    let mut sig = use_signal(|| get(&theme.peek()));
    use_effect(move || {
        let v = get(&theme());
        if *sig.peek() != v {
            sig.set(v);
        }
    });
    sig
}

/// One surface token: colour well, hex field and a bordered swatch so dark values stay visible.
#[component]
fn SurfaceToken(label: &'static str, hint: &'static str, field: u8, on_change: EventHandler<()>) -> Element {
    let mut theme = use_theme();
    let value = {
        let t = theme();
        match field {
            0 => t.surface_0,
            1 => t.surface_1,
            2 => t.surface_2,
            _ => t.surface_3,
        }
    };
    let mut set = move |v: String| {
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
    };
    rsx! {
        div { class: "sc-token",
            div { class: "sc-token__swatch", style: "background-color: {value};" }
            div { class: "sc-token__body",
                span { class: "sc-control-label", "{label}" }
                span { class: "sc-subtle-text", "{hint}" }
                div { class: "sc-token__inputs",
                    input {
                        r#type: "color",
                        class: "sc-color-input",
                        aria_label: "{label}",
                        value: "{value}",
                        oninput: move |e| set(e.value()),
                    }
                    input {
                        class: "sd-input sc-mono",
                        aria_label: "{label} (hex)",
                        value: "{value}",
                        oninput: move |e| set(e.value()),
                    }
                }
            }
        }
    }
}

/// Extra base class for one component (Level 2 override).
#[component]
fn OverrideField(component: &'static str, placeholder: &'static str, on_change: EventHandler<()>) -> Element {
    let mut theme = use_theme();
    rsx! {
        ControlGroup { label: format!("{component}: classe base aggiuntiva"),
            input {
                class: "sd-input sc-mono",
                aria_label: "{component}",
                placeholder,
                value: theme().components.get(component).and_then(|c| c.base_class.clone()).unwrap_or_default(),
                oninput: move |e| {
                    let val = e.value();
                    let mut ov = theme.write().components.remove(component).unwrap_or_default();
                    ov.base_class = if val.trim().is_empty() { None } else { Some(val) };
                    theme.write().components.insert(component.to_string(), ov);
                    on_change.call(());
                },
            }
        }
    }
}

#[component]
pub fn ThemeBuilderPage() -> Element {
    let mut theme = use_theme();
    let mut toast = use_notification();

    // Editor local controls
    let active_section = use_signal(|| "palette".to_string());
    let primary_sig = use_theme_field(|t| t.primary_color.clone());
    let radius_sig = use_theme_field(|t| t.radius_scale);
    let elevation_sig = use_theme_field(|t| t.elevation_style);
    let glow_sig = use_theme_field(|t| t.glow_mode);
    let auto_save = use_signal(|| true);
    let test_input_val = use_signal(|| "Valore di test reattivo".to_string());
    let test_checkbox = use_signal(|| true);
    let test_switch = use_signal(|| true);
    let mut test_dropdown_open = use_signal(|| false);
    let mut custom_hex_input = use_signal(|| {
        theme().primary_color.hex().to_string()
    });
    let mut custom_name_input = use_signal(|| "Custom Accent".to_string());
    let mut custom_prefix_input = use_signal(|| "custom".to_string());
    let mut json_input = use_signal(String::new);
    let mut json_parse_error = use_signal(|| Option::<String>::None);

    // Funzione helper per sincronizzare le modifiche e applicare auto-save
    let apply_change = move || {
        if *auto_save.read() {
            save_theme_to_storage(&theme());
        }
    };

    // Generazione codice Rust dinamico
    let generated_rust_code = {
        let t = theme();
        let primary_code = match &t.primary_color {
            PrimaryColor::Violet => "PrimaryColor::Violet".to_string(),
            PrimaryColor::Obsidian => "PrimaryColor::Obsidian".to_string(),
            PrimaryColor::Crimson => "PrimaryColor::Crimson".to_string(),
            PrimaryColor::Cyan => "PrimaryColor::Cyan".to_string(),
            PrimaryColor::Emerald => "PrimaryColor::Emerald".to_string(),
            PrimaryColor::Custom { name, hex, css_prefix } => {
                format!(
                    "PrimaryColor::custom_with_prefix(\"{}\", \"{}\", \"{}\")",
                    name,
                    hex,
                    css_prefix.as_deref().unwrap_or("custom")
                )
            }
        };

        let radius_code = match t.radius_scale {
            RadiusScale::Sharp => "RadiusScale::Sharp",
            RadiusScale::Medium => "RadiusScale::Medium",
            RadiusScale::Soft => "RadiusScale::Soft",
        };

        let elevation_code = match t.elevation_style {
            ElevationStyle::NeonGlow => "ElevationStyle::NeonGlow",
            ElevationStyle::Flat => "ElevationStyle::Flat",
        };

        let glow_code = match t.glow_mode {
            GlowMode::Off => "GlowMode::Off",
            GlowMode::Subtle => "GlowMode::Subtle",
            GlowMode::Expressive => "GlowMode::Expressive",
        };

        let mut overrides_code = String::new();
        for (comp_name, ovr) in &t.components {
            let mut chained = String::new();
            if let Some(base) = &ovr.base_class {
                if !base.is_empty() {
                    chained.push_str(&format!(".with_base_class(\"{base}\")"));
                }
            }
            for (v_name, v_class) in &ovr.variant_classes {
                chained.push_str(&format!(".with_variant(\"{v_name}\", \"{v_class}\")"));
            }
            for (s_name, s_class) in &ovr.size_classes {
                chained.push_str(&format!(".with_size(\"{s_name}\", \"{s_class}\")"));
            }
            if let Some(sh) = &ovr.shadow_class {
                chained.push_str(&format!(".with_shadow(\"{sh}\")"));
            }
            if !chained.is_empty() {
                overrides_code.push_str(&format!("\n        .override_component(\"{comp_name}\", |c| c{chained})"));
            }
        }

        format!(
r#"use shadow_design::theme::{{ThemeBuilder, PrimaryColor, RadiusScale, ElevationStyle, GlowMode}};

/// Genera il tema custom con tutte le personalizzazioni e override componenti.
pub fn build_my_custom_theme() -> shadow_design::theme::Theme {{
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

// Per renderlo attivo in tutta l'applicazione Dioxus:
// use_context_provider(|| Signal::new(build_my_custom_theme()));"#,
            name = t.name,
            primary = primary_code,
            radius = radius_code,
            elevation = elevation_code,
            glow = glow_code,
            s0 = t.surface_0,
            s1 = t.surface_1,
            s2 = t.surface_2,
            s3 = t.surface_3,
            overrides = overrides_code,
        )
    };

    let theme_json = serde_json::to_string_pretty(&theme()).unwrap_or_default();
    let rust_to_copy = generated_rust_code.clone();
    let json_to_copy = theme_json.clone();

    rsx! {
        PageHeader {
            title: "Theme Builder".to_string(),
            description: "Crea, personalizza e collauda il tema in tempo reale. Le modifiche valgono per tutto lo showcase e, con l'auto-save, restano dopo il ricaricamento.".to_string(),
        }

        div { class: "sc-stack",
            div { class: "sc-actions",
                Button {
                    variant: ButtonVariant::Primary,
                    size: ButtonSize::Sm,
                    left_icon: rsx! {
                        Icon { kind: IconKind::Check, size: 14 }
                    },
                    onclick: move |_| {
                        save_theme_to_storage(&theme());
                        toast.success("Tema salvato nel browser! Rimarrà attivo durante la navigazione e i riavvii.");
                    },
                    "Salva nel browser"
                }
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    left_icon: rsx! {
                        Icon { kind: IconKind::Copy, size: 14 }
                    },
                    onclick: move |_| {
                        let text = rust_to_copy.clone();
                        spawn(async move {
                            if copy_text(&text).await {
                                toast.info("Codice Rust (ThemeBuilder) copiato negli appunti!");
                            } else {
                                toast.danger("Impossibile copiare negli appunti.");
                            }
                        });
                    },
                    "Copia codice Rust"
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    size: ButtonSize::Sm,
                    left_icon: rsx! {
                        Icon { kind: IconKind::X, size: 14 }
                    },
                    onclick: move |_| {
                        clear_theme_from_storage();
                        *theme.write() = Theme::shadow();
                        toast.warning("Tema ripristinato ai valori predefiniti (Ultraviolet Shadow).");
                    },
                    "Ripristina default"
                }
            }

            Card {
                div { class: "sc-title-row",
                    CheckControl { checked: auto_save, label: "Auto-save su modifiche (localStorage)".to_string() }
                    span { class: "sc-counter", "Tema attivo: " strong { "{theme().name}" } }
                }
                hr { class: "sc-control-divider" }
                div { class: "sc-actions",
                    span { class: "sc-control-label", "Preset" }
                    for (label , make) in [
                        ("Shadow", Theme::shadow as fn() -> Theme),
                        ("Obsidian", Theme::obsidian as fn() -> Theme),
                        ("Crimson", Theme::crimson as fn() -> Theme),
                        ("Cyan", Theme::cyan as fn() -> Theme),
                        ("Emerald", Theme::emerald as fn() -> Theme),
                    ] {
                        Button {
                            key: "{label}",
                            variant: ButtonVariant::Secondary,
                            size: ButtonSize::Sm,
                            onclick: move |_| {
                                *theme.write() = make();
                                apply_change();
                                toast.info(format!("Applicato preset: {label}"));
                            },
                            "{label}"
                        }
                    }
                }
            }

            Card {
                title: "Configurazione parametri".to_string(),
                subtitle: "Modifica i parametri e osserva i componenti aggiornarsi all'istante".to_string(),
                Tabs {
                    div { class: "sc-panel__head sc-tabs-head",
                        TabList { aria_label: "Sezioni del builder",
                            Tab { value: "palette", active: active_section, icon: Some(IconKind::Palette), "Palette e superfici" }
                            Tab { value: "shape", active: active_section, icon: Some(IconKind::Box), "Raggi e glow" }
                            Tab { value: "overrides", active: active_section, icon: Some(IconKind::Layers), "Override componenti" }
                            Tab { value: "json", active: active_section, icon: Some(IconKind::Code), "JSON" }
                        }
                    }
                    div { class: "sc-tabs-body",
                        TabPanel { value: "palette", active: active_section,
                            div { class: "sc-builder-grid",
                                div { class: "sc-controls",
                                    ControlGroup { label: "Nome tema".to_string(),
                                        input {
                                            class: "sd-input",
                                            aria_label: "Nome tema",
                                            value: "{theme().name}",
                                            placeholder: "Es. Cyber Amber Studio...",
                                            oninput: move |e| {
                                                theme.write().name = e.value();
                                                apply_change();
                                            },
                                        }
                                    }
                                    ChoicePicker {
                                        label: "Colore primario".to_string(),
                                        options: vec![
                                            (PrimaryColor::Violet, "Violet".to_string()),
                                            (PrimaryColor::Obsidian, "Obsidian".to_string()),
                                            (PrimaryColor::Crimson, "Crimson".to_string()),
                                            (PrimaryColor::Cyan, "Cyan".to_string()),
                                            (PrimaryColor::Emerald, "Emerald".to_string()),
                                        ],
                                        value: primary_sig,
                                        on_change: move |p: PrimaryColor| {
                                            custom_hex_input.set(p.hex().to_string());
                                            theme.write().apply_primary_color(p);
                                            apply_change();
                                        },
                                    }
                                    hr { class: "sc-control-divider" }
                                    span { class: "sc-section-title", "Colore custom (HEX libero)" }
                                    ControlGroup { label: "Selettore / HEX".to_string(),
                                        div { class: "sc-token__inputs",
                                            input {
                                                r#type: "color",
                                                class: "sc-color-input",
                                                aria_label: "Selettore colore custom",
                                                value: "{custom_hex_input}",
                                                oninput: move |e| {
                                                    let val = e.value();
                                                    custom_hex_input.set(val.clone());
                                                    theme.write().apply_primary_color(PrimaryColor::custom_with_prefix(custom_name_input(), val, custom_prefix_input()));
                                                    apply_change();
                                                },
                                            }
                                            input {
                                                class: "sd-input sc-mono",
                                                aria_label: "HEX colore custom",
                                                value: "{custom_hex_input}",
                                                oninput: move |e| {
                                                    let val = e.value();
                                                    custom_hex_input.set(val.clone());
                                                    if is_valid_hex(&val) {
                                                        theme.write().apply_primary_color(PrimaryColor::custom_with_prefix(custom_name_input(), val, custom_prefix_input()));
                                                        apply_change();
                                                    }
                                                },
                                            }
                                        }
                                    }
                                    ControlGroup { label: "Nome colore".to_string(),
                                        input {
                                            class: "sd-input",
                                            aria_label: "Nome colore",
                                            value: "{custom_name_input}",
                                            oninput: move |e| {
                                                let val = e.value();
                                                custom_name_input.set(val.clone());
                                                theme.write().apply_primary_color(PrimaryColor::custom_with_prefix(val, custom_hex_input(), custom_prefix_input()));
                                                apply_change();
                                            },
                                        }
                                    }
                                    ControlGroup { label: "CSS prefix".to_string(),
                                        input {
                                            class: "sd-input sc-mono",
                                            aria_label: "CSS prefix",
                                            value: "{custom_prefix_input}",
                                            oninput: move |e| {
                                                let val = e.value();
                                                custom_prefix_input.set(val.clone());
                                                theme.write().apply_primary_color(PrimaryColor::custom_with_prefix(custom_name_input(), custom_hex_input(), val));
                                                apply_change();
                                            },
                                        }
                                    }
                                }

                                div { class: "sc-controls",
                                    span { class: "sc-control-label", "Token superfici (CSS variables)" }
                                    SurfaceToken { label: "surface_0", hint: "Void, sfondo pagina", field: 0, on_change: move |_| apply_change() }
                                    SurfaceToken { label: "surface_1", hint: "Contenitori", field: 1, on_change: move |_| apply_change() }
                                    SurfaceToken { label: "surface_2", hint: "Layer flottanti", field: 2, on_change: move |_| apply_change() }
                                    SurfaceToken { label: "surface_3", hint: "Hover nei layer, secondario", field: 3, on_change: move |_| apply_change() }
                                }
                            }
                        }

                        TabPanel { value: "shape", active: active_section,
                            div { class: "sc-controls",
                                ChoicePicker {
                                    label: "Raggio di curvatura".to_string(),
                                    options: vec![
                                        (RadiusScale::Sharp, "Sharp".to_string()),
                                        (RadiusScale::Medium, "Medium".to_string()),
                                        (RadiusScale::Soft, "Soft".to_string()),
                                    ],
                                    value: radius_sig,
                                    on_change: move |r: RadiusScale| {
                                        theme.write().apply_radius_scale(r);
                                        apply_change();
                                    },
                                }
                                ChoicePicker {
                                    label: "Intensità glow".to_string(),
                                    options: vec![
                                        (GlowMode::Off, "Off".to_string()),
                                        (GlowMode::Subtle, "Subtle".to_string()),
                                        (GlowMode::Expressive, "Expressive".to_string()),
                                    ],
                                    value: glow_sig,
                                    on_change: move |g: GlowMode| {
                                        theme.write().apply_glow_mode(g);
                                        apply_change();
                                    },
                                }
                                ChoicePicker {
                                    label: "Elevazione".to_string(),
                                    options: vec![
                                        (ElevationStyle::NeonGlow, "Glow attivo".to_string()),
                                        (ElevationStyle::Flat, "Flat (nessun glow)".to_string()),
                                    ],
                                    value: elevation_sig,
                                    on_change: move |e: ElevationStyle| {
                                        theme.write().apply_elevation_style(e);
                                        apply_change();
                                    },
                                }
                            }
                        }

                        TabPanel { value: "overrides", active: active_section,
                            div { class: "sc-controls",
                                p { class: "sc-muted-text",
                                    "Il livello 2 applica classi personalizzate a singoli componenti. Gli override hanno priorità automatica grazie ai cascade layer."
                                }
                                OverrideField { component: "button", placeholder: "Es. my-btn", on_change: move |_| apply_change() }
                                OverrideField { component: "card", placeholder: "Es. my-card", on_change: move |_| apply_change() }
                                OverrideField { component: "badge", placeholder: "Es. my-badge", on_change: move |_| apply_change() }
                            }
                        }

                        TabPanel { value: "json", active: active_section,
                            div { class: "sc-controls",
                                div { class: "sc-title-row",
                                    span { class: "sc-control-label", "JSON serializzato" }
                                    Button {
                                        size: ButtonSize::Xs,
                                        variant: ButtonVariant::Secondary,
                                        left_icon: rsx! {
                                            Icon { kind: IconKind::Copy, size: 12 }
                                        },
                                        onclick: move |_| {
                                            let text = json_to_copy.clone();
                                            spawn(async move {
                                                if copy_text(&text).await {
                                                    toast.info("JSON del tema copiato negli appunti!");
                                                } else {
                                                    toast.danger("Impossibile copiare negli appunti.");
                                                }
                                            });
                                        },
                                        "Copia JSON"
                                    }
                                }
                                CodeBlock { code: theme_json.clone(), language: "json" }
                                hr { class: "sc-control-divider" }
                                ControlGroup { label: "Incolla JSON per importare".to_string(),
                                    textarea {
                                        class: "sd-input sc-mono sc-textarea",
                                        aria_label: "JSON da importare",
                                        placeholder: "Incolla qui JSON valido...",
                                        value: "{json_input}",
                                        oninput: move |e| {
                                            json_input.set(e.value());
                                            json_parse_error.set(None);
                                        },
                                    }
                                }
                                if let Some(err_msg) = json_parse_error() {
                                    p { class: "sc-danger-text sc-mono", "{err_msg}" }
                                }
                                div { class: "sc-actions",
                                    Button {
                                        variant: ButtonVariant::Secondary,
                                        onclick: move |_| {
                                            let text = json_input();
                                            match serde_json::from_str::<Theme>(&text) {
                                                Ok(loaded) => {
                                                    *theme.write() = loaded;
                                                    apply_change();
                                                    json_parse_error.set(None);
                                                    toast.success("Tema importato con successo!");
                                                }
                                                Err(err) => {
                                                    let err_msg = format!("Errore parsing JSON: {}", err);
                                                    json_parse_error.set(Some(err_msg.clone()));
                                                    toast.danger(err_msg);
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

            // Live preview of the real components under the current theme.
            div { class: "sc-title-row",
                h2 { class: "sc-section-title", "Anteprima live" }
                span { class: "sc-subtle-text", "Tutti i componenti reagiscono al tema" }
            }

            Card {
                title: "Pulsanti".to_string(),
                subtitle: "Varianti, taglie e stati".to_string(),
                div { class: "sc-stack",
                    div { class: "sc-actions",
                        Button { variant: ButtonVariant::Primary, "Primary" }
                        Button { variant: ButtonVariant::Secondary, "Secondary" }
                        Button { variant: ButtonVariant::Outline, "Outline" }
                        Button { variant: ButtonVariant::Ghost, "Ghost" }
                        Button { variant: ButtonVariant::Danger, "Danger" }
                        Button { variant: ButtonVariant::Success, "Success" }
                    }
                    hr { class: "sc-control-divider" }
                    div { class: "sc-actions",
                        Button {
                            size: ButtonSize::Sm,
                            variant: ButtonVariant::Secondary,
                            left_icon: rsx! {
                                Icon { kind: IconKind::Sparkles, size: 13 }
                            },
                            "Taglia Sm"
                        }
                        Button {
                            size: ButtonSize::Md,
                            variant: ButtonVariant::Secondary,
                            right_icon: rsx! {
                                Icon { kind: IconKind::ExternalLink, size: 14 }
                            },
                            "Taglia Md"
                        }
                        Button {
                            size: ButtonSize::Lg,
                            variant: ButtonVariant::Secondary,
                            left_icon: rsx! {
                                Icon { kind: IconKind::Check, size: 16 }
                            },
                            "Taglia Lg"
                        }
                        Button { variant: ButtonVariant::Secondary, loading: true }
                    }
                }
            }

            div { class: "sc-card-grid sc-card-grid--3",
                Card {
                    title: "Card standard".to_string(),
                    subtitle: "Contenitore statico".to_string(),
                    footer: rsx! {
                        div { class: "sc-title-row",
                            span { class: "sc-subtle-text", "Footer" }
                            Button { size: ButtonSize::Xs, variant: ButtonVariant::Ghost, "Azione" }
                        }
                    },
                    p { class: "sc-muted-text", "Raggi ed elevazione seguono il tema in modo reattivo." }
                }
                Card { variant: CardVariant::Interactive,
                    div { class: "sc-stack",
                        span { class: "sc-control-label", "Card interattiva" }
                        p { class: "sc-muted-text", "L'intera card è cliccabile: bordo ed elevazione reagiscono al passaggio." }
                    }
                }
                Card { variant: CardVariant::Metric,
                    div { class: "sc-title-row",
                        div { class: "sc-stack",
                            span { class: "sc-control-label", "Metrica" }
                            span { class: "sc-section-title", "99.98%" }
                            span { class: "sc-success-text sc-mono", "Runtime ottimizzato" }
                        }
                        Icon { kind: IconKind::Sparkles, size: 18 }
                    }
                }
            }

            div { class: "sc-card-grid",
                Card {
                    title: "Input e dropdown".to_string(),
                    subtitle: "Campi recessi e menu ancorato".to_string(),
                    div { class: "sc-stack",
                        TextInput {
                            value: test_input_val,
                            label: "Input con icona e clear".to_string(),
                            placeholder: "Scrivi qualcosa...".to_string(),
                            left_icon: Some(IconKind::Terminal.into()),
                            clearable: true,
                            help_text: "Modifica il testo per verificare il binding reattivo.".to_string(),
                        }
                        Dropdown {
                            is_open: test_dropdown_open,
                            placement: DropdownPlacement::BottomLeft,
                            trigger: rsx! {
                                Button {
                                    variant: ButtonVariant::Secondary,
                                    size: ButtonSize::Sm,
                                    right_icon: rsx! {
                                        Icon { kind: IconKind::ChevronDown, size: 14 }
                                    },
                                    "Apri dropdown"
                                }
                            },
                            DropdownItem {
                                icon: Some(IconKind::Sparkles),
                                onclick: move |_| {
                                    toast.info("Azione eseguita dal Dropdown!");
                                    test_dropdown_open.set(false);
                                },
                                "Azione rapida"
                            }
                            DropdownItem {
                                icon: Some(IconKind::Settings),
                                onclick: move |_| {
                                    test_dropdown_open.set(false);
                                },
                                "Impostazioni"
                            }
                            DropdownDivider {}
                            DropdownItem {
                                icon: Some(IconKind::XCircle),
                                danger: true,
                                onclick: move |_| {
                                    toast.danger("Azione distruttiva annullata.");
                                    test_dropdown_open.set(false);
                                },
                                "Elimina risorsa"
                            }
                        }
                    }
                }

                Card {
                    title: "Badge, checkbox e switch".to_string(),
                    subtitle: "Stati di selezione e pillole".to_string(),
                    div { class: "sc-stack",
                        div { class: "sc-actions",
                            Badge { label: "Default".to_string(), variant: BadgeVariant::Default }
                            Badge { label: "Primary".to_string(), variant: BadgeVariant::Primary }
                            Badge { label: "Success".to_string(), variant: BadgeVariant::Success, dot: true }
                            Badge { label: "Warning".to_string(), variant: BadgeVariant::Warning }
                            Badge { label: "Danger".to_string(), variant: BadgeVariant::Danger }
                        }
                        hr { class: "sc-control-divider" }
                        CheckControl {
                            checked: test_checkbox,
                            label: "Checkbox di verifica".to_string(),
                            description: Some("Controlla il feedback visivo al click.".to_string()),
                        }
                        ToggleControl {
                            checked: test_switch,
                            label: "Switch".to_string(),
                            description: Some("Segue l'accento del tema.".to_string()),
                        }
                    }
                }
            }

            div { class: "sc-card-grid",
                Alert { title: "Informazione di sistema".to_string(), variant: AlertVariant::Info,
                    "Il tema appena creato è attivo in tutta l'applicazione."
                }
                Alert { title: "Operazione riuscita".to_string(), variant: AlertVariant::Success,
                    "Bordi e contrasti sono calibrati secondo il linguaggio Lit Glass."
                }
            }

            Card {
                title: "Codice Rust generato".to_string(),
                subtitle: "Incolla questo codice nella tua applicazione Dioxus".to_string(),
                CodeBlock { code: generated_rust_code, language: "rust" }
            }
        }
    }
}
