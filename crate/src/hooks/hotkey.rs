use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Registers a keyboard shortcut (e.g. "Ctrl+K", "Escape", "Shift+Enter", "Alt+S")
/// and invokes the provided callback when triggered.
pub fn use_hotkey(
    shortcut: &'static str,
    on_trigger: impl FnMut() + 'static,
) {
    let parts: Vec<&str> = shortcut.split('+').map(|s| s.trim()).collect();
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

    let cb = Rc::new(RefCell::new(on_trigger));

    use_effect(move || {
        let js = format!(
            r#"
            (function() {{
                function handler(e) {{
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
                }}
                window.addEventListener("keydown", handler);
            }})();
            "#
        );

        let cb_clone = Rc::clone(&cb);
        spawn(async move {
            let mut eval = document::eval(&js);
            while let Ok(_) = eval.recv::<serde_json::Value>().await {
                if let Ok(mut f) = cb_clone.try_borrow_mut() {
                    f();
                }
            }
        });
    });
}
