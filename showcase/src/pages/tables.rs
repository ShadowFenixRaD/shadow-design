use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[derive(Clone, PartialEq, Debug)]
pub struct PeerRecord {
    pub id: &'static str,
    pub name: &'static str,
    pub latency: u32,
    pub region: &'static str,
    pub status: &'static str,
    pub tx_rate: &'static str,
}

#[component]
pub fn TablesPage() -> Element {
    let mut striped = use_signal(|| true);
    let mut bordered = use_signal(|| false);
    let mut hoverable = use_signal(|| true);
    let mut compact = use_signal(|| false);
    let mut loading = use_signal(|| false);

    let page = use_signal(|| 1usize);
    let sort_key = use_signal(|| Some("latency".to_string()));
    let sort_direction = use_signal(|| SortDirection::Ascending);

    let all_records = vec![
        PeerRecord { id: "node-01", name: "Relay Alpha", latency: 8, region: "EU-Central", status: "Online", tx_rate: "128 MB/s" },
        PeerRecord { id: "node-02", name: "Relay Beta", latency: 14, region: "EU-West", status: "Online", tx_rate: "94 MB/s" },
        PeerRecord { id: "node-03", name: "Gate Gamma", latency: 26, region: "US-East", status: "Online", tx_rate: "65 MB/s" },
        PeerRecord { id: "node-04", name: "Router Delta", latency: 42, region: "US-West", status: "Syncing", tx_rate: "31 MB/s" },
        PeerRecord { id: "node-05", name: "Relay Epsilon", latency: 58, region: "AP-East", status: "Online", tx_rate: "48 MB/s" },
        PeerRecord { id: "node-06", name: "Storage Zeta", latency: 74, region: "SA-East", status: "Syncing", tx_rate: "19 MB/s" },
        PeerRecord { id: "node-07", name: "Sensor Eta", latency: 91, region: "AF-South", status: "Offline", tx_rate: "0 MB/s" },
        PeerRecord { id: "node-08", name: "Cluster Theta", latency: 110, region: "OC-East", status: "Offline", tx_rate: "0 MB/s" },
    ];

    let columns = vec![
        DataTableColumn::text("id", "ID Nodo", |r: &PeerRecord| r.id.to_string())
            .with_width("110px"),
        DataTableColumn::new("name", "Nome Dispositivo", |r: &PeerRecord| {
            let name = r.name;
            rsx! {
                span { class: "font-semibold text-zinc-100", "{name}" }
            }
        }),
        DataTableColumn::text("region", "Regione", |r: &PeerRecord| r.region.to_string())
            .with_width("130px"),
        DataTableColumn::new("latency", "Latenza", |r: &PeerRecord| {
            let lat = r.latency;
            let badge_variant = if lat < 30 {
                BadgeVariant::Success
            } else if lat < 70 {
                BadgeVariant::Warning
            } else {
                BadgeVariant::Danger
            };
            let label_str = format!("{lat} ms");
            rsx! {
                Badge { variant: badge_variant, label: label_str }
            }
        })
        .with_align(TableAlign::Center)
        .with_width("120px"),
        DataTableColumn::new("status", "Stato", |r: &PeerRecord| {
            let st = r.status;
            let (dot_color, text_color) = match st {
                "Online" => ("bg-emerald-400 shadow-[0_0_8px_rgba(52,211,153,0.5)]", "text-emerald-300"),
                "Syncing" => ("bg-purple-400 shadow-[0_0_8px_rgba(168,85,247,0.5)]", "text-purple-300"),
                _ => ("bg-zinc-600", "text-zinc-500"),
            };
            rsx! {
                div { class: "inline-flex items-center gap-2",
                    span { class: "w-2 h-2 rounded-full {dot_color}" }
                    span { class: "text-xs font-mono {text_color}", "{st}" }
                }
            }
        })
        .with_width("120px"),
        DataTableColumn::text("tx_rate", "Throughput", |r: &PeerRecord| r.tx_rate.to_string())
            .with_align(TableAlign::Right)
            .with_width("130px"),
    ];

    let props = vec![
        PropRow {
            name: "striped",
            r#type: "bool",
            default: "false",
            description: "Alterna il colore di sfondo delle righe pari per una migliore leggibilità",
        },
        PropRow {
            name: "bordered",
            r#type: "bool",
            default: "false",
            description: "Aggiunge bordi verticali e orizzontali tra tutte le celle",
        },
        PropRow {
            name: "hoverable",
            r#type: "bool",
            default: "true",
            description: "Evidenzia con micro-glow e accento viola le righe al passaggio del mouse",
        },
        PropRow {
            name: "compact",
            r#type: "bool",
            default: "false",
            description: "Riduce il padding delle celle per tabelle ad alta densità informativa",
        },
        PropRow {
            name: "sticky_header",
            r#type: "bool",
            default: "false",
            description: "Mantiene l'intestazione fissa durante lo scorrimento verticale",
        },
        PropRow {
            name: "page",
            r#type: "Option<Signal<usize>>",
            default: "None",
            description: "Signal della pagina corrente per paginazione attiva",
        },
        PropRow {
            name: "loading",
            r#type: "bool",
            default: "false",
            description: "Mostra overlay semitrasparente con blur e spinner di caricamento",
        },
    ];

    let code_snippet = r#"DataTable {
    data: peer_records,
    columns: columns,
    page: Some(page),
    page_size: 4,
    sort_key: Some(sort_key),
    sort_direction: Some(sort_direction),
    striped: true,
    hoverable: true,
}"#
    .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Table & DataTable" }
                Text { dimmed: true, size: TextSize::Sm, "Tabelle semantiche responsive ad alte prestazioni con ordinamento interattivo, paginazione, overlay di caricamento e righe selezionabili." }
            }

            FeatureBanner { feature_name: "table" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "space-y-6",
                        // Live DataTable
                        DataTable {
                            data: all_records,
                            columns: columns,
                            page: Some(page),
                            page_size: 4,
                            sort_key: Some(sort_key),
                            sort_direction: Some(sort_direction),
                            striped: striped(),
                            bordered: bordered(),
                            hoverable: hoverable(),
                            compact: compact(),
                            loading: loading(),
                        }

                        // Controls
                        div { class: "p-5 rounded-xl bg-zinc-950/60 border border-white/[0.08] flex flex-wrap gap-6 items-center text-xs text-zinc-300",
                            label { class: "flex items-center gap-2 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    checked: striped(),
                                    onchange: move |e| striped.set(e.checked())
                                }
                                "Righe Alternate (Striped)"
                            }
                            label { class: "flex items-center gap-2 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    checked: bordered(),
                                    onchange: move |e| bordered.set(e.checked())
                                }
                                "Bordi Celle (Bordered)"
                            }
                            label { class: "flex items-center gap-2 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    checked: hoverable(),
                                    onchange: move |e| hoverable.set(e.checked())
                                }
                                "Hover Righe"
                            }
                            label { class: "flex items-center gap-2 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    checked: compact(),
                                    onchange: move |e| compact.set(e.checked())
                                }
                                "Compatta (Compact)"
                            }
                            label { class: "flex items-center gap-2 cursor-pointer",
                                input {
                                    r#type: "checkbox",
                                    checked: loading(),
                                    onchange: move |e| loading.set(e.checked())
                                }
                                "Stato di Caricamento (Loading)"
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Sezione Primitivi Semplici Table
            div { class: "space-y-4 pt-4 border-t border-white/[0.06]",
                Heading { order: HeadingOrder::H3, "Costruzione con Primitive Semantiche" }
                Text { dimmed: true, size: TextSize::Sm, "Uso diretto dei componenti Table, TableHeader, TableBody, TableRow e TableCell per layout personalizzati." }

                Table {
                    striped: true,
                    hoverable: true,
                    TableHeader {
                        TableRow {
                            TableCell { is_header: true, "Pacchetto Algoritmo" }
                            TableCell { is_header: true, align: TableAlign::Center, "Standard Post-Quantico" }
                            TableCell { is_header: true, align: TableAlign::Right, "Dimensione Chiave" }
                        }
                    }
                    TableBody {
                        TableRow {
                            TableCell { "Kyber-768" }
                            TableCell { align: TableAlign::Center,
                                Badge { variant: BadgeVariant::Success, label: "NIST Round 4".to_string() }
                            }
                            TableCell { align: TableAlign::Right, "1.184 byte" }
                        }
                        TableRow {
                            TableCell { "Dilithium-3" }
                            TableCell { align: TableAlign::Center,
                                Badge { variant: BadgeVariant::Success, label: "NIST Round 4".to_string() }
                            }
                            TableCell { align: TableAlign::Right, "4.000 byte" }
                        }
                        TableRow {
                            TableCell { "Falcon-512" }
                            TableCell { align: TableAlign::Center,
                                Badge { variant: BadgeVariant::Warning, label: "NIST FIPS".to_string() }
                            }
                            TableCell { align: TableAlign::Right, "897 byte" }
                        }
                    }
                }
            }
        }
    }
}
