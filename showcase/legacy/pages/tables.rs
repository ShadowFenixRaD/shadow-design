use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{CheckControl, Playground, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

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
    let striped = use_signal(|| true);
    let bordered = use_signal(|| false);
    let hoverable = use_signal(|| true);
    let compact = use_signal(|| false);
    let loading = use_signal(|| false);

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
                strong { "{name}" }
            }
        })
        .with_sort_by(|r: &PeerRecord| r.name),
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
        .with_width("120px")
        .with_sort_by(|r: &PeerRecord| r.latency),
        DataTableColumn::new("status", "Stato", |r: &PeerRecord| {
            let st = r.status;
            let variant = match st {
                "Online" => BadgeVariant::Success,
                "Syncing" => BadgeVariant::Primary,
                _ => BadgeVariant::Default,
            };
            rsx! {
                Badge { variant, label: st.to_string(), dot: true }
            }
        })
        .with_width("120px")
        .with_sort_by(|r: &PeerRecord| r.status),
        DataTableColumn::text("tx_rate", "Throughput", |r: &PeerRecord| r.tx_rate.to_string())
            .with_align(TableAlign::Right)
            .with_width("130px")
            .with_sort_by(|r: &PeerRecord| {
                r.tx_rate.split_whitespace().next().and_then(|v| v.parse::<u32>().ok()).unwrap_or(0)
            }),
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
            name: "sort_key",
            r#type: "Option<Signal<Option<String>>>",
            default: "None",
            description: "Signal con la chiave della colonna attualmente ordinata",
        },
        PropRow {
            name: "sort_direction",
            r#type: "Option<Signal<SortDirection>>",
            default: "None",
            description: "Signal con la direzione attiva dell'ordinamento (None, Ascending, Descending)",
        },
        PropRow {
            name: "on_sort",
            r#type: "Option<EventHandler<(String, SortDirection)>>",
            default: "None",
            description: "Callback invocato al cambio di colonna o direzione di ordinamento",
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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Table & DataTable".to_string(),
                description: "Tabelle semantiche responsive ad alte prestazioni con ordinamento interattivo, paginazione, overlay di caricamento e righe selezionabili.".to_string(),
            }

            FeatureBanner { feature_name: "table" }

            ShowcaseTabs {
                preview: rsx! {
                    Playground {
                        stage_start: true,
                        stage: rsx! {
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
                        },
                        controls: rsx! {
                            CheckControl { label: "Righe alternate (Striped)".to_string(), checked: striped }
                            CheckControl { label: "Bordi celle (Bordered)".to_string(), checked: bordered }
                            CheckControl { label: "Hover righe".to_string(), checked: hoverable }
                            CheckControl { label: "Compatta (Compact)".to_string(), checked: compact }
                            CheckControl { label: "Stato di caricamento (Loading)".to_string(), checked: loading }
                        },
                    }
                },
                code: code_snippet,
                props: props,
            }

            // Sezione Primitivi Semplici Table
            div { class: "sc-stack",
                h2 { class: "sc-section-title", "Costruzione con primitive semantiche" }
                p { class: "sc-page-desc", "Uso diretto dei componenti Table, TableHeader, TableBody, TableRow e TableCell per layout personalizzati." }

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
