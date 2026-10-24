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

    // Stile cromatico badge per estensione/tipo di file
    let type_badge_style = match ftype.as_str() {
        "PDF" => "color: #fca5a5; background: rgba(239, 68, 68, 0.2); border-color: rgba(239, 68, 68, 0.4);",
        "DOCX" | "DOC" => "color: #93c5fd; background: rgba(59, 130, 246, 0.2); border-color: rgba(59, 130, 246, 0.4);",
        "XLSX" | "XLS" | "CSV" => "color: #6ee7b7; background: rgba(16, 185, 129, 0.2); border-color: rgba(16, 185, 129, 0.4);",
        "RS" | "TS" | "JS" | "PY" => "color: #d8b4fe; background: rgba(168, 85, 247, 0.2); border-color: rgba(168, 85, 247, 0.4);",
        "ZIP" | "TAR" | "GZ" => "color: #fcd34d; background: rgba(245, 158, 11, 0.2); border-color: rgba(245, 158, 11, 0.4);",
        _ => "color: #d4d4d8; background: rgba(39, 39, 42, 0.8); border-color: rgba(82, 82, 91, 0.5);",
    };

    let has_actions = on_download.is_some() || on_share.is_some() || on_delete.is_some();

    rsx! {
        div {
            class: "sd-doc-card group {class}",
            onclick: move |evt| {
                if let Some(handler) = &on_click {
                    handler.call(evt);
                }
            },

            // Area Thumbnail o Icona File
            div { class: "sd-doc-card__preview",
                if let Some(url) = &preview_url {
                    img {
                        src: "{url}",
                        alt: "{file_name}",
                    }
                    span {
                        class: "sd-doc-card__badge",
                        style: "{type_badge_style}",
                        "{ftype}"
                    }
                } else {
                    div { class: "sd-doc-card__center-icon",
                        Icon { kind: IconKind::Paperclip, size: 36, class: "text-zinc-500" }
                        span {
                            class: "text-[11px] font-mono px-2 py-0.5 rounded border",
                            style: "{type_badge_style}",
                            "{ftype}"
                        }
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
                        span { class: "truncate max-w-[100px]", "{author.name}" }
                    }

                    div { class: "sd-doc-card__meta",
                        if let Some(sz) = &file_size {
                            span { "{sz}" }
                        }
                        if let Some(v) = views {
                            span { class: "flex items-center gap-1",
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
