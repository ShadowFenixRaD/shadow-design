use dioxus::prelude::*;

async fn sleep_ms(ms: u64) {
    #[cfg(target_arch = "wasm32")]
    {
        gloo_timers::future::TimeoutFuture::new(ms as u32).await;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
    }
}

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
    pub position: Signal<HoverCardPosition>,
    pub width: Signal<Option<u32>>,
    pub is_over_trigger: Signal<bool>,
    pub is_over_dropdown: Signal<bool>,
    pub open_delay: u32,
    pub close_delay: u32,
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
    let mut pos_sig = use_signal(|| position);
    let mut width_sig = use_signal(|| width);
    let is_over_trigger = use_signal(|| false);
    let is_over_dropdown = use_signal(|| false);

    use_effect(move || {
        pos_sig.set(position);
        width_sig.set(width);
    });

    use_context_provider(|| HoverCardContext {
        is_open,
        position: pos_sig,
        width: width_sig,
        is_over_trigger,
        is_over_dropdown,
        open_delay,
        close_delay,
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
            class: "sd-hovercard-trigger {class}",
            onmouseenter: move |_| {
                ctx.is_over_trigger.set(true);
                let open_delay = ctx.open_delay;
                spawn(async move {
                    if open_delay > 0 {
                        sleep_ms(open_delay as u64).await;
                    }
                    if *ctx.is_over_trigger.read() {
                        ctx.is_open.set(true);
                    }
                });
            },
            onmouseleave: move |_| {
                ctx.is_over_trigger.set(false);
                let close_delay = ctx.close_delay;
                spawn(async move {
                    sleep_ms(close_delay as u64).await;
                    if !*ctx.is_over_trigger.read() && !*ctx.is_over_dropdown.read() {
                        ctx.is_open.set(false);
                    }
                });
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
    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    let is_open = *ctx.is_open.read();

    if !is_open {
        return rsx! {};
    }

    let pos_class = ctx.position.read().class_name();

    let style = if let Some(w) = *ctx.width.read() {
        format!("width: {w}px;")
    } else {
        String::new()
    };

    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("hover_card").unwrap_or("");
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("hover_card", glow);
    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) { "sd-hover-card--glow" } else { "" };

    rsx! {
        div {
            role: "dialog",
            class: "sd-hovercard-dropdown {pos_class} {glow_class} {base_override} {class}",
            style: "{style}",
            onmouseenter: move |_| {
                ctx.is_over_dropdown.set(true);
            },
            onmouseleave: move |_| {
                ctx.is_over_dropdown.set(false);
                let close_delay = ctx.close_delay;
                spawn(async move {
                    sleep_ms(close_delay as u64).await;
                    if !*ctx.is_over_trigger.read() && !*ctx.is_over_dropdown.read() {
                        ctx.is_open.set(false);
                    }
                });
            },
            {children}
        }
    }
}
