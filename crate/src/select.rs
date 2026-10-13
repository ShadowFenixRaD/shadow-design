use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Rappresenta una singola voce di selezione all'interno di `Select`, `MultiSelect` o `Combobox`.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectItem {
    /// Valore identificativo della voce (inviato nel form o nello stato).
    pub value: String,
    /// Etichetta visualizzata all'utente.
    pub label: String,
    /// Descrizione secondaria opzionale (sottotitolo per arricchire la voce).
    pub description: Option<String>,
    /// Indica se la voce è disabilitata e non selezionabile.
    pub disabled: bool,
    /// Gruppo di appartenenza opzionale (per raggruppamento con intestazione).
    pub group: Option<String>,
}

impl SelectItem {
    /// Crea una nuova voce con valore ed etichetta.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            disabled: false,
            group: None,
        }
    }

    /// Assegna una descrizione secondaria.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Assegna la voce a un gruppo specifico.
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Contrassegna la voce come disabilitata.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl From<&str> for SelectItem {
    fn from(s: &str) -> Self {
        Self::new(s, s)
    }
}

impl From<String> for SelectItem {
    fn from(s: String) -> Self {
        Self::new(s.clone(), s)
    }
}

impl From<(&str, &str)> for SelectItem {
    fn from((val, lbl): (&str, &str)) -> Self {
        Self::new(val, lbl)
    }
}

impl From<(String, String)> for SelectItem {
    fn from((val, lbl): (String, String)) -> Self {
        Self::new(val, lbl)
    }
}

/// Componente `Select` a scelta singola con supporto a ricerca live, raggruppamento opzioni,
/// creazione di nuovi valori a runtime e stati chiaro/scuro coerenti con il tema Shadow.
#[component]
pub fn Select(
    /// Collezione di elementi selezionabili.
    data: Vec<SelectItem>,
    /// Valore attualmente selezionato collegato reattivamente.
    value: Signal<Option<String>>,
    /// Testo segnaposto visualizzato quando nessun elemento è selezionato.
    #[props(default)]
    placeholder: Option<String>,
    /// Etichetta descrittiva superiore del campo.
    #[props(default)]
    label: Option<String>,
    /// Testo di aiuto o descrizione secondaria posizionato sotto l'etichetta.
    #[props(default)]
    description: Option<String>,
    /// Messaggio di errore di validazione visualizzato in rosso fenice.
    #[props(default)]
    error: Option<String>,
    /// Disabilita ogni interazione con il controllo.
    #[props(default)]
    disabled: bool,
    /// Abilita la barra di ricerca interna al menu a discesa per filtrare le opzioni.
    #[props(default)]
    searchable: bool,
    /// Mostra un'icona di cancellazione rapida (X) quando è presente un valore selezionato.
    #[props(default)]
    clearable: bool,
    /// Consente all'utente di selezionare/creare il testo digitato nella ricerca se non presente.
    #[props(default)]
    creatable: bool,
    /// Mostra uno spinner animato al posto dell'indicatore a freccia.
    #[props(default)]
    loading: bool,
    /// Alone ottico perimetrale coordinato con il tema attivo.
    #[props(default)]
    glow: Option<bool>,
    /// Classi CSS o SCSS addizionali.
    #[props(default)]
    class: String,
    /// Callback invocata alla selezione di un nuovo valore (o `None` al clear).
    #[props(default)]
    on_change: Option<EventHandler<Option<String>>>,
    /// Callback invocata alla creazione di una voce custom non presente nei dati.
    #[props(default)]
    on_create: Option<EventHandler<String>>,
) -> Element {
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(String::new);

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let radius_class = theme_val.radius_scale.md();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("select").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("select", &class);
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("select", glow);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let radius_class = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow.unwrap_or(false) {
        "sd-select-trigger--glow".to_string()
    } else {
        String::new()
    };

    #[cfg(feature = "animation")]
    let anim_class = crate::animation::Animations::DROPDOWN_ENTER;
    #[cfg(not(feature = "animation"))]
    let anim_class = "animate-dropdown-enter";

    let open = is_open();
    let current_val = value();
    let selected_item = data.iter().find(|i| current_val.as_ref() == Some(&i.value));

    let display_label = match selected_item {
        Some(item) => item.label.clone(),
        None => placeholder
            .clone()
            .unwrap_or_else(|| "Seleziona un'opzione...".to_string()),
    };
    let is_placeholder = selected_item.is_none();

    let open_trigger_class = if open { "sd-select-trigger--open" } else { "" };
    let error_trigger_class = if error.is_some() {
        "sd-select-trigger--error"
    } else {
        ""
    };
    let disabled_trigger_class = if disabled {
        "sd-select-trigger--disabled"
    } else {
        ""
    };
    let open_container_class = if open { "sd-select-container--open" } else { "" };

    let q = search_query().trim().to_lowercase();
    let filtered_items: Vec<&SelectItem> = if q.is_empty() {
        data.iter().collect()
    } else {
        data.iter()
            .filter(|i| {
                i.label.to_lowercase().contains(&q)
                    || i.value.to_lowercase().contains(&q)
                    || i.description
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&q)
            })
            .collect()
    };

    let exact_match = !q.is_empty()
        && data
            .iter()
            .any(|i| i.label.to_lowercase() == q || i.value.to_lowercase() == q);
    let show_create_option = creatable && !q.is_empty() && !exact_match;

    // Raccoglie i gruppi in ordine di apparizione per renderizzare le intestazioni
    let mut groups: Vec<Option<String>> = Vec::new();
    for item in &filtered_items {
        if !groups.contains(&item.group) {
            groups.push(item.group.clone());
        }
    }

    let has_clear_btn = clearable && current_val.is_some() && !disabled;

    rsx! {
        div {
            class: "sd-select-wrapper {class_modifiers} {class}",
            if let Some(lbl) = label {
                label { class: "sd-select-label", "{lbl}" }
            }
            if let Some(desc) = description {
                span { class: "sd-select-description", "{desc}" }
            }
            div {
                class: "sd-select-container {open_container_class}",
                div {
                    class: "sd-select-trigger {open_trigger_class} {error_trigger_class} {disabled_trigger_class} {base_override} {radius_class} {glow_class}",
                    onclick: move |_| {
                        if !disabled {
                            let next = !is_open();
                            is_open.set(next);
                            if !next {
                                search_query.set(String::new());
                            }
                        }
                    },
                    div {
                        class: if is_placeholder { "sd-select-value sd-select-value--placeholder" } else { "sd-select-value" },
                        "{display_label}"
                    }
                    div {
                        class: "sd-select-actions",
                        if loading {
                            div {
                                class: "sd-select-action-btn animate-spin",
                                Icon { kind: IconKind::Spinner, size: 14 }
                            }
                        } else {
                            if has_clear_btn {
                                button {
                                    r#type: "button",
                                    class: "sd-select-action-btn",
                                    title: "Rimuovi selezione",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        *value.write() = None;
                                        if let Some(h) = &on_change {
                                            h.call(None);
                                        }
                                    },
                                    Icon { kind: IconKind::X, size: 12 }
                                }
                            }
                            div {
                                class: if open { "sd-select-chevron sd-select-chevron--open" } else { "sd-select-chevron" },
                                Icon { kind: IconKind::ChevronDown, size: 14 }
                            }
                        }
                    }
                }

                if open {
                    div {
                        class: "sd-select-backdrop",
                        onclick: move |e| {
                            e.stop_propagation();
                            is_open.set(false);
                            search_query.set(String::new());
                        },
                    }
                    div {
                        class: "sd-select-menu {anim_class} {radius_class}",
                        onclick: move |e| e.stop_propagation(),

                        if searchable {
                            div {
                                class: "sd-select-search-container",
                                Icon { kind: IconKind::Search, size: 13, class: "sd-select-search-icon" }
                                input {
                                    class: "sd-select-search-input",
                                    placeholder: "Cerca opzioni...",
                                    value: "{search_query()}",
                                    autofocus: true,
                                    oninput: move |e| search_query.set(e.value()),
                                    onclick: move |e| e.stop_propagation(),
                                }
                                if !search_query().is_empty() {
                                    button {
                                        r#type: "button",
                                        class: "sd-select-search-clear",
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            search_query.set(String::new());
                                        },
                                        Icon { kind: IconKind::X, size: 11 }
                                    }
                                }
                            }
                        }

                        if filtered_items.is_empty() && !show_create_option {
                            div { class: "sd-select-empty", "Nessuna opzione trovata" }
                        } else {
                            for grp in groups.iter() {
                                if let Some(grp_name) = grp {
                                    div { class: "sd-select-group-header", "{grp_name}" }
                                }
                                for item in filtered_items.iter().filter(|i| &i.group == grp) {
                                    {
                                        let item_val = item.value.clone();
                                        let item_lbl = item.label.clone();
                                        let item_desc = item.description.clone();
                                        let item_disabled = item.disabled;
                                        let is_selected = current_val.as_ref() == Some(&item.value);
                                        let selected_class = if is_selected { "sd-select-item--selected" } else { "" };
                                        let disabled_class = if item_disabled { "sd-select-item--disabled" } else { "" };

                                        rsx! {
                                            button {
                                                r#type: "button",
                                                key: "{item.value}",
                                                class: "sd-select-item {selected_class} {disabled_class}",
                                                disabled: item_disabled,
                                                onclick: move |e| {
                                                    e.stop_propagation();
                                                    if !item_disabled {
                                                        *value.write() = Some(item_val.clone());
                                                        if let Some(h) = &on_change {
                                                            h.call(Some(item_val.clone()));
                                                        }
                                                        is_open.set(false);
                                                        search_query.set(String::new());
                                                    }
                                                },
                                                div { class: "sd-select-item-content",
                                                    span { class: "sd-select-item-label", "{item_lbl}" }
                                                    if let Some(desc_txt) = item_desc {
                                                        span { class: "sd-select-item-description", "{desc_txt}" }
                                                    }
                                                }
                                                if is_selected {
                                                    div { class: "sd-select-item-check",
                                                        Icon { kind: IconKind::Check, size: 14 }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if show_create_option {
                                {
                                    let create_val = search_query();
                                    let create_val_display = create_val.clone();
                                    rsx! {
                                        button {
                                            r#type: "button",
                                            class: "sd-select-item sd-select-item--creatable",
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                let val_to_add = create_val.clone();
                                                *value.write() = Some(val_to_add.clone());
                                                if let Some(hc) = &on_create {
                                                    hc.call(val_to_add.clone());
                                                }
                                                if let Some(h) = &on_change {
                                                    h.call(Some(val_to_add));
                                                }
                                                is_open.set(false);
                                                search_query.set(String::new());
                                            },
                                            div { class: "sd-select-item-content",
                                                span { class: "sd-select-item-label", "+ Crea \"{create_val_display}\"" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(err) = error {
                span { class: "sd-select-error", "{err}" }
            }
        }
    }
}

/// Componente `MultiSelect` per la selezione di valori multipli con chip/tag dismissabili,
/// ricerca live e gestione integrata di limiti visivi.
#[component]
pub fn MultiSelect(
    /// Collezione di elementi selezionabili.
    data: Vec<SelectItem>,
    /// Vettore dei valori attualmente selezionati collegato reattivamente.
    values: Signal<Vec<String>>,
    /// Testo segnaposto quando non ci sono valori selezionati.
    #[props(default)]
    placeholder: Option<String>,
    /// Etichetta descrittiva superiore del campo.
    #[props(default)]
    label: Option<String>,
    /// Testo di aiuto o descrizione secondaria posizionato sotto l'etichetta.
    #[props(default)]
    description: Option<String>,
    /// Messaggio di errore di validazione visualizzato in rosso fenice.
    #[props(default)]
    error: Option<String>,
    /// Disabilita ogni interazione con il controllo.
    #[props(default)]
    disabled: bool,
    /// Abilita la barra di ricerca interna per filtrare le opzioni.
    #[props(default)]
    searchable: bool,
    /// Mostra un'icona di cancellazione totale (X) per svuotare tutti i valori selezionati.
    #[props(default)]
    clearable: bool,
    /// Consente all'utente di aggiungere un nuovo valore digitato nella ricerca.
    #[props(default)]
    creatable: bool,
    /// Mostra uno spinner animato al posto della freccia.
    #[props(default)]
    loading: bool,
    /// Numero massimo di chip/tag renderizzati nel trigger (i rimanenti sono riassunti con `+N altri`).
    #[props(default)]
    max_selected_display: Option<usize>,
    /// Alone ottico perimetrale coordinato con il tema attivo.
    #[props(default)]
    glow: Option<bool>,
    /// Classi CSS o SCSS addizionali.
    #[props(default)]
    class: String,
    /// Callback invocata alla mutazione della lista di valori selezionati.
    #[props(default)]
    on_change: Option<EventHandler<Vec<String>>>,
    /// Callback invocata alla creazione di un nuovo tag personalizzato.
    #[props(default)]
    on_create: Option<EventHandler<String>>,
) -> Element {
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(String::new);

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let radius_class = theme_val.radius_scale.md();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("select").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("select", &class);
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("select", glow);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let radius_class = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow.unwrap_or(false) {
        "sd-select-trigger--glow".to_string()
    } else {
        String::new()
    };

    #[cfg(feature = "animation")]
    let anim_class = crate::animation::Animations::DROPDOWN_ENTER;
    #[cfg(not(feature = "animation"))]
    let anim_class = "animate-dropdown-enter";

    let open = is_open();
    let selected_vals = values();
    let is_empty = selected_vals.is_empty();

    let open_trigger_class = if open { "sd-select-trigger--open" } else { "" };
    let error_trigger_class = if error.is_some() {
        "sd-select-trigger--error"
    } else {
        ""
    };
    let disabled_trigger_class = if disabled {
        "sd-select-trigger--disabled"
    } else {
        ""
    };
    let open_container_class = if open { "sd-select-container--open" } else { "" };

    let q = search_query().trim().to_lowercase();
    let filtered_items: Vec<&SelectItem> = if q.is_empty() {
        data.iter().collect()
    } else {
        data.iter()
            .filter(|i| {
                i.label.to_lowercase().contains(&q)
                    || i.value.to_lowercase().contains(&q)
                    || i.description
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&q)
            })
            .collect()
    };

    let exact_match = !q.is_empty()
        && data
            .iter()
            .any(|i| i.label.to_lowercase() == q || i.value.to_lowercase() == q);
    let show_create_option = creatable && !q.is_empty() && !exact_match;

    let has_clear_btn = clearable && !is_empty && !disabled;

    let max_display = max_selected_display.unwrap_or(usize::MAX);
    let displayed_slice: Vec<String> = selected_vals.iter().take(max_display).cloned().collect();
    let overflow_count = if selected_vals.len() > max_display {
        selected_vals.len() - max_display
    } else {
        0
    };

    rsx! {
        div {
            class: "sd-select-wrapper {class_modifiers} {class}",
            if let Some(lbl) = label {
                label { class: "sd-select-label", "{lbl}" }
            }
            if let Some(desc) = description {
                span { class: "sd-select-description", "{desc}" }
            }
            div {
                class: "sd-select-container {open_container_class}",
                div {
                    class: "sd-select-trigger {open_trigger_class} {error_trigger_class} {disabled_trigger_class} {base_override} {radius_class} {glow_class}",
                    onclick: move |_| {
                        if !disabled {
                            let next = !is_open();
                            is_open.set(next);
                            if !next {
                                search_query.set(String::new());
                            }
                        }
                    },
                    div {
                        class: "sd-select-value",
                        if is_empty {
                            span {
                                class: "sd-select-value--placeholder",
                                {
                                    placeholder
                                        .clone()
                                        .unwrap_or_else(|| "Seleziona opzioni...".to_string())
                                }
                            }
                        } else {
                            div {
                                class: "sd-select-tags",
                                for val in displayed_slice.iter() {
                                    {
                                        let item_label = data
                                            .iter()
                                            .find(|i| &i.value == val)
                                            .map(|i| i.label.clone())
                                            .unwrap_or_else(|| val.clone());
                                        let v_remove = val.clone();

                                        rsx! {
                                            div {
                                                key: "{val}",
                                                class: "sd-select-chip",
                                                span { "{item_label}" }
                                                if !disabled {
                                                    button {
                                                        r#type: "button",
                                                        class: "sd-select-chip__remove",
                                                        title: "Rimuovi",
                                                        onclick: move |e| {
                                                            e.stop_propagation();
                                                            let mut cur = values();
                                                            if let Some(pos) = cur.iter().position(|x| x == &v_remove) {
                                                                cur.remove(pos);
                                                                *values.write() = cur.clone();
                                                                if let Some(h) = &on_change {
                                                                    h.call(cur);
                                                                }
                                                            }
                                                        },
                                                        Icon { kind: IconKind::X, size: 10 }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                if overflow_count > 0 {
                                    span { class: "sd-select-chip-counter", "+{overflow_count}" }
                                }
                            }
                        }
                    }
                    div {
                        class: "sd-select-actions",
                        if loading {
                            div {
                                class: "sd-select-action-btn animate-spin",
                                Icon { kind: IconKind::Spinner, size: 14 }
                            }
                        } else {
                            if has_clear_btn {
                                button {
                                    r#type: "button",
                                    class: "sd-select-action-btn",
                                    title: "Svuota selezioni",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        *values.write() = Vec::new();
                                        if let Some(h) = &on_change {
                                            h.call(Vec::new());
                                        }
                                    },
                                    Icon { kind: IconKind::X, size: 12 }
                                }
                            }
                            div {
                                class: if open { "sd-select-chevron sd-select-chevron--open" } else { "sd-select-chevron" },
                                Icon { kind: IconKind::ChevronDown, size: 14 }
                            }
                        }
                    }
                }

                if open {
                    div {
                        class: "sd-select-backdrop",
                        onclick: move |e| {
                            e.stop_propagation();
                            is_open.set(false);
                            search_query.set(String::new());
                        },
                    }
                    div {
                        class: "sd-select-menu {anim_class} {radius_class}",
                        onclick: move |e| e.stop_propagation(),

                        if searchable {
                            div {
                                class: "sd-select-search-container",
                                Icon { kind: IconKind::Search, size: 13, class: "sd-select-search-icon" }
                                input {
                                    class: "sd-select-search-input",
                                    placeholder: "Cerca opzioni...",
                                    value: "{search_query()}",
                                    autofocus: true,
                                    oninput: move |e| search_query.set(e.value()),
                                    onclick: move |e| e.stop_propagation(),
                                }
                                if !search_query().is_empty() {
                                    button {
                                        r#type: "button",
                                        class: "sd-select-search-clear",
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            search_query.set(String::new());
                                        },
                                        Icon { kind: IconKind::X, size: 11 }
                                    }
                                }
                            }
                        }

                        if filtered_items.is_empty() && !show_create_option {
                            div { class: "sd-select-empty", "Nessuna opzione trovata" }
                        } else {
                            for item in filtered_items.iter() {
                                {
                                    let item_val = item.value.clone();
                                    let item_lbl = item.label.clone();
                                    let item_desc = item.description.clone();
                                    let item_disabled = item.disabled;
                                    let is_selected = selected_vals.contains(&item.value);
                                    let selected_class = if is_selected { "sd-select-item--selected" } else { "" };
                                    let disabled_class = if item_disabled { "sd-select-item--disabled" } else { "" };

                                    rsx! {
                                        button {
                                            r#type: "button",
                                            key: "{item.value}",
                                            class: "sd-select-item {selected_class} {disabled_class}",
                                            disabled: item_disabled,
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                if !item_disabled {
                                                    let mut cur = values();
                                                    if let Some(pos) = cur.iter().position(|x| x == &item_val) {
                                                        cur.remove(pos);
                                                    } else {
                                                        cur.push(item_val.clone());
                                                    }
                                                    *values.write() = cur.clone();
                                                    if let Some(h) = &on_change {
                                                        h.call(cur);
                                                    }
                                                }
                                            },
                                            div { class: "sd-select-item-content",
                                                span { class: "sd-select-item-label", "{item_lbl}" }
                                                if let Some(desc_txt) = item_desc {
                                                    span { class: "sd-select-item-description", "{desc_txt}" }
                                                }
                                            }
                                            if is_selected {
                                                div { class: "sd-select-item-check",
                                                    Icon { kind: IconKind::Check, size: 14 }
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if show_create_option {
                                {
                                    let create_val = search_query();
                                    let create_val_display = create_val.clone();
                                    rsx! {
                                        button {
                                            r#type: "button",
                                            class: "sd-select-item sd-select-item--creatable",
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                let val_to_add = create_val.clone();
                                                let mut cur = values();
                                                if !cur.contains(&val_to_add) {
                                                    cur.push(val_to_add.clone());
                                                }
                                                *values.write() = cur.clone();
                                                if let Some(hc) = &on_create {
                                                    hc.call(val_to_add.clone());
                                                }
                                                if let Some(h) = &on_change {
                                                    h.call(cur);
                                                }
                                                search_query.set(String::new());
                                            },
                                            div { class: "sd-select-item-content",
                                                span { class: "sd-select-item-label", "+ Aggiungi \"{create_val_display}\"" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(err) = error {
                span { class: "sd-select-error", "{err}" }
            }
        }
    }
}

/// Componente `Combobox` per l'inserimento testuale libero combinato con suggerimenti di completamento istantanei.
#[component]
pub fn Combobox(
    /// Collezione di suggerimenti disponibili.
    data: Vec<SelectItem>,
    /// Valore testuale attualmente inserito collegato reattivamente.
    value: Signal<String>,
    /// Testo segnaposto del campo input.
    #[props(default)]
    placeholder: Option<String>,
    /// Etichetta descrittiva superiore del campo.
    #[props(default)]
    label: Option<String>,
    /// Testo di aiuto posizionato sotto l'etichetta.
    #[props(default)]
    description: Option<String>,
    /// Messaggio di errore di validazione.
    #[props(default)]
    error: Option<String>,
    /// Disabilita il campo input.
    #[props(default)]
    disabled: bool,
    /// Mostra un'icona di cancellazione rapida del testo quando non è vuoto.
    #[props(default)]
    clearable: bool,
    /// Mostra uno spinner animato.
    #[props(default)]
    loading: bool,
    /// Alone ottico perimetrale coordinato con il tema attivo.
    #[props(default)]
    glow: Option<bool>,
    /// Classi CSS o SCSS addizionali.
    #[props(default)]
    class: String,
    /// Callback invocata ad ogni variazione del valore testuale.
    #[props(default)]
    on_change: Option<EventHandler<String>>,
) -> Element {
    let mut is_open = use_signal(|| false);

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let radius_class = theme_val.radius_scale.md();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("select").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("select", &class);
    #[cfg(feature = "theme")]
    let glow_class = theme_val.resolve_elevation_class("select", glow);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();
    #[cfg(not(feature = "theme"))]
    let radius_class = "";
    #[cfg(not(feature = "theme"))]
    let glow_class = if glow.unwrap_or(false) {
        "sd-select-trigger--glow".to_string()
    } else {
        String::new()
    };

    #[cfg(feature = "animation")]
    let anim_class = crate::animation::Animations::DROPDOWN_ENTER;
    #[cfg(not(feature = "animation"))]
    let anim_class = "animate-dropdown-enter";

    let open = is_open();
    let current_text = value();
    let has_clear_btn = clearable && !current_text.is_empty() && !disabled;

    let q = current_text.trim().to_lowercase();
    let filtered_items: Vec<&SelectItem> = if q.is_empty() {
        data.iter().collect()
    } else {
        data.iter()
            .filter(|i| {
                i.label.to_lowercase().contains(&q)
                    || i.value.to_lowercase().contains(&q)
                    || i.description
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&q)
            })
            .collect()
    };

    let placeholder_str = placeholder
        .clone()
        .unwrap_or_else(|| "Digita o seleziona...".to_string());

    let open_trigger_class = if open { "sd-select-trigger--open" } else { "" };
    let error_trigger_class = if error.is_some() {
        "sd-select-trigger--error"
    } else {
        ""
    };
    let disabled_trigger_class = if disabled {
        "sd-select-trigger--disabled"
    } else {
        ""
    };
    let open_container_class = if open { "sd-select-container--open" } else { "" };

    rsx! {
        div {
            class: "sd-select-wrapper {class_modifiers} {class}",
            if let Some(lbl) = label {
                label { class: "sd-select-label", "{lbl}" }
            }
            if let Some(desc) = description {
                span { class: "sd-select-description", "{desc}" }
            }
            div {
                class: "sd-select-container {open_container_class}",
                div {
                    class: "sd-select-trigger {open_trigger_class} {error_trigger_class} {disabled_trigger_class} {base_override} {radius_class} {glow_class}",
                    input {
                        class: "sd-combobox-input",
                        r#type: "text",
                        placeholder: "{placeholder_str}",
                        disabled: disabled,
                        value: "{current_text}",
                        onfocus: move |_| is_open.set(true),
                        onclick: move |e| {
                            e.stop_propagation();
                            is_open.set(true);
                        },
                        oninput: move |e| {
                            let val = e.value();
                            *value.write() = val.clone();
                            is_open.set(true);
                            if let Some(h) = &on_change {
                                h.call(val);
                            }
                        },
                    }
                    div {
                        class: "sd-select-actions",
                        if loading {
                            div {
                                class: "sd-select-action-btn animate-spin",
                                Icon { kind: IconKind::Spinner, size: 14 }
                            }
                        } else {
                            if has_clear_btn {
                                button {
                                    r#type: "button",
                                    class: "sd-select-action-btn",
                                    title: "Cancella testo",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        *value.write() = String::new();
                                        if let Some(h) = &on_change {
                                            h.call(String::new());
                                        }
                                    },
                                    Icon { kind: IconKind::X, size: 12 }
                                }
                            }
                            button {
                                r#type: "button",
                                class: "sd-select-action-btn",
                                onclick: move |e| {
                                    e.stop_propagation();
                                    let next = !is_open();
                                    is_open.set(next);
                                },
                                div {
                                    class: if open { "sd-select-chevron sd-select-chevron--open" } else { "sd-select-chevron" },
                                    Icon { kind: IconKind::ChevronDown, size: 14 }
                                }
                            }
                        }
                    }
                }

                if open && !filtered_items.is_empty() {
                    div {
                        class: "sd-select-backdrop",
                        onclick: move |e| {
                            e.stop_propagation();
                            is_open.set(false);
                        },
                    }
                    div {
                        class: "sd-select-menu {anim_class} {radius_class}",
                        onclick: move |e| e.stop_propagation(),
                        for item in filtered_items.iter() {
                            {
                                let item_lbl = item.label.clone();
                                let item_desc = item.description.clone();
                                let item_disabled = item.disabled;
                                let is_selected = current_text == item.label || current_text == item.value;
                                let selected_class = if is_selected { "sd-select-item--selected" } else { "" };
                                let disabled_class = if item_disabled { "sd-select-item--disabled" } else { "" };

                                rsx! {
                                    button {
                                        r#type: "button",
                                        key: "{item.value}",
                                        class: "sd-select-item {selected_class} {disabled_class}",
                                        disabled: item_disabled,
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            if !item_disabled {
                                                *value.write() = item_lbl.clone();
                                                if let Some(h) = &on_change {
                                                    h.call(item_lbl.clone());
                                                }
                                                is_open.set(false);
                                            }
                                        },
                                        div { class: "sd-select-item-content",
                                            span { class: "sd-select-item-label", "{item_lbl}" }
                                            if let Some(desc_txt) = item_desc {
                                                span { class: "sd-select-item-description", "{desc_txt}" }
                                            }
                                        }
                                        if is_selected {
                                            div { class: "sd-select-item-check",
                                                Icon { kind: IconKind::Check, size: 14 }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(err) = error {
                span { class: "sd-select-error", "{err}" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_item_builders() {
        let item = SelectItem::new("val1", "Label 1")
            .with_description("Desc 1")
            .with_group("Group A");

        assert_eq!(item.value, "val1");
        assert_eq!(item.label, "Label 1");
        assert_eq!(item.description.as_deref(), Some("Desc 1"));
        assert_eq!(item.group.as_deref(), Some("Group A"));
        assert!(!item.disabled);

        let disabled_item = item.disabled();
        assert!(disabled_item.disabled);
    }

    #[test]
    fn test_select_item_conversions() {
        let item1: SelectItem = "option_1".into();
        assert_eq!(item1.value, "option_1");
        assert_eq!(item1.label, "option_1");

        let item2: SelectItem = ("key", "Display").into();
        assert_eq!(item2.value, "key");
        assert_eq!(item2.label, "Display");
    }
}
