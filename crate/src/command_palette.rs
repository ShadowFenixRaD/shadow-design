use crate::icon::{Icon, IconKind};
#[cfg(feature = "theme")]
use crate::theme::use_theme;
use crate::typography::Kbd;
use dioxus::prelude::*;

/// Rappresenta una singola azione all'interno della [`CommandPalette`].
#[derive(Clone, PartialEq, Debug)]
pub struct CommandAction {
    /// Identificativo univoco dell'azione.
    pub id: String,
    /// Titolo visibile dell'azione.
    pub title: String,
    /// Descrizione opzionale o contesto secondario.
    pub description: Option<String>,
    /// Categoria di raggruppamento (es. "Navigazione", "Sistema", "Temi").
    pub category: Option<String>,
    /// Scorciatoia da tastiera associata da visualizzare come badge Kbd (es. "Ctrl+P").
    pub shortcut: Option<String>,
    /// Icona grafica associata.
    pub icon: Option<IconKind>,
    /// Parole chiave aggiuntive per la ricerca fuzzy.
    pub keywords: Vec<String>,
}

impl CommandAction {
    /// Crea una nuova azione con i parametri minimi obbligatori.
    #[must_use]
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            category: None,
            shortcut: None,
            icon: None,
            keywords: Vec::new(),
        }
    }

    /// Imposta la descrizione secondaria.
    #[must_use]
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Imposta la categoria di appartenenza.
    #[must_use]
    pub fn with_category(mut self, cat: impl Into<String>) -> Self {
        self.category = Some(cat.into());
        self
    }

    /// Imposta la scorciatoia da tastiera.
    #[must_use]
    pub fn with_shortcut(mut self, sc: impl Into<String>) -> Self {
        self.shortcut = Some(sc.into());
        self
    }

    /// Imposta l'icona grafica.
    #[must_use]
    pub fn with_icon(mut self, icon: IconKind) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Aggiunge parole chiave per la ricerca.
    #[must_use]
    pub fn with_keywords(mut self, keywords: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.keywords.extend(keywords.into_iter().map(Into::into));
        self
    }
}

/// Modal di ricerca e comando rapido universale (Spotlight) con filtro reattivo,
/// raggruppamento per categoria e navigazione completa da tastiera.
#[component]
pub fn CommandPalette(
    /// Segnale di controllo dell'apertura del palette.
    opened: Signal<bool>,
    /// Lista di tutte le azioni disponibili.
    #[props(default)]
    actions: Vec<CommandAction>,
    /// Testo segnaposto dell'input di ricerca. Defaults to "Cerca comandi o naviga...".
    #[props(default = "Cerca comandi o naviga...".to_string())]
    placeholder: String,
    /// Callback invocato quando un'azione viene selezionata.
    #[props(default)]
    on_select: Option<EventHandler<CommandAction>>,
    /// Se chiudere automaticamente il palette alla selezione di un'azione. Defaults to true.
    #[props(default = true)]
    close_on_select: bool,
    /// Classi CSS aggiuntive per il contenitore della modale.
    #[props(default)]
    class: String,
) -> Element {
    let search_query = use_signal(String::new);
    let mut active_index = use_signal(|| 0usize);

    if !*opened.read() {
        return rsx! {};
    }

    let query_val = search_query.read().trim().to_lowercase();

    // Filtra le azioni in base alla query
    let filtered_actions: Vec<CommandAction> = actions
        .iter()
        .filter(|action| {
            if query_val.is_empty() {
                return true;
            }
            if action.title.to_lowercase().contains(&query_val) {
                return true;
            }
            if let Some(desc) = &action.description {
                if desc.to_lowercase().contains(&query_val) {
                    return true;
                }
            }
            if let Some(cat) = &action.category {
                if cat.to_lowercase().contains(&query_val) {
                    return true;
                }
            }
            action
                .keywords
                .iter()
                .any(|kw| kw.to_lowercase().contains(&query_val))
        })
        .cloned()
        .collect();

    let count = filtered_actions.len();

    // Normalizza active_index per prevenire overflow
    let current_idx = {
        let mut idx = *active_index.read();
        if count == 0 {
            0
        } else if idx >= count {
            idx = count - 1;
            active_index.set(idx);
            idx
        } else {
            idx
        }
    };

    let actions_clone = filtered_actions.clone();
    let on_select_clone = on_select;

    let handle_select = move |action: CommandAction| {
        if let Some(ref handler) = on_select_clone {
            handler.call(action);
        }
        if close_on_select {
            let mut op = opened;
            let mut sq = search_query;
            op.set(false);
            sq.set(String::new());
        }
    };

    let select_current = {
        let acts = actions_clone.clone();
        move || {
            if let Some(action) = acts.get(current_idx) {
                handle_select(action.clone());
            }
        }
    };

    let handle_keydown = move |e: KeyboardEvent| {
        match e.key() {
            Key::ArrowDown => {
                e.prevent_default();
                if count > 0 {
                    active_index.set((current_idx + 1) % count);
                }
            }
            Key::ArrowUp => {
                e.prevent_default();
                if count > 0 {
                    active_index.set(if current_idx == 0 { count - 1 } else { current_idx - 1 });
                }
            }
            Key::Enter => {
                e.prevent_default();
                select_current();
            }
            Key::Escape => {
                e.prevent_default();
                let mut op = opened;
                let mut sq = search_query;
                op.set(false);
                sq.set(String::new());
            }
            _ => {}
        }
    };

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    rsx! {
        div {
            class: "sd-command-palette-backdrop",
            onclick: move |_| {
                let mut op = opened;
                let mut sq = search_query;
                op.set(false);
                sq.set(String::new());
            },
            div {
                class: "sd-command-palette-dialog {class}",
                onclick: move |e| e.stop_propagation(),
                onkeydown: handle_keydown,

                // Input bar superiore
                div { class: "sd-command-palette-input-wrap",
                    Icon { kind: IconKind::Search, size: 18, class: "sd-command-palette-search-icon" }
                    input {
                        r#type: "text",
                        autofocus: true,
                        class: "sd-command-palette-input",
                        placeholder: "{placeholder}",
                        value: "{search_query()}",
                        oninput: move |e| {
                            let mut sq = search_query;
                            sq.set(e.value());
                            active_index.set(0);
                        },
                    }
                    div { class: "sd-command-palette-badges",
                        Kbd { "Esc" }
                    }
                }

                // Lista delle azioni
                div { class: "sd-command-palette-list",
                    if filtered_actions.is_empty() {
                        div { class: "sd-command-palette-empty",
                            Icon { kind: IconKind::AlertCircle, size: 24 }
                            span { "Nessun comando o risultato per \"{search_query()}\"" }
                        }
                    } else {
                        {
                            let mut last_category: Option<String> = None;
                            rsx! {
                                for (i, action) in filtered_actions.iter().enumerate() {
                                    {
                                        let is_new_category = action.category != last_category;
                                        if is_new_category {
                                            last_category = action.category.clone();
                                        }
                                        let is_active = i == current_idx;
                                        let act_copy = action.clone();
                                        let item_class = if is_active { "sd-command-palette-item sd-command-palette-item--active" } else { "sd-command-palette-item" };

                                        rsx! {
                                            if is_new_category {
                                                if let Some(cat) = &action.category {
                                                    div { class: "sd-command-palette-group-title", "{cat}" }
                                                }
                                            }
                                            div {
                                                class: "{item_class}",
                                                onmouseenter: move |_| active_index.set(i),
                                                onclick: {
                                                    let act = act_copy.clone();
                                                    move |_| handle_select(act.clone())
                                                },
                                                div { class: "sd-command-palette-item-left",
                                                    if let Some(icon) = action.icon {
                                                        Icon { kind: icon, size: 16, class: "sd-command-palette-item-icon" }
                                                    }
                                                    div { class: "sd-command-palette-item-text",
                                                        span { class: "sd-command-palette-item-title", "{action.title}" }
                                                        if let Some(desc) = &action.description {
                                                            span { class: "sd-command-palette-item-desc", "{desc}" }
                                                        }
                                                    }
                                                }
                                                if let Some(sc) = &action.shortcut {
                                                    div { class: "sd-command-palette-item-shortcut",
                                                        Kbd { "{sc}" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Footer con legenda navigazione
                div { class: "sd-command-palette-footer",
                    div { class: "sd-command-palette-footer-item",
                        Kbd { "↑" }
                        Kbd { "↓" }
                        span { "per navigare" }
                    }
                    div { class: "sd-command-palette-footer-item",
                        Kbd { "↵" }
                        span { "per selezionare" }
                    }
                    div { class: "sd-command-palette-footer-item",
                        Kbd { "Esc" }
                        span { "per chiudere" }
                    }
                }
            }
        }
    }
}
