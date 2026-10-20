use dioxus::prelude::*;
use crate::icon::{Icon, IconKind, RenderSvgIcon, SvgIcon};
use crate::stepper::{StepperCtx, StepperOrientation};

/// Stato di avanzamento di un singolo step.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum StepState {
    /// Step futuro non ancora raggiunto.
    #[default]
    Upcoming,
    /// Step corrente in corso di completamento.
    Active,
    /// Step completato con successo.
    Completed,
    /// Step che presenta un errore di validazione o blocco.
    Error,
}

impl StepState {
    #[must_use]
    pub fn as_class(&self) -> &'static str {
        match self {
            StepState::Upcoming => "sd-stepper-step--upcoming",
            StepState::Active => "sd-stepper-step--active",
            StepState::Completed => "sd-stepper-step--completed",
            StepState::Error => "sd-stepper-step--error",
        }
    }
}

/// Proprietà del singolo step [`StepperStep`].
#[derive(Props, Clone, PartialEq)]
pub struct StepperStepProps {
    /// Indice numerico dello step (0-based).
    #[props(default)]
    pub step: usize,
    /// Etichetta testuale principale dello step.
    pub label: String,
    /// Descrizione secondaria opzionale posizionata sotto l'etichetta.
    #[props(default)]
    pub description: Option<String>,
    /// Icona personalizzata per lo step. Se assente, visualizza il numero d'ordine (step + 1).
    #[props(default)]
    pub icon: Option<SvgIcon>,
    /// Icona personalizzata quando lo step è completato (default: Check).
    #[props(default)]
    pub completed_icon: Option<SvgIcon>,
    /// Stato esplicito (override del calcolo automatico basato su active index).
    #[props(default)]
    pub state: Option<StepState>,
    /// Disabilita il click o l'interazione su questo step.
    #[props(default)]
    pub disabled: bool,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Contenuto associato a questo step (renderizzato quando lo step è attivo).
    #[props(default)]
    pub children: Option<Element>,
}

/// Singolo step all'interno del componente [`Stepper`].
#[component]
pub fn StepperStep(props: StepperStepProps) -> Element {
    let ctx_sig = try_use_context::<Signal<StepperCtx>>();
    let active_idx = ctx_sig.as_ref().map(|c| c.read().active).unwrap_or(0);
    let orientation = ctx_sig.as_ref().map(|c| c.read().orientation).unwrap_or(StepperOrientation::Horizontal);
    let allow_click = ctx_sig.as_ref().map(|c| c.read().allow_click_completed).unwrap_or(true);

    let current_state = props.state.unwrap_or(if props.step < active_idx {
        StepState::Completed
    } else if props.step == active_idx {
        StepState::Active
    } else {
        StepState::Upcoming
    });

    let is_completed = current_state == StepState::Completed;
    let is_active = current_state == StepState::Active;
    let is_clickable = !props.disabled && ((is_completed && allow_click) || is_active);

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("stepper_step").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("stepper_step", &props.class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let orientation_class = orientation.as_class();
    let state_class = current_state.as_class();
    let clickable_class = if is_clickable { "sd-stepper-step--clickable" } else { "" };
    let disabled_class = if props.disabled { "sd-stepper-step--disabled" } else { "" };

    // Rendering dell'icona o del numero nel cerchio
    let circle_content = match current_state {
        StepState::Completed => {
            if let Some(ref c_icon) = props.completed_icon {
                rsx! { RenderSvgIcon { icon: c_icon.clone(), size: 16 } }
            } else {
                rsx! { Icon { kind: IconKind::Check, size: 16 } }
            }
        }
        StepState::Error => {
            rsx! { Icon { kind: IconKind::AlertCircle, size: 16 } }
        }
        StepState::Active | StepState::Upcoming => {
            if let Some(ref ic) = props.icon {
                rsx! { RenderSvgIcon { icon: ic.clone(), size: 16 } }
            } else {
                let num = props.step + 1;
                rsx! { span { class: "sd-stepper-step__number", "{num}" } }
            }
        }
    };

    let step_idx = props.step;
    let on_click_step = move |_| {
        if is_clickable {
            if let Some(ref c) = ctx_sig {
                if let Some(ref handler) = c.read().on_change {
                    handler.call(step_idx);
                }
            }
        }
    };

    rsx! {
        div {
            class: "sd-stepper-step {orientation_class} {state_class} {clickable_class} {disabled_class} {base_override} {class_modifiers} {props.class}",
            button {
                r#type: "button",
                class: "sd-stepper-step__header",
                disabled: !is_clickable,
                "aria-current": if is_active { "step" } else { "" },
                onclick: on_click_step,
                div { class: "sd-stepper-step__circle",
                    {circle_content}
                }
                div { class: "sd-stepper-step__label-container",
                    span { class: "sd-stepper-step__label", "{props.label}" }
                    if let Some(ref desc) = props.description {
                        span { class: "sd-stepper-step__description", "{desc}" }
                    }
                }
            }
            // Linea connettiva tra step
            div { class: "sd-stepper-step__line", "aria-hidden": "true" }

            // Contenuto dello step (mostrato quando attivo)
            if is_active {
                if let Some(ref content) = props.children {
                    div { class: "sd-stepper-step__content",
                        {content}
                    }
                }
            }
        }
    }
}
