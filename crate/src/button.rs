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
    pub fn classes(&self) -> &'static str {
        match self {
            ButtonVariant::Primary => {
                "bg-purple-600 hover:bg-purple-500 active:bg-purple-700 text-white shadow-md border border-purple-400/40 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.2)]"
            }
            ButtonVariant::Secondary => {
                "bg-[#121218] hover:bg-[#1a1a24] active:bg-[#0e0e14] text-zinc-200 border border-white/[0.08] hover:border-white/20 hover:text-white shadow-sm shadow-[inset_0_1px_0_0_rgba(255,255,255,0.05)]"
            }
            ButtonVariant::Outline => {
                "bg-transparent hover:bg-purple-950/30 text-purple-300 border border-purple-500/30 hover:border-purple-400/60 hover:text-purple-100"
            }
            ButtonVariant::Ghost => {
                "bg-transparent hover:bg-white/[0.06] text-zinc-300 hover:text-white"
            }
            ButtonVariant::Danger => {
                "bg-rose-600 hover:bg-rose-500 active:bg-rose-700 text-white shadow-md border border-rose-400/40 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.2)]"
            }
            ButtonVariant::Success => {
                "bg-emerald-600 hover:bg-emerald-500 active:bg-emerald-700 text-white shadow-md border border-emerald-400/40 shadow-[inset_0_1px_0_0_rgba(255,255,255,0.2)]"
            }
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
    pub fn classes(&self) -> &'static str {
        match self {
            ButtonSize::Xs => "px-2 py-0.5 text-[11px] gap-1.5",
            ButtonSize::Sm => "px-3 py-1 text-xs gap-2",
            ButtonSize::Md => "px-4 py-2 text-xs sm:text-sm gap-2",
            ButtonSize::Lg => "px-5 py-2.5 text-sm sm:text-base gap-2.5 font-semibold",
            ButtonSize::Xl => "px-6 py-3.5 text-base sm:text-lg gap-3 font-semibold",
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
    let base = "inline-flex items-center justify-center font-medium transition-all duration-150 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none select-none active:scale-[0.98]";

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let variant_classes = theme().button_variant_classes(variant).to_string();
    #[cfg(feature = "theme")]
    let radius_class = match size {
        ButtonSize::Xs | ButtonSize::Sm => theme().radius_scale.sm(),
        ButtonSize::Md => theme().radius_scale.md(),
        ButtonSize::Lg | ButtonSize::Xl => theme().radius_scale.lg(),
    };
    #[cfg(feature = "theme")]
    let glow_class = match glow {
        Some(true) => theme().glow_button.clone(),
        Some(false) => String::new(),
        None => {
            if variant == ButtonVariant::Primary && theme().glow_mode != crate::theme::GlowMode::Off
            {
                theme().glow_button.clone()
            } else {
                String::new()
            }
        }
    };

    #[cfg(not(feature = "theme"))]
    let variant_classes = variant.classes();
    #[cfg(not(feature = "theme"))]
    let radius_class = match size {
        ButtonSize::Xs | ButtonSize::Sm => "rounded-md",
        ButtonSize::Md => "rounded-lg",
        ButtonSize::Lg | ButtonSize::Xl => "rounded-xl",
    };
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow == Some(true) || (glow.is_none() && variant == ButtonVariant::Primary)
    {
        "shadow-[0_0_15px_rgba(147,51,234,0.35)]"
    } else {
        ""
    };

    let scss_variant = match variant {
        ButtonVariant::Primary => "sd-btn--primary",
        ButtonVariant::Secondary => "sd-btn--secondary",
        ButtonVariant::Outline => "sd-btn--outline",
        ButtonVariant::Ghost => "sd-btn--ghost",
        ButtonVariant::Danger => "sd-btn--danger",
        ButtonVariant::Success => "sd-btn--success",
    };
    let scss_size = match size {
        ButtonSize::Xs => "sd-btn--xs",
        ButtonSize::Sm => "sd-btn--sm",
        ButtonSize::Md => "sd-btn--md",
        ButtonSize::Lg => "sd-btn--lg",
        ButtonSize::Xl => "sd-btn--xl",
    };
    let scss_full = if full_width { "sd-btn--full" } else { "" };
    let size_classes = size.classes();
    let width_class = if full_width { "w-full" } else { "" };
    let is_disabled = disabled || loading;

    rsx! {
        button {
            r#type: "button",
            class: "sd-btn {scss_variant} {scss_size} {scss_full} {base} {variant_classes} {size_classes} {radius_class} {glow_class} {width_class} {class}",
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
