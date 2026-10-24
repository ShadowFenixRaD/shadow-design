use dioxus::prelude::*;

/// Centralized strings and labels used by shadow-design components.
/// English is the default language, and custom or localized strings can be provided
/// globally via [`LocaleProvider`] or overridden per-component via props.
#[derive(Clone, PartialEq, Debug)]
pub struct Locale {
    pub select_placeholder: String,
    pub multiselect_placeholder: String,
    pub combobox_placeholder: String,
    pub search_placeholder: String,
    pub nothing_found: String,
    pub create_label_prefix: String,
    pub add_label_prefix: String,
    pub remove: String,
    pub clear_selection: String,
    pub clear_text: String,
    pub loading: String,
    pub close: String,
    pub info: String,
    pub success: String,
    pub warning: String,
    pub error: String,
    /// Generic cancel action.
    pub cancel: String,
    /// Generic confirm action.
    pub confirm: String,
    /// Generic send/submit action.
    pub send: String,
    /// Reply action.
    pub reply: String,
    /// Share action.
    pub share: String,
    /// Download action.
    pub download: String,
    /// Delete action.
    pub delete: String,
    /// Generic "previous" navigation label.
    pub previous: String,
    /// Generic "next" navigation label.
    pub next: String,
    /// Step-back label.
    pub back: String,
    /// Step-forward label.
    pub forward: String,
    /// Finish/complete label.
    pub finish: String,
    /// Copy action.
    pub copy: String,
    /// Copy confirmation.
    pub copied: String,
    /// Copy button accessible label.
    pub copy_to_clipboard: String,
    /// Activity feed reply placeholder.
    pub activity_reply_placeholder: String,
    /// Activity feed empty state.
    pub activity_empty: String,
    /// Breadcrumbs landmark label.
    pub breadcrumbs_label: String,
    /// Breadcrumbs expand button title.
    pub breadcrumbs_expand: String,
    /// Breadcrumbs collapse button title.
    pub breadcrumbs_collapse: String,
    /// Chart tooltip "value" row label.
    pub chart_value: String,
    /// Chart tooltip "percentage" row label.
    pub chart_percentage: String,
    /// Heatmap fallback cell title prefix.
    pub chart_cell: String,
    /// Chat delivery status: sending.
    pub delivery_sending: String,
    /// Chat delivery status: sent.
    pub delivery_sent: String,
    /// Chat delivery status: delivered.
    pub delivery_delivered: String,
    /// Chat delivery status: failed.
    pub delivery_failed: String,
    /// Coachmark step badge template; placeholders `{step}` and `{total}`.
    pub coachmark_step: String,
    /// Coachmark close button title.
    pub coachmark_close: String,
    /// Command palette search placeholder.
    pub command_placeholder: String,
    /// Command palette empty state; placeholder `{query}`.
    pub command_empty: String,
    /// Command palette footer hint.
    pub command_hint_navigate: String,
    /// Command palette footer hint.
    pub command_hint_select: String,
    /// Command palette footer hint.
    pub command_hint_close: String,
    /// Date picker placeholder.
    pub date_placeholder: String,
    /// Date range picker placeholder.
    pub date_range_placeholder: String,
    /// Dropzone title.
    pub dropzone_title: String,
    /// Facepile overflow button title; placeholder `{n}`.
    pub facepile_overflow_title: String,
    /// Facepile overflow menu heading; placeholder `{n}`.
    pub facepile_overflow_heading: String,
    /// Facepile add button title.
    pub facepile_add: String,
    /// Navbar landmark label.
    pub navbar_label: String,
    /// Navbar toggle label when closed.
    pub navbar_open: String,
    /// Navbar toggle label when open.
    pub navbar_close: String,
    /// Navigation menu preview badge.
    pub navigation_featured: String,
    /// Number input increment button label.
    pub number_increment: String,
    /// Number input decrement button label.
    pub number_decrement: String,
    /// Pagination landmark label.
    pub pagination_label: String,
    /// Pagination first-page button label.
    pub pagination_first: String,
    /// Pagination previous-page button label.
    pub pagination_previous: String,
    /// Pagination next-page button label.
    pub pagination_next: String,
    /// Pagination last-page button label.
    pub pagination_last: String,
    /// Pagination page button label; placeholder `{n}`.
    pub pagination_page: String,
    /// Rich text editor placeholder.
    pub rich_text_placeholder: String,
    /// Range slider minimum thumb label.
    pub slider_min: String,
    /// Range slider maximum thumb label.
    pub slider_max: String,
    /// Spoiler expand label.
    pub spoiler_show_more: String,
    /// Spoiler collapse label.
    pub spoiler_show_less: String,
    /// Data table loading overlay text.
    pub table_loading: String,
    /// Data table empty state.
    pub table_empty: String,
    /// Data table footer: item count label.
    pub table_items: String,
    /// Data table footer: page label.
    pub table_page: String,
    /// Data table footer: "page X of Y" connector.
    pub table_of: String,
    /// Data table previous-page button.
    pub table_prev: String,
    /// Data table next-page button.
    pub table_next: String,
    /// Transfer list search placeholder.
    pub transfer_search: String,
    /// Transfer list empty source panel.
    pub transfer_empty_source: String,
    /// Transfer list empty target panel.
    pub transfer_empty_target: String,
    /// Transfer list control title.
    pub transfer_move_all_right: String,
    /// Transfer list control title.
    pub transfer_move_selected_right: String,
    /// Transfer list control title.
    pub transfer_move_selected_left: String,
    /// Transfer list control title.
    pub transfer_move_all_left: String,
    /// Back-to-top affix title.
    pub back_to_top: String,
    /// Carousel region label.
    pub carousel_label: String,
    /// Carousel previous control label.
    pub carousel_previous: String,
    /// Carousel next control label.
    pub carousel_next: String,
    /// Carousel dot label; placeholder `{n}`.
    pub carousel_goto: String,
    /// App shell aside landmark label.
    pub app_shell_aside_label: String,
    /// App shell side navigation landmark label.
    pub app_shell_navbar_label: String,
    /// Bottom navigation landmark label.
    pub bottom_navigation_label: String,
    /// Menubar label.
    pub menubar_label: String,
    /// Clickable card link label.
    pub card_link: String,
    /// Combobox suggestions toggle label.
    pub toggle_suggestions: String,
    /// Full month names, January first (12 entries).
    pub months: Vec<String>,
    /// Single-thumb slider fallback accessible label.
    pub slider_label: String,
    /// Rating fallback accessible label.
    pub rating_label: String,
    /// Pin input group accessible label.
    pub pin_input_label: String,
    /// Pin input digit label; placeholders `{n}` and `{total}`.
    pub pin_digit_label: String,
    /// Color picker format switch accessible label.
    pub color_format_label: String,
    /// Color input swatch button title.
    pub color_open_picker: String,
    /// Two-letter weekday headers, Monday first (7 entries).
    pub weekdays_short: Vec<String>,
    /// Three-letter weekday abbreviations, Monday first (7 entries).
    pub weekdays_abbr: Vec<String>,
}

impl Default for Locale {
    fn default() -> Self {
        Self::english()
    }
}

impl Locale {
    /// English default strings for shadow-design.
    #[must_use]
    pub fn english() -> Self {
        Self {
            select_placeholder: "Select an option...".to_string(),
            multiselect_placeholder: "Select options...".to_string(),
            combobox_placeholder: "Type or select...".to_string(),
            search_placeholder: "Search options...".to_string(),
            nothing_found: "No options found".to_string(),
            create_label_prefix: "+ Create".to_string(),
            add_label_prefix: "+ Add".to_string(),
            remove: "Remove".to_string(),
            clear_selection: "Clear selection".to_string(),
            clear_text: "Clear text".to_string(),
            loading: "Loading...".to_string(),
            close: "Close".to_string(),
            info: "Info".to_string(),
            success: "Success".to_string(),
            warning: "Warning".to_string(),
            error: "Error".to_string(),
            cancel: "Cancel".to_string(),
            confirm: "Confirm".to_string(),
            send: "Send".to_string(),
            reply: "Reply".to_string(),
            share: "Share".to_string(),
            download: "Download".to_string(),
            delete: "Delete".to_string(),
            previous: "Previous".to_string(),
            next: "Next".to_string(),
            back: "Back".to_string(),
            forward: "Next".to_string(),
            finish: "Finish".to_string(),
            copy: "Copy".to_string(),
            copied: "Copied!".to_string(),
            copy_to_clipboard: "Copy to clipboard".to_string(),
            activity_reply_placeholder: "Write a comment...".to_string(),
            activity_empty: "No activity recorded.".to_string(),
            breadcrumbs_label: "Breadcrumb".to_string(),
            breadcrumbs_expand: "Show full path".to_string(),
            breadcrumbs_collapse: "Collapse path".to_string(),
            chart_value: "Value".to_string(),
            chart_percentage: "Percentage".to_string(),
            chart_cell: "Cell".to_string(),
            delivery_sending: "Sending...".to_string(),
            delivery_sent: "Sent".to_string(),
            delivery_delivered: "Delivered".to_string(),
            delivery_failed: "Delivery failed".to_string(),
            coachmark_step: "Step {step} of {total}".to_string(),
            coachmark_close: "Close tour".to_string(),
            command_placeholder: "Search commands or navigate...".to_string(),
            command_empty: "No commands or results for \"{query}\"".to_string(),
            command_hint_navigate: "to navigate".to_string(),
            command_hint_select: "to select".to_string(),
            command_hint_close: "to close".to_string(),
            date_placeholder: "Select date...".to_string(),
            date_range_placeholder: "Select range...".to_string(),
            dropzone_title: "Drag files here or click to browse".to_string(),
            facepile_overflow_title: "View all {n} additional collaborators".to_string(),
            facepile_overflow_heading: "Other contributors ({n})".to_string(),
            facepile_add: "Add new member".to_string(),
            navbar_label: "Main".to_string(),
            navbar_open: "Open navigation menu".to_string(),
            navbar_close: "Close navigation menu".to_string(),
            navigation_featured: "Featured".to_string(),
            number_increment: "Increment".to_string(),
            number_decrement: "Decrement".to_string(),
            pagination_label: "Pagination".to_string(),
            pagination_first: "First page".to_string(),
            pagination_previous: "Previous page".to_string(),
            pagination_next: "Next page".to_string(),
            pagination_last: "Last page".to_string(),
            pagination_page: "Page {n}".to_string(),
            rich_text_placeholder: "Write your content here...".to_string(),
            slider_min: "Minimum".to_string(),
            slider_max: "Maximum".to_string(),
            spoiler_show_more: "Show more".to_string(),
            spoiler_show_less: "Show less".to_string(),
            table_loading: "Loading data...".to_string(),
            table_empty: "No data available".to_string(),
            table_items: "Items".to_string(),
            table_page: "Page".to_string(),
            table_of: "of".to_string(),
            table_prev: "« Prev".to_string(),
            table_next: "Next »".to_string(),
            transfer_search: "Search items...".to_string(),
            transfer_empty_source: "No items available".to_string(),
            transfer_empty_target: "No items assigned".to_string(),
            transfer_move_all_right: "Move all to the right".to_string(),
            transfer_move_selected_right: "Move selected to the right".to_string(),
            transfer_move_selected_left: "Move selected to the left".to_string(),
            transfer_move_all_left: "Move all to the left".to_string(),
            back_to_top: "Back to top".to_string(),
            carousel_label: "Content carousel".to_string(),
            carousel_previous: "Previous slide".to_string(),
            carousel_next: "Next slide".to_string(),
            carousel_goto: "Go to slide {n}".to_string(),
            app_shell_aside_label: "Auxiliary side panel".to_string(),
            app_shell_navbar_label: "Application side navigation".to_string(),
            bottom_navigation_label: "Main bottom navigation".to_string(),
            menubar_label: "Main menu bar".to_string(),
            card_link: "Card link".to_string(),
            toggle_suggestions: "Toggle suggestions".to_string(),
            slider_label: "Slider".to_string(),
            rating_label: "Rating".to_string(),
            pin_input_label: "PIN code input".to_string(),
            pin_digit_label: "Digit {n} of {total}".to_string(),
            color_format_label: "Format selector".to_string(),
            color_open_picker: "Open color picker".to_string(),
            months: vec!["January".to_string(), "February".to_string(), "March".to_string(), "April".to_string(), "May".to_string(), "June".to_string(), "July".to_string(), "August".to_string(), "September".to_string(), "October".to_string(), "November".to_string(), "December".to_string()],
            weekdays_short: vec!["Mo".to_string(), "Tu".to_string(), "We".to_string(), "Th".to_string(), "Fr".to_string(), "Sa".to_string(), "Su".to_string()],
            weekdays_abbr: vec!["Mon".to_string(), "Tue".to_string(), "Wed".to_string(), "Thu".to_string(), "Fri".to_string(), "Sat".to_string(), "Sun".to_string()],
        }
    }

    /// Italian preset strings for compatibility and localized showcases.
    #[must_use]
    pub fn italian() -> Self {
        Self {
            select_placeholder: "Seleziona un'opzione...".to_string(),
            multiselect_placeholder: "Seleziona opzioni...".to_string(),
            combobox_placeholder: "Digita o seleziona...".to_string(),
            search_placeholder: "Cerca opzioni...".to_string(),
            nothing_found: "Nessuna opzione trovata".to_string(),
            create_label_prefix: "+ Crea".to_string(),
            add_label_prefix: "+ Aggiungi".to_string(),
            remove: "Rimuovi".to_string(),
            clear_selection: "Svuota selezioni".to_string(),
            clear_text: "Cancella testo".to_string(),
            loading: "Caricamento...".to_string(),
            close: "Chiudi".to_string(),
            info: "Informazione".to_string(),
            success: "Successo".to_string(),
            warning: "Attenzione".to_string(),
            error: "Errore".to_string(),
            cancel: "Annulla".to_string(),
            confirm: "Conferma".to_string(),
            send: "Invia".to_string(),
            reply: "Rispondi".to_string(),
            share: "Condividi".to_string(),
            download: "Scarica".to_string(),
            delete: "Elimina".to_string(),
            previous: "Precedente".to_string(),
            next: "Successivo".to_string(),
            back: "Indietro".to_string(),
            forward: "Avanti".to_string(),
            finish: "Completa".to_string(),
            copy: "Copia".to_string(),
            copied: "Copiato!".to_string(),
            copy_to_clipboard: "Copia negli appunti".to_string(),
            activity_reply_placeholder: "Scrivi un commento...".to_string(),
            activity_empty: "Nessuna attività registrata.".to_string(),
            breadcrumbs_label: "Percorso di navigazione".to_string(),
            breadcrumbs_expand: "Mostra percorso completo".to_string(),
            breadcrumbs_collapse: "Comprimi percorso".to_string(),
            chart_value: "Valore".to_string(),
            chart_percentage: "Percentuale".to_string(),
            chart_cell: "Cella".to_string(),
            delivery_sending: "In invio...".to_string(),
            delivery_sent: "Inviato".to_string(),
            delivery_delivered: "Consegnato".to_string(),
            delivery_failed: "Invio non riuscito".to_string(),
            coachmark_step: "Passo {step} di {total}".to_string(),
            coachmark_close: "Chiudi guida".to_string(),
            command_placeholder: "Cerca comandi o naviga...".to_string(),
            command_empty: "Nessun comando o risultato per \"{query}\"".to_string(),
            command_hint_navigate: "per navigare".to_string(),
            command_hint_select: "per selezionare".to_string(),
            command_hint_close: "per chiudere".to_string(),
            date_placeholder: "Seleziona data...".to_string(),
            date_range_placeholder: "Seleziona intervallo...".to_string(),
            dropzone_title: "Trascina qui i file o clicca per sfogliare".to_string(),
            facepile_overflow_title: "Visualizza tutti i {n} collaboratori aggiuntivi".to_string(),
            facepile_overflow_heading: "Altri collaboratori ({n})".to_string(),
            facepile_add: "Aggiungi nuovo membro".to_string(),
            navbar_label: "Principale".to_string(),
            navbar_open: "Apri menu di navigazione".to_string(),
            navbar_close: "Chiudi menu di navigazione".to_string(),
            navigation_featured: "In evidenza".to_string(),
            number_increment: "Incrementa".to_string(),
            number_decrement: "Decrementa".to_string(),
            pagination_label: "Paginazione".to_string(),
            pagination_first: "Prima pagina".to_string(),
            pagination_previous: "Pagina precedente".to_string(),
            pagination_next: "Pagina successiva".to_string(),
            pagination_last: "Ultima pagina".to_string(),
            pagination_page: "Pagina {n}".to_string(),
            rich_text_placeholder: "Scrivi qui il contenuto...".to_string(),
            slider_min: "Minimo".to_string(),
            slider_max: "Massimo".to_string(),
            spoiler_show_more: "Mostra di più".to_string(),
            spoiler_show_less: "Mostra meno".to_string(),
            table_loading: "Caricamento dati...".to_string(),
            table_empty: "Nessun dato disponibile".to_string(),
            table_items: "Elementi".to_string(),
            table_page: "Pagina".to_string(),
            table_of: "di".to_string(),
            table_prev: "« Prec".to_string(),
            table_next: "Succ »".to_string(),
            transfer_search: "Cerca elementi...".to_string(),
            transfer_empty_source: "Nessun elemento disponibile".to_string(),
            transfer_empty_target: "Nessun elemento assegnato".to_string(),
            transfer_move_all_right: "Sposta tutti a destra".to_string(),
            transfer_move_selected_right: "Sposta selezionati a destra".to_string(),
            transfer_move_selected_left: "Sposta selezionati a sinistra".to_string(),
            transfer_move_all_left: "Sposta tutti a sinistra".to_string(),
            back_to_top: "Torna in cima".to_string(),
            carousel_label: "Carosello di contenuti".to_string(),
            carousel_previous: "Slide precedente".to_string(),
            carousel_next: "Slide successiva".to_string(),
            carousel_goto: "Vai alla slide {n}".to_string(),
            app_shell_aside_label: "Pannello secondario ausiliario".to_string(),
            app_shell_navbar_label: "Navigazione laterale dell'applicazione".to_string(),
            bottom_navigation_label: "Navigazione inferiore principale".to_string(),
            menubar_label: "Barra dei menu principale".to_string(),
            card_link: "Link della card".to_string(),
            toggle_suggestions: "Mostra o nascondi suggerimenti".to_string(),
            slider_label: "Cursore".to_string(),
            rating_label: "Valutazione".to_string(),
            pin_input_label: "Inserimento codice PIN".to_string(),
            pin_digit_label: "Cifra {n} di {total}".to_string(),
            color_format_label: "Selettore formato".to_string(),
            color_open_picker: "Apri selettore colore".to_string(),
            months: vec!["Gennaio".to_string(), "Febbraio".to_string(), "Marzo".to_string(), "Aprile".to_string(), "Maggio".to_string(), "Giugno".to_string(), "Luglio".to_string(), "Agosto".to_string(), "Settembre".to_string(), "Ottobre".to_string(), "Novembre".to_string(), "Dicembre".to_string()],
            weekdays_short: vec!["Lu".to_string(), "Ma".to_string(), "Me".to_string(), "Gi".to_string(), "Ve".to_string(), "Sa".to_string(), "Do".to_string()],
            weekdays_abbr: vec!["Lun".to_string(), "Mar".to_string(), "Mer".to_string(), "Gio".to_string(), "Ven".to_string(), "Sab".to_string(), "Dom".to_string()],
        }
    }

    /// Full name of a month (1-12), or an empty string when out of range.
    #[must_use]
    pub fn month_name(&self, month: u32) -> &str {
        month
            .checked_sub(1)
            .and_then(|i| self.months.get(i as usize))
            .map_or("", String::as_str)
    }

    /// Renders a `{key}` template by substituting each `(key, value)` pair.
    #[must_use]
    pub fn fmt(template: &str, args: &[(&str, &str)]) -> String {
        let mut out = template.to_string();
        for (k, v) in args {
            out = out.replace(&format!("{{{k}}}"), v);
        }
        out
    }

    #[must_use]
    pub fn format_create(&self, val: &str) -> String {
        format!("{} \"{}\"", self.create_label_prefix, val)
    }

    #[must_use]
    pub fn format_add(&self, val: &str) -> String {
        format!("{} \"{}\"", self.add_label_prefix, val)
    }
}

/// Provides a global [`Locale`] configuration to the Dioxus component tree.
#[component]
pub fn LocaleProvider(#[props(default)] locale: Option<Signal<Locale>>, children: Element) -> Element {
    let owned = use_signal(Locale::default);
    let active = locale.unwrap_or(owned);
    use_context_provider(|| active);
    rsx! { {children} }
}

/// Hook to read the active [`Locale`], falling back to English defaults if no provider is present.
#[must_use]
pub fn use_locale() -> Locale {
    match try_use_context::<Signal<Locale>>() {
        Some(sig) => sig.read().clone(),
        None => Locale::default(),
    }
}
