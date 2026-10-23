use dioxus::prelude::*;

/// Reactively matches a CSS media query string (e.g. "(min-width: 768px)" or "(prefers-color-scheme: dark)")
/// and returns a reactive `Signal<bool>`.
pub fn use_media_query(query: impl Into<String>) -> Signal<bool> {
    let q_str = query.into();
    let mut matches = use_signal(|| false);
    let listener_key = crate::a11y::use_unique_id("sd-mq", None);

    let key_for_effect = listener_key.clone();
    let q_clone = q_str.clone();
    use_effect(move || {
        let js = format!(
            r#"
            (function() {{
                const key = "{key_for_effect}";
                window._sd_mq = window._sd_mq || {{}};
                if (window._sd_mq[key]) {{
                    window._sd_mq[key].mql.removeEventListener("change", window._sd_mq[key].handler);
                }}
                const mql = window.matchMedia("{q_clone}");
                dioxus.send(mql.matches);
                function onChange(e) {{
                    dioxus.send(e.matches);
                }}
                mql.addEventListener("change", onChange);
                window._sd_mq[key] = {{ mql, handler: onChange }};
            }})();
            "#
        );

        spawn(async move {
            let mut eval = document::eval(&js);
            while let Ok(msg) = eval.recv::<serde_json::Value>().await {
                if let Some(b) = msg.as_bool() {
                    matches.set(b);
                }
            }
        });
    });

    let key_for_drop = listener_key;
    use_drop(move || {
        let _ = document::eval(&format!(
            r#"
            if (window._sd_mq && window._sd_mq["{key_for_drop}"]) {{
                window._sd_mq["{key_for_drop}"].mql.removeEventListener("change", window._sd_mq["{key_for_drop}"].handler);
                delete window._sd_mq["{key_for_drop}"];
            }}
            "#
        ));
    });

    matches
}
