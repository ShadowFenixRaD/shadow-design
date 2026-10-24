//! Shell: header + sidebar (desktop) / drawer (mobile) + routed page.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::app::header::Header;
use crate::app::search::SearchPalette;
use crate::app::sidebar::SidebarNav;
use crate::Route;

/// Scrolls the active sidebar link to the middle of its own scroll container (desktop
/// aside and mobile drawer body), without scrolling the page.
const SCROLL_ACTIVE_JS: &str = r#"
requestAnimationFrame(() => requestAnimationFrame(() => {
  for (const sel of ['.sc-sidebar', '.sc-sidebar-drawer']) {
    const root = document.querySelector(sel);
    const el = root && root.querySelector('.sd-sidebar-link--active');
    if (!el) continue;
    let box = el.parentElement;
    while (box && box !== document.body && box.scrollHeight <= box.clientHeight) box = box.parentElement;
    if (!box || box === document.body) continue;
    const delta = el.getBoundingClientRect().top - box.getBoundingClientRect().top;
    box.scrollTop += delta - box.clientHeight / 2;
  }
}));
"#;

#[component]
pub fn Layout() -> Element {
    let theme = use_theme();
    let sidebar_collapsed = use_signal(|| false);
    let mut drawer_open = use_signal(|| false);
    let mut search_open = use_signal(|| false);
    let mut filter = use_signal(String::new);
    let route = use_route::<Route>();
    let wide = matches!(route, Route::Home {});

    use_hotkey("Ctrl+K", move || search_open.set(true));

    // On every navigation (and on first load): close the drawer, clear its filter and
    // bring the active sidebar item into view.
    use_effect(use_reactive((&route,), move |(_r,)| {
        drawer_open.set(false);
        filter.set(String::new());
        let _ = document::eval(SCROLL_ACTIVE_JS);
    }));
    // When the drawer opens, its list is mounted: centre the active item in it.
    use_effect(move || {
        if drawer_open() {
            let _ = document::eval(SCROLL_ACTIVE_JS);
        }
    });

    rsx! {
        div { class: "{theme().root} sc-app",
            SearchPalette { is_open: search_open }
            Header { sidebar_collapsed, drawer_open, search_open }

            Drawer {
                is_open: drawer_open,
                title: Some("Navigazione".to_string()),
                position: DrawerPosition::Left,
                size: DrawerSize::Sm,
                div { class: "sc-sidebar-drawer",
                    TextInput {
                        value: filter,
                        placeholder: Some("Filtra componenti".to_string()),
                        left_icon: Some(SvgIcon::kind(IconKind::Search)),
                        clearable: true,
                    }
                    SidebarNav { collapsed: false, filter: filter() }
                }
            }

            div { class: "sc-shell",
                aside {
                    class: if sidebar_collapsed() { "sc-sidebar sc-sidebar--collapsed" } else { "sc-sidebar" },
                    aria_label: "Navigazione componenti",
                    SidebarNav { collapsed: sidebar_collapsed() }
                }

                main { class: "{theme().content} sc-main",
                    div { class: if wide { "sc-page sc-page--wide" } else { "sc-page" }, Outlet::<Route> {} }
                }
            }
        }
    }
}
