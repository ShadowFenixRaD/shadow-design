use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::{Route, HEADER_SVG};

#[component]
pub fn Layout() -> Element {
    let mut theme = use_theme();

    let primary = theme().primary_color;
    let radius = theme().radius_scale;
    let glow = theme().glow_mode;

    rsx! {
        div { class: "{theme().root}",
            // Header Superiore con Mantine-Style Theme Toolbar
            header { class: "{theme().header} sticky top-0 z-30 px-4 sm:px-6 py-3 flex flex-wrap items-center justify-between gap-4",
                // Brand
                div { class: "flex items-center gap-4",
                    Link {
                        to: Route::Home {},
                        class: "flex items-center gap-3 text-lg font-bold tracking-tight text-white hover:opacity-90 transition-opacity group",
                        div { class: "relative flex items-center justify-center",
                            div { class: "absolute -inset-1 rounded-full bg-purple-600/30 blur-sm group-hover:bg-purple-600/50 transition-all" }
                            img { src: HEADER_SVG, class: "relative h-8 w-auto drop-shadow-[0_0_8px_rgba(168,85,247,0.5)]", alt: "Shadowfenix" }
                        }
                        div { class: "flex flex-col leading-none",
                            span { class: "text-base font-extrabold tracking-tight text-white group-hover:text-purple-200 transition-colors", "Shadow Design" }
                            span { class: "text-[10px] font-mono tracking-widest text-purple-400 uppercase", "by shadowfenix" }
                        }
                    }
                    Badge {
                        label: "v0.1.0 • Dioxus 0.7".to_string(),
                        variant: BadgeVariant::Primary,
                        pill: true,
                    }
                }

                // Mantine-Style Theme Controls Toolbar
                div { class: "flex items-center gap-3 bg-[#060608] px-3 py-1.5 rounded-xl border border-white/[0.08] shadow-inner",
                    // Swatches per Colore Primario
                    div { class: "flex items-center gap-1.5 pr-2.5 border-r border-white/[0.08]",
                        span { class: "text-[10px] font-mono text-zinc-400 uppercase mr-1 hidden md:inline", "Color:" }
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
                    div { class: "flex items-center gap-1 pr-2.5 border-r border-white/[0.08]",
                        span { class: "text-[10px] font-mono text-zinc-400 uppercase mr-0.5 hidden lg:inline", "Radius:" }
                        button {
                            r#type: "button",
                            class: if radius == RadiusScale::Sharp {
                                "px-2 py-0.5 text-[10px] font-mono font-bold rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 cursor-pointer shadow-sm"
                            } else {
                                "px-2 py-0.5 text-[10px] font-mono rounded bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors"
                            },
                            onclick: move |_| theme.write().apply_radius_scale(RadiusScale::Sharp),
                            "Sharp"
                        }
                        button {
                            r#type: "button",
                            class: if radius == RadiusScale::Medium {
                                "px-2 py-0.5 text-[10px] font-mono font-bold rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 cursor-pointer shadow-sm"
                            } else {
                                "px-2 py-0.5 text-[10px] font-mono rounded bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors"
                            },
                            onclick: move |_| theme.write().apply_radius_scale(RadiusScale::Medium),
                            "Medium"
                        }
                        button {
                            r#type: "button",
                            class: if radius == RadiusScale::Soft {
                                "px-2 py-0.5 text-[10px] font-mono font-bold rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 cursor-pointer shadow-sm"
                            } else {
                                "px-2 py-0.5 text-[10px] font-mono rounded bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors"
                            },
                            onclick: move |_| theme.write().apply_radius_scale(RadiusScale::Soft),
                            "Soft"
                        }
                    }

                    // Toggle Glow
                    div { class: "flex items-center gap-1",
                        span { class: "text-[10px] font-mono text-zinc-400 uppercase mr-0.5 hidden xl:inline", "Glow:" }
                        button {
                            r#type: "button",
                            class: match glow {
                                GlowMode::Off => "px-2 py-0.5 text-[10px] font-mono font-medium rounded bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer transition-colors",
                                GlowMode::Subtle => "px-2 py-0.5 text-[10px] font-mono font-bold rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 cursor-pointer shadow-sm",
                                GlowMode::Expressive => "px-2 py-0.5 text-[10px] font-mono font-bold rounded bg-purple-600 text-white border border-purple-400/50 shadow-[0_0_12px_rgba(147,51,234,0.5)] cursor-pointer",
                            },
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
                }
            }

            // Layout Principale con Sidebar e Contenuto
            div { class: "flex-1 flex max-w-7xl w-full mx-auto",
                // Sidebar
                aside { class: "{theme().sidebar} w-64 p-5 flex flex-col gap-6 shrink-0",
                    div {
                        p { class: "text-[11px] font-semibold text-zinc-400 uppercase tracking-wider mb-2 font-mono flex items-center gap-1.5",
                            Icon { kind: IconKind::Terminal, size: 12 }
                            "Generale"
                        }
                        div { class: "flex flex-col gap-1",
                            SidebarLink { to: Route::Home {}, icon: IconKind::Layers, label: "Panoramica" }
                            SidebarLink { to: Route::TypographyPage {}, icon: IconKind::Code, label: "Typography" }
                        }
                    }

                    div {
                        p { class: "text-[11px] font-semibold text-zinc-400 uppercase tracking-wider mb-2 font-mono flex items-center gap-1.5",
                            Icon { kind: IconKind::Box, size: 12 }
                            "Componenti Core"
                        }
                        div { class: "flex flex-col gap-1",
                            SidebarLink { to: Route::ButtonsPage {}, icon: IconKind::Sparkles, label: "Button" }
                            SidebarLink { to: Route::CardsPage {}, icon: IconKind::Box, label: "Card" }
                            SidebarLink { to: Route::InputsPage {}, icon: IconKind::Terminal, label: "TextInput" }
                            SidebarLink { to: Route::CheckboxPage {}, icon: IconKind::Check, label: "Checkbox" }
                            SidebarLink { to: Route::BadgesPage {}, icon: IconKind::CheckCircle, label: "Badge" }
                            SidebarLink { to: Route::AlertsPage {}, icon: IconKind::AlertTriangle, label: "Alert" }
                            SidebarLink { to: Route::ModalsPage {}, icon: IconKind::Eye, label: "Modal" }
                            SidebarLink { to: Route::NavbarDemoPage {}, icon: IconKind::Menu, label: "Navbar" }
                        }
                    }

                    div {
                        p { class: "text-[11px] font-semibold text-zinc-400 uppercase tracking-wider mb-2 font-mono flex items-center gap-1.5",
                            Icon { kind: IconKind::Sparkles, size: 12 }
                            "Interattività & Avanzati"
                        }
                        div { class: "flex flex-col gap-1",
                            SidebarLink { to: Route::NotificationsPage {}, icon: IconKind::Info, label: "Notifications" }
                            SidebarLink { to: Route::TabsPage {}, icon: IconKind::Layers, label: "Tabs" }
                            SidebarLink { to: Route::SwitchPage {}, icon: IconKind::Settings, label: "Switch" }
                            SidebarLink { to: Route::TooltipPage {}, icon: IconKind::Sparkles, label: "Tooltip" }
                            SidebarLink { to: Route::DrawerPage {}, icon: IconKind::ChevronRight, label: "Drawer" }
                            SidebarLink { to: Route::DropdownPage {}, icon: IconKind::ChevronDown, label: "Dropdown" }
                        }
                    }

                    // Telemetria Tema Attivo
                    div { class: "{theme().surface} rounded-xl p-3.5 text-xs text-zinc-400 mt-auto",
                        div { class: "flex items-center justify-between mb-2 pb-2 border-b border-white/[0.06]",
                            div { class: "flex items-center gap-1.5 font-semibold text-purple-300",
                                Icon { kind: IconKind::Palette, size: 14 }
                                span { "Mantine Engine" }
                            }
                            span { class: "text-[10px] font-mono px-1.5 py-0.2 rounded bg-purple-950/60 text-purple-200 border border-purple-500/30",
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

                // Page Content
                main { class: "{theme().content} flex-1 p-6 sm:p-8 overflow-y-auto",
                    Outlet::<Route> {}
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
        "ring-2 ring-white ring-offset-2 ring-offset-black scale-110"
    } else {
        "hover:scale-105 opacity-80 hover:opacity-100"
    };

    rsx! {
        button {
            r#type: "button",
            class: "w-5 h-5 rounded-full cursor-pointer transition-all duration-150 {ring_class}",
            style: "background-color: {hex};",
            title,
            onclick: move |e| onclick.call(e),
        }
    }
}

#[component]
pub fn SidebarLink(to: Route, icon: IconKind, label: &'static str) -> Element {
    let theme = use_theme();
    rsx! {
        Link {
            to,
            class: "{theme().sidebar_link}",
            active_class: "{theme().sidebar_link_active}",
            Icon { kind: icon, size: 15 }
            span { "{label}" }
        }
    }
}
