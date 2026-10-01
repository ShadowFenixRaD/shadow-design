use dioxus::prelude::*;

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
        div { class: "rounded-xl border border-white/[0.08] bg-[#060608] overflow-hidden shadow-inner",
            div { class: "overflow-x-auto",
                table { class: "w-full text-left text-xs border-collapse",
                    thead {
                        tr { class: "border-b border-white/[0.08] bg-[#0c0c12]/90 text-zinc-400 font-mono text-[11px]",
                            th { class: "px-4 py-2.5 font-semibold", "Prop" }
                            th { class: "px-4 py-2.5 font-semibold", "Tipo Rust" }
                            th { class: "px-4 py-2.5 font-semibold", "Default" }
                            th { class: "px-4 py-2.5 font-semibold", "Descrizione" }
                        }
                    }
                    tbody { class: "divide-y divide-white/[0.04]",
                        for row in rows {
                            tr { class: "hover:bg-white/[0.02] transition-colors",
                                td { class: "px-4 py-2.5 font-mono text-purple-300 font-medium", "{row.name}" }
                                td { class: "px-4 py-2.5 font-mono text-zinc-400", "{row.r#type}" }
                                td { class: "px-4 py-2.5 font-mono text-zinc-500", "{row.default}" }
                                td { class: "px-4 py-2.5 text-zinc-300 leading-normal", "{row.description}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
