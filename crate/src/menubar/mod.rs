pub mod shortcut;
pub mod item;
pub mod menu;

pub use shortcut::{MenubarShortcut, MenubarSeparator};
pub use item::{MenubarItem, MenubarItemProps};
pub use menu::{MenubarMenu, MenubarMenuProps, MenubarTrigger, MenubarTriggerProps, MenubarContent, MenubarContentProps};

use dioxus::prelude::*;

/// Contesto di coordinamento per la barra dei menu.
#[derive(Clone, Copy, PartialEq)]
pub struct MenubarCtx {
    pub active_menu: Signal<Option<String>>,
}

/// Proprietà della barra dei menu [`Menubar`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarProps {
    /// Segnale reattivo per controllare o monitorare l'id del menu correntemente aperto.
    #[props(default)]
    pub active_menu: Option<Signal<Option<String>>>,
    /// Mostra la linea di separazione inferiore con Specular Hairline. Default: true.
    #[props(default = true)]
    pub border: bool,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Elenco dei menu ([`MenubarMenu`]) o azioni.
    pub children: Element,
}

/// Barra orizzontale dei menu in stile app desktop / IDE (File, Modifica, Visualizza, ecc.).
/// Supporta apertura al click e transizione rapida su hover se un menu è già aperto,
/// shortcut da tastiera visive e scala z-index standard.
#[component]
pub fn Menubar(props: MenubarProps) -> Element {
    let internal_active = use_signal(|| None::<String>);
    let active_sig = props.active_menu.unwrap_or(internal_active);

    use_context_provider(|| MenubarCtx {
        active_menu: active_sig,
    });

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("menubar").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("menubar", &props.class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let border_class = if props.border { "sd-menubar--border" } else { "" };

    rsx! {
        div {
            role: "menubar",
            "aria-label": "Barra dei menu principale",
            class: "sd-menubar {border_class} {base_override} {class_modifiers} {props.class}",
            {props.children}
        }
    }
}

/// Proprietà per il componente [`CommandBar`].
#[derive(Props, Clone, PartialEq)]
pub struct CommandBarProps {
    /// Barra dei menu integrata a sinistra ([`Menubar`]).
    #[props(default)]
    pub menubar: Option<Element>,
    /// Slot per widget di stato, palette shortcut o ricerca a destra.
    #[props(default)]
    pub action: Option<Element>,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
}

/// Barra di comando completa che combina una [`Menubar`] a sinistra e azioni/ricerca a destra.
#[component]
pub fn CommandBar(props: CommandBarProps) -> Element {
    rsx! {
        div { class: "sd-command-bar {props.class}",
            if let Some(ref mb) = props.menubar {
                div { class: "sd-command-bar__menubar",
                    {mb}
                }
            }
            if let Some(ref act) = props.action {
                div { class: "sd-command-bar__action",
                    {act}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menubar_props_defaults() {
        let props = MenubarProps {
            active_menu: None,
            border: true,
            class: "custom-bar".to_string(),
            children: rsx! {},
        };
        assert!(props.border);
        assert_eq!(props.class, "custom-bar");
    }

    #[test]
    fn test_command_bar_props() {
        let props = CommandBarProps {
            menubar: None,
            action: None,
            class: "test-cmd".to_string(),
        };
        assert_eq!(props.class, "test-cmd");
    }
}
