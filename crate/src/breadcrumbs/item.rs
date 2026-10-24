use crate::icon::{RenderSvgIcon, SvgIcon};
use dioxus::prelude::*;

/// Singolo elemento di navigazione all'interno di [`Breadcrumbs`].
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbItemProps {
    /// URL di destinazione. Se non specificato e privo di handler `on_click`, è renderizzato come testo passivo.
    #[props(default)]
    pub href: Option<String>,
    /// Indica se questo è l'elemento corrente attivo della pagina (applica `aria-current="page"`).
    #[props(default)]
    pub is_current: bool,
    /// Icona opzionale visualizzata a sinistra dell'etichetta.
    #[props(default)]
    pub icon: Option<SvgIcon>,
    /// Callback invocata al click dell'elemento.
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    /// Classi CSS addizionali per il tag item.
    #[props(default)]
    pub class: String,
    /// Contenuto o etichetta dell'elemento.
    pub children: Element,
}

/// Componente BreadcrumbItem per ciascuna tappa del percorso di navigazione.
#[component]
pub fn BreadcrumbItem(props: BreadcrumbItemProps) -> Element {
    let current_class = if props.is_current {
        "sd-breadcrumb-item--current"
    } else {
        ""
    };
    let has_action = props.href.is_some() || props.on_click.is_some();

    let icon_el = if let Some(ref ic) = props.icon {
        rsx! {
            span { class: "sd-breadcrumb-item__icon",
                RenderSvgIcon { icon: ic.clone(), size: 14 }
            }
        }
    } else {
        rsx! {}
    };

    if props.is_current || !has_action {
        rsx! {
            li {
                class: "sd-breadcrumb-item {current_class} {props.class}",
                "aria-current": if props.is_current { Some("page") } else { None },
                span { class: "sd-breadcrumb-item__content",
                    {icon_el}
                    span { class: "sd-breadcrumb-item__label", {props.children} }
                }
            }
        }
    } else if let Some(ref href) = props.href {
        rsx! {
            li {
                class: "sd-breadcrumb-item {props.class}",
                a {
                    class: "sd-breadcrumb-item__link",
                    href: "{href}",
                    onclick: move |evt| {
                        if let Some(ref handler) = props.on_click {
                            handler.call(evt);
                        }
                    },
                    {icon_el}
                    span { class: "sd-breadcrumb-item__label", {props.children} }
                }
            }
        }
    } else {
        rsx! {
            li {
                class: "sd-breadcrumb-item {props.class}",
                button {
                    r#type: "button",
                    class: "sd-breadcrumb-item__link sd-breadcrumb-item__button",
                    onclick: move |evt| {
                        if let Some(ref handler) = props.on_click {
                            handler.call(evt);
                        }
                    },
                    {icon_el}
                    span { class: "sd-breadcrumb-item__label", {props.children} }
                }
            }
        }
    }
}
