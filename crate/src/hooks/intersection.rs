use dioxus::prelude::*;

/// Observes whether an element with `element_id` is currently intersecting the viewport.
pub fn use_intersection_observer(
    element_id: impl Into<String>,
    threshold: Option<f64>,
) -> Signal<bool> {
    let el_id = element_id.into();
    let mut is_intersecting = use_signal(|| false);
    let th = threshold.unwrap_or(0.0);
    let listener_key = crate::a11y::use_unique_id("sd-io", None);

    let key_for_effect = listener_key.clone();
    let el_id_clone = el_id.clone();
    use_effect(move || {
        let js = format!(
            r#"
            (function() {{
                const key = "{key_for_effect}";
                window._sd_io = window._sd_io || {{}};
                if (window._sd_io[key]) {{
                    window._sd_io[key].disconnect();
                }}
                const el = document.getElementById("{el_id_clone}");
                if (!el) return;
                const obs = new IntersectionObserver((entries) => {{
                    for (const entry of entries) {{
                        dioxus.send(entry.isIntersecting);
                    }}
                }}, {{ threshold: {th} }});
                obs.observe(el);
                window._sd_io[key] = obs;
            }})();
            "#
        );

        spawn(async move {
            let mut eval = document::eval(&js);
            while let Ok(msg) = eval.recv::<serde_json::Value>().await {
                if let Some(b) = msg.as_bool() {
                    is_intersecting.set(b);
                }
            }
        });
    });

    let key_for_drop = listener_key;
    use_drop(move || {
        let _ = document::eval(&format!(
            r#"
            if (window._sd_io && window._sd_io["{key_for_drop}"]) {{
                window._sd_io["{key_for_drop}"].disconnect();
                delete window._sd_io["{key_for_drop}"];
            }}
            "#
        ));
    });

    is_intersecting
}
