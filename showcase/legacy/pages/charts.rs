use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    Controls, FeatureBanner, OptionPicker, PageHeader, PropRow, ShowcaseTabs, Stage,
};

#[component]
pub fn ChartsPage() -> Element {
    let selected_chart_tab = use_signal(|| "area".to_string());
    let heatmap_mode = use_signal(|| "month".to_string());

    // Dati di esempio per grafici cartesiani (Area, Bar, Line)
    let sample_points = vec![
        ChartDataPoint::point("Gen", [("revenue", 120.0), ("costs", 80.0), ("profit", 40.0)]),
        ChartDataPoint::point("Feb", [("revenue", 150.0), ("costs", 90.0), ("profit", 60.0)]),
        ChartDataPoint::point("Mar", [("revenue", 180.0), ("costs", 95.0), ("profit", 85.0)]),
        ChartDataPoint::point("Apr", [("revenue", 220.0), ("costs", 110.0), ("profit", 110.0)]),
        ChartDataPoint::point("Mag", [("revenue", 210.0), ("costs", 105.0), ("profit", 105.0)]),
        ChartDataPoint::point("Giu", [("revenue", 260.0), ("costs", 120.0), ("profit", 140.0)]),
    ];

    let sample_series = vec![
        ChartSeries::new("revenue").with_label("Ricavi (€k)").with_color("#a855f7"),  // sd-lint-allow: runtime data value, not styling
        ChartSeries::new("costs").with_label("Costi (€k)").with_color("#ff1744").with_dashed(true),  // sd-lint-allow: runtime data value, not styling
        ChartSeries::new("profit").with_label("Margine (€k)").with_color("#00ff9d"),  // sd-lint-allow: runtime data value, not styling
    ];

    // Dati Pie & Donut
    let sample_pie = vec![
        PieSegment::new("Direct", 45.0).with_color("#a855f7"),  // sd-lint-allow: runtime data value, not styling
        PieSegment::new("Referral", 25.0).with_color("#00f0ff"),  // sd-lint-allow: runtime data value, not styling
        PieSegment::new("Social", 20.0).with_color("#00ff9d"),  // sd-lint-allow: runtime data value, not styling
        PieSegment::new("Organic", 10.0).with_color("#f59e0b"),  // sd-lint-allow: runtime data value, not styling
    ];

    // Dati Radar
    let sample_radar = vec![
        RadarDataPoint::new("Velocità").with_value("Prodotto A", 85.0).with_value("Prodotto B", 65.0),
        RadarDataPoint::new("Affidabilità").with_value("Prodotto A", 90.0).with_value("Prodotto B", 80.0),
        RadarDataPoint::new("Comfort").with_value("Prodotto A", 70.0).with_value("Prodotto B", 95.0),
        RadarDataPoint::new("Sicurezza").with_value("Prodotto A", 95.0).with_value("Prodotto B", 85.0),
        RadarDataPoint::new("Design").with_value("Prodotto A", 80.0).with_value("Prodotto B", 75.0),
    ];
    let radar_series = vec![
        ChartSeries::new("Prodotto A").with_color("#a855f7"),  // sd-lint-allow: runtime data value, not styling
        ChartSeries::new("Prodotto B").with_color("#00f0ff"),  // sd-lint-allow: runtime data value, not styling
    ];

    // Dati Scatter & Bubble
    let sample_scatter = vec![
        ScatterPoint::new(10.0, 20.0).with_z(5.0).with_label("Alpha"),
        ScatterPoint::new(25.0, 45.0).with_z(12.0).with_label("Beta"),
        ScatterPoint::new(40.0, 30.0).with_z(8.0).with_label("Gamma"),
        ScatterPoint::new(55.0, 75.0).with_z(18.0).with_label("Delta"),
        ScatterPoint::new(70.0, 60.0).with_z(14.0).with_label("Epsilon"),
        ScatterPoint::new(85.0, 90.0).with_z(22.0).with_label("Zeta"),
    ];

    // Dati Sparkline (Matematicamente coerenti con i delta percentuali e assoluti mostrati)
    // Partenza da 32.7 con incremento del +28.4% (+9.3pt) = 42.0!
    let spark_up = vec![32.7, 33.5, 35.0, 34.2, 37.8, 38.5, 40.1, 42.0];
    // Partenza da 60.0ms con delta di -42ms = 18.0ms esatti!
    let spark_down = vec![60.0, 56.0, 52.0, 48.0, 38.0, 32.0, 24.0, 18.0];

    // Dati Heatmap Mensile
    let sample_heatmap = {
        let mut cells = Vec::new();
        for x in 0..12 {
            for y in 0..5 {
                let v = ((x * 7 + y * 13) % 40) as f64;
                cells.push(HeatmapCell::new(x, y, v).with_label(format!("Mese {}, Canale {}", x + 1, y + 1)));
            }
        }
        cells
    };

    // Dati Heatmap Giornaliera (Calendar Heatmap 52 Settimane x 7 Giorni: Lun-Dom)
    let sample_daily_heatmap = {
        let mut cells = Vec::new();
        for w in 0..52 {
            for d in 0..7 {
                let count = if (w * 7 + d * 13) % 4 == 0 {
                    ((w * 3 + d * 5) % 15) as f64
                } else if (w * 11 + d) % 3 == 0 {
                    ((w + d * 2) % 8) as f64
                } else {
                    0.0
                };
                let day_name = match d {
                    0 => "Lun", 1 => "Mar", 2 => "Mer", 3 => "Gio", 4 => "Ven", 5 => "Sab", _ => "Dom",
                };
                cells.push(HeatmapCell::new(w, d, count).with_label(format!("Settimana {}, {}: {} commit", w + 1, day_name, count as usize)));
            }
        }
        cells
    };

    let props = vec![
        PropRow {
            name: "data",
            r#type: "Vec<ChartDataPoint> | Vec<PieSegment> | ...",
            default: "-",
            description: "Collezione fortemente tipizzata di coordinate o metriche",
        },
        PropRow {
            name: "series",
            r#type: "Vec<ChartSeries>",
            default: "-",
            description: "Definizione serie con etichette, colori ed eventuale tratteggio dashed",
        },
        PropRow {
            name: "with_tooltip",
            r#type: "bool",
            default: "true",
            description: "Abilita popover di hover con valori formattati in tempo reale",
        },
        PropRow {
            name: "with_legend",
            r#type: "bool",
            default: "true",
            description: "Abilita legenda interattiva con toggle visibilità serie",
        },
        PropRow {
            name: "height",
            r#type: "u32",
            default: "300",
            description: "Altezza minima del grafico in pixel all'interno dell'SVG responsive",
        },
    ];

    let code_snippet = r#"AreaChart {
    data: sample_points,
    series: sample_series,
    curve_type: CurveType::Monotone,
    with_gradient: true,
    with_dots: true,
    height: 300,
}"#
    .to_string();

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Charts & Data Visualization".to_string(),
                description: "Suite completa di grafici vettoriali SVG nativi in Rust per Dioxus 0.7: zero dipendenze JavaScript esterne, rendering reattivo e glow coordinato con il tema.".to_string(),
            }

            FeatureBanner { feature_name: "charts" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "sc-stack",
                        Controls {
                            OptionPicker {
                                label: "Tipologia Grafico".to_string(),
                                options: [
                                    ("area", "Area"),
                                    ("bar", "Bar"),
                                    ("line", "Line"),
                                    ("pie", "Pie & Donut"),
                                    ("radar", "Radar"),
                                    ("scatter", "Scatter / Bubble"),
                                    ("sparkline", "Sparkline"),
                                    ("gauge", "Gauge & Radial"),
                                    ("heatmap", "Heatmap"),
                                ]
                                    .iter()
                                    .map(|(v, l)| (v.to_string(), l.to_string()))
                                    .collect::<Vec<_>>(),
                                value: selected_chart_tab,
                            }
                            if selected_chart_tab() == "heatmap" {
                                OptionPicker {
                                    label: "Vista Heatmap".to_string(),
                                    options: vec![
                                        ("month".to_string(), "Mensile (12 Mesi x 5 Canali)".to_string()),
                                        ("day".to_string(), "Giornaliera (52 Settimane x 7 Giorni)".to_string()),
                                    ],
                                    value: heatmap_mode,
                                }
                            }
                        }
                    Stage { wide: true,
                        if selected_chart_tab() == "area" {
                            AreaChart {
                                data: sample_points.clone(),
                                series: sample_series.clone(),
                                curve_type: CurveType::Monotone,
                                with_gradient: true,
                                with_dots: true,
                                height: 320,
                            }
                        } else if selected_chart_tab() == "bar" {
                            BarChart {
                                data: sample_points.clone(),
                                series: sample_series.clone(),
                                chart_type: BarChartType::Default,
                                height: 320,
                            }
                        } else if selected_chart_tab() == "line" {
                            LineChart {
                                data: sample_points.clone(),
                                series: sample_series.clone(),
                                curve_type: CurveType::Monotone,
                                with_dots: true,
                                height: 320,
                            }
                        } else if selected_chart_tab() == "pie" {
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-8 w-full items-center",
                                div { class: "flex flex-col items-center",
                                    span { class: "sc-control-label", "PieChart" }
                                    PieChart {
                                        data: sample_pie.clone(),
                                        height: 260,
                                    }
                                }
                                div { class: "flex flex-col items-center",
                                    span { class: "sc-control-label", "DonutChart (KPI)" }
                                    DonutChart {
                                        data: sample_pie.clone(),
                                        center_label: Some("100%".to_string()),
                                        center_sublabel: Some("Totale Conversioni".to_string()),
                                        height: 260,
                                    }
                                }
                            }
                        } else if selected_chart_tab() == "radar" {
                            RadarChart {
                                data: sample_radar.clone(),
                                series: radar_series.clone(),
                                height: 320,
                            }
                        } else if selected_chart_tab() == "scatter" {
                            ScatterChart {
                                data: sample_scatter.clone(),
                                height: 320,
                            }
                        } else if selected_chart_tab() == "sparkline" {
                            div { class: "flex flex-wrap items-center justify-center gap-8 py-8",
                                Card {
                                    div { class: "sc-title-row",
                                    div { class: "sc-stack",
                                        span { class: "sc-control-label", "CPU Load" }
                                        span { class: "sc-section-title", "42.0%" }
                                        span { class: "sc-success-text sc-mono", "+28.4% (+9.3pt)" }
                                    }
                                        Sparkline { data: spark_up.clone(), trend: TrendDirection::Positive }
                                    }
                                }
                                Card {
                                    div { class: "sc-title-row",
                                    div { class: "sc-stack",
                                        span { class: "sc-control-label", "Latency" }
                                        span { class: "sc-section-title", "18 ms" }
                                        span { class: "sc-success-text sc-mono", "-42 ms (da 60 ms)" }
                                    }
                                        Sparkline { data: spark_down.clone(), trend: TrendDirection::Negative }
                                    }
                                }
                            }
                        } else if selected_chart_tab() == "gauge" {
                            div { class: "grid grid-cols-1 sm:grid-cols-2 gap-8 w-full items-center",
                                div { class: "flex flex-col items-center",
                                    span { class: "sc-control-label", "GaugeChart" }
                                    GaugeChart {
                                        value: 78.5,
                                        label: Some("Storage Usato".to_string()),
                                        sublabel: Some("785 GB / 1000 GB".to_string()),
                                        height: 220,
                                    }
                                }
                                div { class: "flex flex-col items-center",
                                    span { class: "sc-control-label", "RadialBar (360°)" }
                                    RadialBar {
                                        value: 84.0,
                                        label: Some("Battery".to_string()),
                                        height: 180,
                                    }
                                }
                            }
                        } else if selected_chart_tab() == "heatmap" {
                            div { class: "flex flex-col items-center w-full gap-4",
                                                                if heatmap_mode() == "month" {
                                    Heatmap {
                                        data: sample_heatmap.clone(),
                                        x_labels: vec!["G".to_string(), "F".to_string(), "M".to_string(), "A".to_string(), "M".to_string(), "G".to_string(), "L".to_string(), "A".to_string(), "S".to_string(), "O".to_string(), "N".to_string(), "D".to_string()],
                                        y_labels: vec!["S1".to_string(), "S2".to_string(), "S3".to_string(), "S4".to_string(), "S5".to_string()],
                                        height: 220,
                                    }
                                } else {
                                    div { class: "w-full overflow-x-auto py-2",
                                        CalendarHeatmap {
                                            data: sample_daily_heatmap.clone(),
                                            height: 150,
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
