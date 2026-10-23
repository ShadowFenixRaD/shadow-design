use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn NavigationMenuPage() -> Element {
    let mut selected_link = use_signal(|| "Nessuno".to_string());

    let menu_items = vec![
        NavigationMenuItem::new(
            "Prodotti",
            vec![
                NavigationMenuSection {
                    heading: Some("Sistemi Core".to_string()),
                    links: vec![
                        NavigationLinkItem::new("Shadow Engine")
                            .with_description("Framework grafico ad altissime prestazioni")
                            .with_icon(IconKind::Sparkles)
                            .with_badge("v0.4"),
                        NavigationLinkItem::new("Mesh Gateway")
                            .with_description("Routing decentralizzato crittografato P2P")
                            .with_icon(IconKind::Radio),
                        NavigationLinkItem::new("Telemetry Suite")
                            .with_description("Grafici SVG e tracciamento in tempo reale")
                            .with_icon(IconKind::Layers),
                    ],
                },
                NavigationMenuSection {
                    heading: Some("Sicurezza & Accesso".to_string()),
                    links: vec![
                        NavigationLinkItem::new("Zero Trust Auth")
                            .with_description("Verifica continua identità e permessi")
                            .with_icon(IconKind::Lock),
                        NavigationLinkItem::new("Hardware Keytips")
                            .with_description("Scorciatoie di navigazione da tastiera")
                            .with_icon(IconKind::Terminal),
                    ],
                },
            ],
        )
        .with_preview(NavigationPreviewCard {
            title: "Shadow Design v0.4".to_string(),
            description: "Nuova suite di grafici SVG e componenti di navigazione avanzata pronta all'uso.".to_string(),
            cta_label: Some("Scopri di più".to_string()),
            image_url: Some("https://picsum.photos/seed/navprev/400/200".to_string()),
            on_cta_click: Some(EventHandler::new(move |_| {
                selected_link.set("CTA Preview: Shadow Design v0.4".to_string());
            })),
        }),
        NavigationMenuItem::new(
            "Documentazione",
            vec![
                NavigationMenuSection {
                    heading: Some("Guide & Quickstart".to_string()),
                    links: vec![
                        NavigationLinkItem::new("Installazione")
                            .with_description("Aggiungi la libreria al workspace Cargo")
                            .with_icon(IconKind::Code),
                        NavigationLinkItem::new("Personalizzazione Temi")
                            .with_description("Palette cromatiche, raggio e micro-glow")
                            .with_icon(IconKind::Palette),
                    ],
                },
            ],
        ),
    ];

    let props = vec![
        PropRow {
            name: "items",
            r#type: "Vec<NavigationMenuItem>",
            default: "-",
            description: "Struttura dati delle voci principali, sezioni a colonna e schede anteprima",
        },
        PropRow {
            name: "on_select",
            r#type: "Option<EventHandler<NavigationLinkItem>>",
            default: "None",
            description: "Callback scatenata quando l'utente clicca su un link del MegaMenu",
        },
    ];

    let code_snippet = r#"NavigationMenu {
    items: menu_items,
    on_select: move |link| println!("Navigazione: {}", link.title),
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "NavigationMenu (Mega-Menu)" }
                Text { dimmed: true, size: TextSize::Sm, "Barra di navigazione per applicazioni ad elevata complessità con pannelli a discesa a larghezza estesa, suddivisi in più colonne con schede di anteprima incorporate." }
            }

            FeatureBanner { feature_name: "navigation-menu" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "w-full p-8 rounded-2xl bg-zinc-950/70 border border-white/[0.08] min-h-[360px] flex flex-col justify-between",
                        NavigationMenu {
                            items: menu_items,
                            on_select: move |item: NavigationLinkItem| {
                                selected_link.set(item.title);
                            },
                        }

                        div { class: "mt-12 p-3 rounded-xl bg-white/[0.02] border border-white/[0.06] text-xs font-mono text-zinc-400 flex items-center justify-between",
                            span { "Ultimo link cliccato:" }
                            span { class: "text-purple-400 font-bold", "{selected_link()}" }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
