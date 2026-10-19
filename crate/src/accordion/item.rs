use dioxus::prelude::*;

use super::{AccordionContext, AccordionItemContext};

/// Individual collapsible item container within an [`super::Accordion`].
///
/// Wraps an [`super::AccordionControl`] trigger and an [`super::AccordionPanel`] content region,
/// exposing contextual state to both subcomponents.
#[component]
pub fn AccordionItem(
    /// Unique identifier for this item within the parent [`super::Accordion`].
    value: String,
    /// Disables this item, preventing toggling.
    #[props(default)]
    disabled: bool,
    /// Additional CSS or SCSS classes.
    #[props(default)]
    class: String,
    /// Must contain [`super::AccordionControl`] and [`super::AccordionPanel`].
    children: Element,
) -> Element {
    let acc_ctx = use_context::<AccordionContext>();
    let is_expanded = acc_ctx.is_expanded(&value);

    let val_slug = value.replace(' ', "-").to_lowercase();
    let control_id = format!("{}-control-{}", acc_ctx.base_id, val_slug);
    let panel_id = format!("{}-panel-{}", acc_ctx.base_id, val_slug);

    let item_ctx = AccordionItemContext {
        value: value.clone(),
        disabled,
        is_expanded,
        control_id,
        panel_id,
    };
    provide_context(item_ctx);

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val
        .resolve_component_base("accordion-item")
        .unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("accordion-item", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let expanded_class = if is_expanded {
        "sd-accordion__item--expanded"
    } else {
        ""
    };
    let disabled_class = if disabled {
        "sd-accordion__item--disabled"
    } else {
        ""
    };

    rsx! {
        div {
            class: "sd-accordion__item {expanded_class} {disabled_class} {base_override} {class_modifiers} {class}",
            "data-expanded": "{is_expanded}",
            "data-disabled": "{disabled}",
            {children}
        }
    }
}
