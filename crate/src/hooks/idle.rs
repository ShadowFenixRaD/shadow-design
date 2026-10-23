use dioxus::prelude::*;

/// Detects user inactivity after `timeout_ms` of no mouse, keyboard, touch, or scroll events.
#[must_use]
pub fn use_idle(timeout_ms: u64) -> Signal<bool> {
    let mut is_idle = use_signal(|| false);
    let listener_key = crate::a11y::use_unique_id("sd-idle", None);

    let key_for_effect = listener_key.clone();
    use_effect(move || {
        let js = format!(
            r#"
            (function() {{
                const key = "{key_for_effect}";
                window._sd_idle = window._sd_idle || {{}};
                let isIdle = false;
                let timer = null;

                function reset() {{
                    if (isIdle) {{
                        isIdle = false;
                        dioxus.send(false);
                    }}
                    if (timer) clearTimeout(timer);
                    timer = setTimeout(() => {{
                        isIdle = true;
                        dioxus.send(true);
                    }}, {timeout_ms});
                }}

                const events = ["mousemove", "mousedown", "keydown", "touchstart", "scroll", "wheel"];
                if (window._sd_idle[key]) {{
                    events.forEach(ev => window.removeEventListener(ev, window._sd_idle[key]));
                }}
                window._sd_idle[key] = reset;
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

    let key_for_drop = listener_key;
    use_drop(move || {
        let _ = document::eval(&format!(
            r#"
            if (window._sd_idle && window._sd_idle["{key_for_drop}"]) {{
                const events = ["mousemove", "mousedown", "keydown", "touchstart", "scroll", "wheel"];
                events.forEach(ev => window.removeEventListener(ev, window._sd_idle["{key_for_drop}"]));
                delete window._sd_idle["{key_for_drop}"];
            }}
            "#
        ));
    });

    is_idle
}
