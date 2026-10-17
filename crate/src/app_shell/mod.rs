pub mod aside;
pub mod footer;
pub mod header;
pub mod main;
pub mod navbar;

pub use aside::{AppShellAside, AppShellAsideProps};
pub use footer::{AppShellFooter, AppShellFooterProps};
pub use header::{AppShellHeader, AppShellHeaderProps};
pub use main::{AppShellMain, AppShellMainProps};
pub use navbar::{AppShellNavbar, AppShellNavbarProps};

use dioxus::prelude::*;

/// Contesto condiviso dell'AppShell per monitorare o pilotare lo stato di collasso di navbar e aside.
#[derive(Clone, Copy, PartialEq)]
pub struct AppShellCtx {
    pub navbar_collapsed: Signal<bool>,
    pub aside_collapsed: Signal<bool>,
}

/// Proprietà del layout contenitore master [`AppShell`].
#[derive(Props, Clone, PartialEq)]
pub struct AppShellProps {
    /// Slot superiore opzionale per [`AppShellHeader`].
    #[props(default)]
    pub header: Option<Element>,
    /// Slot laterale sinistro opzionale per [`AppShellNavbar`].
    #[props(default)]
    pub navbar: Option<Element>,
    /// Slot laterale destro opzionale per [`AppShellAside`].
    #[props(default)]
    pub aside: Option<Element>,
    /// Slot inferiore opzionale per [`AppShellFooter`].
    #[props(default)]
    pub footer: Option<Element>,
    /// Segnale reattivo per controllare il collasso della barra laterale sinistra (Navbar).
    #[props(default)]
    pub navbar_collapsed: Option<Signal<bool>>,
    /// Segnale reattivo per controllare il collasso del pannello laterale destro (Aside).
    #[props(default)]
    pub aside_collapsed: Option<Signal<bool>>,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Area centrale dei contenuti ([`AppShellMain`] o elementi diretti).
    pub children: Element,
}

/// Layout master a 5 zone (Header, Navbar, Aside, Main, Footer) per applicazioni complesse.
/// Rispetta rigorosamente la scala z-index del sistema (`Header z-30`, `Navbar z-25`, `Aside z-25`, `Footer z-20`, `AppShell z-10`),
/// include il top specular hairline e gestisce lo scrolling indipendente e le transizioni fluide di collasso.
#[component]
pub fn AppShell(props: AppShellProps) -> Element {
    let internal_nav_collapsed = use_signal(|| false);
    let nav_collapsed = props.navbar_collapsed.unwrap_or(internal_nav_collapsed);

    let internal_aside_collapsed = use_signal(|| false);
    let aside_collapsed = props.aside_collapsed.unwrap_or(internal_aside_collapsed);

    use_context_provider(|| AppShellCtx {
        navbar_collapsed: nav_collapsed,
        aside_collapsed,
    });

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("app_shell").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("app_shell", &props.class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        div {
            class: "sd-app-shell {base_override} {class_modifiers} {props.class}",
            if let Some(ref hdr) = props.header {
                {hdr}
            }
            div { class: "sd-app-shell__body",
                if let Some(ref nav) = props.navbar {
                    {nav}
                }
                div { class: "sd-app-shell__content-area",
                    {props.children}
                }
                if let Some(ref asd) = props.aside {
                    {asd}
                }
            }
            if let Some(ref ftr) = props.footer {
                {ftr}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_shell_props_construction() {
        let props = AppShellProps {
            header: None,
            navbar: None,
            aside: None,
            footer: None,
            navbar_collapsed: None,
            aside_collapsed: None,
            class: String::new(),
            children: rsx! {},
        };
        assert!(props.class.is_empty());
    }
}
