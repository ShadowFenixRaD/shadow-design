use dioxus::prelude::*;

use super::{AccordionContext, AccordionItemContext};

/// Content region for an [`super::AccordionItem`].
///
/// Automatically connected via `id` and `aria-labelledby` with its companion [`super::AccordionControl`].
/// Rendered when expanded with smooth optical height animation and internal padding.
#[component]
pub fn AccordionPanel(
    /// Additional CSS or SCSS classes.
    #[props(default)]
    class: String,
    /// Content displayed inside the collapsible panel.
    children: Element,
) -> Element {
    let acc_ctx = try_use_context::<AccordionContext>();
    let item_ctx = use_context::<AccordionItemContext>();
    let is_expanded = acc_ctx
        .as_ref()
        .map(|ctx| ctx.is_expanded(&item_ctx.value))
        .unwrap_or(item_ctx.is_expanded);

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val
        .resolve_component_base("accordion-panel")
        .unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("accordion-panel", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let state_class = if is_expanded {
        "sd-accordion__panel--expanded"
    } else {
        "sd-accordion__panel--collapsed"
    };

    rsx! {
        div {
            role: "region",
            id: "{item_ctx.panel_id}",
            "aria-labelledby": "{item_ctx.control_id}",
            "aria-hidden": if !is_expanded { "true" } else { "false" },
            class: "sd-accordion__panel {state_class} {base_override} {class_modifiers} {class}",
            div { class: "sd-accordion__content",
                {children}
            }
        }
    }
}
