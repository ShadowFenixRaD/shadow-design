use dioxus::prelude::*;
use crate::icon::{Icon, IconKind, RenderSvgIcon, SvgIcon};
use crate::menubar::shortcut::MenubarShortcut;
use crate::menubar::MenubarCtx;

/// Proprietà della singola voce di menu [`MenubarItem`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarItemProps {
    /// Etichetta testuale della voce.
    #[props(default)]
    pub label: Option<String>,
    /// Icona vettoriale a sinistra.
    #[props(default)]
    pub icon: Option<SvgIcon>,
    /// Scorciatoia da tastiera visualizzata a destra (es. "Ctrl+S", "⌘N").
    #[props(default)]
    pub shortcut: Option<String>,
    /// Stato di spunta opzionale (None = voce normale, Some(bool) = checkbox item).
    #[props(default)]
    pub checked: Option<bool>,
    /// Disabilita la voce prevenendo interazioni.
    #[props(default)]
    pub disabled: bool,
    /// Callback invocata al click sulla voce.
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Contenuto personalizzato alternativo a `label`.
    #[props(default)]
    pub children: Option<Element>,
}

/// Singola voce interattiva all'interno di [`MenubarContent`].
#[component]
pub fn MenubarItem(props: MenubarItemProps) -> Element {
    let ctx = try_use_context::<MenubarCtx>();

    let disabled_class = if props.disabled { "sd-menubar__item--disabled" } else { "" };

    let check_indicator = match props.checked {
        Some(true) => rsx! {
            span { class: "sd-menubar__item-check",
                Icon { kind: IconKind::Check, size: 14 }
            }
        },
        Some(false) => rsx! {
            span { class: "sd-menubar__item-check sd-menubar__item-check--empty" }
        },
        None => rsx! {},
    };

    let icon_el = if let Some(ref ic) = props.icon {
        rsx! {
            span { class: "sd-menubar__item-icon",
                RenderSvgIcon { icon: ic.clone(), size: 15 }
            }
        }
    } else {
        rsx! {}
    };

    let shortcut_el = if let Some(ref sc) = props.shortcut {
        rsx! {
            MenubarShortcut { "{sc}" }
        }
    } else {
        rsx! {}
    };

    let label_content = if let Some(ref lbl) = props.label {
        rsx! { span { class: "sd-menubar__item-label", "{lbl}" } }
    } else if let Some(ref children) = props.children {
        rsx! { {children} }
    } else {
        rsx! {}
    };

    let on_click = move |evt: MouseEvent| {
        if !props.disabled {
            if let Some(ref handler) = props.on_click {
                handler.call(evt);
            }
            // Chiude il menu aperto
            if let Some(c) = ctx {
                let mut sig = c.active_menu;
                sig.set(None);
            }
        }
    };

    rsx! {
        button {
            r#type: "button",
            role: "menuitem",
            disabled: props.disabled,
            class: "sd-menubar__item {disabled_class} {props.class}",
            onclick: on_click,
            {check_indicator}
            {icon_el}
            {label_content}
            {shortcut_el}
        }
    }
}
