use dioxus::prelude::*;
use shadow_design::date_picker::Date;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

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
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "DatePicker & Calendar" }
                Text { dimmed: true, size: TextSize::Sm, "Griglia calendario mensile/annuale, input sincronizzato e selettore intervalli temporali privo di dipendenze esterne." }
            }

            FeatureBanner { feature_name: "date_picker" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-8 shadow-inner",
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-8 items-start",
                            // 1. Singolo DatePicker
                            div { class: "p-5 rounded-xl border border-white/[0.06] bg-zinc-950/40 space-y-4",
                                span { class: "text-xs font-mono text-purple-300 font-semibold uppercase tracking-wider", "Selezione Data Singola" }
                                p { class: "text-xs text-zinc-400", "Clicca sull'input per aprire il calendario con evidenziazione del giorno." }
                                DatePicker {
                                    value: selected_date,
                                    placeholder: "Scegli una data...".to_string(),
                                }
                                div { class: "text-xs font-mono p-2.5 rounded bg-white/[0.02] border border-white/[0.04] text-zinc-300",
                                    span { class: "text-zinc-500", "Data attiva: " }
                                    span { class: "text-purple-300 font-bold", "{display_single}" }
                                }
                            }

                            // 2. DateRangePicker
                            div { class: "p-5 rounded-xl border border-white/[0.06] bg-zinc-950/40 space-y-4",
                                span { class: "text-xs font-mono text-cyan-300 font-semibold uppercase tracking-wider", "Selezione Range Temporale" }
                                p { class: "text-xs text-zinc-400", "Seleziona prima la data di inizio e poi quella di fine con traccia luminosa." }
                                DateRangePicker {
                                    start_date: range_start,
                                    end_date: range_end,
                                    placeholder: "Scegli intervallo...".to_string(),
                                }
                                div { class: "text-xs font-mono p-2.5 rounded bg-white/[0.02] border border-white/[0.04] text-zinc-300",
                                    span { class: "text-zinc-500", "Range: " }
                                    span { class: "text-cyan-300 font-bold", "{display_range}" }
                                }
                            }
                        }

                        // 3. Calendar inline fisso
                        div { class: "p-6 rounded-xl border border-white/[0.06] bg-zinc-950/40 flex flex-col items-center gap-4",
                            span { class: "text-xs font-mono text-zinc-400 font-semibold uppercase", "Componente Calendar Inline Diretto" }
                            Calendar {
                                selected: *selected_date.read(),
                                on_select: move |d| *selected_date.write() = Some(d),
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
