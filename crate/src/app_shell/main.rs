use dioxus::prelude::*;

/// Proprietà dell'area di contenuto principale [`AppShellMain`].
#[derive(Props, Clone, PartialEq)]
pub struct AppShellMainProps {
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Contenuto principale applicativo.
    pub children: Element,
}

/// Area centrale di visualizzazione dei contenuti con scrolling indipendente.
#[component]
pub fn AppShellMain(props: AppShellMainProps) -> Element {
    rsx! {
        main {
            role: "main",
            class: "sd-app-shell__main {props.class}",
            {props.children}
        }
    }
}
