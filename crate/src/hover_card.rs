use dioxus::prelude::*;

/// Position placement for [`HoverCard`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum HoverCardPosition {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

impl HoverCardPosition {
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Bottom => "sd-hovercard-dropdown--bottom",
            Self::Top => "sd-hovercard-dropdown--top",
            Self::Left => "sd-hovercard-dropdown--left",
            Self::Right => "sd-hovercard-dropdown--right",
        }
    }
}

/// Context passed down to children of [`HoverCard`].
#[derive(Clone, Copy, PartialEq)]
pub struct HoverCardContext {
    pub is_open: Signal<bool>,
    pub position: HoverCardPosition,
    pub width: Option<u32>,
}

/// Rich contextual preview card triggered on hover.
#[component]
pub fn HoverCard(
    /// Delay before opening the card on hover in milliseconds. Defaults to 150ms.
    #[props(default = 150)]
    open_delay: u32,
    /// Delay before closing the card on mouseleave in milliseconds. Defaults to 200ms.
    #[props(default = 200)]
    close_delay: u32,
    /// Custom pixel width of the card.
    #[props(default)]
    width: Option<u32>,
    /// Placement position of the hover preview. Defaults to Bottom.
    #[props(default)]
    position: HoverCardPosition,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Children: should contain [`HoverCardTrigger`] and [`HoverCardDropdown`].
    children: Element,
) -> Element {
    let is_open = use_signal(|| false);

    use_context_provider(|| HoverCardContext {
        is_open,
        position,
        width,
    });

    rsx! {
        div {
            class: "sd-hovercard {class}",
            {children}
        }
    }
}

/// Trigger element inside [`HoverCard`] activating the preview when hovered.
#[component]
pub fn HoverCardTrigger(
    /// Class name for the trigger wrapper.
    #[props(default)]
    class: String,
    /// Trigger content.
    children: Element,
) -> Element {
    let mut ctx = use_context::<HoverCardContext>();

    rsx! {
        div {
            class: "sd-hovercard-trigger inline-flex cursor-pointer {class}",
            onmouseenter: move |_| {
                ctx.is_open.set(true);
            },
            onmouseleave: move |_| {
                ctx.is_open.set(false);
            },
            {children}
        }
    }
}

/// Dropdown panel rendered when [`HoverCard`] is hovered.
#[component]
pub fn HoverCardDropdown(
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Perimetric optical glow.
    #[props(default)]
    glow: Option<bool>,
    /// Dropdown contents.
    children: Element,
) -> Element {
    let mut ctx = use_context::<HoverCardContext>();
    let is_open = *ctx.is_open.read();

    if !is_open {
        return rsx! {};
    }

    let pos_class = ctx.position.class_name();
    let glow_class = if glow == Some(true) {
        "sd-hovercard-dropdown--glow"
    } else {
        ""
    };

    let style = if let Some(w) = ctx.width {
        format!("width: {w}px;")
    } else {
        String::new()
    };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("hover_card").unwrap_or("");
    #[cfg(not(feature = "theme"))]
    let base_override = "";

    rsx! {
        div {
            role: "dialog",
            class: "sd-hovercard-dropdown {pos_class} {glow_class} {base_override} {class}",
            style: "{style}",
            onmouseenter: move |_| {
                ctx.is_open.set(true);
            },
            onmouseleave: move |_| {
                ctx.is_open.set(false);
            },
            {children}
        }
    }
}
