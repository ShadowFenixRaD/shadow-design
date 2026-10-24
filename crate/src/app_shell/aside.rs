use crate::app_shell::AppShellCtx;
use dioxus::prelude::*;

/// Proprietà del pannello laterale destro [`AppShellAside`].
#[derive(Props, Clone, PartialEq)]
pub struct AppShellAsideProps {
    /// Larghezza da aperto (default: "280px").
    #[props(default)]
    pub width: Option<String>,
    /// Stato di collasso controllato localmente (sovrascrive il context di AppShell se specificato).
    #[props(default)]
    pub collapsed: Option<bool>,
    /// Include il bordo separatore sinistro. Default: true.
    #[props(default = true)]
    pub border: bool,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Elementi del pannello ausiliario (ispettore, chat P2P, dettagli nodo).
    pub children: Element,
}

/// Pannello laterale destro collassabile con z-index 25 per ispettori e chat.
#[component]
pub fn AppShellAside(props: AppShellAsideProps) -> Element {
    let locale = crate::locale::use_locale();
    let ctx = try_use_context::<AppShellCtx>();
    let is_collapsed = props
        .collapsed
        .unwrap_or_else(|| ctx.map(|c| (c.aside_collapsed)()).unwrap_or(false));

    let width_val = props.width.unwrap_or_else(|| "280px".to_string());
    let border_class = if props.border {
        "sd-app-shell__aside--border"
    } else {
        ""
    };
    let collapsed_class = if is_collapsed {
        "sd-app-shell__aside--collapsed"
    } else {
        ""
    };

    rsx! {
        aside {
            role: "complementary",
            "aria-label": "{locale.app_shell_aside_label}",
            "aria-hidden": if is_collapsed { "true" } else { "false" },
            inert: is_collapsed,
            class: "sd-app-shell__aside {border_class} {collapsed_class} {props.class}",
            style: if is_collapsed {
                "width: 0px; min-width: 0px; max-width: 0px; padding: 0px; border-left-width: 0px;"
            } else {
                "width: {width_val}; min-width: {width_val};"
            },
            div {
                class: "sd-app-shell__aside-inner",
                style: "width: {width_val}; min-width: {width_val};",
                {props.children}
            }
        }
    }
}
