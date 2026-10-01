use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::FeatureBanner;

#[component]
pub fn ButtonsPage() -> Element {
    let mut click_count = use_signal(|| 0);

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                h2 { class: "text-2xl font-bold text-white", "Button Component" }
                p { class: "text-zinc-400 text-sm mt-1", "Pulsanti altamente configurabili con varianti cromatiche, dimensioni e stati." }
            }

            FeatureBanner { feature_name: "button" }

            // Interactive Playground
            Card {
                title: "Test Interattivo".to_string(),
                shadow: CardShadow::Md,
                div { class: "flex items-center gap-4",
                    Button {
                        variant: ButtonVariant::Primary,
                        onclick: move |_| *click_count.write() += 1,
                        "Incrementa contatore"
                    }
                    Button {
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| *click_count.write() = 0,
                        "Reset"
                    }
                    span { class: "text-sm text-zinc-300 ml-2 font-mono",
                        "Click registrati: "
                        strong { class: "text-purple-400 text-base", "{click_count}" }
                    }
                }
            }

            // Variants
            Card {
                title: "Varianti".to_string(),
                shadow: CardShadow::Sm,
                div { class: "flex flex-wrap items-center gap-3",
                    Button { variant: ButtonVariant::Primary, "Primary" }
                    Button { variant: ButtonVariant::Secondary, "Secondary" }
                    Button { variant: ButtonVariant::Outline, "Outline" }
                    Button { variant: ButtonVariant::Ghost, "Ghost" }
                    Button { variant: ButtonVariant::Success, "Success" }
                    Button { variant: ButtonVariant::Danger, "Danger" }
                    Button { disabled: true, "Disabled" }
                }
            }

            // Sizes
            Card {
                title: "Dimensioni".to_string(),
                shadow: CardShadow::Sm,
                div { class: "flex items-center gap-4",
                    Button { size: ButtonSize::Sm, "Small (Sm)" }
                    Button { size: ButtonSize::Md, "Medium (Md)" }
                    Button { size: ButtonSize::Lg, "Large (Lg)" }
                }
            }
        }
    }
}
