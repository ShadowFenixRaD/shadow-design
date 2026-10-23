use dioxus::prelude::*;
use crate::menubar::MenubarCtx;

/// Proprietà del menu a discesa [`MenubarMenu`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarMenuProps {
    /// Identificatore univoco del menu (es. "file", "edit", "view"). Se omesso, viene generato automaticamente.
    #[props(default)]
    pub id: Option<String>,
    /// Trigger e contenuto del menu ([`MenubarTrigger`] e [`MenubarContent`]).
    pub children: Element,
}

/// Contesto condiviso per il singolo menu.
#[derive(Clone, PartialEq)]
pub struct SingleMenuCtx {
    pub menu_id: String,
}

/// Contenitore per un singolo menu a tendina all'interno della [`Menubar`].
#[component]
pub fn MenubarMenu(props: MenubarMenuProps) -> Element {
    let menu_id = crate::a11y::use_unique_id("sd-menu", props.id);

    use_context_provider(|| SingleMenuCtx {
        menu_id: menu_id.clone(),
    });

    rsx! {
        div { class: "sd-menubar__menu",
            {props.children}
        }
    }
}

/// Proprietà del pulsante di attivazione [`MenubarTrigger`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarTriggerProps {
    /// Disabilita il trigger.
    #[props(default)]
    pub disabled: bool,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Etichetta o contenuto del trigger.
    pub children: Element,
}

/// Pulsante visibile nella barra orizzontale che controlla l'apertura del menu.
#[component]
pub fn MenubarTrigger(props: MenubarTriggerProps) -> Element {
    let menubar_ctx = try_use_context::<MenubarCtx>();
    let single_ctx = try_use_context::<SingleMenuCtx>();

    let menu_id = single_ctx.map(|c| c.menu_id).unwrap_or_default();
    let is_open = menubar_ctx.as_ref().and_then(|c| (c.active_menu)().as_ref().map(|id| id == &menu_id)).unwrap_or(false);
    let is_any_open = menubar_ctx.as_ref().map(|c| (c.active_menu)().is_some()).unwrap_or(false);

    let active_class = if is_open { "sd-menubar__trigger--active" } else { "" };
    let disabled_class = if props.disabled { "sd-menubar__trigger--disabled" } else { "" };

    let menu_id_toggle = menu_id.clone();
    let on_click = move |_| {
        if !props.disabled {
            if let Some(mut c) = menubar_ctx {
                let current = (c.active_menu)();
                if current.as_deref() == Some(&menu_id_toggle) {
                    c.active_menu.set(None);
                } else {
                    c.active_menu.set(Some(menu_id_toggle.clone()));
                }
            }
        }
    };

    let menu_id_hover = menu_id.clone();
    let on_mouseenter = move |_| {
        if !props.disabled && is_any_open {
            if let Some(mut c) = menubar_ctx {
                c.active_menu.set(Some(menu_id_hover.clone()));
            }
        }
    };

    rsx! {
        button {
            r#type: "button",
            role: "menuitem",
            "aria-haspopup": "menu",
            "aria-expanded": if is_open { "true" } else { "false" },
            disabled: props.disabled,
            class: "sd-menubar__trigger {active_class} {disabled_class} {props.class}",
            onclick: on_click,
            onmouseenter: on_mouseenter,
            onkeydown: move |e: KeyboardEvent| {
                if e.key() == Key::Escape {
                    if let Some(c) = menubar_ctx {
                        let mut sig = c.active_menu;
                        sig.set(None);
                    }
                }
            },
            {props.children}
        }
    }
}

/// Proprietà del pannello a comparsa [`MenubarContent`].
#[derive(Props, Clone, PartialEq)]
pub struct MenubarContentProps {
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
    /// Voci del menu ([`MenubarItem`], [`MenubarSeparator`]).
    pub children: Element,
}

/// Finestra a tendina fluttuante aperta da [`MenubarTrigger`].
#[component]
pub fn MenubarContent(props: MenubarContentProps) -> Element {
    let menubar_ctx = try_use_context::<MenubarCtx>();
    let single_ctx = try_use_context::<SingleMenuCtx>();

    let menu_id = single_ctx.map(|c| c.menu_id).unwrap_or_default();
    let is_open = menubar_ctx.as_ref().and_then(|c| (c.active_menu)().as_ref().map(|id| id == &menu_id)).unwrap_or(false);

    if is_open {
        let close_ctx = menubar_ctx;
        rsx! {
            // Backdrop invisibile a z-[995] per catturare click esterni e chiudere il menu
            div {
                class: "sd-menubar__backdrop",
                onclick: move |_| {
                    if let Some(c) = close_ctx {
                        let mut sig = c.active_menu;
                        sig.set(None);
                    }
                },
            }
            // Pannello menu a comparsa
            div {
                role: "menu",
                class: "sd-menubar__content {props.class}",
                "aria-orientation": "vertical",
                onkeydown: move |e: KeyboardEvent| {
                    if e.key() == Key::Escape {
                        if let Some(c) = close_ctx {
                            let mut sig = c.active_menu;
                            sig.set(None);
                        }
                    }
                },
                {props.children}
            }
        }
    } else {
        rsx! {}
    }
}
