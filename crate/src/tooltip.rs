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
    pub fn bubble_classes(&self) -> &'static str {
        match self {
            TooltipPosition::Top => "bottom-full left-1/2 -translate-x-1/2 mb-2",
            TooltipPosition::Bottom => "top-full left-1/2 -translate-x-1/2 mt-2",
            TooltipPosition::Left => "right-full top-1/2 -translate-y-1/2 mr-2",
            TooltipPosition::Right => "left-full top-1/2 -translate-y-1/2 ml-2",
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
    let position_classes = position.bubble_classes();

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => theme().glow.clone(),
        Some(false) => String::new(),
        None => {
            if theme().glow_mode != crate::theme::GlowMode::Off {
                theme().glow.clone()
            } else {
                String::new()
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) {
        "shadow-[0_0_15px_rgba(147,51,234,0.3)]"
    } else {
        ""
    };

    rsx! {
        div {
            class: "relative group inline-flex {class}",
            {children}
            div {
                class: "absolute z-[100] px-2.5 py-1 text-[11px] font-medium text-zinc-200 bg-[#0e0e14] border border-white/10 rounded-md shadow-2xl whitespace-nowrap opacity-0 pointer-events-none group-hover:opacity-100 transition-all duration-150 specular-top select-none {position_classes} {glow_class}",
                "{content}"
            }
        }
    }
}
