pub(crate) mod control;
pub(crate) mod item;
pub(crate) mod panel;

pub use control::*;
pub use item::*;
pub use panel::*;

use dioxus::prelude::*;

/// Visual presentation variant for the [`Accordion`] container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AccordionVariant {
    /// Seamless list styling with thin separators between items.
    #[default]
    Default,
    /// Boxed surface with a unified border and background.
    Contained,
    /// Distinct separated cards with gaps between each item.
    Separated,
}

impl AccordionVariant {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "sd-accordion--default",
            Self::Contained => "sd-accordion--contained",
            Self::Separated => "sd-accordion--separated",
        }
    }
}

/// Shared context propagated down from [`Accordion`] to coordinated items.
#[derive(Clone, PartialEq)]
pub struct AccordionContext {
    pub multiple: Signal<bool>,
    pub value: Signal<Vec<String>>,
    pub variant: AccordionVariant,
    pub base_id: String,
}

impl AccordionContext {
    /// Checks whether an item with the given value identifier is currently open.
    #[must_use]
    pub fn is_expanded(&self, val: &str) -> bool {
        self.value.read().contains(&val.to_string())
    }

    /// Toggles the expanded status of an item.
    pub fn toggle(&mut self, val: &str) {
        let mut list = self.value.write();
        if *self.multiple.read() {
            if let Some(pos) = list.iter().position(|x| x == val) {
                list.remove(pos);
            } else {
                list.push(val.to_string());
            }
        } else if list.contains(&val.to_string()) {
            list.clear();
        } else {
            *list = vec![val.to_string()];
        }
    }
}

/// Shared context for an individual [`AccordionItem`].
#[derive(Clone, PartialEq)]
pub struct AccordionItemContext {
    pub value: String,
    pub disabled: bool,
    pub is_expanded: bool,
    pub control_id: String,
    pub panel_id: String,
}

/// Accessible, multi-variant accordion container following WAI-ARIA APG standards.
///
/// Supports single or multiple item expansion, keyboard navigation,
/// smooth height transitions, and three visual variants (`Default`, `Contained`, `Separated`).
#[component]
pub fn Accordion(
    /// Whether multiple panels can be expanded simultaneously. Defaults to `false`.
    #[props(default)]
    multiple: bool,
    /// Optional externally managed signal holding the list of expanded item values.
    #[props(default)]
    value: Option<Signal<Vec<String>>>,
    /// Default values expanded when unmanaged.
    #[props(default)]
    default_value: Option<Vec<String>>,
    /// Visual style variant of the accordion.
    #[props(default)]
    variant: AccordionVariant,
    /// Unique identifier base for generating ARIA IDs.
    #[props(default)]
    id: Option<String>,
    /// Additional CSS or SCSS classes.
    #[props(default)]
    class: String,
    /// Must contain [`AccordionItem`] components.
    children: Element,
) -> Element {
    let base_id = crate::a11y::use_unique_id("sd-accordion", id);
    let fallback_signal = use_signal(|| default_value.unwrap_or_default());
    let active_value = value.unwrap_or(fallback_signal);

    let mut multiple_sig = use_signal(|| multiple);
    use_effect(move || {
        if *multiple_sig.peek() != multiple {
            *multiple_sig.write() = multiple;
        }
    });

    let ctx = AccordionContext {
        multiple: multiple_sig,
        value: active_value,
        variant,
        base_id: base_id.clone(),
    };
    provide_context(ctx);

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("accordion").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("accordion", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let variant_class = variant.as_str();

    rsx! {
        div {
            id: "{base_id}",
            class: "sd-accordion {variant_class} {base_override} {class_modifiers} {class}",
            {children}
        }
    }
}

/// Standalone collapsible container for toggling visibility of arbitrary elements.
///
/// Animates content height smoothly using pure CSS transitions without requiring JavaScript calculations.
#[component]
pub fn Collapsible(
    /// Reactive signal controlling whether the collapsible content is open.
    open: Signal<bool>,
    /// Additional CSS or SCSS classes.
    #[props(default)]
    class: String,
    /// Content inside the collapsible container.
    children: Element,
) -> Element {
    let is_open = *open.read();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val
        .resolve_component_base("collapsible")
        .unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("collapsible", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let state_class = if is_open {
        "sd-collapsible--open"
    } else {
        "sd-collapsible--closed"
    };

    rsx! {
        div {
            class: "sd-collapsible {state_class} {base_override} {class_modifiers} {class}",
            "aria-hidden": if !is_open { "true" } else { "false" },
            div { class: "sd-collapsible__inner",
                {children}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accordion_variant_classes() {
        assert_eq!(AccordionVariant::Default.as_str(), "sd-accordion--default");
        assert_eq!(
            AccordionVariant::Contained.as_str(),
            "sd-accordion--contained"
        );
        assert_eq!(
            AccordionVariant::Separated.as_str(),
            "sd-accordion--separated"
        );
    }

    #[test]
    fn test_single_expansion_toggle() {
        let mut dom = VirtualDom::new(|| {
            let val = use_signal(|| vec!["item-1".to_string()]);
            let mult = use_signal(|| false);
            let mut ctx = AccordionContext {
                multiple: mult,
                value: val,
                variant: AccordionVariant::Default,
                base_id: "test".into(),
            };

            assert!(ctx.is_expanded("item-1"));
            assert!(!ctx.is_expanded("item-2"));

            // Toggle item-2: replaces item-1
            ctx.toggle("item-2");
            assert!(!ctx.is_expanded("item-1"));
            assert!(ctx.is_expanded("item-2"));

            // Toggle item-2 again: collapses all
            ctx.toggle("item-2");
            assert!(!ctx.is_expanded("item-2"));

            rsx! { div {} }
        });
        dom.rebuild_in_place();
    }

    #[test]
    fn test_multiple_expansion_toggle() {
        let mut dom = VirtualDom::new(|| {
            let val = use_signal(|| vec!["item-1".to_string()]);
            let mult = use_signal(|| true);
            let mut ctx = AccordionContext {
                multiple: mult,
                value: val,
                variant: AccordionVariant::Separated,
                base_id: "test".into(),
            };

            assert!(ctx.is_expanded("item-1"));
            assert!(!ctx.is_expanded("item-2"));

            // Toggle item-2: keeps item-1, adds item-2
            ctx.toggle("item-2");
            assert!(ctx.is_expanded("item-1"));
            assert!(ctx.is_expanded("item-2"));

            // Toggle item-1: removes item-1, keeps item-2
            ctx.toggle("item-1");
            assert!(!ctx.is_expanded("item-1"));
            assert!(ctx.is_expanded("item-2"));

            rsx! { div {} }
        });
        dom.rebuild_in_place();
    }
}
