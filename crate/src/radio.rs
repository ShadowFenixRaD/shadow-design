use dioxus::prelude::*;

#[cfg(feature = "icon")]
use crate::icon::{Icon, IconKind};

/// Orientamento per il layout del gruppo radio.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum RadioOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl RadioOrientation {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            RadioOrientation::Vertical => "sd-radio-group--vertical",
            RadioOrientation::Horizontal => "sd-radio-group--horizontal",
        }
    }
}

/// Contesto condiviso fornito da `RadioGroup` ai componenti `Radio` e `RadioCard` figli.
#[derive(Clone, PartialEq)]
pub struct RadioContext {
    pub value: Signal<String>,
    pub name: Option<String>,
    pub disabled: bool,
}

/// Raggruppamento semantico per pulsanti radio mutualmente esclusivi.
#[component]
pub fn RadioGroup(
    value: Signal<String>,
    #[props(default)] name: Option<String>,
    #[props(default)] orientation: RadioOrientation,
    #[props(default)] label: Option<String>,
    #[props(default)] help_text: Option<String>,
    #[props(default)] error: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] id: Option<String>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let group_id = crate::a11y::use_unique_id("radio-group", id);
    let name_str = name.clone().unwrap_or_else(|| group_id.clone());

    provide_context(RadioContext {
        value,
        name: Some(name_str),
        disabled,
    });

    let orient_class = orientation.scss_class();

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let theme_val = theme.read();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("radio_group").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("radio_group", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let is_err = error.is_some();

    rsx! {
        div {
            class: "sd-radio-group-wrapper {class_modifiers} {class}",
            role: "radiogroup",
            "aria-labelledby": label.as_ref().map(|_| format!("{group_id}-label")),
            "aria-invalid": if is_err { "true" } else { "false" },

            if let Some(lbl) = &label {
                span {
                    id: "{group_id}-label",
                    class: "sd-radio-group-label",
                    "{lbl}"
                }
            }

            div {
                class: "sd-radio-group-items {orient_class} {base_override}",
                {children}
            }

            if let Some(err) = &error {
                p { class: "sd-radio-group-error", "{err}" }
            } else if let Some(help) = &help_text {
                p { class: "sd-radio-group-help", "{help}" }
            }
        }
    }
}

/// Pulsante radio classico circolare con dot centrale animato ed etichetta/descrizione.
#[component]
pub fn Radio(
    value: String,
    #[props(default)] checked: Option<Signal<String>>,
    #[props(default)] name: Option<String>,
    #[props(default)] label: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] id: Option<String>,
    #[props(default)] class: String,
    #[props(default)] on_change: Option<EventHandler<String>>,
) -> Element {
    let ctx = try_use_context::<RadioContext>();
    let active_signal = checked.or_else(|| ctx.as_ref().map(|c| c.value));
    let parent_disabled = ctx.as_ref().is_some_and(|c| c.disabled);
    let is_disabled = disabled || parent_disabled;
    let group_name = name.or_else(|| ctx.and_then(|c| c.name));

    let radio_id = crate::a11y::use_unique_id("sd-radio", id);
    let is_selected = active_signal.is_some_and(|s| *s.read() == value);

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let theme_val = theme.read();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("radio").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("radio", &class);
    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let selected_class = if is_selected { "sd-radio__circle--checked" } else { "" };
    let disabled_class = if is_disabled { "sd-radio--disabled" } else { "" };

    let value_for_click = value.clone();
    let handle_select = move || {
        if !is_disabled {
            if let Some(mut sig) = active_signal {
                *sig.write() = value_for_click.clone();
            }
            if let Some(handler) = on_change {
                handler.call(value_for_click.clone());
            }
        }
    };

    rsx! {
        label {
            r#for: "{radio_id}",
            class: "sd-radio {base_override} {disabled_class} {class_modifiers} {class}",
            "aria-disabled": if is_disabled { "true" } else { "false" },
            onclick: {
                let handle = handle_select;
                move |_| {
                    handle();
                }
            },

            input {
                r#type: "radio",
                id: "{radio_id}",
                name: group_name.as_deref().unwrap_or_default(),
                value: "{value}",
                checked: is_selected,
                disabled: is_disabled,
                class: "sd-radio__input",
                tabindex: "-1",
                "aria-hidden": "true",
            }

            span {
                class: "sd-radio__circle {selected_class}",
                span { class: "sd-radio__dot" }
            }

            if label.is_some() || description.is_some() {
                span { class: "sd-radio__content",
                    if let Some(lbl) = &label {
                        span { class: "sd-radio__label", "{lbl}" }
                    }
                    if let Some(desc) = &description {
                        span { class: "sd-radio__description", "{desc}" }
                    }
                }
            }
        }
    }
}

/// Scheda di selezione radio ricca con icona, titolo, descrizione, badge opzionale e glow attivo.
#[component]
pub fn RadioCard(
    value: String,
    #[props(default)] checked: Option<Signal<String>>,
    #[props(default)] name: Option<String>,
    #[props(default)] title: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] icon: Option<IconKind>,
    #[props(default)] badge: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] id: Option<String>,
    #[props(default)] class: String,
    #[props(default)] on_change: Option<EventHandler<String>>,
    #[props(default)] children: Element,
) -> Element {
    let ctx = try_use_context::<RadioContext>();
    let active_signal = checked.or_else(|| ctx.as_ref().map(|c| c.value));
    let parent_disabled = ctx.as_ref().is_some_and(|c| c.disabled);
    let is_disabled = disabled || parent_disabled;
    let group_name = name.or_else(|| ctx.and_then(|c| c.name));

    let card_id = crate::a11y::use_unique_id("sd-radio-card", id);
    let is_selected = active_signal.is_some_and(|s| *s.read() == value);

    #[cfg(feature = "theme")]
    let theme = crate::theme::use_theme();
    #[cfg(feature = "theme")]
    let theme_val = theme.read();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("radio_card").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("radio_card", &class);
    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let selected_card_class = if is_selected { "sd-radio-card--selected" } else { "" };
    let disabled_class = if is_disabled { "sd-radio-card--disabled" } else { "" };
    let val_click1 = value.clone();
    let val_click2 = value.clone();
    let circle_selected_class = if is_selected { "sd-radio__circle--checked" } else { "" };

    rsx! {
        div {
            class: "sd-radio-card {base_override} {selected_card_class} {disabled_class} {class_modifiers} {class}",
            "aria-disabled": if is_disabled { "true" } else { "false" },
            role: "radio",
            "aria-checked": "{is_selected}",
            tabindex: if is_disabled { "-1" } else { "0" },
            onclick: move |e| {
                e.prevent_default();
                if !is_disabled {
                    if let Some(mut sig) = active_signal {
                        *sig.write() = val_click1.clone();
                    }
                    if let Some(handler) = on_change {
                        handler.call(val_click1.clone());
                    }
                }
            },
            onkeydown: move |e| {
                if (e.key() == Key::Enter || e.key() == Key::Character(" ".to_string())) && !is_disabled {
                    e.prevent_default();
                    if let Some(mut sig) = active_signal {
                        *sig.write() = val_click2.clone();
                    }
                    if let Some(handler) = on_change {
                        handler.call(val_click2.clone());
                    }
                }
            },

            input {
                r#type: "radio",
                id: "{card_id}",
                name: group_name.as_deref().unwrap_or_default(),
                value: "{value}",
                checked: is_selected,
                disabled: is_disabled,
                class: "sd-radio-card__input",
            }

            div { class: "sd-radio-card__header",
                div { class: "sd-radio-card__lead",
                    if let Some(ic) = icon {
                        span { class: "sd-radio-card__icon",
                            Icon { kind: ic, size: 20 }
                        }
                    }

                    div { class: "sd-radio-card__text",
                        if let Some(t) = &title {
                            span { class: "sd-radio-card__title", "{t}" }
                        }
                        if let Some(desc) = &description {
                            span { class: "sd-radio-card__description", "{desc}" }
                        }
                    }
                }

                div { class: "sd-radio-card__trail",
                    if let Some(b) = &badge {
                        span { class: "sd-radio-card__badge", "{b}" }
                    }

                    span {
                        class: "sd-radio__circle {circle_selected_class}",
                        span { class: "sd-radio__dot" }
                    }
                }
            }

            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_orientation_classes() {
        assert_eq!(RadioOrientation::Vertical.scss_class(), "sd-radio-group--vertical");
        assert_eq!(RadioOrientation::Horizontal.scss_class(), "sd-radio-group--horizontal");
    }

    #[test]
    fn test_radio_orientation_default() {
        assert_eq!(RadioOrientation::default(), RadioOrientation::Vertical);
    }
}
