use dioxus::prelude::*;
use std::f64::consts::PI;

use super::primitives::{series_line_style, ChartContainer};

/// Grafico a tachimetro (Gauge) ad arco o barra radiale circolare per metriche di completamento.
#[component]
pub fn GaugeChart(
    value: f64,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 14)] thickness: u32,
    #[props(default)] label: Option<String>,
    #[props(default)] sublabel: Option<String>,
    #[props(default)] color: Option<String>,
    #[props(default)] thresholds: Vec<(f64, String)>,
    #[props(default = 220)] height: u32,
    #[props(default)] class: String,
) -> Element {
    let (lo, hi) = if min <= max { (min, max) } else { (max, min) };
    let clamped_val = value.clamp(lo, hi);
    let range = if (max - min).abs() < f64::EPSILON { 1.0 } else { max - min };
    let fraction = (clamped_val - min) / range;

    let svg_w = 260.0;
    let svg_h = 200.0;
    let center_x = svg_w / 2.0;
    let center_y = 135.0;
    let radius = 85.0;

    // Arco da 150° a 390° (sweep di 240 gradi)
    let start_angle = 150.0 * (PI / 180.0);
    let total_sweep = 240.0 * (PI / 180.0);
    let current_sweep = fraction * total_sweep;
    let end_angle = start_angle + total_sweep;
    let value_angle = start_angle + current_sweep;

    // Colore attivo basato sulle soglie o di default
    let resolved_color = if let Some(c) = color {
        c
    } else if !thresholds.is_empty() {
        let mut chosen = "var(--sd-chart-1)".to_string();
        for (thresh, col) in &thresholds {
            if clamped_val >= *thresh {
                chosen = col.clone();
            }
        }
        chosen
    } else {
        "var(--sd-chart-1)".to_string()
    };

    // Coordinate arco di sfondo
    let bg_x1 = center_x + radius * start_angle.cos();
    let bg_y1 = center_y + radius * start_angle.sin();
    let bg_x2 = center_x + radius * end_angle.cos();
    let bg_y2 = center_y + radius * end_angle.sin();

    let bg_arc_d = format!(
        "M {:.2} {:.2} A {:.2} {:.2} 0 1 1 {:.2} {:.2}",
        bg_x1, bg_y1, radius, radius, bg_x2, bg_y2
    );

    // Coordinate arco di valore
    let val_x2 = center_x + radius * value_angle.cos();
    let val_y2 = center_y + radius * value_angle.sin();
    let large_arc = if current_sweep > PI { 1 } else { 0 };

    let val_arc_d = format!(
        "M {:.2} {:.2} A {:.2} {:.2} 0 {} 1 {:.2} {:.2}",
        bg_x1, bg_y1, radius, radius, large_arc, val_x2, val_y2
    );

    rsx! {
        div { class: "sd-chart-root sd-chart-root--center {class}",
            ChartContainer {
                view_box: "0 0 {svg_w} {svg_h}",
                height: height,

                // Arco di sfondo neutro
                path {
                    d: "{bg_arc_d}",
                    class: "sd-chart__track",
                    stroke_width: "{thickness}",
                    stroke_linecap: "round",
                }

                // Arco di progresso colorato
                if fraction > 0.001 {
                    path {
                        d: "{val_arc_d}",
                        fill: "none",
                        stroke_width: "{thickness}",
                        stroke_linecap: "round",
                        style: "{series_line_style(&resolved_color)}",
                    }
                }

                // Valore e testo centrale
                text {
                    x: "{center_x}",
                    y: "{center_y - 8.0}",
                    text_anchor: "middle",
                    class: "sd-chart__value sd-chart__value--lg",
                    "{clamped_val:.1}"
                }

                if let Some(lbl) = &label {
                    text {
                        x: "{center_x}",
                        y: "{center_y + 14.0}",
                        text_anchor: "middle",
                        class: "sd-chart__caption",
                        "{lbl}"
                    }
                }

                if let Some(sub) = &sublabel {
                    text {
                        x: "{center_x}",
                        y: "{center_y + 30.0}",
                        text_anchor: "middle",
                        class: "sd-chart__label",
                        "{sub}"
                    }
                }

                // Min e Max agli estremi
                text {
                    x: "{bg_x1 - 4.0}",
                    y: "{bg_y1 + 16.0}",
                    text_anchor: "middle",
                    class: "sd-chart__label",
                    "{min:.0}"
                }
                text {
                    x: "{bg_x2 + 4.0}",
                    y: "{bg_y2 + 16.0}",
                    text_anchor: "middle",
                    class: "sd-chart__label",
                    "{max:.0}"
                }
            }
        }
    }
}

/// Barra di progresso circolare radiale (RadialBar a 360°).
#[component]
pub fn RadialBar(
    value: f64,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 10)] thickness: u32,
    #[props(default = 60.0)] radius: f64,
    #[props(default)] color: Option<String>,
    #[props(default)] label: Option<String>,
    #[props(default = 160)] height: u32,
    #[props(default)] class: String,
) -> Element {
    let (lo, hi) = if min <= max { (min, max) } else { (max, min) };
    let clamped_val = value.clamp(lo, hi);
    let range = if (max - min).abs() < f64::EPSILON { 1.0 } else { max - min };
    let fraction = (clamped_val - min) / range;

    let circumference = 2.0 * PI * radius;
    let stroke_offset = circumference * (1.0 - fraction);

    let resolved_color = color.unwrap_or_else(|| "var(--sd-chart-1)".to_string());
    let size = (radius * 2.0 + thickness as f64 * 2.0 + 20.0).max(120.0);
    let center = size / 2.0;

    rsx! {
        div { class: "sd-chart-root sd-chart-root--center {class}",
            ChartContainer {
                view_box: "0 0 {size} {size}",
                height: height,

                // Cerchio di fondo
                circle {
                    cx: "{center}",
                    cy: "{center}",
                    r: "{radius}",
                    class: "sd-chart__track",
                    stroke_width: "{thickness}",
                }

                // Cerchio attivo con dashoffset
                circle {
                    cx: "{center}",
                    cy: "{center}",
                    r: "{radius}",
                    fill: "none",
                    stroke_width: "{thickness}",
                    stroke_dasharray: "{circumference}",
                    stroke_dashoffset: "{stroke_offset}",
                    stroke_linecap: "round",
                    transform: "rotate(-90 {center} {center})",
                    class: "sd-chart__arc",
                    style: "{series_line_style(&resolved_color)}",
                }

                // Testo percentuale interno
                text {
                    x: "{center}",
                    y: if label.is_some() { "{center - 2.0}" } else { "{center + 5.0}" },
                    text_anchor: "middle",
                    class: "sd-chart__value",
                    "{(fraction * 100.0):.0}%"
                }

                if let Some(lbl) = &label {
                    text {
                        x: "{center}",
                        y: "{center + 14.0}",
                        text_anchor: "middle",
                        class: "sd-chart__label",
                        "{lbl}"
                    }
                }
            }
        }
    }
}
