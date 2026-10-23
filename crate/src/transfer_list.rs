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
    #[props(default = ("Origine".to_string(), "Destinazione".to_string()))] titles: (String, String),
    #[props(default = "320px".to_string())] height: String,
    #[props(default)] on_change: Option<EventHandler<(Vec<TransferItem>, Vec<TransferItem>)>>,
    #[props(default)] class: String,
) -> Element {
    let mut selected_source = use_signal(BTreeSet::<String>::new);
    let mut selected_target = use_signal(BTreeSet::<String>::new);
    let mut search_source = use_signal(String::new);
    let mut search_target = use_signal(String::new);

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

    rsx! {
        div { class: "sd-transfer-list flex flex-col md:flex-row items-center gap-4 w-full {class}",
            // Colonna Sorgente
            div { class: "sd-transfer-list__column flex-1 w-full flex flex-col rounded-xl border border-white/[0.12] bg-zinc-950/70 overflow-hidden shadow-lg",
                // Intestazione colonna
                div { class: "p-3 bg-white/[0.03] border-b border-white/[0.08] flex items-center justify-between",
                    span { class: "text-sm font-semibold text-white", "{titles.0}" }
                    span { class: "text-xs font-mono text-zinc-400 px-2 py-0.5 rounded bg-white/[0.05]",
                        "{selected_source().len()} / {source().len()}"
                    }
                }

                if searchable {
                    div { class: "p-2 border-b border-white/[0.06]",
                        input {
                            r#type: "text",
                            class: "w-full px-3 py-1.5 rounded-lg bg-zinc-900 border border-white/[0.10] text-xs text-white placeholder-zinc-500 outline-none focus:border-purple-500",
                            placeholder: "Cerca elementi...",
                            value: "{search_source}",
                            oninput: move |e| search_source.set(e.value()),
                        }
                    }
                }

                div {
                    class: "p-2 overflow-y-auto flex flex-col gap-1",
                    style: "height: {height};",
                    if filtered_source.is_empty() {
                        div { class: "p-6 text-center text-xs text-zinc-500 font-mono",
                            "Nessun elemento disponibile"
                        }
                    } else {
                        for item in filtered_source {
                            {
                                let id = item.id.clone();
                                let is_selected = selected_source().contains(&id);
                                let disabled = item.disabled;
                                let item_cls = if is_selected {
                                    "bg-purple-950/40 border-purple-500/40 text-purple-200"
                                } else {
                                    "hover:bg-white/[0.04] border-transparent text-zinc-300"
                                };
                                let dis_cls = if disabled { "opacity-40 cursor-not-allowed" } else { "" };

                                let chk_cls = if is_selected {
                                    "bg-purple-600 border-purple-500 text-white shadow-sm"
                                } else {
                                    "border-white/20 bg-white/[0.04] text-transparent"
                                };

                                rsx! {
                                    div {
                                        key: "{id}",
                                        class: "flex items-center gap-2.5 p-2 rounded-lg cursor-pointer transition-colors border {item_cls} {dis_cls}",
                                        onclick: move |_| {
                                            if !disabled {
                                                let mut set = selected_source();
                                                if set.contains(&id) {
                                                    set.remove(&id);
                                                } else {
                                                    set.insert(id.clone());
                                                }
                                                selected_source.set(set);
                                            }
                                        },
                                        div { class: "w-4 h-4 rounded flex items-center justify-center shrink-0 border transition-all {chk_cls}",
                                            Icon { kind: IconKind::Check, size: 12 }
                                        }
                                        div { class: "flex flex-col min-w-0 flex-1",
                                            span { class: "text-xs font-medium truncate", "{item.label}" }
                                            if let Some(desc) = &item.description {
                                                span { class: "text-[11px] text-zinc-400 truncate", "{desc}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Pulsanti Centrali di Controllo
            div { class: "flex md:flex-col items-center justify-center gap-2 shrink-0",
                button {
                    r#type: "button",
                    title: "Sposta tutti a destra",
                    class: "p-2 rounded-lg bg-zinc-900 border border-white/[0.12] text-zinc-300 hover:text-white hover:bg-purple-600/30 hover:border-purple-500/40 transition-all cursor-pointer shadow-sm",
                    onclick: move_all_to_target,
                    Icon { kind: IconKind::ChevronRight, size: 16 }
                }
                button {
                    r#type: "button",
                    title: "Sposta selezionati a destra",
                    disabled: selected_source().is_empty(),
                    class: "p-2 rounded-lg bg-zinc-900 border border-white/[0.12] text-zinc-300 hover:text-white hover:bg-purple-600/30 hover:border-purple-500/40 transition-all cursor-pointer shadow-sm disabled:opacity-40 disabled:cursor-not-allowed",
                    onclick: move_selected_to_target,
                    Icon { kind: IconKind::ChevronRight, size: 16 }
                }
                button {
                    r#type: "button",
                    title: "Sposta selezionati a sinistra",
                    disabled: selected_target().is_empty(),
                    class: "p-2 rounded-lg bg-zinc-900 border border-white/[0.12] text-zinc-300 hover:text-white hover:bg-purple-600/30 hover:border-purple-500/40 transition-all cursor-pointer shadow-sm disabled:opacity-40 disabled:cursor-not-allowed",
                    onclick: move_selected_to_source,
                    Icon { kind: IconKind::ChevronLeft, size: 16 }
                }
                button {
                    r#type: "button",
                    title: "Sposta tutti a sinistra",
                    class: "p-2 rounded-lg bg-zinc-900 border border-white/[0.12] text-zinc-300 hover:text-white hover:bg-purple-600/30 hover:border-purple-500/40 transition-all cursor-pointer shadow-sm",
                    onclick: move_all_to_source,
                    Icon { kind: IconKind::ChevronLeft, size: 16 }
                }
            }

            // Colonna Destinazione
            div { class: "sd-transfer-list__column flex-1 w-full flex flex-col rounded-xl border border-white/[0.12] bg-zinc-950/70 overflow-hidden shadow-lg",
                // Intestazione colonna
                div { class: "p-3 bg-white/[0.03] border-b border-white/[0.08] flex items-center justify-between",
                    span { class: "text-sm font-semibold text-white", "{titles.1}" }
                    span { class: "text-xs font-mono text-zinc-400 px-2 py-0.5 rounded bg-white/[0.05]",
                        "{selected_target().len()} / {target().len()}"
                    }
                }

                if searchable {
                    div { class: "p-2 border-b border-white/[0.06]",
                        input {
                            r#type: "text",
                            class: "w-full px-3 py-1.5 rounded-lg bg-zinc-900 border border-white/[0.10] text-xs text-white placeholder-zinc-500 outline-none focus:border-purple-500",
                            placeholder: "Cerca elementi...",
                            value: "{search_target}",
                            oninput: move |e| search_target.set(e.value()),
                        }
                    }
                }

                div {
                    class: "p-2 overflow-y-auto flex flex-col gap-1",
                    style: "height: {height};",
                    if filtered_target.is_empty() {
                        div { class: "p-6 text-center text-xs text-zinc-500 font-mono",
                            "Nessun elemento assegnato"
                        }
                    } else {
                        for item in filtered_target {
                            {
                                let id = item.id.clone();
                                let is_selected = selected_target().contains(&id);
                                let disabled = item.disabled;
                                let item_cls = if is_selected {
                                    "bg-purple-950/40 border-purple-500/40 text-purple-200"
                                } else {
                                    "hover:bg-white/[0.04] border-transparent text-zinc-300"
                                };
                                let dis_cls = if disabled { "opacity-40 cursor-not-allowed" } else { "" };

                                let chk_cls = if is_selected {
                                    "bg-purple-600 border-purple-500 text-white shadow-sm"
                                } else {
                                    "border-white/20 bg-white/[0.04] text-transparent"
                                };

                                rsx! {
                                    div {
                                        key: "{id}",
                                        class: "flex items-center gap-2.5 p-2 rounded-lg cursor-pointer transition-colors border {item_cls} {dis_cls}",
                                        onclick: move |_| {
                                            if !disabled {
                                                let mut set = selected_target();
                                                if set.contains(&id) {
                                                    set.remove(&id);
                                                } else {
                                                    set.insert(id.clone());
                                                }
                                                selected_target.set(set);
                                            }
                                        },
                                        div { class: "w-4 h-4 rounded flex items-center justify-center shrink-0 border transition-all {chk_cls}",
                                            Icon { kind: IconKind::Check, size: 12 }
                                        }
                                        div { class: "flex flex-col min-w-0 flex-1",
                                            span { class: "text-xs font-medium truncate", "{item.label}" }
                                            if let Some(desc) = &item.description {
                                                span { class: "text-[11px] text-zinc-400 truncate", "{desc}" }
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
}
