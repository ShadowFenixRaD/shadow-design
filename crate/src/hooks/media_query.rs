use dioxus::prelude::*;

/// Reactively matches a CSS media query string (e.g. "(min-width: 768px)" or "(prefers-color-scheme: dark)")
/// and returns a reactive `Signal<bool>`.
pub fn use_media_query(query: &'static str) -> Signal<bool> {
    let mut matches = use_signal(|| false);

    use_effect(move || {
        let js = format!(
            r#"
            (function() {{
                const mql = window.matchMedia("{query}");
                dioxus.send(mql.matches);
                function onChange(e) {{
                    dioxus.send(e.matches);
                }}
                mql.addEventListener("change", onChange);
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

    matches
}
