use dioxus::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::PI;

use super::primitives::{get_series_color, ChartContainer, ChartLegend, ChartSeries, ChartTooltip};

/// Singolo punto o asse di valutazione per grafico radar polare.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RadarDataPoint {
    pub axis: String,
    pub values: BTreeMap<String, f64>,
}

impl RadarDataPoint {
    #[must_use]
    pub fn new(axis: impl Into<String>) -> Self {
        Self {
            axis: axis.into(),
            values: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn with_value(mut self, key: impl Into<String>, val: f64) -> Self {
        self.values.insert(key.into(), val);
        self
    }
}

/// Grafico radar polare per benchmarking multi-dimensionale ad alta precisione.
#[allow(clippy::type_complexity)]
#[component]
pub fn RadarChart(
    data: Vec<RadarDataPoint>,
    series: Vec<ChartSeries>,
    #[props(default = 4)] levels: usize,
    #[props(default = true)] with_dots: bool,
    #[props(default = true)] with_legend: bool,
    #[props(default = true)] with_tooltip: bool,
    #[props(default = 320)] height: u32,
    #[props(default)] class: String,
) -> Element {
    let hidden_series = use_signal(BTreeSet::<String>::new);
    let mut hover_axis = use_signal(|| None::<usize>);

    let svg_w = 400.0;
    let svg_h = 320.0;
    let center_x = svg_w / 2.0;
    let center_y = svg_h / 2.0 - 10.0;
    let max_radius = 110.0;

    let num_axes = data.len();
    if num_axes < 3 {
        return rsx! {
            div { class: "p-4 text-center text-xs text-zinc-500 font-mono",
                "RadarChart richiede almeno 3 assi dati."
            }
        };
    }

    let active_series: Vec<(usize, &ChartSeries)> = series
        .iter()
        .enumerate()
        .filter(|(_, s)| !hidden_series().contains(&s.name))
        .collect();

    // Calcolo max_val globale
    let mut max_val = 1.0f64;
    for pt in &data {
        for (_, s) in &active_series {
            if let Some(&v) = pt.values.get(&s.name) {
                if v > max_val {
                    max_val = v;
                }
            }
        }
    }

    // Angolo per ciascun asse
    let angle_step = (2.0 * PI) / num_axes as f64;

    // Poligoni concentrici di griglia
    let mut grid_polygons = Vec::new();
    for lvl in 1..=levels {
        let r = (lvl as f64 / levels as f64) * max_radius;
        let mut pts = Vec::new();
        for i in 0..num_axes {
            let angle = -PI / 2.0 + i as f64 * angle_step;
            let px = center_x + r * angle.cos();
            let py = center_y + r * angle.sin();
            pts.push(format!("{px:.2},{py:.2}"));
        }
        grid_polygons.push(pts.join(" "));
    }

    // Raggi per gli assi
    let mut axis_lines = Vec::new();
    let mut axis_labels = Vec::new();
    for (i, pt) in data.iter().enumerate() {
        let angle = -PI / 2.0 + i as f64 * angle_step;
        let px = center_x + max_radius * angle.cos();
        let py = center_y + max_radius * angle.sin();
        axis_lines.push((px, py));

        let lbl_x = center_x + (max_radius + 20.0) * angle.cos();
        let lbl_y = center_y + (max_radius + 15.0) * angle.sin();
        axis_labels.push((pt.axis.clone(), lbl_x, lbl_y));
    }

    // Poligoni per le serie attive
    let series_polygons: Vec<(usize, String, String, String, Vec<(f64, f64)>)> = active_series
        .iter()
        .map(|&(orig_idx, s)| {
            let color = get_series_color(orig_idx, s.color.as_deref());
            let mut pts_str = Vec::new();
            let mut coords = Vec::new();

            for (i, pt) in data.iter().enumerate() {
                let val = pt.values.get(&s.name).copied().unwrap_or(0.0);
                let r = (val / max_val) * max_radius;
                let angle = -PI / 2.0 + i as f64 * angle_step;
                let px = center_x + r * angle.cos();
                let py = center_y + r * angle.sin();
                pts_str.push(format!("{px:.2},{py:.2}"));
                coords.push((px, py));
            }

            (orig_idx, s.name.clone(), color, pts_str.join(" "), coords)
        })
        .collect();

    let current_tooltip_data = hover_axis().and_then(|idx| {
        if idx >= data.len() {
            return None;
        }
        let pt = &data[idx];
        let mut entries = Vec::new();
        for &(orig_idx, ref name, ref color, _, _) in &series_polygons {
            if let Some(&v) = pt.values.get(name) {
                let s_label = series[orig_idx].display_label().to_string();
                entries.push((s_label, format!("{v:.1}"), color.clone()));
            }
        }
        let angle = -PI / 2.0 + idx as f64 * angle_step;
        let cx = center_x + (max_radius * 0.75) * angle.cos();
        let cy = center_y + (max_radius * 0.75) * angle.sin();
        let x_pct = (cx / svg_w) * 100.0;
        let y_pct = (cy / svg_h) * 100.0;
        let is_right = x_pct > 50.0;
        Some((pt.axis.clone(), entries, x_pct, y_pct, is_right))
    });

    rsx! {
        div { class: "w-full flex flex-col items-center {class}",
            div {
                class: "relative w-full max-w-md",
                onmouseleave: move |_| {
                    hover_axis.set(None);
                },
                onpointerleave: move |_| {
                    hover_axis.set(None);
                },
                ChartContainer {
                    view_box: "0 0 {svg_w} {svg_h}",
                    height: height,
                    onmouseleave: move |_| {
                        hover_axis.set(None);
                    },
                    rect {
                        x: "0",
                        y: "0",
                        width: "{svg_w}",
                        height: "{svg_h}",
                        fill: "rgba(0, 0, 0, 0.001)",
                        style: "pointer-events: all;",
                        onmouseenter: move |_| hover_axis.set(None),
                    }

                    // Griglia concentrica
                    for (lvl_idx, poly_pts) in grid_polygons.iter().enumerate() {
                        polygon {
                            key: "grid-poly-{lvl_idx}",
                            points: "{poly_pts}",
                            fill: "none",
                            stroke: "rgba(255, 255, 255, 0.08)",
                            stroke_width: "1",
                            pointer_events: "none",
                        }
                    }

                    // Raggi assiali
                    for (i, (px, py)) in axis_lines.iter().enumerate() {
                        line {
                            key: "axis-line-{i}",
                            x1: "{center_x}",
                            y1: "{center_y}",
                            x2: "{px}",
                            y2: "{py}",
                            stroke: "rgba(255, 255, 255, 0.10)",
                            stroke_width: "1",
                            pointer_events: "none",
                        }
                    }

                    // Poligoni serie
                    for (orig_idx, _, color, poly_pts, coords) in &series_polygons {
                        g { key: "series-radar-{orig_idx}",
                            polygon {
                                points: "{poly_pts}",
                                fill: "{color}",
                                fill_opacity: "0.25",
                                stroke: "{color}",
                                stroke_width: "2",
                                style: "filter: drop-shadow(0 0 6px {color}66);",
                                pointer_events: "none",
                            }
                            if with_dots {
                                for (pt_idx, (cx, cy)) in coords.iter().enumerate() {
                                    {
                                        let is_h = hover_axis() == Some(pt_idx);
                                        rsx! {
                                            circle {
                                                key: "radar-dot-{orig_idx}-{pt_idx}",
                                                cx: "{cx}",
                                                cy: "{cy}",
                                                r: if is_h { "5.5" } else { "3.5" },
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

                    // Etichette assi interattive
                    for (i, (lbl, lx, ly)) in axis_labels.iter().enumerate() {
                        {
                            let is_h = hover_axis() == Some(i);
                            rsx! {
                                text {
                                    key: "axis-lbl-{i}",
                                    x: "{lx}",
                                    y: "{ly}",
                                    text_anchor: "middle",
                                    dominant_baseline: "middle",
                                    fill: if is_h { "#ffffff" } else { "rgba(255, 255, 255, 0.55)" },
                                    font_size: "10.5",
                                    font_family: "monospace",
                                    font_weight: if is_h { "bold" } else { "normal" },
                                    class: "cursor-pointer transition-colors",
                                    onmouseenter: move |_| hover_axis.set(Some(i)),
                                    onmouseleave: move |_| hover_axis.set(None),
                                    onpointerleave: move |_| hover_axis.set(None),
                                    "{lbl}"
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
