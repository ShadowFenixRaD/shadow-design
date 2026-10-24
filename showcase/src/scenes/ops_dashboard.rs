//! Scene: the "Shadowfenix Mesh" operations dashboard (REVAMP §1.3).
//! Metric cards with sparklines, a three-series area chart, a sortable data table,
//! a range picker and one primary action.
use dioxus::prelude::*;
use shadow_design::prelude::*;

#[derive(Clone, PartialEq)]
struct NodeRow {
    name: &'static str,
    region: &'static str,
    status: &'static str,
    latency: f64,
    peers: u32,
}

const NODES: [NodeRow; 5] = [
    NodeRow { name: "edge-07", region: "Milano", status: "Online", latency: 0.9, peers: 42 },
    NodeRow { name: "edge-12", region: "Francoforte", status: "Online", latency: 1.4, peers: 37 },
    NodeRow { name: "relay-03", region: "Parigi", status: "Degradato", latency: 3.8, peers: 18 },
    NodeRow { name: "core-01", region: "Amsterdam", status: "Online", latency: 0.6, peers: 64 },
    NodeRow { name: "edge-21", region: "Madrid", status: "Offline", latency: 0.0, peers: 0 },
];

fn status_badge(status: &str) -> BadgeVariant {
    match status {
        "Online" => BadgeVariant::Success,
        "Degradato" => BadgeVariant::Warning,
        _ => BadgeVariant::Danger,
    }
}

fn columns(compact: bool) -> Vec<DataTableColumn<NodeRow>> {
    let mut cols = vec![DataTableColumn::new("name", "Nodo", |r: &NodeRow| {
        let name = r.name;
        rsx! {
            span { class: "sc-num", "{name}" }
        }
    })
    .with_sort_by(|r: &NodeRow| r.name)];
    if !compact {
        cols.push(DataTableColumn::text("region", "Regione", |r: &NodeRow| r.region.to_string()));
    }
    cols.push(
        DataTableColumn::new("status", "Stato", |r: &NodeRow| {
            let status = r.status;
            rsx! {
                Badge { label: status.to_string(), variant: status_badge(status), pill: true, dot: true }
            }
        })
        .with_sort_by(|r: &NodeRow| r.status),
    );
    cols.push(
        DataTableColumn::new("latency", "Latenza", |r: &NodeRow| {
            let text = if r.peers == 0 { "—".to_string() } else { format!("{:.1} ms", r.latency) };
            rsx! {
                span { class: "sc-num", "{text}" }
            }
        })
        .with_align(TableAlign::Right)
        .with_sort_by(|r: &NodeRow| (r.latency * 10.0) as i64),
    );
    if !compact {
        cols.push(
            DataTableColumn::new("peers", "Peer", |r: &NodeRow| {
                let peers = r.peers;
                rsx! {
                    span { class: "sc-num", "{peers}" }
                }
            })
            .with_align(TableAlign::Right)
            .with_sort_by(|r: &NodeRow| r.peers),
        );
    }
    cols
}

/// Six points per range; series `latenza`, `throughput`, `errori` take their colours
/// from the `--sd-chart-n` fallback (no `with_color`), so chart-1 follows the accent.
fn traffic(range: &str) -> Vec<ChartDataPoint> {
    let (labels, scale): ([&str; 6], f64) = match range {
        "24h" => (["00", "04", "08", "12", "16", "20"], 0.4),
        "30g" => (["S1", "S2", "S3", "S4", "S5", "S6"], 1.6),
        _ => (["Gen", "Feb", "Mar", "Apr", "Mag", "Giu"], 1.0),
    };
    let base = [(34.0, 52.0, 6.0), (41.0, 58.0, 4.0), (38.0, 66.0, 7.0), (52.0, 71.0, 3.0), (47.0, 84.0, 5.0), (58.0, 92.0, 2.0)];
    labels
        .iter()
        .zip(base)
        .map(|(l, (lat, thr, err))| {
            ChartDataPoint::new(*l)
                .with_value("latenza", lat * scale)
                .with_value("throughput", thr * scale)
                .with_value("errori", err * scale)
        })
        .collect()
}

#[component]
fn Metric(label: &'static str, value: &'static str, delta: &'static str, data: Vec<f64>, trend: TrendDirection) -> Element {
    rsx! {
        Card { class: "sc-metric".to_string(),
            div { class: "sc-metric__body",
                div { class: "sc-metric__text",
                    span { class: "sc-eyebrow", "{label}" }
                    span { class: "sc-metric__value", "{value}" }
                    span { class: "sc-hint", "{delta}" }
                }
                Sparkline { data, trend, width: 160, height: 32, class: "sc-metric__spark" }
            }
        }
    }
}

#[component]
pub fn OpsDashboard(#[props(default)] compact: bool) -> Element {
    let range = use_signal(|| "7g".to_string());
    let mut toast = use_notification();
    let data = traffic(&range());
    let metric_cols = if compact { None } else { Some(3) };

    rsx! {
        div { class: "sc-scene-body",
            div { class: "sc-scene__head",
                div { class: "sc-scene__title-group",
                    span { class: "sc-eyebrow", "Shadowfenix Mesh" }
                    span { class: "sc-scene__title", "Operazioni di rete" }
                }
                Group { gap: SpacingToken::Sm, align: AlignItems::Center,
                    SegmentedControl {
                        size: SegmentedControlSize::Sm,
                        options: vec![
                            SegmentOption::new("24h", "24h"),
                            SegmentOption::new("7g", "7g"),
                            SegmentOption::new("30g", "30g"),
                        ],
                        value: range,
                    }
                    Button {
                        size: ButtonSize::Sm,
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Plus, size: 14 }
                        }),
                        onclick: move |_| {
                            toast.success("Nodo edge-22 in provisioning");
                        },
                        "Nuovo nodo"
                    }
                    Tooltip { content: "Giulia Ferri · in linea".to_string(), position: TooltipPosition::Bottom,
                        Avatar { name: Some("Giulia Ferri".to_string()), size: AvatarSize::Sm, status: Some(PresenceStatus::Online) }
                    }
                }
            }

            SimpleGrid { cols: 1, sm: metric_cols, spacing: GridSpacing::Sm,
                Metric {
                    label: "Latenza media",
                    value: "1.2 ms",
                    delta: "−8% rispetto a ieri",
                    data: vec![1.6, 1.5, 1.7, 1.4, 1.3, 1.25, 1.2],
                    trend: TrendDirection::Positive,
                }
                Metric {
                    label: "Richieste",
                    value: "4.8M",
                    delta: "+12% sulla settimana",
                    data: vec![3.1, 3.4, 3.3, 3.9, 4.2, 4.5, 4.8],
                    trend: TrendDirection::Positive,
                }
                Metric {
                    label: "Affidabilità",
                    value: "99.99%",
                    delta: "1 incidente aperto",
                    data: vec![99.99, 99.98, 99.99, 99.97, 99.99, 99.99, 99.99],
                    trend: TrendDirection::Neutral,
                }
            }

            Card { title: Some("Traffico per nodo".to_string()), subtitle: Some("Latenza, throughput ed errori".to_string()),
                buttons: Some(rsx! {
                    Badge { label: "Live".to_string(), variant: BadgeVariant::Success, pill: true, dot: true }
                }),
                AreaChart {
                    data,
                    series: vec![
                        ChartSeries::new("latenza").with_label("Latenza"),
                        ChartSeries::new("throughput").with_label("Throughput"),
                        ChartSeries::new("errori").with_label("Errori"),
                    ],
                    stacked: false,
                    with_dots: false,
                    height: if compact { 160 } else { 180 },
                }
            }

            DataTable { data: NODES.to_vec(), columns: columns(compact), page_size: 5, compact: true }
        }
    }
}
