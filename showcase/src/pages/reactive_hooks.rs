use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{FeatureBanner, PropRow, ShowcaseTabs};

#[component]
pub fn ReactiveHooksPage() -> Element {
    // 1. Hotkey test
    let mut hotkey_count = use_signal(|| 0u32);
    let mut last_hotkey = use_signal(|| "Nessuno".to_string());
    use_hotkey("ctrl+k", move || {
        hotkey_count += 1;
        last_hotkey.set("Ctrl+K".to_string());
    });
    use_hotkey("escape", move || {
        hotkey_count += 1;
        last_hotkey.set("Escape".to_string());
    });

    // 2. Debounce test
    let mut search_query = use_signal(|| "".to_string());
    let debounced_query = use_debounced_signal(search_query, 300);

    // 3. Media Query test
    let is_wide = use_media_query("(min-width: 768px)");

    // 4. Clipboard test
    let clipboard = use_clipboard(Some(2000));

    // 5. Local Storage test
    let mut local_store = use_local_storage("shadow_hook_demo", "Valore memorizzato localmente");

    // 6. Idle test
    let is_user_idle = use_idle(5000);

    // 7. Scroll test
    let scroll = use_window_scroll();
    let (scroll_x, scroll_y) = *scroll.position.read();

    let props = vec![
        PropRow {
            name: "use_hotkey",
            r#type: "fn(shortcut: &'static str, on_trigger: FnMut())",
            default: "-",
            description: "Registra listener globale di combinazioni da tastiera (es. 'Ctrl+K', 'Escape', 'Alt+S')",
        },
        PropRow {
            name: "use_debounced_signal",
            r#type: "fn(source: Signal<T>, delay_ms: u64) -> Signal<T>",
            default: "-",
            description: "Crea una replica debounced reattiva che si aggiorna solo dopo un periodo di inattività",
        },
        PropRow {
            name: "use_media_query",
            r#type: "fn(query: &'static str) -> Signal<bool>",
            default: "-",
            description: "Valuta reattivamente una CSS Media Query sincronizzandosi con window.matchMedia",
        },
        PropRow {
            name: "use_clipboard",
            r#type: "fn(timeout_ms: Option<u32>) -> UseClipboard",
            default: "timeout: 2000ms",
            description: "Copia negli appunti di sistema con tracciamento di stato reattivo e reset temporizzato",
        },
        PropRow {
            name: "use_local_storage",
            r#type: "fn(key: &'static str, default: String) -> Signal<String>",
            default: "-",
            description: "Sincronizza bidirezionalmente uno stato reattivo con localStorage del browser",
        },
        PropRow {
            name: "use_idle",
            r#type: "fn(timeout_ms: u64) -> Signal<bool>",
            default: "-",
            description: "Rileva inattività dell'utente dopo timeout_ms di assenza di interazioni mouse/tastiera",
        },
        PropRow {
            name: "use_window_scroll",
            r#type: "fn() -> UseWindowScroll",
            default: "-",
            description: "Monitora le coordinate di scroll della finestra (x, y) e fornisce scroll_to fluido",
        },
    ];

    let code_snippet = r#"// Scorciatoia da tastiera globale
use_hotkey("ctrl+k", move || {
    println!("Apro Command Palette!");
});

// Ricerca debounced
let search = use_signal(|| "".to_string());
let debounced = use_debounced_signal(search, 300);

// Appunti
let clipboard = use_clipboard(Some(2000));
clipboard.copy("Testo copiato!");

// Media Query reattiva
let is_desktop = use_media_query("(min-width: 1024px)");"#
        .to_string();

    rsx! {
        div { class: "max-w-4xl space-y-8",
            div {
                Heading { order: HeadingOrder::H2, "Reactive Hooks & Utilities" }
                Text { dimmed: true, size: TextSize::Sm, "Collezione di primitive reattive ad alte prestazioni per scorciatoie, media query, debounce, appunti, storage locale e monitoraggio scroll/inattività." }
            }

            FeatureBanner { feature_name: "hooks" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "p-8 rounded-xl bg-[#060608] border border-white/[0.08] flex flex-col gap-8 shadow-inner",
                        // Griglia interattiva dei moduli hook
                        div { class: "grid grid-cols-1 md:grid-cols-2 gap-4",
                            // 1. Hotkey Card
                            div { class: "p-5 rounded-xl border border-white/[0.06] bg-zinc-950/40 space-y-3",
                                div { class: "flex items-center justify-between",
                                    span { class: "text-xs font-mono text-purple-300 font-semibold", "use_hotkey" }
                                    span { class: "text-[10px] font-mono px-2 py-0.5 rounded bg-purple-950/80 text-purple-200 border border-purple-500/30", "Ctrl+K o Escape" }
                                }
                                p { class: "text-xs text-zinc-400", "Premi Ctrl+K oppure Escape sulla tastiera per testare l'intercettazione globale." }
                                div { class: "flex items-center justify-between text-xs font-mono p-2.5 rounded bg-white/[0.02] border border-white/[0.04]",
                                    span { class: "text-zinc-500", "Attivazioni: {hotkey_count()}" }
                                    span { class: "text-purple-300 font-bold", "Ultimo: {last_hotkey()}" }
                                }
                            }

                            // 2. Debounce Card
                            div { class: "p-5 rounded-xl border border-white/[0.06] bg-zinc-950/40 space-y-3",
                                div { class: "flex items-center justify-between",
                                    span { class: "text-xs font-mono text-cyan-300 font-semibold", "use_debounced_signal" }
                                    span { class: "text-[10px] font-mono px-2 py-0.5 rounded bg-cyan-950/80 text-cyan-200 border border-cyan-500/30", "300ms Delay" }
                                }
                                input {
                                    r#type: "text",
                                    value: "{search_query()}",
                                    placeholder: "Scrivi velocemente qui...",
                                    class: "w-full px-3 py-1.5 text-xs font-mono rounded-lg border border-white/[0.1] bg-zinc-900 text-zinc-200 focus:outline-none focus:ring-1 focus:ring-cyan-400",
                                    oninput: move |e| search_query.set(e.value()),
                                }
                                div { class: "flex items-center justify-between text-xs font-mono p-2.5 rounded bg-white/[0.02] border border-white/[0.04]",
                                    span { class: "text-zinc-500", "Debounced:" }
                                    span { class: "text-cyan-300 font-bold truncate max-w-[160px]", if debounced_query().is_empty() { "(in attesa)" } else { "{debounced_query()}" } }
                                }
                            }

                            // 3. Media Query Card
                            div { class: "p-5 rounded-xl border border-white/[0.06] bg-zinc-950/40 space-y-3",
                                div { class: "flex items-center justify-between",
                                    span { class: "text-xs font-mono text-emerald-300 font-semibold", "use_media_query" }
                                    span { class: "text-[10px] font-mono px-2 py-0.5 rounded bg-emerald-950/80 text-emerald-200 border border-emerald-500/30", "(min-width: 768px)" }
                                }
                                p { class: "text-xs text-zinc-400", "Ridimensiona la larghezza della finestra del browser per vedere il matching live." }
                                div { class: "flex items-center justify-between text-xs font-mono p-2.5 rounded bg-white/[0.02] border border-white/[0.04]",
                                    span { class: "text-zinc-500", "Layout Corrente:" }
                                    span {
                                        class: if is_wide() { "text-emerald-300 font-bold" } else { "text-amber-300 font-bold" },
                                        if is_wide() { "Desktop (>= 768px)" } else { "Mobile (< 768px)" }
                                    }
                                }
                            }

                            // 4. Clipboard Card
                            div { class: "p-5 rounded-xl border border-white/[0.06] bg-zinc-950/40 space-y-3",
                                div { class: "flex items-center justify-between",
                                    span { class: "text-xs font-mono text-rose-300 font-semibold", "use_clipboard" }
                                    span { class: "text-[10px] font-mono px-2 py-0.5 rounded bg-rose-950/80 text-rose-200 border border-rose-500/30", "2000ms Reset" }
                                }
                                p { class: "text-xs text-zinc-400", "Copia istantaneamente una stringa negli appunti con feedback visivo temporizzato." }
                                button {
                                    r#type: "button",
                                    class: format!("w-full py-2 text-xs font-mono rounded-lg border transition-all cursor-pointer font-semibold {}", if *clipboard.copied.read() { "bg-emerald-950/80 text-emerald-200 border-emerald-500/50 shadow-[0_0_10px_rgba(16,185,129,0.3)]" } else { "bg-white/[0.04] text-zinc-300 border-white/[0.08] hover:bg-white/[0.08]" }),
                                    onclick: move |_| {
                                        clipboard.copy("Token Segreto: shadow-token-9988");
                                    },
                                    if *clipboard.copied.read() { "✓ Copiato negli Appunti!" } else { "Copia Token di Prova" }
                                }
                            }

                            // 5. Idle Card
                            div { class: "p-5 rounded-xl border border-white/[0.06] bg-zinc-950/40 space-y-3",
                                div { class: "flex items-center justify-between",
                                    span { class: "text-xs font-mono text-amber-300 font-semibold", "use_idle" }
                                    span { class: "text-[10px] font-mono px-2 py-0.5 rounded bg-amber-950/80 text-amber-200 border border-amber-500/30", "Timeout: 5s" }
                                }
                                p { class: "text-xs text-zinc-400", "Non toccare mouse e tastiera per 5 secondi per verificare lo stato di inattività." }
                                div { class: "flex items-center justify-between text-xs font-mono p-2.5 rounded bg-white/[0.02] border border-white/[0.04]",
                                    span { class: "text-zinc-500", "Stato Utente:" }
                                    span {
                                        class: if is_user_idle() { "text-amber-400 font-bold animate-pulse" } else { "text-emerald-300 font-bold" },
                                        if is_user_idle() { "Inattivo (Idle)" } else { "Attivo (Interacting)" }
                                    }
                                }
                            }

                            // 6. Local Storage Card
                            div { class: "p-5 rounded-xl border border-white/[0.06] bg-zinc-950/40 space-y-3",
                                div { class: "flex items-center justify-between",
                                    span { class: "text-xs font-mono text-blue-300 font-semibold", "use_local_storage" }
                                    span { class: "text-[10px] font-mono px-2 py-0.5 rounded bg-blue-950/80 text-blue-200 border border-blue-500/30", "Key: shadow_hook_demo" }
                                }
                                p { class: "text-xs text-zinc-400", "Modifica il valore e ricarica la pagina: il dato rimane salvato in localStorage." }
                                input {
                                    r#type: "text",
                                    value: "{local_store()}",
                                    class: "w-full px-3 py-1.5 text-xs font-mono rounded-lg border border-white/[0.1] bg-zinc-900 text-zinc-200 focus:outline-none focus:ring-1 focus:ring-blue-400",
                                    oninput: move |e| local_store.set(e.value()),
                                }
                            }

                            // 7. Scroll Card
                            div { class: "p-5 rounded-xl border border-white/[0.06] bg-zinc-950/40 space-y-3",
                                div { class: "flex items-center justify-between",
                                    span { class: "text-xs font-mono text-purple-300 font-semibold", "use_window_scroll" }
                                    button {
                                        r#type: "button",
                                        class: "text-[10px] font-mono px-2 py-0.5 rounded bg-white/[0.04] text-zinc-300 border border-white/[0.08] hover:bg-white/[0.08] cursor-pointer",
                                        onclick: move |_| scroll.scroll_to_top(),
                                        "Torna in Cima"
                                    }
                                }
                                p { class: "text-xs text-zinc-400", "Traccia la coordinata Y della pagina in pixel durante lo scorrimento." }
                                div { class: "flex items-center justify-between text-xs font-mono p-2.5 rounded bg-white/[0.02] border border-white/[0.04]",
                                    span { class: "text-zinc-500", "Coordinate Viewport:" }
                                    span { class: "text-purple-300 font-bold", "X: {scroll_x as i32}px | Y: {scroll_y as i32}px" }
                                }
                            }
                        }
                    }
                },
                code: code_snippet,
                props: props,
            }
        }
    }
}
