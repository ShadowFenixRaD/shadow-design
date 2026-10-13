use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::nav::SearchPalette;
use crate::{Route, HEADER_SVG};

#[component]
pub fn Layout() -> Element {
    let mut theme = use_theme();
    let mut is_drawer_open = use_signal(|| false);
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
                    if ((e.ctrlKey || e.metaKey) && e.key === "k") {
                        e.preventDefault();
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
        PrimaryColor::Emerald => "bg-emerald-950/80 text-emerald-200 border border-emerald-500/40 shadow-sm",
        PrimaryColor::Obsidian => "bg-zinc-800/80 text-zinc-100 border border-zinc-600/50 shadow-sm",
        _ => "bg-purple-950/80 text-purple-200 border border-purple-500/40 shadow-sm",
    };

    let seg_inactive_class = "bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] transition-colors";

    let sidebar_content = rsx! {
        div { class: "flex flex-col gap-6",
            div {
                p { class: "text-[11px] font-semibold text-zinc-400 uppercase tracking-wider mb-2 font-mono flex items-center gap-1.5",
                    Icon { kind: IconKind::Terminal, size: 12 }
                    "Generale"
                }
                div { class: "flex flex-col gap-1",
                    SidebarLink { to: Route::Home {}, icon: IconKind::Layers, label: "Panoramica" }
                    SidebarLink { to: Route::ThemeBuilderPage {}, icon: IconKind::Palette, label: "Theme Builder" }
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
                    SidebarLink { to: Route::SelectPage {}, icon: IconKind::ChevronDown, label: "Select" }
                    SidebarLink { to: Route::TextareaPage {}, icon: IconKind::Code, label: "Textarea" }
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
                    SidebarLink { to: Route::IconsPage {}, icon: IconKind::Search, label: "Icons" }
                    SidebarLink { to: Route::AnimationsPage {}, icon: IconKind::Sparkles, label: "Animation" }
                }
            }

            div {
                p { class: "text-[11px] font-semibold text-purple-400 uppercase tracking-wider mb-2 font-mono flex items-center gap-1.5",
                    Icon { kind: IconKind::MessageSquare, size: 12 }
                    "Chat & Mesh P2P"
                }
                div { class: "flex flex-col gap-1",
                    SidebarLink { to: Route::AvatarsPage {}, icon: IconKind::Sparkles, label: "Avatar" }
                    SidebarLink { to: Route::ChatPage {}, icon: IconKind::MessageSquare, label: "Chat Bubble" }
                    SidebarLink { to: Route::AttachmentCardsPage {}, icon: IconKind::Paperclip, label: "AttachmentCard" }
                    SidebarLink { to: Route::ProgressPage {}, icon: IconKind::Spinner, label: "Progress" }
                    SidebarLink { to: Route::SegmentedControlsPage {}, icon: IconKind::Layers, label: "SegmentedControl" }
                    SidebarLink { to: Route::DividersPage {}, icon: IconKind::Code, label: "Divider" }
                    SidebarLink { to: Route::TreesPage {}, icon: IconKind::Radio, label: "TreeView (Mesh)" }
                    SidebarLink { to: Route::SkeletonsPage {}, icon: IconKind::Box, label: "Skeleton" }
                }
            }

            // Telemetria Tema Attivo
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
    };

    rsx! {
        div { class: "{theme().root}",
            // Hidden button for global Ctrl+K trigger
            button {
                id: "sd-search-palette-trigger",
                class: "hidden",
                onclick: move |_| is_search_open.set(true),
            }

            SearchPalette { is_open: is_search_open }

            // Mobile Drawer Sidebar (VQA-01)
            Drawer {
                is_open: is_drawer_open,
                position: DrawerPosition::Left,
                size: DrawerSize::Sm,
                title: Some("Navigazione".to_string()),
                div { class: "p-4 overflow-y-auto max-h-[calc(100vh-80px)]",
                    {sidebar_content.clone()}
                }
            }

            // Header Superiore con Mantine-Style Theme Toolbar
            header { class: "{theme().header} sticky top-0 z-30 px-4 sm:px-6 py-3 flex flex-wrap items-center justify-between gap-4",
                // Brand e Mobile Toggle
                div { class: "flex items-center gap-3",
                    // Mobile Hamburger (VQA-01)
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Sm,
                        class: "lg:hidden p-1.5 text-zinc-300 hover:text-white",
                        onclick: move |_| is_drawer_open.set(true),
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
                        label: "v0.2.0 • Dioxus 0.7".to_string(),
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

            // Layout Principale con Sidebar e Contenuto (VQA-01)
            div { class: "flex-1 flex max-w-7xl w-full mx-auto min-w-0",
                // Sidebar Desktop (>=1024px)
                aside { class: "{theme().sidebar} hidden lg:flex w-64 p-5 flex-col gap-6 shrink-0 sticky top-[64px] h-[calc(100vh-64px)] overflow-y-auto",
                    {sidebar_content}
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
