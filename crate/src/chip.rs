use dioxus::prelude::*;
use crate::a11y::ControlSize;
use crate::icon::{Icon, IconKind};
use crate::util::cx;

/// Visual variant styles for [`Chip`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ChipVariant {
    /// Solid filled accent background with micro-glow when active.
    #[default]
    Filled,
    /// Translucent surface with illuminated active border.
    Outline,
    /// Subtle accent background tint.
    Light,
    /// Minimalist ghost styling without default borders.
    Subtle,
}

/// Shared context provided by a parent [`ChipGroup`].
#[derive(Clone, Copy, PartialEq)]
pub struct ChipGroupContext {
    /// Reactive list of currently selected string values.
    pub values: Signal<Vec<String>>,
    /// Whether multiple chips can be selected concurrently.
    pub multiple: bool,
    /// Shared size modifier inherited by child chips.
    pub size: Option<ControlSize>,
    /// Shared variant inherited by child chips.
    pub variant: Option<ChipVariant>,
    /// Callback forwarded from the group.
    pub on_change: Option<EventHandler<Vec<String>>>,
}

/// Properties for configuring an individual [`Chip`] component.
#[derive(Props, Clone, PartialEq)]
pub struct ChipProps {
    /// Reactive boolean signal binding the checked state of a standalone chip.
    #[props(default)]
    pub checked: Option<Signal<bool>>,
    /// Default checked state if no reactive signal is provided.
    #[props(default = false)]
    pub default_checked: bool,
    /// String identifier for this chip when used within a [`ChipGroup`].
    #[props(default)]
    pub value: Option<String>,
    /// Visual style variant (Filled, Outline, Light, Subtle).
    #[props(default = ChipVariant::Filled)]
    pub variant: ChipVariant,
    /// Size scaling (Sm, Md, Lg).
    #[props(default = ControlSize::Md)]
    pub size: ControlSize,
    /// Optional leading icon (SVG native).
    #[props(default)]
    pub icon: Option<IconKind>,
    /// Whether to show a checkmark icon when the chip is active.
    #[props(default = true)]
    pub show_check: bool,
    /// Whether the chip is disabled and non-interactive.
    #[props(default = false)]
    pub disabled: bool,
    /// Optional additional CSS classes.
    #[props(default)]
    pub class: Option<String>,
    /// Children elements (usually label text).
    pub children: Element,
    /// Click event handler.
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    /// Toggle state change event handler.
    #[props(default)]
    pub on_toggle: Option<EventHandler<bool>>,
}

/// Properties for configuring a [`ChipGroup`] container.
#[derive(Props, Clone, PartialEq)]
pub struct ChipGroupProps {
    /// Reactive signal containing the list of currently selected chip values.
    pub value: Signal<Vec<String>>,
    /// Whether multiple chips can be selected simultaneously. If false, single-select mode.
    #[props(default = false)]
    pub multiple: bool,
    /// Shared size applied to all child chips if not overridden.
    #[props(default)]
    pub size: Option<ControlSize>,
    /// Shared variant applied to all child chips if not overridden.
    #[props(default)]
    pub variant: Option<ChipVariant>,
    /// Optional additional CSS classes for the group container.
    #[props(default)]
    pub class: Option<String>,
    /// Group children containing [`Chip`] items.
    pub children: Element,
    /// Callback triggered whenever the selection changes.
    #[props(default)]
    pub on_change: Option<EventHandler<Vec<String>>>,
}

/// Group container managing single or multi-selection across child [`Chip`] items.
#[component]
pub fn ChipGroup(props: ChipGroupProps) -> Element {
    use_context_provider(|| ChipGroupContext {
        values: props.value,
        multiple: props.multiple,
        size: props.size,
        variant: props.variant,
        on_change: props.on_change,
    });

    let container_class = cx([
        "sd-chip-group",
        props.class.as_deref().unwrap_or(""),
    ]);

    rsx! {
        div {
            class: "{container_class}",
            role: if props.multiple { "group" } else { "radiogroup" },
            {props.children}
        }
    }
}

/// Tactile filter and selection chip component with micro-glow,
/// customizable icon, check animation, and single/multiple group binding.
#[component]
pub fn Chip(props: ChipProps) -> Element {
    let group_ctx = try_use_context::<ChipGroupContext>();
    let mut local_checked = use_signal(|| props.default_checked);

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let class_modifiers = {
        let extra = props.class.as_deref().unwrap_or("");
        theme.read().resolve_class_modifiers("chip", extra)
    };
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let effective_size = group_ctx
        .and_then(|c| c.size)
        .unwrap_or(props.size);

    let effective_variant = group_ctx
        .and_then(|c| c.variant)
        .unwrap_or(props.variant);

    let is_checked = if let (Some(ctx), Some(val)) = (group_ctx, &props.value) {
        ctx.values.read().contains(val)
    } else if let Some(sig) = props.checked {
        *sig.read()
    } else {
        *local_checked.read()
    };

    let size_modifier = match effective_size {
        ControlSize::Sm => "sd-chip--sm",
        ControlSize::Md => "sd-chip--md",
        ControlSize::Lg => "sd-chip--lg",
    };

    let variant_modifier = match effective_variant {
        ChipVariant::Filled => "sd-chip--filled",
        ChipVariant::Outline => "sd-chip--outline",
        ChipVariant::Light => "sd-chip--light",
        ChipVariant::Subtle => "sd-chip--subtle",
    };

    let checked_modifier = if is_checked { "sd-chip--checked" } else { "" };

    let chip_class = cx([
        "sd-chip",
        size_modifier,
        variant_modifier,
        checked_modifier,
        class_modifiers.as_str(),
        props.class.as_deref().unwrap_or(""),
    ]);

    let icon_size = match effective_size {
        ControlSize::Sm => 12,
        ControlSize::Md => 14,
        ControlSize::Lg => 16,
    };

    let val_opt = props.value.clone();
    let on_click_prop = props.on_click;
    let on_toggle_prop = props.on_toggle;
    let checked_sig = props.checked;

    let handle_click = move |evt: MouseEvent| {
        if props.disabled {
            return;
        }

        if let Some(handler) = on_click_prop.as_ref() {
            handler.call(evt);
        }

        if let (Some(mut ctx), Some(val)) = (group_ctx, &val_opt) {
            let mut list = ctx.values.read().clone();
            if ctx.multiple {
                if let Some(pos) = list.iter().position(|x| x == val) {
                    list.remove(pos);
                } else {
                    list.push(val.clone());
                }
            } else {
                list = vec![val.clone()];
            }
            ctx.values.set(list.clone());
            if let Some(handler) = ctx.on_change.as_ref() {
                handler.call(list);
            }
        } else if let Some(mut sig) = checked_sig {
            let next = !is_checked;
            sig.set(next);
            if let Some(handler) = on_toggle_prop.as_ref() {
                handler.call(next);
            }
        } else {
            let next = !is_checked;
            local_checked.set(next);
            if let Some(handler) = on_toggle_prop.as_ref() {
                handler.call(next);
            }
        }
    };

    let role = if let Some(ctx) = group_ctx {
        if ctx.multiple { "checkbox" } else { "radio" }
    } else {
        "checkbox"
    };

    rsx! {
        button {
            r#type: "button",
            class: "{chip_class}",
            role: "{role}",
            aria_checked: if is_checked { "true" } else { "false" },
            aria_disabled: if props.disabled { "true" } else { "false" },
            disabled: props.disabled,
            onclick: handle_click,

            // Leading icon or animated checkmark
            if is_checked && props.show_check {
                span { class: "sd-chip__icon sd-chip__icon--check",
                    Icon { kind: IconKind::Check, size: icon_size }
                }
            } else if let Some(ic) = props.icon {
                span { class: "sd-chip__icon",
                    Icon { kind: ic, size: icon_size }
                }
            }

            span { class: "sd-chip__label",
                {props.children}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip_variant_default() {
        assert_eq!(ChipVariant::default(), ChipVariant::Filled);
    }

    #[test]
    fn test_chip_control_size_modifiers() {
        assert_eq!(ControlSize::Sm.class_modifier(), "--sm");
        assert_eq!(ControlSize::Md.class_modifier(), "--md");
        assert_eq!(ControlSize::Lg.class_modifier(), "--lg");
    }

    #[test]
    fn test_chip_group_selection_logic() {
        // Multi-select toggle
        let mut selected = vec!["a".to_string(), "b".to_string()];
        let item = "a".to_string();
        if let Some(pos) = selected.iter().position(|x| x == &item) {
            selected.remove(pos);
        } else {
            selected.push(item);
        }
        assert_eq!(selected, vec!["b".to_string()]);

        // Toggle back in
        let item_a = "a".to_string();
        if let Some(pos) = selected.iter().position(|x| x == &item_a) {
            selected.remove(pos);
        } else {
            selected.push(item_a);
        }
        assert_eq!(selected, vec!["b".to_string(), "a".to_string()]);

        // Single-select
        let single_item = "c".to_string();
        let single_selected = vec![single_item.clone()];
        assert_eq!(single_selected, vec!["c".to_string()]);
    }
}

