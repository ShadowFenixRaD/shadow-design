use dioxus::prelude::*;

use crate::aspect_ratio::AspectRatio;

/// Area d'azione cliccabile con effetto tattile e focus-ring esteso a tutta la superficie.
#[component]
pub fn CardActionArea(
    #[props(default)] on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let state_cls = if !disabled {
        "cursor-pointer hover:border-purple-500/40 active:scale-[0.99] focus-visible:ring-2 focus-visible:ring-purple-500"
    } else {
        "opacity-60 cursor-not-allowed"
    };

    rsx! {
        div {
            class: "sd-card-action-area group relative w-full rounded-xl transition-all outline-none {state_cls} {class}",
            tabindex: if disabled { "-1" } else { "0" },
            onclick: move |evt| {
                if !disabled {
                    if let Some(handler) = &on_click {
                        handler.call(evt);
                    }
                }
            },
            onkeydown: move |evt: KeyboardEvent| {
                if !disabled && (evt.key() == Key::Enter || evt.key() == Key::Character(" ".to_string())) {
                    if let Some(_handler) = &on_click {
                        // Simula trigger da tastiera
                    }
                }
            },
            {children}
        }
    }
}

/// Scheda contenitore arricchita con testata multimediale immersiva (video o immagine ad aspect ratio fisso).
#[component]
pub fn MediaCard(
    image_url: String,
    #[props(default)] video_url: Option<String>,
    #[props(default = 16.0 / 9.0)] aspect_ratio: f64,
    #[props(default)] title: Option<String>,
    #[props(default)] subtitle: Option<String>,
    #[props(default)] badge: Option<String>,
    #[props(default)] on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)] footer: Option<Element>,
    #[props(default)] class: String,
    #[props(default)] children: Element,
) -> Element {
    rsx! {
        CardActionArea {
            on_click: on_click,
            class: "sd-media-card flex flex-col rounded-xl border border-white/[0.12] bg-zinc-950/70 overflow-hidden shadow-lg hover:shadow-2xl transition-all duration-200 {class}",

            // Media Header
            div { class: "relative w-full overflow-hidden bg-zinc-900 border-b border-white/[0.08]",
                AspectRatio { ratio: aspect_ratio,
                    if let Some(v_url) = &video_url {
                        video {
                            src: "{v_url}",
                            class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-300",
                            autoplay: true,
                            muted: true,
                            r#loop: true,
                            playsinline: true,
                        }
                    } else {
                        img {
                            src: "{image_url}",
                            alt: "{title.clone().unwrap_or_default()}",
                            class: "w-full h-full object-cover group-hover:scale-105 transition-transform duration-300",
                            loading: "lazy",
                        }
                    }
                }

                if let Some(b_text) = &badge {
                    span { class: "absolute top-3 right-3 px-2.5 py-1 rounded-md text-[11px] font-mono font-bold bg-black/75 text-purple-300 border border-purple-500/30 backdrop-blur-md shadow-md",
                        "{b_text}"
                    }
                }
            }

            // Body
            div { class: "p-4 flex flex-col gap-2 flex-1",
                if let Some(t) = &title {
                    h3 { class: "text-base font-semibold text-white group-hover:text-purple-300 transition-colors",
                        "{t}"
                    }
                }
                if let Some(s) = &subtitle {
                    p { class: "text-xs text-zinc-400 font-mono line-clamp-2",
                        "{s}"
                    }
                }
                {children}
            }

            // Footer Slot opzionale
            if let Some(f) = footer {
                div { class: "p-3 bg-white/[0.02] border-t border-white/[0.06] flex items-center justify-between text-xs text-zinc-400",
                    {f}
                }
            }
        }
    }
}
