use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

/// Persists and synchronizes a reactive `Signal<String>` with browser `localStorage`.
pub fn use_local_storage(
    key: &'static str,
    default_value: impl Into<String>,
) -> Signal<String> {
    let def_str = default_value.into();
    let mut signal = use_signal(|| def_str.clone());
    let mut is_loaded = use_signal(|| false);
    let last_saved = use_hook(|| Arc::new(Mutex::new(def_str)));

    // Initial load from localStorage
    use_hook(|| {
        spawn(async move {
            let eval = document::eval(&format!(
                r#"
                try {{
                    return (typeof window !== "undefined" && window.localStorage)
                        ? (window.localStorage.getItem("{key}") || "")
                        : "";
                }} catch(e) {{
                    return "";
                }}
                "#
            ));
            if let Ok(val) = eval.await {
                if let Some(s) = val.as_str() {
                    if !s.is_empty() {
                        signal.set(s.to_string());
                    }
                }
            }
            is_loaded.set(true);
        });
    });

    // Auto-save on change
    let last_clone = Arc::clone(&last_saved);
    use_effect(move || {
        if !*is_loaded.read() {
            return;
        }
        let current = signal.read().clone();
        if let Ok(mut guard) = last_clone.lock() {
            if *guard == current {
                return;
            }
            *guard = current.clone();
        }

        let json_val = serde_json::to_string(&current).unwrap_or_default();
        spawn(async move {
            let eval = document::eval(&format!(
                r#"
                try {{
                    if (typeof window !== "undefined" && window.localStorage) {{
                        window.localStorage.setItem("{key}", {json_val});
                    }}
                }} catch(e) {{}}
                "#
            ));
            let _ = eval.await;
        });
    });

    signal
}
