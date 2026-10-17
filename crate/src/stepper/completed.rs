use dioxus::prelude::*;
use crate::stepper::StepperCtx;

/// Proprietà del componente di completamento [`StepperCompleted`].
#[derive(Props, Clone, PartialEq)]
pub struct StepperCompletedProps {
    /// Mostra forzatamente il completamento (override).
    #[props(default)]
    pub show: Option<bool>,
    /// Indice minimo dello step attivo affinché il completamento sia visibile.
    #[props(default)]
    pub completed_step: Option<usize>,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Contenuto renderizzato alla conclusione di tutti gli step del wizard.
    pub children: Element,
}

/// Contenitore per lo stato conclusivo di uno [`Stepper`].
#[component]
pub fn StepperCompleted(props: StepperCompletedProps) -> Element {
    let ctx = try_use_context::<StepperCtx>();
    let active_idx = ctx.as_ref().map(|c| c.active).unwrap_or(0);

    let is_visible = if let Some(show) = props.show {
        show
    } else if let Some(target) = props.completed_step {
        active_idx >= target
    } else {
        // Se non specificato, visibile se l'indice attivo è >= 3 o se specificato dal genitore
        true
    };

    if is_visible {
        rsx! {
            div { class: "sd-stepper-completed {props.class}",
                {props.children}
            }
        }
    } else {
        rsx! {}
    }
}
