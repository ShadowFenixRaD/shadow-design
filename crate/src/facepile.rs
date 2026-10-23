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
    let mut show_overflow_menu = use_signal(|| false);

    let total = personas.len();
    let (visible, overflow) = if total > max_displayable {
        let (v, o) = personas.split_at(max_displayable);
        (v.to_vec(), o.to_vec())
    } else {
        (personas, Vec::new())
    };

    let overflow_count = overflow.len();

    rsx! {
        div { class: "sd-facepile relative inline-flex items-center select-none {class}",
            // Avatar visibili sovrapposti
            for (idx, p) in visible.into_iter().enumerate() {
                {
                    let persona_clone = p.clone();
                    let margin_style = if idx > 0 { "margin-left: -10px;" } else { "" };
                    let title_text = if let Some(r) = &p.role {
                        format!("{} ({})", p.name, r)
                    } else {
                        p.name.clone()
                    };
                    rsx! {
                        div {
                            key: "{p.name}-{idx}",
                            class: "relative rounded-full ring-2 ring-black hover:z-10 hover:scale-105 transition-all cursor-pointer",
                            style: "{margin_style}",
                            title: "{title_text}",
                            onclick: {
                                let p_c = persona_clone.clone();
                                move |_| {
                                    if let Some(handler) = &on_persona_click {
                                        handler.call(p_c.clone());
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

            // Pulsante "+N altri" se ci sono membri in eccesso
            if overflow_count > 0 {
                div { class: "relative", style: "margin-left: -10px;",
                    button {
                        r#type: "button",
                        class: "sd-facepile__overflow-btn flex items-center justify-center rounded-full bg-zinc-800 hover:bg-zinc-700 text-zinc-200 text-xs font-mono font-bold ring-2 ring-black shadow-md cursor-pointer transition-colors z-0 hover:z-10",
                        style: "width: {size.px_size()}px; height: {size.px_size()}px;",
                        title: "Visualizza tutti i {overflow_count} collaboratori aggiuntivi",
                        onclick: move |_| {
                            let curr = show_overflow_menu();
                            show_overflow_menu.set(!curr);
                        },
                        "+{overflow_count}"
                    }

                    // Tendina elenco membri in eccesso
                    if show_overflow_menu() {
                        div {
                            class: "fixed inset-0 z-[994]",
                            onclick: move |_| show_overflow_menu.set(false),
                        }
                        div {
                            class: "sd-facepile__overflow-menu absolute top-full left-0 mt-2 z-[995] w-64 p-2 rounded-xl bg-zinc-950 border border-white/[0.12] shadow-2xl backdrop-blur-xl animate-scale-in flex flex-col gap-1 max-h-64 overflow-y-auto",
                            onclick: move |e| e.stop_propagation(),

                            div { class: "px-2 py-1 text-[11px] font-mono text-zinc-400 font-bold border-b border-white/[0.06] mb-1",
                                "Altri collaboratori ({overflow_count})"
                            }

                            for p in overflow {
                                {
                                    let p_c = p.clone();
                                    rsx! {
                                        div {
                                            key: "{p.name}",
                                            class: "flex items-center gap-2.5 p-2 rounded-lg hover:bg-white/[0.05] transition-colors cursor-pointer",
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
                                            div { class: "flex flex-col min-w-0 flex-1",
                                                span { class: "text-xs font-medium text-zinc-200 truncate", "{p.name}" }
                                                if let Some(r) = &p.role {
                                                    span { class: "text-[10px] text-zinc-400 truncate", "{r}" }
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

            // Pulsante circolare opzionale "+ Aggiungi"
            if let Some(add_handler) = on_add_click {
                button {
                    r#type: "button",
                    class: "sd-facepile__add-btn ml-2 flex items-center justify-center rounded-full border border-dashed border-white/20 hover:border-purple-500/60 bg-white/[0.02] hover:bg-purple-600/20 text-zinc-400 hover:text-purple-300 transition-all cursor-pointer",
                    style: "width: {size.px_size()}px; height: {size.px_size()}px;",
                    title: "Aggiungi nuovo membro",
                    onclick: move |_| add_handler.call(()),
                    Icon { kind: IconKind::Plus, size: (size.px_size() / 2).max(12) }
                }
            }
        }
    }
}
