use dioxus::prelude::*;

/// Proprietà del piè di pagina [`AppShellFooter`].
#[derive(Props, Clone, PartialEq)]
pub struct AppShellFooterProps {
    /// Altezza esplicita del footer (default: "40px").
    #[props(default)]
    pub height: Option<String>,
    /// Include la sottile linea superiore semi-trasparente. Default: true.
    #[props(default = true)]
    pub border: bool,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Contenuto del footer (status bar, copyright, metriche di sistema).
    pub children: Element,
}

/// Zona inferiore dell'AppShell a tutta larghezza con z-index 20.
#[component]
pub fn AppShellFooter(props: AppShellFooterProps) -> Element {
    let height_val = props.height.unwrap_or_else(|| "40px".to_string());
    let border_class = if props.border {
        "sd-app-shell__footer--border"
    } else {
        ""
    };

    rsx! {
        footer {
            role: "contentinfo",
            class: "sd-app-shell__footer {border_class} {props.class}",
            style: "height: {height_val}; min-height: {height_val};",
            {props.children}
        }
    }
}
