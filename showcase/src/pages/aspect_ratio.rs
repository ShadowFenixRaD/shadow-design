use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::banner::FeatureBanner;
use crate::components::props_table::PropRow;
use crate::components::showcase_tabs::ShowcaseTabs;

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
    let mut selected_preset = use_signal(|| RatioPreset::Widescreen16_9);

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
    div { class: "p-6 rounded-xl bg-purple-950/40 border border-purple-800/40 flex flex-col items-center justify-center text-center",
        Icon { kind: IconKind::Layers, size: 32 }
        span { class: "mt-2 font-mono text-sm text-purple-200 font-bold", "Video Stream Player" }
        span { class: "text-xs text-purple-400/80", "Aspect Ratio 16:9 garantito senza CLS" }
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "AspectRatio" }
                Text { dimmed: true, size: TextSize::Sm, "Contenitore universale per vincolare il rapporto di forma di immagini, canvas, mappe, video player o iframe, azzerando il Cumulative Layout Shift (CLS)." }
            }

            FeatureBanner { feature_name: "aspect_ratio" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6 w-full",
                        // Live AspectRatio Container
                        div { class: "w-full max-w-lg mx-auto",
                            AspectRatio {
                                ratio: selected_preset().value(),
                                class: "rounded-2xl border border-white/10 shadow-lg overflow-hidden bg-zinc-950",
                                div { class: "p-6 bg-purple-950/40 border border-purple-800/40 flex flex-col items-center justify-center text-center",
                                    span { class: "p-3 rounded-full bg-purple-500/10 border border-purple-500/20 text-purple-300 mb-2 flex items-center justify-center",
                                        Icon { kind: IconKind::Layers, size: 28 }
                                    }
                                    span { class: "text-sm font-bold text-white font-mono", "{selected_preset().label()}" }
                                    span { class: "text-xs text-purple-300 font-mono mt-1", "ratio = {selected_preset().value():.3}" }
                                    div { class: "mt-3 flex items-center gap-2",
                                        Badge { label: "Zero CLS".to_string(), variant: BadgeVariant::Success, dot: true }
                                        Badge { label: "Responsive Fit".to_string(), variant: BadgeVariant::Primary }
                                    }
                                }
                            }
                        }

                        // Configurator Controls
                        div { class: "p-4 rounded-xl bg-white/[0.02] border border-white/[0.08] space-y-4",
                            span { class: "text-xs font-mono uppercase tracking-wider text-zinc-400 font-semibold block", "Preset Rapporto di Forma (Ratio)" }
                            div { class: "flex flex-wrap gap-2",
                                for p in [RatioPreset::Widescreen16_9, RatioPreset::Standard4_3, RatioPreset::Square1_1, RatioPreset::Cinematic21_9] {
                                    button {
                                        class: if selected_preset() == p {
                                            "px-3 py-1.5 text-xs font-mono rounded-lg bg-purple-950 border border-purple-600 text-purple-200 font-bold"
                                        } else {
                                            "px-3 py-1.5 text-xs font-mono rounded-lg bg-zinc-900 border border-white/10 text-zinc-400 hover:text-white"
                                        },
                                        onclick: move |_| selected_preset.set(p),
                                        "{p.label()}"
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
