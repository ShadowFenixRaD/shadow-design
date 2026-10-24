//! Story: DatePicker, DateRangePicker and Calendar (feature `date_picker`). Stateful: the
//! playground goes through `DateDemo { knobs }`.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

const PRELUDE: &str = "let value = use_signal(|| Some(Date::new(2026, 10, 24)));";

#[component]
fn DateDemo(knobs: Knobs) -> Element {
    let value = use_signal(|| Some(Date::new(2026, 10, 24)));
    rsx! {
        DatePicker {
            value,
            placeholder: knobs.text("placeholder"),
            clearable: knobs.bool("clearable"),
            disabled: knobs.bool("disabled"),
            min_date: knobs.bool("min_date").then(|| Date::new(2026, 10, 20)),
            max_date: knobs.bool("max_date").then(|| Date::new(2026, 12, 31)),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        DateDemo { knobs: k.clone() }
    }
}

#[component]
fn Range() -> Element {
    let start = use_signal(|| Some(Date::new(2026, 11, 2)));
    let end = use_signal(|| Some(Date::new(2026, 11, 6)));
    rsx! {
        DateRangePicker { start_date: start, end_date: end, placeholder: "Inizio — fine".to_string() }
    }
}
const RANGE_CODE: &str = r#"let start = use_signal(|| Some(Date::new(2026, 11, 2)));
let end = use_signal(|| Some(Date::new(2026, 11, 6)));

DateRangePicker { start_date: start, end_date: end, placeholder: "Inizio — fine".to_string() }"#;

#[component]
fn Inline() -> Element {
    let mut day = use_signal(|| Some(Date::new(2026, 10, 24)));
    rsx! {
        Calendar {
            selected: day(),
            min_date: Some(Date::new(2026, 10, 1)),
            on_select: move |d| day.set(Some(d)),
        }
    }
}
const INLINE_CODE: &str = r#"let mut day = use_signal(|| Some(Date::new(2026, 10, 24)));

Calendar {
    selected: day(),
    min_date: Some(Date::new(2026, 10, 1)),
    on_select: move |d| day.set(Some(d)),
}"#;

#[component]
fn InContext() -> Element {
    let day = use_signal(|| None::<Date>);
    let window = use_signal(|| "night".to_string());
    let reason = use_signal(|| "Rotazione chiavi Kyber".to_string());
    let mut toast = use_notification();
    rsx! {
        Card {
            title: Some("Pianifica manutenzione".to_string()),
            subtitle: Some("edge-07 · Milano".to_string()),
            footer: Some(rsx! {
                Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
                    Button { variant: ButtonVariant::Ghost, "Annulla" }
                    Button {
                        disabled: day().is_none(),
                        left_icon: Some(rsx! {
                            Icon { kind: IconKind::Calendar, size: 15 }
                        }),
                        onclick: move |_| {
                            if let Some(d) = day() {
                                toast.success(format!("Manutenzione fissata il {}", d.format_it()));
                            }
                        },
                        "Pianifica"
                    }
                }
            }),
            Stack { gap: SpacingToken::Md,
                DatePicker {
                    value: day,
                    placeholder: "Scegli il giorno".to_string(),
                    min_date: Some(Date::new(2026, 10, 24)),
                }
                SegmentedControl {
                    options: vec![SegmentOption::new("now", "Subito"), SegmentOption::new("night", "Notte"), SegmentOption::new("weekend", "Weekend")],
                    value: window,
                    full_width: true,
                }
                TextInput { value: reason, label: Some("Motivo".to_string()) }
            }
        }
    }
}
const IN_CONTEXT_CODE: &str = r#"let day = use_signal(|| None::<Date>);
let window = use_signal(|| "night".to_string());
let reason = use_signal(|| "Rotazione chiavi Kyber".to_string());
let mut toast = use_notification();

Card {
    title: Some("Pianifica manutenzione".to_string()),
    subtitle: Some("edge-07 · Milano".to_string()),
    footer: Some(rsx! {
        Group { gap: SpacingToken::Sm, justify: JustifyContent::End,
            Button { variant: ButtonVariant::Ghost, "Annulla" }
            Button {
                disabled: day().is_none(),
                left_icon: Some(rsx! {
                    Icon { kind: IconKind::Calendar, size: 15 }
                }),
                onclick: move |_| {
                    if let Some(d) = day() {
                        toast.success(format!("Manutenzione fissata il {}", d.format_it()));
                    }
                },
                "Pianifica"
            }
        }
    }),
    Stack { gap: SpacingToken::Md,
        DatePicker {
            value: day,
            placeholder: "Scegli il giorno".to_string(),
            min_date: Some(Date::new(2026, 10, 24)),
        }
        SegmentedControl {
            options: vec![SegmentOption::new("now", "Subito"), SegmentOption::new("night", "Notte"), SegmentOption::new("weekend", "Weekend")],
            value: window,
            full_width: true,
        }
        TextInput { value: reason, label: Some("Motivo".to_string()) }
    }
}"#;

const PICKER_PROPS: &[PropDoc] = &[
    prop("value", "Signal<Option<Date>>", "-", "Data scelta; None quando vuoto."),
    prop("placeholder", "String", "\"\"", "Segnaposto; da Locale (date_placeholder) se vuoto."),
    prop("clearable", "bool", "true", "Pulsante per svuotare la data."),
    prop("disabled", "bool", "false", "Campo inattivo, il calendario non si apre."),
    prop("min_date / max_date", "Option<Date>", "None", "Giorni fuori intervallo disabilitati."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const RANGE_PROPS: &[PropDoc] = &[
    prop("start_date / end_date", "Signal<Option<Date>>", "-", "Estremi dell'intervallo; il primo clic sceglie l'inizio, il secondo la fine."),
    prop("placeholder", "String", "\"\"", "Segnaposto; da Locale (date_range_placeholder) se vuoto."),
    prop("disabled", "bool", "false", "Campo inattivo."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const CALENDAR_PROPS: &[PropDoc] = &[
    prop("selected", "Option<Date>", "None", "Giorno evidenziato."),
    prop("range", "Option<(Date, Date)>", "None", "Intervallo evidenziato."),
    prop("view_date", "Option<Date>", "None", "Mese mostrato all'apertura."),
    prop("min_date / max_date", "Option<Date>", "None", "Limiti selezionabili."),
    prop("on_select", "Option<EventHandler<Date>>", "None", "Giorno cliccato."),
    prop("class", "String", "\"\"", "Classi aggiuntive."),
];
const DATE_PROPS: &[PropDoc] = &[
    prop("Date::new(year, month, day)", "fn", "-", "Costruttore; month e day partono da 1."),
    prop(".format_it()", "fn -> String", "-", "Formato italiano gg/mm/aaaa."),
    prop(".format_iso() / Date::parse_iso(..)", "fn", "-", "Conversione da e verso aaaa-mm-gg."),
];

pub fn story() -> Story {
    Story {
        slug: "date-picker",
        name: "DatePicker",
        category: Category::Form,
        features: &["date_picker"],
        icon: IconKind::Calendar,
        tagline: "Scelta di una data o di un intervallo da un calendario, con limiti minimo e massimo.",
        keywords: &["date picker", "data", "calendario", "intervallo", "range", "scadenza"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "DatePicker",
            controls: vec![
                Control::text("placeholder", "Placeholder", "Scegli una data")
                    .emit(Emit::Template("placeholder: \"{v}\".to_string()")),
                Control::bool("clearable", "Azzerabile", true),
                Control::bool("disabled", "Disabilitato", false),
                Control::bool("min_date", "Dal 20 ottobre", false).emit(Emit::Template("min_date: Some(Date::new(2026, 10, 20))")),
                Control::bool("max_date", "Fino a fine anno", false)
                    .emit(Emit::Template("max_date: Some(Date::new(2026, 12, 31))")),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["value,"],
            children_code: None,
            stage: Stage::Form,
        }),
        examples: vec![
            Example {
                id: "range",
                title: "DateRangePicker",
                description: "Inizio e fine su due signal; l'intervallo è evidenziato.",
                code: RANGE_CODE,
                render: || rsx! { Range {} },
                span: Span::Half,
            },
            Example {
                id: "inline",
                title: "Calendar inline",
                description: "Il calendario senza campo, con i giorni passati disabilitati.",
                code: INLINE_CODE,
                render: || rsx! { Inline {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "Pianificare una finestra di manutenzione: l'azione si attiva dopo la data.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "DatePicker", rows: PICKER_PROPS },
            PropGroup { component: "DateRangePicker", rows: RANGE_PROPS },
            PropGroup { component: "Calendar", rows: CALENDAR_PROPS },
            PropGroup { component: "Date", rows: DATE_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Recessed field (§3.6) + floating panel (§3.2) + selected (§3.7)",
            depth: "Well (campo) · Raised (calendario)",
            tokens: &["--sd-well", "--sd-border-strong", "--sd-surface-2", "--sd-elev-2", "--sd-radius-lg", "--sd-accent-soft", "--sd-accent-border"],
            notes: "Il giorno scelto usa sd-selected; l'intervallo è una tinta accento continua tra i due estremi.",
        }),
        a11y: &[
            "Il campo è un input di sola lettura: il valore viene annunciato come testo (formato gg/mm/aaaa).",
            "I giorni sono <button> nativi; quelli fuori da min_date / max_date sono disabled.",
            "Esc chiude il calendario; mese precedente e successivo hanno aria-label da Locale.",
            "Il pulsante di cancellazione usa il testo di Locale (clear_text) come aria-label.",
        ],
        related: &["inputs", "select", "timelines", "popover"],
    }
}
