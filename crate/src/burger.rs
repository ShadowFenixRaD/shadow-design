use dioxus::prelude::*;

/// Predefined sizes for [`Burger`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BurgerSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl BurgerSize {
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Sm => "sd-burger--sm",
            Self::Md => "sd-burger--md",
            Self::Lg => "sd-burger--lg",
        }
    }
}

/// Interactive hamburger menu button that smoothly transitions to an 'X' icon when opened.
#[component]
pub fn Burger(
    /// Whether the burger is currently in the open / close-cross state.
    opened: Signal<bool>,
    /// Size of the burger toggle. Defaults to [`BurgerSize::Md`].
    #[props(default)]
    size: BurgerSize,
    /// Accessible label for screen readers. Defaults to "Toggle navigation".
    #[props(default = "Toggle navigation".to_string())]
    aria_label: String,
    /// Custom color hex or CSS string for the burger lines.
    #[props(default)]
    color: Option<String>,
    /// Click event handler. If provided, will be called alongside toggling the `opened` signal.
    #[props(default)]
    onclick: Option<EventHandler<MouseEvent>>,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
) -> Element {
    let is_open = *opened.read();
    let size_class = size.class_name();
    let open_class = if is_open { "sd-burger--open" } else { "" };

    let style = if let Some(ref c) = color {
        format!("--sd-burger-color: {c};")
    } else {
        String::new()
    };

    let handle_click = move |e: MouseEvent| {
        let mut sig = opened;
        let next = !*sig.read();
        sig.set(next);
        if let Some(h) = onclick {
            h.call(e);
        }
    };

    rsx! {
        button {
            r#type: "button",
            class: "sd-burger {size_class} {open_class} {class}",
            style: "{style}",
            "aria-label": "{aria_label}",
            "aria-expanded": "{is_open}",
            onclick: handle_click,
            span { class: "sd-burger-line sd-burger-line--top" }
            span { class: "sd-burger-line sd-burger-line--middle" }
            span { class: "sd-burger-line sd-burger-line--bottom" }
        }
    }
}
