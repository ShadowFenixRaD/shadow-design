use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn MediaCardPage() -> Element {
    let mut click_count = use_signal(|| 0);

    let props = vec![
        PropRow {
            name: "image_url",
            r#type: "String",
            default: "-",
            description: "URL dell'immagine immersiva per la testata della card",
        },
        PropRow {
            name: "aspect_ratio",
            r#type: "f64",
            default: "16.0 / 9.0",
            description: "Rapporto di forma fisso della testata per prevenire il Cumulative Layout Shift (CLS)",
        },
        PropRow {
            name: "title",
            r#type: "Option<String>",
            default: "None",
            description: "Titolo principale della card con evidenziazione in hover",
        },
        PropRow {
            name: "badge",
            r#type: "Option<String>",
            default: "None",
            description: "Etichetta in sovrimpressione nell'angolo superiore della testata",
        },
        PropRow {
            name: "footer",
            r#type: "Option<Element>",
            default: "None",
            description: "Slot personalizzato per azioni e pulsanti a piè di card",
        },
    ];

    let code_snippet = r#"MediaCard {
    image_url: "https://picsum.photos/seed/cyber/800/450".to_string(),
    aspect_ratio: 16.0 / 9.0,
    title: Some("Quantum Computing Lab".to_string()),
    subtitle: Some("Simulazione avanzata di nodi mesh".to_string()),
    badge: Some("LIVE".to_string()),
    on_click: move |_| println!("Card cliccata!"),
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "MediaCard & CardActionArea" }
                Text { dimmed: true, size: TextSize::Sm, "Scheda contenitore arricchita con testata multimediale a rapporto d'aspetto fisso ed effetto cliccabile tattile esteso a tutta la superficie." }
            }

            FeatureBanner { feature_name: "media_card" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 sm:grid-cols-2 gap-6 w-full p-4",
                        MediaCard {
                            image_url: "https://picsum.photos/seed/cyber/800/450".to_string(),
                            title: Some("Stazione Radio Mesh".to_string()),
                            subtitle: Some("Canale crittografato P2P ad alta frequenza".to_string()),
                            badge: Some("ATTIVO".to_string()),
                            on_click: move |_| click_count += 1,
                            footer: rsx! {
                                span { class: "font-mono text-purple-400 font-bold", "Click: {click_count()}" }
                                span { "Visualizza dettagli →" }
                            },
                        }

                        MediaCard {
                            image_url: "https://picsum.photos/seed/matrix/800/450".to_string(),
                            title: Some("Telemetry Matrix".to_string()),
                            subtitle: Some("Analisi vettoriale SVG e streaming live".to_string()),
                            badge: Some("PROD".to_string()),
                            on_click: move |_| click_count += 1,
                            footer: rsx! {
                                span { class: "text-zinc-500", "Latenza: 12ms" }
                                span { class: "text-emerald-400 font-mono", "Online" }
                            },
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
