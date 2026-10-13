use dioxus::prelude::*;
use shadow_design::prelude::*;
use crate::components::clipboard::copy_text;
use crate::components::ShowcaseTabs;
use crate::components::props_table::PropRow;

#[component]
pub fn IconsPage() -> Element {
    let search = use_signal(String::new);
    let mut icon_size = use_signal(|| 20usize);
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
        div { class: "flex flex-col gap-6",
            // Controls
            div { class: "flex flex-col sm:flex-row items-stretch sm:items-center justify-between gap-4 p-4 rounded-xl border border-white/[0.08] bg-[#0c0c12]/80",
                div { class: "flex-1 max-w-sm",
                    TextInput {
                        value: search,
                        placeholder: "Cerca icona (es. Check, Chevron, User)...".to_string(),
                        left_icon: SvgIcon::Kind(IconKind::Search),
                    }
                }
                div { class: "flex items-center gap-3 text-xs text-zinc-300 font-mono",
                    span { "Size: {icon_size}px" }
                    input {
                        r#type: "range",
                        min: "12",
                        max: "48",
                        value: "{icon_size}",
                        class: "cursor-pointer",
                        oninput: move |e| {
                            if let Ok(v) = e.value().parse::<usize>() {
                                icon_size.set(v);
                            }
                        }
                    }
                }
            }

            // Grid
            div { class: "grid grid-cols-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 gap-3",
                for icon in filtered_icons {
                    button {
                        key: "{icon.name()}",
                        r#type: "button",
                        class: "flex flex-col items-center justify-center p-3 gap-2 rounded-xl border border-white/[0.06] bg-white/[0.02] hover:bg-white/[0.06] hover:border-purple-500/30 transition-all cursor-pointer group text-center active:scale-95",
                        onclick: move |_| {
                            let name = icon.name();
                            let s = icon_size();
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
                        div { class: "text-zinc-200 group-hover:text-purple-300 transition-colors",
                            Icon { kind: icon, size: icon_size() as u32 }
                        }
                        span { class: "text-[11px] font-mono text-zinc-400 group-hover:text-zinc-200 truncate w-full",
                            "{icon.name()}"
                        }
                    }
                }
            }

            // Custom SVG Icon Example
            div { class: "mt-4 p-5 rounded-xl border border-white/[0.08] bg-[#0c0c12]/80 flex flex-col gap-3",
                Heading { order: HeadingOrder::H3, "SvgIcon::Custom Example" }
                Text { size: TextSize::Sm, class: "text-zinc-400",
                    "Shadow Design supporta anche icone SVG customizzate con il tipo polimorfico SvgIcon."
                }
                div { class: "flex items-center gap-4 mt-2",
                    span { class: "text-purple-400",
                        Icon { kind: IconKind::Sparkles, size: 24 }
                    }
                    span { class: "text-emerald-400",
                        Icon { kind: IconKind::CheckCircle, size: 24 }
                    }
                    span { class: "text-cyan-400",
                        Icon { kind: IconKind::Code, size: 24 }
                    }
                }
            }
        }
    };

    rsx! {
        div { class: "flex flex-col gap-6",
            div { class: "flex flex-col gap-2",
                Heading { order: HeadingOrder::H1, "Icon" }
                Text { size: TextSize::Md, class: "text-zinc-400",
                    "Suite completa di 50 icone geometriche native vettoriali con stroke a 1.5px e zero dipendenze esterne."
                }
            }
            ShowcaseTabs {
                preview: preview,
                code: code_snippet.to_string(),
                props: props_rows,
            }
        }
    }
}
