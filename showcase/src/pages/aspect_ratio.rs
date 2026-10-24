use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs,
};

#[derive(Clone, Copy, PartialEq, Debug)]
enum RatioPreset {
    Widescreen16_9,
    Standard4_3,
    Square1_1,
    Cinematic21_9,
}

impl RatioPreset {
    fn value(&self) -> f64 {
        match self {
            Self::Widescreen16_9 => 16.0 / 9.0,
            Self::Standard4_3 => 4.0 / 3.0,
            Self::Square1_1 => 1.0,
            Self::Cinematic21_9 => 21.0 / 9.0,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Widescreen16_9 => "16:9 (Widescreen)",
            Self::Standard4_3 => "4:3 (Standard)",
            Self::Square1_1 => "1:1 (Quadrato)",
            Self::Cinematic21_9 => "21:9 (Cinematic Ultrawide)",
        }
    }
}

#[component]
pub fn AspectRatioPage() -> Element {
    let selected_preset = use_signal(|| RatioPreset::Widescreen16_9);

    let props = vec![
        PropRow {
            name: "ratio",
            r#type: "f64",
            default: "16.0 / 9.0 (1.777...)",
            description: "Rapporto larghezza / altezza desiderato (es. 16.0 / 9.0, 4.0 / 3.0, 1.0, 21.0 / 9.0).",
        },
        PropRow {
            name: "class",
            r#type: "String",
            default: "\"\"",
            description: "Classi CSS aggiuntive applicate al contenitore AspectRatio.",
        },
    ];

    let code_snippet = r#"AspectRatio {
    ratio: 16.0 / 9.0,
    div { class: "sc-demo-cell sc-demo-cell--accent",
        Icon { kind: IconKind::Layers, size: 32 }
        span { class: "sc-section-title", "Video Stream Player" }
        span { class: "sc-subtle-text", "Aspect Ratio 16:9 garantito senza CLS" }
    }
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "AspectRatio".to_string(),
                description: "Contenitore universale per vincolare il rapporto di forma di immagini, canvas, mappe, video player o iframe, azzerando il Cumulative Layout Shift (CLS).".to_string(),
            }

            FeatureBanner { feature_name: "aspect_ratio" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage: rsx! {
                            div { class: "w-full",
                                AspectRatio {
                                    ratio: selected_preset().value(),
                                    class: "sc-demo-cell sc-demo-cell--accent",
                                    div { class: "sc-stack items-center text-center",
                                        Icon { kind: IconKind::Layers, size: 28 }
                                        span { class: "sc-section-title sc-mono", "{selected_preset().label()}" }
                                        span { class: "sc-subtle-text sc-mono", "ratio = {selected_preset().value():.3}" }
                                        div { class: "sc-actions",
                                            Badge { label: "Zero CLS".to_string(), variant: BadgeVariant::Success, dot: true }
                                            Badge { label: "Responsive Fit".to_string(), variant: BadgeVariant::Primary }
                                        }
                                    }
                                }
                            }
                        },
                        controls: rsx! {
                            ChoicePicker {
                                label: "Preset Rapporto di Forma (Ratio)".to_string(),
                                options: [RatioPreset::Widescreen16_9, RatioPreset::Standard4_3, RatioPreset::Square1_1, RatioPreset::Cinematic21_9]
                                    .iter()
                                    .map(|p| (*p, p.label().to_string()))
                                    .collect::<Vec<_>>(),
                                value: selected_preset,
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
