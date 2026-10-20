use dioxus::prelude::*;
use crate::icon::{Icon, IconKind};

/// Proprietà del contenitore o sezione di raggruppamento per voci di navigazione laterale.
#[derive(Props, Clone, PartialEq)]
pub struct SidebarMenuProps {
    /// Titolo della sezione (es. "GESTIONE SISTEMA", "WORKSPACE").
    #[props(default)]
    pub title: Option<String>,
    /// Se true, l'intera sezione può essere aperta/collassata.
    #[props(default)]
    pub collapsible: bool,
    /// Stato di apertura controllato esternamente.
    #[props(default)]
    pub opened: Option<Signal<bool>>,
    /// Stato di apertura iniziale se non controllato (default: true).
    #[props(default = true)]
    pub default_opened: bool,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Voci di navigazione figlie ([`NavLink`]).
    pub children: Element,
}

/// Raggruppamento semantico per barre laterali con intestazione opzionale e collasso.
#[component]
pub fn SidebarMenu(props: SidebarMenuProps) -> Element {
    let mut local_opened = use_signal(|| props.default_opened);
    let is_open = props.opened.map(|sig| sig()).unwrap_or(local_opened());

    let toggle = move |_| {
        if props.collapsible {
            if let Some(mut sig) = props.opened {
                sig.set(!is_open);
            } else {
                local_opened.set(!is_open);
            }
        }
    };

    rsx! {
        div { class: "sd-sidebar-menu {props.class}",
            if let Some(ref title) = props.title {
                if props.collapsible {
                    button {
                        r#type: "button",
                        class: "sd-sidebar-menu__header sd-sidebar-menu__header--collapsible",
                        onclick: toggle,
                        span { class: "sd-sidebar-menu__title", "{title}" }
                        span {
                            class: if is_open { "sd-sidebar-menu__chevron sd-sidebar-menu__chevron--open" } else { "sd-sidebar-menu__chevron" },
                            Icon { kind: IconKind::ChevronDown, size: 12 }
                        }
                    }
                } else {
                    div { class: "sd-sidebar-menu__header",
                        span { class: "sd-sidebar-menu__title", "{title}" }
                    }
                }
            }
            if is_open {
                div { class: "sd-sidebar-menu__content",
                    {props.children}
                }
            }
        }
    }
}
