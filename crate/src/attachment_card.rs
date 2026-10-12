use dioxus::prelude::*;

#[cfg(feature = "icon")]
use crate::icon::{Icon, IconKind, RenderSvgIcon, SvgIcon};
#[cfg(feature = "progress")]
use crate::progress::{ProgressBar, ProgressSize};

/// Scheda per allegati file, documenti, media o profili utente in chat e conversazioni.
/// Struttura:
/// - Avatar tondo sulla sinistra
/// - Nome file o utente sulla destra
/// - Sotto: descrizione opzionale oppure bottoni per scaricare, condividere e gestire l'allegato.
#[component]
pub fn AttachmentCard(
    title: String,
    #[props(default)] subtitle: Option<String>,
    #[props(default)] description: Option<String>,
    #[props(default)] avatar: Option<Element>,
    #[props(default)] avatar_src: Option<String>,
    #[cfg(feature = "icon")]
    #[props(default)] avatar_icon: Option<SvgIcon>,
    #[props(default)] avatar_name: Option<String>,
    #[props(default)] progress: Option<f32>,
    #[props(default)] actions: Option<Element>,
    #[props(default)] on_download: Option<EventHandler<MouseEvent>>,
    #[props(default)] on_share: Option<EventHandler<MouseEvent>>,
    #[props(default)] on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)] class: String,
) -> Element {
    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("attachment_card").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("attachment_card", &class);
    #[cfg(feature = "theme")]
    let radius = theme_val.radius_scale.sm();

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let radius = "";

    let has_actions = actions.is_some() || on_download.is_some() || on_share.is_some();

    rsx! {
        div {
            class: "sd-attachment-card {radius} {base_override} {class_modifiers} {class}",
            onclick: move |evt| {
                if let Some(handler) = &on_click {
                    handler.call(evt);
                }
            },

            div {
                class: "sd-attachment-card__top",

                // Avatar tondo sulla sinistra
                if let Some(custom_avatar) = avatar {
                    div {
                        class: "sd-attachment-card__avatar-custom shrink-0",
                        {custom_avatar}
                    }
                } else {
                    div {
                        class: "sd-attachment-card__avatar",
                        if let Some(src) = avatar_src {
                            img {
                                src: "{src}",
                                alt: "{title}",
                                class: "sd-attachment-card__avatar-img",
                            }
                        } else if let Some(custom_icon) = avatar_icon {
                            RenderSvgIcon { icon: custom_icon, size: 20 }
                        } else {
                            Icon { kind: IconKind::Paperclip, size: 20 }
                        }
                    }
                }

                // Intestazione con titolo e sottotitolo sulla destra
                div {
                    class: "sd-attachment-card__info",
                    h4 {
                        class: "sd-attachment-card__title",
                        "{title}"
                    }
                    if let Some(sub) = subtitle {
                        p {
                            class: "sd-attachment-card__subtitle",
                            "{sub}"
                        }
                    }
                }
            }

            // Barra di avanzamento opzionale se il file è in transito/upload
            if let Some(prog_val) = progress {
                div {
                    class: "sd-attachment-card__progress",
                    ProgressBar {
                        value: prog_val,
                        size: ProgressSize::Sm,
                        show_label: true,
                    }
                }
            }

            // Descrizione opzionale al di sotto
            if let Some(desc) = description {
                p {
                    class: "sd-attachment-card__desc",
                    "{desc}"
                }
            }

            // Bottoni per scaricare, condividere e gestire l'allegato
            if has_actions {
                div {
                    class: "sd-attachment-card__actions",
                    onclick: move |evt| evt.stop_propagation(),

                    if let Some(dl_handler) = on_download {
                        button {
                            r#type: "button",
                            class: "sd-attachment-card__action-btn",
                            onclick: move |evt| dl_handler.call(evt),
                            Icon { kind: IconKind::Download, size: 14 }
                            span { "Scarica" }
                        }
                    }

                    if let Some(sh_handler) = on_share {
                        button {
                            r#type: "button",
                            class: "sd-attachment-card__action-btn",
                            onclick: move |evt| sh_handler.call(evt),
                            Icon { kind: IconKind::Share2, size: 14 }
                            span { "Condividi" }
                        }
                    }

                    if let Some(extra_actions) = actions {
                        {extra_actions}
                    }
                }
            }
        }
    }
}
