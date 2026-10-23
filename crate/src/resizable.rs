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
    /// Callback invocato in tempo reale durante il ridimensionamento con la percentuale del primo pannello.
    #[props(default)]
    on_resize: Option<EventHandler<f64>>,
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

    let group_id = use_signal(|| {
        static ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        format!("sd-res-group-{}", ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    });

    let dir_is_horiz = direction == ResizableDirection::Horizontal;

    let mut dir_signal = use_signal(|| direction);
    if *dir_signal.read() != direction {
        dir_signal.set(direction);
    }

    use_effect(move || {
        let gid_str = group_id.read().clone();
        let current_dir = *dir_signal.read();
        let is_horiz_val = current_dir == ResizableDirection::Horizontal;

        let js = format!(
            r#"
            (function() {{
                let attempts = 0;
                function tryInit() {{
                    const group = document.getElementById("{gid_str}");
                    if (!group) {{
                        if (attempts++ < 30) setTimeout(tryInit, 40);
                        return;
                    }}

                    const isH = {is_horiz_val};
                    group.setAttribute("data-direction", isH ? "true" : "false");

                    // Reimposta le dimensioni dei pannelli al cambio di direzione
                    const panels = group.querySelectorAll(".sd-resizable-panel");
                    panels.forEach(function(p) {{
                        const def = p.getAttribute("data-default-size");
                        const pctStr = def ? parseFloat(def).toFixed(2) + "%" : "50.00%";
                        p.style.flex = "0 1 " + pctStr;
                        p.style.minWidth = "0px";
                        p.style.minHeight = "0px";
                        if (isH) {{
                            p.style.width = pctStr;
                            p.style.height = "100%";
                        }} else {{
                            p.style.width = "100%";
                            p.style.height = pctStr;
                        }}
                    }});

                    if (group._hasResizableInit) return;
                    group._hasResizableInit = true;

                    group.addEventListener("pointerdown", function(e) {{
                        const handle = e.target.closest("[data-resizable-handle]");
                        if (!handle || handle.classList.contains("sd-resizable-handle--disabled")) return;

                        const prevPanel = handle.previousElementSibling;
                        const nextPanel = handle.nextElementSibling;
                        if (!prevPanel || !nextPanel) return;

                        e.preventDefault();
                        e.stopPropagation();

                        if (handle.setPointerCapture) {{
                            try {{
                                handle.setPointerCapture(e.pointerId);
                            }} catch (_) {{}}
                        }}

                        handle.classList.add("sd-resizable-handle--active");
                        document.body.style.userSelect = "none";

                        const currentIsHoriz = group.classList.contains("sd-resizable-group--horizontal") || group.getAttribute("data-direction") === "true";
                        document.body.style.cursor = currentIsHoriz ? "col-resize" : "row-resize";

                        const prevMin = parseFloat(prevPanel.getAttribute("data-min-size") || "10");
                        const prevMax = parseFloat(prevPanel.getAttribute("data-max-size") || "90");
                        const nextMin = parseFloat(nextPanel.getAttribute("data-min-size") || "10");
                        const nextMax = parseFloat(nextPanel.getAttribute("data-max-size") || "90");

                        const groupRect = group.getBoundingClientRect();
                        const groupSize = currentIsHoriz ? groupRect.width : groupRect.height;
                        if (groupSize <= 0) return;

                        const prevRect = prevPanel.getBoundingClientRect();
                        const nextRect = nextPanel.getBoundingClientRect();
                        const pairSize = currentIsHoriz ? (prevRect.width + nextRect.width) : (prevRect.height + nextRect.height);
                        const pairPct = (pairSize / groupSize) * 100;
                        const prevOffsetPct = currentIsHoriz 
                            ? ((prevRect.left - groupRect.left) / groupSize) * 100 
                            : ((prevRect.top - groupRect.top) / groupSize) * 100;

                        const effectiveMin = Math.max(prevMin, pairPct - nextMax);
                        const effectiveMax = Math.min(prevMax, pairPct - nextMin);

                        function onMove(ev) {{
                            const currentPosPct = currentIsHoriz
                                ? ((ev.clientX - groupRect.left) / groupSize) * 100
                                : ((ev.clientY - groupRect.top) / groupSize) * 100;

                            let prevPct = currentPosPct - prevOffsetPct;
                            if (prevPct < effectiveMin) prevPct = effectiveMin;
                            if (prevPct > effectiveMax) prevPct = effectiveMax;

                            let nextPct = pairPct - prevPct;
                            if (nextPct < 0) nextPct = 0;

                            const prevPctStr = prevPct.toFixed(2) + "%";
                            const nextPctStr = nextPct.toFixed(2) + "%";

                            prevPanel.style.flex = "0 1 " + prevPctStr;
                            prevPanel.style.minWidth = "0px";
                            prevPanel.style.minHeight = "0px";
                            prevPanel.style.overflow = "auto";

                            nextPanel.style.flex = "0 1 " + nextPctStr;
                            nextPanel.style.minWidth = "0px";
                            nextPanel.style.minHeight = "0px";
                            nextPanel.style.overflow = "auto";

                            if (currentIsHoriz) {{
                                prevPanel.style.width = prevPctStr;
                                prevPanel.style.height = "100%";
                                nextPanel.style.width = nextPctStr;
                                nextPanel.style.height = "100%";
                            }} else {{
                                prevPanel.style.width = "100%";
                                prevPanel.style.height = prevPctStr;
                                nextPanel.style.width = "100%";
                                nextPanel.style.height = nextPctStr;
                            }}

                            const el1 = document.getElementById("sd-demo-panel-1-size");
                            if (el1) el1.textContent = "Size: " + prevPct.toFixed(1) + "% (Min 20% / Max 70%)";
                            const el2 = document.getElementById("sd-demo-panel-2-size");
                            if (el2) el2.textContent = "Size: " + nextPct.toFixed(1) + "%";

                            try {{
                                dioxus.send(prevPct);
                            }} catch (_) {{}}
                        }}

                        function onUp(ev) {{
                            if (handle.releasePointerCapture && ev && ev.pointerId) {{
                                try {{
                                    handle.releasePointerCapture(ev.pointerId);
                                }} catch (_) {{}}
                            }}
                            handle.classList.remove("sd-resizable-handle--active");
                            document.body.style.userSelect = "";
                            document.body.style.cursor = "";
                            window.removeEventListener("pointermove", onMove);
                            window.removeEventListener("pointerup", onUp);
                            window.removeEventListener("pointercancel", onUp);
                            handle.removeEventListener("lostpointercapture", onUp);
                        }}

                        window.addEventListener("pointermove", onMove);
                        window.addEventListener("pointerup", onUp);
                        window.addEventListener("pointercancel", onUp);
                        handle.addEventListener("lostpointercapture", onUp);
                    }});
                }}
                tryInit();
            }})();
            "#
        );

        let mut eval = document::eval(&js);
        spawn(async move {
            while let Ok(pct) = eval.recv::<f64>().await {
                if let Some(ref handler) = on_resize {
                    handler.call(pct);
                }
            }
        });
    });

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    rsx! {
        div {
            id: "{group_id()}",
            class: "sd-resizable-group {dir_class} {class}",
            "data-direction": "{dir_is_horiz}",
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
            "data-default-size": "{size_clamped:.2}",
            "data-min-size": "{min_size}",
            "data-max-size": "{max_size}",
            style: "flex: 0 1 {flex_basis}; min-width: 0; min-height: 0; overflow: auto;",
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
