use dioxus::prelude::*;
use crate::badge::{Badge, BadgeVariant};
use crate::icon::{RenderSvgIcon, SvgIcon};

/// Proprietà della singola voce o destinazione [`BottomNavItem`].
#[derive(Props, Clone, PartialEq)]
pub struct BottomNavItemProps {
    /// Etichetta testuale della destinazione visualizzata sotto l'icona.
    pub label: String,
    /// Icona vettoriale di contesto.
    pub icon: SvgIcon,
    /// Indica se questa destinazione è attualmente attiva.
    #[props(default)]
    pub active: bool,
    /// Badge numerico o indicatore testuale posizionato sopra l'icona (es. "3", "•", "NEW").
    #[props(default)]
    pub badge: Option<String>,
    /// Variante cromatica del badge (default: `BadgeVariant::Default`).
    #[props(default)]
    pub badge_variant: Option<BadgeVariant>,
    /// URL di navigazione (renderizza un tag `<a>`).
    #[props(default)]
    pub href: Option<String>,
    /// Callback invocata al click sulla destinazione.
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    /// Disabilita la voce prevenendo interazioni.
    #[props(default)]
    pub disabled: bool,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
}

/// Singola destinazione all'interno della barra di navigazione inferiore.
#[component]
pub fn BottomNavItem(props: BottomNavItemProps) -> Element {
    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("bottom_nav_item").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("bottom_nav_item", &props.class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let active_class = if props.active { "sd-bottom-nav-item--active" } else { "" };
    let disabled_class = if props.disabled { "sd-bottom-nav-item--disabled" } else { "" };

    let badge_el = if let Some(ref b) = props.badge {
        let variant = props.badge_variant.unwrap_or(BadgeVariant::Default);
        rsx! {
            span { class: "sd-bottom-nav-item__badge-wrapper",
                Badge {
                    label: b.clone(),
                    variant: variant,
                    pill: true,
                }
            }
        }
    } else {
        rsx! {}
    };

    let inner_content = rsx! {
        span { class: "sd-bottom-nav-item__icon-container",
            RenderSvgIcon { icon: props.icon.clone(), size: 20 }
            {badge_el}
            span { class: "sd-bottom-nav-item__active-pill", "aria-hidden": "true" }
        }
        span { class: "sd-bottom-nav-item__label", "{props.label}" }
    };

    if let Some(ref href) = props.href {
        let href_attr = if props.disabled { None } else { Some(href.as_str()) };
        let aria_disabled = if props.disabled { Some("true") } else { None };
        let tabindex_attr = if props.disabled { Some("-1") } else { None };
        rsx! {
            a {
                class: "sd-bottom-nav-item {active_class} {disabled_class} {base_override} {class_modifiers} {props.class}",
                href: href_attr,
                "aria-disabled": aria_disabled,
                tabindex: tabindex_attr,
                "aria-current": if props.active { Some("page") } else { None },
                onclick: move |evt| {
                    if props.disabled {
                        evt.prevent_default();
                        return;
                    }
                    if let Some(ref handler) = props.on_click {
                        handler.call(evt);
                    }
                },
                {inner_content}
            }
        }
    } else {
        rsx! {
            button {
                r#type: "button",
                class: "sd-bottom-nav-item sd-bottom-nav-item--button {active_class} {disabled_class} {base_override} {class_modifiers} {props.class}",
                disabled: props.disabled,
                "aria-current": if props.active { "page" } else { "" },
                onclick: move |evt| {
                    if let Some(ref handler) = props.on_click {
                        handler.call(evt);
                    }
                },
                {inner_content}
            }
        }
    }
}
