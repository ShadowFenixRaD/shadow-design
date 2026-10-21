use dioxus::prelude::*;
use crate::group::{AlignItems, JustifyContent, SpacingToken};

/// Primitiva Flexbox verticale a colonna con spaziatura tokenizzata e allineamento uniforme.
#[component]
pub fn Stack(
    #[props(default)] gap: SpacingToken,
    #[props(default = AlignItems::Stretch)] align: AlignItems,
    #[props(default)] justify: JustifyContent,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let gap_class = gap.scss_stack_gap_class();
    let align_class = align.scss_stack_class();
    let justify_class = justify.scss_stack_class();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("stack").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("stack", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        div {
            class: "sd-stack {gap_class} {align_class} {justify_class} {base_override} {class_modifiers} {class}",
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_gap_classes() {
        assert_eq!(SpacingToken::None.scss_stack_gap_class(), "sd-stack--gap-none");
        assert_eq!(SpacingToken::Xs.scss_stack_gap_class(), "sd-stack--gap-xs");
        assert_eq!(SpacingToken::Md.scss_stack_gap_class(), "sd-stack--gap-md");
        assert_eq!(SpacingToken::Lg.scss_stack_gap_class(), "sd-stack--gap-lg");
    }

    #[test]
    fn test_stack_align_classes() {
        assert_eq!(AlignItems::Stretch.scss_stack_class(), "sd-stack--align-stretch");
        assert_eq!(AlignItems::Center.scss_stack_class(), "sd-stack--align-center");
    }
}
