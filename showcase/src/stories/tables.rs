//! Story: DataTable and the Table primitives (feature `table`). The playground owns the page
//! signal, so it goes through `TableDemo { knobs }`; data and columns are printed in the prelude.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::scenes::OpsDashboard;
use crate::story::schema::*;

#[derive(Clone, PartialEq)]
struct Peer {
    name: &'static str,
    region: &'static str,
    status: &'static str,
    latency: f64,
    throughput: u32,
}

const PEERS: [Peer; 8] = [
    Peer { name: "core-01", region: "Amsterdam", status: "Online", latency: 0.6, throughput: 212 },
    Peer { name: "edge-07", region: "Milano", status: "Online", latency: 0.9, throughput: 128 },
    Peer { name: "edge-12", region: "Francoforte", status: "Online", latency: 1.4, throughput: 94 },
    Peer { name: "edge-15", region: "Vienna", status: "Online", latency: 1.9, throughput: 71 },
    Peer { name: "relay-03", region: "Parigi", status: "Degradato", latency: 3.8, throughput: 31 },
    Peer { name: "relay-05", region: "Varsavia", status: "Degradato", latency: 4.6, throughput: 22 },
    Peer { name: "edge-21", region: "Madrid", status: "Offline", latency: 0.0, throughput: 0 },
    Peer { name: "edge-30", region: "Lisbona", status: "Offline", latency: 0.0, throughput: 0 },
];

fn status_variant(status: &str) -> BadgeVariant {
    match status {
        "Online" => BadgeVariant::Success,
        "Degradato" => BadgeVariant::Warning,
        _ => BadgeVariant::Danger,
    }
}

fn columns() -> Vec<DataTableColumn<Peer>> {
    vec![
        DataTableColumn::new("name", "Nodo", |p: &Peer| {
            let name = p.name;
            rsx! {
                Code { "{name}" }
            }
        })
        .with_sort_by(|p: &Peer| p.name),
        DataTableColumn::text("region", "Regione", |p: &Peer| p.region.to_string()).with_sort_by(|p: &Peer| p.region),
        DataTableColumn::new("status", "Stato", |p: &Peer| {
            let status = p.status;
            rsx! {
                Badge { label: status.to_string(), variant: status_variant(status), pill: true, dot: true }
            }
        })
        .with_sort_by(|p: &Peer| p.status),
        DataTableColumn::text("latency", "Latenza", |p: &Peer| {
            if p.throughput == 0 { "—".to_string() } else { format!("{:.1} ms", p.latency) }
        })
        .with_align(TableAlign::Right)
        .with_sort_by(|p: &Peer| (p.latency * 10.0) as i64),
        DataTableColumn::text("throughput", "Throughput", |p: &Peer| format!("{} MB/s", p.throughput))
            .with_align(TableAlign::Right)
            .with_sort_by(|p: &Peer| p.throughput),
    ]
}

const PRELUDE: &str = r#"#[derive(Clone, PartialEq)]
struct Peer { name: &'static str, region: &'static str, status: &'static str, latency: f64, throughput: u32 }

fn status_variant(status: &str) -> BadgeVariant {
    match status {
        "Online" => BadgeVariant::Success,
        "Degradato" => BadgeVariant::Warning,
        _ => BadgeVariant::Danger,
    }
}

let page = use_signal(|| 1usize);
let data = vec![
    Peer { name: "core-01", region: "Amsterdam", status: "Online", latency: 0.6, throughput: 212 },
    Peer { name: "edge-07", region: "Milano", status: "Online", latency: 0.9, throughput: 128 },
    Peer { name: "edge-12", region: "Francoforte", status: "Online", latency: 1.4, throughput: 94 },
    Peer { name: "edge-15", region: "Vienna", status: "Online", latency: 1.9, throughput: 71 },
    Peer { name: "relay-03", region: "Parigi", status: "Degradato", latency: 3.8, throughput: 31 },
    Peer { name: "relay-05", region: "Varsavia", status: "Degradato", latency: 4.6, throughput: 22 },
    Peer { name: "edge-21", region: "Madrid", status: "Offline", latency: 0.0, throughput: 0 },
    Peer { name: "edge-30", region: "Lisbona", status: "Offline", latency: 0.0, throughput: 0 },
];
let columns = vec![
    DataTableColumn::new("name", "Nodo", |p: &Peer| {
        let name = p.name;
        rsx! { Code { "{name}" } }
    })
    .with_sort_by(|p: &Peer| p.name),
    DataTableColumn::text("region", "Regione", |p: &Peer| p.region.to_string()).with_sort_by(|p: &Peer| p.region),
    DataTableColumn::new("status", "Stato", |p: &Peer| {
        let status = p.status;
        rsx! { Badge { label: status.to_string(), variant: status_variant(status), pill: true, dot: true } }
    })
    .with_sort_by(|p: &Peer| p.status),
    DataTableColumn::text("latency", "Latenza", |p: &Peer| {
        if p.throughput == 0 { "—".to_string() } else { format!("{:.1} ms", p.latency) }
    })
    .with_align(TableAlign::Right)
    .with_sort_by(|p: &Peer| (p.latency * 10.0) as i64),
    DataTableColumn::text("throughput", "Throughput", |p: &Peer| format!("{} MB/s", p.throughput))
        .with_align(TableAlign::Right)
        .with_sort_by(|p: &Peer| p.throughput),
];"#;

#[component]
fn TableDemo(knobs: Knobs) -> Element {
    let page = use_signal(|| 1usize);
    rsx! {
        DataTable {
            data: PEERS.to_vec(),
            columns: columns(),
            page: knobs.bool("paginated").then_some(page),
            page_size: knobs.number("page_size") as usize,
            striped: knobs.bool("striped"),
            bordered: knobs.bool("bordered"),
            hoverable: knobs.bool("hoverable"),
            compact: knobs.bool("compact"),
            loading: knobs.bool("loading"),
        }
    }
}

fn render(k: &Knobs) -> Element {
    rsx! {
        TableDemo { knobs: k.clone() }
    }
}

#[component]
fn Primitives() -> Element {
    rsx! {
        Table { compact: true,
            TableHeader {
                TableRow {
                    TableCell { is_header: true, "Regione" }
                    TableCell { is_header: true, align: TableAlign::Right, "Nodi" }
                    TableCell { is_header: true, align: TableAlign::Right, "Peer" }
                }
            }
            TableBody {
                TableRow {
                    TableCell { "Europa occidentale" }
                    TableCell { align: TableAlign::Right, "12" }
                    TableCell { align: TableAlign::Right, "418" }
                }
                TableRow { selected: true,
                    TableCell { "Europa centrale" }
                    TableCell { align: TableAlign::Right, "8" }
                    TableCell { align: TableAlign::Right, "261" }
                }
                TableRow {
                    TableCell { "Penisola iberica" }
                    TableCell { align: TableAlign::Right, "4" }
                    TableCell { align: TableAlign::Right, "57" }
                }
            }
            TableFooter {
                TableRow {
                    TableCell { "Totale" }
                    TableCell { align: TableAlign::Right, "24" }
                    TableCell { align: TableAlign::Right, "736" }
                }
            }
        }
    }
}
const PRIMITIVES_CODE: &str = r#"Table { compact: true,
    TableHeader {
        TableRow {
            TableCell { is_header: true, "Regione" }
            TableCell { is_header: true, align: TableAlign::Right, "Nodi" }
            TableCell { is_header: true, align: TableAlign::Right, "Peer" }
        }
    }
    TableBody {
        TableRow {
            TableCell { "Europa occidentale" }
            TableCell { align: TableAlign::Right, "12" }
            TableCell { align: TableAlign::Right, "418" }
        }
        TableRow { selected: true,
            TableCell { "Europa centrale" }
            TableCell { align: TableAlign::Right, "8" }
            TableCell { align: TableAlign::Right, "261" }
        }
        TableRow {
            TableCell { "Penisola iberica" }
            TableCell { align: TableAlign::Right, "4" }
            TableCell { align: TableAlign::Right, "57" }
        }
    }
    TableFooter {
        TableRow {
            TableCell { "Totale" }
            TableCell { align: TableAlign::Right, "24" }
            TableCell { align: TableAlign::Right, "736" }
        }
    }
}"#;

#[component]
fn Empty() -> Element {
    let mut toast = use_notification();
    rsx! {
        DataTable {
            data: Vec::<Peer>::new(),
            columns: columns(),
            empty_state: rsx! {
                Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
                    Icon { kind: IconKind::Server, size: 24 }
                    Text { size: TextSize::Sm, "Nessun nodo in questa regione" }
                    Button {
                        size: ButtonSize::Sm,
                        variant: ButtonVariant::Secondary,
                        onclick: move |_| {
                            toast.success("Provisioning avviato");
                        },
                        "Aggiungi un nodo"
                    }
                }
            },
        }
    }
}
const EMPTY_CODE: &str = r#"let mut toast = use_notification();

DataTable {
    data: Vec::<Peer>::new(),
    columns: columns(),
    empty_state: rsx! {
        Stack { gap: SpacingToken::Sm, align: AlignItems::Center,
            Icon { kind: IconKind::Server, size: 24 }
            Text { size: TextSize::Sm, "Nessun nodo in questa regione" }
            Button {
                size: ButtonSize::Sm,
                variant: ButtonVariant::Secondary,
                onclick: move |_| {
                    toast.success("Provisioning avviato");
                },
                "Aggiungi un nodo"
            }
        }
    },
}"#;

#[component]
fn RowClick() -> Element {
    let mut selected = use_signal(|| None::<&'static str>);
    let shown = selected().unwrap_or("nessuno");
    rsx! {
        Stack { gap: SpacingToken::Sm,
            DataTable {
                data: PEERS[..4].to_vec(),
                columns: columns(),
                compact: true,
                on_row_click: move |p: Peer| selected.set(Some(p.name)),
            }
            Text { size: TextSize::Sm, dimmed: true, "Nodo aperto: {shown}" }
        }
    }
}
const ROW_CLICK_CODE: &str = r#"let mut selected = use_signal(|| None::<&'static str>);
let shown = selected().unwrap_or("nessuno");

Stack { gap: SpacingToken::Sm,
    DataTable {
        data: PEERS[..4].to_vec(),
        columns: columns(),
        compact: true,
        on_row_click: move |p: Peer| selected.set(Some(p.name)),
    }
    Text { size: TextSize::Sm, dimmed: true, "Nodo aperto: {shown}" }
}"#;

#[component]
fn InContext() -> Element {
    rsx! {
        OpsDashboard {}
    }
}
const IN_CONTEXT_CODE: &str = r#"// scenes/ops_dashboard.rs (estratto): la tabella dei nodi sotto il grafico del traffico.
DataTable { data: NODES.to_vec(), columns: columns(compact), page_size: 5, compact: true }"#;

const DATA_PROPS: &[PropDoc] = &[
    prop("data", "Vec<T>", "-", "Righe; T: Clone + PartialEq."),
    prop("columns", "Vec<DataTableColumn<T>>", "-", "Colonne: chiave, intestazione, render, allineamento, ordinamento."),
    prop("page", "Option<Signal<usize>>", "None", "Pagina corrente (da 1); attiva la barra di paginazione."),
    prop("page_size", "usize", "10", "Righe per pagina."),
    prop("total_items", "Option<usize>", "None", "Totale lato server: disattiva lo slicing client-side."),
    prop("sort_key / sort_direction", "Option<Signal<…>>", "None", "Ordinamento controllato; senza, la tabella lo gestisce da sola."),
    prop("on_sort", "Option<EventHandler<(String, SortDirection)>>", "None", "Cambio di colonna o direzione."),
    prop("on_row_click", "Option<EventHandler<T>>", "None", "Clic su una riga, riceve l'elemento."),
    prop("loading", "bool", "false", "LoadingOverlay sopra la tabella."),
    prop("empty_state", "Option<Element>", "None", "Contenuto quando non ci sono righe (testo da Locale se assente)."),
    prop("striped / bordered / compact", "bool", "false", "Righe alternate, bordi tra le celle, padding ridotto."),
    prop("hoverable", "bool", "true", "Evidenzia la riga sotto il puntatore."),
    prop("class", "String", "\"\"", "Classi aggiuntive sul wrapper."),
];
const COLUMN_PROPS: &[PropDoc] = &[
    prop("DataTableColumn::new(key, header, render)", "fn", "-", "Colonna con render personalizzato: Fn(&T) -> Element."),
    prop("DataTableColumn::text(key, header, accessor)", "fn", "-", "Colonna di testo: Fn(&T) -> String."),
    prop(".with_sort_by(|r| key)", "fn", "-", "Rende la colonna ordinabile con una chiave Ord."),
    prop(".with_comparator(|a, b| ord)", "fn", "-", "Ordinamento con un comparatore esplicito."),
    prop(".with_align(TableAlign)", "fn", "Left", "Left, Center, Right (numeri a destra)."),
    prop(".with_width(\"8rem\")", "fn", "None", "Larghezza della colonna."),
];
const TABLE_PROPS: &[PropDoc] = &[
    prop("Table", "striped, bordered, hoverable, compact, sticky_header: bool", "hoverable: true", "Contenitore <table> e sue varianti."),
    prop("TableHeader / TableBody / TableFooter", "children", "-", "thead, tbody, tfoot."),
    prop("TableRow", "selected: bool, onclick", "false", "Riga; selected usa il riempimento accent-soft e aria-selected."),
    prop("TableCell", "is_header, align, sort_direction, on_sort, col_span, style", "-", "Cella td o th; on_sort la rende ordinabile da tastiera."),
];

pub fn story() -> Story {
    Story {
        slug: "tables",
        name: "DataTable",
        category: Category::Dati,
        features: &["table"],
        icon: IconKind::Server,
        tagline: "Tabella di dati con ordinamento, paginazione, stato vuoto e caricamento, più le primitive Table per layout liberi.",
        keywords: &["table", "tabella", "data table", "griglia dati", "ordinamento", "sort", "paginazione"],
        status: Status::Stable,
        playground: Some(Playground {
            component: "DataTable",
            controls: vec![
                Control::bool("paginated", "Paginazione", true).emit(Emit::Template("page: Some(page)")),
                Control::number("page_size", "Righe per pagina", 4.0, 2.0, 8.0, 1.0, 0).emit(Emit::Always),
                Control::bool("striped", "Righe alternate", false),
                Control::bool("bordered", "Bordi", false),
                Control::bool("hoverable", "Hover", true),
                Control::bool("compact", "Compatta", false),
                Control::bool("loading", "Loading", false),
            ],
            render,
            prelude_code: Some(PRELUDE),
            fixed_code: &["data,", "columns,"],
            children_code: None,
            stage: Stage::Start,
        }),
        examples: vec![
            Example {
                id: "primitives",
                title: "Primitive Table",
                description: "Header, righe, riga selezionata e footer con i totali.",
                code: PRIMITIVES_CODE,
                render: || rsx! { Primitives {} },
                span: Span::Half,
            },
            Example {
                id: "empty",
                title: "Stato vuoto",
                description: "empty_state con un'azione per uscirne.",
                code: EMPTY_CODE,
                render: || rsx! { Empty {} },
                span: Span::Half,
            },
            Example {
                id: "row-click",
                title: "Clic sulla riga",
                description: "on_row_click riceve il record della riga.",
                code: ROW_CLICK_CODE,
                render: || rsx! { RowClick {} },
                span: Span::Half,
            },
            Example {
                id: "context",
                title: "In contesto",
                description: "La tabella dei nodi nella dashboard operativa.",
                code: IN_CONTEXT_CODE,
                render: || rsx! { InContext {} },
                span: Span::Full,
            },
        ],
        props: vec![
            PropGroup { component: "DataTable", rows: DATA_PROPS },
            PropGroup { component: "DataTableColumn", rows: COLUMN_PROPS },
            PropGroup { component: "Table", rows: TABLE_PROPS },
        ],
        recipe: Some(Recipe {
            recipe: "Container (language.md §3.1) + selected row (§3.7)",
            depth: "Flat-lit",
            tokens: &["--sd-surface-1", "--sd-border", "--sd-tint-1", "--sd-border-subtle", "--sd-accent-soft", "--sd-radius-lg", "--sd-font-mono"],
            notes: "Intestazione su --sd-tint-1; la riga selezionata ha solo il riempimento accent-soft, senza bordo.",
        }),
        a11y: &[
            "Markup <table> semantico: thead, tbody, tfoot, th e td.",
            "Le intestazioni ordinabili sono focalizzabili, si attivano con Invio o Spazio e portano aria-sort.",
            "Le righe selezionate espongono aria-selected=true.",
            "Paginazione, testo vuoto e caricamento usano le stringhe di Locale (table_page, table_empty, table_loading).",
        ],
        related: &["pagination", "lists", "badges", "loading-overlay", "virtual-list"],
    }
}
