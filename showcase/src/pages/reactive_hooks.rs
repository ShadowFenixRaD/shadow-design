use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::components::{Counter, FeatureBanner, PageHeader, PropRow, ShowcaseTabs};

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
    let search_query = use_signal(|| "".to_string());
    let debounced_query = use_debounced_signal(search_query, 300);

    // 3. Media Query test
    let is_wide = use_media_query("(min-width: 768px)");

    // 4. Clipboard test
    let clipboard = use_clipboard(Some(2000));

    // 5. Local Storage test
    let local_store = use_local_storage("shadow_hook_demo", "Valore memorizzato localmente");

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
        div { class: "sc-stack sc-stack--loose",
            PageHeader {
                title: "Reactive Hooks & Utilities".to_string(),
                description: "Collezione di primitive reattive ad alte prestazioni per scorciatoie, media query, debounce, appunti, storage locale e monitoraggio scroll/inattività.".to_string(),
            }

            FeatureBanner { feature_name: "hooks" }

            ShowcaseTabs {
                preview: rsx! {
                    div { class: "sc-card-grid",
                        Card {
                            title: "use_hotkey".to_string(),
                            subtitle: "Premi Ctrl+K oppure Escape sulla tastiera per testare l'intercettazione globale.".to_string(),
                            div { class: "sc-stack",
                                Badge { label: "Ctrl+K o Escape".to_string(), variant: BadgeVariant::Outline }
                                Counter { label: "Attivazioni".to_string(), value: hotkey_count().to_string() }
                                Counter { label: "Ultimo".to_string(), value: last_hotkey() }
                            }
                        }

                        Card {
                            title: "use_debounced_signal".to_string(),
                            subtitle: "Il valore debounced si aggiorna dopo 300ms dall'ultima digitazione.".to_string(),
                            div { class: "sc-stack",
                                TextInput {
                                    value: search_query,
                                    placeholder: "Scrivi velocemente qui...".to_string(),
                                }
                                Counter {
                                    label: "Debounced".to_string(),
                                    value: if debounced_query().is_empty() { "(in attesa)".to_string() } else { debounced_query() },
                                }
                            }
                        }

                        Card {
                            title: "use_media_query".to_string(),
                            subtitle: "Ridimensiona la larghezza della finestra del browser per vedere il matching live.".to_string(),
                            div { class: "sc-stack",
                                Badge { label: "(min-width: 768px)".to_string(), variant: BadgeVariant::Outline }
                                Counter {
                                    label: "Layout corrente".to_string(),
                                    value: if is_wide() { "Desktop (>= 768px)".to_string() } else { "Mobile (< 768px)".to_string() },
                                }
                            }
                        }

                        Card {
                            title: "use_clipboard".to_string(),
                            subtitle: "Copia istantaneamente una stringa negli appunti con feedback visivo temporizzato.".to_string(),
                            div { class: "sc-stack",
                                Badge { label: "2000ms reset".to_string(), variant: BadgeVariant::Outline }
                                Button {
                                    variant: if *clipboard.copied.read() { ButtonVariant::Success } else { ButtonVariant::Secondary },
                                    onclick: move |_| {
                                        clipboard.copy("Token Segreto: shadow-token-9988");
                                    },
                                    if *clipboard.copied.read() { "Copiato negli Appunti!" } else { "Copia Token di Prova" }
                                }
                            }
                        }

                        Card {
                            title: "use_idle".to_string(),
                            subtitle: "Non toccare mouse e tastiera per 5 secondi per verificare lo stato di inattività.".to_string(),
                            div { class: "sc-stack",
                                Badge { label: "Timeout: 5s".to_string(), variant: BadgeVariant::Outline }
                                Counter {
                                    label: "Stato utente".to_string(),
                                    value: if is_user_idle() { "Inattivo (Idle)".to_string() } else { "Attivo (Interacting)".to_string() },
                                }
                            }
                        }

                        Card {
                            title: "use_local_storage".to_string(),
                            subtitle: "Modifica il valore e ricarica la pagina: il dato rimane salvato in localStorage.".to_string(),
                            div { class: "sc-stack",
                                Badge { label: "Key: shadow_hook_demo".to_string(), variant: BadgeVariant::Outline }
                                TextInput { value: local_store }
                            }
                        }

                        Card {
                            title: "use_window_scroll".to_string(),
                            subtitle: "Traccia la coordinata Y della pagina in pixel durante lo scorrimento.".to_string(),
                            div { class: "sc-stack",
                                Counter {
                                    label: "Coordinate viewport".to_string(),
                                    value: format!("X: {}px | Y: {}px", scroll_x as i32, scroll_y as i32),
                                }
                                div { class: "sc-actions",
                                    Button {
                                        variant: ButtonVariant::Ghost,
                                        size: ButtonSize::Sm,
                                        onclick: move |_| scroll.scroll_to_top(),
                                        "Torna in Cima"
                                    }
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
