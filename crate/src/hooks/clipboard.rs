use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Clipboard controller returned by [`use_clipboard`].
#[derive(Clone, Copy)]
pub struct UseClipboard {
    /// Whether a copy operation has recently succeeded.
    pub copied: Signal<bool>,
    /// Whether the last copy operation failed.
    pub error: Signal<bool>,
    timeout_ms: u32,
    version: Signal<Arc<AtomicU64>>,
}

impl UseClipboard {
    /// Copies text to clipboard and flips `copied` to true for `timeout_ms`.
    pub fn copy(&self, text: impl Into<String>) {
        let mut copied = self.copied;
        let mut error = self.error;
        let timeout = self.timeout_ms;
        let text_str = text.into();
        let ver_arc = self.version.read().clone();
        let current_ver = ver_arc.fetch_add(1, Ordering::Relaxed) + 1;

        spawn(async move {
            let json_text = serde_json::to_string(&text_str).unwrap_or_default();
            let js = format!(
                r#"
                (async function() {{
                    let ok = false;
                    try {{
                        if (navigator.clipboard && navigator.clipboard.writeText) {{
                            await navigator.clipboard.writeText({json_text});
                            ok = true;
                        }} else {{
                            const ta = document.createElement("textarea");
                            ta.value = {json_text};
                            ta.style.position = "fixed";
                            ta.style.left = "-9999px";
                            document.body.appendChild(ta);
                            ta.select();
                            ok = document.execCommand("copy");
                            document.body.removeChild(ta);
                        }}
                    }} catch (e) {{
                        ok = false;
                    }}

                    if (ok) {{
                        dioxus.send({{ "status": "copied", "ver": {current_ver} }});
                        setTimeout(() => {{
                            dioxus.send({{ "status": "reset", "ver": {current_ver} }});
                        }}, {timeout});
                    }} else {{
                        dioxus.send({{ "status": "error", "ver": {current_ver} }});
                    }}
                }})();
                "#
            );

            let mut eval = document::eval(&js);
            while let Ok(msg) = eval.recv::<serde_json::Value>().await {
                if let Some(obj) = msg.as_object() {
                    let status = obj.get("status").and_then(|v| v.as_str()).unwrap_or("");
                    let ver = obj.get("ver").and_then(|v| v.as_u64()).unwrap_or(0);

                    match status {
                        "copied" => {
                            copied.set(true);
                            error.set(false);
                        }
                        "reset" => {
                            if ver == ver_arc.load(Ordering::Relaxed) {
                                copied.set(false);
                            }
                        }
                        "error" => {
                            error.set(true);
                            copied.set(false);
                        }
                        _ => {}
                    }
                }
            }
        });
    }

    /// Reset copied and error flags immediately.
    pub fn reset(&self) {
        let mut copied = self.copied;
        let mut error = self.error;
        copied.set(false);
        error.set(false);
    }
}

/// Reactive hook for system clipboard operations with automatic status feedback timeout.
pub fn use_clipboard(timeout_ms: Option<u32>) -> UseClipboard {
    let copied = use_signal(|| false);
    let error = use_signal(|| false);
    let version = use_signal(|| Arc::new(AtomicU64::new(0)));

    UseClipboard {
        copied,
        error,
        timeout_ms: timeout_ms.unwrap_or(2000),
        version,
    }
}
