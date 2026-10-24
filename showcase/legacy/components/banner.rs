use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn FeatureBanner(feature_name: &'static str) -> Element {
    rsx! {
        div { class: "sc-banner",
            div { class: "sc-banner__code",
                span { class: "sc-banner__label",
                    Icon { kind: IconKind::Box, size: 14 }
                    "Feature Cargo"
                }
                Code {
                    "shadow-design = {{ version = \"{VERSION}\", default-features = false, features = [\"{feature_name}\"] }}"
                }
            }
            span { class: "sc-banner__hint", "Zero bloat: importa solo ciò che usi" }
        }
    }
}
