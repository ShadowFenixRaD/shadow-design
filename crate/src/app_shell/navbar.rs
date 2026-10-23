use crate::app_shell::AppShellCtx;
use dioxus::prelude::*;

/// Proprietà del pannello di navigazione sinistro [`AppShellNavbar`].
#[derive(Props, Clone, PartialEq)]
pub struct AppShellNavbarProps {
    /// Larghezza da aperto (default: "260px").
    #[props(default)]
    pub width: Option<String>,
    /// Stato di collasso controllato localmente (sovrascrive il context di AppShell se specificato).
    #[props(default)]
    pub collapsed: Option<bool>,
    /// Include il bordo separatore destro. Default: true.
    #[props(default = true)]
    pub border: bool,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Elementi della barra laterale (voci di navigazione, menu ad albero, sezioni).
    pub children: Element,
}

/// Pannello di navigazione laterale sinistro collassabile con z-index 25.
#[component]
pub fn AppShellNavbar(props: AppShellNavbarProps) -> Element {
    let ctx = try_use_context::<AppShellCtx>();
    let is_collapsed = props
        .collapsed
        .unwrap_or_else(|| ctx.map(|c| (c.navbar_collapsed)()).unwrap_or(false));

    let width_val = props.width.unwrap_or_else(|| "260px".to_string());
    let border_class = if props.border {
        "sd-app-shell__navbar--border"
    } else {
        ""
    };
    let collapsed_class = if is_collapsed {
        "sd-app-shell__navbar--collapsed"
    } else {
        ""
    };

    rsx! {
        aside {
            role: "navigation",
            "aria-label": "Navigazione laterale dell'applicazione",
            "aria-hidden": if is_collapsed { "true" } else { "false" },
            inert: is_collapsed,
            class: "sd-app-shell__navbar {border_class} {collapsed_class} {props.class}",
            style: if is_collapsed {
                "width: 0px; min-width: 0px; max-width: 0px; padding: 0px; border-right-width: 0px;"
            } else {
                "width: {width_val}; min-width: {width_val};"
            },
            div {
                class: "sd-app-shell__navbar-inner",
                style: "width: {width_val}; min-width: {width_val};",
                {props.children}
            }
        }
    }
}
