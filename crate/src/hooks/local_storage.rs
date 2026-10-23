use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

/// Persists and synchronizes a reactive `Signal<String>` with browser `localStorage`.
pub fn use_local_storage(
    key: impl Into<String>,
    default_value: impl Into<String>,
) -> Signal<String> {
    let key_str = key.into();
    let def_str = default_value.into();
    let mut signal = use_signal(|| def_str.clone());
    let mut is_loaded = use_signal(|| false);
    let last_saved = use_hook(|| Arc::new(Mutex::new(def_str)));

    // Initial load from localStorage
    let key_load = key_str.clone();
    use_hook(|| {
        spawn(async move {
            let eval = document::eval(&format!(
                r#"
                try {{
                    if (typeof window !== "undefined" && window.localStorage) {{
                        const val = window.localStorage.getItem("{key_load}");
                        return val;
                    }}
                    return null;
                }} catch(e) {{
                    return null;
                }}
                "#
            ));
            if let Ok(val) = eval.await {
                if let Some(s) = val.as_str() {
                    signal.set(s.to_string());
                }
            }
            is_loaded.set(true);
        });
    });

    // Auto-save on change
    let key_save = key_str;
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
        let k = key_save.clone();
        spawn(async move {
            let eval = document::eval(&format!(
                r#"
                try {{
                    if (typeof window !== "undefined" && window.localStorage) {{
                        window.localStorage.setItem("{k}", {json_val});
                    }}
                }} catch(e) {{}}
                "#
            ));
            let _ = eval.await;
        });
    });

    signal
}
