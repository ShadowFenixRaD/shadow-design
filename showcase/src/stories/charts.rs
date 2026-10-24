//! Story: Charts (feature `charts`). No playground (REVAMP §6): one example per chart type, all
//! visible at once, plus the ops dashboard. Series take `--sd-chart-n` by fallback, so chart-1
//! follows the accent; no series sets an explicit colour.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::scenes::OpsDashboard;
use crate::story::schema::*;

#[component]
fn Area() -> Element {
    rsx! {
        AreaChart {
            data: vec![
                ChartDataPoint::point("Lun", [("eu", 320.0), ("us", 180.0)]),
                ChartDataPoint::point("Mar", [("eu", 410.0), ("us", 210.0)]),
                ChartDataPoint::point("Mer", [("eu", 380.0), ("us", 260.0)]),
                ChartDataPoint::point("Gio", [("eu", 520.0), ("us", 240.0)]),
                ChartDataPoint::point("Ven", [("eu", 470.0), ("us", 300.0)]),
                ChartDataPoint::point("Sab", [("eu", 260.0), ("us", 150.0)]),
                ChartDataPoint::point("Dom", [("eu", 230.0), ("us", 140.0)]),
            ],
            series: vec![
                ChartSeries::new("eu").with_label("Europa"),
                ChartSeries::new("us").with_label("Stati Uniti"),
            ],
            height: 240,
        }
    }
}
const AREA_CODE: &str = r#"AreaChart {
    data: vec![
        ChartDataPoint::point("Lun", [("eu", 320.0), ("us", 180.0)]),
        ChartDataPoint::point("Mar", [("eu", 410.0), ("us", 210.0)]),
        ChartDataPoint::point("Mer", [("eu", 380.0), ("us", 260.0)]),
        ChartDataPoint::point("Gio", [("eu", 520.0), ("us", 240.0)]),
        ChartDataPoint::point("Ven", [("eu", 470.0), ("us", 300.0)]),
        ChartDataPoint::point("Sab", [("eu", 260.0), ("us", 150.0)]),
        ChartDataPoint::point("Dom", [("eu", 230.0), ("us", 140.0)]),
    ],
    series: vec![
        ChartSeries::new("eu").with_label("Europa"),
        ChartSeries::new("us").with_label("Stati Uniti"),
    ],
    height: 240,
}"#;

#[component]
fn Bar() -> Element {
    rsx! {
        BarChart {
            data: vec![
                ChartDataPoint::point("Milano", [("ora", 42.0), ("prima", 35.0)]),
                ChartDataPoint::point("Francoforte", [("ora", 38.0), ("prima", 41.0)]),
                ChartDataPoint::point("Parigi", [("ora", 27.0), ("prima", 22.0)]),
                ChartDataPoint::point("Madrid", [("ora", 19.0), ("prima", 17.0)]),
            ],
            series: vec![
                ChartSeries::new("ora").with_label("Questa settimana"),
                ChartSeries::new("prima").with_label("Settimana scorsa"),
            ],
            height: 240,
        }
    }
}
const BAR_CODE: &str = r#"BarChart {
    data: vec![
        ChartDataPoint::point("Milano", [("ora", 42.0), ("prima", 35.0)]),
        ChartDataPoint::point("Francoforte", [("ora", 38.0), ("prima", 41.0)]),
        ChartDataPoint::point("Parigi", [("ora", 27.0), ("prima", 22.0)]),
        ChartDataPoint::point("Madrid", [("ora", 19.0), ("prima", 17.0)]),
    ],
    series: vec![
        ChartSeries::new("ora").with_label("Questa settimana"),
        ChartSeries::new("prima").with_label("Settimana scorsa"),
    ],
    height: 240,
}"#;

#[component]
fn Line() -> Element {
    rsx! {
        LineChart {
            data: vec![
                ChartDataPoint::point("08:00", [("p50", 18.0), ("p95", 44.0)]),
                ChartDataPoint::point("10:00", [("p50", 21.0), ("p95", 58.0)]),
                ChartDataPoint::point("12:00", [("p50", 26.0), ("p95", 91.0)]),
                ChartDataPoint::point("14:00", [("p50", 23.0), ("p95", 72.0)]),
                ChartDataPoint::point("16:00", [("p50", 19.0), ("p95", 49.0)]),
                ChartDataPoint::point("18:00", [("p50", 17.0), ("p95", 41.0)]),
            ],
            series: vec![
                ChartSeries::new("p50").with_label("p50 (ms)"),
                ChartSeries::new("p95").with_label("p95 (ms)").with_dashed(true),
            ],
            height: 240,
        }
    }
}
const LINE_CODE: &str = r#"LineChart {
    data: vec![
        ChartDataPoint::point("08:00", [("p50", 18.0), ("p95", 44.0)]),
        ChartDataPoint::point("10:00", [("p50", 21.0), ("p95", 58.0)]),
        ChartDataPoint::point("12:00", [("p50", 26.0), ("p95", 91.0)]),
        ChartDataPoint::point("14:00", [("p50", 23.0), ("p95", 72.0)]),
        ChartDataPoint::point("16:00", [("p50", 19.0), ("p95", 49.0)]),
        ChartDataPoint::point("18:00", [("p50", 17.0), ("p95", 41.0)]),
    ],
    series: vec![
        ChartSeries::new("p50").with_label("p50 (ms)"),
        ChartSeries::new("p95").with_label("p95 (ms)").with_dashed(true),
    ],
    height: 240,
}"#;

#[component]
fn Pie() -> Element {
    rsx! {
        SimpleGrid { cols: 1, sm: Some(2), spacing: GridSpacing::Md,
            PieChart {
                data: vec![
                    PieSegment::new("QUIC", 48.0),
                    PieSegment::new("TCP", 31.0),
                    PieSegment::new("WebRTC", 21.0),
                ],
                height: 200,
            }
            DonutChart {
                data: vec![
                    PieSegment::new("Attivi", 412.0),
                    PieSegment::new("Inattivi", 63.0),
                    PieSegment::new("Bloccati", 9.0),
                ],
                center_label: Some("484".to_string()),
                center_sublabel: Some("utenti".to_string()),
                height: 200,
            }
        }
    }
}
const PIE_CODE: &str = r#"SimpleGrid { cols: 1, sm: Some(2), spacing: GridSpacing::Md,
    PieChart {
        data: vec![
            PieSegment::new("QUIC", 48.0),
            PieSegment::new("TCP", 31.0),
            PieSegment::new("WebRTC", 21.0),
        ],
        height: 200,
    }
    DonutChart {
        data: vec![
            PieSegment::new("Attivi", 412.0),
            PieSegment::new("Inattivi", 63.0),
            PieSegment::new("Bloccati", 9.0),
        ],
        center_label: Some("484".to_string()),
        center_sublabel: Some("utenti".to_string()),
        height: 200,
    }
}"#;

#[component]
fn Radar() -> Element {
    rsx! {
        RadarChart {
            data: vec![
                RadarDataPoint::new("Latenza").with_value("edge-07", 88.0).with_value("edge-22", 64.0),
                RadarDataPoint::new("Throughput").with_value("edge-07", 72.0).with_value("edge-22", 91.0),
                RadarDataPoint::new("Uptime").with_value("edge-07", 97.0).with_value("edge-22", 93.0),
                RadarDataPoint::new("Peer").with_value("edge-07", 60.0).with_value("edge-22", 78.0),
                RadarDataPoint::new("Cifratura").with_value("edge-07", 95.0).with_value("edge-22", 95.0),
            ],
            series: vec![ChartSeries::new("edge-07"), ChartSeries::new("edge-22")],
            height: 260,
        }
    }
}
const RADAR_CODE: &str = r#"RadarChart {
    data: vec![
        RadarDataPoint::new("Latenza").with_value("edge-07", 88.0).with_value("edge-22", 64.0),
        RadarDataPoint::new("Throughput").with_value("edge-07", 72.0).with_value("edge-22", 91.0),
        RadarDataPoint::new("Uptime").with_value("edge-07", 97.0).with_value("edge-22", 93.0),
        RadarDataPoint::new("Peer").with_value("edge-07", 60.0).with_value("edge-22", 78.0),
        RadarDataPoint::new("Cifratura").with_value("edge-07", 95.0).with_value("edge-22", 95.0),
    ],
    series: vec![ChartSeries::new("edge-07"), ChartSeries::new("edge-22")],
    height: 260,
}"#;

#[component]
fn Scatter() -> Element {
    rsx! {
        ScatterChart {
            data: vec![
                ScatterPoint::new(12.0, 14.0).with_label("edge-07"),
                ScatterPoint::new(28.0, 19.0).with_label("edge-22"),
                ScatterPoint::new(41.0, 26.0).with_label("edge-31"),
                ScatterPoint::new(55.0, 31.0).with_label("relay-mi"),
                ScatterPoint::new(68.0, 47.0).with_label("relay-fra"),
                ScatterPoint::new(83.0, 72.0).with_label("edge-21"),
            ],
            x_label: Some("Carico (%)".to_string()),
            y_label: Some("Latenza (ms)".to_string()),
            height: 240,
        }
    }
}
const SCATTER_CODE: &str = r#"ScatterChart {
    data: vec![
        ScatterPoint::new(12.0, 14.0).with_label("edge-07"),
        ScatterPoint::new(28.0, 19.0).with_label("edge-22"),
        ScatterPoint::new(41.0, 26.0).with_label("edge-31"),
        ScatterPoint::new(55.0, 31.0).with_label("relay-mi"),
        ScatterPoint::new(68.0, 47.0).with_label("relay-fra"),
        ScatterPoint::new(83.0, 72.0).with_label("edge-21"),
    ],
    x_label: Some("Carico (%)".to_string()),
    y_label: Some("Latenza (ms)".to_string()),
    height: 240,
}"#;

#[component]
fn Sparklines() -> Element {
    rsx! {
        Stack { gap: SpacingToken::Md,
            Group { justify: JustifyContent::SpaceBetween, wrap: false,
                Stack { gap: SpacingToken::Xs,
                    span { class: "sc-eyebrow", "Richieste/s" }
                    span { class: "sc-num", "4.210 · +12%" }
                }
                Sparkline { data: vec![3.1, 3.4, 3.2, 3.8, 3.9, 4.0, 4.2], trend: TrendDirection::Positive }
            }
            Group { justify: JustifyContent::SpaceBetween, wrap: false,
                Stack { gap: SpacingToken::Xs,
                    span { class: "sc-eyebrow", "Errori" }
                    span { class: "sc-num", "0,4% · +0,3 pt" }
                }
                Sparkline { data: vec![0.1, 0.1, 0.2, 0.1, 0.3, 0.3, 0.4], trend: TrendDirection::Negative }
            }
            Group { justify: JustifyContent::SpaceBetween, wrap: false,
                Stack { gap: SpacingToken::Xs,
                    span { class: "sc-eyebrow", "Peer attivi" }
                    span { class: "sc-num", "128 · stabile" }
                }
                Sparkline { data: vec![127.0, 129.0, 128.0, 128.0, 127.0, 129.0, 128.0], trend: TrendDirection::Neutral, with_fill: false }
            }
        }
    }
}
const SPARKLINES_CODE: &str = r#"Stack { gap: SpacingToken::Md,
    Group { justify: JustifyContent::SpaceBetween, wrap: false,
        Stack { gap: SpacingToken::Xs,
            span { "Richieste/s" }
            span { "4.210 · +12%" }
        }
        Sparkline { data: vec![3.1, 3.4, 3.2, 3.8, 3.9, 4.0, 4.2], trend: TrendDirection::Positive }
    }
    Group { justify: JustifyContent::SpaceBetween, wrap: false,
        Stack { gap: SpacingToken::Xs,
            span { "Errori" }
            span { "0,4% · +0,3 pt" }
        }
        Sparkline { data: vec![0.1, 0.1, 0.2, 0.1, 0.3, 0.3, 0.4], trend: TrendDirection::Negative }
    }
    Group { justify: JustifyContent::SpaceBetween, wrap: false,
        Stack { gap: SpacingToken::Xs,
            span { "Peer attivi" }
            span { "128 · stabile" }
        }
        Sparkline { data: vec![127.0, 129.0, 128.0, 128.0, 127.0, 129.0, 128.0], trend: TrendDirection::Neutral, with_fill: false }
    }
}"#;

#[component]
fn Gauges() -> Element {
    rsx! {
        SimpleGrid { cols: 1, sm: Some(2), spacing: GridSpacing::Md,
            GaugeChart {
                value: 78.5,
                label: Some("Disco".to_string()),
                sublabel: Some("785 GB di 1 TB".to_string()),
                thresholds: vec![
                    (0.0, "var(--sd-success)".to_string()),
                    (70.0, "var(--sd-warning)".to_string()),
                    (90.0, "var(--sd-danger)".to_string()),
                ],
                height: 200,
            }
            RadialBar { value: 64.0, label: Some("CPU".to_string()), height: 180 }
        }
    }
}
const GAUGES_CODE: &str = r#"SimpleGrid { cols: 1, sm: Some(2), spacing: GridSpacing::Md,
    GaugeChart {
        value: 78.5,
        label: Some("Disco".to_string()),
        sublabel: Some("785 GB di 1 TB".to_string()),
        thresholds: vec![
            (0.0, "var(--sd-success)".to_string()),
            (70.0, "var(--sd-warning)".to_string()),
            (90.0, "var(--sd-danger)".to_string()),
        ],
        height: 200,
    }
    RadialBar { value: 64.0, label: Some("CPU".to_string()), height: 180 }
}"#;

#[component]
fn HeatmapDemo() -> Element {
    let cells: Vec<HeatmapCell> = (0..7)
        .flat_map(|day| {
            (0..12).map(move |slot| {
                let busy = if (3..10).contains(&slot) && day < 5 { 20.0 } else { 4.0 };
                HeatmapCell::new(slot, day, busy + ((slot * 7 + day * 5) % 11) as f64)
            })
        })
        .collect();
    rsx! {
        Heatmap {
            data: cells,
            x_labels: ["0", "2", "4", "6", "8", "10", "12", "14", "16", "18", "20", "22"].map(String::from).to_vec(),
            y_labels: ["Lun", "Mar", "Mer", "Gio", "Ven", "Sab", "Dom"].map(String::from).to_vec(),
            cell_size: 16,
            height: 200,
        }
    }
}
const HEATMAP_CODE: &str = r#"let cells: Vec<HeatmapCell> = (0..7)
    .flat_map(|day| {
        (0..12).map(move |slot| {
            let busy = if (3..10).contains(&slot) && day < 5 { 20.0 } else { 4.0 };
            HeatmapCell::new(slot, day, busy + ((slot * 7 + day * 5) % 11) as f64)
        })
    })
    .collect();

Heatmap {
    data: cells,
    x_labels: ["0", "2", "4", "6", "8", "10", "12", "14", "16", "18", "20", "22"].map(String::from).to_vec(),
    y_labels: ["Lun", "Mar", "Mer", "Gio", "Ven", "Sab", "Dom"].map(String::from).to_vec(),
    cell_size: 16,
    height: 200,
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        OpsDashboard {}
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/ops_dashboard.rs (estratto): metric card con Sparkline e traffico per serie.
Sparkline { data, trend, width: 160, height: 32 }

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
}"#;

const CARTESIAN_PROPS: &[PropDoc] = &[
    prop("data", "Vec<ChartDataPoint>", "-", "Un punto per etichetta dell'asse X, con un valore per serie."),
    prop("series", "Vec<ChartSeries>", "-", "Serie da disegnare, nell'ordine dei colori --sd-chart-1..8."),
    prop("curve_type", "CurveType", "Monotone", "Area e Line: Monotone, Linear o Step."),
    prop("stacked", "bool", "true", "Solo AreaChart: impila le serie; false le sovrappone."),
    prop("with_gradient", "bool", "true", "Solo AreaChart: riempimento sfumato sotto la linea."),
    prop("with_dots", "bool", "true", "Area e Line: punti sui valori."),
    prop("stroke_width / connect_nulls", "f64 / bool", "2.5 / true", "Solo LineChart: spessore e continuità sui valori mancanti."),
    prop("orientation / chart_type", "ChartOrientation / BarChartType", "Vertical / Default", "Solo BarChart: barre orizzontali; Default, Stacked o Percent."),
    prop("bar_radius", "u32", "4", "Solo BarChart: raggio degli angoli delle barre."),
    prop("with_grid / with_tooltip / with_legend", "bool", "true", "Griglia, tooltip al passaggio e legenda cliccabile."),
    prop("height", "u32", "300", "Altezza minima in px; la larghezza segue il contenitore."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const DATA_PROPS: &[PropDoc] = &[
    prop("ChartDataPoint::point(label, [(key, v), …])", "fn", "-", "Punto con più valori in una riga."),
    prop("ChartDataPoint::new(label).with_value(key, v)", "fn", "-", "Costruzione per passi; with_optional_value per i buchi."),
    prop("ChartSeries::new(key)", "fn", "-", "La chiave lega la serie ai valori dei punti."),
    prop(".with_label(..)", "fn", "-", "Nome mostrato in legenda e tooltip."),
    prop(".with_dashed(true)", "fn", "-", "Linea tratteggiata: distingue una serie anche senza colore."),
];
const PIE_PROPS: &[PropDoc] = &[
    prop("data", "Vec<PieSegment>", "-", "PieSegment::new(nome, valore); le percentuali si calcolano dal totale."),
    prop("inner_radius", "Option<f64> / f64", "None / 65.0", "PieChart pieno; DonutChart con foro centrale."),
    prop("center_label / center_sublabel", "Option<String>", "None", "Testo al centro della ciambella."),
    prop("with_labels", "bool", "false", "Percentuali sugli spicchi."),
    prop("with_tooltip", "bool", "true", "Tooltip al passaggio."),
    prop("stroke_width", "f64", "2.0", "Separazione tra gli spicchi."),
    prop("height", "u32", "300", "Altezza in px."),
];
const RADAR_PROPS: &[PropDoc] = &[
    prop("data", "Vec<RadarDataPoint>", "-", "Un asse per punto: RadarDataPoint::new(asse).with_value(serie, v)."),
    prop("series", "Vec<ChartSeries>", "-", "Una serie per poligono."),
    prop("levels", "usize", "4", "Anelli concentrici della griglia."),
    prop("with_dots / with_legend / with_tooltip", "bool", "true", "Punti, legenda e tooltip."),
    prop("height", "u32", "320", "Altezza in px."),
];
const SCATTER_PROPS: &[PropDoc] = &[
    prop("data", "Vec<ScatterPoint>", "-", "ScatterPoint::new(x, y) con .with_label, .with_series e .with_z (raggio in BubbleChart)."),
    prop("x_label / y_label", "Option<String>", "None", "Titoli degli assi."),
    prop("x_scale / y_scale", "ScaleType", "Linear", "Solo ScatterChart: Linear o Log."),
    prop("with_grid / with_tooltip", "bool", "true", "Solo ScatterChart: griglia e tooltip."),
    prop("height", "u32", "300", "Altezza in px."),
];
const SPARK_PROPS: &[PropDoc] = &[
    prop("data", "Vec<f64>", "-", "Valori in ordine di tempo, senza assi."),
    prop("trend", "TrendDirection", "Auto", "Auto (dal primo e ultimo valore), Positive, Negative, Neutral: sceglie la tinta."),
    prop("with_fill", "bool", "true", "Riempimento sotto la linea."),
    prop("curve_type", "CurveType", "Monotone", "Forma della linea."),
    prop("stroke_width", "f64", "2.0", "Spessore della linea."),
    prop("width / height", "u32", "120 / 36", "Dimensioni in px."),
    prop("color", "Option<String>", "None", "Colore CSS esplicito; senza, segue trend."),
];
const GAUGE_PROPS: &[PropDoc] = &[
    prop("value", "f64", "-", "Valore corrente, limitato tra min e max."),
    prop("min / max", "f64", "0.0 / 100.0", "Intervallo."),
    prop("thickness", "u32", "14 / 10", "Spessore dell'arco (GaugeChart / RadialBar)."),
    prop("label / sublabel", "Option<String>", "None", "Testo sotto il valore; RadialBar ha solo label."),
    prop("thresholds", "Vec<(f64, String)>", "[]", "Solo GaugeChart: colore per soglia, come var(--sd-warning)."),
    prop("radius", "f64", "60.0", "Solo RadialBar: raggio dell'anello."),
    prop("color", "Option<String>", "None", "Colore CSS esplicito; senza, --sd-chart-1."),
    prop("height", "u32", "220 / 160", "Altezza in px."),
];
const HEATMAP_PROPS: &[PropDoc] = &[
    prop("data", "Vec<HeatmapCell>", "-", "HeatmapCell::new(x, y, valore) con .with_label o .with_date per il tooltip."),
    prop("x_labels / y_labels", "Vec<String>", "[]", "Etichette di colonne e righe; CalendarHeatmap usa i giorni da Locale."),
    prop("color_scale", "Vec<String>", "[]", "Scala di colori CSS; vuota usa cinque passi dell'accento."),
    prop("cell_size / cell_gap / cell_radius", "u32", "14 / 3 / 2", "Geometria delle celle (CalendarHeatmap: 11 / 3)."),
    prop("with_tooltip", "bool", "true", "Valore al passaggio."),
    prop("height", "u32", "220", "Altezza in px (CalendarHeatmap: 140)."),
];

fn example(id: &'static str, title: &'static str, description: &'static str, code: &'static str, render: fn() -> Element) -> Example {
    Example { id, title, description, code, render, span: Span::Half }
}

pub fn story() -> Story {
    Story {
        slug: "charts",
        name: "Charts",
        category: Category::Dati,
        features: &["charts"],
        icon: IconKind::Sparkles,
        tagline: "Nove tipi di grafico SVG scritti in Rust, senza JavaScript, che prendono i colori dal tema.",
        keywords: &["chart", "grafico", "area", "barre", "linee", "torta", "radar", "sparkline", "heatmap", "gauge"],
        status: Status::Stable,
        playground: None,
        examples: vec![
            example("area", "AreaChart", "Traffico per regione, serie impilate con sfumatura.", AREA_CODE, || rsx! { Area {} }),
            example("bar", "BarChart", "Confronto settimanale per città, barre raggruppate.", BAR_CODE, || rsx! { Bar {} }),
            example("line", "LineChart", "Latenza p50 e p95; la seconda tratteggiata.", LINE_CODE, || rsx! { Line {} }),
            example("pie", "PieChart e DonutChart", "Quote del totale; la ciambella porta un KPI al centro.", PIE_CODE, || rsx! { Pie {} }),
            example("radar", "RadarChart", "Due nodi a confronto su cinque assi.", RADAR_CODE, || rsx! { Radar {} }),
            example("scatter", "ScatterChart", "Carico contro latenza, un punto per nodo.", SCATTER_CODE, || rsx! { Scatter {} }),
            example("sparkline", "Sparkline", "Tendenze compatte: la tinta segue trend.", SPARKLINES_CODE, || rsx! { Sparklines {} }),
            example("gauge", "GaugeChart e RadialBar", "Soglie semantiche con token, anello in accento.", GAUGES_CODE, || rsx! { Gauges {} }),
            example("heatmap", "Heatmap", "Carico per giorno e fascia oraria, scala sull'accento.", HEATMAP_CODE, || rsx! { HeatmapDemo {} }),
            Example {
                id: "context",
                title: "In contesto",
                description: "La dashboard operativa: sparkline nelle metric card e area per il traffico.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "AreaChart / BarChart / LineChart", rows: CARTESIAN_PROPS },
            PropGroup { component: "ChartDataPoint / ChartSeries", rows: DATA_PROPS },
            PropGroup { component: "PieChart / DonutChart", rows: PIE_PROPS },
            PropGroup { component: "RadarChart", rows: RADAR_PROPS },
            PropGroup { component: "ScatterChart / BubbleChart", rows: SCATTER_PROPS },
            PropGroup { component: "Sparkline", rows: SPARK_PROPS },
            PropGroup { component: "GaugeChart / RadialBar", rows: GAUGE_PROPS },
            PropGroup { component: "Heatmap / CalendarHeatmap", rows: HEATMAP_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Data visualisation (language.md §3.13) in un container (§3.1)",
            depth: "Flat-lit (grafico) · Raised (tooltip)",
            tokens: &["--sd-chart-1", "--sd-chart-2", "--sd-border-subtle", "--sd-text-subtle", "--sd-font-mono", "--sd-text-2xs", "--sd-glow-strength"],
            notes: "Le serie prendono --sd-chart-n in ordine e chart-1 è l'accento: cambiare preset ricolora ogni grafico. Il glow delle linee segue GlowMode.",
        }),
        a11y: &[
            "Le voci di legenda sono <button> con aria-pressed: da tastiera si nasconde e si mostra una serie.",
            "Il tooltip (role=tooltip) compare solo al passaggio del puntatore: scrivi i valori chiave anche nel testo vicino.",
            "L'SVG non ha titolo né ruolo propri: dai al grafico un titolo visibile (per esempio il title della Card).",
            "Non affidarti solo al colore: usa with_dashed per una seconda serie e le etichette per le soglie.",
        ],
        related: &["cards", "tables", "progress", "badges"],
    }
}
