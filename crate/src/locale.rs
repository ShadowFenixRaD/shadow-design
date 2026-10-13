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
}

impl Default for Locale {
    fn default() -> Self {
        Self::english()
    }
}

impl Locale {
    /// English default strings for shadow-design.
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
        }
    }

    /// Italian preset strings for compatibility and localized showcases.
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
        }
    }

    pub fn format_create(&self, val: &str) -> String {
        format!("{} \"{}\"", self.create_label_prefix, val)
    }

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
pub fn use_locale() -> Locale {
    match try_use_context::<Signal<Locale>>() {
        Some(sig) => sig.read().clone(),
        None => Locale::default(),
    }
}
