use crate::components::props_table::{PropRow, PropsTable};
use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn ShowcaseTabs(
    preview: Element,
    code: String,
    props: Vec<PropRow>,
    #[props(default = "rsx")] language: &'static str,
) -> Element {
    let active_tab = use_signal(|| "playground".to_string());

    rsx! {
        div { class: "rounded-2xl border border-white/[0.08] bg-[#09090d] shadow-xl overflow-visible shadow-[inset_0_1px_0_0_rgba(255,255,255,0.06)] relative",
            TabList {
                aria_label: "Modalità espositore",
                Tab {
                    value: "playground",
                    active: active_tab,
                    icon: Some(IconKind::Eye),
                    "Playground"
                }
                Tab {
                    value: "code",
                    active: active_tab,
                    icon: Some(IconKind::Code),
                    "Codice RSX"
                }
                Tab {
                    value: "api",
                    active: active_tab,
                    icon: Some(IconKind::Layers),
                    "API Props"
                }
            }
            div { class: "p-5 sm:p-6 rounded-b-2xl overflow-visible",
                TabPanel {
                    value: "playground",
                    active: active_tab,
                    {preview}
                }
                TabPanel {
                    value: "code",
                    active: active_tab,
                    CodeBlock {
                        code: code,
                        language: language,
                    }
                }
                TabPanel {
                    value: "api",
                    active: active_tab,
                    PropsTable { rows: props }
                }
            }
        }
    }
}
