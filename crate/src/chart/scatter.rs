use dioxus::prelude::*;

use super::primitives::{get_series_color, ChartContainer, ChartGrid, ChartTooltip};

/// Singolo punto o bolla cartesiana per grafici a dispersione.
#[derive(Clone, PartialEq, Debug)]
pub struct ScatterPoint {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
    pub label: Option<String>,
    pub series: Option<String>,
    pub color: Option<String>,
}

impl ScatterPoint {
    #[must_use]
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            z: None,
            label: None,
            series: None,
            color: None,
        }
    }

    #[must_use]
    pub fn with_z(mut self, z: f64) -> Self {
        self.z = Some(z);
        self
    }

    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    #[must_use]
    pub fn with_series(mut self, series: impl Into<String>) -> Self {
        self.series = Some(series.into());
        self
    }

    #[must_use]
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

/// Tipo di scala per assi cartesiani.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ScaleType {
    #[default]
    Linear,
    Log,
}

/// Grafico a dispersione (Scatter) e a bolle (Bubble) vettoriale SVG.
#[component]
pub fn ScatterChart(
    data: Vec<ScatterPoint>,
    #[props(default)] x_label: Option<String>,
    #[props(default)] y_label: Option<String>,
    #[props(default)] x_scale: ScaleType,
    #[props(default)] y_scale: ScaleType,
    #[props(default = true)] with_grid: bool,
    #[props(default = true)] with_tooltip: bool,
    #[props(default = 300)] height: u32,
    #[props(default)] class: String,
) -> Element {
    let mut hover_index = use_signal(|| None::<usize>);

    let svg_w = 600.0;
    let svg_h = 260.0;
    let padding_left = 60.0;
    let padding_right = 30.0;
    let padding_top = 20.0;
    let padding_bottom = 40.0;

    let plot_w = svg_w - padding_left - padding_right;
    let plot_h = svg_h - padding_top - padding_bottom;

    // Calcolo min e max X/Y/Z
    let (mut min_x, mut max_x) = (0.0f64, 1.0f64);
    let (mut min_y, mut max_y) = (0.0f64, 1.0f64);
    let (mut min_z, mut max_z) = (1.0f64, 10.0f64);

    if !data.is_empty() {
        min_x = data[0].x;
        max_x = data[0].x;
        min_y = data[0].y;
        max_y = data[0].y;
        for p in &data {
            if p.x < min_x { min_x = p.x; }
            if p.x > max_x { max_x = p.x; }
            if p.y < min_y { min_y = p.y; }
            if p.y > max_y { max_y = p.y; }
            if let Some(z) = p.z {
                if z < min_z { min_z = z; }
                if z > max_z { max_z = z; }
            }
        }
    }

    if (max_x - min_x).abs() < f64::EPSILON { max_x = min_x + 1.0; }
    if (max_y - min_y).abs() < f64::EPSILON { max_y = min_y + 1.0; }
    if (max_z - min_z).abs() < f64::EPSILON { max_z = min_z + 1.0; }

    let baseline_y = padding_top + plot_h;

    // Mappatura coordinate
    let mapped_points: Vec<(usize, f64, f64, f64, String, ScatterPoint)> = data
        .iter()
        .enumerate()
        .map(|(i, pt)| {
            let cx = padding_left + ((pt.x - min_x) / (max_x - min_x)) * plot_w;
            let cy = padding_top + plot_h - ((pt.y - min_y) / (max_y - min_y)) * plot_h;

            let radius = if let Some(z) = pt.z {
                let frac = (z - min_z) / (max_z - min_z);
                4.0 + frac * 18.0
            } else {
                5.0
            };

            let color = pt
                .color
                .clone()
                .unwrap_or_else(|| get_series_color(i, None));

            (i, cx, cy, radius, color, pt.clone())
        })
        .collect();

    let current_tooltip_data = hover_index().and_then(|idx| {
        if idx >= mapped_points.len() {
            return None;
        }
        let (_, cx, cy, _, ref col, ref pt) = mapped_points[idx];
        let title = pt.label.clone().unwrap_or_else(|| format!("Punto #{}", idx + 1));
        let mut entries = vec![
            ("X".to_string(), format!("{:.1}", pt.x), col.clone()),
            ("Y".to_string(), format!("{:.1}", pt.y), col.clone()),
        ];
        if let Some(z) = pt.z {
            entries.push(("Z / Peso".to_string(), format!("{z:.1}"), col.clone()));
        }
        let x_pct = (cx / svg_w) * 100.0;
        let y_pct = (cy / svg_h) * 100.0;
        let is_right = x_pct > 60.0;
        Some((title, entries, x_pct, y_pct, is_right))
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
                            x_ticks: 5,
                            horizontal: true,
                            vertical: true,
                        }
                    }

                    // Punti o Bolle
                    for (idx, cx, cy, radius, color, _) in &mapped_points {
                        {
                            let is_h = hover_index() == Some(*idx);
                            let r_draw = if is_h { radius + 3.0 } else { *radius };
                            let n_idx = *idx;

                            rsx! {
                                circle {
                                    key: "scatter-{idx}",
                                    cx: "{cx}",
                                    cy: "{cy}",
                                    r: "{r_draw}",
                                    fill: "{color}",
                                    fill_opacity: if is_h { "1.0" } else { "0.7" },
                                    stroke: if is_h { "#ffffff" } else { "{color}" },
                                    stroke_width: if is_h { "2.5" } else { "1.5" },
                                    class: "cursor-pointer transition-all duration-150",
                                    onmouseenter: move |_| hover_index.set(Some(n_idx)),
                                    onmouseleave: move |_| hover_index.set(None),
                                    onpointerleave: move |_| hover_index.set(None),
                                }
                            }
                        }
                    }

                    // Etichette min/max X e Y
                    text {
                        x: "{padding_left}",
                        y: "{baseline_y + 18.0}",
                        fill: "rgba(255, 255, 255, 0.45)",
                        font_size: "10.5",
                        font_family: "monospace",
                        pointer_events: "none",
                        "{min_x:.1}"
                    }
                    text {
                        x: "{padding_left + plot_w}",
                        y: "{baseline_y + 18.0}",
                        text_anchor: "end",
                        fill: "rgba(255, 255, 255, 0.45)",
                        font_size: "10.5",
                        font_family: "monospace",
                        pointer_events: "none",
                        "{max_x:.1}"
                    }
                    text {
                        x: "{padding_left - 8.0}",
                        y: "{padding_top + plot_h}",
                        text_anchor: "end",
                        dominant_baseline: "middle",
                        fill: "rgba(255, 255, 255, 0.45)",
                        font_size: "10.5",
                        font_family: "monospace",
                        pointer_events: "none",
                        "{min_y:.1}"
                    }
                    text {
                        x: "{padding_left - 8.0}",
                        y: "{padding_top}",
                        text_anchor: "end",
                        dominant_baseline: "middle",
                        fill: "rgba(255, 255, 255, 0.45)",
                        font_size: "10.5",
                        font_family: "monospace",
                        pointer_events: "none",
                        "{max_y:.1}"
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
        }
    }
}

/// Grafico a bolle (variante con raggio variabile derivato dalla terza coordinata Z).
#[component]
pub fn BubbleChart(
    data: Vec<ScatterPoint>,
    #[props(default)] x_label: Option<String>,
    #[props(default)] y_label: Option<String>,
    #[props(default = 300)] height: u32,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        ScatterChart {
            data: data,
            x_label: x_label,
            y_label: y_label,
            height: height,
            class: class,
        }
    }
}
