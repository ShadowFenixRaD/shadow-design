use dioxus::prelude::*;

/// 12 position alignments supported by Popover.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PopoverPosition {
    Top,
    TopStart,
    TopEnd,
    #[default]
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
    Right,
    RightStart,
    RightEnd,
}

impl PopoverPosition {
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Top => "sd-popover-dropdown--top",
            Self::TopStart => "sd-popover-dropdown--top-start",
            Self::TopEnd => "sd-popover-dropdown--top-end",
            Self::Bottom => "sd-popover-dropdown--bottom",
            Self::BottomStart => "sd-popover-dropdown--bottom-start",
            Self::BottomEnd => "sd-popover-dropdown--bottom-end",
            Self::Left => "sd-popover-dropdown--left",
            Self::LeftStart => "sd-popover-dropdown--left-start",
            Self::LeftEnd => "sd-popover-dropdown--left-end",
            Self::Right => "sd-popover-dropdown--right",
            Self::RightStart => "sd-popover-dropdown--right-start",
            Self::RightEnd => "sd-popover-dropdown--right-end",
        }
    }
}

/// Context provided to child components of [`Popover`].
#[derive(Clone, Copy, PartialEq)]
pub struct PopoverContext {
    pub opened: Signal<bool>,
    pub position: Signal<PopoverPosition>,
    pub with_arrow: Signal<bool>,
    pub close_on_click_outside: bool,
    pub glow: Option<bool>,
}

/// Popover container anchored to any trigger element.
#[component]
pub fn Popover(
    /// Signal controlling the open/closed state of the popover.
    opened: Signal<bool>,
    /// Relative position of the popover panel (12 positions). Defaults to Bottom.
    #[props(default)]
    position: PopoverPosition,
    /// Whether to display the indicator arrow. Defaults to true.
    #[props(default = true)]
    with_arrow: bool,
    /// Whether pressing the Escape key dismisses the popover. Defaults to true.
    #[props(default = true)]
    close_on_escape: bool,
    /// Whether clicking outside the popover dismisses it. Defaults to true.
    #[props(default = true)]
    close_on_click_outside: bool,
    /// Perimetric optical glow coordinated with the active theme.
    #[props(default)]
    glow: Option<bool>,
    /// Additional CSS or SCSS classes for the container.
    #[props(default)]
    class: String,
    /// Optional unique identifier.
    #[props(default)]
    id: Option<String>,
    /// Children: should contain [`PopoverTrigger`] and [`PopoverDropdown`].
    children: Element,
) -> Element {
    let mut position_sig = use_signal(|| position);
    use_effect(use_reactive(&position, move |pos| {
        position_sig.set(pos);
    }));
    let mut arrow_sig = use_signal(|| with_arrow);
    use_effect(use_reactive(&with_arrow, move |arr| {
        arrow_sig.set(arr);
    }));

    use_context_provider(|| PopoverContext {
        opened,
        position: position_sig,
        with_arrow: arrow_sig,
        close_on_click_outside,
        glow,
    });

    let handle_keydown = move |e: KeyboardEvent| {
        if close_on_escape && e.key() == Key::Escape && *opened.read() {
            *opened.write() = false;
        }
    };

    rsx! {
        div {
            class: "sd-popover {class}",
            id: id,
            tabindex: "-1",
            onkeydown: handle_keydown,
            {children}
        }
    }
}

/// Trigger element inside [`Popover`] that toggles the popover visibility when clicked.
#[component]
pub fn PopoverTrigger(
    /// Class name for the trigger wrapper.
    #[props(default)]
    class: String,
    /// Trigger content (button, avatar, icon, etc.).
    children: Element,
) -> Element {
    let mut ctx = use_context::<PopoverContext>();

    rsx! {
        div {
            class: "sd-popover-trigger inline-flex cursor-pointer {class}",
            "aria-haspopup": "dialog",
            "aria-expanded": "{*ctx.opened.read()}",
            onclick: move |_| {
                let next = !*ctx.opened.read();
                *ctx.opened.write() = next;
            },
            {children}
        }
    }
}

/// Floating content panel rendered when [`Popover`] is open.
#[component]
pub fn PopoverDropdown(
    /// Additional CSS or SCSS classes for the dropdown panel.
    #[props(default)]
    class: String,
    /// Perimetric optical glow.
    #[props(default)]
    glow: Option<bool>,
    /// Contents of the popover dropdown.
    children: Element,
) -> Element {
    let mut ctx = use_context::<PopoverContext>();
    let is_open = *ctx.opened.read();

    if !is_open {
        return rsx! {};
    }

    let pos_class = ctx.position.read().class_name();
    let effective_glow = glow.or(ctx.glow);
    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("popover", effective_glow);
    #[cfg(not(feature = "theme"))]
    let glow_class = if effective_glow == Some(true) {
        "sd-popover-dropdown--glow"
    } else {
        ""
    };
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("popover").unwrap_or("");
    #[cfg(not(feature = "theme"))]
    let base_override = "";

    rsx! {
        if ctx.close_on_click_outside {
            div {
                class: "sd-popover-backdrop",
                onclick: move |e| {
                    e.stop_propagation();
                    *ctx.opened.write() = false;
                },
            }
        }
        div {
            role: "dialog",
            class: "sd-popover-dropdown {pos_class} {glow_class} {base_override} {class}",
            onclick: move |e| e.stop_propagation(),
            if *ctx.with_arrow.read() {
                div { class: "sd-popover-arrow" }
            }
            {children}
        }
    }
}
