use crate::components::props_table::{PropRow, PropsTable};
use dioxus::prelude::*;
use shadow_design::prelude::*;

/// Playground / Codice RSX / API Props panel. The tab track and the active
/// tab come from the library `Tabs`, so they follow the active preset.
#[component]
pub fn ShowcaseTabs(
    preview: Element,
    code: String,
    props: Vec<PropRow>,
    #[props(default = "rsx")] language: &'static str,
) -> Element {
    let active_tab = use_signal(|| "playground".to_string());

    rsx! {
        Tabs { class: "sc-panel",
            div { class: "sc-panel__head",
                TabList { aria_label: "Modalità espositore",
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
            }
            div { class: "sc-panel__body",
                TabPanel { value: "playground", active: active_tab, {preview} }
                TabPanel { value: "code", active: active_tab,
                    CodeBlock { code, language }
                }
                TabPanel { value: "api", active: active_tab, PropsTable { rows: props } }
            }
        }
    }
}
