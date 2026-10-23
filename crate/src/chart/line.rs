use dioxus::prelude::*;
use std::collections::BTreeSet;

use super::primitives::{
    compute_min_max, generate_svg_path, get_series_color, ChartContainer, ChartDataPoint,
    ChartGrid, ChartLegend, ChartSeries, ChartTooltip, CurveType,
};

/// Grafico a linee vettoriale SVG multi-serie con snodi interattivi e curve bezier continue.
#[allow(clippy::type_complexity)]
#[component]
pub fn LineChart(
    data: Vec<ChartDataPoint>,
    series: Vec<ChartSeries>,
    #[props(default)] curve_type: CurveType,
    #[props(default = true)] with_dots: bool,
    #[props(default = true)] with_grid: bool,
    #[props(default = 2.5)] stroke_width: f64,
    #[props(default = true)] connect_nulls: bool,
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

    let baseline_y = padding_top + plot_h;

    // Segmenti di coordinate per ogni serie
    let series_lines: Vec<(usize, String, String, bool, Vec<Vec<(f64, f64)>>)> = active_series
        .iter()
        .map(|&(orig_idx, s)| {
            let color = get_series_color(orig_idx, s.color.as_deref());
            let is_dashed = s.dashed;
            let mut segments: Vec<Vec<(f64, f64)>> = Vec::new();
            let mut current_segment: Vec<(f64, f64)> = Vec::new();

            for (i, pt) in data.iter().enumerate() {
                let cx = padding_left + i as f64 * x_step;
                if let Some(v) = pt.get(&s.name) {
                    let cy = padding_top + plot_h - ((v - min_val) / range) * plot_h;
                    current_segment.push((cx, cy));
                } else if !connect_nulls && !current_segment.is_empty() {
                    segments.push(std::mem::take(&mut current_segment));
                }
            }
            if !current_segment.is_empty() {
                segments.push(current_segment);
            }

            (orig_idx, s.name.clone(), color, is_dashed, segments)
        })
        .collect();

    let current_tooltip_data = hover_index().and_then(|idx| {
        if idx >= data.len() {
            return None;
        }
        let pt = &data[idx];
        let mut entries = Vec::new();
        for &(orig_idx, ref name, ref color, _, _) in &series_lines {
            if let Some(v) = pt.get(name) {
                let s_label = series[orig_idx].display_label().to_string();
                entries.push((s_label, format!("{v:.1}"), color.clone()));
            }
        }
        let cx = padding_left + idx as f64 * x_step;
        let cy = padding_top + plot_h / 2.0;
        let x_pct = (cx / svg_w) * 100.0;
        let y_pct = (cy / svg_h) * 100.0;
        let is_right = x_pct > 60.0;
        Some((pt.label.clone(), entries, x_pct, y_pct, is_right))
    });

    rsx! {
        div { class: "w-full flex flex-col {class}",
            div {
                class: "relative w-full",
                onmouseleave: move |_| {
                    hover_index.set(None);
                },
                onpointerleave: move |_| {
                    hover_index.set(None);
                },
                ChartContainer {
                    view_box: "0 0 {svg_w} {svg_h}",
                    height: height,
                    preserve_aspect_ratio: "none",
                    onmouseleave: move |_| {
                        hover_index.set(None);
                    },
                    rect {
                        x: "0",
                        y: "0",
                        width: "{svg_w}",
                        height: "{svg_h}",
                        fill: "rgba(0, 0, 0, 0.001)",
                        style: "pointer-events: all;",
                        onmouseenter: move |_| hover_index.set(None),
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

                    // Render linee
                    for (orig_idx, _, color, dashed, segments) in &series_lines {
                        for (seg_idx, seg) in segments.iter().enumerate() {
                            {
                                let path_d = generate_svg_path(seg, curve_type);
                                rsx! {
                                    path {
                                        key: "line-{orig_idx}-{seg_idx}",
                                        d: "{path_d}",
                                        fill: "none",
                                        stroke: "{color}",
                                        stroke_width: "{stroke_width}",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke_dasharray: if *dashed { "6 4" } else { "none" },
                                        style: "filter: drop-shadow(0 0 6px {color}88);",
                                        pointer_events: "none",
                                    }
                                }
                            }
                        }
                    }

                    // Render dots
                    if with_dots {
                        for (orig_idx, _, color, _, segments) in &series_lines {
                            for seg in segments {
                                for (pt_idx, (cx, cy)) in seg.iter().enumerate() {
                                    {
                                        let is_hovered = hover_index() == Some(pt_idx);
                                        rsx! {
                                            circle {
                                                key: "dot-{orig_idx}-{pt_idx}",
                                                cx: "{cx}",
                                                cy: "{cy}",
                                                r: if is_hovered { "5.5" } else { "3.5" },
                                                fill: "#09090b",
                                                stroke: if is_hovered { "#ffffff" } else { "{color}" },
                                                stroke_width: if is_hovered { "2.5" } else { "2" },
                                                class: "transition-all duration-150",
                                                pointer_events: "none",
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Linea verticale cursore hover
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
                                    pointer_events: "none",
                                }
                            }
                        }
                    }

                    // Etichette asse X
                    for (i, pt) in data.iter().enumerate() {
                        {
                            let cx = padding_left + i as f64 * x_step;
                            rsx! {
                                text {
                                    key: "lbl-{i}",
                                    x: "{cx}",
                                    y: "{baseline_y + 20.0}",
                                    text_anchor: "middle",
                                    fill: "rgba(255, 255, 255, 0.45)",
                                    font_size: "11",
                                    font_family: "monospace",
                                    pointer_events: "none",
                                    "{pt.label}"
                                }
                            }
                        }
                    }

                    // Hover zone contigue in coordinate SVG native (copertura totale senza gap)
                    for i in 0..n {
                        {
                            let (zone_x, zone_w) = if n <= 1 {
                                (0.0, svg_w)
                            } else if i == 0 {
                                (0.0, padding_left + x_step / 2.0)
                            } else if i == n - 1 {
                                let zx = padding_left + (i as f64 - 0.5) * x_step;
                                (zx, (svg_w - zx).max(1.0))
                            } else {
                                (padding_left + (i as f64 - 0.5) * x_step, x_step)
                            };
                            rsx! {
                                rect {
                                    key: "hover-zone-{i}",
                                    x: "{zone_x}",
                                    y: "0",
                                    width: "{zone_w}",
                                    height: "{svg_h}",
                                    fill: "rgba(0, 0, 0, 0.001)",
                                    style: "pointer-events: all;",
                                    class: "cursor-pointer",
                                    onmouseenter: move |_| hover_index.set(Some(i)),
                                    onmouseleave: move |_| hover_index.set(None),
                                    onpointerleave: move |_| hover_index.set(None),
                                }
                            }
                        }
                    }
                }

                if with_tooltip {
                    if let Some((title, entries, x_pct, y_pct, align_r)) = current_tooltip_data {
                        ChartTooltip {
                            title: title,
                            entries: entries,
                            x_percent: x_pct,
                            y_percent: y_pct,
                            align_right: align_r,
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
