use crate::icon::{Icon, IconKind};
#[cfg(feature = "theme")]
use crate::theme::use_theme;
use crate::button::{Button, ButtonSize, ButtonVariant};
use dioxus::prelude::*;

/// Rappresentazione pura, leggera e priva di dipendenze esterne per date di calendario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    /// Anno solare (es. 2026).
    pub year: i32,
    /// Mese (1..=12).
    pub month: u32,
    /// Giorno del mese (1..=31).
    pub day: u32,
}

impl Date {
    /// Crea una nuova data validandone i limiti del mese.
    #[must_use]
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        let m = month.clamp(1, 12);
        let max_d = Self::days_in_month(year, m);
        let d = day.clamp(1, max_d);
        Self { year, month: m, day: d }
    }

    /// Ritorna true se l'anno è bisestile.
    #[must_use]
    pub const fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Calcola i giorni totali di un dato mese e anno.
    #[must_use]
    pub const fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    /// Calcola il giorno della settimana (0 = Lunedì, ..., 6 = Domenica).
    /// Utilizza l'algoritmo congruente di Tomohiko Sakamoto.
    #[must_use]
    pub fn day_of_week(&self) -> u32 {
        let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let mut y = self.year;
        let m = self.month.clamp(1, 12);
        if m < 3 {
            y -= 1;
        }
        let dow_sun_zero = (y + y / 4 - y / 100 + y / 400 + t[(m - 1) as usize] + self.day as i32).rem_euclid(7);
        // Converti da Domenica=0 a Lunedì=0 ... Domenica=6
        let res = (dow_sun_zero + 6).rem_euclid(7);
        res as u32
    }

    /// Formatta la data in stringa ISO `YYYY-MM-DD`.
    #[must_use]
    pub fn format_iso(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Formatta la data in standard italiano `DD/MM/YYYY`.
    #[must_use]
    pub fn format_it(&self) -> String {
        format!("{:02}/{:02}/{:04}", self.day, self.month, self.year)
    }

    /// Prova a parsare una stringa ISO `YYYY-MM-DD`.
    #[must_use]
    pub fn parse_iso(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.trim().split('-').collect();
        if parts.len() != 3 {
            return None;
        }
        let y: i32 = parts[0].parse().ok()?;
        let m: u32 = parts[1].parse().ok()?;
        let d: u32 = parts[2].parse().ok()?;
        if (1..=12).contains(&m) && (1..=Self::days_in_month(y, m)).contains(&d) {
            Some(Self { year: y, month: m, day: d })
        } else {
            None
        }
    }

    /// Ritorna il mese precedente.
    #[must_use]
    pub fn prev_month(&self) -> Self {
        if self.month == 1 {
            Self::new(self.year - 1, 12, 1)
        } else {
            Self::new(self.year, self.month - 1, 1)
        }
    }

    /// Ritorna il mese successivo.
    #[must_use]
    pub fn next_month(&self) -> Self {
        if self.month == 12 {
            Self::new(self.year + 1, 1, 1)
        } else {
            Self::new(self.year, self.month + 1, 1)
        }
    }
}

/// Modalità di navigazione della vista del calendario.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CalendarViewMode {
    /// Visualizzazione dei giorni del mese (predefinita).
    #[default]
    Day,
    /// Visualizzazione e selezione dei 12 mesi dell'anno.
    Month,
    /// Visualizzazione e selezione dell'anno in finestre da 12 anni.
    Year,
}

/// Griglia calendario mensile interattiva con navigazione e selezione data o range.
#[component]
pub fn Calendar(
    /// Mese e anno visualizzati (default: data selezionata o 2026-10-01).
    #[props(default)]
    view_date: Option<Date>,
    /// Data attualmente selezionata (modalità singola).
    #[props(default)]
    selected: Option<Date>,
    /// Intervallo attualmente selezionato (modalità range: start, end).
    #[props(default)]
    range: Option<(Date, Date)>,
    /// Callback invocato alla selezione di un giorno.
    #[props(default)]
    on_select: Option<EventHandler<Date>>,
    /// Data minima selezionabile.
    #[props(default)]
    min_date: Option<Date>,
    /// Data massima selezionabile.
    #[props(default)]
    max_date: Option<Date>,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let initial_view = view_date
        .or(selected)
        .or_else(|| range.map(|(s, _)| s))
        .unwrap_or(Date::new(2026, 10, 1));

    let mut current_view = use_signal(|| initial_view);

    let view = *current_view.read();
    let year = view.year;
    let month = view.month;

    let days_in_cur_month = Date::days_in_month(year, month);
    let first_day_of_month = Date::new(year, month, 1);
    let start_dow = first_day_of_month.day_of_week(); // 0 = Lunedì

    let prev = view.prev_month();
    let days_in_prev_month = Date::days_in_month(prev.year, prev.month);

    let mut view_mode = use_signal(|| CalendarViewMode::Day);
    let mut year_range_start = use_signal(|| (year / 12) * 12);

    let handle_prev = move |_| {
        match *view_mode.read() {
            CalendarViewMode::Day => {
                let prev_val = current_view.read().prev_month();
                current_view.set(prev_val);
            }
            CalendarViewMode::Month => {
                let cur = *current_view.read();
                current_view.set(Date::new(cur.year.saturating_sub(1), cur.month, 1.min(cur.day)));
            }
            CalendarViewMode::Year => {
                let start = *year_range_start.read();
                year_range_start.set(start.saturating_sub(12));
            }
        }
    };

    let handle_next = move |_| {
        match *view_mode.read() {
            CalendarViewMode::Day => {
                let next_val = current_view.read().next_month();
                current_view.set(next_val);
            }
            CalendarViewMode::Month => {
                let cur = *current_view.read();
                current_view.set(Date::new(cur.year + 1, cur.month, 1.min(cur.day)));
            }
            CalendarViewMode::Year => {
                let start = *year_range_start.read();
                year_range_start.set(start + 12);
            }
        }
    };

    let is_disabled = move |d: Date| -> bool {
        if let Some(min) = min_date {
            if d < min {
                return true;
            }
        }
        if let Some(max) = max_date {
            if d > max {
                return true;
            }
        }
        false
    };

    let is_in_range = move |d: Date| -> bool {
        if let Some((s, e)) = range {
            let (start, end) = if s <= e { (s, e) } else { (e, s) };
            d >= start && d <= end
        } else {
            false
        }
    };

    let is_range_start = move |d: Date| -> bool {
        if let Some((s, _)) = range {
            d == s
        } else {
            false
        }
    };

    let is_range_end = move |d: Date| -> bool {
        if let Some((_, e)) = range {
            d == e
        } else {
            false
        }
    };

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    rsx! {
        div { class: "sd-calendar {class}",
            // Header del calendario (controlli e titoli condizionali in base a view_mode)
            div { class: "sd-calendar-header",
                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    class: "sd-calendar-nav-btn".to_string(),
                    aria_label: locale.previous.clone(),
                    onclick: handle_prev,
                    Icon { kind: IconKind::ChevronLeft, size: 16 }
                }

                match *view_mode.read() {
                    CalendarViewMode::Day => rsx! {
                        div { class: "sd-calendar-title-buttons",
                            button {
                                r#type: "button",
                                class: "sd-calendar-title-btn",
                                onclick: move |_| view_mode.set(CalendarViewMode::Month),
                                "{locale.month_name(month)}"
                            }
                            button {
                                r#type: "button",
                                class: "sd-calendar-title-btn",
                                onclick: move |_| {
                                    year_range_start.set((year / 12) * 12);
                                    view_mode.set(CalendarViewMode::Year);
                                },
                                "{year}"
                            }
                        }
                    },
                    CalendarViewMode::Month => rsx! {
                        button {
                            r#type: "button",
                            class: "sd-calendar-title-btn",
                            onclick: move |_| {
                                year_range_start.set((year / 12) * 12);
                                view_mode.set(CalendarViewMode::Year);
                            },
                            "{year}"
                        }
                    },
                    CalendarViewMode::Year => rsx! {
                        span { class: "sd-calendar-title",
                            "{*year_range_start.read()} - {*year_range_start.read() + 11}"
                        }
                    },
                }

                Button {
                    variant: ButtonVariant::Secondary,
                    size: ButtonSize::Sm,
                    class: "sd-calendar-nav-btn".to_string(),
                    aria_label: locale.next.clone(),
                    onclick: handle_next,
                    Icon { kind: IconKind::ChevronRight, size: 16 }
                }
            }

            // Corpo del calendario condizionale
            match *view_mode.read() {
                CalendarViewMode::Day => rsx! {
                    // Intestazione giorni della settimana
                    div { class: "sd-calendar-weekdays",
                        for dow in locale.weekdays_short.iter() {
                            div { class: "sd-calendar-weekday", "{dow}" }
                        }
                    }

                    // Griglia dei giorni
                    div { class: "sd-calendar-grid",
                        // Giorni del mese precedente (per allineamento)
                        for i in 0..start_dow {
                            {
                                let day_num = days_in_prev_month - start_dow + i + 1;
                                rsx! {
                                    div { class: "sd-calendar-outside", "{day_num}" }
                                }
                            }
                        }

                        // Giorni del mese corrente
                        for day_num in 1..=days_in_cur_month {
                            {
                                let cell_date = Date::new(year, month, day_num);
                                let disabled = is_disabled(cell_date);
                                let is_sel = selected == Some(cell_date);
                                let in_rng = is_in_range(cell_date);
                                let rng_start = is_range_start(cell_date);
                                let rng_end = is_range_end(cell_date);

                                let mut day_class = String::from("sd-calendar-day");
                                if is_sel || rng_start || rng_end {
                                    day_class.push_str(" sd-calendar-day--selected");
                                } else if in_rng {
                                    day_class.push_str(" sd-calendar-day--in-range");
                                }

                                rsx! {
                                    button {
                                        key: "{day_num}",
                                        r#type: "button",
                                        disabled: disabled,
                                        class: "{day_class}",
                                        onclick: move |_| {
                                            if !disabled {
                                                if let Some(ref handler) = on_select {
                                                    handler.call(cell_date);
                                                }
                                            }
                                        },
                                        "{day_num}"
                                    }
                                }
                            }
                        }
                    }
                },
                CalendarViewMode::Month => rsx! {
                    div { class: "sd-calendar-grid--items",
                        for m in 1..=12 {
                            {
                                let is_cur_month = m == month;
                                let btn_cls = if is_cur_month { "sd-calendar-item-btn sd-calendar-item-btn--selected" } else { "sd-calendar-item-btn" };
                                rsx! {
                                    button {
                                        key: "{m}",
                                        r#type: "button",
                                        class: "{btn_cls}",
                                        onclick: move |_| {
                                            let cur = *current_view.read();
                                            current_view.set(Date::new(cur.year, m, 1.min(cur.day)));
                                            view_mode.set(CalendarViewMode::Day);
                                        },
                                        "{locale.month_name(m)}"
                                    }
                                }
                            }
                        }
                    }
                },
                CalendarViewMode::Year => rsx! {
                    div { class: "sd-calendar-grid--items",
                        for y in *year_range_start.read()..=(*year_range_start.read() + 11) {
                            {
                                let is_cur_year = y == year;
                                let btn_cls = if is_cur_year { "sd-calendar-item-btn sd-calendar-item-btn--selected" } else { "sd-calendar-item-btn" };
                                rsx! {
                                    button {
                                        key: "{y}",
                                        r#type: "button",
                                        class: "{btn_cls}",
                                        onclick: move |_| {
                                            let cur = *current_view.read();
                                            current_view.set(Date::new(y, cur.month, 1.min(cur.day)));
                                            view_mode.set(CalendarViewMode::Month);
                                        },
                                        "{y}"
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

/// Campo di input con popup a calendario per la selezione di date singole.
#[component]
pub fn DatePicker(
    /// Binding reattivo al valore della data selezionata.
    mut value: Signal<Option<Date>>,
    /// Testo segnaposto dell'input. Defaults to the locale `date_placeholder` string.
    #[props(default)]
    placeholder: String,
    /// Se mostrare il pulsante per cancellare la selezione. Defaults to true.
    #[props(default = true)]
    clearable: bool,
    /// Se il controllo è disabilitato. Defaults to false.
    #[props(default = false)]
    disabled: bool,
    /// Data minima selezionabile.
    #[props(default)]
    min_date: Option<Date>,
    /// Data massima selezionabile.
    #[props(default)]
    max_date: Option<Date>,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let placeholder = if placeholder.is_empty() { locale.date_placeholder.clone() } else { placeholder };
    let mut is_open = use_signal(|| false);

    let display_text = match *value.read() {
        Some(d) => d.format_it(),
        None => String::new(),
    };

    let toggle_open = move |_| {
        if !disabled {
            is_open.toggle();
        }
    };

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    rsx! {
        div { class: "sd-date-picker-wrap {class}",
            div {
                class: if disabled { "sd-date-picker-input-box sd-date-picker-input-box--disabled" } else { "sd-date-picker-input-box" },
                "aria-disabled": if disabled { "true" } else { "false" },
                onclick: toggle_open,
                Icon { kind: IconKind::Calendar, size: 16, class: "sd-date-picker-icon" }
                input {
                    r#type: "text",
                    readonly: true,
                    disabled: disabled,
                    placeholder: "{placeholder}",
                    value: "{display_text}",
                    class: "sd-date-picker-input",
                }
                if clearable && value.read().is_some() && !disabled {
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Xs,
                        class: "sd-date-picker-clear-btn".to_string(),
                        aria_label: locale.clear_text.clone(),
                        onclick: move |e: MouseEvent| {
                            e.stop_propagation();
                            value.set(None);
                        },
                        Icon { kind: IconKind::X, size: 13 }
                    }
                }
            }

            if *is_open.read() {
                div {
                    class: "sd-date-picker-backdrop",
                    onclick: move |_| is_open.set(false),
                }
                div {
                    class: "sd-date-picker-popup",
                    onkeydown: move |e: KeyboardEvent| {
                        if e.key() == Key::Escape {
                            is_open.set(false);
                        }
                    },
                    Calendar {
                        selected: *value.read(),
                        min_date: min_date,
                        max_date: max_date,
                        on_select: move |selected_date| {
                            value.set(Some(selected_date));
                            is_open.set(false);
                        },
                    }
                }
            }
        }
    }
}

/// Selettore interattivo per intervalli di date (Range Picker).
#[component]
pub fn DateRangePicker(
    /// Binding alla data di inizio intervallo.
    mut start_date: Signal<Option<Date>>,
    /// Binding alla data di fine intervallo.
    mut end_date: Signal<Option<Date>>,
    /// Testo segnaposto. Defaults to the locale `date_range_placeholder` string.
    #[props(default)]
    placeholder: String,
    /// Se il controllo è disabilitato. Defaults to false.
    #[props(default = false)]
    disabled: bool,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let placeholder = if placeholder.is_empty() { locale.date_range_placeholder.clone() } else { placeholder };
    let mut is_open = use_signal(|| false);

    let display_text = match (*start_date.read(), *end_date.read()) {
        (Some(s), Some(e)) => format!("{} — {}", s.format_it(), e.format_it()),
        (Some(s), None) => format!("{} — …", s.format_it()),
        _ => String::new(),
    };

    let range_tuple = match (*start_date.read(), *end_date.read()) {
        (Some(s), Some(e)) => Some((s, e)),
        (Some(s), None) => Some((s, s)),
        _ => None,
    };

    let handle_select = move |d: Date| {
        let cur_start = *start_date.read();
        let cur_end = *end_date.read();

        match (cur_start, cur_end) {
            (None, _) | (Some(_), Some(_)) => {
                start_date.set(Some(d));
                end_date.set(None);
            }
            (Some(s), None) => {
                if d < s {
                    start_date.set(Some(d));
                    end_date.set(Some(s));
                } else {
                    end_date.set(Some(d));
                }
                is_open.set(false);
            }
        }
    };

    rsx! {
        div { class: "sd-date-picker-wrap {class}",
            div {
                class: if disabled { "sd-date-picker-input-box sd-date-picker-input-box--disabled" } else { "sd-date-picker-input-box" },
                "aria-disabled": if disabled { "true" } else { "false" },
                onclick: move |_| {
                    if !disabled {
                        is_open.toggle();
                    }
                },
                Icon { kind: IconKind::Calendar, size: 16, class: "sd-date-picker-icon" }
                input {
                    r#type: "text",
                    readonly: true,
                    disabled: disabled,
                    placeholder: "{placeholder}",
                    value: "{display_text}",
                    class: "sd-date-picker-input",
                }
                if (start_date.read().is_some() || end_date.read().is_some()) && !disabled {
                    Button {
                        variant: ButtonVariant::Ghost,
                        size: ButtonSize::Xs,
                        class: "sd-date-picker-clear-btn".to_string(),
                        aria_label: locale.clear_text.clone(),
                        onclick: move |e: MouseEvent| {
                            e.stop_propagation();
                            start_date.set(None);
                            end_date.set(None);
                        },
                        Icon { kind: IconKind::X, size: 13 }
                    }
                }
            }

            if *is_open.read() {
                div {
                    class: "sd-date-picker-backdrop",
                    onclick: move |_| is_open.set(false),
                }
                div {
                    class: "sd-date-picker-popup",
                    onkeydown: move |e: KeyboardEvent| {
                        if e.key() == Key::Escape {
                            is_open.set(false);
                        }
                    },
                    Calendar {
                        range: range_tuple,
                        on_select: handle_select,
                    }
                }
            }
        }
    }
}
