//! Knob panel: one library control per `Control` (ADR-0006 §3). Pages never style controls by hand.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use super::knobs::Knobs;
use super::schema::{Control, Kind, Value};

/// Above this many options a choice knob is a `Select` instead of a `SegmentedControl`.
const SEGMENT_MAX: usize = 5;

/// Value knobs (choice, text, number, colour) first, in declaration order; the boolean
/// flags after them on one wrapping grid, so the panel stays short on every breakpoint.
#[component]
pub fn KnobPanel(controls: Vec<Control>, knobs: Signal<Knobs>, changed: Signal<Option<&'static str>>) -> Element {
    let (flags, values): (Vec<Control>, Vec<Control>) = controls.into_iter().partition(|c| c.kind == Kind::Bool);
    rsx! {
        div { class: "sc-knobs",
            for c in values {
                Knob { key: "{c.key}", control: c, knobs, changed }
            }
            if !flags.is_empty() {
                div { class: "sc-knob",
                    span { class: "sc-eyebrow", "Opzioni" }
                    div { class: "sc-knobs__flags",
                        for c in flags {
                            Knob { key: "{c.key}", control: c, knobs, changed }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Knob(control: Control, knobs: Signal<Knobs>, changed: Signal<Option<&'static str>>) -> Element {
    match control.kind {
        Kind::Bool => rsx! { BoolKnob { control, knobs, changed } },
        Kind::Choice(_) => rsx! { ChoiceKnob { control, knobs, changed } },
        Kind::Text => rsx! { TextKnob { control, knobs, changed } },
        Kind::Number { .. } => rsx! { NumberKnob { control, knobs, changed } },
        Kind::Color => rsx! { ColorKnob { control, knobs, changed } },
    }
}

#[component]
fn BoolKnob(control: Control, mut knobs: Signal<Knobs>, mut changed: Signal<Option<&'static str>>) -> Element {
    let key = control.key;
    let mut checked = use_signal(|| knobs.peek().bool(key));
    use_effect(move || {
        let v = knobs().bool(key);
        if *checked.peek() != v {
            checked.set(v);
        }
    });
    rsx! {
        Switch {
            checked,
            label: Some(control.label.to_string()),
            description: control.hint.map(str::to_string),
            on_change: move |v: bool| {
                knobs.write().set(key, Value::Bool(v));
                changed.set(Some(key));
            },
        }
    }
}

#[component]
fn ChoiceKnob(control: Control, mut knobs: Signal<Knobs>, mut changed: Signal<Option<&'static str>>) -> Element {
    let key = control.key;
    let options = match control.kind {
        Kind::Choice(o) => o,
        _ => &[],
    };
    let mut idx = use_signal(|| knobs.peek().index(key).to_string());
    let mut sel = use_signal(|| Some(knobs.peek().index(key).to_string()));
    use_effect(move || {
        let i = knobs().index(key).to_string();
        if *idx.peek() != i {
            idx.set(i.clone());
        }
        if sel.peek().as_deref() != Some(i.as_str()) {
            sel.set(Some(i));
        }
    });
    let mut pick = move |s: String| {
        if let Ok(i) = s.parse::<usize>() {
            knobs.write().set(key, Value::Choice(i));
            changed.set(Some(key));
        }
    };
    rsx! {
        div { class: "sc-knob",
            span { class: "sc-eyebrow", "{control.label}" }
            if options.len() > SEGMENT_MAX {
                Select {
                    data: options.iter().enumerate().map(|(i, o)| SelectItem::new(i.to_string(), o.label)).collect::<Vec<_>>(),
                    value: sel,
                    on_change: move |v: Option<String>| {
                        if let Some(v) = v {
                            pick(v);
                        }
                    },
                }
            } else {
                SegmentedControl {
                    size: SegmentedControlSize::Sm,
                    full_width: true,
                    options: options.iter().enumerate().map(|(i, o)| SegmentOption::new(i.to_string(), o.label)).collect::<Vec<_>>(),
                    value: idx,
                    on_change: move |v: String| pick(v),
                }
            }
            if let Some(h) = control.hint {
                span { class: "sc-hint", "{h}" }
            }
        }
    }
}

#[component]
fn TextKnob(control: Control, mut knobs: Signal<Knobs>, mut changed: Signal<Option<&'static str>>) -> Element {
    let key = control.key;
    let text = use_signal(|| knobs.peek().text(key));
    use_effect(move || {
        let v = text();
        if knobs.peek().text(key) != v {
            knobs.write().set(key, Value::Text(v));
            changed.set(Some(key));
        }
    });
    rsx! {
        TextInput {
            value: text,
            label: Some(control.label.to_string()),
            help_text: control.hint.map(str::to_string),
            clearable: true,
        }
    }
}

#[component]
fn NumberKnob(control: Control, mut knobs: Signal<Knobs>, mut changed: Signal<Option<&'static str>>) -> Element {
    let key = control.key;
    let (min, max, step, decimals) = match control.kind {
        Kind::Number { min, max, step, decimals } => (min, max, step, decimals),
        _ => (0.0, 100.0, 1.0, 0),
    };
    let value = use_signal(|| knobs.peek().number(key));
    let shown = format!("{:.*}", decimals as usize, value());
    rsx! {
        div { class: "sc-knob",
            div { class: "sc-knob__row",
                span { class: "sc-eyebrow", "{control.label}" }
                span { class: "sc-readout", "{shown}" }
            }
            Slider {
                value,
                min,
                max,
                step,
                show_tooltip: false,
                on_change: move |v: f64| {
                    knobs.write().set(key, Value::Number(v));
                    changed.set(Some(key));
                },
            }
            if let Some(h) = control.hint {
                span { class: "sc-hint", "{h}" }
            }
        }
    }
}

#[component]
fn ColorKnob(control: Control, mut knobs: Signal<Knobs>, mut changed: Signal<Option<&'static str>>) -> Element {
    let key = control.key;
    let value = use_signal(|| knobs.peek().color(key));
    rsx! {
        ColorInput {
            value,
            label: Some(control.label.to_string()),
            description: control.hint.map(str::to_string),
            on_change: move |v: String| {
                knobs.write().set(key, Value::Color(v));
                changed.set(Some(key));
            },
        }
    }
}
