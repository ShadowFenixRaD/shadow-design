use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn AvatarsPage() -> Element {
    let mut selected_size = use_signal(|| AvatarSize::Lg);
    let mut selected_status = use_signal(|| Some(PresenceStatus::DirectP2P));
    let mut user_name = use_signal(|| "Satoshi Nakamoto".to_string());
    let with_image = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "name",
            r#type: "Option<String>",
            default: "None",
            description: "Nome utente da cui vengono ricavate le iniziali e un gradiente HSL deterministico",
        },
        PropRow {
            name: "src",
            r#type: "Option<String>",
            default: "None",
            description: "URL opzionale dell'immagine profilo",
        },
        PropRow {
            name: "size",
            r#type: "AvatarSize",
            default: "AvatarSize::Md",
            description: "Dimensione dell'avatar (Xs: 24px, Sm: 32px, Md: 40px, Lg: 48px, Xl: 64px)",
        },
        PropRow {
            name: "status",
            r#type: "Option<PresenceStatus>",
            default: "None",
            description: "Indicatore di presenza (Online, Offline, Busy, DirectP2P con micro-pulsazione neon)",
        },
    ];

    let code_snippet = r#"Avatar {
    name: "Satoshi Nakamoto".to_string(),
    size: AvatarSize::Lg,
    status: PresenceStatus::DirectP2P,
}"#
    .to_string();

    let image_url = if with_image() {
        Some("https://images.unsplash.com/photo-1535713875002-d1d0cf377fde?auto=format&fit=crop&w=160&q=80".to_string())
    } else {
        None
    };

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Avatar & Presenza Utente" }
                Text { dimmed: true, size: TextSize::Sm, "Rappresentazione del profilo, rubrica e nodi mesh con fallback ad iniziali su gradiente matematico e stato di connettività real-time." }
            }

            FeatureBanner { feature_name: "avatar" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-center",
                        // Live Preview
                        div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col items-center justify-center gap-6 min-h-[180px] shadow-inner",
                            Avatar {
                                name: Some(user_name()),
                                src: image_url,
                                size: selected_size(),
                                status: selected_status(),
                            }

                            div { class: "flex items-center gap-2 text-xs text-zinc-400 font-mono",
                                span { "Anteprima AvatarGroup:" }
                                AvatarGroup {
                                    Avatar { name: Some("Elena Rostova".to_string()), size: AvatarSize::Sm }
                                    Avatar { name: Some("Marco Polo".to_string()), size: AvatarSize::Sm }
                                    Avatar { name: Some("Alice Vance".to_string()), size: AvatarSize::Sm }
                                    Avatar { name: Some("Direct Node".to_string()), size: AvatarSize::Sm, status: PresenceStatus::Online }
                                }
                            }
                        }

                        // Configurator
                        div { class: "space-y-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Dimensione Avatar:" }
                                div { class: "flex flex-wrap gap-1.5",
                                    for s in [AvatarSize::Xs, AvatarSize::Sm, AvatarSize::Md, AvatarSize::Lg, AvatarSize::Xl] {
                                        button {
                                            r#type: "button",
                                            class: if selected_size() == s {
                                                "px-2.5 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                            } else {
                                                "px-2.5 py-1 rounded bg-white/[0.03] hover:bg-white/[0.08] text-zinc-400 hover:text-white border border-white/[0.06] cursor-pointer"
                                            },
                                            onclick: move |_| selected_size.set(s),
                                            "{s.name().to_uppercase()}"
                                        }
                                    }
                                }
                            }

                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Stato di Presenza:" }
                                div { class: "flex flex-wrap gap-1.5",
                                    button {
                                        r#type: "button",
                                        class: if selected_status().is_none() {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| selected_status.set(None),
                                        "Nessuno"
                                    }
                                    for st in [PresenceStatus::Online, PresenceStatus::Offline, PresenceStatus::Busy, PresenceStatus::DirectP2P] {
                                        button {
                                            r#type: "button",
                                            class: if selected_status() == Some(st) {
                                                "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                            } else {
                                                "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                            },
                                            onclick: move |_| selected_status.set(Some(st)),
                                            "{st.label()}"
                                        }
                                    }
                                }
                            }

                            div { class: "pt-2 border-t border-white/[0.06] space-y-2",
                                Checkbox {
                                    checked: with_image,
                                    label: Some("Simula Immagine Foto Reale".to_string()),
                                }
                                div { class: "space-y-1",
                                    span { class: "text-zinc-400 block text-[10px]", "Nome per Iniziali & Gradiente:" }
                                    input {
                                        r#type: "text",
                                        class: "w-full px-2.5 py-1.5 rounded bg-black/60 border border-white/[0.1] text-zinc-200 focus:outline-none focus:border-purple-500",
                                        value: "{user_name}",
                                        oninput: move |e| user_name.set(e.value()),
                                    }
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
