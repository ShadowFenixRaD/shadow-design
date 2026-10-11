use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::CodeBlock;

pub const STORAGE_KEY: &str = "shadow_design_custom_theme";

pub fn save_theme_to_storage(theme: &Theme) {
    if let Ok(json) = serde_json::to_string(theme) {
        let js = format!(
            "try {{ localStorage.setItem('{}', {:?}); }} catch(e) {{}}",
            STORAGE_KEY, json
        );
        let _ = document::eval(&js);
    }
}

pub fn clear_theme_from_storage() {
    let js = format!("try {{ localStorage.removeItem('{}'); }} catch(e) {{}}", STORAGE_KEY);
    let _ = document::eval(&js);
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum BuilderSection {
    #[default]
    Palette,
    Shape,
    Overrides,
    Json,
}

#[component]
pub fn ThemeBuilderPage() -> Element {
    let mut theme = use_theme();
    let mut toast = use_notification();

    // Editor local controls
    let mut active_section = use_signal(|| BuilderSection::Palette);
    let mut auto_save = use_signal(|| true);
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
            ElevationStyle::HardBlock => "ElevationStyle::HardBlock",
            ElevationStyle::SoftElevated => "ElevationStyle::SoftElevated",
            ElevationStyle::Flat => "ElevationStyle::Flat",
        };

        let glow_code = match t.glow_mode {
            GlowMode::Off => "GlowMode::Off",
            GlowMode::Subtle => "GlowMode::Subtle",
            GlowMode::Expressive => "GlowMode::Expressive",
        };

        format!(
r#"use shadow_design::theme::{{ThemeBuilder, PrimaryColor, RadiusScale, ElevationStyle, GlowMode, ComponentOverride}};

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
        .surface_3("{s3}")
        .build()
}}"#,
            name = t.name,
            primary = primary_code,
            radius = radius_code,
            elevation = elevation_code,
            glow = glow_code,
            s0 = t.surface_0,
            s1 = t.surface_1,
            s2 = t.surface_2,
            s3 = t.surface_3,
        )
    };

    let theme_json = serde_json::to_string_pretty(&theme()).unwrap_or_default();
    let rust_to_copy = generated_rust_code.clone();
    let json_to_copy = theme_json.clone();

    rsx! {
        div { class: "max-w-6xl space-y-8 pb-12",
            // Header e Toolbar Azioni
            div { class: "flex flex-col md:flex-row md:items-center justify-between gap-4 pb-4 border-b border-white/[0.08]",
                div { class: "space-y-1",
                    div { class: "flex items-center gap-2.5",
                        Heading { order: HeadingOrder::H2, "Theme Builder & Live Studio" }
                        Badge {
                            label: "Multi-Pagina • Persistente".to_string(),
                            variant: BadgeVariant::Success,
                            dot: true,
                            pill: true,
                        }
                    }
                    Text { dimmed: true, size: TextSize::Sm,
                        "Crea, personalizza e collauda in tempo reale il tuo tema multi-livello. Le modifiche persistono automaticamente attraverso riavvii, ricaricamenti di pagina e navigazione in tutto lo showcase."
                    }
                }

                // Azioni rapide: Salva, Reset, Export
                div { class: "flex flex-wrap items-center gap-2.5 shrink-0",
                    Button {
                        variant: ButtonVariant::Primary,
                        size: ButtonSize::Sm,
                        left_icon: rsx! { Icon { kind: IconKind::Check, size: 14 } },
                        onclick: move |_| {
                            save_theme_to_storage(&theme());
                            toast.success("Tema salvato nel browser! Rimarrà attivo durante la navigazione e i riavvii.");
                        },
                        "Salva nel Browser"
                    }

                    Button {
                        variant: ButtonVariant::Secondary,
                        size: ButtonSize::Sm,
                        left_icon: rsx! { Icon { kind: IconKind::Copy, size: 14 } },
                        onclick: move |_| {
                            let _ = document::eval(&format!("navigator.clipboard.writeText({:?});", rust_to_copy));
                            toast.info("Codice Rust (ThemeBuilder) copiato negli appunti!");
                        },
                        "Copia Rust Code"
                    }

                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        left_icon: rsx! { Icon { kind: IconKind::X, size: 14 } },
                        onclick: move |_| {
                            clear_theme_from_storage();
                            *theme.write() = Theme::shadow();
                            toast.warning("Tema ripristinato ai valori predefiniti (Ultraviolet Shadow).");
                        },
                        "Ripristina Default"
                    }
                }
            }

            // Barra di Stato Persistenza & Preset di Partenza Rapidi
            Card {
                shadow: CardShadow::Sm,
                div { class: "flex flex-wrap items-center justify-between gap-4",
                    div { class: "flex items-center gap-3",
                        label { class: "flex items-center gap-2 cursor-pointer text-xs font-mono text-zinc-300 select-none",
                            input {
                                r#type: "checkbox",
                                checked: *auto_save.read(),
                                onchange: move |e| *auto_save.write() = e.checked(),
                            }
                            span { "Auto-save su modifiche (localStorage)" }
                        }
                        span { class: "text-zinc-600", "•" }
                        span { class: "text-xs font-mono text-purple-300",
                            "Tema Attivo: "
                            strong { "{theme().name}" }
                        }
                    }

                    // Bottoni Preset Rapidi
                    div { class: "flex flex-wrap items-center gap-1.5",
                        span { class: "text-[11px] font-mono text-zinc-500 uppercase mr-1", "Preset:" }
                        button {
                            r#type: "button",
                            class: "px-2.5 py-1 text-xs font-mono rounded bg-white/[0.04] hover:bg-white/[0.08] text-purple-300 border border-purple-500/30 cursor-pointer transition-colors",
                            onclick: move |_| {
                                *theme.write() = Theme::shadow();
                                apply_change();
                                toast.info("Applicato preset: Ultraviolet Shadow");
                            },
                            "Shadow"
                        }
                        button {
                            r#type: "button",
                            class: "px-2.5 py-1 text-xs font-mono rounded bg-white/[0.04] hover:bg-white/[0.08] text-zinc-300 border border-white/20 cursor-pointer transition-colors",
                            onclick: move |_| {
                                *theme.write() = Theme::obsidian();
                                apply_change();
                                toast.info("Applicato preset: Obsidian Titanium");
                            },
                            "Obsidian"
                        }
                        button {
                            r#type: "button",
                            class: "px-2.5 py-1 text-xs font-mono rounded bg-white/[0.04] hover:bg-white/[0.08] text-rose-300 border border-rose-500/30 cursor-pointer transition-colors",
                            onclick: move |_| {
                                *theme.write() = Theme::crimson();
                                apply_change();
                                toast.info("Applicato preset: Crimson Phoenix");
                            },
                            "Crimson"
                        }
                        button {
                            r#type: "button",
                            class: "px-2.5 py-1 text-xs font-mono rounded bg-white/[0.04] hover:bg-white/[0.08] text-cyan-300 border border-cyan-500/30 cursor-pointer transition-colors",
                            onclick: move |_| {
                                *theme.write() = Theme::cyan();
                                apply_change();
                                toast.info("Applicato preset: Cyber Cyan");
                            },
                            "Cyan"
                        }
                        button {
                            r#type: "button",
                            class: "px-2.5 py-1 text-xs font-mono rounded bg-white/[0.04] hover:bg-white/[0.08] text-emerald-300 border border-emerald-500/30 cursor-pointer transition-colors",
                            onclick: move |_| {
                                *theme.write() = Theme::emerald();
                                apply_change();
                                toast.info("Applicato preset: Emerald Matrix");
                            },
                            "Emerald"
                        }
                        button {
                            r#type: "button",
                            class: "px-2.5 py-1 text-xs font-mono font-bold rounded bg-amber-500/20 hover:bg-amber-500/30 text-amber-300 border border-amber-500/50 cursor-pointer transition-colors shadow-sm",
                            onclick: move |_| {
                                *theme.write() = Theme::cartoon();
                                apply_change();
                                toast.success("Applicato preset: Cartoon Pop Neo-Brutalist!");
                            },
                            "Cartoon Pop"
                        }
                    }
                }
            }

            // Sezione Editor a Schede (Configurazione Tema)
            Card {
                title: "Configurazione Parametri Tema".to_string(),
                subtitle: "Modifica i parametri e osserva i componenti aggiornarsi all'istante in tutto il sito".to_string(),
                shadow: CardShadow::Md,

                // Selettore Schede Editor
                div { class: "flex items-center gap-2 pb-4 mb-6 border-b border-white/[0.08] overflow-x-auto",
                    button {
                        r#type: "button",
                        class: if *active_section.read() == BuilderSection::Palette {
                            "px-3 py-1.5 text-xs font-mono font-bold rounded-lg bg-purple-950/80 text-purple-200 border border-purple-500/40 cursor-pointer shadow-sm flex items-center gap-1.5"
                        } else {
                            "px-3 py-1.5 text-xs font-mono rounded-lg bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors flex items-center gap-1.5"
                        },
                        onclick: move |_| *active_section.write() = BuilderSection::Palette,
                        Icon { kind: IconKind::Palette, size: 14 }
                        "1. Palette & Superfici"
                    }
                    button {
                        r#type: "button",
                        class: if *active_section.read() == BuilderSection::Shape {
                            "px-3 py-1.5 text-xs font-mono font-bold rounded-lg bg-purple-950/80 text-purple-200 border border-purple-500/40 cursor-pointer shadow-sm flex items-center gap-1.5"
                        } else {
                            "px-3 py-1.5 text-xs font-mono rounded-lg bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors flex items-center gap-1.5"
                        },
                        onclick: move |_| *active_section.write() = BuilderSection::Shape,
                        Icon { kind: IconKind::Box, size: 14 }
                        "2. Raggi & Elevazione"
                    }
                    button {
                        r#type: "button",
                        class: if *active_section.read() == BuilderSection::Overrides {
                            "px-3 py-1.5 text-xs font-mono font-bold rounded-lg bg-purple-950/80 text-purple-200 border border-purple-500/40 cursor-pointer shadow-sm flex items-center gap-1.5"
                        } else {
                            "px-3 py-1.5 text-xs font-mono rounded-lg bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors flex items-center gap-1.5"
                        },
                        onclick: move |_| *active_section.write() = BuilderSection::Overrides,
                        Icon { kind: IconKind::Layers, size: 14 }
                        "3. Component Overrides (L2)"
                    }
                    button {
                        r#type: "button",
                        class: if *active_section.read() == BuilderSection::Json {
                            "px-3 py-1.5 text-xs font-mono font-bold rounded-lg bg-purple-950/80 text-purple-200 border border-purple-500/40 cursor-pointer shadow-sm flex items-center gap-1.5"
                        } else {
                            "px-3 py-1.5 text-xs font-mono rounded-lg bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors flex items-center gap-1.5"
                        },
                        onclick: move |_| *active_section.write() = BuilderSection::Json,
                        Icon { kind: IconKind::Code, size: 14 }
                        "4. JSON / Import-Export"
                    }
                }

                // Contenuto Schede
                match *active_section.read() {
                    BuilderSection::Palette => rsx! {
                        div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8",
                            // Colonna Sinistra: Nome e Colore Primario
                            div { class: "space-y-5",
                                div { class: "space-y-1.5",
                                    label { class: "text-xs font-mono uppercase tracking-wider text-zinc-400 block", "Nome Tema:" }
                                    input {
                                        class: "sd-input",
                                        value: "{theme().name}",
                                        oninput: move |e| {
                                            theme.write().name = e.value();
                                            apply_change();
                                        },
                                        placeholder: "Es. Cyber Amber Studio...",
                                    }
                                }

                                div { class: "space-y-3",
                                    label { class: "text-xs font-mono uppercase tracking-wider text-zinc-400 block", "Colore Primario (PrimaryColor):" }
                                    div { class: "flex flex-wrap gap-2",
                                        for p in [PrimaryColor::Violet, PrimaryColor::Obsidian, PrimaryColor::Crimson, PrimaryColor::Cyan, PrimaryColor::Emerald] {
                                            {
                                                let p_choice = p.clone();
                                                let hex = p.hex();
                                                let name = p.name();
                                                let is_active = theme().primary_color == p;
                                                rsx! {
                                                    button {
                                                        r#type: "button",
                                                        class: if is_active {
                                                            "px-3 py-1.5 text-xs font-mono font-bold rounded-lg bg-purple-950/80 text-purple-200 border border-purple-500/50 cursor-pointer shadow-sm flex items-center gap-2"
                                                        } else {
                                                            "px-3 py-1.5 text-xs font-mono rounded-lg bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors flex items-center gap-2"
                                                        },
                                                        onclick: move |_| {
                                                            theme.write().apply_primary_color(p_choice.clone());
                                                            custom_hex_input.set(p_choice.hex().to_string());
                                                            apply_change();
                                                        },
                                                        span { class: "w-3 h-3 rounded-full border border-white/20 shrink-0", style: "background-color: {hex};" }
                                                        "{name}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Personalizzazione Colore Custom HEX
                                div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] space-y-4",
                                    span { class: "text-xs font-mono font-bold text-white uppercase tracking-wider block flex items-center gap-2",
                                        Icon { kind: IconKind::Palette, size: 14 }
                                        "Colore Primario Custom (HEX Libero):"
                                    }

                                    div { class: "grid grid-cols-1 sm:grid-cols-3 gap-3 items-end",
                                        div { class: "space-y-1",
                                            span { class: "text-[11px] font-mono text-zinc-400 block", "Selettore / HEX:" }
                                            div { class: "flex items-center gap-2",
                                                input {
                                                    r#type: "color",
                                                    class: "w-10 h-10 rounded cursor-pointer border border-white/20 bg-transparent p-0",
                                                    value: "{custom_hex_input}",
                                                    oninput: move |e| {
                                                        let val = e.value();
                                                        custom_hex_input.set(val.clone());
                                                        let name = custom_name_input();
                                                        let prefix = custom_prefix_input();
                                                        theme.write().apply_primary_color(PrimaryColor::custom_with_prefix(name, val, prefix));
                                                        apply_change();
                                                    },
                                                }
                                                input {
                                                    class: "sd-input font-mono text-xs",
                                                    value: "{custom_hex_input}",
                                                    oninput: move |e| {
                                                        let val = e.value();
                                                        custom_hex_input.set(val.clone());
                                                        if val.starts_with('#') && (val.len() == 7 || val.len() == 4) {
                                                            let name = custom_name_input();
                                                            let prefix = custom_prefix_input();
                                                            theme.write().apply_primary_color(PrimaryColor::custom_with_prefix(name, val, prefix));
                                                            apply_change();
                                                        }
                                                    },
                                                }
                                            }
                                        }

                                        div { class: "space-y-1",
                                            span { class: "text-[11px] font-mono text-zinc-400 block", "Nome Colore:" }
                                            input {
                                                class: "sd-input text-xs",
                                                value: "{custom_name_input}",
                                                oninput: move |e| {
                                                    let val = e.value();
                                                    custom_name_input.set(val.clone());
                                                    let hex = custom_hex_input();
                                                    let prefix = custom_prefix_input();
                                                    theme.write().apply_primary_color(PrimaryColor::custom_with_prefix(val, hex, prefix));
                                                    apply_change();
                                                },
                                            }
                                        }

                                        div { class: "space-y-1",
                                            span { class: "text-[11px] font-mono text-zinc-400 block", "CSS Prefix:" }
                                            input {
                                                class: "sd-input font-mono text-xs",
                                                value: "{custom_prefix_input}",
                                                oninput: move |e| {
                                                    let val = e.value();
                                                    custom_prefix_input.set(val.clone());
                                                    let name = custom_name_input();
                                                    let hex = custom_hex_input();
                                                    theme.write().apply_primary_color(PrimaryColor::custom_with_prefix(name, hex, val));
                                                    apply_change();
                                                },
                                            }
                                        }
                                    }
                                }
                            }

                            // Colonna Destra: Token di Superficie a 4 Livelli
                            div { class: "space-y-4",
                                label { class: "text-xs font-mono uppercase tracking-wider text-zinc-400 block", "Token Superfici a 4 Livelli (CSS Variables):" }

                                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-3",
                                    // surface_0
                                    div { class: "p-3 rounded-lg bg-[#060608] border border-white/[0.08] space-y-1.5",
                                        div { class: "flex items-center justify-between",
                                            span { class: "text-xs font-mono text-zinc-300", "surface_0 (Void / Sfondo)" }
                                            input {
                                                r#type: "color",
                                                class: "w-6 h-6 rounded cursor-pointer border border-white/20 bg-transparent p-0",
                                                value: "{theme().surface_0}",
                                                oninput: move |e| {
                                                    theme.write().surface_0 = e.value();
                                                    apply_change();
                                                },
                                            }
                                        }
                                        input {
                                            class: "sd-input font-mono text-xs h-8",
                                            value: "{theme().surface_0}",
                                            oninput: move |e| {
                                                theme.write().surface_0 = e.value();
                                                apply_change();
                                            },
                                        }
                                    }

                                    // surface_1
                                    div { class: "p-3 rounded-lg bg-[#060608] border border-white/[0.08] space-y-1.5",
                                        div { class: "flex items-center justify-between",
                                            span { class: "text-xs font-mono text-zinc-300", "surface_1 (Card Base)" }
                                            input {
                                                r#type: "color",
                                                class: "w-6 h-6 rounded cursor-pointer border border-white/20 bg-transparent p-0",
                                                value: "{theme().surface_1}",
                                                oninput: move |e| {
                                                    theme.write().surface_1 = e.value();
                                                    apply_change();
                                                },
                                            }
                                        }
                                        input {
                                            class: "sd-input font-mono text-xs h-8",
                                            value: "{theme().surface_1}",
                                            oninput: move |e| {
                                                theme.write().surface_1 = e.value();
                                                apply_change();
                                            },
                                        }
                                    }

                                    // surface_2
                                    div { class: "p-3 rounded-lg bg-[#060608] border border-white/[0.08] space-y-1.5",
                                        div { class: "flex items-center justify-between",
                                            span { class: "text-xs font-mono text-zinc-300", "surface_2 (Elevata)" }
                                            input {
                                                r#type: "color",
                                                class: "w-6 h-6 rounded cursor-pointer border border-white/20 bg-transparent p-0",
                                                value: "{theme().surface_2}",
                                                oninput: move |e| {
                                                    theme.write().surface_2 = e.value();
                                                    apply_change();
                                                },
                                            }
                                        }
                                        input {
                                            class: "sd-input font-mono text-xs h-8",
                                            value: "{theme().surface_2}",
                                            oninput: move |e| {
                                                theme.write().surface_2 = e.value();
                                                apply_change();
                                            },
                                        }
                                    }

                                    // surface_3
                                    div { class: "p-3 rounded-lg bg-[#060608] border border-white/[0.08] space-y-1.5",
                                        div { class: "flex items-center justify-between",
                                            span { class: "text-xs font-mono text-zinc-300", "surface_3 (Overlay/Menu)" }
                                            input {
                                                r#type: "color",
                                                class: "w-6 h-6 rounded cursor-pointer border border-white/20 bg-transparent p-0",
                                                value: "{theme().surface_3}",
                                                oninput: move |e| {
                                                    theme.write().surface_3 = e.value();
                                                    apply_change();
                                                },
                                            }
                                        }
                                        input {
                                            class: "sd-input font-mono text-xs h-8",
                                            value: "{theme().surface_3}",
                                            oninput: move |e| {
                                                theme.write().surface_3 = e.value();
                                                apply_change();
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    },

                    BuilderSection::Shape => rsx! {
                        div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                            // Raggi di Curvatura
                            div { class: "p-5 rounded-xl bg-[#060608] border border-white/[0.08] space-y-3",
                                span { class: "text-xs font-mono font-bold text-white uppercase tracking-wider block", "Raggio di Curvatura (Radius):" }
                                div { class: "flex flex-col gap-2",
                                    for r in [RadiusScale::Sharp, RadiusScale::Medium, RadiusScale::Soft] {
                                        button {
                                            r#type: "button",
                                            class: if theme().radius_scale == r {
                                                "w-full text-left px-3 py-2 text-xs font-mono font-bold rounded-lg bg-purple-950/80 text-purple-200 border border-purple-500/50 cursor-pointer shadow-sm flex items-center justify-between"
                                            } else {
                                                "w-full text-left px-3 py-2 text-xs font-mono rounded-lg bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors flex items-center justify-between"
                                            },
                                            onclick: move |_| {
                                                theme.write().apply_radius_scale(r);
                                                apply_change();
                                            },
                                            span { "{r.name()}" }
                                            span { class: "text-[10px] opacity-70",
                                                match r {
                                                    RadiusScale::Sharp => "2px",
                                                    RadiusScale::Medium => "8px",
                                                    RadiusScale::Soft => "14px",
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            // Stile di Elevazione
                            div { class: "p-5 rounded-xl bg-[#060608] border border-white/[0.08] space-y-3",
                                span { class: "text-xs font-mono font-bold text-white uppercase tracking-wider block", "Stile di Elevazione (Shadows):" }
                                div { class: "flex flex-col gap-2",
                                    for (e, label, desc) in [
                                        (ElevationStyle::NeonGlow, "Neon Glow", "Specular halo sfumato"),
                                        (ElevationStyle::HardBlock, "Hard Block", "Ombra netta comic/manga"),
                                        (ElevationStyle::SoftElevated, "Soft Elevated", "Ombre morbide neutre"),
                                        (ElevationStyle::Flat, "Flat Minimal", "Zero ombre"),
                                    ] {
                                        button {
                                            r#type: "button",
                                            class: if theme().elevation_style == e {
                                                "w-full text-left px-3 py-2 text-xs font-mono font-bold rounded-lg bg-purple-950/80 text-purple-200 border border-purple-500/50 cursor-pointer shadow-sm flex items-center justify-between"
                                            } else {
                                                "w-full text-left px-3 py-2 text-xs font-mono rounded-lg bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors flex items-center justify-between"
                                            },
                                            onclick: move |_| {
                                                theme.write().apply_elevation_style(e);
                                                apply_change();
                                            },
                                            span { "{label}" }
                                            span { class: "text-[10px] text-zinc-500", "{desc}" }
                                        }
                                    }
                                }
                            }

                            // Glow Mode
                            div { class: "p-5 rounded-xl bg-[#060608] border border-white/[0.08] space-y-3",
                                span { class: "text-xs font-mono font-bold text-white uppercase tracking-wider block", "Intensità Bagliore (Glow Mode):" }
                                div { class: "flex flex-col gap-2",
                                    for g in [GlowMode::Off, GlowMode::Subtle, GlowMode::Expressive] {
                                        button {
                                            r#type: "button",
                                            class: if theme().glow_mode == g {
                                                "w-full text-left px-3 py-2 text-xs font-mono font-bold rounded-lg bg-purple-950/80 text-purple-200 border border-purple-500/50 cursor-pointer shadow-sm flex items-center justify-between"
                                            } else {
                                                "w-full text-left px-3 py-2 text-xs font-mono rounded-lg bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors flex items-center justify-between"
                                            },
                                            onclick: move |_| {
                                                theme.write().apply_glow_mode(g);
                                                apply_change();
                                            },
                                            span {
                                                match g {
                                                    GlowMode::Off => "Off (Disattivato)",
                                                    GlowMode::Subtle => "Subtle (Calibrato)",
                                                    GlowMode::Expressive => "Expressive (Vibrante)",
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },

                    BuilderSection::Overrides => rsx! {
                        div { class: "space-y-6",
                            Text { size: TextSize::Sm, dimmed: true,
                                "Il Livello 2 permette di applicare classi personalizzate per specifici componenti. Gli override hanno priorità automatica grazie all'architettura @layer sd-components."
                            }

                            div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                                // Button Overrides
                                div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] space-y-3",
                                    span { class: "text-xs font-mono font-bold text-white uppercase tracking-wider block", "Button Overrides:" }
                                    div { class: "space-y-2",
                                        span { class: "text-[11px] font-mono text-zinc-400 block", "Classe Base Aggiuntiva:" }
                                        input {
                                            class: "sd-input font-mono text-xs",
                                            placeholder: "Es. sd-btn--comic",
                                            value: theme().components.get("button").and_then(|c| c.base_class.clone()).unwrap_or_default(),
                                            oninput: move |e| {
                                                let val = e.value();
                                                let mut btn_ov = theme.write().components.remove("button").unwrap_or_default();
                                                btn_ov.base_class = if val.trim().is_empty() { None } else { Some(val) };
                                                theme.write().components.insert("button".to_string(), btn_ov);
                                                apply_change();
                                            },
                                        }
                                    }
                                }

                                // Card Overrides
                                div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] space-y-3",
                                    span { class: "text-xs font-mono font-bold text-white uppercase tracking-wider block", "Card Overrides:" }
                                    div { class: "space-y-2",
                                        span { class: "text-[11px] font-mono text-zinc-400 block", "Classe Base Aggiuntiva:" }
                                        input {
                                            class: "sd-input font-mono text-xs",
                                            placeholder: "Es. sd-card--comic",
                                            value: theme().components.get("card").and_then(|c| c.base_class.clone()).unwrap_or_default(),
                                            oninput: move |e| {
                                                let val = e.value();
                                                let mut card_ov = theme.write().components.remove("card").unwrap_or_default();
                                                card_ov.base_class = if val.trim().is_empty() { None } else { Some(val) };
                                                theme.write().components.insert("card".to_string(), card_ov);
                                                apply_change();
                                            },
                                        }
                                    }
                                }

                                // Badge Overrides
                                div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] space-y-3",
                                    span { class: "text-xs font-mono font-bold text-white uppercase tracking-wider block", "Badge Overrides:" }
                                    div { class: "space-y-2",
                                        span { class: "text-[11px] font-mono text-zinc-400 block", "Classe Base Aggiuntiva:" }
                                        input {
                                            class: "sd-input font-mono text-xs",
                                            placeholder: "Es. sd-badge--comic",
                                            value: theme().components.get("badge").and_then(|c| c.base_class.clone()).unwrap_or_default(),
                                            oninput: move |e| {
                                                let val = e.value();
                                                let mut badge_ov = theme.write().components.remove("badge").unwrap_or_default();
                                                badge_ov.base_class = if val.trim().is_empty() { None } else { Some(val) };
                                                theme.write().components.insert("badge".to_string(), badge_ov);
                                                apply_change();
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    },

                    BuilderSection::Json => rsx! {
                        div { class: "space-y-4",
                            div { class: "flex items-center justify-between",
                                span { class: "text-xs font-mono uppercase tracking-wider text-zinc-400", "Rappresentazione JSON Serializzata:" }
                                Button {
                                    size: ButtonSize::Xs,
                                    variant: ButtonVariant::Secondary,
                                    left_icon: rsx! { Icon { kind: IconKind::Copy, size: 12 } },
                                    onclick: move |_| {
                                        let _ = document::eval(&format!("navigator.clipboard.writeText({:?});", json_to_copy));
                                        toast.info("JSON del tema copiato negli appunti!");
                                    },
                                    "Copia JSON"
                                }
                            }

                            pre { class: "p-4 rounded-xl bg-[#030305] border border-white/[0.08] font-mono text-xs text-purple-200 overflow-x-auto max-h-60",
                                code { "{theme_json}" }
                            }

                            // Importazione manuale
                            div { class: "pt-4 border-t border-white/[0.08] space-y-2",
                                span { class: "text-xs font-mono text-zinc-400 block", "Incolla JSON per importare:" }
                                div { class: "flex gap-2",
                                    input {
                                        class: "sd-input font-mono text-xs flex-1",
                                        placeholder: "Incolla qui JSON valido...",
                                        value: "{json_input}",
                                        oninput: move |e| json_input.set(e.value()),
                                    }
                                    Button {
                                        size: ButtonSize::Md,
                                        variant: ButtonVariant::Primary,
                                        onclick: move |_| {
                                            let text = json_input();
                                            match serde_json::from_str::<Theme>(&text) {
                                                Ok(loaded) => {
                                                    *theme.write() = loaded;
                                                    apply_change();
                                                    toast.success("Tema importato con successo!");
                                                }
                                                Err(err) => {
                                                    toast.danger(format!("Errore parsing JSON: {}", err));
                                                }
                                            }
                                        },
                                        "Importa"
                                    }
                                }
                            }
                        }
                    },
                }
            }

            // ==============================================================================
            // LIVE COMPONENT PLAYGROUND & VERIFICA INTEGRATA
            // ==============================================================================
            div { class: "space-y-6 pt-4",
                div { class: "flex items-center justify-between pb-2 border-b border-white/[0.08]",
                    div { class: "flex items-center gap-2",
                        Icon { kind: IconKind::Sparkles, size: 18 }
                        Heading { order: HeadingOrder::H3, "Live Component Playground" }
                    }
                    span { class: "text-xs font-mono text-zinc-500", "Collaudo simultaneo di tutte le componenti" }
                }

                // 1. Buttons Arena
                Card {
                    title: "Pulsanti (Button Suite)".to_string(),
                    subtitle: "Tutte le varianti cromatiche, taglie dimensionali e stati tattili".to_string(),
                    shadow: CardShadow::Md,
                    div { class: "space-y-4",
                        div { class: "flex flex-wrap items-center gap-3",
                            Button { variant: ButtonVariant::Primary, "Primary" }
                            Button { variant: ButtonVariant::Secondary, "Secondary" }
                            Button { variant: ButtonVariant::Outline, "Outline" }
                            Button { variant: ButtonVariant::Ghost, "Ghost" }
                            Button { variant: ButtonVariant::Danger, "Danger" }
                            Button { variant: ButtonVariant::Success, "Success" }
                        }

                        div { class: "flex flex-wrap items-center gap-3 pt-2 border-t border-white/[0.06]",
                            Button {
                                size: ButtonSize::Sm,
                                variant: ButtonVariant::Primary,
                                left_icon: rsx! { Icon { kind: IconKind::Sparkles, size: 13 } },
                                "Taglia Sm"
                            }
                            Button {
                                size: ButtonSize::Md,
                                variant: ButtonVariant::Secondary,
                                right_icon: rsx! { Icon { kind: IconKind::ExternalLink, size: 14 } },
                                "Taglia Md"
                            }
                            Button {
                                size: ButtonSize::Lg,
                                variant: ButtonVariant::Outline,
                                left_icon: rsx! { Icon { kind: IconKind::Check, size: 16 } },
                                "Taglia Lg"
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                loading: true,
                                "Loading..."
                            }
                        }
                    }
                }

                // 2. Schede & Metriche
                div { class: "grid grid-cols-1 md:grid-cols-3 gap-5",
                    Card {
                        title: "Card Standard".to_string(),
                        subtitle: "Contenitore base ad alto contrasto".to_string(),
                        footer: rsx! {
                            div { class: "flex items-center justify-between text-xs text-zinc-400",
                                span { "Footer Card" }
                                Button { size: ButtonSize::Xs, variant: ButtonVariant::Ghost, "Azione" }
                            }
                        },
                        p { class: "text-xs text-zinc-300 leading-relaxed",
                            "Il tema custom applica raggi di curvatura ed elevazione in modo totalmente reattivo."
                        }
                    }

                    Card {
                        variant: CardVariant::Interactive,
                        hover_glow: true,
                        div { class: "space-y-2",
                            span { class: "text-[11px] font-mono uppercase tracking-wider text-purple-300 block", "Card Interactive" }
                            p { class: "text-sm text-white font-semibold", "Passa il mouse per osservare il micro-glow" }
                            span { class: "text-xs text-zinc-400 block", "L'elevazione rispetta la modalità attiva." }
                        }
                    }

                    Card {
                        variant: CardVariant::Metric,
                        div { class: "flex items-start justify-between gap-3",
                            div { class: "space-y-1.5",
                                span { class: "text-[11px] font-mono uppercase tracking-wider text-zinc-400 block", "Metrica Attiva" }
                                div { class: "text-2xl font-extrabold text-white tracking-tight", "99.98%" }
                                span { class: "text-[11px] text-emerald-400 font-mono flex items-center gap-1",
                                    Icon { kind: IconKind::Check, size: 12 }
                                    "Runtime Ottimizzato"
                                }
                            }
                            span { class: "p-2.5 rounded-xl bg-purple-500/10 border border-purple-500/20 text-purple-300 flex items-center justify-center shrink-0",
                                Icon { kind: IconKind::Sparkles, size: 18 }
                            }
                        }
                    }
                }

                // 3. TextInput & Dropdown & Toggle Tattili
                div { class: "grid grid-cols-1 lg:grid-cols-2 gap-6 items-start",
                    Card {
                        title: "Campi di Input & Dropdown".to_string(),
                        subtitle: "Interattività utente e menu ad alta precisione".to_string(),
                        div { class: "space-y-5",
                            TextInput {
                                value: test_input_val,
                                label: "Input con Icona e Clear Button".to_string(),
                                placeholder: "Scrivi qualcosa...".to_string(),
                                left_icon: Some(IconKind::Terminal.into()),
                                clearable: true,
                                help_text: "Modifica il testo per verificare il binding reattivo.".to_string(),
                            }

                            div { class: "pt-2 flex items-center justify-between",
                                span { class: "text-xs text-zinc-400 font-mono", "Menu a Tendina Ancorato:" }
                                Dropdown {
                                    is_open: test_dropdown_open,
                                    placement: DropdownPlacement::BottomRight,
                                    trigger: rsx! {
                                        Button {
                                            variant: ButtonVariant::Primary,
                                            size: ButtonSize::Sm,
                                            right_icon: rsx! { Icon { kind: IconKind::ChevronDown, size: 14 } },
                                            "Apri Dropdown"
                                        }
                                    },
                                    DropdownItem {
                                        icon: Some(IconKind::Sparkles),
                                        onclick: move |_| {
                                            toast.info("Azione eseguita dal Dropdown!");
                                            test_dropdown_open.set(false);
                                        },
                                        "Azione Rapida 1"
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
                                        "Elimina Risorsa"
                                    }
                                }
                            }
                        }
                    }

                    Card {
                        title: "Badge, Checkbox & Switch".to_string(),
                        subtitle: "Stati di selezione e pillole informative".to_string(),
                        div { class: "space-y-5",
                            // Badge
                            div { class: "flex flex-wrap items-center gap-2",
                                Badge { label: "Default".to_string(), variant: BadgeVariant::Default }
                                Badge { label: "Primary".to_string(), variant: BadgeVariant::Primary }
                                Badge { label: "Success".to_string(), variant: BadgeVariant::Success, dot: true }
                                Badge { label: "Warning".to_string(), variant: BadgeVariant::Warning }
                                Badge { label: "Danger".to_string(), variant: BadgeVariant::Danger }
                                Badge { label: "Pill Style".to_string(), variant: BadgeVariant::Primary, pill: true }
                            }

                            // Checkbox & Switch
                            div { class: "pt-3 border-t border-white/[0.06] space-y-3",
                                Checkbox {
                                    checked: test_checkbox,
                                    label: "Checkbox interattiva di verifica".to_string(),
                                    description: "Controlla il feedback visivo al click.".to_string(),
                                }

                                Switch {
                                    checked: test_switch,
                                    label: "Interruttore Switch a scatto".to_string(),
                                    description: "Sincronizzato con l'accento cromatico del tema.".to_string(),
                                }
                            }
                        }
                    }
                }

                // 4. Alert di Sistema
                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                    Alert {
                        title: "Informazione di Sistema".to_string(),
                        variant: AlertVariant::Info,
                        "Il tema appena creato è attivo in tutta l'applicazione."
                    }
                    Alert {
                        title: "Operazione Riuscita".to_string(),
                        variant: AlertVariant::Success,
                        "Bordi e contrasti ottici calibrati secondo gli standard Mantine."
                    }
                }
            }

            // ==============================================================================
            // SNIPPET RUST GENERATO (PRONTO DA COPIARE)
            // ==============================================================================
            Card {
                title: "Codice Rust Generato per il Tuo Progetto".to_string(),
                subtitle: "Incolla questo codice nella tua applicazione Dioxus (es. nakasa o app desktop/web)".to_string(),
                shadow: CardShadow::Lg,
                CodeBlock {
                    code: generated_rust_code,
                    language: "rust",
                }
            }
        }
    }
}
