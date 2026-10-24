pub mod item;
pub use item::{BottomNavItem, BottomNavItemProps};

use dioxus::prelude::*;

/// Modalità di posizionamento della barra di navigazione inferiore.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum BottomNavPosition {
    /// Posizione fissa in basso sullo schermo (`fixed bottom-0 left-0 right-0`).
    #[default]
    Fixed,
    /// Posizione assoluta all'interno di un contenitore relativo (ottimo per preview/mockup).
    Absolute,
    /// Flusso relativo standard all'interno del layout.
    Relative,
}

impl BottomNavPosition {
    pub const ALL: &'static [BottomNavPosition] = &[
        BottomNavPosition::Fixed,
        BottomNavPosition::Absolute,
        BottomNavPosition::Relative,
    ];

    #[must_use]
    pub fn as_class(&self) -> &'static str {
        match self {
            BottomNavPosition::Fixed => "sd-bottom-nav--fixed",
            BottomNavPosition::Absolute => "sd-bottom-nav--absolute",
            BottomNavPosition::Relative => "sd-bottom-nav--relative",
        }
    }

    #[must_use]
    pub fn name(&self) -> &'static str {
        match self {
            BottomNavPosition::Fixed => "Fixed",
            BottomNavPosition::Absolute => "Absolute",
            BottomNavPosition::Relative => "Relative",
        }
    }
}

/// Proprietà del componente contenitore [`BottomNavigation`].
#[derive(Props, Clone, PartialEq)]
pub struct BottomNavigationProps {
    /// Modalità di posizionamento (default: `BottomNavPosition::Fixed`).
    #[props(default)]
    pub position: Option<BottomNavPosition>,
    /// Aggiunge padding inferiore automatico per la safe area dei dispositivi mobili (`env(safe-area-inset-bottom)`). Default: true.
    #[props(default = true)]
    pub safe_area: bool,
    /// Mostra la sottile linea superiore semi-trasparente (Specular Hairline `border-t border-white/10`). Default: true.
    #[props(default = true)]
    pub border: bool,
    /// Attiva l'effetto di sfocatura dello sfondo (glassmorphism backdrop blur). Default: true.
    #[props(default = true)]
    pub blur: bool,
    /// Classi CSS addizionali applicate al contenitore.
    #[props(default)]
    pub class: String,
    /// Voci o destinazioni figlie ([`BottomNavItem`]).
    pub children: Element,
}

/// Barra di navigazione fissa inferiore progettata per dispositivi mobili e schermate compatte.
/// Supporta da 3 a 5 destinazioni, transizioni fluide su icone ed etichette, badge di notifica
/// e gestione safe area integrata.
#[component]
pub fn BottomNavigation(props: BottomNavigationProps) -> Element {
    let locale = crate::locale::use_locale();
    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("bottom_navigation").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("bottom_navigation", &props.class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let pos = props.position.unwrap_or_default();
    let pos_class = pos.as_class();
    let safe_area_class = if props.safe_area { "sd-bottom-nav--safe-area" } else { "" };
    let border_class = if props.border { "sd-bottom-nav--border" } else { "" };
    let blur_class = if props.blur { "sd-bottom-nav--blur" } else { "" };

    rsx! {
        nav {
            role: "navigation",
            "aria-label": "{locale.bottom_navigation_label}",
            class: "sd-bottom-nav {pos_class} {safe_area_class} {border_class} {blur_class} {base_override} {class_modifiers} {props.class}",
            div { class: "sd-bottom-nav__container",
                {props.children}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bottom_nav_position_classes() {
        assert_eq!(BottomNavPosition::Fixed.as_class(), "sd-bottom-nav--fixed");
        assert_eq!(BottomNavPosition::Absolute.as_class(), "sd-bottom-nav--absolute");
        assert_eq!(BottomNavPosition::Relative.as_class(), "sd-bottom-nav--relative");
    }

    #[test]
    fn test_bottom_nav_position_names() {
        assert_eq!(BottomNavPosition::Fixed.name(), "Fixed");
        assert_eq!(BottomNavPosition::Absolute.name(), "Absolute");
        assert_eq!(BottomNavPosition::Relative.name(), "Relative");
        assert_eq!(BottomNavPosition::ALL.len(), 3);
    }
}

