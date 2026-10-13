#[allow(dead_code)]
pub struct ComponentGlowSpec {
    pub component: &'static str,
    pub base_class: &'static str,
    pub default_on: bool,
}

#[allow(dead_code)]
pub const SPECS: &[ComponentGlowSpec] = &[
    ComponentGlowSpec { component: "button", base_class: "sd-btn--glow", default_on: true },
    ComponentGlowSpec { component: "badge", base_class: "sd-badge--glow", default_on: false },
    ComponentGlowSpec { component: "input", base_class: "sd-input--glow", default_on: false },
    ComponentGlowSpec { component: "card", base_class: "sd-card--hover-glow", default_on: false },
    ComponentGlowSpec { component: "tooltip", base_class: "sd-tooltip__bubble--glow", default_on: false },
    ComponentGlowSpec { component: "drawer", base_class: "sd-drawer-panel--glow", default_on: false },
    ComponentGlowSpec { component: "select", base_class: "sd-select-trigger--glow", default_on: false },
    ComponentGlowSpec { component: "switch", base_class: "sd-switch--glow", default_on: false },
    ComponentGlowSpec { component: "checkbox", base_class: "sd-checkbox--glow", default_on: false },
    ComponentGlowSpec { component: "modal", base_class: "sd-modal-dialog--glow", default_on: false },
    ComponentGlowSpec { component: "dropdown", base_class: "sd-dropdown-menu--glow", default_on: false },
    ComponentGlowSpec { component: "alert", base_class: "sd-alert--glow", default_on: false },
    ComponentGlowSpec { component: "notification", base_class: "sd-toast--glow", default_on: false },
];

#[allow(dead_code)]
pub fn get_spec(component: &str) -> Option<&'static ComponentGlowSpec> {
    SPECS.iter().find(|s| s.component == component)
}

/// Resolves the glow CSS class for a component.
///
/// - `glow`: `Some(false)` disables glow; `Some(true)` forces glow on; `None` uses `default_on`.
/// - `theme_glow_enabled`: In theme mode, `true` unless `GlowMode::Off`. In no-theme mode, defaults to `true`.
/// - `color_suffix`: Optional color modifier (e.g. `Some("violet")` -> `sd-card--glow-violet`).
#[allow(dead_code)]
pub(crate) fn resolve_glow(
    component: &str,
    glow: Option<bool>,
    theme_glow_enabled: bool,
    color_suffix: Option<&str>,
) -> String {
    let spec = match get_spec(component) {
        Some(s) => s,
        None => return String::new(),
    };

    let is_on = match glow {
        Some(true) => true,
        Some(false) => false,
        None => spec.default_on && theme_glow_enabled,
    };

    if !is_on {
        return String::new();
    }

    if let Some(color) = color_suffix {
        if !color.is_empty() {
            return format!("{}-{}", spec.base_class, color);
        }
    }

    spec.base_class.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_glow_explicit_false() {
        for spec in SPECS {
            assert_eq!(resolve_glow(spec.component, Some(false), true, None), "");
            assert_eq!(resolve_glow(spec.component, Some(false), true, Some("violet")), "");
        }
    }

    #[test]
    fn test_resolve_glow_explicit_true() {
        for spec in SPECS {
            assert_eq!(
                resolve_glow(spec.component, Some(true), true, None),
                spec.base_class
            );
            assert_eq!(
                resolve_glow(spec.component, Some(true), false, None),
                spec.base_class
            );
            assert_eq!(
                resolve_glow(spec.component, Some(true), true, Some("violet")),
                format!("{}-violet", spec.base_class)
            );
        }
    }

    #[test]
    fn test_resolve_glow_default() {
        // Button defaults on
        assert_eq!(resolve_glow("button", None, true, None), "sd-btn--glow");
        assert_eq!(resolve_glow("button", None, false, None), "");

        // Input defaults off
        assert_eq!(resolve_glow("input", None, true, None), "");
    }
}
