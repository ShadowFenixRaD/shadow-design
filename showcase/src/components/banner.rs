use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn FeatureBanner(feature_name: &'static str) -> Element {
    let theme = use_theme();
    rsx! {
        div { class: "{theme().banner} flex flex-wrap items-center justify-between gap-3 text-xs sm:text-sm",
            div { class: "flex items-center gap-2",
                span { class: "{theme().text_accent} flex items-center gap-1.5 font-semibold",
                    Icon { kind: IconKind::Box, size: 15 }
                    "Feature Cargo:"
                }
                code { class: "px-2 py-0.5 rounded bg-[#030305] {theme().text_accent_bright} font-mono text-xs border border-white/10",
                    "shadow-design = {{ features = [\"{feature_name}\"] }}"
                }
            }
            span { class: "text-xs text-zinc-500 font-mono", "Zero bloat: importa solo ciò che usi" }
        }
    }
}
