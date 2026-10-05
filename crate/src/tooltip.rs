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
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => "sd-tooltip__bubble--glow",
        Some(false) => "",
        None => {
            if theme().glow_mode != crate::theme::GlowMode::Off {
                "sd-tooltip__bubble--glow"
            } else {
                ""
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) {
        "sd-tooltip__bubble--glow"
    } else {
        ""
    };

    rsx! {
        div {
            class: "sd-tooltip {class}",
            {children}
            div {
                class: "sd-tooltip__bubble {position_class} {glow_class}",
                "{content}"
            }
        }
    }
}
