use dioxus::prelude::*;

use crate::aspect_ratio::AspectRatio;
use crate::card::Card;

/// Clickable wrapper that gives a whole area the interactive-container treatment
/// (hover border, press, focus ring) with keyboard activation.
#[component]
pub fn CardActionArea(
    #[props(default)] on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let action_id = crate::a11y::use_unique_id("card-action", None);
    let aid_kd = action_id.clone();
    rsx! {
        div {
            id: "{action_id}",
            class: "sd-card-action-area {class}",
            role: "button",
            tabindex: if disabled { "-1" } else { "0" },
            "aria-disabled": if disabled { "true" } else { "false" },
            onclick: move |evt| {
                if !disabled {
                    if let Some(handler) = &on_click {
                        handler.call(evt);
                    }
                }
            },
            onkeydown: move |evt: KeyboardEvent| {
                if !disabled && (evt.key() == Key::Enter || evt.key() == Key::Character(" ".to_string())) {
                    evt.prevent_default();
                    let aid = aid_kd.clone();
                    let _ = document::eval(&format!(
                        r#"
                        const el = document.getElementById("{aid}");
                        if (el) el.click();
                        "#
                    ));
                }
            },
            {children}
        }
    }
}

/// Card with an immersive media header (video or image at a fixed aspect ratio).
/// Built on the library `Card`: it becomes interactive when `on_click` is set.
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
    let footer_el = footer.map(|f| rsx! { div { class: "sd-media-card__footer", {f} } });
    rsx! {
        Card {
            class: "sd-media-card {class}",
            overflow_hidden: true,
            on_click: on_click,
            footer: footer_el,

            div { class: "sd-media-card__media",
                AspectRatio { ratio: aspect_ratio,
                    if let Some(v_url) = &video_url {
                        video {
                            src: "{v_url}",
                            class: "sd-media-card__asset",
                            autoplay: true,
                            muted: true,
                            r#loop: true,
                            playsinline: true,
                        }
                    } else {
                        img {
                            src: "{image_url}",
                            alt: "{title.clone().unwrap_or_default()}",
                            class: "sd-media-card__asset",
                            loading: "lazy",
                        }
                    }
                }

                if let Some(b_text) = &badge {
                    span { class: "sd-media-card__badge", "{b_text}" }
                }
            }

            div { class: "sd-media-card__body",
                if let Some(t) = &title {
                    h3 { class: "sd-media-card__title", "{t}" }
                }
                if let Some(s) = &subtitle {
                    p { class: "sd-media-card__subtitle", "{s}" }
                }
                {children}
            }
        }
    }
}
