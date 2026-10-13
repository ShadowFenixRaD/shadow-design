use dioxus::prelude::*;

/// Scheda contatto specializzata costruita sul motore `Card`.
/// Offre una API focalizzata e minimale per visualizzare profili, utenti o nodi di rete:
/// - `name`: Nome del contatto o nodo (titolo della card)
/// - `avatar`: Elemento avatar opzionale (con fallback automatico ad `Avatar` con iniziali su HSL hash)
/// - `subtitle`: Sottotitolo opzionale (ruolo, handle, presenza o metadati di rete)
/// - `description`: Descrizione testuale opzionale
/// - `children`: Corpo RSX opzionale per ulteriori informazioni o badge
/// - `on_click`: Callback di interazione al click
#[component]
pub fn ContactCard(
    name: String,
    #[props(default)] avatar: Option<Element>,
    #[props(default)] subtitle: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)] class: String,
    #[props(default)] children: Element,
) -> Element {
    let resolved_avatar = if let Some(custom_av) = avatar {
        Some(custom_av)
    } else {
        Some(rsx! {
            crate::avatar::Avatar {
                name: Some(name.clone()),
                size: crate::avatar::AvatarSize::Md,
            }
        })
    };

    rsx! {
        crate::card::Card {
            class: "sd-contact-card {class}",
            avatar: resolved_avatar,
            title: Some(name),
            subtitle: subtitle,
            on_click: on_click,

            if let Some(desc) = description {
                p { class: "sd-contact-card__desc text-xs text-zinc-300 leading-relaxed mb-1", "{desc}" }
            }
            {children}
        }
    }
}
