use dioxus::prelude::*;

/// Copies text to clipboard using the asynchronous Clipboard API with a textarea fallback.
///
/// Returns true if copying succeeded, false otherwise.
pub async fn copy_text(text: &str) -> bool {
    let json_text = serde_json::to_string(text).unwrap_or_else(|_| format!("\"{}\"", text.replace('"', "\\\"")));
    let script = format!(
        r#"
        const text = {json_text};
        if (navigator.clipboard && window.isSecureContext) {{
            try {{
                await navigator.clipboard.writeText(text);
                return true;
            }} catch (err) {{
                // fallback below
            }}
        }}
        try {{
            const textArea = document.createElement("textarea");
            textArea.value = text;
            textArea.style.position = "fixed";
            textArea.style.left = "-999999px";
            textArea.style.top = "-999999px";
            document.body.appendChild(textArea);
            textArea.focus();
            textArea.select();
            const success = document.execCommand('copy');
            textArea.remove();
            return success;
        }} catch (err) {{
            return false;
        }}
        "#
    );

    let res = document::eval(&script).await;
    match res {
        Ok(val) => val.as_bool().unwrap_or(false),
        Err(_) => false,
    }
}
