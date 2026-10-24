use dioxus::prelude::*;
use std::f64::consts::PI;

use super::primitives::{get_series_color, ChartContainer, ChartTooltip};

/// Singolo spicchio di dati per grafico a torta o ciambella.
#[derive(Clone, PartialEq, Debug)]
pub struct PieSegment {
    pub name: String,
    pub value: f64,
    pub color: Option<String>,
}

impl PieSegment {
    #[must_use]
    pub fn new(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value,
            color: None,
        }
    }

    #[must_use]
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

/// Grafico a torta vettoriale SVG con supporto per hover dinamico ed etichette percentuali.
#[component]
pub fn PieChart(
    data: Vec<PieSegment>,
    #[props(default = true)] with_tooltip: bool,
    #[props(default = false)] with_labels: bool,
    #[props(default)] inner_radius: Option<f64>,
    #[props(default = 2.0)] stroke_width: f64,
    #[props(default)] center_label: Option<String>,
    #[props(default)] center_sublabel: Option<String>,
    #[props(default = 300)] height: u32,
    #[props(default)] class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let mut hover_index = use_signal(|| None::<usize>);

    let svg_w = 400.0;
    let svg_h = 300.0;
    let center_x = svg_w / 2.0;
    let center_y = svg_h / 2.0;
    let outer_radius = 110.0;
    let inner_r = inner_radius.unwrap_or(0.0);

    let total: f64 = data.iter().map(|s| s.value).sum();

    // Calcolo angoli di ciascuno spicchio
    let mut current_angle = -PI / 2.0; // Inizio alle 12 in punto
    let mut slices = Vec::new();

    for (i, seg) in data.iter().enumerate() {
        let fraction = if total > 0.0 { seg.value / total } else { 0.0 };
        let sweep_angle = fraction * 2.0 * PI;
        let start_angle = current_angle;
        let end_angle = current_angle + sweep_angle;
        current_angle = end_angle;

        let color = get_series_color(i, seg.color.as_deref());
        slices.push((i, seg.name.clone(), seg.value, fraction, color, start_angle, end_angle));
    }

    let current_tooltip_data = hover_index().and_then(|idx| {
        let item = slices.iter().find(|s| s.0 == idx)?;
        let (_, ref name, val, frac, ref col, start_a, end_a) = *item;
        let pct_str = format!("{:.1}%", frac * 100.0);
        let entries = vec![
            (name.clone(), format!("{val:.1}"), col.clone()),
            (locale.chart_percentage.clone(), pct_str, col.clone()),
        ];
        let mid_a = (start_a + end_a) / 2.0;
        let r_mid = (inner_r + outer_radius) / 2.0;
        let cx = center_x + r_mid * mid_a.cos();
        let cy = center_y + r_mid * mid_a.sin();
        let x_pct = (cx / svg_w) * 100.0;
        let y_pct = (cy / svg_h) * 100.0;
        let is_right = x_pct > 50.0;
        Some((name.clone(), entries, x_pct, y_pct, is_right))
    });

    rsx! {
        div { class: "w-full flex flex-col items-center {class}",
            div {
                class: "relative w-full max-w-md",
                onmouseleave: move |_| {
                    hover_index.set(None);
                },
                onpointerleave: move |_| {
                    hover_index.set(None);
                },
                ChartContainer {
                    view_box: "0 0 {svg_w} {svg_h}",
                    height: height,
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

                    for (idx, _name, _, _, color, start_a, end_a) in &slices {
                        {
                            let is_hovered = hover_index() == Some(*idx);
                            let r_out = if is_hovered { outer_radius + 6.0 } else { outer_radius };
                            let r_in = inner_r;

                            // Calcolo coordinate arco SVG
                            let x1_out = center_x + r_out * start_a.cos();
                            let y1_out = center_y + r_out * start_a.sin();
                            let x2_out = center_x + r_out * end_a.cos();
                            let y2_out = center_y + r_out * end_a.sin();

                            let sweep = end_a - start_a;
                            let is_full_circle = sweep >= 2.0 * std::f64::consts::PI - 0.0001;
                            let large_arc = if sweep > std::f64::consts::PI { 1 } else { 0 };

                            let path_d = if is_full_circle {
                                if r_in > 0.0 {
                                    format!(
                                        "M {:.2} {:.2} A {:.2} {:.2} 0 1 1 {:.2} {:.2} A {:.2} {:.2} 0 1 1 {:.2} {:.2} M {:.2} {:.2} A {:.2} {:.2} 0 1 0 {:.2} {:.2} A {:.2} {:.2} 0 1 0 {:.2} {:.2} Z",
                                        center_x, center_y - r_out, r_out, r_out, center_x, center_y + r_out, r_out, r_out, center_x, center_y - r_out,
                                        center_x, center_y - r_in, r_in, r_in, center_x, center_y + r_in, r_in, r_in, center_x, center_y - r_in
                                    )
                                } else {
                                    format!(
                                        "M {:.2} {:.2} A {:.2} {:.2} 0 1 1 {:.2} {:.2} A {:.2} {:.2} 0 1 1 {:.2} {:.2} Z",
                                        center_x, center_y - r_out, r_out, r_out, center_x, center_y + r_out, r_out, r_out, center_x, center_y - r_out
                                    )
                                }
                            } else if r_in > 0.0 {
                                let x1_in = center_x + r_in * end_a.cos();
                                let y1_in = center_y + r_in * end_a.sin();
                                let x2_in = center_x + r_in * start_a.cos();
                                let y2_in = center_y + r_in * start_a.sin();

                                format!(
                                    "M {:.2} {:.2} A {:.2} {:.2} 0 {} 1 {:.2} {:.2} L {:.2} {:.2} A {:.2} {:.2} 0 {} 0 {:.2} {:.2} Z",
                                    x1_out, y1_out, r_out, r_out, large_arc, x2_out, y2_out,
                                    x1_in, y1_in, r_in, r_in, large_arc, x2_in, y2_in
                                )
                            } else {
                                format!(
                                    "M {:.2} {:.2} L {:.2} {:.2} A {:.2} {:.2} 0 {} 1 {:.2} {:.2} Z",
                                    center_x, center_y, x1_out, y1_out, r_out, r_out, large_arc, x2_out, y2_out
                                )
                            };

                            let n_idx = *idx;
                            rsx! {
                                path {
                                    key: "slice-{idx}",
                                    d: "{path_d}",
                                    fill: "{color}",
                                    stroke: if is_hovered { "#ffffff" } else { "#09090b" },
                                    stroke_width: if is_hovered { "2.5" } else { "{stroke_width}" },
                                    opacity: if hover_index().is_some() && !is_hovered { "0.6" } else { "1.0" },
                                    style: "cursor: pointer;",
                                    class: "transition-all duration-150",
                                    onmouseenter: move |_| hover_index.set(Some(n_idx)),
                                    onmouseleave: move |_| hover_index.set(None),
                                    onpointerleave: move |_| hover_index.set(None),
                                }
                            }
                        }
                    }

                    // Elemento centrale opzionale per Donut (KPI)
                    if inner_r > 30.0 {
                        g { class: "pointer-events-none select-none text-center",
                            if let Some(lbl) = &center_label {
                                text {
                                    x: "{center_x}",
                                    y: if center_sublabel.is_some() { "{center_y - 4.0}" } else { "{center_y + 6.0}" },
                                    text_anchor: "middle",
                                    fill: "#ffffff",
                                    font_size: "18",
                                    font_weight: "bold",
                                    font_family: "monospace",
                                    "{lbl}"
                                }
                            }
                            if let Some(sub) = &center_sublabel {
                                text {
                                    x: "{center_x}",
                                    y: "{center_y + 16.0}",
                                    text_anchor: "middle",
                                    fill: "rgba(255, 255, 255, 0.45)",
                                    font_size: "11",
                                    font_family: "monospace",
                                    "{sub}"
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

            // Legenda sottostante
            div { class: "flex flex-wrap items-center justify-center gap-4 mt-2 text-xs font-mono",
                for (idx, seg) in data.iter().enumerate() {
                    {
                        let color = get_series_color(idx, seg.color.as_deref());
                        let is_h = hover_index() == Some(idx);
                        let leg_cls = if is_h { "opacity-100 font-bold" } else { "opacity-70" };
                        rsx! {
                            div {
                                key: "leg-{idx}",
                                class: "flex items-center gap-2 cursor-pointer transition-opacity {leg_cls}",
                                onmouseenter: move |_| hover_index.set(Some(idx)),
                                onmouseleave: move |_| hover_index.set(None),
                                span {
                                    class: "w-2.5 h-2.5 rounded-full shrink-0",
                                    style: "background-color: {color}; box-shadow: 0 0 6px {color}66;",
                                }
                                span { "{seg.name}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Grafico a ciambella vettoriale SVG (variante di `PieChart` con raggio interno preconfigurato).
#[component]
pub fn DonutChart(
    data: Vec<PieSegment>,
    #[props(default = true)] with_tooltip: bool,
    #[props(default = false)] with_labels: bool,
    #[props(default = 65.0)] inner_radius: f64,
    #[props(default = 2.0)] stroke_width: f64,
    #[props(default)] center_label: Option<String>,
    #[props(default)] center_sublabel: Option<String>,
    #[props(default = 300)] height: u32,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        PieChart {
            data: data,
            with_tooltip: with_tooltip,
            with_labels: with_labels,
            inner_radius: Some(inner_radius),
            stroke_width: stroke_width,
            center_label: center_label,
            center_sublabel: center_sublabel,
            height: height,
            class: class,
        }
    }
}
