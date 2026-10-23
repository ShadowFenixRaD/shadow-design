use dioxus::prelude::*;

use super::primitives::{ChartContainer, ChartTooltip};

/// Singola cella di una matrice o mappa di calore.
#[derive(Clone, PartialEq, Debug)]
pub struct HeatmapCell {
    pub x: usize,
    pub y: usize,
    pub value: f64,
    pub label: Option<String>,
    pub date: Option<String>,
}

impl HeatmapCell {
    #[must_use]
    pub fn new(x: usize, y: usize, value: f64) -> Self {
        Self {
            x,
            y,
            value,
            label: None,
            date: None,
        }
    }

    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    #[must_use]
    pub fn with_date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }
}

/// Mappa di calore vettoriale a matrice per intensità di frequenza o carichi di lavoro.
#[component]
pub fn Heatmap(
    data: Vec<HeatmapCell>,
    #[props(default)] color_scale: Vec<String>,
    #[props(default = 14)] cell_size: u32,
    #[props(default = 3)] cell_gap: u32,
    #[props(default = 2)] cell_radius: u32,
    #[props(default)] x_labels: Vec<String>,
    #[props(default)] y_labels: Vec<String>,
    #[props(default = true)] with_tooltip: bool,
    #[props(default = 220)] height: u32,
    #[props(default)] class: String,
) -> Element {
    let mut hover_cell = use_signal(|| None::<usize>);
    let mut mouse_pos = use_signal(|| (0.0f64, 0.0f64));

    // Calcolo dimensioni griglia (max X e max Y)
    let max_x = data.iter().map(|c| c.x).max().unwrap_or(0);
    let max_y = data.iter().map(|c| c.y).max().unwrap_or(0);
    let max_val = data.iter().map(|c| c.value).fold(1.0f64, f64::max);

    let default_palette = vec![
        "#18181b".to_string(), // Zinc 900 (quasi spento)
        "#3b0764".to_string(), // Viola scuro
        "#6b21a8".to_string(), // Viola medio
        "#a855f7".to_string(), // Viola neon
        "#d8b4fe".to_string(), // Viola brillante
    ];

    let palette = if color_scale.is_empty() {
        default_palette
    } else {
        color_scale
    };

    let step = (cell_size + cell_gap) as f64;
    let pad_left = if y_labels.is_empty() { 20.0 } else { 45.0 };
    let pad_top = 20.0;

    let svg_w = pad_left + (max_x + 1) as f64 * step + 20.0;
    let svg_h = pad_top + (max_y + 1) as f64 * step + 30.0;

    let current_tooltip_data = hover_cell().and_then(|idx| {
        if idx >= data.len() {
            return None;
        }
        let cell = &data[idx];
        let title = cell
            .label
            .clone()
            .or_else(|| cell.date.clone())
            .unwrap_or_else(|| format!("Cella ({}, {})", cell.x, cell.y));

        let entries = vec![
            ("Valore".to_string(), format!("{:.1}", cell.value), "#a855f7".to_string()),
        ];
        let cx = pad_left + cell.x as f64 * step + (cell_size as f64 / 2.0);
        let cy = pad_top + cell.y as f64 * step + (cell_size as f64 / 2.0);
        let my = if mouse_pos().1 > 0.0 { mouse_pos().1 } else { cy };
        let mx = if mouse_pos().0 > 0.0 { mouse_pos().0 } else { cx };
        let is_right = cx > svg_w / 2.0 || mouse_pos().0 > 300.0;
        Some((title, entries, mx, my, is_right))
    });

    rsx! {
        div { class: "w-full flex flex-col {class}",
            div {
                class: "relative w-full overflow-visible",
                onmousemove: move |evt| {
                    let c = evt.element_coordinates();
                    mouse_pos.set((c.x, c.y));
                    let mut found = None;
                    if c.x >= pad_left && c.y >= pad_top {
                        let grid_x = ((c.x - pad_left) / step).floor() as isize;
                        let grid_y = ((c.y - pad_top) / step).floor() as isize;
                        let off_x = (c.x - pad_left) - (grid_x as f64 * step);
                        let off_y = (c.y - pad_top) - (grid_y as f64 * step);
                        if off_x <= cell_size as f64 && off_y <= cell_size as f64 {
                            for (idx, cell) in data.iter().enumerate() {
                                if cell.x as isize == grid_x && cell.y as isize == grid_y {
                                    found = Some(idx);
                                    break;
                                }
                            }
                        }
                    }
                    hover_cell.set(found);
                },
                onmouseleave: move |_| {
                    hover_cell.set(None);
                },
                ChartContainer {
                    view_box: "0 0 {svg_w} {svg_h}",
                    height: height,

                    // Etichette asse Y
                    for (yi, yl) in y_labels.iter().enumerate() {
                        {
                            let y_pos = pad_top + yi as f64 * step + (cell_size as f64 / 2.0) + 4.0;
                            rsx! {
                                text {
                                    key: "ylbl-{yi}",
                                    x: "{pad_left - 8.0}",
                                    y: "{y_pos}",
                                    text_anchor: "end",
                                    fill: "rgba(255, 255, 255, 0.4)",
                                    font_size: "10",
                                    font_family: "monospace",
                                    "{yl}"
                                }
                            }
                        }
                    }

                    // Render celle
                    for (i, cell) in data.iter().enumerate() {
                        {
                            let bx = pad_left + cell.x as f64 * step;
                            let by = pad_top + cell.y as f64 * step;

                            let frac = if max_val > 0.0 { (cell.value / max_val).clamp(0.0, 1.0) } else { 0.0 };
                            let pal_idx = ((frac * (palette.len() - 1) as f64).round() as usize).min(palette.len() - 1);
                            let color = &palette[pal_idx];

                            let is_h = hover_cell() == Some(i);

                            rsx! {
                                rect {
                                    key: "cell-{i}",
                                    x: "{bx}",
                                    y: "{by}",
                                    width: "{cell_size}",
                                    height: "{cell_size}",
                                    rx: "{cell_radius}",
                                    ry: "{cell_radius}",
                                    fill: "{color}",
                                    stroke: if is_h { "#ffffff" } else { "rgba(255, 255, 255, 0.04)" },
                                    stroke_width: if is_h { "1.5" } else { "1" },
                                    class: "cursor-pointer transition-all duration-150",
                                    style: if is_h { "filter: drop-shadow(0 0 6px {color});" } else { "" },
                                    onmouseenter: move |_| hover_cell.set(Some(i)),
                                    onmouseleave: move |_| hover_cell.set(None),
                                }
                            }
                        }
                    }

                    // Etichette asse X
                    for (xi, xl) in x_labels.iter().enumerate() {
                        {
                            let x_pos = pad_left + xi as f64 * step + (cell_size as f64 / 2.0);
                            let y_pos = pad_top + (max_y + 1) as f64 * step + 16.0;
                            rsx! {
                                text {
                                    key: "xlbl-{xi}",
                                    x: "{x_pos}",
                                    y: "{y_pos}",
                                    text_anchor: "middle",
                                    fill: "rgba(255, 255, 255, 0.4)",
                                    font_size: "10",
                                    font_family: "monospace",
                                    "{xl}"
                                }
                            }
                        }
                    }
                }

                if with_tooltip {
                    if let Some((title, entries, x_pos, y_pos, align_r)) = current_tooltip_data {
                        ChartTooltip {
                            title: title,
                            entries: entries,
                            x: x_pos,
                            y: y_pos,
                            align_right: align_r,
                            visible: true,
                        }
                    }
                }
            }
        }
    }
}

/// Grafico calendario delle attività (in stile commit graph di Forgejo / GitHub).
#[component]
pub fn CalendarHeatmap(
    data: Vec<HeatmapCell>,
    #[props(default)] color_scale: Vec<String>,
    #[props(default = 11)] cell_size: u32,
    #[props(default = 3)] cell_gap: u32,
    #[props(default = 140)] height: u32,
    #[props(default)] class: String,
) -> Element {
    let y_days = vec![
        "Lun".to_string(),
        "".to_string(),
        "Mer".to_string(),
        "".to_string(),
        "Ven".to_string(),
        "".to_string(),
        "".to_string(),
    ];

    rsx! {
        Heatmap {
            data: data,
            color_scale: color_scale,
            cell_size: cell_size,
            cell_gap: cell_gap,
            cell_radius: 2,
            y_labels: y_days,
            height: height,
            class: class,
        }
    }
}
