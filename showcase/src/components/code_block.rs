use dioxus::prelude::*;
use shadow_design::prelude::*;

#[component]
pub fn CodeBlock(
    code: String,
    #[props(default = "rsx")] language: &'static str,
    #[props(default)] class: String,
) -> Element {
    let mut copied = use_signal(|| false);
    let code_to_copy = code.clone();

    rsx! {
        div { class: "relative rounded-xl border border-white/[0.08] bg-[#060608] overflow-hidden shadow-inner {class}",
            // Header bar
            div { class: "flex items-center justify-between px-4 py-2 border-b border-white/[0.06] bg-[#0c0c12]/90 text-xs font-mono",
                span { class: "text-zinc-500 uppercase text-[10px] tracking-wider font-semibold", "{language}" }
                button {
                    r#type: "button",
                    class: "flex items-center gap-1.5 px-2.5 py-1 rounded text-[11px] font-medium transition-all cursor-pointer bg-white/[0.04] hover:bg-white/[0.08] text-zinc-300 hover:text-white border border-white/10 active:scale-95",
                    onclick: move |_| {
                        let _ = document::eval(&format!("navigator.clipboard.writeText({:?});", code_to_copy));
                        copied.set(true);
                    },
                    if copied() {
                        span { class: "text-emerald-400 flex items-center gap-1",
                            Icon { kind: IconKind::Check, size: 13 }
                            "Copiato!"
                        }
                    } else {
                        span { class: "flex items-center gap-1 text-zinc-400 hover:text-zinc-200",
                            Icon { kind: IconKind::Copy, size: 13 }
                            "Copia"
                        }
                    }
                }
            }
            // Code text
            pre { class: "p-4 font-mono text-xs text-purple-200/90 overflow-x-auto leading-relaxed select-text",
                code { "{code}" }
            }
        }
    }
}
