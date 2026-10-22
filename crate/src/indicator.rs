use dioxus::prelude::*;

/// Corner placement position for [`Indicator`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum IndicatorPosition {
    #[default]
    TopEnd,
    TopStart,
    BottomEnd,
    BottomStart,
}

impl IndicatorPosition {
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::TopEnd => "sd-indicator-badge--top-end",
            Self::TopStart => "sd-indicator-badge--top-start",
            Self::BottomEnd => "sd-indicator-badge--bottom-end",
            Self::BottomStart => "sd-indicator-badge--bottom-start",
        }
    }
}

/// Color theme for [`Indicator`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum IndicatorColor {
    #[default]
    Violet,
    Emerald,
    Rose,
    Amber,
    Cyan,
}

impl IndicatorColor {
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Violet => "sd-indicator-badge--violet",
            Self::Emerald => "sd-indicator-badge--emerald",
            Self::Rose => "sd-indicator-badge--rose",
            Self::Amber => "sd-indicator-badge--amber",
            Self::Cyan => "sd-indicator-badge--cyan",
        }
    }
}

/// Predefined sizes for [`Indicator`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum IndicatorSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl IndicatorSize {
    #[must_use]
    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Sm => "sd-indicator-badge--sm",
            Self::Md => "sd-indicator-badge--md",
            Self::Lg => "sd-indicator-badge--lg",
        }
    }
}

/// Visual status dot or badge overlay attached to any corner of wrapped content.
#[component]
pub fn Indicator(
    /// Corner placement. Defaults to [`IndicatorPosition::TopEnd`].
    #[props(default)]
    position: IndicatorPosition,
    /// Color variant. Defaults to [`IndicatorColor::Violet`].
    #[props(default)]
    color: IndicatorColor,
    /// Indicator size. Defaults to [`IndicatorSize::Md`].
    #[props(default)]
    size: IndicatorSize,
    /// Optional label or counter string (e.g. "5" or "99+"). If None, displays a dot.
    #[props(default)]
    label: Option<String>,
    /// Whether to render an animated radar pulse wave indicating active processing.
    #[props(default = false)]
    processing: bool,
    /// Whether the indicator is disabled / hidden. Defaults to false.
    #[props(default = false)]
    disabled: bool,
    /// Optional custom pixel offset from the corner.
    #[props(default)]
    offset: Option<i32>,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Target content wrapped by the indicator.
    children: Element,
) -> Element {
    let pos_class = position.class_name();
    let col_class = color.class_name();
    let sz_class = size.class_name();
    let with_label_class = if label.is_some() { "sd-indicator-badge--with-label" } else { "" };
    let processing_class = if processing { "sd-indicator-badge--processing" } else { "" };

    let offset_style = if let Some(off) = offset {
        match position {
            IndicatorPosition::TopEnd => format!("top: {off}px; right: {off}px;"),
            IndicatorPosition::TopStart => format!("top: {off}px; left: {off}px;"),
            IndicatorPosition::BottomEnd => format!("bottom: {off}px; right: {off}px;"),
            IndicatorPosition::BottomStart => format!("bottom: {off}px; left: {off}px;"),
        }
    } else {
        String::new()
    };

    rsx! {
        div { class: "sd-indicator {class}",
            {children}
            if !disabled {
                div {
                    class: "sd-indicator-badge {pos_class} {col_class} {sz_class} {with_label_class} {processing_class}",
                    style: "{offset_style}",
                    if let Some(ref l) = label {
                        span { class: "sd-indicator-text", "{l}" }
                    }
                    if processing {
                        span { class: "sd-indicator-pulse" }
                    }
                }
            }
        }
    }
}
