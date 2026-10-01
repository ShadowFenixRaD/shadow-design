use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::FeatureBanner;

#[component]
pub fn NavbarDemoPage() -> Element {
    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                h2 { class: "text-2xl font-bold text-white", "Navbar Component" }
                p { class: "text-zinc-400 text-sm mt-1", "Barra di navigazione superiore responsive pronta per ospitare logo, link e azioni." }
            }

            FeatureBanner { feature_name: "navbar" }

            Card {
                title: "Anteprima Navbar".to_string(),
                shadow: CardShadow::Md,
                div { class: "border border-zinc-800 rounded-xl overflow-hidden",
                    Navbar {
                        brand: rsx! {
                            span { class: "font-bold text-white text-base tracking-wide", "MioSitoWeb" }
                        },
                        div { class: "flex items-center gap-4 text-sm text-zinc-300",
                            span { class: "hover:text-white cursor-pointer", "Home" }
                            span { class: "hover:text-white cursor-pointer", "Prodotti" }
                            span { class: "hover:text-white cursor-pointer", "Contatti" }
                            Button { size: ButtonSize::Sm, variant: ButtonVariant::Primary, "Accedi" }
                        }
                    }
                }
            }
        }
    }
}
