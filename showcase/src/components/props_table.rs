use dioxus::prelude::*;
use shadow_design::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct PropRow {
    pub name: &'static str,
    pub r#type: &'static str,
    pub default: &'static str,
    pub description: &'static str,
}

#[component]
pub fn PropsTable(rows: Vec<PropRow>) -> Element {
    rsx! {
        Table {
            hoverable: true,
            bordered: true,
            TableHeader {
                TableRow {
                    TableCell { is_header: true, "Prop" }
                    TableCell { is_header: true, "Tipo Rust" }
                    TableCell { is_header: true, "Default" }
                    TableCell { is_header: true, "Descrizione" }
                }
            }
            TableBody {
                for row in rows {
                    TableRow {
                        TableCell { class: "font-mono text-purple-300 font-medium", "{row.name}" }
                        TableCell { class: "font-mono text-zinc-400", "{row.r#type}" }
                        TableCell { class: "font-mono text-zinc-500", "{row.default}" }
                        TableCell { class: "text-zinc-300 leading-normal", "{row.description}" }
                    }
                }
            }
        }
    }
}
