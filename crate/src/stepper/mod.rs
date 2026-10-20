pub mod step;
pub mod completed;

pub use step::{StepperStep, StepperStepProps, StepState};
pub use completed::{StepperCompleted, StepperCompletedProps};

use dioxus::prelude::*;

/// Orientamento visivo del componente Stepper.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum StepperOrientation {
    /// Disposizione orizzontale classica con nodi in riga e linee di connessione trasversali.
    #[default]
    Horizontal,
    /// Disposizione verticale per wizard procedurali e flussi complessi.
    Vertical,
}

impl StepperOrientation {
    pub const ALL: &'static [StepperOrientation] = &[
        StepperOrientation::Horizontal,
        StepperOrientation::Vertical,
    ];

    #[must_use]
    pub fn as_class(&self) -> &'static str {
        match self {
            StepperOrientation::Horizontal => "sd-stepper--horizontal",
            StepperOrientation::Vertical => "sd-stepper--vertical",
        }
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            StepperOrientation::Horizontal => "Horizontal",
            StepperOrientation::Vertical => "Vertical",
        }
    }
}

/// Contesto condiviso tra il genitore [`Stepper`] e i singoli [`StepperStep`].
#[derive(Clone, PartialEq)]
pub struct StepperCtx {
    pub active: usize,
    pub orientation: StepperOrientation,
    pub allow_click_completed: bool,
    pub on_change: Option<EventHandler<usize>>,
}

/// Proprietà del componente contenitore [`Stepper`].
#[derive(Props, Clone, PartialEq)]
pub struct StepperProps {
    /// Indice numerico dello step correntemente attivo (0-based).
    #[props(default)]
    pub active: usize,
    /// Orientamento dei passi (Horizontal o Vertical). Default: `Horizontal`.
    #[props(default)]
    pub orientation: Option<StepperOrientation>,
    /// Consente all'utente di cliccare sugli step già completati per tornare indietro. Default: true.
    #[props(default = true)]
    pub allow_click_completed: bool,
    /// Callback invocata quando l'utente clicca su uno step abilitato.
    #[props(default)]
    pub on_change: Option<EventHandler<usize>>,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Elenco dei passi ([`StepperStep`]) ed eventuale stato conclusivo ([`StepperCompleted`]).
    pub children: Element,
}

/// Componente Stepper per guidare l'utente attraverso flussi multi-step o wizard.
/// Supporta orientamento orizzontale e verticale, nodi numerati o con icone SVG,
/// linee connettive reattive allo stato attivo, gestione errori e completamento.
#[component]
pub fn Stepper(props: StepperProps) -> Element {
    let orientation = props.orientation.unwrap_or_default();

    let mut ctx = use_context_provider(|| Signal::new(StepperCtx {
        active: props.active,
        orientation,
        allow_click_completed: props.allow_click_completed,
        on_change: props.on_change,
    }));

    let current = StepperCtx {
        active: props.active,
        orientation,
        allow_click_completed: props.allow_click_completed,
        on_change: props.on_change,
    };
    if *ctx.peek() != current {
        *ctx.write() = current;
    }

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("stepper").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("stepper", &props.class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let orientation_class = orientation.as_class();

    rsx! {
        div {
            class: "sd-stepper {orientation_class} {base_override} {class_modifiers} {props.class}",
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stepper_orientation_classes() {
        assert_eq!(StepperOrientation::Horizontal.as_class(), "sd-stepper--horizontal");
        assert_eq!(StepperOrientation::Vertical.as_class(), "sd-stepper--vertical");
    }

    #[test]
    fn test_step_state_classes() {
        assert_eq!(StepState::Upcoming.as_class(), "sd-stepper-step--upcoming");
        assert_eq!(StepState::Active.as_class(), "sd-stepper-step--active");
        assert_eq!(StepState::Completed.as_class(), "sd-stepper-step--completed");
        assert_eq!(StepState::Error.as_class(), "sd-stepper-step--error");
    }
}
