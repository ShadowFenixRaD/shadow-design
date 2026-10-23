use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn MasonryPage() -> Element {
    let sample_items = vec![
        MasonryItem::new("1", "https://picsum.photos/seed/shadow1/600/400")
            .with_title("Architettura Cyber")
            .with_subtitle("Superfici oscure e specular highlights"),
        MasonryItem::new("2", "https://picsum.photos/seed/shadow2/600/750")
            .with_title("Neon Telemetry")
            .with_subtitle("Data visualization su asse cartesiano"),
        MasonryItem::new("3", "https://picsum.photos/seed/shadow3/600/500")
            .with_title("Mesh Radio")
            .with_subtitle("Connettività decentralizzata P2P"),
        MasonryItem::new("4", "https://picsum.photos/seed/shadow4/600/650")
            .with_title("Obsidian Titanium")
            .with_subtitle("Monocromatico puro ad alto contrasto"),
        MasonryItem::new("5", "https://picsum.photos/seed/shadow5/600/450")
            .with_title("Crimson Phoenix")
            .with_subtitle("Bagliori cremisi e gradiente vulcanico"),
        MasonryItem::new("6", "https://picsum.photos/seed/shadow6/600/800")
            .with_title("Emerald Matrix")
            .with_subtitle("Verde smeraldo telemetry SLA"),
    ];

    let props = vec![
        PropRow {
            name: "items",
            r#type: "Vec<MasonryItem>",
            default: "-",
            description: "Collezione di elementi multimediali con URL sorgente, titolo e didascalia",
        },
        PropRow {
            name: "cols",
            r#type: "ResponsiveCols",
            default: "ResponsiveCols::default()",
            description: "Configurazione responsive colonne (base, sm, md, lg, xl)",
        },
        PropRow {
            name: "gap",
            r#type: "ComponentSpacing",
            default: "ComponentSpacing::Sm",
            description: "Spaziatura orizzontale e verticale tra le colonne",
        },
        PropRow {
            name: "with_lightbox",
            r#type: "bool",
            default: "true",
            description: "Abilita il lightbox modale a schermo intero con zoom e navigazione",
        },
    ];

    let code_snippet = r#"ImageList {
    items: sample_items,
    cols: ResponsiveCols::responsive(1, 2, 3, 3, 3),
    gap: ComponentSpacing::Sm,
    with_lightbox: true,
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "MasonryGrid & ImageList" }
                Text { dimmed: true, size: TextSize::Sm, "Griglia a cascata in stile Pinterest con altezze eterogenee distribuite fluidamente e visualizzatore Lightbox modale integrato." }
            }

            FeatureBanner { feature_name: "masonry" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "w-full p-4 rounded-2xl bg-zinc-950/70 border border-white/[0.08]",
                        ImageList {
                            items: sample_items,
                            cols: ResponsiveCols::responsive(1, 2, 3, 3, 3),
                            gap: ComponentSpacing::Sm,
                            with_lightbox: true,
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
