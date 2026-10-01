use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn FeatureBanner(feature_name: &'static str) -> Element {
    let theme = use_theme();
    rsx! {
        div { class: "{theme().banner} flex items-center justify-between text-sm",
            div { class: "flex items-center gap-2",
                span { class: "{theme().text_accent} font-semibold", "📦 Feature Cargo:" }
                code { class: "px-2 py-0.5 rounded bg-black {theme().text_accent_bright} font-mono text-xs border {theme().border}",
                    "shadow-design = {{ features = [\"{feature_name}\"] }}"
                }
            }
            span { class: "text-xs text-zinc-500", "Zero bloat: importa solo ciò che usi" }
        }
    }
}
