use dioxus::prelude::*;
use std::collections::BTreeMap;

/// Tipo di curva di interpolazione tra i punti dati.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CurveType {
    #[default]
    Monotone,
    Linear,
    Step,
}

/// Orientamento di un grafico cartesiano o a barre.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ChartOrientation {
    #[default]
    Vertical,
    Horizontal,
}

/// Singolo punto dati cartesiano con etichetta asse e valori per serie multiple.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ChartDataPoint {
    pub label: String,
    pub values: BTreeMap<String, Option<f64>>,
}

impl ChartDataPoint {
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            values: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn with_value(mut self, key: impl Into<String>, val: f64) -> Self {
        self.values.insert(key.into(), Some(val));
        self
    }

    #[must_use]
    pub fn with_optional_value(mut self, key: impl Into<String>, val: Option<f64>) -> Self {
        self.values.insert(key.into(), val);
        self
    }

    #[must_use]
    pub fn point<I, K>(label: impl Into<String>, items: I) -> Self
    where
        I: IntoIterator<Item = (K, f64)>,
        K: Into<String>,
    {
        let mut p = Self::new(label);
        for (k, v) in items {
            p.values.insert(k.into(), Some(v));
        }
        p
    }

    #[must_use]
    pub fn get(&self, key: &str) -> Option<f64> {
        self.values.get(key).copied().flatten()
    }
}

/// Definizione di una serie di dati all'interno del grafico.
#[derive(Clone, PartialEq, Debug)]
pub struct ChartSeries {
    pub name: String,
    pub label: Option<String>,
    pub color: Option<String>,
    pub dashed: bool,
}

impl ChartSeries {
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: None,
            color: None,
            dashed: false,
        }
    }

    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    #[must_use]
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    #[must_use]
    pub fn with_dashed(mut self, dashed: bool) -> Self {
        self.dashed = dashed;
        self
    }

    #[must_use]
    pub fn display_label(&self) -> &str {
        self.label.as_deref().unwrap_or(&self.name)
    }
}

/// Palette di colori di default per le serie dei grafici Shadow.
pub const DEFAULT_CHART_COLORS: &[&str] = &[
    "#a855f7", // Violet neon (primario ultraviolet)
    "#00f0ff", // Cyber cyan
    "#00ff9d", // Emerald matrix
    "#ff1744", // Crimson fire
    "#f59e0b", // Amber glow
    "#3b82f6", // Electric blue
    "#ec4899", // Neon pink
    "#8b5cf6", // Indigo
];

#[must_use]
pub fn get_series_color(index: usize, custom: Option<&str>) -> String {
    if let Some(c) = custom {
        c.to_string()
    } else {
        DEFAULT_CHART_COLORS[index % DEFAULT_CHART_COLORS.len()].to_string()
    }
}

/// Calcola i limiti min e max di una serie di valori opzionali.
#[must_use]
pub fn compute_min_max(points: &[ChartDataPoint], series: &[ChartSeries]) -> (f64, f64) {
    let mut min_val = 0.0f64;
    let mut max_val = 1.0f64;
    let mut initialized = false;

    for p in points {
        for s in series {
            if let Some(v) = p.get(&s.name) {
                if !initialized {
                    min_val = v;
                    max_val = v;
                    initialized = true;
                } else {
                    if v < min_val {
                        min_val = v;
                    }
                    if v > max_val {
                        max_val = v;
                    }
                }
            }
        }
    }

    if min_val > 0.0 {
        min_val = 0.0;
    }
    if (max_val - min_val).abs() < f64::EPSILON {
        max_val = min_val + 10.0;
    }

    (min_val, max_val)
}

/// Genera il path SVG interpolato per una serie di punti cartesiani (x, y).
#[must_use]
pub fn generate_svg_path(coords: &[(f64, f64)], curve: CurveType) -> String {
    if coords.is_empty() {
        return String::new();
    }
    if coords.len() == 1 {
        return format!("M {:.2} {:.2}", coords[0].0, coords[0].1);
    }

    match curve {
        CurveType::Linear => {
            let mut d = format!("M {:.2} {:.2}", coords[0].0, coords[0].1);
            for pt in &coords[1..] {
                d.push_str(&format!(" L {:.2} {:.2}", pt.0, pt.1));
            }
            d
        }
        CurveType::Step => {
            let mut d = format!("M {:.2} {:.2}", coords[0].0, coords[0].1);
            for pt in &coords[1..] {
                d.push_str(&format!(" H {:.2} V {:.2}", pt.0, pt.1));
            }
            d
        }
        CurveType::Monotone => {
            let mut d = format!("M {:.2} {:.2}", coords[0].0, coords[0].1);
            let n = coords.len();
            for i in 0..n - 1 {
                let p0 = if i > 0 { coords[i - 1] } else { coords[i] };
                let p1 = coords[i];
                let p2 = coords[i + 1];
                let p3 = if i + 2 < n { coords[i + 2] } else { p2 };

                let cp1x = p1.0 + (p2.0 - p0.0) / 6.0;
                let cp1y = p1.1 + (p2.1 - p0.1) / 6.0;
                let cp2x = p2.0 - (p3.0 - p1.0) / 6.0;
                let cp2y = p2.1 - (p3.1 - p1.1) / 6.0;

                d.push_str(&format!(
                    " C {:.2} {:.2}, {:.2} {:.2}, {:.2} {:.2}",
                    cp1x, cp1y, cp2x, cp2y, p2.0, p2.1
                ));
            }
            d
        }
    }
}

/// Genera un path chiuso per il gradiente sottostante (Area).
#[must_use]
pub fn generate_svg_area_path(coords: &[(f64, f64)], baseline_y: f64, curve: CurveType) -> String {
    if coords.is_empty() {
        return String::new();
    }
    let line_path = generate_svg_path(coords, curve);
    let last = coords.last().unwrap();
    let first = coords.first().unwrap();

    format!(
        "{} L {:.2} {:.2} L {:.2} {:.2} Z",
        line_path, last.0, baseline_y, first.0, baseline_y
    )
}

/// Contenitore responsive SVG per i grafici vettoriali Shadow.
#[component]
pub fn ChartContainer(
    view_box: String,
    #[props(default = 300)] height: u32,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "sd-chart w-full relative select-none {class}",
            style: "min-height: {height}px;",
            svg {
                class: "sd-chart__svg w-full h-full overflow-visible",
                view_box: "{view_box}",
                preserve_aspect_ratio: "xMidYMid meet",
                {children}
            }
        }
    }
}

/// Griglia cartesiana con linee guida cartesiane a bassa opacità.
#[component]
pub fn ChartGrid(
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    #[props(default = 4)] y_ticks: usize,
    #[props(default = true)] horizontal: bool,
    #[props(default = false)] vertical: bool,
    #[props(default = 5)] x_ticks: usize,
) -> Element {
    rsx! {
        g { class: "sd-chart__grid",
            if horizontal {
                for i in 0..=y_ticks {
                    {
                        let line_y = y + (height / y_ticks as f64) * i as f64;
                        rsx! {
                            line {
                                key: "h-{i}",
                                x1: "{x}",
                                y1: "{line_y}",
                                x2: "{x + width}",
                                y2: "{line_y}",
                                stroke: "rgba(255, 255, 255, 0.08)",
                                stroke_width: "1",
                                stroke_dasharray: "3 3",
                            }
                        }
                    }
                }
            }
            if vertical {
                for i in 0..=x_ticks {
                    {
                        let line_x = x + (width / x_ticks as f64) * i as f64;
                        rsx! {
                            line {
                                key: "v-{i}",
                                x1: "{line_x}",
                                y1: "{y}",
                                x2: "{line_x}",
                                y2: "{y + height}",
                                stroke: "rgba(255, 255, 255, 0.08)",
                                stroke_width: "1",
                                stroke_dasharray: "3 3",
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Legenda interattiva con toggle di visibilità delle serie.
#[component]
pub fn ChartLegend(
    series: Vec<ChartSeries>,
    hidden_series: Signal<std::collections::BTreeSet<String>>,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        div { class: "sd-chart__legend flex flex-wrap items-center justify-center gap-4 mt-3 text-xs font-mono {class}",
            for (idx, s) in series.into_iter().enumerate() {
                {
                    let name = s.name.clone();
                    let color = get_series_color(idx, s.color.as_deref());
                    let is_hidden = hidden_series().contains(&name);
                    let display = s.display_label().to_string();
                    let state_class = if is_hidden {
                        "opacity-40 border-transparent text-zinc-500"
                    } else {
                        "opacity-100 border-white/[0.06] bg-white/[0.02] text-zinc-200 hover:border-white/[0.12]"
                    };

                    rsx! {
                        button {
                            key: "{name}",
                            r#type: "button",
                            class: "flex items-center gap-2 px-2.5 py-1 rounded-md transition-all cursor-pointer border {state_class}",
                            onclick: {
                                let n = name.clone();
                                move |_| {
                                    let mut set = hidden_series();
                                    if set.contains(&n) {
                                        set.remove(&n);
                                    } else {
                                        set.insert(n.clone());
                                    }
                                    hidden_series.set(set);
                                }
                            },
                            span {
                                class: "w-2.5 h-2.5 rounded-full shrink-0 shadow-sm",
                                style: "background-color: {color}; box-shadow: 0 0 6px {color}66;",
                            }
                            span { "{display}" }
                        }
                    }
                }
            }
        }
    }
}

/// Tooltip popover per grafico con visualizzazione coordinate e valori.
#[component]
pub fn ChartTooltip(
    title: String,
    entries: Vec<(String, String, String)>,
    x: f64,
    y: f64,
    visible: bool,
) -> Element {
    if !visible || entries.is_empty() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "sd-chart__tooltip pointer-events-none absolute z-[100] px-3 py-2 rounded-lg bg-zinc-950/90 border border-white/10 shadow-xl backdrop-blur-md text-xs font-mono transition-transform duration-75",
            style: "left: {x}px; top: {y}px; transform: translate(-50%, -110%);",
            div { class: "font-semibold text-zinc-100 mb-1 pb-1 border-b border-white/[0.08]",
                "{title}"
            }
            div { class: "flex flex-col gap-1",
                for (lbl, val, col) in entries {
                    div { key: "{lbl}", class: "flex items-center justify-between gap-4 min-w-[120px]",
                        div { class: "flex items-center gap-1.5",
                            span {
                                class: "w-2 h-2 rounded-full shrink-0",
                                style: "background-color: {col};",
                            }
                            span { class: "text-zinc-400", "{lbl}:" }
                        }
                        span { class: "font-bold text-zinc-100", "{val}" }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_builder() {
        let pt = ChartDataPoint::point("Gen", [("rev", 100.0), ("exp", 80.0)]);
        assert_eq!(pt.label, "Gen");
        assert_eq!(pt.get("rev"), Some(100.0));
        assert_eq!(pt.get("exp"), Some(80.0));
        assert_eq!(pt.get("missing"), None);
    }

    #[test]
    fn test_generate_svg_path_linear() {
        let coords = vec![(0.0, 100.0), (50.0, 50.0), (100.0, 0.0)];
        let path = generate_svg_path(&coords, CurveType::Linear);
        assert_eq!(path, "M 0.00 100.00 L 50.00 50.00 L 100.00 0.00");
    }

    #[test]
    fn test_generate_svg_area_path() {
        let coords = vec![(0.0, 100.0), (100.0, 50.0)];
        let area = generate_svg_area_path(&coords, 200.0, CurveType::Linear);
        assert_eq!(area, "M 0.00 100.00 L 100.00 50.00 L 100.00 200.00 L 0.00 200.00 Z");
    }
}
