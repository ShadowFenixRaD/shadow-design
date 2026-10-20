use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

use super::{AccordionContext, AccordionItemContext};

/// Interactive header / toggle control for an [`super::AccordionItem`].
///
/// Clicking this button expands or collapses the associated [`super::AccordionPanel`].
/// Supports an optional leading icon, custom or default rotating chevron indicator,
/// and complete WAI-ARIA accordion header keyboard accessibility.
#[component]
pub fn AccordionControl(
    /// Optional leading icon displayed before the header label.
    #[props(default)]
    icon: Option<IconKind>,
    /// Custom chevron element overriding the default animated chevron down icon.
    #[props(default)]
    chevron: Option<Element>,
    /// Disables interaction specifically on this control button.
    #[props(default)]
    disabled: bool,
    /// Additional CSS or SCSS classes.
    #[props(default)]
    class: String,
    /// Content of the control header label.
    children: Element,
) -> Element {
    let acc_ctx = use_context::<AccordionContext>();
    let item_ctx = use_context::<AccordionItemContext>();

    let is_disabled = disabled || item_ctx.disabled;
    let is_expanded = acc_ctx.is_expanded(&item_ctx.value);
    let value = item_ctx.value.clone();

    let mut acc_ctx_click = acc_ctx.clone();
    let toggle_val = value.clone();
    let on_click = move |e: MouseEvent| {
        e.stop_propagation();
        if !is_disabled {
            acc_ctx_click.toggle(&toggle_val);
        }
    };

    let mut acc_ctx_key = acc_ctx;
    let on_keydown = move |e: KeyboardEvent| {
        if is_disabled {
            return;
        }
        if e.key() == Key::Enter || e.key() == Key::Character(" ".into()) {
            e.prevent_default();
            acc_ctx_key.toggle(&value);
        }
    };

    let expanded_class = if is_expanded {
        "sd-accordion__control--expanded"
    } else {
        ""
    };
    let disabled_class = if is_disabled {
        "sd-accordion__control--disabled"
    } else {
        ""
    };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val
        .resolve_component_base("accordion-control")
        .unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("accordion-control", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        button {
            r#type: "button",
            id: "{item_ctx.control_id}",
            "aria-expanded": "{is_expanded}",
            "aria-controls": "{item_ctx.panel_id}",
            "aria-disabled": if is_disabled { "true" } else { "false" },
            disabled: is_disabled,
            class: "sd-accordion__control {expanded_class} {disabled_class} {base_override} {class_modifiers} {class}",
            onclick: on_click,
            onkeydown: on_keydown,

            if let Some(leading_icon) = icon {
                span { class: "sd-accordion__control-icon",
                    Icon { kind: leading_icon, size: 18 }
                }
            }

            span { class: "sd-accordion__control-label",
                {children}
            }

            span {
                class: format!(
                    "sd-accordion__chevron {}",
                    if is_expanded { "sd-accordion__chevron--expanded" } else { "" }
                ),
                if let Some(custom_chevron) = chevron {
                    {custom_chevron}
                } else {
                    Icon { kind: IconKind::ChevronDown, size: 16 }
                }
            }
        }
    }
}
