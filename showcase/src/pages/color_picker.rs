use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn ColorPickerPage() -> Element {
    let picker_color = use_signal(|| "#a855f7".to_string());
    let alpha_color = use_signal(|| "rgba(168, 85, 247, 0.85)".to_string());
    let input_color = use_signal(|| "#06b6d4".to_string());
    let brand_color = use_signal(|| "#10b981".to_string());

    // Playground state
    let play_color = use_signal(|| "#a855f7".to_string());
    let mut play_format = use_signal(|| ColorFormat::Hex);
    let mut play_alpha = use_signal(|| true);
    let mut play_disabled = use_signal(|| false);

    let props = vec![
        PropRow {
            name: "value",
            r#type: "Signal<String>",
            default: "-",
            description: "Signal reattivo che governa e riflette il valore cromatico in formato stringa",
        },
        PropRow {
            name: "format",
            r#type: "ColorFormat",
            default: "ColorFormat::Hex",
            description: "Formato di serializzazione e visualizzazione del colore: Hex, Rgb o Hsl",
        },
        PropRow {
            name: "with_alpha",
            r#type: "bool",
            default: "false",
            description: "Abilita il canale di trasparenza (slider alpha e formati rgba/hsla)",
        },
        PropRow {
            name: "swatches",
            r#type: "Vec<String>",
            default: "default_color_swatches()",
            description: "Lista di campioni colore per la selezione rapida posizionati sotto al picker",
        },
        PropRow {
            name: "size",
            r#type: "ControlSize",
            default: "ControlSize::Md",
            description: "Scala dimensionale del picker (Sm, Md, Lg)",
        },
        PropRow {
            name: "disabled",
            r#type: "bool",
            default: "false",
            description: "Disabilita tutti i controlli slider e la palette di selezione",
        },
        PropRow {
            name: "on_change",
            r#type: "Option<EventHandler<String>>",
            default: "None",
            description: "Callback invocato ad ogni variazione del colore selezionato",
        },
    ];

    let code_snippet = r##"let mut primary_color = use_signal(|| "#a855f7".to_string());

// Standalone ColorPicker
ColorPicker {
    value: primary_color,
    format: ColorFormat::Hex,
    with_alpha: true,
}

// Form Control ColorInput con Popover
ColorInput {
    value: primary_color,
    label: "Colore Accento Brand".to_string(),
    description: "Definisce il glow e i riflessi dell'interfaccia utente".to_string(),
    with_alpha: true,
    clearable: true,
}"##
    .to_string();

    let cur_play_str = play_color.read().clone();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "ColorPicker & ColorInput" }
                Text { dimmed: true, size: TextSize::Sm,
                    "Strumenti avanzati per la selezione e regolazione cromatica. Includono slider hue arcobaleno a 360°, "
                    "slider saturazione e luminosità HSL, canale alpha per la trasparenza, palette swatch tematica e "
                    "form control ColorInput con popup popover ancorato."
                }
            }

            FeatureBanner { feature_name: "color-picker" }

            ShowcaseTabs {
                props: props,
                code: code_snippet,
                preview: rsx! {
                    div { class: "space-y-8",
                        // Live Playground
                        Card {
                            div { class: "p-6 space-y-6",
                                Heading { order: HeadingOrder::H3, "Live Playground" }

                                div { class: "flex flex-wrap gap-4 items-center p-4 bg-zinc-950/60 rounded-lg border border-white/5",
                                    label { class: "flex items-center gap-2 text-sm text-zinc-300",
                                        "Formato:"
                                        select {
                                            class: "bg-zinc-900 text-zinc-100 rounded px-2 py-1 text-sm border border-white/10",
                                            value: match *play_format.read() {
                                                ColorFormat::Hex => "hex",
                                                ColorFormat::Rgb => "rgb",
                                                ColorFormat::Hsl => "hsl",
                                            },
                                            onchange: move |e| {
                                                match e.value().as_str() {
                                                    "rgb" => play_format.set(ColorFormat::Rgb),
                                                    "hsl" => play_format.set(ColorFormat::Hsl),
                                                    _ => play_format.set(ColorFormat::Hex),
                                                }
                                            },
                                            option { value: "hex", "HEX (#RRGGBB)" }
                                            option { value: "rgb", "RGB (rgb/rgba)" }
                                            option { value: "hsl", "HSL (hsl/hsla)" }
                                        }
                                    }

                                    label { class: "flex items-center gap-2 text-sm text-zinc-300 cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: *play_alpha.read(),
                                            onchange: move |e| play_alpha.set(e.checked()),
                                        }
                                        "Canale Alpha (Trasparenza)"
                                    }

                                    label { class: "flex items-center gap-2 text-sm text-zinc-300 cursor-pointer",
                                        input {
                                            r#type: "checkbox",
                                            checked: *play_disabled.read(),
                                            onchange: move |e| play_disabled.set(e.checked()),
                                        }
                                        "Disabilitato"
                                    }
                                }

                                div { class: "grid grid-cols-1 md:grid-cols-2 gap-8 items-center justify-items-center p-6 bg-black/40 rounded-xl border border-white/10",
                                    ColorPicker {
                                        value: play_color,
                                        format: *play_format.read(),
                                        with_alpha: *play_alpha.read(),
                                        disabled: *play_disabled.read(),
                                    }

                                    div { class: "flex flex-col items-center gap-4 w-full max-w-xs",
                                        div {
                                            class: "w-full h-32 rounded-xl border border-white/15 shadow-2xl flex items-center justify-center transition-all",
                                            style: "background-color: {cur_play_str}; box-shadow: 0 0 30px {cur_play_str};",
                                            span { class: "px-3 py-1.5 rounded bg-black/70 backdrop-blur text-white font-mono text-sm font-bold border border-white/20",
                                                "{cur_play_str}"
                                            }
                                        }
                                        div { class: "text-xs text-zinc-400 font-mono text-center",
                                            "Aggiornamento live in tempo reale"
                                        }
                                    }
                                }
                            }
                        }

                        // ColorInput Showcase
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                            Card {
                                div { class: "p-6 space-y-4",
                                    Heading { order: HeadingOrder::H4, "ColorInput Form Control" }
                                    Text { dimmed: true, size: TextSize::Sm,
                                        "Campo di testo sincronizzato con un pulsante swatch che apre il selettore colore in overlay."
                                    }
                                    div { class: "py-2",
                                        ColorInput {
                                            value: input_color,
                                            label: "Colore Primario".to_string(),
                                            description: "Fai click sul riquadro colore per modificare il valore".to_string(),
                                            clearable: true,
                                        }
                                    }
                                }
                            }

                            Card {
                                div { class: "p-6 space-y-4",
                                    Heading { order: HeadingOrder::H4, "ColorInput con Trasparenza Alpha" }
                                    Text { dimmed: true, size: TextSize::Sm,
                                        "Supporta valori RGBA con cursore di opacità e rendering a scacchiera."
                                    }
                                    div { class: "py-2",
                                        ColorInput {
                                            value: alpha_color,
                                            label: "Sfondo Overlay".to_string(),
                                            with_alpha: true,
                                            format: ColorFormat::Rgb,
                                        }
                                    }
                                }
                            }

                            Card {
                                div { class: "p-6 space-y-4",
                                    Heading { order: HeadingOrder::H4, "Stato di Errore" }
                                    Text { dimmed: true, size: TextSize::Sm,
                                        "Evidenziazione perimetrale di validazione per palette e contrasti non conformi."
                                    }
                                    div { class: "py-2",
                                        ColorInput {
                                            value: brand_color,
                                            label: "Colore Non Conforme".to_string(),
                                            error: Some("Il contrasto con lo sfondo è inferiore a 4.5:1 (WCAG AA)".to_string()),
                                        }
                                    }
                                }
                            }

                            Card {
                                div { class: "p-6 space-y-4",
                                    Heading { order: HeadingOrder::H4, "Standalone ColorPicker Hex" }
                                    Text { dimmed: true, size: TextSize::Sm,
                                        "Utilizzabile inline in barre degli strumenti, editor grafici e dashboard."
                                    }
                                    div { class: "flex justify-center py-2",
                                        ColorPicker {
                                            value: picker_color,
                                            size: ControlSize::Sm,
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
