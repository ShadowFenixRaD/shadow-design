use std::sync::Arc;
use dioxus::prelude::*;

/// Allineamento del testo e del contenuto all'interno delle celle della tabella.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TableAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl TableAlign {
    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            Self::Left => "sd-table--align-left",
            Self::Center => "sd-table--align-center",
            Self::Right => "sd-table--align-right",
        }
    }
}

/// Direzione dell'ordinamento per le colonne di una tabella.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SortDirection {
    #[default]
    None,
    Ascending,
    Descending,
}

impl SortDirection {
    #[must_use]
    pub fn next(&self) -> Self {
        match self {
            Self::None => Self::Ascending,
            Self::Ascending => Self::Descending,
            Self::Descending => Self::None,
        }
    }

    #[must_use]
    pub fn scss_class(&self) -> &'static str {
        match self {
            Self::None => "sd-table-cell--sort-none",
            Self::Ascending => "sd-table-cell--sort-asc",
            Self::Descending => "sd-table-cell--sort-desc",
        }
    }
}

/// Contenitore principale per tabelle semantiche ad alte prestazioni.
#[component]
pub fn Table(
    #[props(default)] striped: bool,
    #[props(default)] bordered: bool,
    #[props(default = true)] hoverable: bool,
    #[props(default)] compact: bool,
    #[props(default)] sticky_header: bool,
    #[props(default)] class: String,
    #[props(default)] id: Option<String>,
    children: Element,
) -> Element {
    let striped_class = if striped { "sd-table--striped" } else { "" };
    let bordered_class = if bordered { "sd-table--bordered" } else { "" };
    let hover_class = if hoverable { "sd-table--hoverable" } else { "" };
    let compact_class = if compact { "sd-table--compact" } else { "" };
    let sticky_class = if sticky_header { "sd-table--sticky-header" } else { "" };

    #[cfg(feature = "theme")]
    let theme_val = crate::theme::use_theme()();
    #[cfg(feature = "theme")]
    let base_override = theme_val.resolve_component_base("table").unwrap_or("");
    #[cfg(feature = "theme")]
    let class_modifiers = theme_val.resolve_class_modifiers("table", &class);

    #[cfg(not(feature = "theme"))]
    let base_override = "";
    #[cfg(not(feature = "theme"))]
    let class_modifiers = String::new();

    let combined_table_class = format!(
        "sd-table {striped_class} {bordered_class} {hover_class} {compact_class} {sticky_class} {base_override} {class_modifiers} {class}"
    );

    rsx! {
        div {
            class: "sd-table-container",
            id: id,
            table {
                class: "{combined_table_class}",
                {children}
            }
        }
    }
}

/// Intestazione della tabella (`<thead>`).
#[component]
pub fn TableHeader(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        thead {
            class: "sd-table-header {class}",
            {children}
        }
    }
}

/// Corpo della tabella (`<tbody>`).
#[component]
pub fn TableBody(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        tbody {
            class: "sd-table-body {class}",
            {children}
        }
    }
}

/// Piè di pagina della tabella (`<tfoot>`).
#[component]
pub fn TableFooter(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    rsx! {
        tfoot {
            class: "sd-table-footer {class}",
            {children}
        }
    }
}

/// Singola riga della tabella (`<tr>`) con supporto a stato selezionato e interattività click.
#[component]
pub fn TableRow(
    #[props(default)] selected: bool,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let sel_class = if selected { "sd-table-row--selected" } else { "" };
    let is_clickable = onclick.is_some();
    let clickable_class = if is_clickable { "sd-table-row--clickable" } else { "" };

    rsx! {
        tr {
            class: "sd-table-row {sel_class} {clickable_class} {class}",
            tabindex: if is_clickable { Some("0") } else { None },
            "aria-selected": if selected { Some("true") } else { None },
            onclick: move |e| {
                if let Some(h) = &onclick {
                    h.call(e);
                }
            },
            {children}
        }
    }
}

/// Singola cella della tabella: cella dati (`<td>`) o intestazione (`<th>`), con supporto ad allineamento e ordinamento interattivo.
#[component]
pub fn TableCell(
    #[props(default)] is_header: bool,
    #[props(default)] align: TableAlign,
    #[props(default)] sort_direction: Option<SortDirection>,
    #[props(default)] on_sort: Option<EventHandler<()>>,
    #[props(default)] col_span: Option<usize>,
    #[props(default)] class: String,
    #[props(default)] style: Option<String>,
    children: Element,
) -> Element {
    let align_class = align.scss_class();
    let sortable = on_sort.is_some();
    let on_sort_kd = on_sort;
    let sortable_class = if on_sort.is_some() { "sd-table-cell--sortable" } else { "" };
    let sort_dir_class = sort_direction.map(|d| d.scss_class()).unwrap_or("");

    let combined_class = format!("sd-table-cell {align_class} {sortable_class} {sort_dir_class} {class}");

    let cs_attr = col_span.map(|n| n.to_string());
    if is_header {
        let aria_sort = sort_direction.map(|dir| match dir {
            SortDirection::Ascending => "ascending",
            SortDirection::Descending => "descending",
            SortDirection::None => "none",
        });

        rsx! {
            th {
                class: "{combined_class}",
                style: style,
                colspan: cs_attr.as_deref(),
                aria_sort: aria_sort,
                tabindex: if sortable { Some("0") } else { None },
                onclick: move |_| {
                    if let Some(h) = &on_sort {
                        h.call(());
                    }
                },
                onkeydown: move |e: KeyboardEvent| {
                    if e.key() == Key::Enter || e.key() == Key::Character(" ".to_string()) {
                        e.prevent_default();
                        if let Some(h) = &on_sort_kd {
                            h.call(());
                        }
                    }
                },
                div {
                    class: "sd-table-cell__inner",
                    span { class: "sd-table-cell__label", {children} }
                    if let Some(dir) = sort_direction {
                        span {
                            class: "sd-table-cell__sort-icon",
                            aria_hidden: "true",
                            match dir {
                                SortDirection::Ascending => rsx! { "▲" },
                                SortDirection::Descending => rsx! { "▼" },
                                SortDirection::None => rsx! { "⇅" },
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            td {
                class: "{combined_class}",
                style: style,
                colspan: cs_attr.as_deref(),
                {children}
            }
        }
    }
}

/// Funzione di render per una cella della tabella.
pub type TableCellRender<T> = Arc<dyn Fn(&T) -> Element + Send + Sync>;

/// Funzione comparatore per l'ordinamento di due righe della tabella.
pub type TableComparator<T> = Arc<dyn Fn(&T, &T) -> std::cmp::Ordering + Send + Sync>;

/// Definizione di colonna per il componente `DataTable`.
#[derive(Clone)]
pub struct DataTableColumn<T: 'static> {
    pub key: String,
    pub header: String,
    pub sortable: bool,
    pub align: TableAlign,
    pub width: Option<String>,
    pub render: TableCellRender<T>,
    pub comparator: Option<TableComparator<T>>,
}

impl<T: 'static> PartialEq for DataTableColumn<T> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && self.header == other.header
            && self.sortable == other.sortable
            && self.align == other.align
            && self.width == other.width
    }
}

impl<T: 'static> DataTableColumn<T> {
    pub fn new(
        key: impl Into<String>,
        header: impl Into<String>,
        render: impl Fn(&T) -> Element + Send + Sync + 'static,
    ) -> Self {
        Self {
            key: key.into(),
            header: header.into(),
            sortable: true,
            align: TableAlign::Left,
            width: None,
            render: Arc::new(render),
            comparator: None,
        }
    }

    pub fn text(
        key: impl Into<String>,
        header: impl Into<String>,
        accessor: impl Fn(&T) -> String + Send + Sync + 'static,
    ) -> Self {
        let key_str = key.into();
        let header_str = header.into();
        let acc = Arc::new(accessor);
        let acc_clone = acc.clone();
        Self {
            key: key_str,
            header: header_str,
            sortable: true,
            align: TableAlign::Left,
            width: None,
            render: Arc::new(move |item| {
                let val = acc(item);
                rsx! { "{val}" }
            }),
            comparator: Some(Arc::new(move |a, b| acc_clone(a).cmp(&acc_clone(b)))),
        }
    }

    #[must_use]
    pub fn with_sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    #[must_use]
    pub fn with_align(mut self, align: TableAlign) -> Self {
        self.align = align;
        self
    }

    #[must_use]
    pub fn with_width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }

    #[must_use]
    pub fn with_comparator<F>(mut self, cmp: F) -> Self
    where
        F: Fn(&T, &T) -> std::cmp::Ordering + Send + Sync + 'static,
    {
        self.comparator = Some(Arc::new(cmp));
        self.sortable = true;
        self
    }

    #[must_use]
    pub fn with_sort_by<K, F>(mut self, key_extractor: F) -> Self
    where
        K: Ord + 'static,
        F: Fn(&T) -> K + Send + Sync + 'static,
    {
        self.comparator = Some(Arc::new(move |a, b| key_extractor(a).cmp(&key_extractor(b))));
        self.sortable = true;
        self
    }
}

/// Componente DataTable generico su tipo riga `T`.
/// Include ordinamento interattivo, paginazione dinamica, stato vuoto e overlay di caricamento.
#[component]
pub fn DataTable<T: Clone + PartialEq + 'static>(
    data: Vec<T>,
    columns: Vec<DataTableColumn<T>>,
    #[props(default)] page: Option<Signal<usize>>,
    #[props(default = 10)] page_size: usize,
    #[props(default)] total_items: Option<usize>,
    #[props(default)] sort_key: Option<Signal<Option<String>>>,
    #[props(default)] sort_direction: Option<Signal<SortDirection>>,
    #[props(default)] on_sort: Option<EventHandler<(String, SortDirection)>>,
    #[props(default)] on_row_click: Option<EventHandler<T>>,
    #[props(default)] loading: bool,
    #[props(default)] empty_state: Option<Element>,
    #[props(default)] striped: bool,
    #[props(default)] bordered: bool,
    #[props(default)] compact: bool,
    #[props(default = true)] hoverable: bool,
    #[props(default)] class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let mut internal_sort_key = use_signal(|| None::<String>);
    let mut internal_sort_dir = use_signal(|| SortDirection::None);

    let active_sort_key = match sort_key {
        Some(s) => s(),
        None => internal_sort_key(),
    };
    let active_sort_dir = match sort_direction {
        Some(s) => s(),
        None => internal_sort_dir(),
    };

    let current_page = page.map(|p| p()).unwrap_or(1);

    // Ordinamento client-side se è attiva una chiave di ordinamento e la colonna fornisce un comparatore
    let sorted_data = if let (Some(k), dir) = (&active_sort_key, active_sort_dir) {
        if dir != SortDirection::None {
            if let Some(col) = columns.iter().find(|c| &c.key == k) {
                if let Some(comp) = &col.comparator {
                    let mut items = data.clone();
                    items.sort_by(|a, b| {
                        let ord = comp(a, b);
                        if dir == SortDirection::Descending {
                            ord.reverse()
                        } else {
                            ord
                        }
                    });
                    items
                } else {
                    data.clone()
                }
            } else {
                data.clone()
            }
        } else {
            data.clone()
        }
    } else {
        data.clone()
    };

    let safe_page_size = page_size.max(1);
    let total = total_items.unwrap_or(sorted_data.len());
    let total_pages = if total == 0 { 1 } else { total.div_ceil(safe_page_size) };

    // Slice dei dati per paginazione client-side se total_items non è fornito
    let display_items = if total_items.is_none() && sorted_data.len() > safe_page_size {
        let start = (current_page.saturating_sub(1)) * safe_page_size;
        let end = (start + safe_page_size).min(sorted_data.len());
        if start < sorted_data.len() {
            sorted_data[start..end].to_vec()
        } else {
            vec![]
        }
    } else {
        sorted_data
    };

    rsx! {
        div { class: "sd-data-table-wrapper {class}",
            crate::loading_overlay::LoadingOverlay {
                visible: loading,
                description: Some(locale.table_loading.clone()),
            Table {
                striped: striped,
                bordered: bordered,
                compact: compact,
                hoverable: hoverable,
                TableHeader {
                    TableRow {
                        for col in &columns {
                            {
                                let col_key = col.key.clone();
                                let is_active_col = active_sort_key.as_ref() == Some(&col_key);
                                let col_dir = if is_active_col {
                                    Some(active_sort_dir)
                                } else if col.sortable {
                                    Some(SortDirection::None)
                                } else {
                                    None
                                };

                                let on_sort_handler = if col.sortable {
                                    let c_key = col_key.clone();
                                    Some(EventHandler::new(move |_| {
                                        let next_dir = if is_active_col {
                                            active_sort_dir.next()
                                        } else {
                                            SortDirection::Ascending
                                        };

                                        let next_key = if next_dir == SortDirection::None {
                                            None
                                        } else {
                                            Some(c_key.clone())
                                        };

                                        if let Some(mut sk) = sort_key {
                                            sk.set(next_key.clone());
                                        } else {
                                            internal_sort_key.set(next_key.clone());
                                        }

                                        if let Some(mut sd) = sort_direction {
                                            sd.set(next_dir);
                                        } else {
                                            internal_sort_dir.set(next_dir);
                                        }

                                        if let Some(mut p) = page {
                                            p.set(1);
                                        }

                                        if let Some(h) = &on_sort {
                                            h.call((c_key.clone(), next_dir));
                                        }
                                    }))
                                } else {
                                    None
                                };

                                rsx! {
                                    TableCell {
                                        key: "{col_key}",
                                        is_header: true,
                                        align: col.align,
                                        sort_direction: col_dir,
                                        on_sort: on_sort_handler,
                                        style: col.width.as_ref().map(|w| format!("width: {w};")),
                                        "{col.header}"
                                    }
                                }
                            }
                        }
                    }
                }
                TableBody {
                    if display_items.is_empty() {
                        TableRow {
                            TableCell {
                                class: "sd-table-cell--empty",
                                col_span: Some(columns.len()),
                                if let Some(empty) = empty_state {
                                    {empty}
                                } else {
                                    div { class: "sd-table__empty", "{locale.table_empty}" }
                                }
                            }
                        }
                    } else {
                        for (idx, item) in display_items.iter().enumerate() {
                            {
                                let item_clone = item.clone();
                                let on_click_row = on_row_click.as_ref().map(|h| {
                                    let it = item_clone.clone();
                                    let h_clone = *h;
                                    EventHandler::new(move |_| h_clone.call(it.clone()))
                                });

                                rsx! {
                                    TableRow {
                                        key: "{idx}",
                                        onclick: on_click_row,
                                        for col in &columns {
                                            {
                                                let cell_element = (col.render)(item);
                                                rsx! {
                                                    TableCell {
                                                        key: "{col.key}",
                                                        align: col.align,
                                                        {cell_element}
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

            // Paginazione (componente della libreria) se `page` è fornito
            if let Some(p_sig) = page {
                div { class: "sd-data-table__pagination",
                    div {
                        "{locale.table_items}: "
                        span { class: "sd-data-table__count", "{total}" }
                        " • {locale.table_page} "
                        span { class: "sd-data-table__count", "{current_page}" }
                        " {locale.table_of} "
                        span { class: "sd-data-table__count", "{total_pages}" }
                    }
                    crate::pagination::Pagination {
                        page: p_sig,
                        total: total_pages,
                        size: crate::a11y::ControlSize::Sm,
                    }
                }
            }
        }
    }
}
