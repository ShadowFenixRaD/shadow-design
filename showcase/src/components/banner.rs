use dioxus::prelude::*;

#[component]
pub fn FeatureBanner(feature_name: &'static str) -> Element {
    rsx! {
        div { class: "mb-6 p-4 rounded-xl bg-purple-950/25 border border-purple-900/50 flex items-center justify-between text-sm shadow-md shadow-purple-950/20",
            div { class: "flex items-center gap-2",
                span { class: "text-purple-400 font-semibold", "📦 Feature Cargo:" }
                code { class: "px-2 py-0.5 rounded bg-black text-purple-300 font-mono text-xs border border-purple-900/60",
                    "shadow-design = {{ features = [\"{feature_name}\"] }}"
                }
            }
            span { class: "text-xs text-zinc-500", "Zero bloat: importa solo ciò che usi" }
        }
    }
}
