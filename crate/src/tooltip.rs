use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

impl TooltipPosition {
    #[must_use]
    pub fn bubble_class(&self) -> &'static str {
        match self {
            TooltipPosition::Top => "sd-tooltip__bubble--top",
            TooltipPosition::Bottom => "sd-tooltip__bubble--bottom",
            TooltipPosition::Left => "sd-tooltip__bubble--left",
            TooltipPosition::Right => "sd-tooltip__bubble--right",
        }
    }
}

/// Accessible tooltip component displaying supplementary context on hover and focus.
#[component]
pub fn Tooltip(
    /// Text content displayed in the tooltip bubble.
    content: String,
    /// Optional unique identifier for the tooltip.
    #[props(default)]
    id: Option<String>,
    /// Optional ID of an element explicitly described by this tooltip.
    #[props(default)]
    describes_id: Option<String>,
    /// Relative placement of the tooltip bubble.
    #[props(default)]
    position: TooltipPosition,
    /// Perimetric optical glow coordinated with the active theme.
    #[props(default)]
    glow: Option<bool>,
    /// Additional CSS or SCSS classes.
    #[props(default)]
    class: String,
    /// Interactive element wrapped by the tooltip.
    children: Element,
) -> Element {
    let mut is_visible = use_signal(|| false);
    let mut is_dismissed = use_signal(|| false);

    let position_class = position.bubble_class();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("tooltip").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("tooltip", &class);
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("tooltip", glow);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) {
        "sd-tooltip__bubble--glow".to_string()
    } else {
        String::new()
    };

    let tip_id = crate::a11y::use_unique_id("sd-tooltip", id);
    let describedby_id = describes_id.unwrap_or_else(|| tip_id.clone());

    let visible_class = if is_visible() && !is_dismissed() {
        "sd-tooltip--visible"
    } else if is_dismissed() {
        "sd-tooltip--dismissed"
    } else {
        ""
    };

    rsx! {
        div {
            class: "sd-tooltip {visible_class} {class_modifiers} {class}",
            onmouseenter: move |_| {
                is_visible.set(true);
                is_dismissed.set(false);
            },
            onmouseleave: move |_| {
                is_visible.set(false);
                is_dismissed.set(false);
            },
            onfocusin: move |_| {
                is_visible.set(true);
                is_dismissed.set(false);
            },
            onfocusout: move |_| {
                is_visible.set(false);
                is_dismissed.set(false);
            },
            onkeydown: move |e: KeyboardEvent| {
                if e.key() == Key::Escape {
                    is_dismissed.set(true);
                    is_visible.set(false);
                }
            },
            span {
                class: "sd-tooltip__trigger",
                "aria-describedby": "{describedby_id}",
                {children}
            }
            div {
                role: "tooltip",
                id: "{tip_id}",
                class: "sd-tooltip__bubble {base_override} {position_class} {glow_class}",
                "{content}"
            }
        }
    }
}
