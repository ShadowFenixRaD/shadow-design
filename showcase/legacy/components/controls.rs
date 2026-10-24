//! Reusable playground vocabulary for showcase pages.
//!
//! Everything here is built from library components and `.sc-*` classes
//! (see `showcase/assets/showcase.scss`). Pages never style controls by hand.
use dioxus::prelude::*;
use shadow_design::prelude::*;

/// Above this many options a picker becomes a `Select` instead of a segmented control.
const SEGMENT_MAX: usize = 6;

/// Picker over any `Clone + PartialEq` value (enums included).
///
/// Few options render a `SegmentedControl`, many options a `Select`.
/// `options` is `(value, label)`. Writes the chosen value into `value`.
#[component]
pub fn ChoicePicker<T: Clone + PartialEq + 'static>(
    label: String,
    options: Vec<(T, String)>,
    value: Signal<T>,
    #[props(default)] disabled: bool,
    /// Called after the user picks an option (not on external changes of `value`).
    #[props(default)] on_change: Option<EventHandler<T>>,
) -> Element {
    // Index of the current value as a string; "" when the value is not in `options`.
    let position = |opts: &[(T, String)], v: &T| opts.iter().position(|(o, _)| o == v).map(|n| n.to_string()).unwrap_or_default();

    let mut idx = use_signal(|| position(&options, &value.peek()));
    let mut sel = use_signal(|| Some(position(&options, &value.peek())).filter(|s| !s.is_empty()));

    let sync_opts = options.clone();
    use_effect(move || {
        let i = position(&sync_opts, &value());
        if *idx.peek() != i {
            idx.set(i.clone());
        }
        let s = Some(i).filter(|s| !s.is_empty());
        if *sel.peek() != s {
            sel.set(s);
        }
    });

    let pick_opts = options.clone();
    let mut pick = move |i: String| {
        if let Some((v, _)) = i.parse::<usize>().ok().and_then(|n| pick_opts.get(n)) {
            if *value.peek() != *v {
                value.set(v.clone());
            }
            if let Some(h) = &on_change {
                h.call(v.clone());
            }
        }
    };

    let many = options.len() > SEGMENT_MAX;

    rsx! {
        div { class: "sc-control",
            span { class: "sc-control-label", "{label}" }
            if many {
                Select {
                    data: options
                        .iter()
                        .enumerate()
                        .map(|(i, (_, l))| SelectItem::new(i.to_string(), l.clone()))
                        .collect::<Vec<_>>(),
                    value: sel,
                    disabled,
                    on_change: move |v: Option<String>| {
                        if let Some(v) = v {
                            pick(v);
                        }
                    },
                }
            } else {
                SegmentedControl {
                    size: SegmentedControlSize::Sm,
                    disabled,
                    options: options
                        .iter()
                        .enumerate()
                        .map(|(i, (_, l))| SegmentOption::new(i.to_string(), l.clone()))
                        .collect::<Vec<_>>(),
                    value: idx,
                    on_change: move |v: String| pick(v),
                }
            }
        }
    }
}

/// String flavour of [`ChoicePicker`]: `options` is `(value, label)`, `value` a `Signal<String>`.
#[component]
pub fn OptionPicker(
    label: String,
    options: Vec<(String, String)>,
    value: Signal<String>,
    #[props(default)] disabled: bool,
    #[props(default)] on_change: Option<EventHandler<String>>,
) -> Element {
    rsx! {
        ChoicePicker::<String> { label, options, value, disabled, on_change }
    }
}

/// Boolean toggle (library `Switch`) with a mono label, for playground flags.
#[component]
pub fn ToggleControl(
    label: String,
    checked: Signal<bool>,
    #[props(default)] description: Option<String>,
    #[props(default)] disabled: bool,
) -> Element {
    rsx! {
        Switch { checked, label: Some(label), description, disabled }
    }
}

/// Boolean checkbox (library `Checkbox`), for option lists.
#[component]
pub fn CheckControl(
    label: String,
    checked: Signal<bool>,
    #[props(default)] description: Option<String>,
    #[props(default)] disabled: bool,
) -> Element {
    rsx! {
        Checkbox { checked, label: Some(label), description, disabled }
    }
}

/// A labelled slot for any custom control (slider, number input, ...).
#[component]
pub fn ControlGroup(label: String, children: Element) -> Element {
    rsx! {
        div { class: "sc-control",
            span { class: "sc-control-label", "{label}" }
            {children}
        }
    }
}

/// Wrap several toggles on one wrapping row.
#[component]
pub fn ControlRow(children: Element) -> Element {
    rsx! {
        div { class: "sc-control-row", {children} }
    }
}

/// The right-hand control panel of a [`Playground`].
#[component]
pub fn Controls(children: Element) -> Element {
    rsx! {
        div { class: "sc-controls", {children} }
    }
}

/// Static demo stage (flat-lit, centred). Use `start: true` for left-aligned, full-width content.
#[component]
pub fn Stage(#[props(default)] start: bool, #[props(default)] wide: bool, children: Element) -> Element {
    let start = if start { "sc-stage--start" } else { "" };
    let wide = if wide { "sc-stage--wide" } else { "" };
    rsx! {
        div { class: "sc-stage {start} {wide}", {children} }
    }
}

/// Stage on the left, controls on the right (stacked on small screens).
#[component]
pub fn Playground(
    stage: Element,
    controls: Element,
    #[props(default)] stage_start: bool,
) -> Element {
    rsx! {
        div { class: "sc-playground",
            Stage { start: stage_start, {stage} }
            Controls { {controls} }
        }
    }
}

/// "Click registrati: 0" style readout.
#[component]
pub fn Counter(label: String, value: String) -> Element {
    rsx! {
        span { class: "sc-counter", "{label}: " strong { "{value}" } }
    }
}

/// STATIC placeholder tile for layout demos (Grid, Group, Stack, Masonry, Center, AspectRatio).
/// Never use a `Button` for this: a tile must not look clickable.
#[component]
pub fn DemoCell(
    #[props(default)] index: Option<usize>,
    #[props(default)] label: Option<String>,
    #[props(default)] tall: bool,
    #[props(default)] short: bool,
    #[props(default)] accent: bool,
    #[props(default)] class: String,
) -> Element {
    let tall = if tall { "sc-demo-cell--tall" } else { "" };
    let short = if short { "sc-demo-cell--short" } else { "" };
    let accent = if accent { "sc-demo-cell--accent" } else { "" };
    rsx! {
        div { class: "sc-demo-cell {tall} {short} {accent} {class}",
            if let Some(i) = index {
                span { class: "sc-demo-cell__index", "{i}" }
            }
            if let Some(l) = label {
                span { "{l}" }
            }
        }
    }
}

/// Page title and description with the shared typography.
#[component]
pub fn PageHeader(title: String, description: String) -> Element {
    rsx! {
        header { class: "sc-page-header",
            h1 { class: "sc-page-title", "{title}" }
            p { class: "sc-page-desc", "{description}" }
        }
    }
}
