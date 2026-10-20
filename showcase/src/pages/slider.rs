use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn SliderPage() -> Element {
    let volume = use_signal(|| 45.0);
    let brightness = use_signal(|| 80.0);
    let price_range = use_signal(|| (20.0, 75.0));

    let mut is_disabled = use_signal(|| false);
    let mut show_tooltip = use_signal(|| true);

    let props = vec![
        PropRow {
            name: "value",
            r#type: "Signal<f64> | Signal<(f64, f64)>",
            default: "-",
            description: "Signal reattivo collegato al valore (singolo o tupla min/max per RangeSlider)",
        },
        PropRow {
            name: "min",
            r#type: "f64",
            default: "0.0",
            description: "Valore minimo consentito della scala",
        },
        PropRow {
            name: "max",
            r#type: "f64",
            default: "100.0",
            description: "Valore massimo consentito della scala",
        },
        PropRow {
            name: "step",
            r#type: "f64",
            default: "1.0",
            description: "Passo di incremento e granularità dei punti di scatto",
        },
        PropRow {
            name: "marks",
            r#type: "Vec<SliderMark>",
            default: "vec![]",
            description: "Elenco delle tacche graduate con etichette visive posizionate lungo la traccia",
        },
        PropRow {
            name: "show_tooltip",
            r#type: "bool",
            default: "true",
            description: "Mostra un tooltip fluttuante sopra il cursore al passaggio del mouse o durante il trascinamento",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita l'interazione sul cursore",
        },
    ];

    let code_snippet = r#"let mut volume = use_signal(|| 45.0);
let mut price_range = use_signal(|| (20.0, 75.0));

// Cursore Singolo con tacche
Slider {
    label: "Volume Audio".to_string(),
    value: volume,
    min: 0.0,
    max: 100.0,
    step: 5.0,
    marks: vec![
        SliderMark::new(0.0, "Mute"),
        SliderMark::new(50.0, "50%"),
        SliderMark::new(100.0, "Max"),
    ],
}

// Cursore Doppio Range (Min..Max)
RangeSlider {
    label: "Filtro Prezzo (€)".to_string(),
    value: price_range,
    min: 0.0,
    max: 100.0,
    step: 1.0,
}"#
    .to_string();

    let marks = vec![
        SliderMark::new(0.0, "0%"),
        SliderMark::new(25.0, "25%"),
        SliderMark::new(50.0, "50%"),
        SliderMark::new(75.0, "75%"),
        SliderMark::new(100.0, "100%"),
    ];

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Slider & RangeSlider" }
                Text { dimmed: true, size: TextSize::Sm, "Cursore graduato a trascinamento con traccia evidenziata, tacche (marks), tooltip dinamico e supporto a doppio cursore RangeSlider." }
            }

            FeatureBanner { feature_name: "slider" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "grid grid-cols-1 lg:grid-cols-2 gap-8 items-start",
                        // Live Preview
                        div { class: "p-6 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-8 shadow-inner",
                            // Single Slider
                            Slider {
                                label: "Livello Volume".to_string(),
                                value: volume,
                                min: 0.0,
                                max: 100.0,
                                step: 1.0,
                                show_tooltip: show_tooltip(),
                                disabled: is_disabled(),
                            }

                            // Slider with marks
                            div { class: "pt-4",
                                Slider {
                                    label: "Luminosità Display con Tacche".to_string(),
                                    value: brightness,
                                    min: 0.0,
                                    max: 100.0,
                                    step: 5.0,
                                    marks: marks.clone(),
                                    show_tooltip: show_tooltip(),
                                    disabled: is_disabled(),
                                }
                            }

                            // Range Slider
                            div { class: "pt-6 border-t border-white/[0.06]",
                                RangeSlider {
                                    label: "Intervallo Banda Frequenze (MHz)".to_string(),
                                    value: price_range,
                                    min: 0.0,
                                    max: 100.0,
                                    step: 1.0,
                                    marks: marks.clone(),
                                    show_tooltip: show_tooltip(),
                                    disabled: is_disabled(),
                                }
                            }
                        }

                        // Configurator
                        div { class: "space-y-4 text-xs font-mono",
                            div { class: "flex flex-col gap-2",
                                label { class: "flex items-center gap-2 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: show_tooltip(),
                                        oninput: move |e| show_tooltip.set(e.value() == "true"),
                                    }
                                    "Mostra Tooltip Valore"
                                }
                                label { class: "flex items-center gap-2 cursor-pointer text-zinc-300",
                                    input {
                                        r#type: "checkbox",
                                        checked: is_disabled(),
                                        oninput: move |e| is_disabled.set(e.value() == "true"),
                                    }
                                    "Disabilitato"
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
