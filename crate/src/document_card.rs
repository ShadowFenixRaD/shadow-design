use dioxus::prelude::*;

use crate::avatar::{Avatar, AvatarSize, PersonaProps};
use crate::icon::{Icon, IconKind};

/// Scheda ricca per documenti e file con thumbnail preview, metadati autore e pulsanti azione.
#[component]
pub fn DocumentCard(
    file_name: String,
    file_type: String,
    #[props(default)] file_size: Option<String>,
    #[props(default)] preview_url: Option<String>,
    author: PersonaProps,
    #[props(default)] views: Option<usize>,
    modified_at: String,
    #[props(default)] on_download: Option<EventHandler<MouseEvent>>,
    #[props(default)] on_share: Option<EventHandler<MouseEvent>>,
    #[props(default)] on_delete: Option<EventHandler<MouseEvent>>,
    #[props(default)] on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)] class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let ftype = file_type.to_uppercase();

    // Semantic tone per file type (styled by `sd-doc-card__badge--<tone>`)
    let tone = match ftype.as_str() {
        "PDF" => "danger",
        "DOCX" | "DOC" => "info",
        "XLSX" | "XLS" | "CSV" => "success",
        "RS" | "TS" | "JS" | "PY" => "accent",
        "ZIP" | "TAR" | "GZ" => "warning",
        _ => "neutral",
    };

    let has_actions = on_download.is_some() || on_share.is_some() || on_delete.is_some();

    rsx! {
        crate::card::Card {
            class: "sd-doc-card {class}",
            overflow_hidden: true,
            on_click: on_click,

            // Area Thumbnail o Icona File
            div { class: "sd-doc-card__preview",
                if let Some(url) = &preview_url {
                    img {
                        src: "{url}",
                        alt: "{file_name}",
                    }
                    span { class: "sd-doc-card__badge sd-doc-card__badge--{tone}", "{ftype}" }
                } else {
                    div { class: "sd-doc-card__center-icon",
                        Icon { kind: IconKind::Paperclip, size: 36, class: "sd-doc-card__icon" }
                        span { class: "sd-doc-card__badge sd-doc-card__badge--inline sd-doc-card__badge--{tone}", "{ftype}" }
                    }
                }

                // Barra azioni rapide posizionata con precisione in alto a destra
                if has_actions {
                    div {
                        class: "sd-doc-card__actions",
                        onclick: move |e| e.stop_propagation(),

                        if let Some(d_h) = on_download {
                            button {
                                r#type: "button",
                                class: "sd-doc-card__action-btn",
                                title: "{locale.download}",
                                onclick: move |evt| d_h.call(evt),
                                Icon { kind: IconKind::Download, size: 14 }
                            }
                        }
                        if let Some(s_h) = on_share {
                            button {
                                r#type: "button",
                                class: "sd-doc-card__action-btn",
                                title: "{locale.share}",
                                onclick: move |evt| s_h.call(evt),
                                Icon { kind: IconKind::Share2, size: 14 }
                            }
                        }
                        if let Some(del_h) = on_delete {
                            button {
                                r#type: "button",
                                class: "sd-doc-card__action-btn sd-doc-card__action-btn--delete",
                                title: "{locale.delete}",
                                onclick: move |evt| del_h.call(evt),
                                Icon { kind: IconKind::Trash2, size: 14 }
                            }
                        }
                    }
                }
            }

            // Metadati e dettagli documento
            div { class: "sd-doc-card__body",
                div { class: "sd-doc-card__title-row",
                    h4 {
                        class: "sd-doc-card__title",
                        title: "{file_name}",
                        "{file_name}"
                    }
                }

                // Autore, dimensione e data modifica
                div { class: "sd-doc-card__footer",
                    div { class: "sd-doc-card__author",
                        Avatar {
                            src: author.avatar_url.clone(),
                            name: Some(author.name.clone()),
                            size: AvatarSize::Xs,
                            status: author.status,
                        }
                        span { class: "sd-doc-card__author-name", "{author.name}" }
                    }

                    div { class: "sd-doc-card__meta",
                        if let Some(sz) = &file_size {
                            span { "{sz}" }
                        }
                        if let Some(v) = views {
                            span { class: "sd-doc-card__views",
                                Icon { kind: IconKind::Eye, size: 12 }
                                "{v}"
                            }
                        }
                        span { "{modified_at}" }
                    }
                }
            }
        }
    }
}
