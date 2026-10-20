use crate::a11y::ControlSize;
use dioxus::prelude::*;

/// Elemento del range di paginazione calcolato.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PaginationItem {
    /// Pagina numerata selezionabile.
    Page(usize),
    /// Salto ellittico non cliccabile o con salto rapido.
    Ellipsis,
}

/// Calcola la sequenza di pagine ed ellissi in base ai parametri impostati.
#[must_use]
pub fn calculate_pagination_range(
    total: usize,
    current: usize,
    siblings: usize,
    boundaries: usize,
) -> Vec<PaginationItem> {
    if total == 0 {
        return vec![];
    }
    let current = current.clamp(1, total);
    let total_numbers = boundaries * 2 + siblings * 2 + 1;
    let total_blocks = total_numbers + 2;

    if total <= total_blocks {
        return (1..=total).map(PaginationItem::Page).collect();
    }

    let left_sibling_index = current.saturating_sub(siblings).max(boundaries + 1);
    let right_sibling_index = (current + siblings).min(total.saturating_sub(boundaries));

    let should_show_left_ellipsis = left_sibling_index > boundaries + 2;
    let should_show_right_ellipsis = right_sibling_index < total - boundaries - 1;

    let mut items = Vec::new();

    // Bound sinistro
    for p in 1..=boundaries {
        items.push(PaginationItem::Page(p));
    }

    // Ellissi sinistra o pagine di riempimento
    if should_show_left_ellipsis {
        items.push(PaginationItem::Ellipsis);
    } else {
        for p in (boundaries + 1)..left_sibling_index {
            items.push(PaginationItem::Page(p));
        }
    }

    // Pagine intorno alla pagina corrente (siblings)
    for p in left_sibling_index..=right_sibling_index {
        items.push(PaginationItem::Page(p));
    }

    // Ellissi destra o pagine di riempimento
    if should_show_right_ellipsis {
        items.push(PaginationItem::Ellipsis);
    } else {
        for p in (right_sibling_index + 1)..=(total - boundaries) {
            items.push(PaginationItem::Page(p));
        }
    }

    // Bound destro
    for p in (total - boundaries + 1)..=total {
        items.push(PaginationItem::Page(p));
    }

    items
}

/// Proprietà del controllo singolo di paginazione.
#[derive(Props, Clone, PartialEq)]
pub struct PaginationControlProps {
    #[props(default)]
    pub is_active: bool,
    #[props(default)]
    pub is_disabled: bool,
    #[props(default)]
    pub size: ControlSize,
    #[props(default)]
    pub on_click: Option<EventHandler<MouseEvent>>,
    #[props(default)]
    pub aria_label: Option<String>,
    #[props(default)]
    pub class: String,
    pub children: Element,
}

/// Singolo pulsante o casella di controllo numerica o freccia nella barra di paginazione.
#[component]
pub fn PaginationControl(props: PaginationControlProps) -> Element {
    let size_class = match props.size {
        ControlSize::Sm => "sd-control--sm",
        ControlSize::Md => "sd-control--md",
        ControlSize::Lg => "sd-control--lg",
    };
    let active_class = if props.is_active {
        "sd-pagination__control--active"
    } else {
        ""
    };
    let disabled_class = if props.is_disabled {
        "sd-pagination__control--disabled"
    } else {
        ""
    };

    rsx! {
        button {
            r#type: "button",
            class: "sd-pagination__control {size_class} {active_class} {disabled_class} {props.class}",
            disabled: props.is_disabled,
            "aria-current": if props.is_active { "page" } else { "" },
            "aria-label": props.aria_label.as_deref().unwrap_or(""),
            onclick: move |evt| {
                if !props.is_disabled {
                    if let Some(ref handler) = props.on_click {
                        handler.call(evt);
                    }
                }
            },
            {props.children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_range_small_total() {
        let range = calculate_pagination_range(5, 1, 1, 1);
        assert_eq!(
            range,
            vec![
                PaginationItem::Page(1),
                PaginationItem::Page(2),
                PaginationItem::Page(3),
                PaginationItem::Page(4),
                PaginationItem::Page(5),
            ]
        );
    }

    #[test]
    fn test_pagination_range_with_ellipsis() {
        let range = calculate_pagination_range(10, 5, 1, 1);
        assert_eq!(
            range,
            vec![
                PaginationItem::Page(1),
                PaginationItem::Ellipsis,
                PaginationItem::Page(4),
                PaginationItem::Page(5),
                PaginationItem::Page(6),
                PaginationItem::Ellipsis,
                PaginationItem::Page(10),
            ]
        );
    }

    #[test]
    fn test_pagination_range_at_start() {
        let range = calculate_pagination_range(10, 1, 1, 1);
        assert_eq!(
            range,
            vec![
                PaginationItem::Page(1),
                PaginationItem::Page(2),
                PaginationItem::Ellipsis,
                PaginationItem::Page(10),
            ]
        );
    }
}
