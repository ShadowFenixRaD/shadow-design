use dioxus::prelude::*;

#[component]
pub fn FeatureBanner(feature_name: &'static str) -> Element {
    rsx! {
        div { class: "mb-6 p-4 rounded-xl bg-blue-950/20 border border-blue-900/40 flex items-center justify-between text-sm",
            div { class: "flex items-center gap-2",
                span { class: "text-blue-400 font-semibold", "📦 Feature Cargo:" }
                code { class: "px-2 py-0.5 rounded bg-zinc-900 text-blue-300 font-mono text-xs border border-zinc-700",
                    "shadow-design = {{ features = [\"{feature_name}\"] }}"
                }
            }
            span { class: "text-xs text-zinc-500", "Zero bloat: importa solo ciò che usi" }
        }
    }
}
