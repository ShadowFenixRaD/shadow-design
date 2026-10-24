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
        div { class: "sc-props",
        Table {
            hoverable: false,
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
                        TableCell { class: "sc-props__name", "{row.name}" }
                        TableCell { class: "sc-props__type", "{row.r#type}" }
                        TableCell { class: "sc-props__default", "{row.default}" }
                        TableCell { class: "sc-props__desc", "{row.description}" }
                    }
                }
            }
        }
        }
    }
}
