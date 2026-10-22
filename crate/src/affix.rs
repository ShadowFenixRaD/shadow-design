use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Screen corner placement for [`Affix`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum AffixPosition {
    #[default]
    BottomRight,
    BottomLeft,
    TopRight,
    TopLeft,
}

impl AffixPosition {
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::BottomRight => "sd-affix--bottom-right",
            Self::BottomLeft => "sd-affix--bottom-left",
            Self::TopRight => "sd-affix--top-right",
            Self::TopLeft => "sd-affix--top-left",
        }
    }
}

/// Fixed overlay container anchored to viewport coordinates with optional scroll threshold.
#[component]
pub fn Affix(
    /// Corner placement in the viewport. Defaults to BottomRight.
    #[props(default)]
    position: AffixPosition,
    /// Minimum scroll offset in pixels required before the affix fades in.
    #[props(default)]
    scroll_threshold: Option<u32>,
    /// Custom z-index. Defaults to 60.
    #[props(default = 60)]
    z_index: u32,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Affix content.
    children: Element,
) -> Element {
    let mut is_visible = use_signal(|| scroll_threshold.is_none());

    if let Some(threshold) = scroll_threshold {
        use_hook(move || {
            spawn(async move {
                let mut eval = document::eval(&format!(
                    r#"
                    const checkScroll = () => {{
                        const scrolled = window.scrollY || document.documentElement.scrollTop;
                        return scrolled >= {threshold};
                    }};
                    window.addEventListener('scroll', () => {{
                        dioxus.send(checkScroll());
                    }}, {{ passive: true }});
                    dioxus.send(checkScroll());
                    "#
                ));
                while let Ok(val) = eval.recv::<bool>().await {
                    is_visible.set(val);
                }
            });
        });
    }

    let pos_class = position.class_name();
    let visibility_class = if *is_visible.read() {
        "sd-affix--visible"
    } else {
        "sd-affix--hidden"
    };

    rsx! {
        div {
            class: "sd-affix {pos_class} {visibility_class} {class}",
            style: "z-index: {z_index};",
            {children}
        }
    }
}

/// Floating button to smoothly scroll the page back to top.
#[component]
pub fn BackToTop(
    /// Minimum scroll in pixels before the button appears. Defaults to 200px.
    #[props(default = 200)]
    scroll_threshold: u32,
    /// Optional text label next to the icon.
    #[props(default)]
    label: Option<String>,
    /// Icon displayed on the button. Defaults to [`IconKind::ChevronUp`].
    #[props(default = IconKind::ChevronUp)]
    icon: IconKind,
    /// Viewport corner position. Defaults to BottomRight.
    #[props(default)]
    position: AffixPosition,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
) -> Element {
    let handle_scroll_top = move |_| {
        let eval = document::eval("window.scrollTo({ top: 0, behavior: 'smooth' });");
        spawn(async move {
            let _ = eval.await;
        });
    };

    let icon_only_class = if label.is_none() {
        "sd-back-to-top--icon-only"
    } else {
        ""
    };

    rsx! {
        Affix {
            position: position,
            scroll_threshold: Some(scroll_threshold),
            class: class,
            button {
                r#type: "button",
                class: "sd-back-to-top {icon_only_class}",
                title: "Torna in cima",
                onclick: handle_scroll_top,
                Icon { kind: icon, size: 16 }
                if let Some(l) = label {
                    span { "{l}" }
                }
            }
        }
    }
}
