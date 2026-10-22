use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// Bounding rectangle for active marquee selection.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MarqueeRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// Drag-to-select rectangular lasso container for selecting multiple items or cards.
#[component]
pub fn MarqueeSelection(
    /// Whether marquee lasso dragging is enabled. Defaults to true.
    #[props(default = true)]
    enabled: bool,
    /// Selection change callback receiving coordinates when dragging ends or updates.
    #[props(default)]
    on_change: Option<EventHandler<MarqueeRect>>,
    /// Callback receiving indices of children marked with `data-marquee-item` that intersect with the marquee.
    #[props(default)]
    on_select_indices: Option<EventHandler<Vec<usize>>>,
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Selectable children inside the container.
    children: Element,
) -> Element {
    let container_id = use_hook(|| {
        static ID: AtomicU64 = AtomicU64::new(1);
        format!("sd-marquee-{}", ID.fetch_add(1, Ordering::Relaxed))
    });

    let box_id = format!("{container_id}-box");

    // Interactive pointer capture and collision detection engine
    use_effect(use_reactive((&enabled, &container_id, &box_id), move |(enabled, cid, bid)| {
        if !enabled {
            return;
        }

        let js = format!(
            r#"
            (function() {{
                const container = document.getElementById("{cid}");
                const box = document.getElementById("{bid}");
                if (!container || !box) return;

                let isDragging = false;
                let startX = 0;
                let startY = 0;
                let currentRect = {{ x: 0, y: 0, width: 0, height: 0 }};

                function getChildIndices(rect) {{
                    const items = container.querySelectorAll("[data-marquee-item]");
                    const indices = [];
                    const cRect = container.getBoundingClientRect();
                    const rLeft = cRect.left + rect.x;
                    const rTop = cRect.top + rect.y;
                    const rRight = rLeft + rect.width;
                    const rBottom = rTop + rect.height;

                    items.forEach((item) => {{
                        const raw = item.getAttribute("data-marquee-item");
                        const idx = parseInt(raw, 10);
                        const b = item.getBoundingClientRect();
                        const overlap = !(b.right < rLeft || b.left > rRight || b.bottom < rTop || b.top > rBottom);
                        if (overlap && !isNaN(idx)) {{
                            indices.push(idx);
                            item.classList.add("sd-marquee-item--selected");
                        }} else {{
                            item.classList.remove("sd-marquee-item--selected");
                        }}
                    }});
                    return indices;
                }}

                function onPointerDown(e) {{
                    if (e.button !== 0) return;
                    const cRect = container.getBoundingClientRect();
                    startX = e.clientX - cRect.left;
                    startY = e.clientY - cRect.top;
                    isDragging = true;
                    try {{
                        container.setPointerCapture(e.pointerId);
                    }} catch (_) {{}}

                    currentRect = {{ x: startX, y: startY, width: 0, height: 0 }};
                    box.style.left = startX + "px";
                    box.style.top = startY + "px";
                    box.style.width = "0px";
                    box.style.height = "0px";
                    box.style.display = "none";
                }}

                function onPointerMove(e) {{
                    if (!isDragging) return;
                    const cRect = container.getBoundingClientRect();
                    const curX = e.clientX - cRect.left;
                    const curY = e.clientY - cRect.top;

                    const x = Math.min(startX, curX);
                    const y = Math.min(startY, curY);
                    const width = Math.abs(curX - startX);
                    const height = Math.abs(curY - startY);

                    currentRect = {{ x: x, y: y, width: width, height: height }};

                    if (width > 3 || height > 3) {{
                        box.style.display = "block";
                        box.style.left = x + "px";
                        box.style.top = y + "px";
                        box.style.width = width + "px";
                        box.style.height = height + "px";

                        getChildIndices(currentRect);
                    }}
                }}

                function onPointerUp(e) {{
                    if (!isDragging) return;
                    isDragging = false;
                    try {{
                        container.releasePointerCapture(e.pointerId);
                    }} catch (_) {{}}

                    const hadMoved = (currentRect.width > 3 || currentRect.height > 3);
                    box.style.display = "none";

                    if (hadMoved) {{
                        const indices = getChildIndices(currentRect);
                        dioxus.send({{
                            x: currentRect.x,
                            y: currentRect.y,
                            width: currentRect.width,
                            height: currentRect.height,
                            indices: indices
                        }});
                    }}
                }}

                container.addEventListener("pointerdown", onPointerDown);
                container.addEventListener("pointermove", onPointerMove);
                container.addEventListener("pointerup", onPointerUp);
                container.addEventListener("pointercancel", onPointerUp);
            }})();
            "#
        );

        spawn(async move {
            let mut eval = document::eval(&js);
            while let Ok(msg) = eval.recv::<serde_json::Value>().await {
                if let Some(obj) = msg.as_object() {
                    let x = obj.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let y = obj.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let width = obj.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let height = obj.get("height").and_then(|v| v.as_f64()).unwrap_or(0.0);

                    let rect = MarqueeRect { x, y, width, height };

                    if let Some(h) = on_change {
                        h.call(rect);
                    }

                    if let Some(h) = on_select_indices {
                        if let Some(arr) = obj.get("indices").and_then(|v| v.as_array()) {
                            let indices: Vec<usize> = arr
                                .iter()
                                .filter_map(|v| v.as_u64().map(|n| n as usize))
                                .collect();
                            h.call(indices);
                        }
                    }
                }
            }
        });
    }));

    rsx! {
        div {
            id: "{container_id}",
            class: "sd-marquee-container {class}",
            {children}
            div {
                id: "{box_id}",
                class: "sd-marquee-box",
                style: "display: none;",
            }
        }
    }
}
