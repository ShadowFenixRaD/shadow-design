use dioxus::prelude::*;

/// Spaziatura tokenizzata per layout Flexbox (Group e Stack).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SpacingToken {
    None,
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl SpacingToken {
    #[must_use]
    pub fn scss_group_gap_class(&self) -> &'static str {
        match self {
            Self::None => "sd-group--gap-none",
            Self::Xs => "sd-group--gap-xs",
            Self::Sm => "sd-group--gap-sm",
            Self::Md => "sd-group--gap-md",
            Self::Lg => "sd-group--gap-lg",
            Self::Xl => "sd-group--gap-xl",
        }
    }

    #[must_use]
    pub fn scss_stack_gap_class(&self) -> &'static str {
        match self {
            Self::None => "sd-stack--gap-none",
            Self::Xs => "sd-stack--gap-xs",
            Self::Sm => "sd-stack--gap-sm",
            Self::Md => "sd-stack--gap-md",
            Self::Lg => "sd-stack--gap-lg",
            Self::Xl => "sd-stack--gap-xl",
        }
    }
}

/// Allineamento trasversale (cross-axis / align-items) per contenitori flessibili.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum AlignItems {
    Start,
    #[default]
    Center,
    End,
    Stretch,
    Baseline,
}

impl AlignItems {
    #[must_use]
    pub fn scss_group_class(&self) -> &'static str {
        match self {
            Self::Start => "sd-group--align-start",
            Self::Center => "sd-group--align-center",
            Self::End => "sd-group--align-end",
            Self::Stretch => "sd-group--align-stretch",
            Self::Baseline => "sd-group--align-baseline",
        }
    }

    #[must_use]
    pub fn scss_stack_class(&self) -> &'static str {
        match self {
            Self::Start => "sd-stack--align-start",
            Self::Center => "sd-stack--align-center",
            Self::End => "sd-stack--align-end",
            Self::Stretch => "sd-stack--align-stretch",
            Self::Baseline => "sd-stack--align-baseline",
        }
    }
}

/// Giustificazione lungo l'asse principale (main-axis / justify-content).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum JustifyContent {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl JustifyContent {
    #[must_use]
    pub fn scss_group_class(&self) -> &'static str {
        match self {
            Self::Start => "sd-group--justify-start",
            Self::Center => "sd-group--justify-center",
            Self::End => "sd-group--justify-end",
            Self::SpaceBetween => "sd-group--justify-between",
            Self::SpaceAround => "sd-group--justify-around",
            Self::SpaceEvenly => "sd-group--justify-evenly",
        }
    }

    #[must_use]
    pub fn scss_stack_class(&self) -> &'static str {
        match self {
            Self::Start => "sd-stack--justify-start",
            Self::Center => "sd-stack--justify-center",
            Self::End => "sd-stack--justify-end",
            Self::SpaceBetween => "sd-stack--justify-between",
            Self::SpaceAround => "sd-stack--justify-around",
            Self::SpaceEvenly => "sd-stack--justify-evenly",
        }
    }
}

/// Primitiva Flexbox orizzontale ergonomica con supporto a wrapping, allineamento e crescita elementi.
#[component]
pub fn Group(
    #[props(default)] gap: SpacingToken,
    #[props(default)] align: AlignItems,
    #[props(default)] justify: JustifyContent,
    #[props(default = true)] wrap: bool,
    #[props(default = false)] grow: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let gap_class = gap.scss_group_gap_class();
    let align_class = align.scss_group_class();
    let justify_class = justify.scss_group_class();
    let wrap_class = if wrap { "sd-group--wrap" } else { "sd-group--nowrap" };
    let grow_class = if grow { "sd-group--grow" } else { "" };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("group").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("group", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        div {
            class: "sd-group {gap_class} {align_class} {justify_class} {wrap_class} {grow_class} {base_override} {class_modifiers} {class}",
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacing_token_classes() {
        assert_eq!(SpacingToken::None.scss_group_gap_class(), "sd-group--gap-none");
        assert_eq!(SpacingToken::Xs.scss_group_gap_class(), "sd-group--gap-xs");
        assert_eq!(SpacingToken::Md.scss_group_gap_class(), "sd-group--gap-md");
        assert_eq!(SpacingToken::Lg.scss_group_gap_class(), "sd-group--gap-lg");
    }

    #[test]
    fn test_align_and_justify_classes() {
        assert_eq!(AlignItems::Center.scss_group_class(), "sd-group--align-center");
        assert_eq!(JustifyContent::SpaceBetween.scss_group_class(), "sd-group--justify-between");
        assert_eq!(JustifyContent::SpaceEvenly.scss_group_class(), "sd-group--justify-evenly");
    }
}
