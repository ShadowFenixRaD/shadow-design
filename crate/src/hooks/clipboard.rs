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
            let mut eval = document::eval(
                r#"
                const text = await dioxus.recv();
                let ok = false;
                try {
                    if (navigator.clipboard && navigator.clipboard.writeText) {
                        await navigator.clipboard.writeText(text);
                        ok = true;
                    } else {
                        const ta = document.createElement("textarea");
                        ta.value = text;
                        ta.style.position = "fixed";
                        ta.style.left = "-9999px";
                        document.body.appendChild(ta);
                        ta.select();
                        ok = document.execCommand("copy");
                        document.body.removeChild(ta);
                    }
                } catch (e) {
                    ok = false;
                }
                dioxus.send(ok);
                "#,
            );

            let _ = eval.send(serde_json::Value::String(text_str));
            let ok = eval.recv::<bool>().await.unwrap_or(false);

            if ok {
                copied.set(true);
                error.set(false);

                let ver_check = ver_arc.clone();
                crate::util::sleep_ms(timeout as u64).await;
                if current_ver == ver_check.load(Ordering::Relaxed) {
                    copied.set(false);
                }
            } else {
                error.set(true);
                copied.set(false);
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
#[must_use]
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
