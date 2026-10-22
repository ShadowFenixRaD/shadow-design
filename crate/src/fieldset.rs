use dioxus::prelude::*;

/// Visual variants for [`Fieldset`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum FieldsetVariant {
    #[default]
    Default,
    Filled,
    Ungrouped,
}

impl FieldsetVariant {
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Default => "sd-fieldset--default",
            Self::Filled => "sd-fieldset--filled",
            Self::Ungrouped => "sd-fieldset--ungrouped",
        }
    }
}

/// Semantic fieldset container with styled legend for grouping related form controls.
#[component]
pub fn Fieldset(
    /// Main title displayed inside the `<legend>` element.
    legend: String,
    /// Optional explanatory description shown directly below the legend.
    #[props(default)]
    description: Option<String>,
    /// Visual style variant. Defaults to [`FieldsetVariant::Default`].
    #[props(default)]
    variant: FieldsetVariant,
    /// Whether all form controls inside this fieldset are disabled. Defaults to false.
    #[props(default = false)]
    disabled: bool,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Nested form inputs and elements.
    children: Element,
) -> Element {
    let var_class = variant.class_name();

    rsx! {
        fieldset {
            class: "sd-fieldset {var_class} {class}",
            disabled: disabled,
            legend { class: "sd-fieldset-legend", "{legend}" }
            if let Some(desc) = description {
                p { class: "sd-fieldset-desc", "{desc}" }
            }
            div { class: "sd-fieldset-content",
                {children}
            }
        }
    }
}
