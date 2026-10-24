use dioxus::prelude::*;

/// Responsive navigation header landmark with brand branding, actions container, and accessible menu toggle.
#[component]
pub fn Navbar(
    brand: Element,
    /// Accessible landmark label for screen readers. Defaults to the locale `navbar_label` string.
    #[props(default)]
    aria_label: String,
    /// If true, renders a mobile disclosure button below 640px viewport width.
    #[props(default)]
    collapsible: bool,
    /// Accessible label for the mobile toggle button.
    #[props(default)]
    toggle_aria_label: Option<String>,
    #[props(default)]
    class: String,
    children: Element,
) -> Element {
    let locale = crate::locale::use_locale();
    let aria_label = if aria_label.is_empty() { locale.navbar_label.clone() } else { aria_label };
    let mut is_open = use_signal(|| false);

    let nav_id = crate::a11y::use_unique_id("navbar", None);
    let actions_id = format!("{nav_id}-actions");
    let toggle_id = format!("{nav_id}-toggle");

    let collapsible_class = if collapsible { "sd-navbar--collapsible" } else { "" };
    let open_class = if collapsible && *is_open.read() { "sd-navbar--open" } else { "" };

    let effective_toggle_aria = toggle_aria_label.unwrap_or_else(|| {
        if *is_open.read() {
            locale.navbar_close.clone()
        } else {
            locale.navbar_open.clone()
        }
    });

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("navbar").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("navbar", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let toggle_icon = {
        #[cfg(feature = "icon")]
        {
            if *is_open.read() {
                rsx! { crate::icon::Icon { kind: crate::icon::IconKind::X, size: 18 } }
            } else {
                rsx! { crate::icon::Icon { kind: crate::icon::IconKind::Menu, size: 18 } }
            }
        }
        #[cfg(not(feature = "icon"))]
        {
            if *is_open.read() {
                rsx! { "✕" }
            } else {
                rsx! { "☰" }
            }
        }
    };

    rsx! {
        nav {
            "aria-label": "{aria_label}",
            class: "sd-navbar {base_override} {collapsible_class} {open_class} {class_modifiers} {class}",
            div {
                class: "sd-navbar__inner",
                div { class: "sd-navbar__brand", {brand} }
                if collapsible {
                    button {
                        r#type: "button",
                        id: "{toggle_id}",
                        class: "sd-navbar__toggle",
                        "aria-label": "{effective_toggle_aria}",
                        "aria-expanded": if *is_open.read() { "true" } else { "false" },
                        "aria-controls": "{actions_id}",
                        onclick: move |_| {
                            let curr = *is_open.read();
                            *is_open.write() = !curr;
                        },
                        onkeydown: move |e: KeyboardEvent| {
                            if e.key() == Key::Escape && *is_open.read() {
                                e.prevent_default();
                                *is_open.write() = false;
                            }
                        },
                        {toggle_icon}
                    }
                }
                div {
                    id: "{actions_id}",
                    class: format!(
                        "sd-navbar__actions {}",
                        if collapsible && *is_open.read() {
                            "sd-navbar__actions--open"
                        } else {
                            ""
                        }
                    ),
                    onkeydown: move |e: KeyboardEvent| {
                        if e.key() == Key::Escape && collapsible && *is_open.read() {
                            e.prevent_default();
                            *is_open.write() = false;
                            let tid = toggle_id.clone();
                            let _ = document::eval(&format!(r#"
                                const btn = document.getElementById("{tid}");
                                if (btn) btn.focus();
                            "#));
                        }
                    },
                    {children}
                }
            }
        }
    }
}
