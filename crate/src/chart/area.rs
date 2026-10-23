use dioxus::prelude::*;
use std::collections::BTreeSet;

use super::primitives::{
    compute_min_max, generate_svg_area_path, generate_svg_path, get_series_color, ChartContainer,
    ChartDataPoint, ChartGrid, ChartLegend, ChartSeries, ChartTooltip, CurveType,
};

/// Grafico ad area vettoriale SVG con supporto per gradienti sfumati, multi-serie e tooltip live.
#[allow(clippy::type_complexity)]
#[component]
pub fn AreaChart(
    data: Vec<ChartDataPoint>,
    series: Vec<ChartSeries>,
    #[props(default)] curve_type: CurveType,
    #[props(default = true)] with_gradient: bool,
    #[props(default = true)] with_dots: bool,
    #[props(default = true)] with_grid: bool,
    #[props(default = true)] with_tooltip: bool,
    #[props(default = true)] with_legend: bool,
    #[props(default = 300)] height: u32,
    #[props(default)] class: String,
) -> Element {
    let hidden_series = use_signal(BTreeSet::<String>::new);
    let mut hover_index = use_signal(|| None::<usize>);

    let svg_w = 600.0;
    let svg_h = 260.0;
    let padding_left = 50.0;
    let padding_right = 20.0;
    let padding_top = 20.0;
    let padding_bottom = 40.0;

    let plot_w = svg_w - padding_left - padding_right;
    let plot_h = svg_h - padding_top - padding_bottom;

    let active_series: Vec<(usize, &ChartSeries)> = series
        .iter()
        .enumerate()
        .filter(|(_, s)| !hidden_series().contains(&s.name))
        .collect();

    let (min_val, max_val) = compute_min_max(&data, &series);
    let range = if (max_val - min_val).abs() < f64::EPSILON {
        1.0
    } else {
        max_val - min_val
    };

    let n = data.len();
    let x_step = if n > 1 {
        plot_w / (n - 1) as f64
    } else {
        plot_w
    };

    // Calcolo coordinate (x, y) per ciascuna serie attiva
    let series_coords: Vec<(usize, String, String, Vec<(f64, f64)>)> = active_series
        .iter()
        .map(|&(orig_idx, s)| {
            let color = get_series_color(orig_idx, s.color.as_deref());
            let coords: Vec<(f64, f64)> = data
                .iter()
                .enumerate()
                .filter_map(|(i, pt)| {
                    pt.get(&s.name).map(|v| {
                        let cx = padding_left + i as f64 * x_step;
                        let cy = padding_top + plot_h - ((v - min_val) / range) * plot_h;
                        (cx, cy)
                    })
                })
                .collect();
            (orig_idx, s.name.clone(), color, coords)
        })
        .collect();

    let baseline_y = padding_top + plot_h;

    // Dati per il tooltip attivo
    let current_tooltip_data = hover_index().and_then(|idx| {
        if idx >= data.len() {
            return None;
        }
        let pt = &data[idx];
        let mut entries = Vec::new();
        for &(orig_idx, ref name, ref color, _) in &series_coords {
            if let Some(v) = pt.get(name) {
                let s_label = series[orig_idx].display_label().to_string();
                entries.push((s_label, format!("{v:.1}"), color.clone()));
            }
        }
        let cx = padding_left + idx as f64 * x_step;
        Some((pt.label.clone(), entries, cx, padding_top + 20.0))
    });

    rsx! {
        div { class: "w-full flex flex-col {class}",
            div { class: "relative w-full",
                ChartContainer {
                    view_box: "0 0 {svg_w} {svg_h}",
                    height: height,
                    defs {
                        for (orig_idx, _, color, _) in &series_coords {
                            linearGradient {
                                id: "area-grad-{orig_idx}",
                                x1: "0",
                                y1: "0",
                                x2: "0",
                                y2: "1",
                                stop {
                                    offset: "0%",
                                    stop_color: "{color}",
                                    stop_opacity: "0.45",
                                }
                                stop {
                                    offset: "100%",
                                    stop_color: "{color}",
                                    stop_opacity: "0.02",
                                }
                            }
                        }
                    }

                    if with_grid {
                        ChartGrid {
                            x: padding_left,
                            y: padding_top,
                            width: plot_w,
                            height: plot_h,
                            y_ticks: 4,
                            horizontal: true,
                        }
                    }

                    // Aree e linee
                    for (orig_idx, _, color, coords) in &series_coords {
                        {
                            let area_d = generate_svg_area_path(coords, baseline_y, curve_type);
                            let line_d = generate_svg_path(coords, curve_type);
                            rsx! {
                                if with_gradient {
                                    path {
                                        d: "{area_d}",
                                        fill: "url(#area-grad-{orig_idx})",
                                    }
                                }
                                path {
                                    d: "{line_d}",
                                    fill: "none",
                                    stroke: "{color}",
                                    stroke_width: "2.5",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    style: "filter: drop-shadow(0 0 6px {color}88);",
                                }
                                if with_dots {
                                    for (pt_idx, (cx, cy)) in coords.iter().enumerate() {
                                        circle {
                                            key: "dot-{orig_idx}-{pt_idx}",
                                            cx: "{cx}",
                                            cy: "{cy}",
                                            r: if hover_index() == Some(pt_idx) { "5" } else { "3.5" },
                                            fill: "#09090b",
                                            stroke: "{color}",
                                            stroke_width: "2",
                                            class: "transition-all duration-150",
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Linea cursore hover verticale
                    if let Some(idx) = hover_index() {
                        {
                            let cx = padding_left + idx as f64 * x_step;
                            rsx! {
                                line {
                                    x1: "{cx}",
                                    y1: "{padding_top}",
                                    x2: "{cx}",
                                    y2: "{baseline_y}",
                                    stroke: "rgba(255, 255, 255, 0.25)",
                                    stroke_width: "1.5",
                                    stroke_dasharray: "3 3",
                                }
                            }
                        }
                    }

                    // Etichette asse X
                    for (i, pt) in data.iter().enumerate() {
                        {
                            let cx = padding_left + i as f64 * x_step;
                            let lbl = pt.label.clone();
                            rsx! {
                                text {
                                    key: "lbl-{i}",
                                    x: "{cx}",
                                    y: "{baseline_y + 20.0}",
                                    text_anchor: "middle",
                                    fill: "rgba(255, 255, 255, 0.45)",
                                    font_size: "11",
                                    font_family: "monospace",
                                    "{lbl}"
                                }
                            }
                        }
                    }

                    // Aree invisibili interattive per hover asse X
                    for i in 0..n {
                        {
                            let col_w = if n > 1 { plot_w / n as f64 } else { plot_w };
                            let col_x = padding_left + i as f64 * x_step - (col_w / 2.0);
                            rsx! {
                                rect {
                                    key: "hover-zone-{i}",
                                    x: "{col_x.max(padding_left)}",
                                    y: "{padding_top}",
                                    width: "{col_w}",
                                    height: "{plot_h}",
                                    fill: "transparent",
                                    class: "cursor-pointer",
                                    onmouseenter: move |_| hover_index.set(Some(i)),
                                    onmouseleave: move |_| hover_index.set(None),
                                }
                            }
                        }
                    }
                }

                // Tooltip Dioxus posizionato
                if with_tooltip {
                    if let Some((title, entries, x_pos, y_pos)) = current_tooltip_data {
                        ChartTooltip {
                            title: title,
                            entries: entries,
                            x: x_pos,
                            y: y_pos,
                            visible: true,
                        }
                    }
                }
            }

            if with_legend {
                ChartLegend {
                    series: series.clone(),
                    hidden_series: hidden_series,
                }
            }
        }
    }
}
