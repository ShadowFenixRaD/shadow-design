use dioxus::prelude::*;
use shadow_design::prelude::*;
use crate::components::clipboard::copy_text;
use crate::components::highlight::{tokenize_rsx, TokenKind};

#[derive(Clone, Copy, PartialEq)]
enum CopyState {
    Idle,
    Copied,
    Failed,
}

#[component]
pub fn CodeBlock(
    code: String,
    #[props(default = "rsx")] language: &'static str,
    #[props(default)] class: String,
) -> Element {
    let mut copy_state = use_signal(|| CopyState::Idle);
    let code_to_copy = code.clone();
    let tokens = tokenize_rsx(&code);

    rsx! {
        div { class: "relative rounded-xl border border-white/[0.08] bg-[#060608] overflow-hidden shadow-inner {class}",
            // Header bar
            div { class: "flex items-center justify-between px-4 py-2 border-b border-white/[0.06] bg-[#0c0c12]/90 text-xs font-mono",
                span { class: "text-zinc-500 uppercase text-[10px] tracking-wider font-semibold", "{language}" }
                button {
                    r#type: "button",
                    class: "flex items-center gap-1.5 px-2.5 py-1 rounded text-[11px] font-medium transition-all cursor-pointer bg-white/[0.04] hover:bg-white/[0.08] text-zinc-300 hover:text-white border border-white/10 active:scale-95",
                    onclick: move |_| {
                        let text = code_to_copy.clone();
                        spawn(async move {
                            let ok = copy_text(&text).await;
                            if ok {
                                copy_state.set(CopyState::Copied);
                            } else {
                                copy_state.set(CopyState::Failed);
                            }
                            let _ = document::eval(r#"
                                await new Promise(r => setTimeout(r, 1800));
                            "#).await;
                            copy_state.set(CopyState::Idle);
                        });
                    },
                    match copy_state() {
                        CopyState::Copied => rsx! {
                            span { class: "text-emerald-400 flex items-center gap-1",
                                Icon { kind: IconKind::Check, size: 13 }
                                "Copiato!"
                            }
                        },
                        CopyState::Failed => rsx! {
                            span { class: "text-rose-400 flex items-center gap-1",
                                Icon { kind: IconKind::X, size: 13 }
                                "Copia non riuscita"
                            }
                        },
                        CopyState::Idle => rsx! {
                            span { class: "flex items-center gap-1 text-zinc-400 hover:text-zinc-200",
                                Icon { kind: IconKind::Copy, size: 13 }
                                "Copia"
                            }
                        },
                    }
                }
            }
            // Code text with syntax highlighting
            pre {
                class: "p-4 font-mono text-xs overflow-x-auto leading-relaxed select-text",
                style: "white-space: pre; tab-size: 4;",
                code {
                    for (idx, (kind, text)) in tokens.into_iter().enumerate() {
                        match kind {
                            TokenKind::Keyword => rsx! { span { key: "{idx}", class: "text-purple-400 font-semibold", "{text}" } },
                            TokenKind::Type => rsx! { span { key: "{idx}", class: "text-cyan-300 font-medium", "{text}" } },
                            TokenKind::Prop => rsx! { span { key: "{idx}", class: "text-amber-300", "{text}" } },
                            TokenKind::StringLit => rsx! { span { key: "{idx}", class: "text-emerald-300", "{text}" } },
                            TokenKind::Number => rsx! { span { key: "{idx}", class: "text-orange-300", "{text}" } },
                            TokenKind::Comment => rsx! { span { key: "{idx}", class: "text-zinc-500 italic", "{text}" } },
                            TokenKind::Plain => rsx! { span { key: "{idx}", class: "text-zinc-300", "{text}" } },
                        }
                    }
                }
            }
        }
    }
}
