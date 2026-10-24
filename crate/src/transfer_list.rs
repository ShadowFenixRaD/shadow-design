use dioxus::prelude::*;
use std::collections::BTreeSet;

use crate::icon::{Icon, IconKind};

/// Singolo elemento trasferibile tra due colonne.
#[derive(Clone, PartialEq, Debug)]
pub struct TransferItem {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub group: Option<String>,
    pub disabled: bool,
}

impl TransferItem {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            group: None,
            disabled: false,
        }
    }

    #[must_use]
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    #[must_use]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Interfaccia a due colonne affiancate per spostare elementi da un elenco di origine a destinazione.
#[component]
pub fn TransferList(
    mut source: Signal<Vec<TransferItem>>,
    mut target: Signal<Vec<TransferItem>>,
    #[props(default = true)] searchable: bool,
    #[props(default = ("Source".to_string(), "Target".to_string()))] titles: (String, String),
    #[props(default = "320px".to_string())] height: String,
    #[props(default)] on_change: Option<EventHandler<(Vec<TransferItem>, Vec<TransferItem>)>>,
    #[props(default)] class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let mut selected_source = use_signal(BTreeSet::<String>::new);
    let mut selected_target = use_signal(BTreeSet::<String>::new);
    let search_source = use_signal(String::new);
    let search_target = use_signal(String::new);

    // Sposta selezionati da source a target
    let move_selected_to_target = move |_| {
        let sel = selected_source();
        if sel.is_empty() {
            return;
        }

        let src_list = source();
        let mut tgt_list = target();

        let (moved, remaining): (Vec<TransferItem>, Vec<TransferItem>) = src_list
            .into_iter()
            .partition(|item| sel.contains(&item.id) && !item.disabled);

        tgt_list.extend(moved);
        source.set(remaining);
        target.set(tgt_list.clone());
        selected_source.set(BTreeSet::new());

        if let Some(handler) = &on_change {
            handler.call((source(), tgt_list));
        }
    };

    // Sposta tutti da source a target
    let move_all_to_target = move |_| {
        let src_list = source();
        let mut tgt_list = target();

        let (moved, remaining): (Vec<TransferItem>, Vec<TransferItem>) = src_list
            .into_iter()
            .partition(|item| !item.disabled);

        tgt_list.extend(moved);
        source.set(remaining);
        target.set(tgt_list.clone());
        selected_source.set(BTreeSet::new());

        if let Some(handler) = &on_change {
            handler.call((source(), tgt_list));
        }
    };

    // Sposta selezionati da target a source
    let move_selected_to_source = move |_| {
        let sel = selected_target();
        if sel.is_empty() {
            return;
        }

        let mut src_list = source();
        let tgt_list = target();

        let (moved, remaining): (Vec<TransferItem>, Vec<TransferItem>) = tgt_list
            .into_iter()
            .partition(|item| sel.contains(&item.id) && !item.disabled);

        src_list.extend(moved);
        target.set(remaining);
        source.set(src_list.clone());
        selected_target.set(BTreeSet::new());

        if let Some(handler) = &on_change {
            handler.call((src_list, target()));
        }
    };

    // Sposta tutti da target a source
    let move_all_to_source = move |_| {
        let mut src_list = source();
        let tgt_list = target();

        let (moved, remaining): (Vec<TransferItem>, Vec<TransferItem>) = tgt_list
            .into_iter()
            .partition(|item| !item.disabled);

        src_list.extend(moved);
        target.set(remaining);
        source.set(src_list.clone());
        selected_target.set(BTreeSet::new());

        if let Some(handler) = &on_change {
            handler.call((src_list, target()));
        }
    };

    // Filtro elementi visibili
    let filtered_source: Vec<TransferItem> = {
        let q = search_source().to_lowercase();
        source()
            .into_iter()
            .filter(|i| q.is_empty() || i.label.to_lowercase().contains(&q))
            .collect()
    };

    let filtered_target: Vec<TransferItem> = {
        let q = search_target().to_lowercase();
        target()
            .into_iter()
            .filter(|i| q.is_empty() || i.label.to_lowercase().contains(&q))
            .collect()
    };

    let toggle_source = Callback::new(move |id: String| {
        let mut set = selected_source();
        if !set.remove(&id) {
            set.insert(id);
        }
        selected_source.set(set);
    });
    let toggle_target = Callback::new(move |id: String| {
        let mut set = selected_target();
        if !set.remove(&id) {
            set.insert(id);
        }
        selected_target.set(set);
    });

    let source_total = source().len();
    let target_total = target().len();
    let source_selected = selected_source().clone();
    let target_selected = selected_target().clone();
    let source_selected_len = source_selected.len();
    let target_selected_len = target_selected.len();

    rsx! {
        div { class: "sd-transfer-list {class}",
            TransferColumn {
                title: titles.0.clone(),
                total: source_total,
                selected_count: source_selected_len,
                items: filtered_source,
                selected: source_selected,
                search: search_source,
                searchable,
                height: height.clone(),
                empty_text: locale.transfer_empty_source.clone(),
                search_placeholder: locale.transfer_search.clone(),
                on_toggle: toggle_source,
            }

            div { class: "sd-transfer-list__controls",
                button {
                    r#type: "button",
                    title: "{locale.transfer_move_all_right}",
                    "aria-label": "{locale.transfer_move_all_right}",
                    class: "sd-transfer-list__move",
                    onclick: move_all_to_target,
                    Icon { kind: IconKind::ChevronRight, size: 16 }
                }
                button {
                    r#type: "button",
                    title: "{locale.transfer_move_selected_right}",
                    "aria-label": "{locale.transfer_move_selected_right}",
                    disabled: source_selected_len == 0,
                    class: "sd-transfer-list__move",
                    onclick: move_selected_to_target,
                    Icon { kind: IconKind::ChevronRight, size: 16 }
                }
                button {
                    r#type: "button",
                    title: "{locale.transfer_move_selected_left}",
                    "aria-label": "{locale.transfer_move_selected_left}",
                    disabled: target_selected_len == 0,
                    class: "sd-transfer-list__move",
                    onclick: move_selected_to_source,
                    Icon { kind: IconKind::ChevronLeft, size: 16 }
                }
                button {
                    r#type: "button",
                    title: "{locale.transfer_move_all_left}",
                    "aria-label": "{locale.transfer_move_all_left}",
                    class: "sd-transfer-list__move",
                    onclick: move_all_to_source,
                    Icon { kind: IconKind::ChevronLeft, size: 16 }
                }
            }

            TransferColumn {
                title: titles.1.clone(),
                total: target_total,
                selected_count: target_selected_len,
                items: filtered_target,
                selected: target_selected,
                search: search_target,
                searchable,
                height: height.clone(),
                empty_text: locale.transfer_empty_target.clone(),
                search_placeholder: locale.transfer_search.clone(),
                on_toggle: toggle_target,
            }
        }
    }
}

/// One column of the transfer list: header, optional search field and the item list.
#[component]
fn TransferColumn(
    title: String,
    total: usize,
    selected_count: usize,
    items: Vec<TransferItem>,
    selected: BTreeSet<String>,
    mut search: Signal<String>,
    searchable: bool,
    height: String,
    empty_text: String,
    search_placeholder: String,
    on_toggle: Callback<String>,
) -> Element {
    rsx! {
        div { class: "sd-transfer-list__column",
            div { class: "sd-transfer-list__header",
                span { class: "sd-transfer-list__title", "{title}" }
                span { class: "sd-transfer-list__count", "{selected_count} / {total}" }
            }

            if searchable {
                div { class: "sd-transfer-list__search",
                    div { class: "sd-transfer-list__field",
                        input {
                            r#type: "text",
                            class: "sd-transfer-list__input",
                            placeholder: "{search_placeholder}",
                            "aria-label": "{search_placeholder}",
                            value: "{search}",
                            oninput: move |e| search.set(e.value()),
                        }
                    }
                }
            }

            div {
                class: "sd-transfer-list__body",
                role: "listbox",
                "aria-multiselectable": "true",
                style: "height: {height};",
                if items.is_empty() {
                    div { class: "sd-transfer-list__empty", "{empty_text}" }
                } else {
                    for item in items {
                        {
                            let id = item.id.clone();
                            let id_key = item.id.clone();
                            let is_selected = selected.contains(&id);
                            let disabled = item.disabled;
                            let item_class = if is_selected {
                                "sd-transfer-list__item sd-transfer-list__item--selected"
                            } else {
                                "sd-transfer-list__item"
                            };

                            rsx! {
                                div {
                                    key: "{id}",
                                    class: "{item_class}",
                                    role: "option",
                                    tabindex: if disabled { "-1" } else { "0" },
                                    "aria-selected": if is_selected { "true" } else { "false" },
                                    "aria-disabled": if disabled { "true" } else { "false" },
                                    onclick: move |_| {
                                        if !disabled {
                                            on_toggle.call(id.clone());
                                        }
                                    },
                                    onkeydown: move |evt: KeyboardEvent| {
                                        if !disabled
                                            && (evt.key() == Key::Enter
                                                || evt.key() == Key::Character(" ".to_string()))
                                        {
                                            evt.prevent_default();
                                            on_toggle.call(id_key.clone());
                                        }
                                    },
                                    span { class: "sd-transfer-list__check", "aria-hidden": "true",
                                        if is_selected {
                                            Icon { kind: IconKind::Check, size: 12 }
                                        }
                                    }
                                    span { class: "sd-transfer-list__text",
                                        span { class: "sd-transfer-list__label", "{item.label}" }
                                        if let Some(desc) = &item.description {
                                            span { class: "sd-transfer-list__desc", "{desc}" }
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
}
