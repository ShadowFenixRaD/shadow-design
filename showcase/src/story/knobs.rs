//! Knob state and the RSX code generator (ADR-0006 §4a).
//!
//! The playground code is generated from the knob values on every render, so the
//! live component and the sample can never drift apart.
use super::schema::{Control, Emit, Kind, Playground, Value};

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Knobs {
    values: Vec<(&'static str, Value)>,
}

impl Knobs {
    pub fn from_controls(controls: &[Control]) -> Self {
        Self { values: controls.iter().map(|c| (c.key, c.default.clone())).collect() }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.iter().find(|(k, _)| *k == key).map(|(_, v)| v)
    }

    pub fn set(&mut self, key: &'static str, value: Value) {
        match self.values.iter_mut().find(|(k, _)| *k == key) {
            Some(slot) => slot.1 = value,
            None => self.values.push((key, value)),
        }
    }

    pub fn bool(&self, key: &str) -> bool {
        matches!(self.get(key), Some(Value::Bool(true)))
    }

    pub fn index(&self, key: &str) -> usize {
        match self.get(key) {
            Some(Value::Choice(i)) => *i,
            _ => 0,
        }
    }

    /// Maps a `Choice` knob to a typed table declared in the same order as its options.
    pub fn pick<T: Copy>(&self, key: &str, table: &[T]) -> T {
        table[self.index(key).min(table.len().saturating_sub(1))]
    }

    pub fn text(&self, key: &str) -> String {
        match self.get(key) {
            Some(Value::Text(s)) => s.clone(),
            _ => String::new(),
        }
    }

    pub fn number(&self, key: &str) -> f64 {
        match self.get(key) {
            Some(Value::Number(n)) => *n,
            _ => 0.0,
        }
    }

    pub fn color(&self, key: &str) -> String {
        match self.get(key) {
            Some(Value::Color(s)) => s.clone(),
            _ => String::new(),
        }
    }
}

pub struct Generated {
    pub code: String,
    /// 1-indexed lines to highlight (the knob changed last).
    pub highlight: Vec<usize>,
}

/// Raw value for `{v}` / `{key}` substitution: text unquoted, choice code, number, bool.
fn raw(value: &Value, kind: &Kind) -> String {
    match (value, kind) {
        (Value::Bool(b), _) => b.to_string(),
        (Value::Choice(i), Kind::Choice(opts)) => opts.get(*i).map(|o| o.code.to_string()).unwrap_or_default(),
        (Value::Text(s), _) | (Value::Color(s), _) => s.replace('\\', "\\\\").replace('"', "\\\""),
        (Value::Number(n), Kind::Number { decimals, .. }) => format!("{:.*}", *decimals as usize, n),
        (Value::Number(n), _) => n.to_string(),
        _ => String::new(),
    }
}

/// Literal as it appears after `key:` (text and colour quoted).
fn literal(value: &Value, kind: &Kind) -> String {
    match value {
        Value::Text(_) | Value::Color(_) => format!("\"{}\"", raw(value, kind)),
        _ => raw(value, kind),
    }
}

/// When an `Emit::Template` line is printed. The live component shows a value whenever it is
/// "on", so the sample must show it too, even at the knob's default (no drift):
/// a `Bool` when true, a `Text`/`Color` when non-empty, a `Choice`/`Number` when changed.
fn template_is_on(value: &Value, is_default: bool) -> bool {
    match value {
        Value::Bool(b) => *b,
        Value::Text(s) | Value::Color(s) => !s.is_empty(),
        Value::Choice(_) | Value::Number(_) => !is_default,
    }
}

pub fn generate(pg: &Playground, knobs: &Knobs, changed: Option<&str>) -> Generated {
    let mut lines: Vec<String> = Vec::new();
    let mut highlight = Vec::new();

    if let Some(prelude) = pg.prelude_code {
        lines.extend(prelude.lines().map(str::to_string));
        lines.push(String::new());
    }
    lines.push(format!("{} {{", pg.component));
    for fixed in pg.fixed_code {
        lines.push(format!("    {fixed}"));
    }
    for c in &pg.controls {
        let Some(v) = knobs.get(c.key) else { continue };
        let is_default = *v == c.default;
        let line = match c.emit {
            Emit::Skip => None,
            Emit::Always => Some(format!("{}: {},", c.key, literal(v, &c.kind))),
            Emit::IfChanged if is_default => None,
            Emit::IfChanged => Some(format!("{}: {},", c.key, literal(v, &c.kind))),
            Emit::Template(t) if template_is_on(v, is_default) => Some(format!("{},", t.replace("{v}", &raw(v, &c.kind)))),
            Emit::Template(_) => None,
        };
        if let Some(l) = line {
            if changed == Some(c.key) {
                highlight.push(lines.len() + 1);
            }
            lines.push(format!("    {l}"));
        }
    }
    if let Some(children) = pg.children_code {
        let marker = changed.map(|k| format!("{{{k}}}"));
        for template_line in children.lines() {
            // A children line that carries the changed knob's placeholder is highlighted too.
            if marker.as_deref().is_some_and(|m| template_line.contains(m)) {
                highlight.push(lines.len() + 1);
            }
            let mut l = template_line.to_string();
            for c in &pg.controls {
                if let Some(v) = knobs.get(c.key) {
                    l = l.replace(&format!("{{{}}}", c.key), &raw(v, &c.kind));
                }
            }
            lines.push(format!("    {l}"));
        }
    }
    lines.push("}".to_string());
    Generated { code: lines.join("\n"), highlight }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::schema::{opt, Stage};
    use dioxus::prelude::*;

    // Choice options must live in a `const` (a `&[opt(..)]` literal is not promoted to 'static).
    const VARIANT_OPTS: &[crate::story::schema::ChoiceOption] =
        &[opt("Primary", "ButtonVariant::Primary"), opt("Ghost", "ButtonVariant::Ghost")];

    fn pg() -> Playground {
        Playground {
            component: "Button",
            controls: vec![
                Control::choice("variant", "Variante", VARIANT_OPTS, 0),
                Control::bool("loading", "Loading", false),
                Control::bool("left_icon", "Icona", false).emit(Emit::Template("left_icon: Some(rsx! { Icon { kind: IconKind::Check, size: 15 } })")),
                Control::text("label", "Etichetta", "Salva").emit(Emit::Skip),
            ],
            render: |_| rsx! { "" },
            prelude_code: None,
            fixed_code: &[],
            children_code: Some("\"{label}\""),
            stage: Stage::Center,
        }
    }

    #[test]
    fn defaults_print_only_children() {
        let p = pg();
        let g = generate(&p, &Knobs::from_controls(&p.controls), None);
        assert_eq!(g.code, "Button {\n    \"Salva\"\n}");
    }

    #[test]
    fn changed_knob_is_printed_and_highlighted() {
        let p = pg();
        let mut k = Knobs::from_controls(&p.controls);
        k.set("variant", Value::Choice(1));
        k.set("left_icon", Value::Bool(true));
        let g = generate(&p, &k, Some("left_icon"));
        assert!(g.code.contains("    variant: ButtonVariant::Ghost,"));
        assert!(g.code.contains("    left_icon: Some(rsx! { Icon { kind: IconKind::Check, size: 15 } }),"));
        assert_eq!(g.highlight, vec![3]);
    }

    #[test]
    fn bool_template_on_by_default_is_printed() {
        let mut p = pg();
        p.controls[2].default = Value::Bool(true);
        let g = generate(&p, &Knobs::from_controls(&p.controls), None);
        assert!(g.code.contains("left_icon: Some("), "a default-on icon must appear in the sample: {}", g.code);
    }

    #[test]
    fn text_template_is_printed_while_non_empty() {
        let mut p = pg();
        p.controls[3].emit = Emit::Template("label: Some(\"{v}\".to_string())");
        let k = Knobs::from_controls(&p.controls);
        assert!(generate(&p, &k, None).code.contains("label: Some(\"Salva\".to_string()),"));
        let mut k2 = k.clone();
        k2.set("label", Value::Text(String::new()));
        assert!(!generate(&p, &k2, Some("label")).code.contains("label: Some"));
    }

    #[test]
    fn children_line_is_highlighted_when_its_knob_changes() {
        let p = pg();
        let mut k = Knobs::from_controls(&p.controls);
        k.set("label", Value::Text("Invia".into()));
        let g = generate(&p, &k, Some("label"));
        assert_eq!(g.highlight, vec![2]);
    }
}
