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
            Self::Bold => "Grassetto (Ctrl+B)",
            Self::Italic => "Corsivo (Ctrl+I)",
            Self::Underline => "Sottolineato (Ctrl+U)",
            Self::Strikethrough => "Barrato",
            Self::Highlight => "Evidenziatore",
            Self::ClearFormat => "Cancella formato",
            Self::Heading(lvl) => match lvl {
                1 => "Intestazione H1",
                2 => "Intestazione H2",
                3 => "Intestazione H3",
                _ => "Intestazione",
            },
            Self::Blockquote => "Citazione",
            Self::BulletList => "Elenco puntato",
            Self::OrderedList => "Elenco numerato",
            Self::CodeBlock => "Blocco codice",
            Self::InsertTable => "Tabella",
            Self::InsertLink => "Inserisci link",
            Self::InsertImage => "Inserisci immagine",
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
                "| Intestazione 1 | Intestazione 2 |\n| --- | --- |\n| Cella 1 | Cella 2 |".to_string()
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

    let active_cls = if active {
        "bg-purple-600/30 text-purple-200 border-purple-500/40"
    } else {
        "border-transparent hover:bg-white/[0.06]"
    };
    let disabled_cls = if disabled { "opacity-40 cursor-not-allowed" } else { "" };

    rsx! {
        button {
            r#type: "button",
            class: "sd-rte__control p-1.5 rounded-md text-zinc-300 hover:text-white transition-colors cursor-pointer border {active_cls} {disabled_cls}",
            title: "{lbl}",
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
        div { class: "sd-rte__group flex items-center gap-0.5 px-1 py-0.5 rounded-lg bg-white/[0.02] border border-white/[0.06]",
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
    let sticky_cls = if sticky { "sticky top-0 z-10" } else { "" };

    rsx! {
        div {
            class: "sd-rte__toolbar flex flex-wrap items-center gap-1.5 p-2 bg-zinc-950/80 border-b border-white/[0.08] backdrop-blur-md {sticky_cls}",
            {children}
        }
    }
}

/// Editor di testo ricco in stile TipTap: visuale e WYSIWYG per visualizzare e modificare direttamente il testo formattato, con supporto a Markdown e visualizzazione Split.
#[component]
pub fn RichTextEditor(
    mut value: Signal<String>,
    #[props(default = "Scrivi qui il contenuto...".to_string())] placeholder: String,
    #[props(default = true)] editable: bool,
    #[props(default = true)] sticky_toolbar: bool,
    #[props(default)] mode: Option<RichTextMode>,
    #[props(default)] min_height: Option<String>,
    #[props(default)] class: String,
) -> Element {
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
                RichTextCommand::Highlight => "document.execCommand('hiliteColor', false, 'rgba(168, 85, 247, 0.4)');",
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
                    "const u = prompt('Inserisci URL link:'); if (u) document.execCommand('createLink', false, u);"
                }
                RichTextCommand::InsertImage => {
                    "const u = prompt('Inserisci URL immagine:'); if (u) document.execCommand('insertImage', false, u);"
                }
                RichTextCommand::InsertTable => {
                    "document.execCommand('insertHTML', false, '<table class=\"border border-white/20 w-full my-2 text-xs\"><tr><th class=\"border border-white/20 p-1\">Colonna 1</th><th class=\"border border-white/20 p-1\">Colonna 2</th></tr><tr><td class=\"border border-white/20 p-1\">Dato 1</td><td class=\"border border-white/20 p-1\">Dato 2</td></tr></table>');"
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

    let split_cls = if current_mode() == RichTextMode::Split {
        "grid-cols-2 divide-x divide-white/[0.08]"
    } else {
        "grid-cols-1"
    };

    let btn_preview_cls = if current_mode() == RichTextMode::Wysiwyg { "bg-purple-600 text-white font-semibold" } else { "text-zinc-400 hover:text-zinc-200" };
    let btn_md_cls = if current_mode() == RichTextMode::Markdown { "bg-purple-600 text-white font-semibold" } else { "text-zinc-400 hover:text-zinc-200" };
    let btn_split_cls = if current_mode() == RichTextMode::Split { "bg-purple-600 text-white font-semibold" } else { "text-zinc-400 hover:text-zinc-200" };

    let eid_sync = editor_id.clone();

    rsx! {
        div { class: "sd-rte w-full rounded-xl border border-white/[0.12] bg-zinc-950/60 overflow-hidden shadow-lg transition-all focus-within:border-purple-500/50 {class}",
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
                div { class: "ml-auto flex items-center gap-1 bg-white/[0.04] p-0.5 rounded-lg border border-white/[0.06] text-xs font-mono",
                    button {
                        r#type: "button",
                        class: "px-2 py-1 rounded-md transition-colors {btn_preview_cls}",
                        onclick: move |_| current_mode.set(RichTextMode::Wysiwyg),
                        "Visuale (TipTap)"
                    }
                    button {
                        r#type: "button",
                        class: "px-2 py-1 rounded-md transition-colors {btn_md_cls}",
                        onclick: move |_| current_mode.set(RichTextMode::Markdown),
                        "Markdown"
                    }
                    button {
                        r#type: "button",
                        class: "px-2 py-1 rounded-md transition-colors {btn_split_cls}",
                        onclick: move |_| current_mode.set(RichTextMode::Split),
                        "Split"
                    }
                }
            }

            // Area Contenuto
            div { class: "w-full grid {split_cls}",
                // Vista Editor Markdown
                if current_mode() == RichTextMode::Markdown || current_mode() == RichTextMode::Split {
                    textarea {
                        class: "w-full p-4 bg-transparent text-zinc-100 font-mono text-sm leading-relaxed outline-none resize-y min-h-[{h_style}]",
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
                        class: "sd-rte__editable w-full p-4 text-zinc-100 text-sm leading-relaxed outline-none min-h-[{h_style}] overflow-y-auto cursor-text",
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
                html.push_str("<pre class=\"bg-zinc-900 border border-white/10 p-3 rounded-lg font-mono text-xs text-purple-300 overflow-x-auto my-2\"><code>");
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
            html.push_str(&format!("<h1 class=\"text-2xl font-bold text-white border-b border-white/10 pb-1 mt-3 mb-2\">{}</h1>", format_inline(h1)));
        } else if let Some(h2) = trimmed.strip_prefix("## ") {
            if in_ul { html.push_str("</ul>"); in_ul = false; }
            if in_ol { html.push_str("</ol>"); in_ol = false; }
            html.push_str(&format!("<h2 class=\"text-xl font-bold text-purple-300 mt-3 mb-1.5\">{}</h2>", format_inline(h2)));
        } else if let Some(h3) = trimmed.strip_prefix("### ") {
            if in_ul { html.push_str("</ul>"); in_ul = false; }
            if in_ol { html.push_str("</ol>"); in_ol = false; }
            html.push_str(&format!("<h3 class=\"text-lg font-semibold text-zinc-100 mt-2 mb-1\">{}</h3>", format_inline(h3)));
        } else if let Some(bq) = trimmed.strip_prefix("> ") {
            if in_ul { html.push_str("</ul>"); in_ul = false; }
            if in_ol { html.push_str("</ol>"); in_ol = false; }
            html.push_str(&format!("<blockquote class=\"border-l-4 border-purple-500 pl-3 my-2 text-zinc-400 italic\">{}</blockquote>", format_inline(bq)));
        } else if let Some(li) = trimmed.strip_prefix("- ").or_else(|| trimmed.strip_prefix("* ")) {
            if in_ol { html.push_str("</ol>"); in_ol = false; }
            if !in_ul {
                html.push_str("<ul class=\"list-disc list-inside space-y-1 my-1\">");
                in_ul = true;
            }
            html.push_str(&format!("<li class=\"text-zinc-200\">{}</li>", format_inline(li)));
        } else if trimmed.chars().next().is_some_and(|c| c.is_ascii_digit()) && trimmed.contains(". ") {
            if in_ul { html.push_str("</ul>"); in_ul = false; }
            if !in_ol {
                html.push_str("<ol class=\"list-decimal list-inside space-y-1 my-1\">");
                in_ol = true;
            }
            let text = trimmed.split_once(". ").map(|(_, t)| t).unwrap_or(trimmed);
            html.push_str(&format!("<li class=\"text-zinc-200\">{}</li>", format_inline(text)));
        } else if trimmed.is_empty() {
            if in_ul { html.push_str("</ul>"); in_ul = false; }
            if in_ol { html.push_str("</ol>"); in_ol = false; }
            html.push_str("<p class=\"h-2\"></p>");
        } else {
            if in_ul { html.push_str("</ul>"); in_ul = false; }
            if in_ol { html.push_str("</ol>"); in_ol = false; }
            html.push_str(&format!("<p class=\"text-zinc-200 leading-relaxed my-1\">{}</p>", format_inline(trimmed)));
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
            s = format!("{}<strong class=\"font-bold text-white\">{}</strong>{}", &s[..start], inner, &s[actual_end + 2..]);
        } else {
            break;
        }
    }

    // Italic *text*
    while let Some(start) = s.find('*') {
        if let Some(end) = s[start + 1..].find('*') {
            let actual_end = start + 1 + end;
            let inner = &s[start + 1..actual_end];
            s = format!("{}<em class=\"italic text-purple-200\">{}</em>{}", &s[..start], inner, &s[actual_end + 1..]);
        } else {
            break;
        }
    }

    // Strikethrough ~~text~~
    while let Some(start) = s.find("~~") {
        if let Some(end) = s[start + 2..].find("~~") {
            let actual_end = start + 2 + end;
            let inner = &s[start + 2..actual_end];
            s = format!("{}<del class=\"line-through text-zinc-500\">{}</del>{}", &s[..start], inner, &s[actual_end + 2..]);
        } else {
            break;
        }
    }

    // Highlight <mark>text</mark>
    s = s.replace("&lt;mark&gt;", "<mark class=\"bg-purple-900/60 text-purple-200 px-1 py-0.5 rounded border border-purple-500/30\">");
    s = s.replace("&lt;/mark&gt;", "</mark>");

    // Underline <u>text</u>
    s = s.replace("&lt;u&gt;", "<u class=\"underline decoration-purple-400 decoration-1 underline-offset-2\">");
    s = s.replace("&lt;/u&gt;", "</u>");

    // Inline code `code`
    while let Some(start) = s.find('`') {
        if let Some(end) = s[start + 1..].find('`') {
            let actual_end = start + 1 + end;
            let inner = &s[start + 1..actual_end];
            s = format!("{}<code class=\"bg-zinc-800/80 text-purple-300 font-mono text-xs px-1.5 py-0.5 rounded border border-white/10\">{}</code>{}", &s[..start], inner, &s[actual_end + 1..]);
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
