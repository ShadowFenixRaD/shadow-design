use dioxus::prelude::*;

/// Window scroll controller returned by [`use_window_scroll`].
#[derive(Clone, Copy)]
pub struct UseWindowScroll {
    /// Current window scroll coordinates (x, y) in pixels.
    pub position: Signal<(f64, f64)>,
}

impl UseWindowScroll {
    /// Smoothly or instantly scrolls the window to the given (x, y) coordinates.
    pub fn scroll_to(&self, x: f64, y: f64, smooth: bool) {
        let behavior = if smooth { "smooth" } else { "auto" };
        spawn(async move {
            let eval = document::eval(&format!(
                "window.scrollTo({{ left: {x}, top: {y}, behavior: '{behavior}' }});"
            ));
            let _ = eval.await;
        });
    }

    /// Scrolls to top of the page.
    pub fn scroll_to_top(&self) {
        self.scroll_to(0.0, 0.0, true);
    }
}

/// Tracks window scroll coordinates `(x, y)` reactively and provides smooth programmatic scrolling.
pub fn use_window_scroll() -> UseWindowScroll {
    let mut position = use_signal(|| (0.0, 0.0));

    use_hook(|| {
        let js = r#"
        (function() {
            let ticking = false;
            let lastY = -1;
            let lastX = -1;

            function update() {
                const y = Math.round(window.scrollY || window.pageYOffset || 0);
                const x = Math.round(window.scrollX || window.pageXOffset || 0);
                if (Math.abs(y - lastY) >= 2 || Math.abs(x - lastX) >= 2) {
                    lastY = y;
                    lastX = x;
                    dioxus.send([x, y]);
                }
            }

            window.addEventListener("scroll", () => {
                if (!ticking) {
                    window.requestAnimationFrame(() => {
                        update();
                        ticking = false;
                    });
                    ticking = true;
                }
            }, { passive: true });

            update();
        })();
        "#;

        spawn(async move {
            let mut eval = document::eval(js);
            while let Ok(msg) = eval.recv::<serde_json::Value>().await {
                if let Some(arr) = msg.as_array() {
                    let x = arr.first().and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let y = arr.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0);
                    position.set((x, y));
                }
            }
        });
    });

    UseWindowScroll { position }
}
