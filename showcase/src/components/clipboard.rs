use dioxus::prelude::*;

/// Compatibility helper using document eval
pub async fn copy_text(text: &str) -> bool {
    let script = format!(
        r#"
        if (navigator.clipboard && window.isSecureContext) {{
            try {{
                await navigator.clipboard.writeText({});
                return true;
            }} catch (e) {{}}
        }}
        try {{
            const ta = document.createElement("textarea");
            ta.value = {};
            ta.style.position = "fixed";
            ta.style.left = "-9999px";
            document.body.appendChild(ta);
            ta.focus();
            ta.select();
            const res = document.execCommand("copy");
            ta.remove();
            return res;
        }} catch (e) {{
            return false;
        }}
        "#,
        serde_json::to_string(text).unwrap_or_default(),
        serde_json::to_string(text).unwrap_or_default(),
    );
    let eval = document::eval(&script);
    match eval.await {
        Ok(v) => v.as_bool().unwrap_or(false),
        Err(_) => false,
    }
}
