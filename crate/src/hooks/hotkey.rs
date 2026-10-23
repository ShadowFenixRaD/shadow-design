use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Registers a keyboard shortcut (e.g. "Ctrl+K", "Escape", "Shift+Enter", "Alt+S")
/// and invokes the provided callback when triggered.
pub fn use_hotkey(
    shortcut: impl Into<String>,
    on_trigger: impl FnMut() + 'static,
) {
    let shortcut_str = shortcut.into();
    let cb = use_hook(|| Rc::new(RefCell::new(on_trigger)));
    let listener_key = crate::a11y::use_unique_id("sd-hotkey", None);

    let key_for_effect = listener_key.clone();
    let cb_clone = Rc::clone(&cb);

    use_effect(move || {
        let parts: Vec<&str> = shortcut_str.split('+').map(|s| s.trim()).collect();
        let mut ctrl = false;
        let mut shift = false;
        let mut alt = false;
        let mut meta = false;
        let mut key_name = String::new();

        for part in parts {
            match part.to_lowercase().as_str() {
                "ctrl" | "control" => ctrl = true,
                "shift" => shift = true,
                "alt" => alt = true,
                "cmd" | "meta" | "win" => meta = true,
                k => key_name = k.to_string(),
            }
        }

        let js = format!(
            r#"
            (function() {{
                const key = "{key_for_effect}";
                window._sd_hk = window._sd_hk || {{}};
                if (window._sd_hk[key]) {{
                    window.removeEventListener("keydown", window._sd_hk[key]);
                }}
                window._sd_hk[key] = function(e) {{
                    const matchCtrl = {ctrl} === (e.ctrlKey || false);
                    const matchShift = {shift} === (e.shiftKey || false);
                    const matchAlt = {alt} === (e.altKey || false);
                    const matchMeta = {meta} === (e.metaKey || false);
                    const keyLower = "{key_name}".toLowerCase();
                    const pressedKey = (e.key || "").toLowerCase();

                    if (matchCtrl && matchShift && matchAlt && matchMeta && (pressedKey === keyLower || keyLower === "")) {{
                        e.preventDefault();
                        dioxus.send(true);
                    }}
                }};
                window.addEventListener("keydown", window._sd_hk[key]);
            }})();
            "#
        );

        let cb_inner = Rc::clone(&cb_clone);
        spawn(async move {
            let mut eval = document::eval(&js);
            while eval.recv::<serde_json::Value>().await.is_ok() {
                if let Ok(mut f) = cb_inner.try_borrow_mut() {
                    f();
                }
            }
        });
    });

    let key_for_drop = listener_key;
    use_drop(move || {
        let _ = document::eval(&format!(
            r#"
            if (window._sd_hk && window._sd_hk["{key_for_drop}"]) {{
                window.removeEventListener("keydown", window._sd_hk["{key_for_drop}"]);
                delete window._sd_hk["{key_for_drop}"];
            }}
            "#
        ));
    });
}
