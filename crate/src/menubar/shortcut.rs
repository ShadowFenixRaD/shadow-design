use dioxus::prelude::*;

/// Scorciatoia visiva da tastiera allineata a destra in un [`MenubarItem`].
#[component]
pub fn MenubarShortcut(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        span { class: "sd-menubar__shortcut {class}",
            {children}
        }
    }
}

/// Separatore orizzontale all'interno di un [`MenubarContent`].
#[component]
pub fn MenubarSeparator(#[props(default)] class: String) -> Element {
    rsx! {
        div {
            role: "separator",
            class: "sd-menubar__separator {class}",
            "aria-orientation": "horizontal",
        }
    }
}
