//! Top bar: navigation toggle, brand, search button, theme menu. One row on every breakpoint.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::app::theme_menu::ThemeMenuButton;
use crate::{Route, HEADER_SVG, VERSION};

#[component]
pub fn Header(mut sidebar_collapsed: Signal<bool>, drawer_open: Signal<bool>, mut search_open: Signal<bool>) -> Element {
    rsx! {
        header { class: "sc-topbar z-layer-header",
            // Below md the drawer opens from the burger; from md up the button collapses the sidebar.
            div { class: "sc-only-below-md",
                Burger { opened: drawer_open, size: BurgerSize::Sm, aria_label: "Apri la navigazione".to_string() }
            }
            Button {
                variant: ButtonVariant::Ghost,
                size: ButtonSize::Sm,
                class: "sc-hide-below-md",
                aria_label: Some("Comprimi o espandi la barra laterale".to_string()),
                aria_pressed: Some(sidebar_collapsed()),
                onclick: move |_| sidebar_collapsed.toggle(),
                Icon { kind: IconKind::Menu, size: 18 }
            }

            Link { to: Route::Home {}, class: "sc-brand",
                img { src: HEADER_SVG, class: "sc-brand__logo", alt: "" }
                span { class: "sc-brand__text",
                    span { class: "sc-brand__name", "Shadow Design" }
                    span { class: "sc-brand__tag", "by shadowfenix" }
                }
            }
            Badge { label: format!("v{VERSION}"), pill: true, class: "sc-hide-below-lg".to_string() }

            div { class: "sc-topbar__spacer" }

            Button {
                variant: ButtonVariant::Secondary,
                size: ButtonSize::Sm,
                aria_label: Some("Cerca componenti (Ctrl+K)".to_string()),
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Search, size: 14 }
                }),
                onclick: move |_| search_open.set(true),
                span { class: "sc-hide-below-sm", "Cerca" }
                Kbd { class: "sc-hide-below-md sc-kbd-hint", "Ctrl K" }
            }
            ThemeMenuButton {}
        }
    }
}
