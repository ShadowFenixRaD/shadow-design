use dioxus::prelude::*;

/// Modalità di visualizzazione e comportamento delle barre di scorrimento.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScrollType {
    /// Le barre di scorrimento sono sempre visibili.
    Always,
    /// Le barre di scorrimento appaiono solo quando l'area è in stato hover o attiva.
    #[default]
    Hover,
    /// Comportamento nativo del browser (visibile solo se il contenuto eccede).
    Auto,
    /// Le barre di scorrimento sono completamente invisibili, ma il contenuto resta scrollabile.
    Never,
}

impl ScrollType {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            ScrollType::Always => "sd-scroll-area--always",
            ScrollType::Hover => "sd-scroll-area--hover",
            ScrollType::Auto => "sd-scroll-area--auto",
            ScrollType::Never => "sd-scroll-area--never",
        }
    }
}

/// Proprietà del componente [`ScrollArea`].
#[derive(Props, Clone, PartialEq)]
pub struct ScrollAreaProps {
    /// Modalità di visualizzazione della scrollbar (default: `ScrollType::Hover`).
    #[props(default)]
    pub scroll_type: ScrollType,
    /// Se abilitato, aggiunge padding per distanziare il contenuto dalla scrollbar.
    #[props(default)]
    pub offset_scrollbars: bool,
    /// Aggiunge una maschera a sfumatura sui bordi per indicare visivamente la presenza di contenuto oltre i limiti visibili.
    #[props(default)]
    pub fade_edges: bool,
    /// Abilita lo scorrimento orizzontale (default: false).
    #[props(default)]
    pub scroll_x: bool,
    /// Abilita lo scorrimento verticale (default: true).
    #[props(default = true)]
    pub scroll_y: bool,
    /// Altezza fissa dell'area di scorrimento (es. "300px", "100%").
    #[props(default)]
    pub height: Option<String>,
    /// Altezza massima dell'area di scorrimento (es. "450px").
    #[props(default)]
    pub max_height: Option<String>,
    /// Larghezza fissa dell'area di scorrimento (es. "100%", "500px").
    #[props(default)]
    pub width: Option<String>,
    /// Larghezza massima dell'area di scorrimento.
    #[props(default)]
    pub max_width: Option<String>,
    /// Classi CSS addizionali applicate al contenitore esterno.
    #[props(default)]
    pub class: String,
    /// Classi CSS addizionali applicate al viewport interno.
    #[props(default)]
    pub viewport_class: String,
    /// Contenuto da scorrere.
    pub children: Element,
}

/// Componente ScrollArea con micro-scrollbar Shadow Design, fade edges e supporto hover/always/never.
#[component]
pub fn ScrollArea(props: ScrollAreaProps) -> Element {
    let type_class = props.scroll_type.scss_class();
    let offset_class = if props.offset_scrollbars {
        "sd-scroll-area--offset"
    } else {
        ""
    };
    let fade_class = if props.fade_edges {
        "sd-scroll-area--fade-edges"
    } else {
        ""
    };
    let scroll_x_class = if props.scroll_x {
        "sd-scroll-area--scroll-x"
    } else {
        "sd-scroll-area--no-scroll-x"
    };
    let scroll_y_class = if props.scroll_y {
        "sd-scroll-area--scroll-y"
    } else {
        "sd-scroll-area--no-scroll-y"
    };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val
        .resolve_component_base("scroll_area")
        .unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("scroll_area", &props.class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let mut inline_styles = Vec::new();
    if let Some(ref h) = props.height {
        inline_styles.push(format!("height: {h};"));
    }
    if let Some(ref mh) = props.max_height {
        inline_styles.push(format!("max-height: {mh};"));
    }
    if let Some(ref w) = props.width {
        inline_styles.push(format!("width: {w};"));
    }
    if let Some(ref mw) = props.max_width {
        inline_styles.push(format!("max-width: {mw};"));
    }
    let outer_style = inline_styles.join(" ");

    rsx! {
        div {
            class: "sd-scroll-area {type_class} {offset_class} {fade_class} {scroll_x_class} {scroll_y_class} {base_override} {class_modifiers} {props.class}",
            style: "{outer_style}",
            div {
                class: "sd-scroll-area__viewport {props.viewport_class}",
                div {
                    class: "sd-scroll-area__content",
                    {props.children}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_type_defaults() {
        assert_eq!(ScrollType::default(), ScrollType::Hover);
        assert_eq!(ScrollType::Always.scss_class(), "sd-scroll-area--always");
        assert_eq!(ScrollType::Hover.scss_class(), "sd-scroll-area--hover");
        assert_eq!(ScrollType::Auto.scss_class(), "sd-scroll-area--auto");
        assert_eq!(ScrollType::Never.scss_class(), "sd-scroll-area--never");
    }
}
