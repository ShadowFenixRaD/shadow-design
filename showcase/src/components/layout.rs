use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::{Route, HEADER_SVG};

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col bg-[#0f1116] text-zinc-100",
            // Header
            header { class: "border-b border-zinc-800 bg-[#14171f]/80 backdrop-blur-md sticky top-0 z-30 px-6 py-3.5 flex items-center justify-between",
                div { class: "flex items-center gap-4",
                    Link {
                        to: Route::Home {},
                        class: "flex items-center gap-3 text-lg font-bold tracking-tight text-white hover:opacity-90 transition-opacity",
                        img { src: HEADER_SVG, class: "h-7 w-auto", alt: "Shadow Design" }
                        span { "Shadow Design" }
                    }
                    Badge {
                        label: "v0.1.0 • Dioxus 0.7".to_string(),
                        variant: BadgeVariant::Primary,
                        pill: true,
                    }
                }
                div { class: "flex items-center gap-3 text-sm text-zinc-400",
                    span { "Espositore Componenti UI" }
                }
            }

            // Main container with Sidebar and Content
            div { class: "flex-1 flex max-w-7xl w-full mx-auto",
                // Sidebar
                aside { class: "w-64 border-r border-zinc-800/80 p-5 flex flex-col gap-6 shrink-0",
                    div {
                        p { class: "text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-2", "Generale" }
                        div { class: "flex flex-col gap-1",
                            SidebarLink { to: Route::Home {}, icon: "🏠", label: "Panoramica" }
                        }
                    }

                    div {
                        p { class: "text-xs font-semibold text-zinc-500 uppercase tracking-wider mb-2", "Componenti UI" }
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

                    div { class: "mt-auto p-3.5 rounded-xl bg-zinc-900/60 border border-zinc-800/80 text-xs text-zinc-400",
                        p { class: "font-semibold text-zinc-300 mb-1", "Feature-Gated" }
                        p { "Ogni componente ha la sua feature dedicata in Cargo.toml." }
                    }
                }

                // Page Content
                main { class: "flex-1 p-8 overflow-y-auto",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

#[component]
pub fn SidebarLink(to: Route, icon: &'static str, label: &'static str) -> Element {
    rsx! {
        Link {
            to,
            class: "flex items-center gap-2.5 px-3 py-2 rounded-lg text-sm font-medium text-zinc-300 hover:text-white hover:bg-zinc-800/70 transition-colors",
            active_class: "bg-blue-600/15 text-blue-400 font-semibold border border-blue-500/30",
            span { class: "text-base", "{icon}" }
            span { "{label}" }
        }
    }
}
