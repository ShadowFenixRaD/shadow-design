//! Story: Slider and RangeSlider (feature `slider`). Stateful: the playground goes through `SliderDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const STEPS: [f64; 4] = [1.0, 8.0, 16.0, 32.0];
const STEP_OPTS: &[ChoiceOption] = &[opt("1", "1.0"), opt("8", "8.0"), opt("16", "16.0"), opt("32", "32.0")];

const PRELUDE: &str = "let value = use_signal(|| 64.0);";

fn bandwidth_marks() -> Vec<SliderMark> {
    vec![
        SliderMark::new(0.0, "0"),
        SliderMark::new(64.0, "64"),
        SliderMark::new(128.0, "128"),
        SliderMark::new(192.0, "192"),
        SliderMark::new(256.0, "256"),
    ]
}

#[component]
fn SliderDemo(knobs: Knobs) -> Element {
    let value = use_signal(|| 64.0);
    rsx! {
        Slider {
            value,
            min: 0.0,
            max: 256.0,
            label: Some(knobs.text("label")).filter(|s| !s.is_empty()),
            step: knobs.pick("step", &STEPS),
            marks: if knobs.bool("marks") { bandwidth_marks() } else { Vec::new() },
            show_tooltip: knobs.bool("show_tooltip"),
            disabled: knobs.bool("disabled"),
            glow: knobs.bool("glow").then_some(true),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        SliderDemo { knobs: k.clone() }
    }
}

#[component]
fn Range() -> Element {
    let latency = use_signal(|| (20.0, 80.0));
    rsx! {
        RangeSlider {
            value: latency,
            min: 0.0,
            max: 200.0,
            step: 5.0,
            label: Some("Latenza accettata (ms)".to_string()),
            marks: vec![SliderMark::new(0.0, "0"), SliderMark::new(100.0, "100"), SliderMark::new(200.0, "200")],
        }
    }
}
const RANGE_CODE: &str = r#"let latency = use_signal(|| (20.0, 80.0));

RangeSlider {
    value: latency,
    min: 0.0,
    max: 200.0,
    step: 5.0,
    label: Some("Latenza accettata (ms)".to_string()),
    marks: vec![SliderMark::new(0.0, "0"), SliderMark::new(100.0, "100"), SliderMark::new(200.0, "200")],
}"#;

#[component]
fn Steps() -> Element {
    let replicas = use_signal(|| 3.0);
    let retention = use_signal(|| 30.0);
    rsx! {
        Stack { gap: SpacingToken::Lg,
            Slider {
                value: replicas,
                min: 1.0,
                max: 5.0,
                label: Some("Repliche".to_string()),
                marks: vec![
                    SliderMark::point(1.0),
                    SliderMark::point(2.0),
                    SliderMark::point(3.0),
                    SliderMark::point(4.0),
                    SliderMark::point(5.0),
                ],
            }
            Slider { value: retention, min: 7.0, max: 90.0, label: Some("Conservazione log (giorni)".to_string()), disabled: true }
        }
    }
}
const STEPS_CODE: &str = r#"let replicas = use_signal(|| 3.0);
let retention = use_signal(|| 30.0);

Stack { gap: SpacingToken::Lg,
    Slider {
        value: replicas,
        min: 1.0,
        max: 5.0,
        label: Some("Repliche".to_string()),
        marks: vec![
            SliderMark::point(1.0),
            SliderMark::point(2.0),
            SliderMark::point(3.0),
            SliderMark::point(4.0),
            SliderMark::point(5.0),
        ],
    }
    Slider { value: retention, min: 7.0, max: 90.0, label: Some("Conservazione log (giorni)".to_string()), disabled: true }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        crate::scenes::SettingsForm { compact: false }
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/settings_form.rs (estratto): la banda massima nel Fieldset "Rete".
let bandwidth = use_signal(|| 64.0);

Slider { value: bandwidth, min: 8.0, max: 256.0, step: 8.0, label: Some("Banda massima (Mbit/s)".to_string()) }"#;

const SLIDER_PROPS: &[PropDoc] = &[
    prop("value", "Signal<f64>", "-", "Valore corrente, allineato a step e limitato a min..max."),
    prop("min / max", "f64", "0.0 / 100.0", "Estremi della traccia."),
    prop("step", "f64", "1.0", "Passo di trascinamento e frecce."),
    prop("marks", "Vec<SliderMark>", "[]", "Tacche: SliderMark::new(v, etichetta) o SliderMark::point(v)."),
    prop("show_tooltip", "bool", "true", "Tooltip con il valore sul cursore durante l'interazione."),
    prop("label", "Option<String>", "None", "Etichetta con il valore corrente a destra."),
    prop("disabled", "bool", "false", "Traccia e cursore inattivi."),
    prop("glow", "Option<bool>", "None", "Forza o rimuove il glow del riempimento."),
    prop("on_change", "Option<EventHandler<f64>>", "None", "Nuovo valore."),
    prop("id / class", "Option<String> / String", "None / \"\"", "id dell'input range e classi aggiuntive."),
];
const RANGE_PROPS: &[PropDoc] = &[
    prop("value", "Signal<(f64, f64)>", "-", "Estremi inferiore e superiore dell'intervallo."),
    prop("min / max / step", "f64", "0.0 / 100.0 / 1.0", "Come Slider."),
    prop("marks / show_tooltip / label / disabled / glow", "-", "-", "Come Slider."),
    prop("on_change", "Option<EventHandler<(f64, f64)>>", "None", "Nuovo intervallo."),
];

pub fn story() -> Story {
    Story {
        slug: "slider",
        name: "Slider",
        category: Category::Form,
        features: &["slider"],
        icon: IconKind::SignalHigh,
        tagline: "Scelta di un valore o di un intervallo lungo una traccia, con tacche e tooltip.",
        keywords: &["slider", "cursore", "range", "intervallo", "volume", "soglia"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "Slider",
            controls: vec![
                Control::text("label", "Etichetta", "Banda massima (Mbit/s)")
                    .emit(Emit::Template("label: Some(\"{v}\".to_string())")),
                Control::choice("step", "Passo", STEP_OPTS, 1).emit(Emit::Always),
                Control::bool("marks", "Tacche", true).emit(Emit::Template(
                    "marks: vec![SliderMark::new(0.0, \"0\"), SliderMark::new(64.0, \"64\"), SliderMark::new(128.0, \"128\"), SliderMark::new(192.0, \"192\"), SliderMark::new(256.0, \"256\")]",
                )),
                Control::bool("show_tooltip", "Tooltip", true),
                Control::bool("disabled", "Disabilitato", false),
                Control::bool("glow", "Glow esplicito", false).emit(Emit::Template("glow: Some(true)")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["value,", "min: 0.0,", "max: 256.0,"],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "range",
                title: "RangeSlider",
                description: "Due cursori per un intervallo minimo e massimo.",
                code: RANGE_CODE,
                render: || rsx! { Range {} },
                span: Span::Half,
            },
            Example {
                id: "steps",
                title: "Valori discreti e disabilitato",
                description: "Tacche senza etichetta su ogni passo intero.",
                code: STEPS_CODE,
                render: || rsx! { Steps {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "La banda massima del nodo nel form Impostazioni.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "Slider", rows: SLIDER_PROPS },
            PropGroup { component: "RangeSlider", rows: RANGE_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Value fill (language.md §3.11)",
            depth: "Well (traccia) · value fill (riempimento)",
            tokens: &["--sd-well", "--sd-border-subtle", "--sd-accent", "--sd-glow-sm", "--sd-radius-full", "--sd-font-mono"],
            notes: "La traccia è un pozzetto, mai bianca; il riempimento accento con --sd-glow-sm è l'unico glow del controllo.",
        }),
        a11y: &[
            "Input range nativo: frecce, Pagina su/giù, Home e Fine cambiano il valore.",
            "aria-valuemin, aria-valuemax e aria-valuenow riflettono lo stato.",
            "Senza label usa il testo di Locale (slider_label); RangeSlider distingue Min e Max nei due aria-label.",
            "Il cursore disegnato e il tooltip sono decorativi: lo stato lo annuncia l'input.",
        ],
        related: &["number-input", "rating", "progress", "form"],
    }
}
