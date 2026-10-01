use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::FeatureBanner;

#[component]
pub fn BadgesPage() -> Element {
    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                h2 { class: "text-2xl font-bold text-white", "Badge Component" }
                p { class: "text-zinc-400 text-sm mt-1", "Indicatori di stato ed etichette con forme rettangolari o a pillola." }
            }

            FeatureBanner { feature_name: "badge" }

            Card {
                title: "Varianti Standard".to_string(),
                shadow: CardShadow::Sm,
                div { class: "flex flex-wrap items-center gap-3",
                    Badge { label: "Default".to_string(), variant: BadgeVariant::Default }
                    Badge { label: "Primary".to_string(), variant: BadgeVariant::Primary }
                    Badge { label: "Success".to_string(), variant: BadgeVariant::Success }
                    Badge { label: "Warning".to_string(), variant: BadgeVariant::Warning }
                    Badge { label: "Danger".to_string(), variant: BadgeVariant::Danger }
                    Badge { label: "Outline".to_string(), variant: BadgeVariant::Outline }
                }
            }

            Card {
                title: "Pill Badges (Arrotondati)".to_string(),
                shadow: CardShadow::Sm,
                div { class: "flex flex-wrap items-center gap-3",
                    Badge { label: "Attivo".to_string(), variant: BadgeVariant::Success, pill: true }
                    Badge { label: "In Revisione".to_string(), variant: BadgeVariant::Warning, pill: true }
                    Badge { label: "Errore 404".to_string(), variant: BadgeVariant::Danger, pill: true }
                    Badge { label: "Pro".to_string(), variant: BadgeVariant::Primary, pill: true }
                }
            }
        }
    }
}
