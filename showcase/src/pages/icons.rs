use dioxus::prelude::*;
use shadow_design::prelude::*;
use crate::components::clipboard::copy_text;
use crate::components::{ControlGroup, PageHeader, Playground, PropRow, ShowcaseTabs};

#[component]
pub fn IconsPage() -> Element {
    let search = use_signal(String::new);
    let icon_size = use_signal(|| 20.0f64);
    let mut toast = use_notification();

    let search_term = search().to_lowercase();
    let filtered_icons: Vec<IconKind> = IconKind::ALL
        .iter()
        .copied()
        .filter(|k| k.name().to_lowercase().contains(&search_term))
        .collect();

    let code_snippet = r#"Icon {
    kind: IconKind::Sparkles,
    size: 20,
}"#;

    let props_rows = vec![
        PropRow {
            name: "kind",
            r#type: "IconKind",
            default: "-",
            description: "Variante geometrica vettoriale nitida da 1.5px stroke (50 icone integrate a zero dipendenze).",
        },
        PropRow {
            name: "size",
            r#type: "usize",
            default: "16",
            description: "Dimensione in pixel (larghezza e altezza) del viewport SVG.",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS aggiuntive applicate all'elemento <svg>.",
        },
    ];

    let preview = rsx! {
        div { class: "sc-stack sc-stack--loose",
            Playground {
                stage_start: true,
                stage: rsx! {
                    div { class: "grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-2 w-full",
                        for icon in filtered_icons {
                            Button {
                                key: "{icon.name()}",
                                variant: ButtonVariant::Secondary,
                                size: ButtonSize::Sm,
                                full_width: true,
                                left_icon: rsx! { Icon { kind: icon, size: icon_size() as u32 } },
                                onclick: move |_| {
                                    let name = icon.name();
                                    let s = icon_size() as u32;
                                    let snippet = format!("Icon {{ kind: IconKind::{name}, size: {s} }}");
                                    spawn(async move {
                                        let ok = copy_text(&snippet).await;
                                        if ok {
                                            toast.show_with_title(
                                                format!("Copiato IconKind::{name}"),
                                                "Snippet RSX copiato negli appunti",
                                                NotificationVariant::Success,
                                            );
                                        }
                                    });
                                },
                                "{icon.name()}"
                            }
                        }
                    }
                },
                controls: rsx! {
                    ControlGroup { label: "Cerca".to_string(),
                        TextInput {
                            value: search,
                            placeholder: "Cerca icona (es. Check, Chevron, User)...".to_string(),
                            left_icon: SvgIcon::Kind(IconKind::Search),
                        }
                    }
                    ControlGroup { label: "Size: {icon_size()}px".to_string(),
                        Slider { value: icon_size, min: 12.0, max: 48.0, step: 1.0 }
                    }
                },
            }

            Card {
                title: "SvgIcon::Custom Example".to_string(),
                subtitle: "Shadow Design supporta anche icone SVG customizzate con il tipo polimorfico SvgIcon.".to_string(),
                div { class: "sc-actions",
                    span { class: "sc-accent-text", Icon { kind: IconKind::Sparkles, size: 24 } }
                    span { class: "sc-success-text", Icon { kind: IconKind::CheckCircle, size: 24 } }
                    span { class: "sc-info-text", Icon { kind: IconKind::Code, size: 24 } }
                }
            }
        }
    };

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Icon".to_string(),
                description: "Suite completa di 50 icone geometriche native vettoriali con stroke a 1.5px e zero dipendenze esterne.".to_string(),
            }
            ShowcaseTabs {
                preview: preview,
                code: code_snippet.to_string(),
                props: props_rows,
            }
        }
    }
}
