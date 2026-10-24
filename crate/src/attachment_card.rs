use dioxus::prelude::*;
use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::icon::{Icon, IconKind, RenderSvgIcon, SvgIcon};

/// Scheda allegato file specializzata costruita sul motore unificato `Card`.
/// Offre una API pulita, focalizzata e potente per visualizzare allegati e documenti in chat:
/// - `title`: Nome del file o documento
/// - `avatar`: Elemento avatar o icona (fallback a thumbnail `avatar_src`, icona o paperclip)
/// - `subtitle`: Sottotitolo con tipo e dimensione del file (es. "PDF • 2.4 MB")
/// - `image`: Immagine facoltativa a tutta larghezza in basso con ritaglio ai bordi
/// - `progress`: Avanzamento di trasferimento facoltativo (0.0 .. 100.0)
/// - `description`: Descrizione o testo opzionale
/// - `on_download`: Callback per generare il bottone di download
/// - `on_share`: Callback per generare il bottone di condivisione
/// - `buttons`: Slot aggiuntivo per azioni o pulsanti personalizzati
#[component]
pub fn AttachmentCard(
    title: String,
    #[props(default)] subtitle: Option<String>,
    #[props(default)] avatar: Option<Element>,
    #[props(default)] avatar_src: Option<String>,
    #[props(default)] avatar_icon: Option<SvgIcon>,
    #[props(default)] image: Option<String>,
    #[props(default)] image_alt: Option<String>,
    #[props(default)] progress: Option<f32>,
    #[props(default)] description: Option<String>,
    #[props(default)] on_download: Option<EventHandler<MouseEvent>>,
    #[props(default)] on_share: Option<EventHandler<MouseEvent>>,
    #[props(default)] buttons: Option<Element>,
    #[props(default)] actions: Option<Element>,
    #[props(default)] on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)] class: String,
    #[props(default)] children: Element,
) -> Element {
    let locale = crate::locale::use_locale();
    let resolved_avatar = if let Some(custom_av) = avatar {
        Some(custom_av)
    } else if let Some(src) = avatar_src {
        Some(rsx! {
            div {
                class: "sd-attachment-card__avatar sd-attachment-card__avatar--image",
                img {
                    src: "{src}",
                    alt: "{title}",
                    class: "sd-attachment-card__avatar-img",
                }
            }
        })
    } else if let Some(ic) = avatar_icon {
        Some(rsx! {
            div {
                class: "sd-attachment-card__avatar",
                RenderSvgIcon { icon: ic, size: 18 }
            }
        })
    } else {
        Some(rsx! {
            div {
                class: "sd-attachment-card__avatar",
                Icon { kind: IconKind::Paperclip, size: 18 }
            }
        })
    };

    let has_buttons = on_download.is_some() || on_share.is_some() || buttons.is_some() || actions.is_some();
    let buttons_element = if has_buttons {
        Some(rsx! {
            if let Some(dl) = on_download {
                Button {
                    size: ButtonSize::Sm,
                    variant: ButtonVariant::Outline,
                    left_icon: Some(rsx! { Icon { kind: IconKind::Download, size: 14 } }),
                    onclick: move |evt| dl.call(evt),
                    "{locale.download}"
                }
            }
            if let Some(sh) = on_share {
                Button {
                    size: ButtonSize::Sm,
                    variant: ButtonVariant::Ghost,
                    left_icon: Some(rsx! { Icon { kind: IconKind::Share2, size: 14 } }),
                    onclick: move |evt| sh.call(evt),
                    "{locale.share}"
                }
            }
            if let Some(custom_btns) = buttons {
                {custom_btns}
            }
            if let Some(legacy_actions) = actions {
                {legacy_actions}
            }
        })
    } else {
        None
    };

    rsx! {
        crate::card::Card {
            class: "sd-attachment-card {class}",
            avatar: resolved_avatar,
            title: Some(title),
            subtitle: subtitle,
            progress: progress,
            buttons: buttons_element,
            image: image,
            image_alt: image_alt,
            on_click: on_click,

            if let Some(desc) = description {
                p { class: "sd-attachment-card__desc", "{desc}" }
            }
            {children}
        }
    }
}
