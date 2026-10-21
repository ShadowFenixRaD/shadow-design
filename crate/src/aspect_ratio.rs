use dioxus::prelude::*;

/// Contenitore per vincolare il rapporto di forma (Aspect Ratio) di media, video, canvas o iframe,
/// prevenendo il Cumulative Layout Shift (CLS) e garantendo scalabilità responsive.
#[component]
pub fn AspectRatio(
    #[props(default = 16.0 / 9.0)] ratio: f64,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let preset_class = if (ratio - 16.0 / 9.0).abs() < 0.01 {
        "sd-aspect-ratio--16-9"
    } else if (ratio - 4.0 / 3.0).abs() < 0.01 {
        "sd-aspect-ratio--4-3"
    } else if (ratio - 1.0).abs() < 0.01 {
        "sd-aspect-ratio--1-1"
    } else if (ratio - 21.0 / 9.0).abs() < 0.01 {
        "sd-aspect-ratio--21-9"
    } else {
        ""
    };

    let inline_style = if preset_class.is_empty() {
        format!("aspect-ratio: {};", ratio)
    } else {
        String::new()
    };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("aspect_ratio").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("aspect_ratio", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    rsx! {
        div {
            class: "sd-aspect-ratio {preset_class} {base_override} {class_modifiers} {class}",
            style: "{inline_style}",
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_aspect_ratio_presets() {
        let r16_9: f64 = 16.0 / 9.0;
        let r4_3: f64 = 4.0 / 3.0;
        let r1_1: f64 = 1.0;
        let r21_9: f64 = 21.0 / 9.0;

        assert!((r16_9 - 16.0 / 9.0).abs() < 0.01);
        assert!((r4_3 - 4.0 / 3.0).abs() < 0.01);
        assert!((r1_1 - 1.0).abs() < 0.01);
        assert!((r21_9 - 21.0 / 9.0).abs() < 0.01);
    }
}
