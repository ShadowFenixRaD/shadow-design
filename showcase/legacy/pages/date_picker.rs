use dioxus::prelude::*;
use shadow_design::date_picker::Date;
use shadow_design::prelude::*;

use crate::components::{Counter, FeatureBanner, PageHeader, PropRow, ShowcaseTabs, Stage};

#[component]
pub fn DatePickerPage() -> Element {
    let mut selected_date = use_signal(|| Some(Date::new(2026, 10, 22)));
    let range_start = use_signal(|| Some(Date::new(2026, 10, 15)));
    let range_end = use_signal(|| Some(Date::new(2026, 10, 28)));

    let props = vec![
        PropRow {
            name: "value",
            r#type: "Signal<Option<Date>>",
            default: "-",
            description: "Binding reattivo bidirezionale con la data selezionata",
        },
        PropRow {
            name: "placeholder",
            r#type: "String",
            default: "\"Seleziona data...\"",
            description: "Testo segnaposto dell'input quando nessuna data è attiva",
        },
        PropRow {
            name: "clearable",
            r#type: "bool",
            default: "true",
            description: "Mostra l'icona X per azzerare rapidamente la selezione",
        },
        PropRow {
            name: "min_date / max_date",
            r#type: "Option<Date>",
            default: "None",
            description: "Limiti temporali che disabilitano le celle fuori intervallo",
        },
        PropRow {
            name: "DateRangePicker",
            r#type: "Component",
            default: "-",
            description: "Variante per la selezione continua di intervalli (Start..End)",
        },
    ];

    let code_snippet = r#"let mut date = use_signal(|| Some(Date::new(2026, 10, 22)));

// Singola Data
DatePicker {
    value: date,
    placeholder: "Scegli giorno di scadenza".to_string(),
}

// Intervallo di Date
DateRangePicker {
    start_date: range_start,
    end_date: range_end,
}"#
    .to_string();

    let display_single = match *selected_date.read() {
        Some(d) => format!("{} (Giorno della settimana: {})", d.format_it(), d.day_of_week()),
        None => "Nessuna data".to_string(),
    };

    let display_range = match (*range_start.read(), *range_end.read()) {
        (Some(s), Some(e)) => format!("{} fino al {}", s.format_it(), e.format_it()),
        (Some(s), None) => format!("Dal {} (seleziona fine)", s.format_it()),
        _ => "Nessun intervallo".to_string(),
    };

    rsx! {
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "DatePicker & Calendar".to_string(),
                description: "Griglia calendario mensile/annuale, input sincronizzato e selettore intervalli temporali privo di dipendenze esterne.".to_string(),
            }

            FeatureBanner { feature_name: "date_picker" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "sc-stack sc-stack--loose",
                        div { class: "sc-card-grid",
                            Card {
                                title: "Selezione Data Singola".to_string(),
                                subtitle: "Clicca sull'input per aprire il calendario con evidenziazione del giorno.".to_string(),
                                div { class: "sc-stack",
                                    DatePicker {
                                        value: selected_date,
                                        placeholder: "Scegli una data...".to_string(),
                                    }
                                    Counter { label: "Data attiva".to_string(), value: display_single.to_string() }
                                }
                            }

                            Card {
                                title: "Selezione Range Temporale".to_string(),
                                subtitle: "Seleziona prima la data di inizio e poi quella di fine con traccia luminosa.".to_string(),
                                div { class: "sc-stack",
                                    DateRangePicker {
                                        start_date: range_start,
                                        end_date: range_end,
                                        placeholder: "Scegli intervallo...".to_string(),
                                    }
                                    Counter { label: "Range".to_string(), value: display_range.to_string() }
                                }
                            }
                        }

                        Card { title: "Componente Calendar Inline Diretto".to_string(),
                            Stage {
                                Calendar {
                                    selected: *selected_date.read(),
                                    on_select: move |d| *selected_date.write() = Some(d),
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
