//! Icons: every `IconKind` as a static tile with a copy button (REVAMP §2.4).
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::sections::SectionHead;

const USAGE_CODE: &str = r#"Icon { kind: IconKind::Sparkles, size: 20 }

// Inside components that take an icon:
TextInput { value, left_icon: Some(SvgIcon::kind(IconKind::Search)) }
Button { left_icon: Some(rsx! { Icon { kind: IconKind::Check, size: 15 } }), "Salva" }"#;

#[component]
pub fn IconsPage() -> Element {
    let search = use_signal(String::new);
    let size = use_signal(|| 20.0f64);
    let q = search().trim().to_lowercase();
    let icons: Vec<IconKind> = IconKind::ALL.iter().copied().filter(|k| k.name().to_lowercase().contains(&q)).collect();
    let px = size() as u32;
    let total = IconKind::ALL.len();
    let shown = icons.len();

    rsx! {
        document::Title { "Icons · Shadow Design" }
        div { class: "sc-story",
            header { class: "sc-story__head",
                span { class: "sc-eyebrow", "Fondamenta · feature icon" }
                h1 { class: "sc-title", "Icons" }
                p { class: "sc-lede",
                    "{total} icone SVG native con tratto 1.5, senza dipendenze. Ereditano il colore del testo e si ridimensionano con size."
                }
            }

            section { class: "sc-section",
                div { class: "sc-icon-tools",
                    TextInput {
                        value: search,
                        label: Some("Cerca".to_string()),
                        placeholder: Some("Check, Chevron, Lock…".to_string()),
                        left_icon: Some(SvgIcon::kind(IconKind::Search)),
                        clearable: true,
                    }
                    div { class: "sc-knob",
                        div { class: "sc-knob__row",
                            span { class: "sc-eyebrow", "Dimensione" }
                            span { class: "sc-readout", "{px}px" }
                        }
                        Slider { value: size, min: 12.0, max: 48.0, step: 2.0, show_tooltip: false }
                    }
                }
                span { class: "sc-hint", "{shown} di {total} icone" }
                SimpleGrid { cols: 2, sm: Some(3), md: Some(4), lg: Some(6), spacing: GridSpacing::Sm,
                    for icon in icons {
                        div { key: "{icon.name()}", class: "sc-icon-tile-static",
                            div { class: "sc-icon-tile-static__glyph",
                                Icon { kind: icon, size: px }
                            }
                            span { class: "sc-icon-tile-static__name", "{icon.name()}" }
                            CopyButton {
                                value: format!("IconKind::{}", icon.name()),
                                icon_only: true,
                                aria_label: format!("Copia IconKind::{}", icon.name()),
                            }
                        }
                    }
                }
            }

            section { class: "sc-section",
                SectionHead { eyebrow: "Uso", title: "Icon e SvgIcon" }
                CodeBlock { code: USAGE_CODE.to_string(), language: "rsx", with_line_numbers: false }
            }
        }
    }
}
