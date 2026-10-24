use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Position of the coachmark relative to the target element.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CoachmarkPosition {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

/// Guided onboarding bubble with step-by-step progress and actions.
#[component]
pub fn Coachmark(
    /// Current step number (1-indexed).
    step: usize,
    /// Total number of steps in the tour.
    total_steps: usize,
    /// Step title.
    title: String,
    /// Optional headline or subheader.
    #[props(default)]
    headline: Option<String>,
    /// Optional target DOM element ID being highlighted.
    #[props(default)]
    target_id: Option<String>,
    /// Step position orientation. Defaults to Bottom.
    #[props(default)]
    position: CoachmarkPosition,
    /// Callback triggered when "Successivo" / "Next" is pressed.
    #[props(default)]
    on_next: Option<EventHandler<()>>,
    /// Callback triggered when "Precedente" / "Previous" is pressed.
    #[props(default)]
    on_prev: Option<EventHandler<()>>,
    /// Callback triggered when the tour is dismissed or closed.
    #[props(default)]
    on_dismiss: Option<EventHandler<()>>,
    /// Perimetric optical glow.
    #[props(default)]
    glow: Option<bool>,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Explanatory content or description.
    children: Element,
) -> Element {
    let locale = crate::locale::use_locale();
    let is_first = step <= 1;
    let is_last = step >= total_steps;

    let glow_class = if glow == Some(true) {
        "sd-coachmark--glow"
    } else {
        ""
    };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("coachmark").unwrap_or("");
    #[cfg(not(feature = "theme"))]
    let base_override = "";

    rsx! {
        div {
            class: "sd-coachmark {glow_class} {base_override} {class}",
            div { class: "sd-coachmark-header",
                span { class: "sd-coachmark-step-badge",
                    "{crate::locale::Locale::fmt(&locale.coachmark_step, &[(\"step\", &step.to_string()), (\"total\", &total_steps.to_string())])}"
                }
                if let Some(h) = on_dismiss {
                    button {
                        r#type: "button",
                        class: "sd-coachmark-close-btn",
                        title: "{locale.coachmark_close}",
                        onclick: move |_| h.call(()),
                        Icon { kind: IconKind::X, size: 14 }
                    }
                }
            }

            h4 { class: "sd-coachmark-title", "{title}" }
            if let Some(head) = headline {
                p { class: "sd-coachmark-headline", "{head}" }
            }

            div { class: "sd-coachmark-content",
                {children}
            }

            div { class: "sd-coachmark-footer",
                div { class: "sd-coachmark-dots",
                    for i in 1..=total_steps {
                        div {
                            key: "{i}",
                            class: if i == step { "sd-coachmark-dot sd-coachmark-dot--active" } else { "sd-coachmark-dot" },
                        }
                    }
                }

                div { class: "sd-coachmark-buttons",
                    if !is_first {
                        if let Some(prev_handler) = on_prev {
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Sm,
                                onclick: move |_| prev_handler.call(()),
                                "{locale.back}"
                            }
                        }
                    }
                    if let Some(next_handler) = on_next {
                        Button {
                            variant: ButtonVariant::Primary,
                            size: ButtonSize::Sm,
                            onclick: move |_| next_handler.call(()),
                            if is_last { "{locale.finish}" } else { "{locale.forward}" }
                        }
                    }
                }
            }
        }
    }
}

/// Convenience alias for [`Coachmark`].
#[component]
pub fn TeachingBubble(
    step: usize,
    total_steps: usize,
    title: String,
    #[props(default)] headline: Option<String>,
    #[props(default)] target_id: Option<String>,
    #[props(default)] position: CoachmarkPosition,
    #[props(default)] on_next: Option<EventHandler<()>>,
    #[props(default)] on_prev: Option<EventHandler<()>>,
    #[props(default)] on_dismiss: Option<EventHandler<()>>,
    #[props(default)] glow: Option<bool>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        Coachmark {
            step,
            total_steps,
            title,
            headline,
            target_id,
            position,
            on_next,
            on_prev,
            on_dismiss,
            glow,
            class,
            {children}
        }
    }
}
