use dioxus::prelude::*;

/// Observes whether an element with `element_id` is currently intersecting the viewport.
pub fn use_intersection_observer(
    element_id: &'static str,
    threshold: Option<f64>,
) -> Signal<bool> {
    let mut is_intersecting = use_signal(|| false);
    let th = threshold.unwrap_or(0.0);

    use_effect(move || {
        let js = format!(
            r#"
            (function() {{
                const el = document.getElementById("{element_id}");
                if (!el) return;
                const obs = new IntersectionObserver((entries) => {{
                    for (const entry of entries) {{
                        dioxus.send(entry.isIntersecting);
                    }}
                }}, {{ threshold: {th} }});
                obs.observe(el);
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

    is_intersecting
}
