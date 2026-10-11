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
    pub fn bubble_class(&self) -> &'static str {
        match self {
            TooltipPosition::Top => "sd-tooltip__bubble--top",
            TooltipPosition::Bottom => "sd-tooltip__bubble--bottom",
            TooltipPosition::Left => "sd-tooltip__bubble--left",
            TooltipPosition::Right => "sd-tooltip__bubble--right",
        }
    }
}

#[component]
pub fn Tooltip(
    content: String,
    #[props(default)] position: TooltipPosition,
    #[props(default)] glow: Option<bool>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
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

    rsx! {
        div {
            class: "sd-tooltip {class_modifiers} {class}",
            {children}
            div {
                class: "sd-tooltip__bubble {base_override} {position_class} {glow_class}",
                "{content}"
            }
        }
    }
}
