use dioxus::prelude::*;

/// Proprietà dell'intestazione [`AppShellHeader`].
#[derive(Props, Clone, PartialEq)]
pub struct AppShellHeaderProps {
    /// Altezza esplicita dell'header (default: "60px").
    #[props(default)]
    pub height: Option<String>,
    /// Rende l'header ancorato/sticky in cima allo shell (default: true).
    #[props(default = true)]
    pub sticky: bool,
    /// Include la sottile linea inferiore semi-trasparente con top specular hairline. Default: true.
    #[props(default = true)]
    pub border: bool,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Contenuto dell'header (branding, logo, toolbar, toggle collasso).
    pub children: Element,
}

/// Zona superiore dell'AppShell a tutta larghezza con z-index 30.
#[component]
pub fn AppShellHeader(props: AppShellHeaderProps) -> Element {
    let height_val = props.height.unwrap_or_else(|| "60px".to_string());
    let border_class = if props.border {
        "sd-app-shell__header--border"
    } else {
        ""
    };
    let sticky_class = if props.sticky {
        "sd-app-shell__header--sticky"
    } else {
        ""
    };

    rsx! {
        header {
            role: "banner",
            class: "sd-app-shell__header {border_class} {sticky_class} {props.class}",
            style: "height: {height_val}; min-height: {height_val};",
            {props.children}
        }
    }
}
