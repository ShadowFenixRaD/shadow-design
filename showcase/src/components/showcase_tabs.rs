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

    let base_id = use_hook(|| {
        static ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        format!("sd-showcase-tabs-{}", ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    });

    let playground_tab_id = format!("{base_id}-tab-playground");
    let code_tab_id = format!("{base_id}-tab-code");
    let api_tab_id = format!("{base_id}-tab-api");

    let playground_panel_id = format!("{base_id}-panel-playground");
    let code_panel_id = format!("{base_id}-panel-code");
    let api_panel_id = format!("{base_id}-panel-api");

    let theme_val = use_theme()();

    let tab_btn_base = "flex items-center gap-1.5 px-3 py-1.5 text-xs font-medium rounded-lg transition-all cursor-pointer select-none focus:outline-none focus-visible:ring-2 focus-visible:ring-purple-400";

    let active_class = match theme_val.primary_color {
        PrimaryColor::Crimson => "bg-rose-950/70 text-rose-200 border border-rose-500/40 shadow-[0_0_12px_rgba(244,63,94,0.3)]",
        PrimaryColor::Cyan => "bg-cyan-950/70 text-cyan-200 border border-cyan-500/40 shadow-[0_0_12px_rgba(6,182,212,0.3)]",
        PrimaryColor::Emerald => "bg-emerald-950/70 text-emerald-200 border border-emerald-500/40 shadow-[0_0_12px_rgba(16,185,129,0.3)]",
        PrimaryColor::Obsidian => "bg-zinc-800/80 text-zinc-100 border border-zinc-600/50 shadow-[0_0_10px_rgba(255,255,255,0.15)]",
        _ => "bg-purple-950/70 text-purple-200 border border-purple-500/40 shadow-[0_0_12px_rgba(168,85,247,0.3)]",
    };

    let inactive_class = "text-zinc-400 hover:text-zinc-200 hover:bg-white/[0.04] border border-transparent";

    let handle_keydown = move |e: KeyboardEvent| {
        let tabs = [ShowcaseTab::Playground, ShowcaseTab::Code, ShowcaseTab::Api];
        let curr_idx = tabs.iter().position(|t| *t == current_tab()).unwrap_or(0);
        match e.key() {
            Key::ArrowRight => {
                e.prevent_default();
                let next = tabs[(curr_idx + 1) % tabs.len()];
                current_tab.set(next);
            }
            Key::ArrowLeft => {
                e.prevent_default();
                let prev = tabs[(curr_idx + tabs.len() - 1) % tabs.len()];
                current_tab.set(prev);
            }
            Key::Home => {
                e.prevent_default();
                current_tab.set(ShowcaseTab::Playground);
            }
            Key::End => {
                e.prevent_default();
                current_tab.set(ShowcaseTab::Api);
            }
            _ => {}
        }
    };

    rsx! {
        div { class: "rounded-2xl border border-white/[0.08] bg-[#09090d] shadow-xl overflow-visible shadow-[inset_0_1px_0_0_rgba(255,255,255,0.06)] relative",
            // Tab Header Bar
            div { class: "flex items-center justify-between px-4 py-2.5 border-b border-white/[0.06] bg-[#0c0c12]/90 rounded-t-2xl",
                div {
                    role: "tablist",
                    "aria-label": "Modalità espositore",
                    class: "flex items-center gap-1.5 bg-[#060608] p-1 rounded-xl border border-white/[0.06]",
                    button {
                        r#type: "button",
                        role: "tab",
                        id: "{playground_tab_id}",
                        "aria-controls": "{playground_panel_id}",
                        "aria-selected": current_tab() == ShowcaseTab::Playground,
                        tabindex: if current_tab() == ShowcaseTab::Playground { "0" } else { "-1" },
                        class: if current_tab() == ShowcaseTab::Playground {
                            "{tab_btn_base} {active_class}"
                        } else {
                            "{tab_btn_base} {inactive_class}"
                        },
                        onclick: move |_| current_tab.set(ShowcaseTab::Playground),
                        onkeydown: handle_keydown,
                        Icon { kind: IconKind::Eye, size: 14 }
                        "Playground"
                    }
                    button {
                        r#type: "button",
                        role: "tab",
                        id: "{code_tab_id}",
                        "aria-controls": "{code_panel_id}",
                        "aria-selected": current_tab() == ShowcaseTab::Code,
                        tabindex: if current_tab() == ShowcaseTab::Code { "0" } else { "-1" },
                        class: if current_tab() == ShowcaseTab::Code {
                            "{tab_btn_base} {active_class}"
                        } else {
                            "{tab_btn_base} {inactive_class}"
                        },
                        onclick: move |_| current_tab.set(ShowcaseTab::Code),
                        onkeydown: handle_keydown,
                        Icon { kind: IconKind::Code, size: 14 }
                        "Codice RSX"
                    }
                    button {
                        r#type: "button",
                        role: "tab",
                        id: "{api_tab_id}",
                        "aria-controls": "{api_panel_id}",
                        "aria-selected": current_tab() == ShowcaseTab::Api,
                        tabindex: if current_tab() == ShowcaseTab::Api { "0" } else { "-1" },
                        class: if current_tab() == ShowcaseTab::Api {
                            "{tab_btn_base} {active_class}"
                        } else {
                            "{tab_btn_base} {inactive_class}"
                        },
                        onclick: move |_| current_tab.set(ShowcaseTab::Api),
                        onkeydown: handle_keydown,
                        Icon { kind: IconKind::Layers, size: 14 }
                        "API Props"
                    }
                }
                span { class: "text-[11px] font-mono text-zinc-500 hidden sm:inline", "Dioxus 0.7 Component" }
            }

            // Tab Content
            div { class: "p-5 sm:p-6 rounded-b-2xl overflow-visible",
                div {
                    role: "tabpanel",
                    id: "{playground_panel_id}",
                    "aria-labelledby": "{playground_tab_id}",
                    tabindex: "0",
                    hidden: current_tab() != ShowcaseTab::Playground,
                    if current_tab() == ShowcaseTab::Playground {
                        {preview}
                    }
                }
                div {
                    role: "tabpanel",
                    id: "{code_panel_id}",
                    "aria-labelledby": "{code_tab_id}",
                    tabindex: "0",
                    hidden: current_tab() != ShowcaseTab::Code,
                    if current_tab() == ShowcaseTab::Code {
                        CodeBlock { code, language }
                    }
                }
                div {
                    role: "tabpanel",
                    id: "{api_panel_id}",
                    "aria-labelledby": "{api_tab_id}",
                    tabindex: "0",
                    hidden: current_tab() != ShowcaseTab::Api,
                    if current_tab() == ShowcaseTab::Api {
                        PropsTable { rows: props }
                    }
                }
            }
        }
    }
}
