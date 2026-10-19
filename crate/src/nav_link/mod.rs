pub mod group;
pub use group::{SidebarMenu, SidebarMenuProps};

use dioxus::prelude::*;
use crate::badge::{Badge, BadgeVariant};
use crate::icon::{Icon, IconKind, RenderSvgIcon, SvgIcon};

/// Proprietà del componente [`NavLink`].
#[derive(Props, Clone, PartialEq)]
pub struct NavLinkProps {
    /// Etichetta testuale principale del link.
    pub label: String,
    /// Descrizione secondaria opzionale posizionata sotto l'etichetta.
    #[props(default)]
    pub description: Option<String>,
    /// Icona vettoriale di contesto posizionata a sinistra.
    #[props(default)]
    pub icon: Option<SvgIcon>,
    /// Indica se questo link rappresenta la pagina o destinazione attualmente attiva.
    #[props(default)]
    pub active: bool,
    /// URL di navigazione (renderizza un tag `<a>`).
    #[props(default)]
    pub href: Option<String>,
    /// Callback invocata al click sul link.
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    /// Etichetta badge o contatore numerico a destra (es. "3", "NEW", "PRO").
    #[props(default)]
    pub badge: Option<String>,
    /// Variante cromatica del badge a destra (default: `BadgeVariant::Default`).
    #[props(default)]
    pub badge_variant: Option<BadgeVariant>,
    /// Stato di apertura controllato esternamente per i sottomenu.
    #[props(default)]
    pub opened: Option<Signal<bool>>,
    /// Stato iniziale di apertura per i sottomenu figli (default: false).
    #[props(default)]
    pub default_opened: bool,
    /// Disabilita il link prevenendo interazioni.
    #[props(default)]
    pub disabled: bool,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Sottomenu annidati (altri [`NavLink`] o elementi figli).
    #[props(default)]
    pub children: Option<Element>,
}

/// Componente NavLink per barre laterali di navigazione con indicatore neon attivo, icone e sottomenu collassabili.
#[component]
pub fn NavLink(props: NavLinkProps) -> Element {
    let mut local_opened = use_signal(|| props.default_opened);
    let has_children = props.children.is_some();
    let is_open = props.opened.map(|sig| sig()).unwrap_or(local_opened());

    let opened_sig = props.opened;
    let mut toggle_open = move || {
        if let Some(mut sig) = opened_sig {
            sig.set(!is_open);
        } else {
            local_opened.set(!is_open);
        }
    };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("nav_link").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("nav_link", &props.class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let active_class = if props.active { "sd-nav-link--active" } else { "" };
    let disabled_class = if props.disabled { "sd-nav-link--disabled" } else { "" };
    let open_class = if is_open { "sd-nav-link--open" } else { "" };

    let icon_el = if let Some(ref ic) = props.icon {
        rsx! {
            span { class: "sd-nav-link__icon",
                RenderSvgIcon { icon: ic.clone(), size: 16 }
            }
        }
    } else {
        rsx! {}
    };

    let badge_el = if let Some(ref b) = props.badge {
        let variant = props.badge_variant.unwrap_or(BadgeVariant::Default);
        rsx! {
            span { class: "sd-nav-link__badge",
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

    let chevron_el = if has_children {
        rsx! {
            span {
                class: if is_open { "sd-nav-link__chevron sd-nav-link__chevron--open" } else { "sd-nav-link__chevron" },
                Icon { kind: IconKind::ChevronRight, size: 14 }
            }
        }
    } else {
        rsx! {}
    };

    let actions_el = if props.badge.is_some() || has_children {
        rsx! {
            span { class: "sd-nav-link__actions",
                {badge_el}
                {chevron_el}
            }
        }
    } else {
        rsx! {}
    };

    let main_content = rsx! {
        span { class: "sd-nav-link__indicator", "aria-hidden": "true" }
        {icon_el}
        span { class: "sd-nav-link__body",
            span { class: "sd-nav-link__label", "{props.label}" }
            if let Some(ref desc) = props.description {
                span { class: "sd-nav-link__description", "{desc}" }
            }
        }
        {actions_el}
    };

    let interactive_item = if let Some(ref href) = props.href {
        rsx! {
            a {
                class: "sd-nav-link__item",
                href: "{href}",
                "aria-current": if props.active { "page" } else { "" },
                onclick: move |evt| {
                    if has_children {
                        toggle_open();
                    }
                    if let Some(ref handler) = props.on_click {
                        handler.call(evt);
                    }
                },
                {main_content}
            }
        }
    } else {
        rsx! {
            button {
                r#type: "button",
                class: "sd-nav-link__item sd-nav-link__item--button",
                disabled: props.disabled,
                "aria-current": if props.active { "page" } else { "" },
                onclick: move |evt| {
                    if has_children {
                        toggle_open();
                    }
                    if let Some(ref handler) = props.on_click {
                        handler.call(evt);
                    }
                },
                {main_content}
            }
        }
    };

    rsx! {
        div {
            class: "sd-nav-link {active_class} {disabled_class} {open_class} {base_override} {class_modifiers} {props.class}",
            {interactive_item}
            if has_children && is_open {
                div { class: "sd-nav-link__children",
                    {props.children}
                }
            }
        }
    }
}
