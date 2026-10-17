pub mod control;
pub use control::{
    calculate_pagination_range, PaginationControl, PaginationControlProps, PaginationItem,
};

use crate::a11y::ControlSize;
use crate::icon::{Icon, IconKind};
use dioxus::prelude::*;

/// Proprietà del componente [`Pagination`].
#[derive(Props, Clone, PartialEq)]
pub struct PaginationProps {
    /// Signal reattivo per la pagina corrente (1-indexed).
    pub page: Signal<usize>,
    /// Numero totale di pagine disponibili.
    pub total: usize,
    /// Numero di pagine adiacenti alla pagina attiva da visualizzare (default: 1).
    #[props(default = 1)]
    pub siblings: usize,
    /// Numero di pagine di inizio e fine da visualizzare sempre (default: 1).
    #[props(default = 1)]
    pub boundaries: usize,
    /// Mostra i pulsanti Precedente e Successivo (default: true).
    #[props(default = true)]
    pub show_controls: bool,
    /// Mostra i pulsanti Prima Pagina e Ultima Pagina (default: false).
    #[props(default)]
    pub show_edges: bool,
    /// Dimensione dei controlli tattili (default: `ControlSize::Md`).
    #[props(default)]
    pub size: ControlSize,
    /// Disabilita l'intera barra di paginazione.
    #[props(default)]
    pub disabled: bool,
    /// Evento emesso ad ogni cambio pagina.
    #[props(default)]
    pub on_change: Option<EventHandler<usize>>,
    /// Classi CSS addizionali.
    #[props(default)]
    pub class: String,
}

/// Componente Pagination: barra di navigazione numerata per liste e tabelle.
#[component]
pub fn Pagination(props: PaginationProps) -> Element {
    let current_page = (props.page)().clamp(1, props.total.max(1));
    let items =
        calculate_pagination_range(props.total, current_page, props.siblings, props.boundaries);

    let is_first = current_page <= 1;
    let is_last = current_page >= props.total;

    let page_sig = props.page;
    let on_change = props.on_change;
    let total = props.total;

    let set_page = move |new_p: usize| {
        let target = new_p.clamp(1, total.max(1));
        if target != page_sig() {
            let mut p = page_sig;
            p.set(target);
            if let Some(ref handler) = on_change {
                handler.call(target);
            }
        }
    };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("pagination").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("pagination", &props.class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let size_class = match props.size {
        ControlSize::Sm => "sd-pagination--sm",
        ControlSize::Md => "sd-pagination--md",
        ControlSize::Lg => "sd-pagination--lg",
    };

    rsx! {
        nav {
            class: "sd-pagination {size_class} {base_override} {class_modifiers} {props.class}",
            "aria-label": "Paginazione",
            ul { class: "sd-pagination__list",
                // Pulsante Prima Pagina (se abilitato)
                if props.show_edges {
                    li {
                        PaginationControl {
                            is_disabled: props.disabled || is_first,
                            size: props.size,
                            aria_label: "Prima pagina".to_string(),
                            on_click: move |_| set_page(1),
                            span { class: "flex items-center -space-x-1.5",
                                Icon { kind: IconKind::ChevronLeft, size: 13 }
                                Icon { kind: IconKind::ChevronLeft, size: 13 }
                            }
                        }
                    }
                }

                // Pulsante Pagina Precedente
                if props.show_controls {
                    li {
                        PaginationControl {
                            is_disabled: props.disabled || is_first,
                            size: props.size,
                            aria_label: "Pagina precedente".to_string(),
                            on_click: move |_| set_page(current_page.saturating_sub(1)),
                            Icon { kind: IconKind::ChevronLeft, size: 14 }
                        }
                    }
                }

                // Elementi numerati ed ellissi
                for (idx, item) in items.iter().enumerate() {
                    match item {
                        PaginationItem::Page(p) => {
                            let p_val = *p;
                            rsx! {
                                li { key: "page-{p_val}",
                                    PaginationControl {
                                        is_active: p_val == current_page,
                                        is_disabled: props.disabled,
                                        size: props.size,
                                        aria_label: format!("Pagina {p_val}"),
                                        on_click: move |_| set_page(p_val),
                                        "{p_val}"
                                    }
                                }
                            }
                        }
                        PaginationItem::Ellipsis => {
                            rsx! {
                                li { key: "ellipsis-{idx}", class: "sd-pagination__ellipsis", "aria-hidden": "true",
                                    span { "…" }
                                }
                            }
                        }
                    }
                }

                // Pulsante Pagina Successiva
                if props.show_controls {
                    li {
                        PaginationControl {
                            is_disabled: props.disabled || is_last,
                            size: props.size,
                            aria_label: "Pagina successiva".to_string(),
                            on_click: move |_| set_page(current_page + 1),
                            Icon { kind: IconKind::ChevronRight, size: 14 }
                        }
                    }
                }

                // Pulsante Ultima Pagina (se abilitato)
                if props.show_edges {
                    li {
                        PaginationControl {
                            is_disabled: props.disabled || is_last,
                            size: props.size,
                            aria_label: "Ultima pagina".to_string(),
                            on_click: move |_| set_page(props.total),
                            span { class: "flex items-center -space-x-1.5",
                                Icon { kind: IconKind::ChevronRight, size: 13 }
                                Icon { kind: IconKind::ChevronRight, size: 13 }
                            }
                        }
                    }
                }
            }
        }
    }
}
