//! Story: Hooks (feature `hooks`). No playground and no recipe (REVAMP §6): one example per
//! hook, each with a live readout, since the readout is what a hook produces.
use dioxus::prelude::*;
use shadow_design::prelude::*;

use crate::story::schema::*;

#[component]
fn Hotkey() -> Element {
    let mut count = use_signal(|| 0u32);
    use_hotkey("ctrl+.", move || count += 1);
    use_hotkey("escape", move || count.set(0));
    rsx! {
        Stack { gap: SpacingToken::Sm,
            Group { gap: SpacingToken::Xs,
                Kbd { "Ctrl" }
                Kbd { "." }
                span { class: "sc-muted-text", "incrementa, " }
                Kbd { "Esc" }
                span { class: "sc-muted-text", "azzera" }
            }
            span { class: "sc-readout", "Scorciatoia premuta {count} volte" }
        }
    }
}
const HOTKEY_CODE: &str = r#"let mut count = use_signal(|| 0u32);
use_hotkey("ctrl+.", move || count += 1);
use_hotkey("escape", move || count.set(0));

Stack { gap: SpacingToken::Sm,
    Group { gap: SpacingToken::Xs,
        Kbd { "Ctrl" }
        Kbd { "." }
        span { "incrementa, " }
        Kbd { "Esc" }
        span { "azzera" }
    }
    span { "Scorciatoia premuta {count} volte" }
}"#;

#[component]
fn Debounce() -> Element {
    let query = use_signal(String::new);
    let debounced = use_debounced_signal(query, 400);
    let throttled = use_throttle(query, 400);
    rsx! {
        Stack { gap: SpacingToken::Sm,
            TextInput { value: query, placeholder: Some("Scrivi velocemente…".to_string()) }
            span { class: "sc-readout", "Debounced: {debounced}" }
            span { class: "sc-readout", "Throttled: {throttled}" }
        }
    }
}
const DEBOUNCE_CODE: &str = r#"let query = use_signal(String::new);
let debounced = use_debounced_signal(query, 400);
let throttled = use_throttle(query, 400);

Stack { gap: SpacingToken::Sm,
    TextInput { value: query, placeholder: Some("Scrivi velocemente…".to_string()) }
    span { "Debounced: {debounced}" }
    span { "Throttled: {throttled}" }
}"#;

#[component]
fn Clipboard() -> Element {
    let clipboard = use_clipboard(Some(2000));
    let copied = *clipboard.copied.read();
    rsx! {
        Group { gap: SpacingToken::Sm,
            Button {
                variant: if copied { ButtonVariant::Success } else { ButtonVariant::Secondary },
                left_icon: Some(rsx! {
                    Icon { kind: if copied { IconKind::Check } else { IconKind::Copy }, size: 15 }
                }),
                onclick: move |_| clipboard.copy("edge-07.eu.mesh"),
                if copied { "Copiato" } else { "Copia l'host" }
            }
            if *clipboard.error.read() {
                span { class: "sc-danger-text", "Appunti non disponibili" }
            }
        }
    }
}
const CLIPBOARD_CODE: &str = r#"let clipboard = use_clipboard(Some(2000));
let copied = *clipboard.copied.read();

Group { gap: SpacingToken::Sm,
    Button {
        variant: if copied { ButtonVariant::Success } else { ButtonVariant::Secondary },
        left_icon: Some(rsx! {
            Icon { kind: if copied { IconKind::Check } else { IconKind::Copy }, size: 15 }
        }),
        onclick: move |_| clipboard.copy("edge-07.eu.mesh"),
        if copied { "Copiato" } else { "Copia l'host" }
    }
    if *clipboard.error.read() {
        span { "Appunti non disponibili" }
    }
}"#;

#[component]
fn LocalStorage() -> Element {
    let note = use_local_storage("shadow-design-demo-note", "Controllare relay-fra domattina");
    rsx! {
        Stack { gap: SpacingToken::Sm,
            TextInput { value: note, label: Some("Promemoria".to_string()), help_text: Some("Ricarica la pagina: il testo resta.".to_string()) }
            span { class: "sc-readout", "localStorage[\"shadow-design-demo-note\"] = {note}" }
        }
    }
}
const LOCAL_STORAGE_CODE: &str = r#"let note = use_local_storage("shadow-design-demo-note", "Controllare relay-fra domattina");

Stack { gap: SpacingToken::Sm,
    TextInput { value: note, label: Some("Promemoria".to_string()), help_text: Some("Ricarica la pagina: il testo resta.".to_string()) }
    span { "localStorage[\"shadow-design-demo-note\"] = {note}" }
}"#;

#[component]
fn MediaQuery() -> Element {
    let wide = use_media_query("(min-width: 768px)");
    let reduced = use_media_query("(prefers-reduced-motion: reduce)");
    rsx! {
        Stack { gap: SpacingToken::Sm,
            Group { gap: SpacingToken::Sm,
                Badge { label: if wide() { "Desktop".to_string() } else { "Mobile".to_string() }, variant: BadgeVariant::Primary, pill: true }
                span { class: "sc-readout", "(min-width: 768px) = {wide}" }
            }
            Group { gap: SpacingToken::Sm,
                Badge { label: if reduced() { "Animazioni ridotte".to_string() } else { "Animazioni piene".to_string() }, pill: true }
                span { class: "sc-readout", "(prefers-reduced-motion: reduce) = {reduced}" }
            }
        }
    }
}
const MEDIA_QUERY_CODE: &str = r#"let wide = use_media_query("(min-width: 768px)");
let reduced = use_media_query("(prefers-reduced-motion: reduce)");

Stack { gap: SpacingToken::Sm,
    Group { gap: SpacingToken::Sm,
        Badge { label: if wide() { "Desktop".to_string() } else { "Mobile".to_string() }, variant: BadgeVariant::Primary, pill: true }
        span { "(min-width: 768px) = {wide}" }
    }
    Group { gap: SpacingToken::Sm,
        Badge { label: if reduced() { "Animazioni ridotte".to_string() } else { "Animazioni piene".to_string() }, pill: true }
        span { "(prefers-reduced-motion: reduce) = {reduced}" }
    }
}"#;

#[component]
fn Idle() -> Element {
    let idle = use_idle(5000);
    rsx! {
        Stack { gap: SpacingToken::Sm,
            Badge {
                label: if idle() { "Inattivo".to_string() } else { "Attivo".to_string() },
                variant: if idle() { BadgeVariant::Warning } else { BadgeVariant::Success },
                pill: true,
                dot: true,
            }
            span { class: "sc-muted-text", "Non toccare mouse e tastiera per 5 secondi." }
        }
    }
}
const IDLE_CODE: &str = r#"let idle = use_idle(5000);

Stack { gap: SpacingToken::Sm,
    Badge {
        label: if idle() { "Inattivo".to_string() } else { "Attivo".to_string() },
        variant: if idle() { BadgeVariant::Warning } else { BadgeVariant::Success },
        pill: true,
        dot: true,
    }
    span { "Non toccare mouse e tastiera per 5 secondi." }
}"#;

#[component]
fn WindowScroll() -> Element {
    let scroll = use_window_scroll();
    let (_, y) = *scroll.position.read();
    rsx! {
        Group { gap: SpacingToken::Md, justify: JustifyContent::SpaceBetween,
            span { class: "sc-readout", "scrollY = {y:.0} px" }
            Button {
                variant: ButtonVariant::Ghost,
                size: ButtonSize::Sm,
                disabled: y < 1.0,
                onclick: move |_| scroll.scroll_to_top(),
                "Torna in cima"
            }
        }
    }
}
const WINDOW_SCROLL_CODE: &str = r#"let scroll = use_window_scroll();
let (_, y) = *scroll.position.read();

Group { gap: SpacingToken::Md, justify: JustifyContent::SpaceBetween,
    span { "scrollY = {y:.0} px" }
    Button {
        variant: ButtonVariant::Ghost,
        size: ButtonSize::Sm,
        disabled: y < 1.0,
        onclick: move |_| scroll.scroll_to_top(),
        "Torna in cima"
    }
}"#;

#[component]
fn ClickOutside() -> Element {
    let mut open = use_signal(|| false);
    use_click_outside("hooks-filters-panel", move || open.set(false));
    rsx! {
        div { id: "hooks-filters-panel",
            Stack { gap: SpacingToken::Sm,
                Button {
                    variant: ButtonVariant::Secondary,
                    aria_pressed: Some(open()),
                    onclick: move |_| open.toggle(),
                    "Filtri"
                }
                if open() {
                    Card { title: Some("Filtri".to_string()), subtitle: Some("Clicca fuori dal riquadro per chiuderlo.".to_string()),
                        Group { gap: SpacingToken::Sm,
                            Badge { label: "Milano".to_string(), pill: true }
                            Badge { label: "Solo online".to_string(), pill: true }
                        }
                    }
                }
            }
        }
    }
}
const CLICK_OUTSIDE_CODE: &str = r#"let mut open = use_signal(|| false);
use_click_outside("hooks-filters-panel", move || open.set(false));

div { id: "hooks-filters-panel",
    Stack { gap: SpacingToken::Sm,
        Button {
            variant: ButtonVariant::Secondary,
            aria_pressed: Some(open()),
            onclick: move |_| open.toggle(),
            "Filtri"
        }
        if open() {
            Card { title: Some("Filtri".to_string()), subtitle: Some("Clicca fuori dal riquadro per chiuderlo.".to_string()),
                Group { gap: SpacingToken::Sm,
                    Badge { label: "Milano".to_string(), pill: true }
                    Badge { label: "Solo online".to_string(), pill: true }
                }
            }
        }
    }
}"#;

#[component]
fn Intersection() -> Element {
    let visible = use_intersection_observer("hooks-feed-end", Some(1.0));
    rsx! {
        Stack { gap: SpacingToken::Sm,
            span { class: "sc-readout", if visible() { "Fine elenco visibile: carica altri" } else { "Scorri l'elenco fino in fondo" } }
            ScrollArea { height: Some("8rem".to_string()), scroll_type: ScrollType::Always,
                List { spacing: ListSpacing::Sm,
                    for i in 1..=12 {
                        ListItem { "Evento {i} · edge-07" }
                    }
                }
                div { id: "hooks-feed-end",
                    Badge { label: "Fine".to_string(), pill: true }
                }
            }
        }
    }
}
const INTERSECTION_CODE: &str = r#"let visible = use_intersection_observer("hooks-feed-end", Some(1.0));

Stack { gap: SpacingToken::Sm,
    span { if visible() { "Fine elenco visibile: carica altri" } else { "Scorri l'elenco fino in fondo" } }
    ScrollArea { height: Some("8rem".to_string()), scroll_type: ScrollType::Always,
        List { spacing: ListSpacing::Sm,
            for i in 1..=12 {
                ListItem { "Evento {i} · edge-07" }
            }
        }
        div { id: "hooks-feed-end",
            Badge { label: "Fine".to_string(), pill: true }
        }
    }
}"#;

const HOOK_PROPS: &[PropDoc] = &[
    prop("use_hotkey(shortcut, on_trigger)", "fn(impl Into<String>, impl FnMut())", "-", "Scorciatoia globale (\"ctrl+k\", \"shift+enter\", \"escape\"); chiama preventDefault quando corrisponde."),
    prop("use_debounced_signal(source, delay_ms)", "fn(Signal<T>, u64) -> Signal<T>", "-", "Copia che si aggiorna solo dopo delay_ms senza modifiche."),
    prop("use_throttle(source, interval_ms)", "fn(Signal<T>, u64) -> Signal<T>", "-", "Copia aggiornata al massimo una volta per intervallo."),
    prop("use_clipboard(timeout_ms)", "fn(Option<u32>) -> UseClipboard", "2000", "Copia negli appunti con stato copied/error che si azzera da solo."),
    prop("use_local_storage(key, default)", "fn(impl Into<String>, impl Into<String>) -> Signal<String>", "-", "Signal letto da localStorage al montaggio e salvato a ogni modifica."),
    prop("use_media_query(query)", "fn(impl Into<String>) -> Signal<bool>", "-", "Esito di window.matchMedia, aggiornato al cambio."),
    prop("use_idle(timeout_ms)", "fn(u64) -> Signal<bool>", "-", "true dopo timeout_ms senza mouse, tastiera, touch o scroll."),
    prop("use_window_scroll()", "fn() -> UseWindowScroll", "-", "Posizione di scorrimento della finestra e comandi di scroll."),
    prop("use_click_outside(element_id, on_click_outside)", "fn(impl Into<String>, impl FnMut())", "-", "Chiamato al pointerdown fuori dall'elemento con quell'id."),
    prop("use_intersection_observer(element_id, threshold)", "fn(impl Into<String>, Option<f64>) -> Signal<bool>", "None → 0.0", "true quando l'elemento è visibile almeno per threshold."),
];
const CLIPBOARD_PROPS: &[PropDoc] = &[
    prop("copied", "Signal<bool>", "false", "true dopo una copia riuscita, per timeout_ms."),
    prop("error", "Signal<bool>", "false", "true se la copia è fallita."),
    prop("copy(text)", "fn(&self, impl Into<String>)", "-", "Copia con navigator.clipboard o con un ripiego."),
    prop("reset()", "fn(&self)", "-", "Azzera copied ed error subito."),
];
const SCROLL_PROPS: &[PropDoc] = &[
    prop("position", "Signal<(f64, f64)>", "(0.0, 0.0)", "scrollX e scrollY della finestra."),
    prop("scroll_to(x, y, smooth)", "fn(&self, f64, f64, bool)", "-", "Scorre a una posizione, con animazione se smooth."),
    prop("scroll_to_top()", "fn(&self)", "-", "Torna in cima con animazione."),
];

fn example(id: &'static str, title: &'static str, description: &'static str, code: &'static str, render: fn() -> Element) -> Example {
    Example { id, title, description, code, render, span: Span::Half }
}

pub fn story() -> Story {
    Story {
        slug: "reactive-hooks",
        name: "Hooks",
        category: Category::Utility,
        features: &["hooks"],
        icon: IconKind::Sparkles,
        tagline: "Hook reattivi per scorciatoie, debounce, appunti, storage, media query, inattività, scroll e visibilità.",
        keywords: &["hooks", "hook", "hotkey", "debounce", "throttle", "clipboard", "localstorage", "media query", "scroll"],
        status: Status::Stable,
        playground: None,
        examples: vec![
            example("hotkey", "use_hotkey", "Scorciatoie globali da tastiera, anche più di una per componente.", HOTKEY_CODE, || rsx! { Hotkey {} }),
            example("debounce", "use_debounced_signal e use_throttle", "Due copie ritardate dello stesso signal.", DEBOUNCE_CODE, || rsx! { Debounce {} }),
            example("clipboard", "use_clipboard", "Copia con uno stato che torna a riposo da solo.", CLIPBOARD_CODE, || rsx! { Clipboard {} }),
            example("local-storage", "use_local_storage", "Un signal che sopravvive al ricaricamento.", LOCAL_STORAGE_CODE, || rsx! { LocalStorage {} }),
            example("media-query", "use_media_query", "Larghezza e preferenze di sistema come signal.", MEDIA_QUERY_CODE, || rsx! { MediaQuery {} }),
            example("idle", "use_idle", "Diventa true dopo 5 secondi senza interazioni.", IDLE_CODE, || rsx! { Idle {} }),
            example("window-scroll", "use_window_scroll", "Posizione della pagina e ritorno in cima.", WINDOW_SCROLL_CODE, || rsx! { WindowScroll {} }),
            example("click-outside", "use_click_outside", "Chiude un pannello quando clicchi altrove.", CLICK_OUTSIDE_CODE, || rsx! { ClickOutside {} }),
            example("intersection", "use_intersection_observer", "Sa quando la fine di un elenco entra in vista.", INTERSECTION_CODE, || rsx! { Intersection {} }),
        ],
        props: vec![
            PropGroup { component: "Hook", rows: HOOK_PROPS },
            PropGroup { component: "UseClipboard", rows: CLIPBOARD_PROPS },
            PropGroup { component: "UseWindowScroll", rows: SCROLL_PROPS },
        ],
        recipe: None,
        a11y: &[
            "use_hotkey chiama preventDefault: non rubare scorciatoie del browser o delle tecnologie assistive, e mostra sempre la combinazione con Kbd.",
            "use_click_outside reagisce solo al puntatore: offri anche Esc o un pulsante per chiudere da tastiera.",
            "use_idle e use_media_query leggono le preferenze dell'utente (come prefers-reduced-motion): usale per rispettarle, non per aggirarle.",
            "Gli hook non disegnano nulla: l'accessibilità dipende dai componenti che li usano.",
        ],
        related: &["command-palette", "copy-button", "virtual-list", "keytip"],
    }
}
