use dioxus::prelude::*;
use std::collections::BTreeSet;

use super::primitives::{
    get_series_color, ChartContainer, ChartDataPoint, ChartGrid, ChartLegend, ChartOrientation,
    ChartSeries, ChartTooltip,
};

/// Modalità di calcolo e disposizione delle barre.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BarChartType {
    #[default]
    Default,
    Stacked,
    Percent,
}

/// Istogramma vettoriale SVG a barre verticali o orizzontali, raggruppate o impilate.
#[component]
pub fn BarChart(
    data: Vec<ChartDataPoint>,
    series: Vec<ChartSeries>,
    #[props(default)] orientation: ChartOrientation,
    #[props(default)] chart_type: BarChartType,
    #[props(default = 4)] bar_radius: u32,
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

    let max_val = match chart_type {
        BarChartType::Percent => 100.0,
        BarChartType::Stacked => {
            let mut max_sum = 1.0f64;
            for p in &data {
                let sum: f64 = active_series
                    .iter()
                    .filter_map(|(_, s)| p.get(&s.name))
                    .sum();
                if sum > max_sum {
                    max_sum = sum;
                }
            }
            max_sum
        }
        BarChartType::Default => {
            let mut max = 1.0f64;
            for p in &data {
                for (_, s) in &active_series {
                    if let Some(v) = p.get(&s.name) {
                        if v > max {
                            max = v;
                        }
                    }
                }
            }
            max
        }
    };

    let n = data.len();
    let group_w = if n > 0 { plot_w / n as f64 } else { plot_w };
    let group_inner_w = group_w * 0.75;
    let group_offset_x = (group_w - group_inner_w) / 2.0;

    let num_active = active_series.len().max(1);
    let bar_w = group_inner_w / num_active as f64;

    let baseline_y = padding_top + plot_h;

    let current_tooltip_data = hover_index().and_then(|idx| {
        if idx >= data.len() {
            return None;
        }
        let pt = &data[idx];
        let mut entries = Vec::new();
        for &(orig_idx, s) in &active_series {
            if let Some(v) = pt.get(&s.name) {
                let col = get_series_color(orig_idx, s.color.as_deref());
                let s_label = s.display_label().to_string();
                entries.push((s_label, format!("{v:.1}"), col));
            }
        }
        let cx = padding_left + idx as f64 * group_w + group_w / 2.0;
        Some((pt.label.clone(), entries, cx, padding_top + 20.0))
    });

    rsx! {
        div { class: "w-full flex flex-col {class}",
            div { class: "relative w-full",
                ChartContainer {
                    view_box: "0 0 {svg_w} {svg_h}",
                    height: height,
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

                    // Render delle barre
                    for (i, pt) in data.iter().enumerate() {
                        {
                            let is_hovered = hover_index() == Some(i);
                            let group_x = padding_left + i as f64 * group_w + group_offset_x;
                            let opacity_val = if is_hovered { "1.0" } else { "0.85" };
                            let label_color = if is_hovered { "rgba(255, 255, 255, 0.9)" } else { "rgba(255, 255, 255, 0.45)" };

                            rsx! {
                                g { key: "bar-group-{i}",
                                    if chart_type == BarChartType::Default {
                                        for (sub_idx, &(orig_idx, s)) in active_series.iter().enumerate() {
                                            {
                                                let val = pt.get(&s.name).unwrap_or(0.0);
                                                let bh = ((val / max_val) * plot_h).max(0.0);
                                                let bx = group_x + sub_idx as f64 * bar_w;
                                                let by = baseline_y - bh;
                                                let color = get_series_color(orig_idx, s.color.as_deref());
                                                let shadow_style = if is_hovered { format!("filter: drop-shadow(0 0 8px {color}aa);") } else { String::new() };

                                                rsx! {
                                                    rect {
                                                        key: "bar-{i}-{sub_idx}",
                                                        x: "{bx + 1.0}",
                                                        y: "{by}",
                                                        width: "{(bar_w - 2.0).max(1.0)}",
                                                        height: "{bh}",
                                                        rx: "{bar_radius}",
                                                        ry: "{bar_radius}",
                                                        fill: "{color}",
                                                        opacity: "{opacity_val}",
                                                        style: "{shadow_style}",
                                                        class: "transition-all duration-150",
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let total: f64 = active_series.iter().filter_map(|(_, s)| pt.get(&s.name)).sum();
                                            let mut current_bottom = baseline_y;

                                            rsx! {
                                                for &(orig_idx, s) in &active_series {
                                                    {
                                                        let val = pt.get(&s.name).unwrap_or(0.0);
                                                        let norm_val = if chart_type == BarChartType::Percent {
                                                            if total > 0.0 { (val / total) * 100.0 } else { 0.0 }
                                                        } else {
                                                            val
                                                        };
                                                        let bh = ((norm_val / max_val) * plot_h).max(0.0);
                                                        let by = current_bottom - bh;
                                                        let color = get_series_color(orig_idx, s.color.as_deref());
                                                        current_bottom = by;
                                                        let shadow_style = if is_hovered { format!("filter: drop-shadow(0 0 8px {color}aa);") } else { String::new() };

                                                        rsx! {
                                                            rect {
                                                                key: "stack-bar-{i}-{orig_idx}",
                                                                x: "{group_x}",
                                                                y: "{by}",
                                                                width: "{group_inner_w}",
                                                                height: "{bh}",
                                                                rx: "{bar_radius}",
                                                                ry: "{bar_radius}",
                                                                fill: "{color}",
                                                                opacity: "{opacity_val}",
                                                                style: "{shadow_style}",
                                                                class: "transition-all duration-150",
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Etichetta asse X
                                    text {
                                        x: "{group_x + group_inner_w / 2.0}",
                                        y: "{baseline_y + 20.0}",
                                        text_anchor: "middle",
                                        fill: "{label_color}",
                                        font_size: "11",
                                        font_family: "monospace",
                                        "{pt.label}"
                                    }

                                    // Zona interattiva hover
                                    rect {
                                        x: "{padding_left + i as f64 * group_w}",
                                        y: "{padding_top}",
                                        width: "{group_w}",
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
                }

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
