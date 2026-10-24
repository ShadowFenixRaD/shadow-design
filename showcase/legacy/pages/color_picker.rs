use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{
    ChoicePicker, ControlRow, Counter, FeatureBanner, PageHeader, Playground, PropRow, ShowcaseTabs, ToggleControl,
};

#[component]
pub fn ColorPickerPage() -> Element {
    let picker_color = use_signal(|| "#a855f7".to_string());  // sd-lint-allow: runtime data value, not styling
    let alpha_color = use_signal(|| "rgba(168, 85, 247, 0.85)".to_string());
    let input_color = use_signal(|| "#06b6d4".to_string());  // sd-lint-allow: runtime data value, not styling
    let brand_color = use_signal(|| "#10b981".to_string());  // sd-lint-allow: runtime data value, not styling

    // Playground state
    let play_color = use_signal(|| "#a855f7".to_string());  // sd-lint-allow: runtime data value, not styling
    let play_format = use_signal(|| ColorFormat::Hex);
    let play_alpha = use_signal(|| true);
    let play_disabled = use_signal(|| false);

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

    let code_snippet = r##"let mut primary_color = use_signal(|| "#a855f7".to_string());  // sd-lint-allow: runtime data value, not styling

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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "ColorPicker & ColorInput".to_string(),
                description: "Strumenti avanzati per la selezione e regolazione cromatica. Includono slider hue a 360°, slider saturazione e luminosità HSL, canale alpha per la trasparenza, palette swatch tematica e form control ColorInput con popup popover ancorato.".to_string(),
            }

            FeatureBanner { feature_name: "color_picker" }

            ShowcaseTabs {
                props: props,
                code: code_snippet,
                preview: rsx! {
                    div { class: "sc-stack sc-stack--loose",
                        Playground {
                            stage: rsx! {
                                ColorPicker {
                                    value: play_color,
                                    format: *play_format.read(),
                                    with_alpha: *play_alpha.read(),
                                    disabled: *play_disabled.read(),
                                }
                                Counter { label: "Valore".to_string(), value: cur_play_str.to_string() }
                            },
                            controls: rsx! {
                                ChoicePicker {
                                    label: "Formato".to_string(),
                                    options: vec![
                                        (ColorFormat::Hex, "HEX".to_string()),
                                        (ColorFormat::Rgb, "RGB".to_string()),
                                        (ColorFormat::Hsl, "HSL".to_string()),
                                    ],
                                    value: play_format,
                                }
                                ControlRow {
                                    ToggleControl { label: "Canale Alpha (Trasparenza)".to_string(), checked: play_alpha }
                                    ToggleControl { label: "Disabilitato".to_string(), checked: play_disabled }
                                }
                            },
                        }

                        div { class: "sc-card-grid",
                            Card {
                                title: "ColorInput Form Control".to_string(),
                                subtitle: "Campo di testo sincronizzato con un pulsante swatch che apre il selettore colore in overlay.".to_string(),
                                ColorInput {
                                    value: input_color,
                                    label: "Colore Primario".to_string(),
                                    description: "Fai click sul riquadro colore per modificare il valore".to_string(),
                                    clearable: true,
                                }
                            }

                            Card {
                                title: "ColorInput con Trasparenza Alpha".to_string(),
                                subtitle: "Supporta valori RGBA con cursore di opacità e rendering a scacchiera.".to_string(),
                                ColorInput {
                                    value: alpha_color,
                                    label: "Sfondo Overlay".to_string(),
                                    with_alpha: true,
                                    format: ColorFormat::Rgb,
                                }
                            }

                            Card {
                                title: "Stato di Errore".to_string(),
                                subtitle: "Evidenziazione perimetrale di validazione per palette e contrasti non conformi.".to_string(),
                                ColorInput {
                                    value: brand_color,
                                    label: "Colore Non Conforme".to_string(),
                                    error: Some("Il contrasto con lo sfondo è inferiore a 4.5:1 (WCAG AA)".to_string()),
                                }
                            }

                            Card {
                                title: "Standalone ColorPicker Hex".to_string(),
                                subtitle: "Utilizzabile inline in barre degli strumenti, editor grafici e dashboard.".to_string(),
                                ColorPicker {
                                    value: picker_color,
                                    size: ControlSize::Sm,
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}
