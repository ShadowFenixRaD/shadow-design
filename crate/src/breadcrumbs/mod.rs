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

/// Componente Breadcrumbs per navigazione gerarchica, wayfinding e percorsi strutturati.
#[component]
pub fn Breadcrumbs(props: BreadcrumbsProps) -> Element {
    let mut is_expanded = use_signal(|| false);

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
                        let should_collapse = !is_expanded()
                            && props.max_items.map(|max| total > max && total > (props.items_before_collapse + props.items_after_collapse)).unwrap_or(false);

                        if should_collapse {
                            let head = &items[0..props.items_before_collapse];
                            let tail_start = total.saturating_sub(props.items_after_collapse);
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
                                        onclick: move |_| is_expanded.set(true),
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
}
