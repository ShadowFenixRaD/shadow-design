use dioxus::prelude::*;

/// Tipo di lista HTML: non ordinata (`<ul>`) o ordinata (`<ol>`).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ListType {
    #[default]
    Unordered,
    Ordered,
}

/// Marcatore o stile del punto elenco.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ListMarker {
    #[default]
    Disc,
    Circle,
    Square,
    Decimal,
    LowerAlpha,
    UpperAlpha,
    LowerRoman,
    UpperRoman,
    None,
    Custom,
}

impl ListMarker {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            Self::Disc => "sd-list--marker-disc",
            Self::Circle => "sd-list--marker-circle",
            Self::Square => "sd-list--marker-square",
            Self::Decimal => "sd-list--marker-decimal",
            Self::LowerAlpha => "sd-list--marker-lower-alpha",
            Self::UpperAlpha => "sd-list--marker-upper-alpha",
            Self::LowerRoman => "sd-list--marker-lower-roman",
            Self::UpperRoman => "sd-list--marker-upper-roman",
            Self::None => "sd-list--marker-none",
            Self::Custom => "sd-list--marker-custom",
        }
    }
}

/// Spaziatura verticale o orizzontale tra gli elementi della lista.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ListSpacing {
    Xs,
    #[default]
    Sm,
    Md,
    Lg,
    Xl,
}

impl ListSpacing {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            Self::Xs => "sd-list--spacing-xs",
            Self::Sm => "sd-list--spacing-sm",
            Self::Md => "sd-list--spacing-md",
            Self::Lg => "sd-list--spacing-lg",
            Self::Xl => "sd-list--spacing-xl",
        }
    }
}

/// Scala dimensionale del testo della lista.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ListSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl ListSize {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            Self::Xs => "sd-list--size-xs",
            Self::Sm => "sd-list--size-sm",
            Self::Md => "sd-list--size-md",
            Self::Lg => "sd-list--size-lg",
            Self::Xl => "sd-list--size-xl",
        }
    }
}

/// Orientamento della lista: verticale standard o orizzontale in riga (es. breadcrumb o tag-list).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ListOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl ListOrientation {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            Self::Vertical => "sd-list--vertical",
            Self::Horizontal => "sd-list--horizontal",
        }
    }
}

/// Contesto condiviso tra il contenitore `List` e i singoli `ListItem`.
#[derive(Clone, PartialEq, Default)]
pub struct ListContext {
    pub list_type: ListType,
    pub marker: ListMarker,
    pub spacing: ListSpacing,
    pub size: ListSize,
    pub orientation: ListOrientation,
    pub center: bool,
    pub icon: Option<Element>,
}

/// Contenitore per elenchi ordinati (`<ol>`) o non ordinati (`<ul>`) con supporto a marcatori personalizzati,
/// spaziatura configurabile, orientamento orizzontale/verticale e sub-liste annidate.
#[component]
pub fn List(
    #[props(default)] list_type: ListType,
    #[props(default)] marker: Option<ListMarker>,
    #[props(default)] spacing: ListSpacing,
    #[props(default)] size: ListSize,
    #[props(default)] orientation: ListOrientation,
    #[props(default)] center: bool,
    #[props(default)] with_padding: Option<bool>,
    #[props(default)] icon: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] id: Option<String>,
    children: Element,
) -> Element {
    let effective_marker = marker.unwrap_or_else(|| match list_type {
        ListType::Ordered => ListMarker::Decimal,
        ListType::Unordered => {
            if icon.is_some() {
                ListMarker::Custom
            } else {
                ListMarker::Disc
            }
        }
    });

    let has_custom_icon = icon.is_some() || effective_marker == ListMarker::Custom;
    let padding_class = match with_padding {
        Some(true) => "sd-list--with-padding",
        Some(false) => "sd-list--no-padding",
        None => {
            if has_custom_icon
                || orientation == ListOrientation::Horizontal
                || effective_marker == ListMarker::None
            {
                "sd-list--no-padding"
            } else {
                "sd-list--with-padding"
            }
        }
    };

    let center_class = if center { "sd-list--center" } else { "" };
    let marker_class = effective_marker.scss_class();
    let spacing_class = spacing.scss_class();
    let size_class = size.scss_class();
    let orient_class = orientation.scss_class();

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("list").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("list", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let combined_class = format!(
        "sd-list {marker_class} {spacing_class} {size_class} {orient_class} {padding_class} {center_class} {base_override} {class_modifiers} {class}"
    );

    use_context_provider(|| ListContext {
        list_type,
        marker: effective_marker,
        spacing,
        size,
        orientation,
        center,
        icon: icon.clone(),
    });

    match list_type {
        ListType::Ordered => rsx! {
            ol {
                class: "{combined_class}",
                id: id,
                {children}
            }
        },
        ListType::Unordered => rsx! {
            ul {
                class: "{combined_class}",
                id: id,
                {children}
            }
        },
    }
}

/// Singolo elemento di una lista (`<li>`), con supporto a icona personalizzata individuale o ereditata dalla lista padre.
#[component]
pub fn ListItem(
    #[props(default)] icon: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] id: Option<String>,
    children: Element,
) -> Element {
    let ctx = use_context::<ListContext>();
    let effective_icon = icon.or(ctx.icon);

    let center_class = if ctx.center {
        "sd-list-item--center"
    } else {
        ""
    };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("list_item").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("list_item", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    if let Some(ic) = effective_icon {
        rsx! {
            li {
                class: "sd-list-item sd-list-item--with-icon {center_class} {base_override} {class_modifiers} {class}",
                id: id,
                span {
                    class: "sd-list-item__icon",
                    aria_hidden: "true",
                    {ic}
                }
                span {
                    class: "sd-list-item__content",
                    {children}
                }
            }
        }
    } else {
        rsx! {
            li {
                class: "sd-list-item {center_class} {base_override} {class_modifiers} {class}",
                id: id,
                span {
                    class: "sd-list-item__content",
                    {children}
                }
            }
        }
    }
}
