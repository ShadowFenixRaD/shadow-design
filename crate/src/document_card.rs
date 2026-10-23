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
    let ftype = file_type.to_uppercase();

    // Badge cromatico per tipo di file
    let type_badge_color = match ftype.as_str() {
        "PDF" => "bg-red-500/20 text-red-300 border-red-500/30",
        "DOCX" | "DOC" => "bg-blue-500/20 text-blue-300 border-blue-500/30",
        "XLSX" | "XLS" | "CSV" => "bg-emerald-500/20 text-emerald-300 border-emerald-500/30",
        "RS" | "TS" | "JS" | "PY" => "bg-purple-500/20 text-purple-300 border-purple-500/30",
        "ZIP" | "TAR" | "GZ" => "bg-amber-500/20 text-amber-300 border-amber-500/30",
        _ => "bg-zinc-800 text-zinc-300 border-zinc-700",
    };

    rsx! {
        div {
            class: "sd-doc-card group relative flex flex-col rounded-xl border border-white/[0.12] bg-zinc-950/70 overflow-hidden shadow-md hover:shadow-xl hover:border-purple-500/40 transition-all duration-200 cursor-pointer {class}",
            onclick: move |evt| {
                if let Some(handler) = &on_click {
                    handler.call(evt);
                }
            },

            // Area Thumbnail o Icona File
            div { class: "sd-doc-card__preview relative w-full h-36 bg-zinc-900/80 border-b border-white/[0.08] flex items-center justify-center overflow-hidden",
                if let Some(url) = &preview_url {
                    img {
                        src: "{url}",
                        alt: "{file_name}",
                        class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-300",
                    }
                } else {
                    div { class: "flex flex-col items-center gap-2",
                        Icon { kind: IconKind::Paperclip, size: 36, class: "text-zinc-500 group-hover:text-purple-400 transition-colors" }
                        span { class: "text-[11px] font-mono px-2 py-0.5 rounded border {type_badge_color}",
                            "{ftype}"
                        }
                    }
                }

                if preview_url.is_some() {
                    span { class: "absolute top-2.5 left-2.5 text-[10px] font-mono font-bold px-2 py-0.5 rounded border backdrop-blur-md {type_badge_color}",
                        "{ftype}"
                    }
                }

                // Azioni rapide fluttuanti in hover
                div {
                    class: "absolute top-2.5 right-2.5 flex items-center gap-1.5 opacity-0 group-hover:opacity-100 transition-opacity bg-black/70 backdrop-blur-md p-1 rounded-lg border border-white/10",
                    onclick: move |e| e.stop_propagation(),

                    if let Some(d_h) = on_download {
                        button {
                            r#type: "button",
                            class: "p-1.5 rounded-md hover:bg-white/10 text-zinc-300 hover:text-white transition-colors",
                            title: "Download",
                            onclick: move |evt| d_h.call(evt),
                            Icon { kind: IconKind::Download, size: 14 }
                        }
                    }
                    if let Some(s_h) = on_share {
                        button {
                            r#type: "button",
                            class: "p-1.5 rounded-md hover:bg-white/10 text-zinc-300 hover:text-white transition-colors",
                            title: "Condividi",
                            onclick: move |evt| s_h.call(evt),
                            Icon { kind: IconKind::Share2, size: 14 }
                        }
                    }
                    if let Some(del_h) = on_delete {
                        button {
                            r#type: "button",
                            class: "p-1.5 rounded-md hover:bg-red-500/20 text-zinc-300 hover:text-red-300 transition-colors",
                            title: "Elimina",
                            onclick: move |evt| del_h.call(evt),
                            Icon { kind: IconKind::Trash2, size: 14 }
                        }
                    }
                }
            }

            // Metadati e dettagli
            div { class: "p-4 flex flex-col gap-3",
                div { class: "flex items-start justify-between gap-2",
                    h4 { class: "text-sm font-semibold text-zinc-100 group-hover:text-purple-300 transition-colors line-clamp-1",
                        "{file_name}"
                    }
                }

                // Autore e data modifica
                div { class: "flex items-center justify-between text-xs text-zinc-400 font-mono pt-2 border-t border-white/[0.06]",
                    div { class: "flex items-center gap-2",
                        Avatar {
                            src: author.avatar_url.clone(),
                            name: Some(author.name.clone()),
                            size: AvatarSize::Xs,
                            status: author.status,
                        }
                        span { class: "truncate max-w-[100px]", "{author.name}" }
                    }

                    div { class: "flex items-center gap-3 shrink-0",
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
