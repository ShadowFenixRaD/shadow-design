use dioxus::prelude::*;

/// Detects user inactivity after `timeout_ms` of no mouse, keyboard, touch, or scroll events.
pub fn use_idle(timeout_ms: u64) -> Signal<bool> {
    let mut is_idle = use_signal(|| false);

    use_effect(move || {
        let js = format!(
            r#"
            (function() {{
                let timer = null;
                function reset() {{
                    dioxus.send(false);
                    if (timer) clearTimeout(timer);
                    timer = setTimeout(() => {{
                        dioxus.send(true);
                    }}, {timeout_ms});
                }}
                const events = ["mousemove", "mousedown", "keydown", "touchstart", "scroll", "wheel"];
                events.forEach(ev => window.addEventListener(ev, reset, {{ passive: true }}));
                reset();
            }})();
            "#
        );

        spawn(async move {
            let mut eval = document::eval(&js);
            while let Ok(msg) = eval.recv::<serde_json::Value>().await {
                if let Some(b) = msg.as_bool() {
                    is_idle.set(b);
                }
            }
        });
    });

    is_idle
}
