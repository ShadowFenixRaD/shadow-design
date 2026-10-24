use dioxus::prelude::*;

use crate::icon::{Icon, IconKind};

/// Modalità di visualizzazione ed editing dell'editor.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum RichTextMode {
    #[default]
    Wysiwyg,
    Markdown,
    Split,
}

/// Comandi di formattazione supportati dalla toolbar dell'editor.
#[derive(Clone, PartialEq, Debug)]
pub enum RichTextCommand {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Highlight,
    ClearFormat,
    Heading(u8),
    Blockquote,
    BulletList,
    OrderedList,
    CodeBlock,
    InsertTable,
    InsertLink,
    InsertImage,
}

impl RichTextCommand {
    #[must_use]
    pub fn icon(&self) -> IconKind {
        match self {
            Self::Bold => IconKind::Sparkles,
            Self::Italic => IconKind::Sparkles,
            Self::Underline => IconKind::Sparkles,
            Self::Strikethrough => IconKind::X,
            Self::Highlight => IconKind::Sparkles,
            Self::ClearFormat => IconKind::XCircle,
            Self::Heading(_) => IconKind::Layers,
            Self::Blockquote => IconKind::MessageSquare,
            Self::BulletList => IconKind::Menu,
            Self::OrderedList => IconKind::Layers,
            Self::CodeBlock => IconKind::Code,
            Self::InsertTable => IconKind::Box,
            Self::InsertLink => IconKind::ExternalLink,
            Self::InsertImage => IconKind::Paperclip,
        }
    }

    #[must_use]
    pub fn label(&self) -> &'static str {
        match self {
            Self::Bold => "Bold (Ctrl+B)",
            Self::Italic => "Italic (Ctrl+I)",
            Self::Underline => "Underline (Ctrl+U)",
            Self::Strikethrough => "Strikethrough",
            Self::Highlight => "Highlight",
            Self::ClearFormat => "Clear formatting",
            Self::Heading(lvl) => match lvl {
                1 => "Heading 1",
                2 => "Heading 2",
                3 => "Heading 3",
                _ => "Heading",
            },
            Self::Blockquote => "Quote",
            Self::BulletList => "Bullet list",
            Self::OrderedList => "Numbered list",
            Self::CodeBlock => "Code block",
            Self::InsertTable => "Table",
            Self::InsertLink => "Insert link",
            Self::InsertImage => "Insert image",
        }
    }

    /// Applica il comando su un testo markdown selezionato o per inserimento sintassi.
    #[must_use]
    pub fn apply_to_text(&self, text: &str) -> String {
        match self {
            Self::Bold => {
                if text.starts_with("**") && text.ends_with("**") && text.len() >= 4 {
                    text[2..text.len() - 2].to_string()
                } else {
                    format!("**{text}**")
                }
            }
            Self::Italic => {
                if text.starts_with('*') && text.ends_with('*') && text.len() >= 2 {
                    text[1..text.len() - 1].to_string()
                } else {
                    format!("*{text}*")
                }
            }
            Self::Underline => format!("<u>{text}</u>"),
            Self::Strikethrough => format!("~~{text}~~"),
            Self::Highlight => format!("<mark>{text}</mark>"),
            Self::ClearFormat => text
                .replace("**", "")
                .replace('*', "")
                .replace("~~", "")
                .replace("<u>", "")
                .replace("</u>", "")
                .replace("<mark>", "")
                .replace("</mark>", ""),
            Self::Heading(lvl) => {
                let hashes = "#".repeat(*lvl as usize);
                format!("{hashes} {text}")
            }
            Self::Blockquote => format!("> {text}"),
            Self::BulletList => format!("- {text}"),
            Self::OrderedList => format!("1. {text}"),
            Self::CodeBlock => format!("```rust\n{text}\n```"),
            Self::InsertTable => {
                "| Heading 1 | Heading 2 |\n| --- | --- |\n| Cell 1 | Cell 2 |".to_string()
            }
            Self::InsertLink => format!("[{text}](https://)"),
            Self::InsertImage => format!("![{text}](https://)"),
        }
    }
}

/// Singolo pulsante di controllo per la toolbar del RichTextEditor.
#[component]
pub fn RichTextControl(
    command: RichTextCommand,
    #[props(default)] active: bool,
    #[props(default)] disabled: bool,
    onclick: EventHandler<RichTextCommand>,
) -> Element {
    let cmd = command.clone();
    let lbl = command.label();
    let icon_kind = command.icon();

    let active_cls = if active { "sd-rte__control--active" } else { "" };

    rsx! {
        button {
            r#type: "button",
            class: "sd-rte__control {active_cls}",
            title: "{lbl}",
            "aria-label": "{lbl}",
            "aria-pressed": "{active}",
            disabled: disabled,
            onclick: move |_| {
                if !disabled {
                    onclick.call(cmd.clone());
                }
            },
            Icon { kind: icon_kind, size: 15 }
        }
    }
}

/// Gruppo visivo di pulsanti di controllo per la toolbar.
#[component]
pub fn RichTextControlsGroup(children: Element) -> Element {
    rsx! {
        div { class: "sd-rte__group",
            {children}
        }
    }
}

/// Barra degli strumenti modulare per `RichTextEditor`.
#[component]
pub fn RichTextToolbar(
    #[props(default = true)] sticky: bool,
    children: Element,
) -> Element {
    let sticky_cls = if sticky { "sd-rte__toolbar--sticky" } else { "" };

    rsx! {
        div {
            class: "sd-rte__toolbar {sticky_cls}",
            {children}
        }
    }
}

/// Editor di testo ricco in stile TipTap: visuale e WYSIWYG per visualizzare e modificare direttamente il testo formattato, con supporto a Markdown e visualizzazione Split.
#[component]
pub fn RichTextEditor(
    mut value: Signal<String>,
    #[props(default)] placeholder: String,
    #[props(default = true)] editable: bool,
    #[props(default = true)] sticky_toolbar: bool,
    #[props(default)] mode: Option<RichTextMode>,
    #[props(default)] min_height: Option<String>,
    #[props(default)] class: String,
) -> Element {
    let locale = crate::locale::use_locale();
    let placeholder = if placeholder.is_empty() { locale.rich_text_placeholder.clone() } else { placeholder };
    let mut current_mode = use_signal(|| mode.unwrap_or(RichTextMode::Wysiwyg));
    let h_style = min_height.unwrap_or_else(|| "240px".to_string());
    let editor_id = crate::a11y::use_unique_id("rte", None);

    let rendered_html = markdown_to_html(&value());

    let eid = editor_id.clone();
    let apply_cmd = EventHandler::new(move |cmd: RichTextCommand| {
        let el_id = eid.clone();
        spawn(async move {
            let js_cmd = match &cmd {
                RichTextCommand::Bold => "document.execCommand('bold', false, null);",
                RichTextCommand::Italic => "document.execCommand('italic', false, null);",
                RichTextCommand::Underline => "document.execCommand('underline', false, null);",
                RichTextCommand::Strikethrough => "document.execCommand('strikeThrough', false, null);",
                RichTextCommand::Highlight => "document.execCommand('hiliteColor', false, getComputedStyle(el).getPropertyValue('--sd-accent-soft').trim() || 'transparent');",
                RichTextCommand::ClearFormat => "document.execCommand('removeFormat', false, null);",
                RichTextCommand::Heading(1) => "document.execCommand('formatBlock', false, '<h1>');",
                RichTextCommand::Heading(2) => "document.execCommand('formatBlock', false, '<h2>');",
                RichTextCommand::Heading(3) => "document.execCommand('formatBlock', false, '<h3>');",
                RichTextCommand::Heading(_) => "document.execCommand('formatBlock', false, '<p>');",
                RichTextCommand::Blockquote => "document.execCommand('formatBlock', false, '<blockquote>');",
                RichTextCommand::BulletList => "document.execCommand('insertUnorderedList', false, null);",
                RichTextCommand::OrderedList => "document.execCommand('insertOrderedList', false, null);",
                RichTextCommand::CodeBlock => "document.execCommand('formatBlock', false, '<pre>');",
                RichTextCommand::InsertLink => {
                    "const u = prompt('Link URL:'); if (u) document.execCommand('createLink', false, u);"
                }
                RichTextCommand::InsertImage => {
                    "const u = prompt('Image URL:'); if (u) document.execCommand('insertImage', false, u);"
                }
                RichTextCommand::InsertTable => {
                    "document.execCommand('insertHTML', false, '<table><tr><th>Column 1</th><th>Column 2</th></tr><tr><td>Data 1</td><td>Data 2</td></tr></table>');"
                }
            };

            let script = format!(
                r#"
                const el = document.getElementById("{el_id}");
                if (el) {{
                    el.focus();
                    {js_cmd}
                    const html = el.innerHTML || "";
                    return html;
                }}
                return "";
                "#
            );
            let eval = document::eval(&script);
            if let Ok(res) = eval.await {
                if let Some(h) = res.as_str() {
                    let md = html_to_markdown(h);
                    value.set(md);
                }
            }
        });
    });

    let panes_cls = if current_mode() == RichTextMode::Split { "sd-rte__panes--split" } else { "" };
    let mode_cls = |m: RichTextMode| if current_mode() == m { "sd-rte__mode sd-rte__mode--active" } else { "sd-rte__mode" };
    let btn_preview_cls = mode_cls(RichTextMode::Wysiwyg);
    let btn_md_cls = mode_cls(RichTextMode::Markdown);
    let btn_split_cls = mode_cls(RichTextMode::Split);

    let eid_sync = editor_id.clone();

    rsx! {
        div {
            class: "sd-rte {class}",
            style: "--sd-rte-min-height: {h_style};",
            // Toolbar master
            RichTextToolbar { sticky: sticky_toolbar,
                // Gruppo Formattazione
                RichTextControlsGroup {
                    RichTextControl { command: RichTextCommand::Bold, onclick: apply_cmd }
                    RichTextControl { command: RichTextCommand::Italic, onclick: apply_cmd }
                    RichTextControl { command: RichTextCommand::Underline, onclick: apply_cmd }
                    RichTextControl { command: RichTextCommand::Strikethrough, onclick: apply_cmd }
                    RichTextControl { command: RichTextCommand::Highlight, onclick: apply_cmd }
                    RichTextControl { command: RichTextCommand::ClearFormat, onclick: apply_cmd }
                }

                // Gruppo Struttura
                RichTextControlsGroup {
                    RichTextControl { command: RichTextCommand::Heading(1), onclick: apply_cmd }
                    RichTextControl { command: RichTextCommand::Heading(2), onclick: apply_cmd }
                    RichTextControl { command: RichTextCommand::Heading(3), onclick: apply_cmd }
                    RichTextControl { command: RichTextCommand::Blockquote, onclick: apply_cmd }
                    RichTextControl { command: RichTextCommand::BulletList, onclick: apply_cmd }
                    RichTextControl { command: RichTextCommand::OrderedList, onclick: apply_cmd }
                }

                // Gruppo Elementi e Codice
                RichTextControlsGroup {
                    RichTextControl { command: RichTextCommand::CodeBlock, onclick: apply_cmd }
                    RichTextControl { command: RichTextCommand::InsertTable, onclick: apply_cmd }
                    RichTextControl { command: RichTextCommand::InsertLink, onclick: apply_cmd }
                    RichTextControl { command: RichTextCommand::InsertImage, onclick: apply_cmd }
                }

                // Switcher modalità (WYSIWYG TipTap / Markdown / Split)
                div { class: "sd-rte__modes",
                    button {
                        r#type: "button",
                        class: "{btn_preview_cls}",
                        "aria-pressed": "{current_mode() == RichTextMode::Wysiwyg}",
                        onclick: move |_| current_mode.set(RichTextMode::Wysiwyg),
                        "Visual"
                    }
                    button {
                        r#type: "button",
                        class: "{btn_md_cls}",
                        "aria-pressed": "{current_mode() == RichTextMode::Markdown}",
                        onclick: move |_| current_mode.set(RichTextMode::Markdown),
                        "Markdown"
                    }
                    button {
                        r#type: "button",
                        class: "{btn_split_cls}",
                        "aria-pressed": "{current_mode() == RichTextMode::Split}",
                        onclick: move |_| current_mode.set(RichTextMode::Split),
                        "Split"
                    }
                }
            }

            // Area Contenuto
            div { class: "sd-rte__panes {panes_cls}",
                // Vista Editor Markdown
                if current_mode() == RichTextMode::Markdown || current_mode() == RichTextMode::Split {
                    textarea {
                        class: "sd-rte__source",
                        placeholder: "{placeholder}",
                        value: "{value}",
                        readonly: !editable,
                        oninput: move |evt| value.set(evt.value()),
                    }
                }

                // Vista Visuale WYSIWYG interattiva in stile TipTap
                if current_mode() == RichTextMode::Wysiwyg || current_mode() == RichTextMode::Split {
                    div {
                        id: "{eid_sync}",
                        class: "sd-rte__editable",
                        contenteditable: if editable { "true" } else { "false" },
                        dangerous_inner_html: "{rendered_html}",
                        oninput: {
                            let el_id = eid_sync.clone();
                            move |_| {
                                let el_id = el_id.clone();
                                spawn(async move {
                                    let script = format!(
                                        r#"
                                        const el = document.getElementById("{el_id}");
                                        return el ? el.innerHTML || "" : "";
                                        "#
                                    );
                                    let eval = document::eval(&script);
                                    if let Ok(res) = eval.await {
                                        if let Some(h) = res.as_str() {
                                            let md = html_to_markdown(h);
                                            value.set(md);
                                        }
                                    }
                                });
                            }
                        },
                    }
                }
            }
        }
    }
}

/// Converte Markdown in HTML semantico formattato ad alte prestazioni.
#[must_use]
pub fn markdown_to_html(md: &str) -> String {
    let mut html = String::new();
    let mut in_code_block = false;
    let mut in_ul = false;
    let mut in_ol = false;

    for line in md.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            if in_code_block {
                html.push_str("</code></pre>");
                in_code_block = false;
            } else {
                if in_ul { html.push_str("</ul>"); in_ul = false; }
                if in_ol { html.push_str("</ol>"); in_ol = false; }
                html.push_str("<pre><code>");
                in_code_block = true;
            }
            continue;
        }

        if in_code_block {
            html.push_str(&html_escape(line));
            html.push('\n');
            continue;
        }

        if let Some(h1) = trimmed.strip_prefix("# ") {
            if in_ul { html.push_str("</ul>"); in_ul = false; }
            if in_ol { html.push_str("</ol>"); in_ol = false; }
            html.push_str(&format!("<h1>{}</h1>", format_inline(h1)));
        } else if let Some(h2) = trimmed.strip_prefix("## ") {
            if in_ul { html.push_str("</ul>"); in_ul = false; }
            if in_ol { html.push_str("</ol>"); in_ol = false; }
            html.push_str(&format!("<h2>{}</h2>", format_inline(h2)));
        } else if let Some(h3) = trimmed.strip_prefix("### ") {
            if in_ul { html.push_str("</ul>"); in_ul = false; }
            if in_ol { html.push_str("</ol>"); in_ol = false; }
            html.push_str(&format!("<h3>{}</h3>", format_inline(h3)));
        } else if let Some(bq) = trimmed.strip_prefix("> ") {
            if in_ul { html.push_str("</ul>"); in_ul = false; }
            if in_ol { html.push_str("</ol>"); in_ol = false; }
            html.push_str(&format!("<blockquote>{}</blockquote>", format_inline(bq)));
        } else if let Some(li) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")) {
            if in_ol { html.push_str("</ol>"); in_ol = false; }
            if !in_ul {
                html.push_str("<ul>");
                in_ul = true;
            }
            html.push_str(&format!("<li>{}</li>", format_inline(li)));
        } else if trimmed.chars().next().is_some_and(|c| c.is_ascii_digit()) && trimmed.contains(". ") {
            if in_ul { html.push_str("</ul>"); in_ul = false; }
            if !in_ol {
                html.push_str("<ol>");
                in_ol = true;
            }
            let text = trimmed.split_once(". ").map(|(_, t)| t).unwrap_or(trimmed);
            html.push_str(&format!("<li>{}</li>", format_inline(text)));
        } else if trimmed.is_empty() {
            if in_ul { html.push_str("</ul>"); in_ul = false; }
            if in_ol { html.push_str("</ol>"); in_ol = false; }
            html.push_str("<p></p>");
        } else {
            if in_ul { html.push_str("</ul>"); in_ul = false; }
            if in_ol { html.push_str("</ol>"); in_ol = false; }
            html.push_str(&format!("<p>{}</p>", format_inline(trimmed)));
        }
    }

    if in_ul { html.push_str("</ul>"); }
    if in_ol { html.push_str("</ol>"); }
    if in_code_block { html.push_str("</code></pre>"); }

    html
}

fn format_inline(text: &str) -> String {
    let mut s = html_escape(text);

    // Bold **text**
    while let Some(start) = s.find("**") {
        if let Some(end) = s[start + 2..].find("**") {
            let actual_end = start + 2 + end;
            let inner = &s[start + 2..actual_end];
            s = format!("{}<strong>{}</strong>{}", &s[..start], inner, &s[actual_end + 2..]);
        } else {
            break;
        }
    }

    // Italic *text*
    while let Some(start) = s.find('*') {
        if let Some(end) = s[start + 1..].find('*') {
            let actual_end = start + 1 + end;
            let inner = &s[start + 1..actual_end];
            s = format!("{}<em>{}</em>{}", &s[..start], inner, &s[actual_end + 1..]);
        } else {
            break;
        }
    }

    // Strikethrough ~~text~~
    while let Some(start) = s.find("~~") {
        if let Some(end) = s[start + 2..].find("~~") {
            let actual_end = start + 2 + end;
            let inner = &s[start + 2..actual_end];
            s = format!("{}<del>{}</del>{}", &s[..start], inner, &s[actual_end + 2..]);
        } else {
            break;
        }
    }

    // Highlight <mark>text</mark>
    s = s.replace("&lt;mark&gt;", "<mark>");
    s = s.replace("&lt;/mark&gt;", "</mark>");

    // Underline <u>text</u>
    s = s.replace("&lt;u&gt;", "<u>");
    s = s.replace("&lt;/u&gt;", "</u>");

    // Inline code `code`
    while let Some(start) = s.find('`') {
        if let Some(end) = s[start + 1..].find('`') {
            let actual_end = start + 1 + end;
            let inner = &s[start + 1..actual_end];
            s = format!("{}<code>{}</code>{}", &s[..start], inner, &s[actual_end + 1..]);
        } else {
            break;
        }
    }

    s
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Converte frammenti HTML modificati nel DOM in Markdown pulito.
#[must_use]
pub fn html_to_markdown(html: &str) -> String {
    let mut s = html.to_string();

    s = s.replace("</h1>", "\n\n").replace("</h2>", "\n\n").replace("</h3>", "\n\n");
    s = s.replace("</p>", "\n\n").replace("<br>", "\n").replace("<br/>", "\n").replace("<br />", "\n");
    s = s.replace("</blockquote>", "\n\n");
    s = s.replace("</li>", "\n");
    s = s.replace("</ul>", "\n").replace("</ol>", "\n");

    while let Some(pos) = s.find("<h1") {
        if let Some(close) = s[pos..].find('>') {
            s.replace_range(pos..pos + close + 1, "# ");
        } else { break; }
    }
    while let Some(pos) = s.find("<h2") {
        if let Some(close) = s[pos..].find('>') {
            s.replace_range(pos..pos + close + 1, "## ");
        } else { break; }
    }
    while let Some(pos) = s.find("<h3") {
        if let Some(close) = s[pos..].find('>') {
            s.replace_range(pos..pos + close + 1, "### ");
        } else { break; }
    }
    while let Some(pos) = s.find("<blockquote") {
        if let Some(close) = s[pos..].find('>') {
            s.replace_range(pos..pos + close + 1, "> ");
        } else { break; }
    }
    while let Some(pos) = s.find("<li") {
        if let Some(close) = s[pos..].find('>') {
            s.replace_range(pos..pos + close + 1, "- ");
        } else { break; }
    }

    s = s.replace("<b>", "**").replace("</b>", "**");
    s = s.replace("<strong>", "**").replace("</strong>", "**");
    while let Some(pos) = s.find("<strong") {
        if let Some(close) = s[pos..].find('>') {
            s.replace_range(pos..pos + close + 1, "**");
        } else { break; }
    }

    s = s.replace("<i>", "*").replace("</i>", "*");
    s = s.replace("<em>", "*").replace("</em>", "*");
    while let Some(pos) = s.find("<em") {
        if let Some(close) = s[pos..].find('>') {
            s.replace_range(pos..pos + close + 1, "*");
        } else { break; }
    }

    s = s.replace("<del>", "~~").replace("</del>", "~~");
    s = s.replace("<s>", "~~").replace("</s>", "~~");
    while let Some(pos) = s.find("<del") {
        if let Some(close) = s[pos..].find('>') {
            s.replace_range(pos..pos + close + 1, "~~");
        } else { break; }
    }

    s = s.replace("<code>", "`").replace("</code>", "`");
    while let Some(pos) = s.find("<code") {
        if let Some(close) = s[pos..].find('>') {
            s.replace_range(pos..pos + close + 1, "`");
        } else { break; }
    }

    // Rimuove tag HTML rimanenti
    let mut clean = String::new();
    let mut inside = false;
    for c in s.chars() {
        if c == '<' {
            inside = true;
        } else if c == '>' {
            inside = false;
        } else if !inside {
            clean.push(c);
        }
    }

    clean.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .trim()
        .to_string()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_html_headings_and_inline() {
        let md = "# Titolo\nTesto con **grassetto** e *corsivo*.";
        let html = markdown_to_html(md);
        assert!(html.contains("<h1"));
        assert!(html.contains("Titolo</h1>"));
        assert!(html.contains("<strong"));
        assert!(html.contains("grassetto</strong>"));
        assert!(html.contains("<em"));
        assert!(html.contains("corsivo</em>"));
    }

    #[test]
    fn test_html_to_markdown_conversion() {
        let html = "<h1>Titolo</h1><p>Testo <b>forte</b> e <i>inclinato</i>.</p>";
        let md = html_to_markdown(html);
        assert!(md.contains("# Titolo"));
        assert!(md.contains("**forte**"));
        assert!(md.contains("*inclinato*"));
    }
}
