use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::{Route, HEADER_SVG};

#[component]
pub fn Layout() -> Element {
    let mut theme = use_theme();

    let is_shadow = theme().name.contains("Shadow");
    let is_obsidian = theme().name.contains("Obsidian");
    let is_crimson = theme().name.contains("Crimson");

    let shadow_btn = if is_shadow {
        "px-2 py-1 text-[10px] rounded border font-mono transition-all cursor-pointer bg-purple-900 border-purple-500 text-white font-bold"
    } else {
        "px-2 py-1 text-[10px] rounded border font-mono transition-all cursor-pointer bg-black/60 border-zinc-800 text-zinc-400 hover:text-white"
    };

    let obsidian_btn = if is_obsidian {
        "px-2 py-1 text-[10px] rounded border font-mono transition-all cursor-pointer bg-zinc-800 border-zinc-400 text-white font-bold"
    } else {
        "px-2 py-1 text-[10px] rounded border font-mono transition-all cursor-pointer bg-black/60 border-zinc-800 text-zinc-400 hover:text-white"
    };

    let crimson_btn = if is_crimson {
        "px-2 py-1 text-[10px] rounded border font-mono transition-all cursor-pointer bg-rose-900 border-rose-500 text-white font-bold"
    } else {
        "px-2 py-1 text-[10px] rounded border font-mono transition-all cursor-pointer bg-black/60 border-zinc-800 text-zinc-400 hover:text-white"
    };

    rsx! {
        div { class: "{theme().root}",
            // Header
            header { class: "{theme().header} sticky top-0 z-30 px-6 py-3.5 flex items-center justify-between",
                div { class: "flex items-center gap-4",
                    Link {
                        to: Route::Home {},
                        class: "flex items-center gap-3 text-lg font-bold tracking-tight text-white hover:opacity-90 transition-opacity group",
                        div { class: "relative flex items-center justify-center",
                            div { class: "absolute -inset-1 rounded-full bg-purple-600/30 blur-sm group-hover:bg-purple-600/50 transition-all" }
                            img { src: HEADER_SVG, class: "relative h-8 w-auto drop-shadow-[0_0_8px_rgba(168,85,247,0.5)]", alt: "Shadowfenix" }
                        }
                        div { class: "flex flex-col leading-none",
                            span { class: "text-base font-extrabold tracking-wide text-white group-hover:text-purple-200 transition-colors", "Shadow Design" }
                            span { class: "text-[10px] font-mono tracking-widest text-purple-400 uppercase", "by shadowfenix" }
                        }
                    }
                    Badge {
                        label: "v0.1.0 • Dioxus 0.7".to_string(),
                        variant: BadgeVariant::Primary,
                        pill: true,
                    }
                }
                div { class: "flex items-center gap-4 text-sm text-zinc-400",
                    span { class: "hidden sm:inline text-xs font-mono tracking-wide px-2.5 py-1 rounded-full border border-purple-900/60 bg-purple-950/30 text-purple-300",
                        "TEMA: {theme().name.to_uppercase()}"
                    }
                    span { class: "text-xs text-zinc-400", "Espositore Componenti UI" }
                }
            }

            // Main container with Sidebar and Content
            div { class: "flex-1 flex max-w-7xl w-full mx-auto",
                // Sidebar
                aside { class: "{theme().sidebar} w-64 p-5 flex flex-col gap-6 shrink-0",
                    div {
                        p { class: "text-xs font-semibold {theme().text_accent} uppercase tracking-wider mb-2 font-mono", "Generale" }
                        div { class: "flex flex-col gap-1",
                            SidebarLink { to: Route::Home {}, icon: "🏠", label: "Panoramica" }
                        }
                    }

                    div {
                        p { class: "text-xs font-semibold {theme().text_accent} uppercase tracking-wider mb-2 font-mono", "Componenti UI" }
                        div { class: "flex flex-col gap-1",
                            SidebarLink { to: Route::ButtonsPage {}, icon: "🔘", label: "Button" }
                            SidebarLink { to: Route::CardsPage {}, icon: "🃏", label: "Card" }
                            SidebarLink { to: Route::InputsPage {}, icon: "📝", label: "TextInput" }
                            SidebarLink { to: Route::BadgesPage {}, icon: "🏷️", label: "Badge" }
                            SidebarLink { to: Route::AlertsPage {}, icon: "⚠️", label: "Alert" }
                            SidebarLink { to: Route::ModalsPage {}, icon: "🪟", label: "Modal" }
                            SidebarLink { to: Route::NavbarDemoPage {}, icon: "🧭", label: "Navbar" }
                        }
                    }

                    // Selettore Rapido Feature Theme
                    div { class: "{theme().surface} rounded-xl p-3.5 text-xs text-zinc-400 shadow-lg",
                        div { class: "flex items-center justify-between mb-2",
                            div { class: "flex items-center gap-1.5 font-semibold text-purple-300",
                                span { "🎨" }
                                span { "Feature Theme" }
                            }
                        }
                        p { class: "text-[11px] mb-2.5 text-zinc-400", "Cambia o sovrascrivi il tema in tempo reale:" }
                        div { class: "grid grid-cols-3 gap-1.5",
                            button {
                                class: "{shadow_btn}",
                                onclick: move |_| *theme.write() = Theme::shadow(),
                                "Shadow"
                            }
                            button {
                                class: "{obsidian_btn}",
                                onclick: move |_| *theme.write() = Theme::obsidian(),
                                "Obsidian"
                            }
                            button {
                                class: "{crimson_btn}",
                                onclick: move |_| *theme.write() = Theme::crimson(),
                                "Crimson"
                            }
                        }
                    }
                }

                // Page Content
                main { class: "{theme().content} flex-1 p-8 overflow-y-auto",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[component]
pub fn SidebarLink(to: Route, icon: &'static str, label: &'static str) -> Element {
    let theme = use_theme();
    rsx! {
        Link {
            to,
            class: "{theme().sidebar_link}",
            active_class: "{theme().sidebar_link_active}",
            span { class: "text-base", "{icon}" }
            span { "{label}" }
        }
    }
}
