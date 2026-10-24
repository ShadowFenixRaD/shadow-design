use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, ControlGroup, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn AvatarsPage() -> Element {
    let selected_size = use_signal(|| AvatarSize::Lg);
    let selected_status = use_signal(|| Some(PresenceStatus::DirectP2P));
    let user_name = use_signal(|| "Satoshi Nakamoto".to_string());
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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Avatar & Presenza Utente".to_string(),
                description: "Rappresentazione del profilo, rubrica e nodi mesh con fallback ad iniziali su gradiente matematico e stato di connettività real-time.".to_string(),
            }

            FeatureBanner { feature_name: "avatar" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            Avatar {
                                name: Some(user_name()),
                                src: image_url,
                                size: selected_size(),
                                status: selected_status(),
                            }
                            div { class: "sc-actions",
                                span { class: "sc-subtle-text sc-mono", "Anteprima AvatarGroup:" }
                                AvatarGroup {
                                    Avatar { name: Some("Elena Rostova".to_string()), size: AvatarSize::Sm }
                                    Avatar { name: Some("Marco Polo".to_string()), size: AvatarSize::Sm }
                                    Avatar { name: Some("Alice Vance".to_string()), size: AvatarSize::Sm }
                                    Avatar { name: Some("Direct Node".to_string()), size: AvatarSize::Sm, status: PresenceStatus::Online }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Dimensione Avatar".to_string(),
                                options: [AvatarSize::Xs, AvatarSize::Sm, AvatarSize::Md, AvatarSize::Lg, AvatarSize::Xl]
                                    .iter()
                                    .map(|s| (*s, s.name().to_uppercase()))
                                    .collect::<Vec<_>>(),
                                value: selected_size,
                            }
                            ChoicePicker {
                                label: "Stato di Presenza".to_string(),
                                options: vec![
                                    (None, "Nessuno".to_string()),
                                    (Some(PresenceStatus::Online), PresenceStatus::Online.label().to_string()),
                                    (Some(PresenceStatus::Offline), PresenceStatus::Offline.label().to_string()),
                                    (Some(PresenceStatus::Busy), PresenceStatus::Busy.label().to_string()),
                                    (Some(PresenceStatus::DirectP2P), PresenceStatus::DirectP2P.label().to_string()),
                                ],
                                value: selected_status,
                            }
                            hr { class: "sc-control-divider" }
                            ToggleControl { label: "Simula Immagine Foto Reale".to_string(), checked: with_image }
                            ControlGroup { label: "Nome per Iniziali & Gradiente".to_string(),
                                TextInput { value: user_name }
                            }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
