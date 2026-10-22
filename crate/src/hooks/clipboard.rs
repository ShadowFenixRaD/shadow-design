use dioxus::prelude::*;

/// Clipboard controller returned by [`use_clipboard`].
#[derive(Clone, Copy)]
pub struct UseClipboard {
    /// Whether a copy operation has recently succeeded.
    pub copied: Signal<bool>,
    /// Whether the last copy operation failed.
    pub error: Signal<bool>,
    timeout_ms: u32,
}

impl UseClipboard {
    /// Copies text to clipboard and flips `copied` to true for `timeout_ms`.
    pub fn copy(&self, text: impl Into<String>) {
        let mut copied = self.copied;
        let mut error = self.error;
        let timeout = self.timeout_ms;
        let text_str = text.into();

        spawn(async move {
            let json_text = serde_json::to_string(&text_str).unwrap_or_default();
            let js = format!(
                r#"
                (async function() {{
                    try {{
                        if (navigator.clipboard && navigator.clipboard.writeText) {{
                            await navigator.clipboard.writeText({json_text});
                            return true;
                        }}
                        const ta = document.createElement("textarea");
                        ta.value = {json_text};
                        ta.style.position = "fixed";
                        ta.style.left = "-9999px";
                        document.body.appendChild(ta);
                        ta.select();
                        const ok = document.execCommand("copy");
                        document.body.removeChild(ta);
                        return ok;
                    }} catch (e) {{
                        return false;
                    }}
                }})()
                "#
            );

            let eval = document::eval(&js);
            if let Ok(res) = eval.await {
                if res.as_bool().unwrap_or(false) {
                    copied.set(true);
                    error.set(false);

                    #[cfg(target_arch = "wasm32")]
                    {
                        gloo_timers::future::TimeoutFuture::new(timeout).await;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        tokio::time::sleep(std::time::Duration::from_millis(timeout as u64)).await;
                    }
                    copied.set(false);
                } else {
                    error.set(true);
                }
            } else {
                error.set(true);
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

    UseClipboard {
        copied,
        error,
        timeout_ms: timeout_ms.unwrap_or(2000),
    }
}
