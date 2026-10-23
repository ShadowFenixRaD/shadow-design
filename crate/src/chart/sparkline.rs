use dioxus::prelude::*;

use super::primitives::{generate_svg_area_path, generate_svg_path, CurveType};

/// Direzione o tendenza del trend per colorazione automatica.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TrendDirection {
    #[default]
    Auto,
    Positive,
    Negative,
    Neutral,
}

/// Micro-grafico vettoriale compatto senza assi, ottimizzato per metric card e cruscotti finanziari.
#[component]
pub fn Sparkline(
    data: Vec<f64>,
    #[props(default)] trend: TrendDirection,
    #[props(default = true)] with_fill: bool,
    #[props(default)] curve_type: CurveType,
    #[props(default = 2.0)] stroke_width: f64,
    #[props(default = 120)] width: u32,
    #[props(default = 36)] height: u32,
    #[props(default)] color: Option<String>,
    #[props(default)] class: String,
) -> Element {
    if data.is_empty() {
        return rsx! {
            div { class: "inline-block {class}", style: "width: {width}px; height: {height}px;" }
        };
    }

    let min_val = data.iter().copied().fold(f64::INFINITY, f64::min);
    let max_val = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let range = if (max_val - min_val).abs() < f64::EPSILON {
        1.0
    } else {
        max_val - min_val
    };

    let w = width as f64;
    let h = height as f64;
    let pad_y = stroke_width + 1.0;
    let plot_h = (h - pad_y * 2.0).max(1.0);

    let n = data.len();
    let x_step = if n > 1 { w / (n - 1) as f64 } else { w };

    let coords: Vec<(f64, f64)> = data
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            let cx = i as f64 * x_step;
            let cy = pad_y + plot_h - ((v - min_val) / range) * plot_h;
            (cx, cy)
        })
        .collect();

    // Risoluzione colore in base al trend
    let resolved_color = if let Some(c) = color {
        c
    } else {
        match trend {
            TrendDirection::Positive => "#00ff9d".to_string(), // Emerald
            TrendDirection::Negative => "#ff1744".to_string(), // Crimson
            TrendDirection::Neutral => "#a1a1aa".to_string(),  // Zinc
            TrendDirection::Auto => {
                let first = data.first().copied().unwrap_or(0.0);
                let last = data.last().copied().unwrap_or(0.0);
                if last > first {
                    "#00ff9d".to_string()
                } else if last < first {
                    "#ff1744".to_string()
                } else {
                    "#a855f7".to_string() // Violet neon
                }
            }
        }
    };

    let line_path = generate_svg_path(&coords, curve_type);
    let area_path = generate_svg_area_path(&coords, h, curve_type);

    let grad_id = format!("spark-grad-{:x}", data.len() + width as usize);
    let mut hover_index = use_signal(|| None::<usize>);

    rsx! {
        div {
            class: "relative inline-block select-none shrink-0 {class}",
            style: "width: {width}px; height: {height}px;",
            onmousemove: move |evt| {
                let c = evt.element_coordinates();
                if c.x < 0.0 || c.x > w || c.y < -5.0 || c.y > h + 5.0 {
                    hover_index.set(None);
                    return;
                }
                let ratio = (c.x / w).clamp(0.0, 1.0);
                let idx = (ratio * (n - 1) as f64).round() as usize;
                hover_index.set(Some(idx.min(n - 1)));
            },
            onmouseleave: move |_| {
                hover_index.set(None);
            },
            onpointerleave: move |_| {
                hover_index.set(None);
            },
            svg {
                class: "w-full h-full overflow-visible",
                view_box: "0 0 {width} {height}",
                onmouseleave: move |_| {
                    hover_index.set(None);
                },
                onpointerleave: move |_| {
                    hover_index.set(None);
                },
                defs {
                    linearGradient {
                        id: "{grad_id}",
                        x1: "0",
                        y1: "0",
                        x2: "0",
                        y2: "1",
                        stop {
                            offset: "0%",
                            stop_color: "{resolved_color}",
                            stop_opacity: "0.35",
                        }
                        stop {
                            offset: "100%",
                            stop_color: "{resolved_color}",
                            stop_opacity: "0.0",
                        }
                    }
                }
                if with_fill {
                    path {
                        d: "{area_path}",
                        fill: "url(#{grad_id})",
                        pointer_events: "none",
                    }
                }
                path {
                    d: "{line_path}",
                    fill: "none",
                    stroke: "{resolved_color}",
                    stroke_width: "{stroke_width}",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    style: "filter: drop-shadow(0 0 4px {resolved_color}88);",
                    pointer_events: "none",
                }

                // Punto interattivo attivo su hover
                if let Some(idx) = hover_index() {
                    if idx < coords.len() {
                        {
                            let (cx, cy) = coords[idx];
                            rsx! {
                                circle {
                                    cx: "{cx}",
                                    cy: "{cy}",
                                    r: "3.5",
                                    fill: "{resolved_color}",
                                    stroke: "#ffffff",
                                    stroke_width: "1.5",
                                    pointer_events: "none",
                                }
                            }
                        }
                    }
                }
            }

            // Micro-tooltip con dato live
            if let Some(idx) = hover_index() {
                if idx < data.len() {
                    {
                        let val = data[idx];
                        let (cx, cy) = coords[idx];
                        let is_right = cx > w / 2.0;
                        let (pos_style, transform) = if is_right {
                            (format!("left: {cx}px; top: {cy}px;"), "transform: translate(-100%, -50%); margin-left: -8px;")
                        } else {
                            (format!("left: {cx}px; top: {cy}px;"), "transform: translateY(-50%); margin-left: 8px;")
                        };
                        rsx! {
                            div {
                                class: "pointer-events-none absolute z-[1000] px-2 py-0.5 rounded-md bg-zinc-950/95 border border-white/20 text-[11px] font-mono font-bold text-white shadow-xl backdrop-blur-md whitespace-nowrap",
                                style: "{pos_style} {transform}",
                                "{val:.2}"
                            }
                        }
                    }
                }
            }
        }
    }
}
