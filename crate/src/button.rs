use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
    Success,
}

impl ButtonVariant {
    pub fn scss_class(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => "sd-btn--primary",
            ButtonVariant::Secondary => "sd-btn--secondary",
            ButtonVariant::Outline => "sd-btn--outline",
            ButtonVariant::Ghost => "sd-btn--ghost",
            ButtonVariant::Danger => "sd-btn--danger",
            ButtonVariant::Success => "sd-btn--success",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl ButtonSize {
    pub fn scss_class(&self) -> &'static str {
        match self {
            ButtonSize::Xs => "sd-btn--xs",
            ButtonSize::Sm => "sd-btn--sm",
            ButtonSize::Md => "sd-btn--md",
            ButtonSize::Lg => "sd-btn--lg",
            ButtonSize::Xl => "sd-btn--xl",
        }
    }
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] disabled: bool,
    #[props(default)] loading: bool,
    #[props(default)] full_width: bool,
    #[props(default)] glow: Option<bool>,
    #[props(default)] left_icon: Option<Element>,
    #[props(default)] right_icon: Option<Element>,
    #[props(default)] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let size_class = size.scss_class();
    let full_class = if full_width { "sd-btn--full" } else { "" };

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();

    #[cfg(feature = "theme")]
    let variant_class = if variant == ButtonVariant::Primary {
        match theme().primary_color {
            crate::theme::PrimaryColor::Violet => "sd-btn--primary",
            crate::theme::PrimaryColor::Obsidian => "sd-btn--primary-obsidian",
            crate::theme::PrimaryColor::Crimson => "sd-btn--primary-crimson",
            crate::theme::PrimaryColor::Cyan => "sd-btn--primary-cyan",
            crate::theme::PrimaryColor::Emerald => "sd-btn--primary-emerald",
        }
    } else {
        variant.scss_class()
    };

    #[cfg(feature = "theme")]
    let radius_class = match size {
        ButtonSize::Xs | ButtonSize::Sm => theme().radius_scale.sm(),
        ButtonSize::Md => theme().radius_scale.md(),
        ButtonSize::Lg | ButtonSize::Xl => theme().radius_scale.lg(),
    };

    #[cfg(feature = "theme")]
    let theme_val = theme();
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => theme_val.component_glow("button"),
        Some(false) => "",
        None => {
            if variant == ButtonVariant::Primary && theme_val.glow_mode != crate::theme::GlowMode::Off
            {
                theme_val.component_glow("button")
            } else {
                ""
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let variant_class = variant.scss_class();
    #[cfg(not(feature = "theme"))]
    let radius_class = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) || (glow.is_none() && variant == ButtonVariant::Primary)
    {
        "sd-btn--glow"
    } else {
        ""
    };

    let is_disabled = disabled || loading;

    rsx! {
        button {
            r#type: "button",
            class: "sd-btn {variant_class} {size_class} {full_class} {radius_class} {glow_class} {class}",
            disabled: is_disabled,
            onclick: move |evt| {
                if !is_disabled {
                    if let Some(handler) = onclick.as_ref() {
                        handler.call(evt);
                    }
                }
            },
            if loading {
                svg {
                    class: "animate-spin shrink-0 h-4 w-4 mr-1",
                    xmlns: "http://www.w3.org/2000/svg",
                    fill: "none",
                    view_box: "0 0 24 24",
                    circle {
                        class: "opacity-25",
                        cx: "12",
                        cy: "12",
                        r: "10",
                        stroke: "currentColor",
                        stroke_width: "4",
                    }
                    path {
                        class: "opacity-75",
                        fill: "currentColor",
                        d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
                    }
                }
            } else if let Some(licon) = left_icon {
                span { class: "shrink-0 inline-flex items-center", {licon} }
            }
            {children}
            if !loading {
                if let Some(ricon) = right_icon {
                    span { class: "shrink-0 inline-flex items-center", {ricon} }
                }
            }
        }
    }
}
