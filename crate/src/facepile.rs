use dioxus::prelude::*;

use crate::avatar::{Avatar, AvatarSize, PersonaProps};
use crate::icon::{Icon, IconKind};

/// Cluster compatto di avatar di collaboratori con disposizione sovrapposta e badge "+N altri".
#[component]
pub fn Facepile(
    personas: Vec<PersonaProps>,
    #[props(default = 4)] max_displayable: usize,
    #[props(default)] size: AvatarSize,
    #[props(default = true)] show_presence: bool,
    #[props(default)] on_add_click: Option<EventHandler<()>>,
    #[props(default)] on_persona_click: Option<EventHandler<PersonaProps>>,
    #[props(default)] class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let mut show_overflow_menu = use_signal(|| false);

    let total = personas.len();
    let (visible, overflow) = if total > max_displayable {
        let (v, o) = personas.split_at(max_displayable);
        (v.to_vec(), o.to_vec())
    } else {
        (personas, Vec::new())
    };

    let overflow_count = overflow.len();

    let interactive = on_persona_click.is_some();
    let item_class = if interactive {
        "sd-facepile__item sd-facepile__item--interactive"
    } else {
        "sd-facepile__item"
    };
    let item_role = if interactive { Some("button") } else { None };
    let item_tab = if interactive { Some("0") } else { None };
    let dim = format!("width: {px}px; height: {px}px;", px = size.px_size());

    rsx! {
        div { class: "sd-facepile {class}",
            div { class: "sd-avatar-group sd-facepile__stack",
                // Visible avatars (overlap comes from .sd-avatar-group)
                for (idx, p) in visible.into_iter().enumerate() {
                    {
                        let persona_clone = p.clone();
                        let persona_key = p.clone();
                        let title_text = if let Some(r) = &p.role {
                            format!("{} ({})", p.name, r)
                        } else {
                            p.name.clone()
                        };
                        rsx! {
                            div {
                                key: "{p.name}-{idx}",
                                class: "{item_class}",
                                role: item_role,
                                tabindex: item_tab,
                                title: "{title_text}",
                                onclick: {
                                    let p_c = persona_clone.clone();
                                    move |_| {
                                        if let Some(handler) = &on_persona_click {
                                            handler.call(p_c.clone());
                                        }
                                    }
                                },
                                onkeydown: move |evt: KeyboardEvent| {
                                    if evt.key() == Key::Enter || evt.key() == Key::Character(" ".to_string()) {
                                        evt.prevent_default();
                                        if let Some(handler) = &on_persona_click {
                                            handler.call(persona_key.clone());
                                        }
                                    }
                                },
                                Avatar {
                                    src: p.avatar_url,
                                    name: Some(p.name),
                                    size: size,
                                    status: if show_presence { p.status } else { None },
                                }
                            }
                        }
                    }
                }

                // "+N" overflow trigger
                if overflow_count > 0 {
                    div { class: "sd-facepile__overflow",
                        button {
                            r#type: "button",
                            class: "sd-facepile__overflow-btn",
                            style: "{dim}",
                            title: "{crate::locale::Locale::fmt(&locale.facepile_overflow_title, &[(\"n\", &overflow_count.to_string())])}",
                            "aria-expanded": "{show_overflow_menu()}",
                            "aria-haspopup": "true",
                            onclick: move |_| {
                                let curr = show_overflow_menu();
                                show_overflow_menu.set(!curr);
                            },
                            "+{overflow_count}"
                        }

                        if show_overflow_menu() {
                            div {
                                class: "sd-facepile__backdrop",
                                onclick: move |_| show_overflow_menu.set(false),
                            }
                            div {
                                class: "sd-facepile__overflow-menu",
                                onclick: move |e| e.stop_propagation(),

                                div { class: "sd-facepile__menu-heading",
                                    "{crate::locale::Locale::fmt(&locale.facepile_overflow_heading, &[(\"n\", &overflow_count.to_string())])}"
                                }

                                for p in overflow {
                                    {
                                        let p_c = p.clone();
                                        rsx! {
                                            button {
                                                key: "{p.name}",
                                                r#type: "button",
                                                class: "sd-facepile__menu-item",
                                                onclick: move |_| {
                                                    if let Some(handler) = &on_persona_click {
                                                        handler.call(p_c.clone());
                                                    }
                                                    show_overflow_menu.set(false);
                                                },
                                                Avatar {
                                                    src: p.avatar_url,
                                                    name: Some(p.name.clone()),
                                                    size: AvatarSize::Xs,
                                                    status: if show_presence { p.status } else { None },
                                                }
                                                span { class: "sd-facepile__menu-text",
                                                    span { class: "sd-facepile__menu-name", "{p.name}" }
                                                    if let Some(r) = &p.role {
                                                        span { class: "sd-facepile__menu-role", "{r}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Optional "+" add button
            if let Some(add_handler) = on_add_click {
                button {
                    r#type: "button",
                    class: "sd-facepile__add-btn",
                    style: "{dim}",
                    title: "{locale.facepile_add}",
                    onclick: move |_| add_handler.call(()),
                    Icon { kind: IconKind::Plus, size: (size.px_size() / 2).max(12) }
                }
            }
        }
    }
}
