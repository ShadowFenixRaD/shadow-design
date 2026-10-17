use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn ScrollAreaPage() -> Element {
    let mut scroll_type = use_signal(|| ScrollType::Hover);
    let offset_scrollbars = use_signal(|| true);
    let fade_edges = use_signal(|| true);
    let mut demo_mode = use_signal(|| 0); // 0: Vertical, 1: Horizontal, 2: Both

    let props = vec![
        PropRow {
            name: "scroll_type",
            r#type: "ScrollType",
            default: "ScrollType::Hover",
            description: "Comportamento di visualizzazione scrollbar: Hover, Always, Auto o Never",
        },
        PropRow {
            name: "offset_scrollbars",
            r#type: "bool",
            default: "false",
            description: "Aggiunge padding automatico per evitare che la scrollbar si sovrapponga visivamente al testo",
        },
        PropRow {
            name: "fade_edges",
            r#type: "bool",
            default: "false",
            description: "Attiva una maschera gradiente ottica (fade edges) alle estremità per evidenziare contenuto non visibile",
        },
        PropRow {
            name: "scroll_x",
            r#type: "bool",
            default: "false",
            description: "Abilita lo scorrimento lungo l'asse orizzontale",
        },
        PropRow {
            name: "scroll_y",
            r#type: "bool",
            default: "true",
            description: "Abilita lo scorrimento lungo l'asse verticale",
        },
        PropRow {
            name: "height / max_height",
            r#type: "Option<String>",
            default: "None",
            description: "Vincolo di altezza o altezza massima per l'area di scorrimento (es. '280px')",
        },
        PropRow {
            name: "width / max_width",
            r#type: "Option<String>",
            default: "None",
            description: "Vincolo di larghezza o larghezza massima per l'area di scorrimento",
        },
        PropRow {
            name: "viewport_class",
            r#type: "String",
            default: "String::new()",
            description: "Classi CSS addizionali applicate al viewport interno di scorrimento",
        },
    ];

    let code_snippet = r#"ScrollArea {
    scroll_type: ScrollType::Hover,
    offset_scrollbars: true,
    fade_edges: true,
    height: Some("280px".to_string()),
    div { class: "space-y-2 p-2",
        // Contenuto scrollabile con micro-scrollbar
    }
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "ScrollArea" }
                Text { dimmed: true, size: TextSize::Sm, "Contenitore a scorrimento con micro-scrollbar stilizzate coordinate al tema Shadow, dissolvenza bordi (fade edges) e supporto a scomparsa su inattività." }
            }

            FeatureBanner { feature_name: "scroll_area" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6",
                        // Toolbar interattiva
                        div { class: "p-4 rounded-xl bg-[#060608] border border-white/[0.08] grid grid-cols-1 md:grid-cols-3 gap-4 text-xs font-mono",
                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Modalità Scrollbar:" }
                                div { class: "grid grid-cols-2 gap-1.5",
                                    button {
                                        r#type: "button",
                                        class: if scroll_type() == ScrollType::Hover {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| scroll_type.set(ScrollType::Hover),
                                        "Hover"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if scroll_type() == ScrollType::Always {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| scroll_type.set(ScrollType::Always),
                                        "Always"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if scroll_type() == ScrollType::Auto {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| scroll_type.set(ScrollType::Auto),
                                        "Auto"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if scroll_type() == ScrollType::Never {
                                            "px-2 py-1 rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| scroll_type.set(ScrollType::Never),
                                        "Never"
                                    }
                                }
                            }

                            div {
                                span { class: "text-zinc-400 block mb-1.5 uppercase tracking-wider text-[10px]", "Demo Orientamento:" }
                                div { class: "flex flex-col gap-1.5",
                                    button {
                                        r#type: "button",
                                        class: if demo_mode() == 0 {
                                            "px-2 py-1 text-left rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 text-left rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| demo_mode.set(0),
                                        "Verticale (Feed log)"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if demo_mode() == 1 {
                                            "px-2 py-1 text-left rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 text-left rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| demo_mode.set(1),
                                        "Orizzontale (Code / Timeline)"
                                    }
                                    button {
                                        r#type: "button",
                                        class: if demo_mode() == 2 {
                                            "px-2 py-1 text-left rounded bg-purple-950/80 text-purple-200 border border-purple-500/40 font-bold cursor-pointer"
                                        } else {
                                            "px-2 py-1 text-left rounded bg-white/[0.03] text-zinc-400 border border-white/[0.06] cursor-pointer"
                                        },
                                        onclick: move |_| demo_mode.set(2),
                                        "Bidirezionale (X + Y)"
                                    }
                                }
                            }

                            div { class: "flex flex-col justify-center space-y-3 pt-2 md:pt-0 border-t md:border-t-0 md:border-l border-white/[0.06] md:pl-4",
                                Checkbox {
                                    checked: offset_scrollbars,
                                    label: Some("Offset Scrollbars".to_string()),
                                }
                                Checkbox {
                                    checked: fade_edges,
                                    label: Some("Fade Edges Ottico".to_string()),
                                }
                            }
                        }

                        // Preview box
                        div { class: "p-6 rounded-xl bg-[#030304] border border-white/[0.08] shadow-2xl relative",
                            div { class: "text-[11px] font-mono text-zinc-400 mb-3 flex items-center justify-between",
                                span { "SHADOW VIRTUAL SCROLL CONTAINER • 6px MICRO-TRACK" }
                                span { class: "text-purple-400/80", "ScrollType: {scroll_type():?}" }
                            }

                            if demo_mode() == 0 {
                                ScrollArea {
                                    scroll_type: scroll_type(),
                                    offset_scrollbars: offset_scrollbars(),
                                    fade_edges: fade_edges(),
                                    scroll_x: false,
                                    scroll_y: true,
                                    height: Some("260px".to_string()),
                                    class: "border border-white/[0.06] rounded-lg bg-[#08080a] p-2",
                                    div { class: "space-y-2 p-2",
                                        for i in 1..=20 {
                                            div {
                                                key: "{i}",
                                                class: "p-2.5 rounded bg-white/[0.02] border border-white/[0.04] flex items-center justify-between hover:bg-white/[0.04] transition-colors",
                                                div { class: "flex items-center gap-3",
                                                    span { class: "text-xs font-mono text-purple-400 font-semibold", "#{i:02}" }
                                                    div {
                                                        div { class: "text-xs text-zinc-200 font-medium", "Evento di sicurezza registrato nel blocco #{1000 + i}" }
                                                        div { class: "text-[10px] text-zinc-500 font-mono", "Handshake crittografico PQ completato con successo (0.84ms)" }
                                                    }
                                                }
                                                Badge {
                                                    label: if i % 3 == 0 { "VERIFIED".to_string() } else if i % 2 == 0 { "SYNC".to_string() } else { "LOG".to_string() },
                                                    variant: if i % 3 == 0 { BadgeVariant::Success } else if i % 2 == 0 { BadgeVariant::Warning } else { BadgeVariant::Default },
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if demo_mode() == 1 {
                                ScrollArea {
                                    scroll_type: scroll_type(),
                                    offset_scrollbars: offset_scrollbars(),
                                    fade_edges: fade_edges(),
                                    scroll_x: true,
                                    scroll_y: false,
                                    width: Some("100%".to_string()),
                                    class: "border border-white/[0.06] rounded-lg bg-[#08080a] p-4",
                                    div { class: "flex gap-4 w-max p-2",
                                        for i in 1..=12 {
                                            div {
                                                key: "{i}",
                                                class: "w-64 p-4 rounded-lg bg-zinc-900/60 border border-white/[0.06] shrink-0 space-y-2",
                                                div { class: "flex justify-between items-center",
                                                    span { class: "text-xs font-bold text-zinc-200", "Nodo di Routing #{i}" }
                                                    Badge {
                                                        label: "ATTIVO".to_string(),
                                                        variant: BadgeVariant::Success,
                                                    }
                                                }
                                                div { class: "text-[11px] text-zinc-400 font-mono", "Throughput: {i * 120} MB/s" }
                                                div { class: "text-[10px] text-zinc-500", "Latenza p99: {i * 2 + 5}ms • Zero packet drops" }
                                            }
                                        }
                                    }
                                }
                            } else {
                                ScrollArea {
                                    scroll_type: scroll_type(),
                                    offset_scrollbars: offset_scrollbars(),
                                    fade_edges: fade_edges(),
                                    scroll_x: true,
                                    scroll_y: true,
                                    height: Some("280px".to_string()),
                                    width: Some("100%".to_string()),
                                    class: "border border-white/[0.06] rounded-lg bg-[#08080a] p-3",
                                    div { class: "w-[1000px] space-y-3 p-2",
                                        for row in 1..=15 {
                                            div {
                                                key: "{row}",
                                                class: "grid grid-cols-5 gap-4 p-2.5 rounded bg-white/[0.02] border border-white/[0.04] text-xs font-mono items-center",
                                                span { class: "text-purple-400 font-bold", "TX-ROW-#{row:03}" }
                                                span { class: "text-zinc-300", "0x{row * 4192:x}..{row * 987:x}" }
                                                span { class: "text-emerald-400", "CONFIRMED ({row} blocks)" }
                                                span { class: "text-zinc-400", "Gas: {row * 21000} Gwei" }
                                                span { class: "text-zinc-500 text-right", "Timestamp: 1729170{row:03}Z" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
