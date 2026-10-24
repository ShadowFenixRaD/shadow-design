//! Sidebar navigation, computed from the registry (ADR-0006 §2). Desktop aside and mobile drawer
//! render the same `SidebarNav`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::app::theme_menu::preset_name;
use crate::pages::FOUNDATION;
use crate::story::registry;
use crate::story::schema::Category;
use crate::{Route, VERSION};

/// Foundation pages first, then the eight categories in `Category::ALL` order.
/// `filter` narrows the list (mobile drawer search); an empty filter shows everything.
#[component]
pub fn SidebarNav(#[props(default)] collapsed: bool, #[props(default)] filter: String) -> Element {
    let q = filter.trim().to_lowercase();
    let foundation: Vec<_> = FOUNDATION.iter().filter(|p| q.is_empty() || p.name.to_lowercase().contains(&q)).collect();
    let any = !foundation.is_empty()
        || Category::ALL.iter().any(|c| registry::by_category(*c).iter().any(|s| registry::matches(s, &q)));

    rsx! {
        for cat in Category::ALL {
            {
                let stories: Vec<_> = registry::by_category(cat).into_iter().filter(|s| registry::matches(s, &q)).collect();
                let pages = if cat == Category::Fondamenta { foundation.clone() } else { Vec::new() };
                rsx! {
                    if !stories.is_empty() || !pages.is_empty() {
                        nav { class: "sc-nav", key: "{cat.label()}", aria_label: "{cat.label()}",
                            if collapsed {
                                div { class: "sc-nav__sep" }
                            } else {
                                p { class: "sc-nav__label",
                                    Icon { kind: cat.icon(), size: 12 }
                                    "{cat.label()}"
                                }
                            }
                            for p in pages {
                                SidebarLink { key: "{p.name}", to: p.route.clone(), icon: p.icon, label: p.name, collapsed }
                            }
                            for s in stories {
                                SidebarLink { key: "{s.slug}", to: Route::story(s.slug), icon: s.icon, label: s.name, collapsed }
                            }
                        }
                    }
                }
            }
        }
        if !any {
            p { class: "sc-hint sc-nav__empty", "Nessun risultato per \"{filter}\"." }
        }
        if !collapsed && q.is_empty() {
            ThemeCard {}
        }
    }
}

/// "Tema attivo" summary at the bottom of the sidebar. Static: it reports, it does not act.
#[component]
fn ThemeCard() -> Element {
    let theme = use_theme();
    let t = theme();
    let accent = preset_name(&t.primary_color);
    let radius = crate::app::theme_menu::radius_key(t.radius_scale);
    let glow = crate::app::theme_menu::glow_key(t.glow_mode);
    rsx! {
        Card { class: "sc-theme-card".to_string(), title: Some("Tema attivo".to_string()),
            buttons: Some(rsx! {
                Badge { label: format!("v{VERSION}"), pill: true }
            }),
            dl { class: "sc-theme-card__rows",
                dt { "Preset" }
                dd { "{t.name}" }
                dt { "Accento" }
                dd { "{accent}" }
                dt { "Raggio" }
                dd { "{radius}" }
                dt { "Glow" }
                dd { "{glow}" }
            }
        }
    }
}

/// Router `Link` carrying the library sidebar classes (`sd-sidebar-link`).
#[component]
pub fn SidebarLink(to: Route, icon: IconKind, label: &'static str, #[props(default = false)] collapsed: bool) -> Element {
    let theme = use_theme();
    rsx! {
        Link {
            to,
            class: "{theme().sidebar_link}",
            active_class: "{theme().sidebar_link_active}",
            title: "{label}",
            Icon { kind: icon, size: 16 }
            if !collapsed {
                span { class: "truncate", "{label}" }
            }
        }
    }
}
