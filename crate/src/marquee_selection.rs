use dioxus::prelude::*;

/// Bounding rectangle for active marquee selection.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
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
    /// Additional CSS classes.
    #[props(default)]
    class: String,
    /// Selectable children inside the container.
    children: Element,
) -> Element {
    let mut is_dragging = use_signal(|| false);
    let mut start_pos = use_signal(|| (0.0f64, 0.0f64));
    let mut current_rect = use_signal(|| MarqueeRect::default());

    let handle_mousedown = move |e: MouseEvent| {
        if !enabled {
            return;
        }
        // Only primary mouse button (button 0)
        let coords = e.coordinates();
        let (cx, cy) = (coords.element().x, coords.element().y);
        start_pos.set((cx, cy));
        is_dragging.set(true);
        current_rect.set(MarqueeRect {
            x: cx,
            y: cy,
            width: 0.0,
            height: 0.0,
        });
    };

    let handle_mousemove = move |e: MouseEvent| {
        if !*is_dragging.read() {
            return;
        }
        let coords = e.coordinates();
        let (curr_x, curr_y) = (coords.element().x, coords.element().y);
        let (sx, sy) = *start_pos.read();

        let x = sx.min(curr_x);
        let y = sy.min(curr_y);
        let width = (curr_x - sx).abs();
        let height = (curr_y - sy).abs();

        let rect = MarqueeRect { x, y, width, height };
        current_rect.set(rect);
    };

    let handle_mouseup = move |_| {
        if *is_dragging.read() {
            is_dragging.set(false);
            if let Some(h) = on_change {
                h.call(*current_rect.read());
            }
        }
    };

    let rect = *current_rect.read();
    let dragging = *is_dragging.read();
    let box_style = format!(
        "left: {}px; top: {}px; width: {}px; height: {}px;",
        rect.x, rect.y, rect.width, rect.height
    );

    rsx! {
        div {
            class: "sd-marquee-container {class}",
            onmousedown: handle_mousedown,
            onmousemove: handle_mousemove,
            onmouseup: handle_mouseup,
            {children}
            if dragging && (rect.width > 3.0 || rect.height > 3.0) {
                div {
                    class: "sd-marquee-box",
                    style: "{box_style}",
                }
            }
        }
    }
}
