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

    /// Applica il comando su un testo selezionato o per inserimento sintassi.
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

/// Editor di testo ricco WYSIWYG / Markdown con toolbar modulare e zero dipendenze esterne.
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
    let h_style = min_height.unwrap_or_else(|| "220px".to_string());

    let apply_cmd = move |cmd: RichTextCommand| {
        let text = value();
        let new_text = cmd.apply_to_text(&text);
        value.set(new_text);
    };

    let split_cls = if current_mode() == RichTextMode::Split {
        "grid-cols-2 divide-x divide-white/[0.08]"
    } else {
        "grid-cols-1"
    };

    let btn_preview_cls = if current_mode() == RichTextMode::Wysiwyg { "bg-purple-600 text-white font-semibold" } else { "text-zinc-400 hover:text-zinc-200" };
    let btn_md_cls = if current_mode() == RichTextMode::Markdown { "bg-purple-600 text-white font-semibold" } else { "text-zinc-400 hover:text-zinc-200" };
    let btn_split_cls = if current_mode() == RichTextMode::Split { "bg-purple-600 text-white font-semibold" } else { "text-zinc-400 hover:text-zinc-200" };

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

                // Switcher modalità (WYSIWYG / Markdown / Split)
                div { class: "ml-auto flex items-center gap-1 bg-white/[0.04] p-0.5 rounded-lg border border-white/[0.06] text-xs font-mono",
                    button {
                        r#type: "button",
                        class: "px-2 py-1 rounded-md transition-colors {btn_preview_cls}",
                        onclick: move |_| current_mode.set(RichTextMode::Wysiwyg),
                        "Preview"
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

                // Vista Visuale Renderizzata
                if current_mode() == RichTextMode::Wysiwyg || current_mode() == RichTextMode::Split {
                    div {
                        class: "w-full p-4 text-zinc-200 text-sm leading-relaxed min-h-[{h_style}] overflow-y-auto prose prose-invert max-w-none",
                        if value().is_empty() {
                            span { class: "text-zinc-500 italic select-none", "{placeholder}" }
                        } else {
                            RenderMarkdownSimple { content: value() }
                        }
                    }
                }
            }
        }
    }
}

/// Renderer interno semplificato del Markdown per preview visuale ad alta velocità.
#[component]
fn RenderMarkdownSimple(content: String) -> Element {
    let lines: Vec<&str> = content.lines().collect();

    rsx! {
        div { class: "flex flex-col gap-2.5",
            for (idx, line) in lines.iter().enumerate() {
                {
                    let trimmed = line.trim();
                    if let Some(h1_text) = trimmed.strip_prefix("# ") {
                        rsx! { h1 { key: "{idx}", class: "text-2xl font-bold text-white mt-2 pb-1 border-b border-white/10", "{h1_text}" } }
                    } else if let Some(h2_text) = trimmed.strip_prefix("## ") {
                        rsx! { h2 { key: "{idx}", class: "text-xl font-bold text-purple-300 mt-2", "{h2_text}" } }
                    } else if let Some(h3_text) = trimmed.strip_prefix("### ") {
                        rsx! { h3 { key: "{idx}", class: "text-lg font-semibold text-zinc-200 mt-1", "{h3_text}" } }
                    } else if let Some(b_text) = trimmed.strip_prefix("> ") {
                        rsx! { blockquote { key: "{idx}", class: "pl-3 border-l-2 border-purple-500 text-zinc-400 italic my-1", "{b_text}" } }
                    } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                        let li_text = if let Some(t) = trimmed.strip_prefix("- ") { t } else { trimmed.strip_prefix("* ").unwrap_or(trimmed) };
                        rsx! { li { key: "{idx}", class: "ml-4 list-disc text-zinc-300", "{li_text}" } }
                    } else if trimmed.starts_with("```") {
                        rsx! { div { key: "{idx}", class: "px-3 py-2 rounded-lg bg-zinc-900 border border-white/10 font-mono text-xs text-purple-300", "{trimmed}" } }
                    } else if trimmed.starts_with('|') && trimmed.ends_with('|') {
                        rsx! { div { key: "{idx}", class: "font-mono text-xs text-zinc-300 bg-white/[0.02] p-2 rounded-md border border-white/10 overflow-x-auto", "{trimmed}" } }
                    } else if !trimmed.is_empty() {
                        rsx! { p { key: "{idx}", class: "text-zinc-200 leading-relaxed", "{trimmed}" } }
                    } else {
                        rsx! { div { key: "{idx}", class: "h-1" } }
                    }
                }
            }
        }
    }
}
