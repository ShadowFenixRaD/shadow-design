use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::nav::{SearchPalette, CATEGORIES, COMPONENTS, ComponentItem};
use crate::{Route, HEADER_SVG, VERSION};

#[component]
pub fn Layout() -> Element {
    let mut theme = use_theme();
    let mut is_sidebar_collapsed = use_signal(|| false);
    let mut is_search_open = use_signal(|| false);

    let primary = theme().primary_color;
    let radius = theme().radius_scale;
    let glow = theme().glow_mode;

    // Listener globale per scorciatoia tastiera Ctrl+K / Cmd+K (SC-24)
    use_hook(|| {
        spawn(async move {
            let eval = document::eval(
                r#"
                window.addEventListener("keydown", (e) => {
                    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
                        e.preventDefault();
                        const demoBtn = document.getElementById("sd-command-palette-demo-trigger");
                        if (demoBtn) {
                            demoBtn.click();
                            return;
                        }
                        const btn = document.getElementById("sd-search-palette-trigger");
                        if (btn) btn.click();
                    }
                });
                "#,
            );
            let _ = eval.await;
        });
    });

    let seg_active_class = match primary {
        PrimaryColor::Crimson => "bg-rose-950/80 text-rose-200 border border-rose-500/40 shadow-sm",
        PrimaryColor::Cyan => "bg-cyan-950/80 text-cyan-200 border border-cyan-500/40 shadow-sm",
        PrimaryColor::Emerald => {
            "bg-emerald-950/80 text-emerald-200 border border-emerald-500/40 shadow-sm"
        }
        PrimaryColor::Obsidian => {
            "bg-zinc-800/80 text-zinc-100 border border-zinc-600/50 shadow-sm"
        }
        _ => "bg-purple-950/80 text-purple-200 border border-purple-500/40 shadow-sm",
    };

    let seg_inactive_class = "bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] transition-colors";

    rsx! {
        div { class: "{theme().root}",
            // Hidden button for global Ctrl+K trigger
            button {
                id: "sd-search-palette-trigger",
                class: "hidden",
                onclick: move |_| is_search_open.set(true),
            }

            SearchPalette { is_open: is_search_open }

            // Header Superiore con Mantine-Style Theme Toolbar
            header { class: "{theme().header} sticky top-0 z-30 px-4 sm:px-6 py-3 flex flex-wrap items-center justify-between gap-4",
                // Brand e Sidebar Toggle
                div { class: "flex items-center gap-3",
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        class: "p-1.5 text-zinc-300 hover:text-white",
                        onclick: move |_| is_sidebar_collapsed.toggle(),
                        Icon { kind: IconKind::Menu, size: 18 }
                    }

                    Link {
                        to: Route::Home {},
                        class: "flex items-center gap-3 text-lg font-bold tracking-tight text-white hover:opacity-90 transition-opacity group",
                        div { class: "relative flex items-center justify-center",
                            div {
                                class: "absolute -inset-1 rounded-full blur-sm transition-all",
                                style: "background-color: {primary.hex()}; opacity: 0.25;",
                            }
                            img {
                                src: HEADER_SVG,
                                class: "relative h-8 w-auto",
                                style: "filter: drop-shadow(0 0 8px {primary.hex()});",
                                alt: "Shadowfenix",
                            }
                        }
                        div { class: "flex flex-col leading-none",
                            span { class: "text-base font-extrabold tracking-tight text-white group-hover:text-purple-200 transition-colors", "Shadow Design" }
                            span { class: "text-[10px] font-mono tracking-widest text-zinc-400 uppercase", "by shadowfenix" }
                        }
                    }
                    Badge {
                        label: format!("v{VERSION} • Dioxus 0.7"),
                        variant: BadgeVariant::Primary,
                        pill: true,
                    }
                }

                // Toolbar e Search (VQA-01, VQA-08, SC-24)
                div { class: "sd-toolbar flex items-center gap-2 flex-wrap max-w-full",
                    // Search Palette button
                    button {
                        r#type: "button",
                        class: "flex items-center gap-2 px-2.5 py-1 text-xs font-mono rounded-lg border border-white/[0.08] bg-white/[0.03] hover:bg-white/[0.08] text-zinc-300 hover:text-white transition-all cursor-pointer shadow-sm",
                        onclick: move |_| is_search_open.set(true),
                        Icon { kind: IconKind::Search, size: 13 }
                        span { class: "hidden sm:inline", "Cerca..." }
                        kbd { class: "hidden md:inline text-[9px] px-1 py-0.5 rounded bg-white/[0.08] text-zinc-400 font-mono", "Ctrl K" }
                    }

                    // Swatches per Colore Primario
                    div { class: "flex items-center gap-1.5 pr-2 border-r border-white/[0.08]",
                        span { class: "text-[10px] font-mono text-zinc-400 uppercase mr-0.5 hidden md:inline", "Color:" }
                        ColorSwatchButton {
                            color: PrimaryColor::Violet,
                            is_active: primary == PrimaryColor::Violet,
                            title: "Violet Ultraviolet",
                            onclick: move |_| theme.write().apply_primary_color(PrimaryColor::Violet),
                        }
                        ColorSwatchButton {
                            color: PrimaryColor::Obsidian,
                            is_active: primary == PrimaryColor::Obsidian,
                            title: "Obsidian Titanium",
                            onclick: move |_| theme.write().apply_primary_color(PrimaryColor::Obsidian),
                        }
                        ColorSwatchButton {
                            color: PrimaryColor::Crimson,
                            is_active: primary == PrimaryColor::Crimson,
                            title: "Crimson Phoenix",
                            onclick: move |_| theme.write().apply_primary_color(PrimaryColor::Crimson),
                        }
                        ColorSwatchButton {
                            color: PrimaryColor::Cyan,
                            is_active: primary == PrimaryColor::Cyan,
                            title: "Cyber Cyan",
                            onclick: move |_| theme.write().apply_primary_color(PrimaryColor::Cyan),
                        }
                        ColorSwatchButton {
                            color: PrimaryColor::Emerald,
                            is_active: primary == PrimaryColor::Emerald,
                            title: "Emerald Matrix",
                            onclick: move |_| theme.write().apply_primary_color(PrimaryColor::Emerald),
                        }
                    }

                    // Selettore Raggio Globale (Border Radius)
                    div { class: "flex items-center gap-1 pr-2 border-r border-white/[0.08]",
                        span { class: "text-[10px] font-mono text-zinc-400 uppercase mr-0.5 hidden lg:inline", "Radius:" }
                        button {
                            r#type: "button",
                            class: format!("px-2 py-0.5 text-[10px] font-mono rounded cursor-pointer transition-colors {}", if radius == RadiusScale::Sharp { seg_active_class } else { seg_inactive_class }),
                            onclick: move |_| theme.write().apply_radius_scale(RadiusScale::Sharp),
                            "Sharp"
                        }
                        button {
                            r#type: "button",
                            class: format!("px-2 py-0.5 text-[10px] font-mono rounded cursor-pointer transition-colors {}", if radius == RadiusScale::Medium { seg_active_class } else { seg_inactive_class }),
                            onclick: move |_| theme.write().apply_radius_scale(RadiusScale::Medium),
                            "Medium"
                        }
                        button {
                            r#type: "button",
                            class: format!("px-2 py-0.5 text-[10px] font-mono rounded cursor-pointer transition-colors {}", if radius == RadiusScale::Soft { seg_active_class } else { seg_inactive_class }),
                            onclick: move |_| theme.write().apply_radius_scale(RadiusScale::Soft),
                            "Soft"
                        }
                    }

                    // Toggle Glow
                    div { class: "flex items-center gap-1 pr-2 border-r border-white/[0.08]",
                        span { class: "text-[10px] font-mono text-zinc-400 uppercase mr-0.5 hidden xl:inline", "Glow:" }
                        button {
                            r#type: "button",
                            class: format!("px-2 py-0.5 text-[10px] font-mono rounded cursor-pointer transition-colors {}", match glow {
                                GlowMode::Off => seg_inactive_class,
                                GlowMode::Subtle | GlowMode::Expressive => seg_active_class,
                            }),
                            onclick: move |_| {
                                let next_glow = match glow {
                                    GlowMode::Off => GlowMode::Subtle,
                                    GlowMode::Subtle => GlowMode::Expressive,
                                    GlowMode::Expressive => GlowMode::Off,
                                };
                                theme.write().apply_glow_mode(next_glow);
                            },
                            match glow {
                                GlowMode::Off => "Off",
                                GlowMode::Subtle => "Subtle",
                                GlowMode::Expressive => "Expressive",
                            }
                        }
                    }

                    // Link Rapido Theme Builder
                    div { class: "pl-1",
                        Link {
                            to: Route::ThemeBuilderPage {},
                            class: format!("flex items-center gap-1.5 px-2.5 py-1 text-[11px] font-mono font-semibold rounded transition-all shadow-sm {}", seg_active_class),
                            Icon { kind: IconKind::Palette, size: 12 }
                            span { class: "hidden sm:inline", "Custom Builder" }
                        }
                    }
                }
            }

            // Layout Principale con Sidebar e Contenuto (Inline a sinistra, non overlay)
            div { class: "flex-1 flex max-w-7xl w-full mx-auto min-w-0",
                // Sidebar Desktop/Mobile a sinistra (non overlay, comprimibile)
                aside {
                    class: format!(
                        "{} flex flex-col gap-6 shrink-0 sticky top-[64px] h-[calc(100vh-64px)] overflow-y-auto transition-all duration-200 {}",
                        theme().sidebar,
                        if is_sidebar_collapsed() { "w-16 p-2 items-center" } else { "w-64 p-5" }
                    ),
                    SidebarView {
                        collapsed: is_sidebar_collapsed(),
                        on_toggle: move |_| is_sidebar_collapsed.toggle(),
                    }
                }

                // Page Content
                main { class: "{theme().content} flex-1 min-w-0 p-4 sm:p-8 overflow-y-auto",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[component]
fn SidebarView(collapsed: bool, on_toggle: EventHandler<()>) -> Element {
    let theme = use_theme();
    let primary = theme().primary_color;
    let radius = theme().radius_scale;
    let glow = theme().glow_mode;

    rsx! {
        div { class: "flex flex-col gap-5 w-full h-full",
            // Barra superiore sidebar con bottone comprimi/espandi
            div {
                class: format!("flex items-center pb-2 border-b border-white/[0.06] {}", if collapsed { "justify-center" } else { "justify-between" }),
                if !collapsed {
                    span { class: "text-[11px] font-mono font-semibold text-zinc-400 uppercase tracking-wider", "Navigazione" }
                }
                button {
                    r#type: "button",
                    class: "p-1.5 rounded-lg text-zinc-400 hover:text-white hover:bg-white/[0.08] transition-colors cursor-pointer",
                    onclick: move |_| on_toggle.call(()),
                    title: if collapsed { "Espandi barra laterale" } else { "Comprimi barra laterale" },
                    Icon {
                        kind: if collapsed { IconKind::ChevronRight } else { IconKind::ChevronLeft },
                        size: 15,
                    }
                }
            }

                        // Sezioni generate dinamicamente dal registry componenti unificato
            {
                CATEGORIES.iter().filter_map(|cat| {
                    let items: Vec<&'static ComponentItem> = COMPONENTS.iter().filter(|c| c.class == cat.name).collect();
                    if items.is_empty() {
                        None
                    } else {
                        Some(rsx! {
                            div { class: "flex flex-col",
                                if collapsed {
                                    div { class: "w-full my-1 border-t border-white/[0.06]" }
                                } else {
                                    p {
                                        class: format!(
                                            "text-[11px] font-semibold {} uppercase tracking-wider mb-2 font-mono flex items-center gap-1.5",
                                            if cat.is_accent { "text-purple-400" } else { "text-zinc-400" }
                                        ),
                                        Icon { kind: cat.icon, size: 12 }
                                        "{cat.name}"
                                    }
                                }
                                div { class: "flex flex-col gap-1",
                                    for comp in items {
                                        SidebarLink {
                                            to: comp.route.clone(),
                                            icon: comp.icon,
                                            label: comp.name,
                                            collapsed,
                                        }
                                    }
                                }
                            }
                        })
                    }
                })
            }

            // Telemetria Tema Attivo
            if collapsed {
                div { class: "mt-auto p-2 rounded-xl bg-white/[0.03] border border-white/[0.06] flex items-center justify-center",
                    Link {
                        to: Route::ThemeBuilderPage {},
                        class: "text-zinc-400 hover:text-white transition-colors cursor-pointer p-1",
                        title: "Tema attivo: {theme().name} ({primary.name()})",
                        Icon { kind: IconKind::Palette, size: 16 }
                    }
                }
            } else {
                div { class: "{theme().surface} rounded-xl p-3.5 text-xs text-zinc-400 mt-auto",
                    div { class: "flex items-center justify-between mb-2 pb-2 border-b border-white/[0.06] gap-2",
                        div { class: "flex items-center gap-1.5 font-semibold text-zinc-200 shrink-0",
                            Icon { kind: IconKind::Palette, size: 14 }
                            span { "Mantine Engine" }
                        }
                        Link {
                            to: Route::ThemeBuilderPage {},
                            class: "text-[10px] font-mono px-1.5 py-0.5 rounded text-zinc-200 border border-white/[0.1] transition-colors cursor-pointer truncate max-w-[100px]",
                            title: "Apri Theme Builder",
                            "{theme().name}"
                        }
                    }
                    div { class: "space-y-1 font-mono text-[10px] text-zinc-500",
                        div { class: "flex justify-between",
                            span { "Primary:" }
                            span { class: "text-zinc-300 font-semibold", "{primary.name()}" }
                        }
                        div { class: "flex justify-between",
                            span { "Radius:" }
                            span { class: "text-zinc-300 font-semibold", "{radius.name()}" }
                        }
                        div { class: "flex justify-between",
                            span { "Glow:" }
                            span { class: "text-zinc-300 font-semibold", "{glow:?}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ColorSwatchButton(
    color: PrimaryColor,
    is_active: bool,
    title: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let hex = color.hex();
    let ring_class = if is_active {
        "sd-swatch-active"
    } else {
        "hover:scale-105 opacity-80 hover:opacity-100"
    };

    rsx! {
        button {
            r#type: "button",
            class: "w-5 h-5 rounded-full border border-white/20 cursor-pointer transition-all duration-150 {ring_class}",
            style: "background-color: {hex};",
            title,
            onclick: move |e| onclick.call(e),
        }
    }
}

#[component]
pub fn SidebarLink(
    to: Route,
    icon: IconKind,
    label: &'static str,
    #[props(default = false)] collapsed: bool,
) -> Element {
    let theme = use_theme();
    let collapsed_class = if collapsed {
        "justify-center px-0 py-2 w-full"
    } else {
        "px-3 py-2"
    };
    rsx! {
        Link {
            to,
            class: "{theme().sidebar_link} {collapsed_class}",
            active_class: "{theme().sidebar_link_active}",
            title: "{label}",
            Icon { kind: icon, size: 16 }
            if !collapsed {
                span { class: "truncate text-xs", "{label}" }
            }
        }
    }
}
