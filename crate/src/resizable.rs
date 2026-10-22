use crate::icon::{Icon, IconKind};
#[cfg(feature = "theme")]
use crate::theme::use_theme;
use dioxus::prelude::*;

/// Direzione di divisione e ridimensionamento dei pannelli.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResizableDirection {
    /// Disposizione orizzontale (pannelli affiancati da sinistra a destra).
    #[default]
    Horizontal,
    /// Disposizione verticale (pannelli incolonnati dall'alto in basso).
    Vertical,
}

/// Contenitore per pannelli ridimensionabili con maniglie di separazione trascinabili.
#[component]
pub fn ResizablePanelGroup(
    /// Direzione di divisione dei pannelli. Defaults to `Horizontal`.
    #[props(default)]
    direction: ResizableDirection,
    /// Elementi figli (pannelli [`ResizablePanel`] e maniglie [`ResizableHandle`]).
    children: Element,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    let dir_class = match direction {
        ResizableDirection::Horizontal => "sd-resizable-group--horizontal",
        ResizableDirection::Vertical => "sd-resizable-group--vertical",
    };

    let group_id = use_hook(|| {
        static ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        format!("sd-res-group-{}", ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    });

    use_effect(use_reactive((&group_id, &direction), move |(gid, dir)| {
        let is_horiz = dir == ResizableDirection::Horizontal;
        let js = format!(
            r#"
            (function() {{
                let attempts = 0;
                function tryInit() {{
                    const group = document.getElementById("{gid}");
                    if (!group) {{
                        if (attempts++ < 30) setTimeout(tryInit, 30);
                        return;
                    }}

                    group.onpointerdown = function(e) {{
                        const handle = e.target.closest("[data-resizable-handle]");
                        if (!handle || handle.classList.contains("sd-resizable-handle--disabled")) return;

                        e.preventDefault();
                        handle.classList.add("sd-resizable-handle--active");
                        document.body.style.userSelect = "none";
                        document.body.style.cursor = {is_horiz} ? "col-resize" : "row-resize";

                        const prevPanel = handle.previousElementSibling;
                        const nextPanel = handle.nextElementSibling;
                        if (!prevPanel) return;

                        const minPct = parseFloat(prevPanel.getAttribute("data-min-size") || "10");
                        const maxPct = parseFloat(prevPanel.getAttribute("data-max-size") || "90");

                        function onMove(ev) {{
                            const rect = group.getBoundingClientRect();
                            let pct = 50;
                            if ({is_horiz}) {{
                                if (rect.width <= 0) return;
                                pct = ((ev.clientX - rect.left) / rect.width) * 100;
                            }} else {{
                                if (rect.height <= 0) return;
                                pct = ((ev.clientY - rect.top) / rect.height) * 100;
                            }}

                            if (pct < minPct) pct = minPct;
                            if (pct > maxPct) pct = maxPct;

                            prevPanel.style.flex = "0 0 " + pct.toFixed(2) + "%";
                            if (nextPanel) {{
                                nextPanel.style.flex = "1 1 0px";
                            }}
                        }}

                        function onUp() {{
                            handle.classList.remove("sd-resizable-handle--active");
                            document.body.style.userSelect = "";
                            document.body.style.cursor = "";
                            window.removeEventListener("pointermove", onMove);
                            window.removeEventListener("pointerup", onUp);
                            window.removeEventListener("pointercancel", onUp);
                        }}

                        window.addEventListener("pointermove", onMove);
                        window.addEventListener("pointerup", onUp);
                        window.addEventListener("pointercancel", onUp);
                    }};
                }}
                tryInit();
            }})();
            "#
        );
        let _ = document::eval(&js);
    }));

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    rsx! {
        div {
            id: "{group_id}",
            class: "sd-resizable-group {dir_class} {class}",
            {children}
        }
    }
}

/// Singolo pannello all'interno di un [`ResizablePanelGroup`] con dimensione percentuale e vincoli min/max.
#[component]
pub fn ResizablePanel(
    /// Dimensione percentuale iniziale o corrente (0.0..=100.0). Defaults to 50.0.
    #[props(default = 50.0)]
    default_size: f64,
    /// Dimensione percentuale minima consentita. Defaults to 10.0.
    #[props(default = 10.0)]
    min_size: f64,
    /// Dimensione percentuale massima consentita. Defaults to 90.0.
    #[props(default = 90.0)]
    max_size: f64,
    /// Contenuto interno del pannello.
    children: Element,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    let size_clamped = default_size.clamp(min_size, max_size);
    let flex_basis = format!("{size_clamped:.2}%");

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    rsx! {
        div {
            class: "sd-resizable-panel {class}",
            "data-min-size": "{min_size}",
            "data-max-size": "{max_size}",
            style: "flex: 0 0 {flex_basis}; min-width: 0; min-height: 0; overflow: auto;",
            {children}
        }
    }
}

/// Maniglia divisoria interattiva trascinabile posizionata tra due [`ResizablePanel`].
#[component]
pub fn ResizableHandle(
    /// Se mostrare l'icona centrale con indicatore di presa tattile. Defaults to true.
    #[props(default = true)]
    with_handle: bool,
    /// Se disabilitare il ridimensionamento della maniglia. Defaults to false.
    #[props(default = false)]
    disabled: bool,
    /// Callback invocato durante il trascinamento con delta pixel o percentuale.
    #[props(default)]
    on_drag: Option<EventHandler<f64>>,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    let disabled_class = if disabled { "sd-resizable-handle--disabled" } else { "" };

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    rsx! {
        div {
            class: "sd-resizable-handle {disabled_class} {class}",
            "data-resizable-handle": "true",
            if with_handle {
                div { class: "sd-resizable-handle-grip",
                    Icon { kind: IconKind::GripVertical, size: 14 }
                }
            }
        }
    }
}
