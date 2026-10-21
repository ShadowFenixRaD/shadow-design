use dioxus::prelude::*;

/// Spaziatura tokenizzata per colonne e righe dei layout a griglia.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GridSpacing {
    None,
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl GridSpacing {
    #[must_use]
    pub fn scss_gutter_class(&self) -> &'static str {
        match self {
            Self::None => "sd-grid--gutter-none",
            Self::Xs => "sd-grid--gutter-xs",
            Self::Sm => "sd-grid--gutter-sm",
            Self::Md => "sd-grid--gutter-md",
            Self::Lg => "sd-grid--gutter-lg",
            Self::Xl => "sd-grid--gutter-xl",
        }
    }

    #[must_use]
    pub fn scss_row_gutter_class(&self) -> &'static str {
        match self {
            Self::None => "sd-grid--row-gutter-none",
            Self::Xs => "sd-grid--row-gutter-xs",
            Self::Sm => "sd-grid--row-gutter-sm",
            Self::Md => "sd-grid--row-gutter-md",
            Self::Lg => "sd-grid--row-gutter-lg",
            Self::Xl => "sd-grid--row-gutter-xl",
        }
    }

    #[must_use]
    pub fn scss_spacing_class(&self) -> &'static str {
        match self {
            Self::None => "sd-simple-grid--spacing-none",
            Self::Xs => "sd-simple-grid--spacing-xs",
            Self::Sm => "sd-simple-grid--spacing-sm",
            Self::Md => "sd-simple-grid--spacing-md",
            Self::Lg => "sd-simple-grid--spacing-lg",
            Self::Xl => "sd-simple-grid--spacing-xl",
        }
    }

    #[must_use]
    pub fn scss_vertical_spacing_class(&self) -> &'static str {
        match self {
            Self::None => "sd-simple-grid--vertical-spacing-none",
            Self::Xs => "sd-simple-grid--vertical-spacing-xs",
            Self::Sm => "sd-simple-grid--vertical-spacing-sm",
            Self::Md => "sd-simple-grid--vertical-spacing-md",
            Self::Lg => "sd-simple-grid--vertical-spacing-lg",
            Self::Xl => "sd-simple-grid--vertical-spacing-xl",
        }
    }
}

/// Allineamento verticale degli elementi all'interno della griglia.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GridAlign {
    Start,
    Center,
    End,
    #[default]
    Stretch,
    Baseline,
}

impl GridAlign {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            Self::Start => "sd-grid--align-start",
            Self::Center => "sd-grid--align-center",
            Self::End => "sd-grid--align-end",
            Self::Stretch => "sd-grid--align-stretch",
            Self::Baseline => "sd-grid--align-baseline",
        }
    }
}

/// Giustificazione orizzontale del contenuto della griglia.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum GridJustify {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

impl GridJustify {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            Self::Start => "sd-grid--justify-start",
            Self::Center => "sd-grid--justify-center",
            Self::End => "sd-grid--justify-end",
            Self::SpaceBetween => "sd-grid--justify-between",
            Self::SpaceAround => "sd-grid--justify-around",
        }
    }
}

/// Sistema a griglia responsive a 12 colonne ispirato a Mantine e MUI.
#[component]
pub fn Grid(
    #[props(default)] gutter: GridSpacing,
    #[props(default)] row_gutter: Option<GridSpacing>,
    #[props(default)] align: GridAlign,
    #[props(default)] justify: GridJustify,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let gutter_class = gutter.scss_gutter_class();
    let row_gutter_class = row_gutter
        .map(|rg| rg.scss_row_gutter_class())
        .unwrap_or("");
    let align_class = align.scss_class();
    let justify_class = justify.scss_class();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("grid").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("grid", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        div {
            class: "sd-grid {gutter_class} {row_gutter_class} {align_class} {justify_class} {base_override} {class_modifiers} {class}",
            {children}
        }
    }
}

/// Cella interna della griglia con span e offset configurabili, con supporto responsive.
#[component]
pub fn GridCol(
    #[props(default = 12)] span: u8,
    #[props(default = 0)] offset: u8,
    #[props(default)] sm: Option<u8>,
    #[props(default)] md: Option<u8>,
    #[props(default)] lg: Option<u8>,
    #[props(default)] xl: Option<u8>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let clamped_span = span.clamp(1, 12);
    let span_class = format!("sd-grid-col--span-{}", clamped_span);
    let offset_class = if offset > 0 && offset <= 11 {
        format!("sd-grid-col--offset-{}", offset)
    } else {
        String::new()
    };

    let sm_class = sm
        .map(|s| format!("sm:sd-grid-col--span-{}", s.clamp(1, 12)))
        .unwrap_or_default();
    let md_class = md
        .map(|s| format!("md:sd-grid-col--span-{}", s.clamp(1, 12)))
        .unwrap_or_default();
    let lg_class = lg
        .map(|s| format!("lg:sd-grid-col--span-{}", s.clamp(1, 12)))
        .unwrap_or_default();
    let xl_class = xl
        .map(|s| format!("xl:sd-grid-col--span-{}", s.clamp(1, 12)))
        .unwrap_or_default();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("grid_col").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("grid_col", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        div {
            class: "sd-grid-col {span_class} {offset_class} {sm_class} {md_class} {lg_class} {xl_class} {base_override} {class_modifiers} {class}",
            {children}
        }
    }
}

/// Griglia a colonne di larghezza equivalente automatica con gutter uniforme.
#[component]
pub fn SimpleGrid(
    #[props(default = 1)] cols: u8,
    #[props(default)] sm: Option<u8>,
    #[props(default)] md: Option<u8>,
    #[props(default)] lg: Option<u8>,
    #[props(default)] xl: Option<u8>,
    #[props(default)] spacing: GridSpacing,
    #[props(default)] vertical_spacing: Option<GridSpacing>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let clamped_cols = cols.clamp(1, 12);
    let cols_class = format!("sd-simple-grid--cols-{}", clamped_cols);
    let spacing_class = spacing.scss_spacing_class();
    let vertical_spacing_class = vertical_spacing
        .map(|vs| vs.scss_vertical_spacing_class())
        .unwrap_or("");

    let sm_class = sm
        .map(|c| format!("sm:sd-simple-grid--cols-{}", c.clamp(1, 12)))
        .unwrap_or_default();
    let md_class = md
        .map(|c| format!("md:sd-simple-grid--cols-{}", c.clamp(1, 12)))
        .unwrap_or_default();
    let lg_class = lg
        .map(|c| format!("lg:sd-simple-grid--cols-{}", c.clamp(1, 12)))
        .unwrap_or_default();
    let xl_class = xl
        .map(|c| format!("xl:sd-simple-grid--cols-{}", c.clamp(1, 12)))
        .unwrap_or_default();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("simple_grid").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("simple_grid", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        div {
            class: "sd-simple-grid {cols_class} {spacing_class} {vertical_spacing_class} {sm_class} {md_class} {lg_class} {xl_class} {base_override} {class_modifiers} {class}",
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_spacing_classes() {
        assert_eq!(GridSpacing::Xs.scss_gutter_class(), "sd-grid--gutter-xs");
        assert_eq!(GridSpacing::Md.scss_gutter_class(), "sd-grid--gutter-md");
        assert_eq!(GridSpacing::Lg.scss_row_gutter_class(), "sd-grid--row-gutter-lg");
        assert_eq!(GridSpacing::Sm.scss_spacing_class(), "sd-simple-grid--spacing-sm");
        assert_eq!(GridSpacing::Xl.scss_vertical_spacing_class(), "sd-simple-grid--vertical-spacing-xl");
    }

    #[test]
    fn test_grid_align_and_justify_classes() {
        assert_eq!(GridAlign::Start.scss_class(), "sd-grid--align-start");
        assert_eq!(GridAlign::Center.scss_class(), "sd-grid--align-center");
        assert_eq!(GridAlign::Stretch.scss_class(), "sd-grid--align-stretch");
        assert_eq!(GridJustify::SpaceBetween.scss_class(), "sd-grid--justify-between");
    }
}
