use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::{Route, HEADER_SVG};

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col bg-[#000000] text-zinc-100 selection:bg-purple-800 selection:text-white",
            // Header
            header { class: "border-b border-purple-950/60 bg-black/90 backdrop-blur-md sticky top-0 z-30 px-6 py-3.5 flex items-center justify-between shadow-[0_4px_30px_rgba(0,0,0,0.8)]",
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
                    span { class: "hidden sm:inline text-xs text-purple-400/90 font-mono tracking-wide px-2.5 py-1 rounded-full border border-purple-900/60 bg-purple-950/30 shadow-[0_0_10px_rgba(107,33,168,0.2)]", "ORGANIZZAZIONE SHADOWFENIX" }
                    span { class: "text-xs text-zinc-400", "Espositore Componenti UI" }
                }
            }

            // Main container with Sidebar and Content
            div { class: "flex-1 flex max-w-7xl w-full mx-auto",
                // Sidebar
                aside { class: "w-64 border-r border-purple-950/40 p-5 flex flex-col gap-6 shrink-0 bg-black",
                    div {
                        p { class: "text-xs font-semibold text-purple-400/80 uppercase tracking-wider mb-2 font-mono", "Generale" }
                        div { class: "flex flex-col gap-1",
                            SidebarLink { to: Route::Home {}, icon: "🏠", label: "Panoramica" }
                        }
                    }

                    div {
                        p { class: "text-xs font-semibold text-purple-400/80 uppercase tracking-wider mb-2 font-mono", "Componenti UI" }
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

                    div { class: "mt-auto p-3.5 rounded-xl bg-[#09080e] border border-purple-950/70 text-xs text-zinc-400 shadow-lg shadow-purple-950/20",
                        div { class: "flex items-center gap-1.5 mb-1 text-purple-300 font-semibold",
                            span { "✨" }
                            span { "Shadow Theme" }
                        }
                        p { class: "text-zinc-400 text-[11px] leading-relaxed", "Sfondo nero puro 0 0 0 e viola scuro profondo ispirato a shadowfenix." }
                    }
                }

                // Page Content
                main { class: "flex-1 p-8 overflow-y-auto bg-black",
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
            class: "flex items-center gap-2.5 px-3 py-2 rounded-lg text-sm font-medium text-zinc-400 hover:text-purple-200 hover:bg-purple-950/30 transition-all",
            active_class: "bg-purple-950/60 text-purple-300 font-semibold border border-purple-800/60 shadow-[0_0_15px_rgba(107,33,168,0.25)]",
            span { class: "text-base", "{icon}" }
            span { "{label}" }
        }
    }
}
