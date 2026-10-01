use crate::components::code_block::CodeBlock;
use crate::components::props_table::{PropRow, PropsTable};
use dioxus::prelude::*;
use shadow_design::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ShowcaseTab {
    #[default]
    Playground,
    Code,
    Api,
}

#[component]
pub fn ShowcaseTabs(
    preview: Element,
    code: String,
    props: Vec<PropRow>,
    #[props(default = "rsx")] language: &'static str,
) -> Element {
    let mut current_tab = use_signal(|| ShowcaseTab::Playground);

    let tab_btn_base = "flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium rounded-lg transition-all cursor-pointer select-none";

    rsx! {
        div { class: "rounded-2xl border border-white/[0.08] bg-[#09090d] shadow-xl overflow-visible shadow-[inset_0_1px_0_0_rgba(255,255,255,0.06)] relative",
            // Tab Header Bar
            div { class: "flex items-center justify-between px-4 py-2.5 border-b border-white/[0.06] bg-[#0c0c12]/90 rounded-t-2xl",
                div { class: "flex items-center gap-1.5 bg-[#060608] p-1 rounded-xl border border-white/[0.06]",
                    button {
                        r#type: "button",
                        class: if current_tab() == ShowcaseTab::Playground {
                            "{tab_btn_base} bg-purple-950/60 text-purple-200 border border-purple-500/30 shadow-sm"
                        } else {
                            "{tab_btn_base} text-zinc-400 hover:text-zinc-200 hover:bg-white/[0.04]"
                        },
                        onclick: move |_| current_tab.set(ShowcaseTab::Playground),
                        Icon { kind: IconKind::Eye, size: 14 }
                        "Playground"
                    }
                    button {
                        r#type: "button",
                        class: if current_tab() == ShowcaseTab::Code {
                            "{tab_btn_base} bg-purple-950/60 text-purple-200 border border-purple-500/30 shadow-sm"
                        } else {
                            "{tab_btn_base} text-zinc-400 hover:text-zinc-200 hover:bg-white/[0.04]"
                        },
                        onclick: move |_| current_tab.set(ShowcaseTab::Code),
                        Icon { kind: IconKind::Code, size: 14 }
                        "Codice RSX"
                    }
                    button {
                        r#type: "button",
                        class: if current_tab() == ShowcaseTab::Api {
                            "{tab_btn_base} bg-purple-950/60 text-purple-200 border border-purple-500/30 shadow-sm"
                        } else {
                            "{tab_btn_base} text-zinc-400 hover:text-zinc-200 hover:bg-white/[0.04]"
                        },
                        onclick: move |_| current_tab.set(ShowcaseTab::Api),
                        Icon { kind: IconKind::Layers, size: 14 }
                        "API Props"
                    }
                }
                span { class: "text-[11px] font-mono text-zinc-500 hidden sm:inline", "Dioxus 0.7 Component" }
            }

            // Tab Content
            div { class: "p-5 sm:p-6 rounded-b-2xl overflow-visible",
                match current_tab() {
                    ShowcaseTab::Playground => rsx! {
                        {preview}
                    },
                    ShowcaseTab::Code => rsx! {
                        CodeBlock { code, language }
                    },
                    ShowcaseTab::Api => rsx! {
                        PropsTable { rows: props }
                    },
                }
            }
        }
    }
}
