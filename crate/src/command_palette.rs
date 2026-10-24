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
    /// Testo segnaposto dell'input di ricerca. Defaults to the locale `command_placeholder`.
    #[props(default)]
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
    let locale = crate::locale::use_locale();
    let placeholder = if placeholder.is_empty() { locale.command_placeholder.clone() } else { placeholder };
    let search_query = use_signal(String::new);
    let mut active_index = use_signal(|| 0usize);

    // early return moved below hooks

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

    // Normalizza active_index per prevenire overflow senza mutare nel body
    let current_idx = if count == 0 {
        0
    } else {
        (*active_index.read()).min(count - 1)
    };

    use_effect(move || {
        let is_open = *opened.read();
        if is_open {
            let _ = document::eval(
                r#"
                setTimeout(() => {
                    const input = document.getElementById("sd-command-palette-input");
                    if (input) {
                        input.focus();
                        input.select();
                    }
                }, 25);
                "#
            );
        }
    });

    use_effect(move || {
        let idx = *active_index.read();
        let _ = document::eval(&format!(
            r#"
            setTimeout(() => {{
                const el = document.getElementById("sd-command-palette-item-{idx}");
                if (el) {{
                    el.scrollIntoView({{ block: "nearest", behavior: "smooth" }});
                }}
            }}, 15);
            "#
        ));
    });

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

    // Gestore eventi da tastiera nativo / globale quando il palette è aperto
    let filtered_actions_for_keys = filtered_actions.clone();
    use_effect(move || {
        let is_open = *opened.read();
        if !is_open {
            return;
        }

        let mut eval = document::eval(
            r#"
            if (window._sdCmdPaletteCleanup) {
                window._sdCmdPaletteCleanup();
            }
            const onKeyDown = (e) => {
                if (e.key === "ArrowDown") {
                    e.preventDefault();
                    dioxus.send("down");
                } else if (e.key === "ArrowUp") {
                    e.preventDefault();
                    dioxus.send("up");
                } else if (e.key === "Enter") {
                    e.preventDefault();
                    dioxus.send("enter");
                } else if (e.key === "Escape") {
                    e.preventDefault();
                    dioxus.send("escape");
                }
            };
            window.addEventListener("keydown", onKeyDown, true);
            window._sdCmdPaletteCleanup = () => {
                window.removeEventListener("keydown", onKeyDown, true);
                window._sdCmdPaletteCleanup = null;
            };
            "#
        );

        let filtered_len = filtered_actions_for_keys.len();
        let current_acts = filtered_actions_for_keys.clone();
        let handle_sel = handle_select;

        spawn(async move {
            while let Ok(msg) = eval.recv::<String>().await {
                match msg.as_str() {
                    "down" => {
                        if filtered_len > 0 {
                            let cur = *active_index.read();
                            active_index.set((cur + 1) % filtered_len);
                        }
                    }
                    "up" => {
                        if filtered_len > 0 {
                            let cur = *active_index.read();
                            active_index.set(if cur == 0 { filtered_len - 1 } else { cur - 1 });
                        }
                    }
                    "enter" => {
                        let cur = *active_index.read();
                        if let Some(act) = current_acts.get(cur) {
                            handle_sel(act.clone());
                        }
                    }
                    "escape" => {
                        let mut op = opened;
                        let mut sq = search_query;
                        op.set(false);
                        sq.set(String::new());
                    }
                    _ => {}
                }
            }
        });
    });

    use_drop(move || {
        let _ = document::eval(
            r#"
            if (window._sdCmdPaletteCleanup) {
                window._sdCmdPaletteCleanup();
            }
            "#
        );
    });

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    if !*opened.read() {
        return rsx! {};
    }

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
                role: "dialog",
                "aria-modal": "true",
                tabindex: "-1",
                onclick: move |e| e.stop_propagation(),

                // Input bar superiore
                div { class: "sd-command-palette-input-wrap",
                    Icon { kind: IconKind::Search, size: 18, class: "sd-command-palette-search-icon" }
                    input {
                        id: "sd-command-palette-input",
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
                div { class: "sd-command-palette-list", role: "listbox",
                    if filtered_actions.is_empty() {
                        div { class: "sd-command-palette-empty",
                            Icon { kind: IconKind::AlertCircle, size: 24 }
                            span { "{crate::locale::Locale::fmt(&locale.command_empty, &[(\"query\", &search_query())])}" }
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
                                                id: "sd-command-palette-item-{i}",
                                                class: "{item_class}",
                                                role: "option",
                                                "aria-selected": "{is_active}",
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
                        span { "{locale.command_hint_navigate}" }
                    }
                    div { class: "sd-command-palette-footer-item",
                        Kbd { "↵" }
                        span { "{locale.command_hint_select}" }
                    }
                    div { class: "sd-command-palette-footer-item",
                        Kbd { "Esc" }
                        span { "{locale.command_hint_close}" }
                    }
                }
            }
        }
    }
}
