use crate::icon::{Icon, IconKind};
use crate::locale::use_locale;
use dioxus::prelude::*;

/// Represents an individual item in a `Select`, `MultiSelect`, or `Combobox` control.
///
/// # Requirements
/// The `value` field should be unique across all items in a selection list.
/// Duplicate values may lead to improper selection highlighting and keyed list reconciliation issues.
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectItem {
    /// Unique identifier for the item (submitted with forms or stored in state).
    pub value: String,
    /// Human-readable label displayed in the UI.
    pub label: String,
    /// Optional secondary description (sub-label providing additional context).
    pub description: Option<String>,
    /// Indicates whether the item is disabled and non-selectable.
    pub disabled: bool,
    /// Optional grouping category (for grouped sections with headers).
    pub group: Option<String>,
}

impl SelectItem {
    /// Creates a new selectable item with a value and label.
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            disabled: false,
            group: None,
        }
    }

    /// Attaches an optional secondary description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Assigns the item to a specific category group.
    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Marks the item as disabled.
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

/// Helper to determine the display label and placeholder state for a selected value.
pub fn display_label(data: &[SelectItem], value: Option<&str>, placeholder: &str) -> (String, bool) {
    match value {
        Some(val) => {
            if let Some(item) = data.iter().find(|i| i.value == val) {
                (item.label.clone(), false)
            } else {
                (val.to_string(), false)
            }
        }
        None => (placeholder.to_string(), true),
    }
}

/// Helper to filter items based on a query string across label, value, and description.
pub fn filter_items(data: &[SelectItem], q: &str) -> Vec<SelectItem> {
    let q = q.trim().to_lowercase();
    if q.is_empty() {
        data.to_vec()
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
            .cloned()
            .collect()
    }
}

/// Reusable accessible option item component shared across Select, MultiSelect, and Combobox.
#[component]
fn SelectOption(
    id: String,
    item: SelectItem,
    selected: bool,
    active: bool,
    on_pick: EventHandler<String>,
) -> Element {
    let mut opt_mount = use_signal(|| None::<std::rc::Rc<MountedData>>);
    let item_val = item.value.clone();
    let item_disabled = item.disabled;
    let selected_class = if selected { "sd-select-item--selected" } else { "" };
    let disabled_class = if item_disabled { "sd-select-item--disabled" } else { "" };
    let active_class = if active { "sd-select-item--active" } else { "" };

    use_effect(move || {
        if active {
            if let Some(m) = opt_mount() {
                spawn(async move {
                    let _ = m.scroll_to(ScrollBehavior::Instant).await;
                });
            }
        }
    });

    rsx! {
        div {
            id: "{id}",
            role: "option",
            tabindex: "-1",
            "aria-selected": "{selected}",
            "aria-disabled": if item_disabled { "true" } else { "false" },
            class: "sd-select-item {selected_class} {disabled_class} {active_class}",
            onmounted: move |m| opt_mount.set(Some(m.data())),
            onmousedown: move |e| e.prevent_default(),
            onclick: move |e| {
                e.stop_propagation();
                if !item_disabled {
                    on_pick.call(item_val.clone());
                }
            },
            div {
                class: "sd-select-item-content",
                span { class: "sd-select-item-label", "{item.label}" }
                if let Some(desc_txt) = item.description {
                    span { class: "sd-select-item-description", "{desc_txt}" }
                }
            }
            if selected {
                div {
                    class: "sd-select-item-check",
                    Icon { kind: IconKind::Check, size: 14 }
                }
            }
        }
    }
}

/// Single-choice `Select` dropdown component with support for live search, option grouping,
/// runtime creatable values, and keyboard-operable WAI-ARIA combobox/listbox pattern.
#[component]
pub fn Select(
    /// Unique identifier for the select control and associated labels.
    #[props(default)]
    id: Option<String>,
    /// Collection of selectable items.
    data: Vec<SelectItem>,
    /// Currently selected value bound reactively.
    value: Signal<Option<String>>,
    /// Placeholder text displayed when no item is selected.
    #[props(default)]
    placeholder: Option<String>,
    /// Placeholder for the search input when `searchable` is true.
    #[props(default)]
    search_placeholder: Option<String>,
    /// Text displayed when no items match the search query.
    #[props(default)]
    nothing_found: Option<String>,
    /// Label text displayed above the control.
    #[props(default)]
    label: Option<String>,
    /// Help or description text displayed below the label.
    #[props(default)]
    description: Option<String>,
    /// Validation error message displayed below the control.
    #[props(default)]
    error: Option<String>,
    /// Disables all interactions with the control.
    #[props(default)]
    disabled: bool,
    /// Enables live filtering search bar inside the dropdown menu.
    #[props(default)]
    searchable: bool,
    /// Shows a quick clear button (X) when a value is selected.
    #[props(default)]
    clearable: bool,
    /// Allows the user to create and select custom values typed into search.
    #[props(default)]
    creatable: bool,
    /// Shows an animated loading spinner.
    #[props(default)]
    loading: bool,
    /// Perimetric optical glow coordinated with the active theme.
    #[props(default)]
    glow: Option<bool>,
    /// Additional CSS or SCSS classes.
    #[props(default)]
    class: String,
    /// Callback invoked when the selected value changes (or `None` on clear).
    #[props(default)]
    on_change: Option<EventHandler<Option<String>>>,
    /// Callback invoked when a custom value is created.
    #[props(default)]
    on_create: Option<EventHandler<String>>,
) -> Element {
    let locale = use_locale();
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(String::new);
    let mut active = use_signal(|| None::<usize>);
    let mut local_created = use_signal(Vec::<SelectItem>::new);
    let mut trigger_mount = use_signal(|| None::<std::rc::Rc<MountedData>>);

    #[cfg(debug_assertions)]
    {
        let mut seen = std::collections::HashSet::new();
        for item in &data {
            debug_assert!(
                seen.insert(&item.value),
                "Duplicate SelectItem value detected: '{}'. Each SelectItem must have a unique value.",
                item.value
            );
        }
    }

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

    let select_id = crate::a11y::use_unique_id("sd-select", id);
    let label_id = format!("{select_id}-label");
    let desc_id = format!("{select_id}-desc");
    let err_id = format!("{select_id}-err");
    let listbox_id = format!("{select_id}-listbox");

    let aria_describedby = if error.is_some() {
        Some(err_id.clone())
    } else if description.is_some() {
        Some(desc_id.clone())
    } else {
        None
    };

    let open = is_open();
    let current_val = value();

    // Merge data with locally created items so created values display properly
    let mut all_data = data.clone();
    for created in local_created.read().iter() {
        if !all_data.iter().any(|item| item.value == created.value) {
            all_data.push(created.clone());
        }
    }

    let placeholder_text = placeholder.unwrap_or_else(|| locale.select_placeholder.clone());
    let (display_lbl, is_placeholder) = display_label(&all_data, current_val.as_deref(), &placeholder_text);

    let open_trigger_class = if open { "sd-select-trigger--open" } else { "" };
    let error_trigger_class = if error.is_some() { "sd-select-trigger--error" } else { "" };
    let disabled_trigger_class = if disabled { "sd-select-trigger--disabled" } else { "" };
    let open_container_class = if open { "sd-select-container--open" } else { "" };

    let q = search_query().trim().to_lowercase();
    let filtered_items = filter_items(&all_data, &q);

    let exact_match = !q.is_empty()
        && all_data
            .iter()
            .any(|i| i.label.to_lowercase() == q || i.value.to_lowercase() == q);
    let show_create_option = creatable && !q.is_empty() && !exact_match;

    let non_disabled_indices: Vec<usize> = filtered_items
        .iter()
        .enumerate()
        .filter(|(_, item)| !item.disabled)
        .map(|(i, _)| i)
        .collect();

    let active_opt_id = active().map(|idx| {
        if idx < filtered_items.len() {
            format!("{select_id}-opt-{idx}")
        } else {
            format!("{select_id}-opt-create")
        }
    });

    let mut groups: Vec<Option<String>> = Vec::new();
    for item in &filtered_items {
        if !groups.contains(&item.group) {
            groups.push(item.group.clone());
        }
    }

    let has_clear_btn = clearable && current_val.is_some() && !disabled;

    let focus_trigger = move || {
        if let Some(m) = trigger_mount() {
            spawn(async move {
                let _ = m.set_focus(true).await;
            });
        }
    };

    let search_placeholder_text = search_placeholder.unwrap_or_else(|| locale.search_placeholder.clone());
    let nothing_found_text = nothing_found.unwrap_or_else(|| locale.nothing_found.clone());

    let key_items = filtered_items.clone();
    let show_create_for_keys = show_create_option;
    let non_disabled_for_keys = non_disabled_indices.clone();

    let handle_keydown = move |e: KeyboardEvent| {
        if disabled {
            return;
        }

        match e.key() {
            Key::ArrowDown => {
                e.prevent_default();
                if !is_open() {
                    is_open.set(true);
                    if let Some(&first) = non_disabled_for_keys.first() {
                        active.set(Some(first));
                    }
                } else {
                    let next = match active() {
                        None => non_disabled_for_keys.first().copied(),
                        Some(curr) => {
                            let mut found = None;
                            for &idx in &non_disabled_for_keys {
                                if idx > curr {
                                    found = Some(idx);
                                    break;
                                }
                            }
                            if found.is_none() && show_create_for_keys && curr < key_items.len() {
                                found = Some(key_items.len());
                            }
                            found.or_else(|| non_disabled_for_keys.first().copied())
                        }
                    };
                    active.set(next);
                }
            }
            Key::ArrowUp => {
                e.prevent_default();
                if !is_open() {
                    is_open.set(true);
                    if let Some(&last) = non_disabled_for_keys.last() {
                        active.set(Some(last));
                    }
                } else {
                    let prev = match active() {
                        None => non_disabled_for_keys.last().copied(),
                        Some(curr) => {
                            let mut found = None;
                            for &idx in non_disabled_for_keys.iter().rev() {
                                if idx < curr {
                                    found = Some(idx);
                                    break;
                                }
                            }
                            found.or_else(|| {
                                if show_create_for_keys {
                                    Some(key_items.len())
                                } else {
                                    non_disabled_for_keys.last().copied()
                                }
                            })
                        }
                    };
                    active.set(prev);
                }
            }
            Key::Home => {
                if is_open() {
                    e.prevent_default();
                    active.set(non_disabled_for_keys.first().copied());
                }
            }
            Key::End => {
                if is_open() {
                    e.prevent_default();
                    if show_create_for_keys {
                        active.set(Some(key_items.len()));
                    } else {
                        active.set(non_disabled_for_keys.last().copied());
                    }
                }
            }
            Key::Enter => {
                e.prevent_default();
                if is_open() {
                    if let Some(idx) = active() {
                        if idx < key_items.len() {
                            let item = &key_items[idx];
                            if !item.disabled {
                                let chosen_val = item.value.clone();
                                *value.write() = Some(chosen_val.clone());
                                if let Some(h) = &on_change {
                                    h.call(Some(chosen_val));
                                }
                                is_open.set(false);
                                search_query.set(String::new());
                                active.set(None);
                                focus_trigger();
                            }
                        } else if show_create_for_keys {
                            let trimmed = search_query().trim().to_string();
                            if !trimmed.is_empty() {
                                let new_item = SelectItem::new(trimmed.clone(), trimmed.clone());
                                local_created.write().push(new_item);
                                *value.write() = Some(trimmed.clone());
                                if let Some(hc) = &on_create {
                                    hc.call(trimmed.clone());
                                }
                                if let Some(h) = &on_change {
                                    h.call(Some(trimmed));
                                }
                                is_open.set(false);
                                search_query.set(String::new());
                                active.set(None);
                                focus_trigger();
                            }
                        }
                    }
                } else {
                    is_open.set(true);
                    if let Some(&first) = non_disabled_for_keys.first() {
                        active.set(Some(first));
                    }
                }
            }
            Key::Character(ref s) if s == " " => {
                if !searchable || !is_open() {
                    e.prevent_default();
                    if is_open() {
                        if let Some(idx) = active() {
                            if idx < key_items.len() {
                                let item = &key_items[idx];
                                if !item.disabled {
                                    let chosen_val = item.value.clone();
                                    *value.write() = Some(chosen_val.clone());
                                    if let Some(h) = &on_change {
                                        h.call(Some(chosen_val));
                                    }
                                    is_open.set(false);
                                    search_query.set(String::new());
                                    active.set(None);
                                    focus_trigger();
                                }
                            }
                        }
                    } else {
                        is_open.set(true);
                        if let Some(&first) = non_disabled_for_keys.first() {
                            active.set(Some(first));
                        }
                    }
                }
            }
            Key::Escape => {
                if is_open() {
                    e.prevent_default();
                    is_open.set(false);
                    search_query.set(String::new());
                    active.set(None);
                    focus_trigger();
                }
            }
            Key::Tab if is_open() => {
                is_open.set(false);
                search_query.set(String::new());
                active.set(None);
            }
            Key::Character(ref s) if !searchable && !is_open() => {
                let target = s.to_lowercase();
                if let Some((idx, item)) = key_items.iter().enumerate().find(|(_, item)| {
                    !item.disabled && item.label.to_lowercase().starts_with(&target)
                }) {
                    let chosen_val = item.value.clone();
                    *value.write() = Some(chosen_val.clone());
                    if let Some(h) = &on_change {
                        h.call(Some(chosen_val));
                    }
                    active.set(Some(idx));
                }
            }
            _ => {}
        }
    };

    rsx! {
        div {
            class: "sd-select-wrapper {class_modifiers} {class}",
            if let Some(lbl) = &label {
                label {
                    id: "{label_id}",
                    class: "sd-select-label",
                    "{lbl}"
                }
            }
            if let Some(desc) = &description {
                span {
                    id: "{desc_id}",
                    class: "sd-select-description",
                    "{desc}"
                }
            }
            div {
                class: "sd-select-container {open_container_class}",
                div {
                    role: "combobox",
                    id: "{select_id}",
                    tabindex: if disabled { "-1" } else { "0" },
                    "aria-haspopup": "listbox",
                    "aria-expanded": "{open}",
                    "aria-controls": "{listbox_id}",
                    "aria-labelledby": if label.is_some() { Some(label_id.as_str()) } else { None },
                    "aria-describedby": aria_describedby.as_deref(),
                    "aria-disabled": if disabled { "true" } else { "false" },
                    "aria-invalid": error.is_some(),
                    "aria-activedescendant": active_opt_id.as_deref(),
                    class: "sd-select-trigger {open_trigger_class} {error_trigger_class} {disabled_trigger_class} {base_override} {radius_class} {glow_class}",
                    onmounted: move |m| trigger_mount.set(Some(m.data())),
                    onclick: move |_| {
                        if !disabled {
                            let next = !is_open();
                            is_open.set(next);
                            if !next {
                                search_query.set(String::new());
                                active.set(None);
                            }
                        }
                    },
                    onkeydown: handle_keydown,
                    div {
                        class: if is_placeholder { "sd-select-value sd-select-value--placeholder" } else { "sd-select-value" },
                        "{display_lbl}"
                    }
                    div {
                        class: "sd-select-actions",
                        if loading {
                            div {
                                class: "sd-select-action-btn",
                                Icon { kind: IconKind::Spinner, size: 14 }
                            }
                        } else {
                            if has_clear_btn {
                                button {
                                    r#type: "button",
                                    class: "sd-select-action-btn",
                                    "aria-label": "{locale.clear_selection}",
                                    title: "{locale.clear_selection}",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        *value.write() = None;
                                        if let Some(h) = &on_change {
                                            h.call(None);
                                        }
                                        focus_trigger();
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
                            active.set(None);
                        },
                    }
                    div {
                        role: "listbox",
                        id: "{listbox_id}",
                        "aria-labelledby": if label.is_some() { Some(label_id.as_str()) } else { None },
                        class: "sd-select-menu {anim_class} {radius_class}",
                        onclick: move |e| e.stop_propagation(),

                        if searchable {
                            div {
                                class: "sd-select-search-container",
                                Icon { kind: IconKind::Search, size: 13, class: "sd-select-search-icon" }
                                input {
                                    class: "sd-select-search-input",
                                    placeholder: "{search_placeholder_text}",
                                    value: "{search_query()}",
                                    autofocus: true,
                                    oninput: move |e| {
                                        search_query.set(e.value());
                                        active.set(None);
                                    },
                                    onclick: move |e| e.stop_propagation(),
                                }
                                if !search_query().is_empty() {
                                    button {
                                        r#type: "button",
                                        class: "sd-select-search-clear",
                                        "aria-label": "{locale.clear_text}",
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            search_query.set(String::new());
                                            active.set(None);
                                        },
                                        Icon { kind: IconKind::X, size: 11 }
                                    }
                                }
                            }
                        }

                        if filtered_items.is_empty() && !show_create_option {
                            div { class: "sd-select-empty", "{nothing_found_text}" }
                        } else {
                            for grp in groups.iter() {
                                {
                                    let grp_name = grp.clone();
                                    let grp_key = grp_name.clone().unwrap_or_else(|| "__default__".to_string());
                                    let grp_header_id = format!("{listbox_id}-grp-{grp_key}");
                                    rsx! {
                                        div {
                                            key: "{grp_key}",
                                            role: "group",
                                            "aria-labelledby": if grp_name.is_some() { Some(grp_header_id.as_str()) } else { None },
                                            if let Some(name) = &grp_name {
                                                div {
                                                    id: "{grp_header_id}",
                                                    role: "presentation",
                                                    class: "sd-select-group-header",
                                                    "{name}"
                                                }
                                            }
                                            for (idx, item) in filtered_items.iter().enumerate().filter(|(_, i)| &i.group == grp) {
                                                SelectOption {
                                                    key: "{item.value}",
                                                    id: format!("{select_id}-opt-{idx}"),
                                                    item: item.clone(),
                                                    selected: current_val.as_ref() == Some(&item.value),
                                                    active: active() == Some(idx),
                                                    on_pick: move |val: String| {
                                                        *value.write() = Some(val.clone());
                                                        if let Some(h) = &on_change {
                                                            h.call(Some(val));
                                                        }
                                                        is_open.set(false);
                                                        search_query.set(String::new());
                                                        active.set(None);
                                                        focus_trigger();
                                                    },
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if show_create_option {
                                {
                                    let create_idx = filtered_items.len();
                                    let is_active_create = active() == Some(create_idx);
                                    let active_create_class = if is_active_create { "sd-select-item--active" } else { "" };
                                    let create_id = format!("{select_id}-opt-create");
                                    let create_val = search_query().trim().to_string();
                                    let create_label_str = locale.format_create(&create_val);
                                    let create_val_captured = create_val.clone();

                                    rsx! {
                                        div {
                                            id: "{create_id}",
                                            role: "option",
                                            tabindex: "-1",
                                            "aria-selected": "false",
                                            class: "sd-select-item sd-select-item--creatable {active_create_class}",
                                            onmousedown: move |e| e.prevent_default(),
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                let val_to_add = create_val_captured.clone();
                                                if !val_to_add.is_empty() {
                                                    let new_item = SelectItem::new(val_to_add.clone(), val_to_add.clone());
                                                    local_created.write().push(new_item);
                                                    *value.write() = Some(val_to_add.clone());
                                                    if let Some(hc) = &on_create {
                                                        hc.call(val_to_add.clone());
                                                    }
                                                    if let Some(h) = &on_change {
                                                        h.call(Some(val_to_add));
                                                    }
                                                    is_open.set(false);
                                                    search_query.set(String::new());
                                                    active.set(None);
                                                    focus_trigger();
                                                }
                                            },
                                            div {
                                                class: "sd-select-item-content",
                                                span { class: "sd-select-item-label", "{create_label_str}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(err) = &error {
                span {
                    id: "{err_id}",
                    class: "sd-select-error",
                    "{err}"
                }
            }
        }
    }
}

/// Multi-choice `MultiSelect` component with dismissible tag chips, live search,
/// and keyboard-operable WAI-ARIA combobox/listbox pattern.
#[component]
pub fn MultiSelect(
    /// Unique identifier for the multi-select control.
    #[props(default)]
    id: Option<String>,
    /// Collection of selectable items.
    data: Vec<SelectItem>,
    /// Vector of currently selected values bound reactively.
    values: Signal<Vec<String>>,
    /// Placeholder text when no values are selected.
    #[props(default)]
    placeholder: Option<String>,
    /// Placeholder for the search input.
    #[props(default)]
    search_placeholder: Option<String>,
    /// Text displayed when no items match the search query.
    #[props(default)]
    nothing_found: Option<String>,
    /// Label text displayed above the control.
    #[props(default)]
    label: Option<String>,
    /// Help or description text displayed below the label.
    #[props(default)]
    description: Option<String>,
    /// Validation error message.
    #[props(default)]
    error: Option<String>,
    /// Disables all interactions with the control.
    #[props(default)]
    disabled: bool,
    /// Enables live filtering search bar.
    #[props(default)]
    searchable: bool,
    /// Shows a clear button (X) to remove all selected values.
    #[props(default)]
    clearable: bool,
    /// Allows the user to add custom values typed into search.
    #[props(default)]
    creatable: bool,
    /// Shows an animated spinner.
    #[props(default)]
    loading: bool,
    /// Maximum number of chips/tags rendered in the trigger.
    #[props(default)]
    max_selected_display: Option<usize>,
    /// Perimetric optical glow.
    #[props(default)]
    glow: Option<bool>,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Callback invoked when the list of selected values changes.
    #[props(default)]
    on_change: Option<EventHandler<Vec<String>>>,
    /// Callback invoked when a new custom tag is created.
    #[props(default)]
    on_create: Option<EventHandler<String>>,
) -> Element {
    let locale = use_locale();
    let mut is_open = use_signal(|| false);
    let mut search_query = use_signal(String::new);
    let mut active = use_signal(|| None::<usize>);
    let mut local_created = use_signal(Vec::<SelectItem>::new);
    let mut trigger_mount = use_signal(|| None::<std::rc::Rc<MountedData>>);

    #[cfg(debug_assertions)]
    {
        let mut seen = std::collections::HashSet::new();
        for item in &data {
            debug_assert!(
                seen.insert(&item.value),
                "Duplicate SelectItem value detected: '{}'. Each SelectItem must have a unique value.",
                item.value
            );
        }
    }

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

    let select_id = crate::a11y::use_unique_id("sd-multiselect", id);
    let label_id = format!("{select_id}-label");
    let desc_id = format!("{select_id}-desc");
    let err_id = format!("{select_id}-err");
    let listbox_id = format!("{select_id}-listbox");

    let aria_describedby = if error.is_some() {
        Some(err_id.clone())
    } else if description.is_some() {
        Some(desc_id.clone())
    } else {
        None
    };

    let open = is_open();
    let selected_vals = values();
    let is_empty = selected_vals.is_empty();

    let mut all_data = data.clone();
    for created in local_created.read().iter() {
        if !all_data.iter().any(|item| item.value == created.value) {
            all_data.push(created.clone());
        }
    }

    let open_trigger_class = if open { "sd-select-trigger--open" } else { "" };
    let error_trigger_class = if error.is_some() { "sd-select-trigger--error" } else { "" };
    let disabled_trigger_class = if disabled { "sd-select-trigger--disabled" } else { "" };
    let open_container_class = if open { "sd-select-container--open" } else { "" };

    let q = search_query().trim().to_lowercase();
    let filtered_items = filter_items(&all_data, &q);

    let exact_match = !q.is_empty()
        && all_data
            .iter()
            .any(|i| i.label.to_lowercase() == q || i.value.to_lowercase() == q);
    let show_create_option = creatable && !q.is_empty() && !exact_match;

    let non_disabled_indices: Vec<usize> = filtered_items
        .iter()
        .enumerate()
        .filter(|(_, item)| !item.disabled)
        .map(|(i, _)| i)
        .collect();

    let active_opt_id = active().map(|idx| {
        if idx < filtered_items.len() {
            format!("{select_id}-opt-{idx}")
        } else {
            format!("{select_id}-opt-create")
        }
    });

    let mut groups: Vec<Option<String>> = Vec::new();
    for item in &filtered_items {
        if !groups.contains(&item.group) {
            groups.push(item.group.clone());
        }
    }

    let has_clear_btn = clearable && !is_empty && !disabled;

    let max_display = max_selected_display.unwrap_or(usize::MAX);
    let displayed_slice: Vec<String> = selected_vals.iter().take(max_display).cloned().collect();
    let overflow_count = if selected_vals.len() > max_display {
        selected_vals.len() - max_display
    } else {
        0
    };

    let placeholder_text = placeholder.unwrap_or_else(|| locale.multiselect_placeholder.clone());
    let search_placeholder_text = search_placeholder.unwrap_or_else(|| locale.search_placeholder.clone());
    let nothing_found_text = nothing_found.unwrap_or_else(|| locale.nothing_found.clone());

    let focus_trigger = move || {
        if let Some(m) = trigger_mount() {
            spawn(async move {
                let _ = m.set_focus(true).await;
            });
        }
    };

    let key_items = filtered_items.clone();
    let show_create_for_keys = show_create_option;
    let non_disabled_for_keys = non_disabled_indices.clone();

    let handle_keydown = move |e: KeyboardEvent| {
        if disabled {
            return;
        }

        match e.key() {
            Key::ArrowDown => {
                e.prevent_default();
                if !is_open() {
                    is_open.set(true);
                    if let Some(&first) = non_disabled_for_keys.first() {
                        active.set(Some(first));
                    }
                } else {
                    let next = match active() {
                        None => non_disabled_for_keys.first().copied(),
                        Some(curr) => {
                            let mut found = None;
                            for &idx in &non_disabled_for_keys {
                                if idx > curr {
                                    found = Some(idx);
                                    break;
                                }
                            }
                            if found.is_none() && show_create_for_keys && curr < key_items.len() {
                                found = Some(key_items.len());
                            }
                            found.or_else(|| non_disabled_for_keys.first().copied())
                        }
                    };
                    active.set(next);
                }
            }
            Key::ArrowUp => {
                e.prevent_default();
                if !is_open() {
                    is_open.set(true);
                    if let Some(&last) = non_disabled_for_keys.last() {
                        active.set(Some(last));
                    }
                } else {
                    let prev = match active() {
                        None => non_disabled_for_keys.last().copied(),
                        Some(curr) => {
                            let mut found = None;
                            for &idx in non_disabled_for_keys.iter().rev() {
                                if idx < curr {
                                    found = Some(idx);
                                    break;
                                }
                            }
                            found.or_else(|| {
                                if show_create_for_keys {
                                    Some(key_items.len())
                                } else {
                                    non_disabled_for_keys.last().copied()
                                }
                            })
                        }
                    };
                    active.set(prev);
                }
            }
            Key::Home => {
                if is_open() {
                    e.prevent_default();
                    active.set(non_disabled_for_keys.first().copied());
                }
            }
            Key::End => {
                if is_open() {
                    e.prevent_default();
                    if show_create_for_keys {
                        active.set(Some(key_items.len()));
                    } else {
                        active.set(non_disabled_for_keys.last().copied());
                    }
                }
            }
            Key::Enter => {
                e.prevent_default();
                if is_open() {
                    if let Some(idx) = active() {
                        if idx < key_items.len() {
                            let item = &key_items[idx];
                            if !item.disabled {
                                let item_val = item.value.clone();
                                let mut cur = values();
                                if let Some(pos) = cur.iter().position(|x| x == &item_val) {
                                    cur.remove(pos);
                                } else {
                                    cur.push(item_val);
                                }
                                *values.write() = cur.clone();
                                if let Some(h) = &on_change {
                                    h.call(cur);
                                }
                            }
                        } else if show_create_for_keys {
                            let trimmed = search_query().trim().to_string();
                            if !trimmed.is_empty() {
                                let new_item = SelectItem::new(trimmed.clone(), trimmed.clone());
                                local_created.write().push(new_item);
                                let mut cur = values();
                                if !cur.contains(&trimmed) {
                                    cur.push(trimmed.clone());
                                }
                                *values.write() = cur.clone();
                                if let Some(hc) = &on_create {
                                    hc.call(trimmed.clone());
                                }
                                if let Some(h) = &on_change {
                                    h.call(cur);
                                }
                                search_query.set(String::new());
                                active.set(None);
                            }
                        }
                    }
                } else {
                    is_open.set(true);
                    if let Some(&first) = non_disabled_for_keys.first() {
                        active.set(Some(first));
                    }
                }
            }
            Key::Character(ref s) if s == " " => {
                if !searchable || !is_open() {
                    e.prevent_default();
                    if is_open() {
                        if let Some(idx) = active() {
                            if idx < key_items.len() {
                                let item = &key_items[idx];
                                if !item.disabled {
                                    let item_val = item.value.clone();
                                    let mut cur = values();
                                    if let Some(pos) = cur.iter().position(|x| x == &item_val) {
                                        cur.remove(pos);
                                    } else {
                                        cur.push(item_val);
                                    }
                                    *values.write() = cur.clone();
                                    if let Some(h) = &on_change {
                                        h.call(cur);
                                    }
                                }
                            }
                        }
                    } else {
                        is_open.set(true);
                        if let Some(&first) = non_disabled_for_keys.first() {
                            active.set(Some(first));
                        }
                    }
                }
            }
            Key::Escape => {
                if is_open() {
                    e.prevent_default();
                    is_open.set(false);
                    search_query.set(String::new());
                    active.set(None);
                    focus_trigger();
                }
            }
            Key::Tab if is_open() => {
                is_open.set(false);
                search_query.set(String::new());
                active.set(None);
            }
            _ => {}
        }
    };

    rsx! {
        div {
            class: "sd-select-wrapper {class_modifiers} {class}",
            if let Some(lbl) = &label {
                label {
                    id: "{label_id}",
                    class: "sd-select-label",
                    "{lbl}"
                }
            }
            if let Some(desc) = &description {
                span {
                    id: "{desc_id}",
                    class: "sd-select-description",
                    "{desc}"
                }
            }
            div {
                class: "sd-select-container {open_container_class}",
                div {
                    role: "combobox",
                    id: "{select_id}",
                    tabindex: if disabled { "-1" } else { "0" },
                    "aria-haspopup": "listbox",
                    "aria-expanded": "{open}",
                    "aria-controls": "{listbox_id}",
                    "aria-labelledby": if label.is_some() { Some(label_id.as_str()) } else { None },
                    "aria-describedby": aria_describedby.as_deref(),
                    "aria-disabled": if disabled { "true" } else { "false" },
                    "aria-invalid": error.is_some(),
                    "aria-activedescendant": active_opt_id.as_deref(),
                    class: "sd-select-trigger {open_trigger_class} {error_trigger_class} {disabled_trigger_class} {base_override} {radius_class} {glow_class}",
                    onmounted: move |m| trigger_mount.set(Some(m.data())),
                    onclick: move |_| {
                        if !disabled {
                            let next = !is_open();
                            is_open.set(next);
                            if !next {
                                search_query.set(String::new());
                                active.set(None);
                            }
                        }
                    },
                    onkeydown: handle_keydown,
                    div {
                        class: "sd-select-value",
                        if is_empty {
                            span {
                                class: "sd-select-value--placeholder",
                                "{placeholder_text}"
                            }
                        } else {
                            div {
                                class: "sd-select-tags",
                                for val in displayed_slice.iter() {
                                    {
                                        let item_label = all_data
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
                                                        "aria-label": format!("{}: {}", locale.remove, item_label),
                                                        title: "{locale.remove}",
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
                                class: "sd-select-action-btn",
                                Icon { kind: IconKind::Spinner, size: 14 }
                            }
                        } else {
                            if has_clear_btn {
                                button {
                                    r#type: "button",
                                    class: "sd-select-action-btn",
                                    "aria-label": "{locale.clear_selection}",
                                    title: "{locale.clear_selection}",
                                    onclick: move |e| {
                                        e.stop_propagation();
                                        *values.write() = Vec::new();
                                        if let Some(h) = &on_change {
                                            h.call(Vec::new());
                                        }
                                        focus_trigger();
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
                            active.set(None);
                        },
                    }
                    div {
                        role: "listbox",
                        id: "{listbox_id}",
                        "aria-labelledby": if label.is_some() { Some(label_id.as_str()) } else { None },
                        "aria-multiselectable": "true",
                        class: "sd-select-menu {anim_class} {radius_class}",
                        onclick: move |e| e.stop_propagation(),

                        if searchable {
                            div {
                                class: "sd-select-search-container",
                                Icon { kind: IconKind::Search, size: 13, class: "sd-select-search-icon" }
                                input {
                                    class: "sd-select-search-input",
                                    placeholder: "{search_placeholder_text}",
                                    value: "{search_query()}",
                                    autofocus: true,
                                    oninput: move |e| {
                                        search_query.set(e.value());
                                        active.set(None);
                                    },
                                    onclick: move |e| e.stop_propagation(),
                                }
                                if !search_query().is_empty() {
                                    button {
                                        r#type: "button",
                                        class: "sd-select-search-clear",
                                        "aria-label": "{locale.clear_text}",
                                        onclick: move |e| {
                                            e.stop_propagation();
                                            search_query.set(String::new());
                                            active.set(None);
                                        },
                                        Icon { kind: IconKind::X, size: 11 }
                                    }
                                }
                            }
                        }

                        if filtered_items.is_empty() && !show_create_option {
                            div { class: "sd-select-empty", "{nothing_found_text}" }
                        } else {
                            for grp in groups.iter() {
                                {
                                    let grp_name = grp.clone();
                                    let grp_key = grp_name.clone().unwrap_or_else(|| "__default__".to_string());
                                    let grp_header_id = format!("{listbox_id}-grp-{grp_key}");
                                    rsx! {
                                        div {
                                            key: "{grp_key}",
                                            role: "group",
                                            "aria-labelledby": if grp_name.is_some() { Some(grp_header_id.as_str()) } else { None },
                                            if let Some(name) = &grp_name {
                                                div {
                                                    id: "{grp_header_id}",
                                                    role: "presentation",
                                                    class: "sd-select-group-header",
                                                    "{name}"
                                                }
                                            }
                                            for (idx, item) in filtered_items.iter().enumerate().filter(|(_, i)| &i.group == grp) {
                                                SelectOption {
                                                    key: "{item.value}",
                                                    id: format!("{select_id}-opt-{idx}"),
                                                    item: item.clone(),
                                                    selected: selected_vals.contains(&item.value),
                                                    active: active() == Some(idx),
                                                    on_pick: move |val: String| {
                                                        let mut cur = values();
                                                        if let Some(pos) = cur.iter().position(|x| x == &val) {
                                                            cur.remove(pos);
                                                        } else {
                                                            cur.push(val);
                                                        }
                                                        *values.write() = cur.clone();
                                                        if let Some(h) = &on_change {
                                                            h.call(cur);
                                                        }
                                                    },
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if show_create_option {
                                {
                                    let create_idx = filtered_items.len();
                                    let is_active_create = active() == Some(create_idx);
                                    let active_create_class = if is_active_create { "sd-select-item--active" } else { "" };
                                    let create_id = format!("{select_id}-opt-create");
                                    let create_val = search_query().trim().to_string();
                                    let create_label_str = locale.format_add(&create_val);
                                    let create_val_captured = create_val.clone();

                                    rsx! {
                                        div {
                                            id: "{create_id}",
                                            role: "option",
                                            tabindex: "-1",
                                            "aria-selected": "false",
                                            class: "sd-select-item sd-select-item--creatable {active_create_class}",
                                            onmousedown: move |e| e.prevent_default(),
                                            onclick: move |e| {
                                                e.stop_propagation();
                                                let val_to_add = create_val_captured.clone();
                                                if !val_to_add.is_empty() {
                                                    let new_item = SelectItem::new(val_to_add.clone(), val_to_add.clone());
                                                    local_created.write().push(new_item);
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
                                                    active.set(None);
                                                }
                                            },
                                            div {
                                                class: "sd-select-item-content",
                                                span { class: "sd-select-item-label", "{create_label_str}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(err) = &error {
                span {
                    id: "{err_id}",
                    class: "sd-select-error",
                    "{err}"
                }
            }
        }
    }
}

/// Combobox component combining free-text input with instant autocomplete suggestions
/// adhering to the APG Editable Combobox with List Autocomplete pattern.
#[component]
pub fn Combobox(
    /// Unique identifier for the combobox input.
    #[props(default)]
    id: Option<String>,
    /// Collection of available suggestions.
    data: Vec<SelectItem>,
    /// Current text value bound reactively.
    value: Signal<String>,
    /// Placeholder text for the input field.
    #[props(default)]
    placeholder: Option<String>,
    /// Label text displayed above the input.
    #[props(default)]
    label: Option<String>,
    /// Help text displayed below the label.
    #[props(default)]
    description: Option<String>,
    /// Validation error message.
    #[props(default)]
    error: Option<String>,
    /// Disables the input field.
    #[props(default)]
    disabled: bool,
    /// Shows a quick clear button (X) when the text is not empty.
    #[props(default)]
    clearable: bool,
    /// Shows an animated spinner.
    #[props(default)]
    loading: bool,
    /// Perimetric optical glow.
    #[props(default)]
    glow: Option<bool>,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Callback invoked on text change.
    #[props(default)]
    on_change: Option<EventHandler<String>>,
) -> Element {
    let locale = use_locale();
    let mut is_open = use_signal(|| false);
    let mut active = use_signal(|| None::<usize>);
    let mut escape_count = use_signal(|| 0usize);

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

    let combobox_id = crate::a11y::use_unique_id("sd-combobox", id);
    let input_id = format!("{combobox_id}-input");
    let desc_id = format!("{combobox_id}-desc");
    let err_id = format!("{combobox_id}-err");
    let listbox_id = format!("{combobox_id}-listbox");

    let aria_describedby = if error.is_some() {
        Some(err_id.clone())
    } else if description.is_some() {
        Some(desc_id.clone())
    } else {
        None
    };

    let open = is_open();
    let current_text = value();
    let has_clear_btn = clearable && !current_text.is_empty() && !disabled;

    let q = current_text.trim().to_lowercase();
    let filtered_items = filter_items(&data, &q);

    let non_disabled_indices: Vec<usize> = filtered_items
        .iter()
        .enumerate()
        .filter(|(_, item)| !item.disabled)
        .map(|(i, _)| i)
        .collect();

    let active_opt_id = active().map(|idx| format!("{combobox_id}-opt-{idx}"));

    let placeholder_str = placeholder.unwrap_or_else(|| locale.combobox_placeholder.clone());

    let open_trigger_class = if open { "sd-select-trigger--open" } else { "" };
    let error_trigger_class = if error.is_some() { "sd-select-trigger--error" } else { "" };
    let disabled_trigger_class = if disabled { "sd-select-trigger--disabled" } else { "" };
    let open_container_class = if open { "sd-select-container--open" } else { "" };

    let key_items = filtered_items.clone();
    let non_disabled_for_keys = non_disabled_indices.clone();

    let handle_input_keydown = move |e: KeyboardEvent| {
        if disabled {
            return;
        }

        match e.key() {
            Key::ArrowDown => {
                e.prevent_default();
                if !is_open() {
                    is_open.set(true);
                    active.set(non_disabled_for_keys.first().copied());
                } else {
                    let next = match active() {
                        None => non_disabled_for_keys.first().copied(),
                        Some(curr) => {
                            let mut found = None;
                            for &idx in &non_disabled_for_keys {
                                if idx > curr {
                                    found = Some(idx);
                                    break;
                                }
                            }
                            found.or_else(|| non_disabled_for_keys.first().copied())
                        }
                    };
                    active.set(next);
                }
            }
            Key::ArrowUp => {
                e.prevent_default();
                if !is_open() {
                    is_open.set(true);
                    active.set(non_disabled_for_keys.last().copied());
                } else {
                    let prev = match active() {
                        None => non_disabled_for_keys.last().copied(),
                        Some(curr) => {
                            let mut found = None;
                            for &idx in non_disabled_for_keys.iter().rev() {
                                if idx < curr {
                                    found = Some(idx);
                                    break;
                                }
                            }
                            found.or_else(|| non_disabled_for_keys.last().copied())
                        }
                    };
                    active.set(prev);
                }
            }
            Key::Enter => {
                if is_open() {
                    if let Some(idx) = active() {
                        e.prevent_default();
                        if idx < key_items.len() {
                            let item = &key_items[idx];
                            if !item.disabled {
                                let item_label = item.label.clone();
                                *value.write() = item_label.clone();
                                if let Some(h) = &on_change {
                                    h.call(item_label);
                                }
                                is_open.set(false);
                                active.set(None);
                            }
                        }
                    }
                }
            }
            Key::Escape => {
                e.prevent_default();
                if is_open() {
                    is_open.set(false);
                    active.set(None);
                    escape_count.set(1);
                } else if escape_count() > 0 {
                    *value.write() = String::new();
                    if let Some(h) = &on_change {
                        h.call(String::new());
                    }
                    escape_count.set(0);
                }
            }
            Key::Tab if is_open() => {
                is_open.set(false);
                active.set(None);
            }
            _ => {}
        }
    };

    rsx! {
        div {
            class: "sd-select-wrapper {class_modifiers} {class}",
            if let Some(lbl) = &label {
                label {
                    r#for: "{input_id}",
                    class: "sd-select-label",
                    "{lbl}"
                }
            }
            if let Some(desc) = &description {
                span {
                    id: "{desc_id}",
                    class: "sd-select-description",
                    "{desc}"
                }
            }
            div {
                class: "sd-select-container {open_container_class}",
                onfocusout: move |_| {
                    is_open.set(false);
                    active.set(None);
                },
                div {
                    class: "sd-select-trigger {open_trigger_class} {error_trigger_class} {disabled_trigger_class} {base_override} {radius_class} {glow_class}",
                    input {
                        id: "{input_id}",
                        class: "sd-combobox-input",
                        r#type: "text",
                        role: "combobox",
                        placeholder: "{placeholder_str}",
                        disabled: disabled,
                        value: "{current_text}",
                        "aria-expanded": "{open && !filtered_items.is_empty()}",
                        "aria-controls": "{listbox_id}",
                        "aria-autocomplete": "list",
                        "aria-activedescendant": active_opt_id.as_deref(),
                        "aria-describedby": aria_describedby.as_deref(),
                        "aria-invalid": error.is_some(),
                        onfocus: move |_| is_open.set(true),
                        onclick: move |e| {
                            e.stop_propagation();
                            is_open.set(true);
                        },
                        oninput: move |e| {
                            let val = e.value();
                            *value.write() = val.clone();
                            is_open.set(true);
                            active.set(None);
                            escape_count.set(0);
                            if let Some(h) = &on_change {
                                h.call(val);
                            }
                        },
                        onkeydown: handle_input_keydown,
                    }
                    div {
                        class: "sd-select-actions",
                        if loading {
                            div {
                                class: "sd-select-action-btn",
                                Icon { kind: IconKind::Spinner, size: 14 }
                            }
                        } else {
                            if has_clear_btn {
                                button {
                                    r#type: "button",
                                    class: "sd-select-action-btn",
                                    "aria-label": "{locale.clear_text}",
                                    title: "{locale.clear_text}",
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
                                tabindex: "-1",
                                "aria-label": "Toggle suggestions",
                                "aria-expanded": "{open}",
                                onclick: move |e| {
                                    e.stop_propagation();
                                    let next = !is_open();
                                    is_open.set(next);
                                    if !next {
                                        active.set(None);
                                    }
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
                        role: "listbox",
                        id: "{listbox_id}",
                        class: "sd-select-menu {anim_class} {radius_class}",
                        onclick: move |e| e.stop_propagation(),
                        for (idx, item) in filtered_items.iter().enumerate() {
                            {
                                let picked_label = item.label.clone();
                                rsx! {
                                    SelectOption {
                                        key: "{item.value}",
                                        id: format!("{combobox_id}-opt-{idx}"),
                                        item: item.clone(),
                                        selected: current_text == item.label || current_text == item.value,
                                        active: active() == Some(idx),
                                        on_pick: move |_| {
                                            let pl = picked_label.clone();
                                            *value.write() = pl.clone();
                                            if let Some(h) = &on_change {
                                                h.call(pl);
                                            }
                                            is_open.set(false);
                                            active.set(None);
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if let Some(err) = &error {
                span {
                    id: "{err_id}",
                    class: "sd-select-error",
                    "{err}"
                }
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

    #[test]
    fn test_display_label() {
        let data = vec![
            SelectItem::new("val1", "Label 1"),
            SelectItem::new("val2", "Label 2"),
        ];

        let (lbl, is_placeholder) = display_label(&data, Some("val1"), "Select an option...");
        assert_eq!(lbl, "Label 1");
        assert!(!is_placeholder);

        let (lbl, is_placeholder) = display_label(&data, Some("custom_val"), "Select an option...");
        assert_eq!(lbl, "custom_val");
        assert!(!is_placeholder);

        let (lbl, is_placeholder) = display_label(&data, None, "Select an option...");
        assert_eq!(lbl, "Select an option...");
        assert!(is_placeholder);
    }

    #[test]
    fn test_filter_items() {
        let data = vec![
            SelectItem::new("apple", "Apple").with_description("Fruit"),
            SelectItem::new("banana", "Banana").with_description("Yellow fruit"),
            SelectItem::new("carrot", "Carrot").with_description("Vegetable"),
        ];

        let all = filter_items(&data, "");
        assert_eq!(all.len(), 3);

        let fruit = filter_items(&data, "fruit");
        assert_eq!(fruit.len(), 2);
        assert_eq!(fruit[0].value, "apple");
        assert_eq!(fruit[1].value, "banana");

        let ban = filter_items(&data, "BAN");
        assert_eq!(ban.len(), 1);
        assert_eq!(ban[0].value, "banana");

        let none = filter_items(&data, "xyz");
        assert_eq!(none.len(), 0);
    }
}
