pub mod item;
pub use item::{BreadcrumbItem, BreadcrumbItemProps};

use crate::icon::{Icon, IconKind, SvgIcon};
use dioxus::prelude::*;

/// Struttura dati per definire una tappa della gerarchia breadcrumb in modalità guidata dai dati.
#[derive(Clone, PartialEq, Debug)]
pub struct BreadcrumbData {
    /// Testo visibile dell'elemento.
    pub label: String,
    /// URL di navigazione opzionale.
    pub href: Option<String>,
    /// Icona opzionale posizionata prima del testo.
    pub icon: Option<SvgIcon>,
}

impl BreadcrumbData {
    /// Crea un nuovo elemento breadcrumb con etichetta.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
            icon: None,
        }
    }

    /// Imposta l'URL di navigazione.
    #[must_use]
    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }

    /// Imposta un'icona vettoriale associata.
    #[must_use]
    pub fn with_icon(mut self, icon: SvgIcon) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// Proprietà del componente [`Breadcrumbs`].
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbsProps {
    /// Lista dichiarativa degli elementi di navigazione (supporta ellissi automatica con `max_items`).
    #[props(default)]
    pub items: Option<Vec<BreadcrumbData>>,
    /// Numero massimo di elementi visibili prima dell'attivazione dell'ellissi (`...`).
    #[props(default)]
    pub max_items: Option<usize>,
    /// Numero di elementi mantenuti all'inizio prima dell'ellissi (default: 1).
    #[props(default = 1)]
    pub items_before_collapse: usize,
    /// Numero di elementi mantenuti alla fine dopo l'ellissi (default: 1).
    #[props(default = 1)]
    pub items_after_collapse: usize,
    /// Stato di espansione controllato opzionale (se fornito, sovrascrive lo stato interno).
    #[props(default)]
    pub expanded: Option<Signal<bool>>,
    /// Callback invocato quando lo stato di espansione cambia (tramite click sull'ellissi o pulsante di compressione).
    #[props(default)]
    pub on_toggle_expand: Option<EventHandler<bool>>,
    /// Elemento visivo personalizzato per il separatore (sovrascrive il separatore di default).
    #[props(default)]
    pub separator: Option<Element>,
    /// Separatore testuale semplice (es. "/", ">", "•").
    #[props(default)]
    pub separator_text: Option<String>,
    /// Classi CSS addizionali per il contenitore di navigazione `<nav>`.
    #[props(default)]
    pub class: String,
    /// Contenuto compositivo personalizzato (istanze di [`BreadcrumbItem`]).
    #[props(default)]
    pub children: Element,
}

/// Calcola se il percorso deve essere collassato e la quantità di elementi da preservare in testa (`head`) e coda (`tail`).
/// Quando `max_items` è impostato e `items_before_collapse + items_after_collapse < max`, distribuisce gli elementi
/// visibili in modo da mostrare esattamente `max` elementi.
#[must_use]
pub fn calculate_collapse_bounds(
    total: usize,
    max_items: Option<usize>,
    items_before_collapse: usize,
    items_after_collapse: usize,
) -> (bool, usize, usize) {
    let Some(max) = max_items else {
        return (false, items_before_collapse, items_after_collapse);
    };

    if total <= max || max == 0 {
        return (false, items_before_collapse, items_after_collapse);
    }

    // Se l'utente non ha impostato una somma esplicita maggiore o uguale a max,
    // distribuiamo gli elementi visibili per raggiungere esattamente `max`.
    let (before, after) = if items_before_collapse + items_after_collapse < max {
        let b = items_before_collapse.max(1);
        let a = items_after_collapse.max(max.saturating_sub(b));
        (b, a)
    } else {
        (items_before_collapse, items_after_collapse)
    };

    let should_collapse = total > (before + after);
    (should_collapse, before, after)
}

/// Componente Breadcrumbs per navigazione gerarchica, wayfinding e percorsi strutturati.
#[component]
pub fn Breadcrumbs(props: BreadcrumbsProps) -> Element {
    let mut internal_expanded = use_signal(|| false);
    let mut last_max_items = use_signal(|| props.max_items);

    // Se max_items cambia da esterno, ripristiniamo la compressione interna
    if *last_max_items.read() != props.max_items {
        last_max_items.set(props.max_items);
        internal_expanded.set(false);
    }

    let is_expanded = match props.expanded {
        Some(s) => s(),
        None => internal_expanded(),
    };

    let mut set_expanded = move |val: bool| {
        if let Some(mut ext_sig) = props.expanded {
            ext_sig.set(val);
        } else {
            internal_expanded.set(val);
        }
        if let Some(ref handler) = props.on_toggle_expand {
            handler.call(val);
        }
    };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val
        .resolve_component_base("breadcrumbs")
        .unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("breadcrumbs", &props.class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let render_separator = |idx: usize| -> Element {
        if let Some(ref custom_sep) = props.separator {
            rsx! {
                li { key: "sep-{idx}", class: "sd-breadcrumbs__separator", "aria-hidden": "true",
                    {custom_sep.clone()}
                }
            }
        } else if let Some(ref text) = props.separator_text {
            rsx! {
                li { key: "sep-{idx}", class: "sd-breadcrumbs__separator sd-breadcrumbs__separator--text", "aria-hidden": "true",
                    "{text}"
                }
            }
        } else {
            rsx! {
                li { key: "sep-{idx}", class: "sd-breadcrumbs__separator", "aria-hidden": "true",
                    Icon { kind: IconKind::ChevronRight, size: 12 }
                }
            }
        }
    };

    rsx! {
        nav {
            class: "sd-breadcrumbs {base_override} {class_modifiers} {props.class}",
            "aria-label": "Breadcrumb",
            ol { class: "sd-breadcrumbs__list",
                if let Some(ref items) = props.items {
                    {
                        let total = items.len();
                        let (can_collapse, before_count, after_count) = calculate_collapse_bounds(
                            total,
                            props.max_items,
                            props.items_before_collapse,
                            props.items_after_collapse,
                        );

                        if can_collapse && !is_expanded {
                            let head = &items[0..before_count.min(total)];
                            let tail_start = total.saturating_sub(after_count).max(head.len());
                            let tail = &items[tail_start..total];

                            rsx! {
                                for (idx, item) in head.iter().enumerate() {
                                    BreadcrumbItem {
                                        key: "item-head-{idx}",
                                        href: item.href.clone(),
                                        icon: item.icon.clone(),
                                        "{item.label}"
                                    }
                                    {render_separator(idx)}
                                }
                                li {
                                    class: "sd-breadcrumbs__ellipsis",
                                    button {
                                        r#type: "button",
                                        class: "sd-breadcrumbs__ellipsis-button",
                                        title: "Mostra percorso completo",
                                        onclick: move |_| set_expanded(true),
                                        "..."
                                    }
                                }
                                {render_separator(head.len())}
                                for (idx, item) in tail.iter().enumerate() {
                                    {
                                        let is_last = idx + 1 == tail.len();
                                        rsx! {
                                            BreadcrumbItem {
                                                key: "item-tail-{idx}",
                                                href: if is_last { None } else { item.href.clone() },
                                                is_current: is_last,
                                                icon: item.icon.clone(),
                                                "{item.label}"
                                            }
                                            if !is_last {
                                                {render_separator(total - tail.len() + idx)}
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                for (idx, item) in items.iter().enumerate() {
                                    {
                                        let is_last = idx + 1 == total;
                                        rsx! {
                                            BreadcrumbItem {
                                                key: "item-{idx}",
                                                href: if is_last { None } else { item.href.clone() },
                                                is_current: is_last,
                                                icon: item.icon.clone(),
                                                "{item.label}"
                                            }
                                            if !is_last {
                                                {render_separator(idx)}
                                            }
                                        }
                                    }
                                }
                                if can_collapse && is_expanded {
                                    li {
                                        class: "sd-breadcrumbs__ellipsis",
                                        button {
                                            r#type: "button",
                                            class: "sd-breadcrumbs__ellipsis-button",
                                            title: "Comprimi percorso",
                                            onclick: move |_| set_expanded(false),
                                            "«"
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {props.children}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumb_data_builder() {
        let b = BreadcrumbData::new("Home").with_href("/");
        assert_eq!(b.label, "Home");
        assert_eq!(b.href, Some("/".to_string()));
    }

    #[test]
    fn test_collapse_bounds_with_max_3() {
        // 5 elementi con max_items = 3: deve collassare e mostrare 1 elemento in testa e 2 in coda (totale 3 visibili)
        let (should_collapse, before, after) = calculate_collapse_bounds(5, Some(3), 1, 1);
        assert!(should_collapse);
        assert_eq!(before, 1);
        assert_eq!(after, 2);
        assert_eq!(before + after, 3);
    }

    #[test]
    fn test_collapse_bounds_not_exceeding_max() {
        // 3 elementi con max_items = 3: non deve collassare
        let (should_collapse, _, _) = calculate_collapse_bounds(3, Some(3), 1, 1);
        assert!(!should_collapse);
    }

    #[test]
    fn test_collapse_bounds_custom_overrides() {
        // Utente definisce esplicitamente 2 in testa e 2 in coda con max_items = 4
        let (should_collapse, before, after) = calculate_collapse_bounds(6, Some(4), 2, 2);
        assert!(should_collapse);
        assert_eq!(before, 2);
        assert_eq!(after, 2);
    }
}
