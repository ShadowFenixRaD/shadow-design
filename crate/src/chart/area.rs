use dioxus::prelude::*;
use std::collections::BTreeSet;

use super::primitives::{
    compute_min_max, generate_stacked_area_band_path, generate_svg_area_path, generate_svg_path,
    get_series_color, ChartContainer, ChartDataPoint, ChartGrid, ChartLegend, ChartSeries,
    ChartTooltip, CurveType,
};

/// Grafico ad area vettoriale SVG con supporto per serie sovrapposte/impilate (Stacked Area), gradienti sfumati e tooltip interattivo.
#[allow(clippy::type_complexity)]
#[component]
pub fn AreaChart(
    data: Vec<ChartDataPoint>,
    series: Vec<ChartSeries>,
    #[props(default)] curve_type: CurveType,
    #[props(default = true)] stacked: bool,
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
    let baseline_y = padding_top + plot_h;

    let active_series: Vec<(usize, &ChartSeries)> = series
        .iter()
        .enumerate()
        .filter(|(_, s)| !hidden_series().contains(&s.name))
        .collect();

    let n = data.len();
    let x_step = if n > 1 {
        plot_w / (n - 1) as f64
    } else {
        plot_w
    };

    // Calcolo scale e coordinate (Stacked o Normal)
    let (series_renders, max_val_used) = if stacked {
        // Modalità Stacked: calcolo della somma cumulativa ad ogni punto
        let mut max_sum = 1.0f64;
        for pt in &data {
            let sum: f64 = active_series.iter().filter_map(|(_, s)| pt.get(&s.name)).sum();
            if sum > max_sum {
                max_sum = sum;
            }
        }

        let mut renders = Vec::new();
        // Tracciamento dal basso verso l'alto
        let mut prev_top_coords = vec![(0.0f64, baseline_y); n];
        for (i, _) in data.iter().enumerate() {
            let cx = padding_left + i as f64 * x_step;
            prev_top_coords[i] = (cx, baseline_y);
        }

        let mut cumulative_values = vec![0.0f64; n];

        for &(orig_idx, s) in &active_series {
            let color = get_series_color(orig_idx, s.color.as_deref());
            let mut top_coords = Vec::with_capacity(n);
            let mut bottom_coords = Vec::with_capacity(n);

            for (i, pt) in data.iter().enumerate() {
                let val = pt.get(&s.name).unwrap_or(0.0).max(0.0);
                let cx = padding_left + i as f64 * x_step;
                let bot_y = prev_top_coords[i].1;

                cumulative_values[i] += val;
                let top_y = padding_top + plot_h - (cumulative_values[i] / max_sum) * plot_h;

                top_coords.push((cx, top_y));
                bottom_coords.push((cx, bot_y));
            }

            prev_top_coords = top_coords.clone();
            renders.push((orig_idx, s.name.clone(), color, top_coords, bottom_coords));
        }

        (renders, max_sum)
    } else {
        // Modalità Non-stacked classica
        let (min_val, max_val) = compute_min_max(&data, &series);
        let range = if (max_val - min_val).abs() < f64::EPSILON {
            1.0
        } else {
            max_val - min_val
        };

        let renders = active_series
            .iter()
            .map(|&(orig_idx, s)| {
                let color = get_series_color(orig_idx, s.color.as_deref());
                let top_coords: Vec<(f64, f64)> = data
                    .iter()
                    .enumerate()
                    .map(|(i, pt)| {
                        let v = pt.get(&s.name).unwrap_or(0.0);
                        let cx = padding_left + i as f64 * x_step;
                        let cy = padding_top + plot_h - ((v - min_val) / range) * plot_h;
                        (cx, cy)
                    })
                    .collect();
                let bottom_coords: Vec<(f64, f64)> = (0..n)
                    .map(|i| (padding_left + i as f64 * x_step, baseline_y))
                    .collect();
                (orig_idx, s.name.clone(), color, top_coords, bottom_coords)
            })
            .collect();

        (renders, max_val)
    };

    // Dati per il tooltip attivo
    let current_tooltip_data = hover_index().and_then(|idx| {
        if idx >= data.len() {
            return None;
        }
        let pt = &data[idx];
        let mut entries = Vec::new();
        for &(orig_idx, ref name, ref color, _, _) in &series_renders {
            if let Some(v) = pt.get(name) {
                let s_label = series[orig_idx].display_label().to_string();
                entries.push((s_label, format!("{v:.1}"), color.clone()));
            }
        }
        let cx = padding_left + idx as f64 * x_step;
        let cy = series_renders.last().map(|s| s.3[idx].1).unwrap_or(padding_top + plot_h / 2.0);
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
                    defs {
                        for (orig_idx, _, color, _, _) in &series_renders {
                            linearGradient {
                                id: "area-grad-{orig_idx}",
                                x1: "0",
                                y1: "0",
                                x2: "0",
                                y2: "1",
                                stop {
                                    offset: "0%",
                                    stop_color: "{color}",
                                    stop_opacity: "0.55",
                                }
                                stop {
                                    offset: "100%",
                                    stop_color: "{color}",
                                    stop_opacity: "0.05",
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

                    // Aree e linee (ordinate per render corretto)
                    for (orig_idx, _, color, top_coords, bottom_coords) in &series_renders {
                        {
                            let area_d = if stacked {
                                generate_stacked_area_band_path(top_coords, bottom_coords, curve_type)
                            } else {
                                generate_svg_area_path(top_coords, baseline_y, curve_type)
                            };
                            let line_d = generate_svg_path(top_coords, curve_type);
                            rsx! {
                                if with_gradient {
                                    path {
                                        d: "{area_d}",
                                        fill: "url(#area-grad-{orig_idx})",
                                        pointer_events: "none",
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
                                    pointer_events: "none",
                                }
                                if with_dots {
                                    for (pt_idx, (cx, cy)) in top_coords.iter().enumerate() {
                                        {
                                            let is_h = hover_index() == Some(pt_idx);
                                            let dot_r = if is_h { "5.5" } else { "3.5" };
                                            rsx! {
                                                circle {
                                                    key: "dot-{orig_idx}-{pt_idx}",
                                                    cx: "{cx}",
                                                    cy: "{cy}",
                                                    r: "{dot_r}",
                                                    fill: "#09090b",
                                                    stroke: if is_h { "#ffffff" } else { "{color}" },
                                                    stroke_width: if is_h { "2.5" } else { "2" },
                                                    class: "transition-all duration-150",
                                                    pointer_events: "none",
                                                }
                                            }
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
                                    pointer_events: "none",
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
                                    pointer_events: "none",
                                    "{lbl}"
                                }
                            }
                        }
                    }

                    // Etichette asse Y (massimo)
                    text {
                        x: "{padding_left - 8.0}",
                        y: "{padding_top + 6.0}",
                        text_anchor: "end",
                        fill: "rgba(255, 255, 255, 0.40)",
                        font_size: "10",
                        font_family: "monospace",
                        pointer_events: "none",
                        "{max_val_used:.0}"
                    }

                    // Zone interattive hover per ciascun punto dati (copertura totale senza gap)
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

                // Tooltip Dioxus posizionato in percentuale
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
