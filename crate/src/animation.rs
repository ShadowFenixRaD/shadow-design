use dioxus::prelude::*;

/// Varianti di animazione per componenti interattivi, toast, modali e dialoghi.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum AnimationVariant {
    #[default]
    FadeIn,
    FadeOut,
    SlideInRight,
    SlideInLeft,
    SlideInBottom,
    SlideInTop,
    ScaleIn,
    ModalEnter,
    ToastEnter,
    ToastExit,
    DropdownEnter,
    DrawerSlideLeft,
    DrawerSlideRight,
    PulseGlow,
    TactilePress,
    Spin,
    Indeterminate,
}

impl AnimationVariant {
    #[must_use]
    pub fn classes(&self) -> &'static str {
        match self {
            AnimationVariant::FadeIn => "animate-fade-in",
            AnimationVariant::FadeOut => "animate-fade-out",
            AnimationVariant::SlideInRight => "animate-slide-in-right",
            AnimationVariant::SlideInLeft => "animate-slide-in-left",
            AnimationVariant::SlideInBottom => "animate-slide-in-bottom",
            AnimationVariant::SlideInTop => "animate-slide-in-top",
            AnimationVariant::ScaleIn => "animate-scale-in",
            AnimationVariant::ModalEnter => "animate-modal-enter",
            AnimationVariant::ToastEnter => "animate-toast-enter",
            AnimationVariant::ToastExit => "animate-toast-exit",
            AnimationVariant::DropdownEnter => "animate-dropdown-enter",
            AnimationVariant::DrawerSlideLeft => "animate-drawer-slide-left",
            AnimationVariant::DrawerSlideRight => "animate-drawer-slide-right",
            AnimationVariant::PulseGlow => "animate-pulse-glow",
            AnimationVariant::TactilePress => "animate-tactile-press tactile-press select-none cursor-pointer",
            AnimationVariant::Spin => "animate-spin",
            AnimationVariant::Indeterminate => "animate-indeterminate",
        }
    }
}

/// Helper di transizione e animazione per Dioxus.
#[component]
pub fn Transition(
    #[props(default)] variant: AnimationVariant,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let anim_class = variant.classes();
    rsx! {
        div {
            class: "{anim_class} {class}",
            {children}
        }
    }
}

/// Costanti e classi standard di animazione esposte dalla libreria.
pub struct Animations;

impl Animations {
    pub const FADE_IN: &'static str = "animate-fade-in";
    pub const FADE_OUT: &'static str = "animate-fade-out";
    pub const SLIDE_IN_RIGHT: &'static str = "animate-slide-in-right";
    pub const SLIDE_IN_LEFT: &'static str = "animate-slide-in-left";
    pub const SLIDE_IN_BOTTOM: &'static str = "animate-slide-in-bottom";
    pub const SLIDE_IN_TOP: &'static str = "animate-slide-in-top";
    pub const SCALE_IN: &'static str = "animate-scale-in";
    pub const TOAST_ENTER: &'static str = "animate-toast-enter";
    pub const TOAST_EXIT: &'static str = "animate-toast-exit";
    pub const MODAL_ENTER: &'static str = "animate-modal-enter";
    pub const DRAWER_LEFT: &'static str = "animate-drawer-slide-left";
    pub const DRAWER_RIGHT: &'static str = "animate-drawer-slide-right";
    pub const DROPDOWN_ENTER: &'static str = "animate-dropdown-enter";
    pub const PULSE_GLOW: &'static str = "animate-pulse-glow";
    pub const SPIN: &'static str = "animate-spin";
    pub const INDETERMINATE: &'static str = "animate-indeterminate";
    pub const TACTILE: &'static str = "tactile-press";
}
