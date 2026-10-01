use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::FeatureBanner;

#[component]
pub fn CardsPage() -> Element {
    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                h2 { class: "text-2xl font-bold text-white", "Card Component" }
                p { class: "text-zinc-400 text-sm mt-1", "Contenitori di contenuto strutturato con intestazione, corpo, footer e diversi livelli di ombra." }
            }

            FeatureBanner { feature_name: "card" }

            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                Card {
                    title: "Card Standard".to_string(),
                    subtitle: "Con titolo e sottotitolo".to_string(),
                    shadow: CardShadow::Md,
                    p { class: "text-sm text-zinc-300", "Questo è il corpo principale della card. Supporta qualsiasi elemento o componente nidificato." }
                }

                Card {
                    title: "Card con Footer".to_string(),
                    shadow: CardShadow::Lg,
                    footer: rsx! {
                        div { class: "flex justify-between items-center",
                            span { "Ultima modifica: Oggi" }
                            Button { size: ButtonSize::Sm, variant: ButtonVariant::Ghost, "Dettagli" }
                        }
                    },
                    p { class: "text-sm text-zinc-300", "Questa card include una sezione footer integrata ideale per metadati e azioni rapide." }
                }
            }

            Card {
                title: "Livelli di Ombra (Shadows)".to_string(),
                shadow: CardShadow::Sm,
                div { class: "grid grid-cols-1 sm:grid-cols-3 gap-4 pt-2",
                    div { class: "p-4 rounded-lg bg-zinc-800/60 border border-zinc-700/60 text-xs text-center shadow-sm", "CardShadow::Sm" }
                    div { class: "p-4 rounded-lg bg-zinc-800/60 border border-zinc-700/60 text-xs text-center shadow-md", "CardShadow::Md" }
                    div { class: "p-4 rounded-lg bg-zinc-800/60 border border-zinc-700/60 text-xs text-center shadow-2xl", "CardShadow::Xl" }
                }
            }
        }
    }
}
