use dioxus::prelude::*;

use crate::dropdown::{Dropdown, DropdownItem, DropdownPlacement};
use crate::icon::{Icon, IconKind};

/// Singola azione o pulsante per la barra degli strumenti.
#[derive(Clone, PartialEq, Debug)]
pub struct ToolbarAction {
    pub id: String,
    pub label: String,
    pub icon: Option<IconKind>,
    pub shortcut: Option<String>,
    pub disabled: bool,
    pub danger: bool,
}

impl ToolbarAction {
    #[must_use]
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            danger: false,
        }
    }

    #[must_use]
    pub fn with_icon(mut self, icon: IconKind) -> Self {
        self.icon = Some(icon);
        self
    }

    #[must_use]
    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    #[must_use]
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    #[must_use]
    pub fn with_danger(mut self, danger: bool) -> Self {
        self.danger = danger;
        self
    }
}

/// Barra degli strumenti responsive che sposta automaticamente le azioni in eccesso in un dropdown.
#[component]
pub fn OverflowSet(
    items: Vec<ToolbarAction>,
    #[props(default)] overflow_items: Vec<ToolbarAction>,
    #[props(default)] max_visible: Option<usize>,
    #[props(default = "Altre azioni".to_string())] overflow_button_label: String,
    #[props(default)] on_action: Option<EventHandler<String>>,
    #[props(default)] class: String,
) -> Element {
    let is_menu_open = use_signal(|| false);
    let limit = max_visible.unwrap_or(4);

    let (visible, excess) = if items.len() > limit {
        let (v, e) = items.split_at(limit);
        (v.to_vec(), e.to_vec())
    } else {
        (items, Vec::new())
    };

    let all_overflow: Vec<ToolbarAction> = excess
        .into_iter()
        .chain(overflow_items.into_iter())
        .collect();

    let has_overflow = !all_overflow.is_empty();

    rsx! {
        div { class: "sd-overflow-set flex items-center gap-1.5 p-1 rounded-xl bg-zinc-950/70 border border-white/[0.08] backdrop-blur-md shadow-sm {class}",
            // Azioni Visibili
            for act in visible {
                {
                    let action_id = act.id.clone();
                    let lbl = act.label.clone();
                    let icon_opt = act.icon;
                    let disabled = act.disabled;
                    let danger = act.danger;
                    let btn_cls = if disabled {
                        "opacity-40 cursor-not-allowed text-zinc-500"
                    } else if danger {
                        "text-red-300 hover:text-white hover:bg-red-500/20 hover:border-red-500/30"
                    } else {
                        "text-zinc-300 hover:text-white hover:bg-white/[0.06] hover:border-white/[0.08]"
                    };

                    rsx! {
                        button {
                            key: "{action_id}",
                            r#type: "button",
                            disabled: disabled,
                            class: "sd-overflow-set__item flex items-center gap-2 px-3 py-1.5 rounded-lg text-xs font-medium transition-all cursor-pointer border border-transparent {btn_cls}",
                            onclick: {
                                let aid = action_id.clone();
                                move |_| {
                                    if !disabled {
                                        if let Some(handler) = &on_action {
                                            handler.call(aid.clone());
                                        }
                                    }
                                }
                            },
                            if let Some(ik) = icon_opt {
                                Icon { kind: ik, size: 14 }
                            }
                            span { "{lbl}" }
                        }
                    }
                }
            }

            // Dropdown di Overflow
            if has_overflow {
                div { class: "ml-auto border-l border-white/[0.08] pl-1.5",
                    Dropdown {
                        is_open: is_menu_open,
                        placement: DropdownPlacement::BottomRight,
                        trigger: rsx! {
                            button {
                                r#type: "button",
                                class: "sd-overflow-set__more-btn flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs font-medium text-zinc-400 hover:text-white hover:bg-white/[0.06] transition-colors cursor-pointer",
                                title: "{overflow_button_label}",
                                span { "•••" }
                            }
                        },
                        for act in all_overflow {
                            {
                                let action_id = act.id.clone();
                                let lbl = act.label.clone();
                                rsx! {
                                    DropdownItem {
                                        key: "{action_id}",
                                        icon: act.icon,
                                        disabled: act.disabled,
                                        danger: act.danger,
                                        onclick: {
                                            let aid = action_id.clone();
                                            move |_| {
                                                if let Some(h) = &on_action {
                                                    h.call(aid.clone());
                                                }
                                            }
                                        },
                                        "{lbl}"
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

/// Alias per barra degli strumenti responsive (Microsoft Fluent UI pattern).
#[component]
pub fn ResponsiveToolbar(
    items: Vec<ToolbarAction>,
    #[props(default)] overflow_items: Vec<ToolbarAction>,
    #[props(default)] max_visible: Option<usize>,
    #[props(default = "Altre azioni".to_string())] overflow_button_label: String,
    #[props(default)] on_action: Option<EventHandler<String>>,
    #[props(default)] class: String,
) -> Element {
    rsx! {
        OverflowSet {
            items: items,
            overflow_items: overflow_items,
            max_visible: max_visible,
            overflow_button_label: overflow_button_label,
            on_action: on_action,
            class: class,
        }
    }
}
