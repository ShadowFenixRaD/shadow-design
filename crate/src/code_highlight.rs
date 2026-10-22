use crate::copy_button::CopyButton;
#[cfg(feature = "theme")]
use crate::theme::use_theme;
use dioxus::prelude::*;

/// Categoria sintattica del token evidenziato.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxTokenKind {
    Keyword,
    Function,
    Type,
    String,
    Comment,
    Number,
    Operator,
    Plain,
}

impl SyntaxTokenKind {
    /// Ritorna la classe CSS associata al tipo di token.
    #[must_use]
    pub const fn class_name(&self) -> &'static str {
        match self {
            Self::Keyword => "sd-token-keyword",
            Self::Function => "sd-token-fn",
            Self::Type => "sd-token-type",
            Self::String => "sd-token-string",
            Self::Comment => "sd-token-comment",
            Self::Number => "sd-token-number",
            Self::Operator => "sd-token-operator",
            Self::Plain => "sd-token-plain",
        }
    }
}

/// Token sintattico analizzato.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyntaxToken {
    pub kind: SyntaxTokenKind,
    pub text: String,
}

/// Tokenizer sintattico leggero a zero dipendenze per linguaggi comuni.
#[must_use]
pub fn tokenize_line(line: &str, lang: &str) -> Vec<SyntaxToken> {
    let lower_lang = lang.to_lowercase();
    let mut tokens = Vec::new();

    // Rileva commenti riga intera
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with('#') {
        tokens.push(SyntaxToken {
            kind: SyntaxTokenKind::Comment,
            text: line.to_string(),
        });
        return tokens;
    }

    let is_rust = lower_lang.contains("rust") || lower_lang == "rs";
    let is_js = lower_lang.contains("js") || lower_lang.contains("ts");
    let is_py = lower_lang.contains("py");

    let rust_keywords = [
        "pub", "fn", "let", "mut", "const", "static", "struct", "enum", "impl", "trait", "type",
        "match", "if", "else", "for", "in", "while", "loop", "return", "use", "mod", "as",
        "async", "await", "move", "where", "self", "Self", "rsx", "component",
    ];

    let js_keywords = [
        "const", "let", "var", "function", "return", "if", "else", "for", "while", "import",
        "export", "from", "default", "class", "async", "await", "new", "this", "typeof",
    ];

    let py_keywords = [
        "def", "class", "return", "if", "elif", "else", "for", "while", "import", "from",
        "as", "try", "except", "with", "async", "await", "self",
    ];

    // Tokenizer a scansione di caratteri
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let ch = chars[i];

        // Stringhe letterali
        if ch == '"' || ch == '\'' || ch == '`' {
            let quote = ch;
            let mut s = String::new();
            s.push(quote);
            i += 1;
            while i < len {
                let c = chars[i];
                s.push(c);
                if c == quote && (i == 0 || chars[i - 1] != '\\') {
                    i += 1;
                    break;
                }
                i += 1;
            }
            tokens.push(SyntaxToken {
                kind: SyntaxTokenKind::String,
                text: s,
            });
            continue;
        }

        // Commenti inline //
        if ch == '/' && i + 1 < len && chars[i + 1] == '/' {
            let comment_text: String = chars[i..].iter().collect();
            tokens.push(SyntaxToken {
                kind: SyntaxTokenKind::Comment,
                text: comment_text,
            });
            break;
        }

        // Numeri
        if ch.is_ascii_digit() && (i == 0 || !chars[i - 1].is_alphanumeric()) {
            let mut num = String::new();
            while i < len && (chars[i].is_ascii_digit() || chars[i] == '.' || chars[i] == '_') {
                num.push(chars[i]);
                i += 1;
            }
            tokens.push(SyntaxToken {
                kind: SyntaxTokenKind::Number,
                text: num,
            });
            continue;
        }

        // Identificatori / Parole chiave
        if ch.is_alphabetic() || ch == '_' {
            let mut word = String::new();
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                word.push(chars[i]);
                i += 1;
            }

            let is_keyword = (is_rust && rust_keywords.contains(&word.as_str()))
                || (is_js && js_keywords.contains(&word.as_str()))
                || (is_py && py_keywords.contains(&word.as_str()));

            let kind = if is_keyword {
                SyntaxTokenKind::Keyword
            } else if word.chars().next().is_some_and(|c| c.is_uppercase()) {
                SyntaxTokenKind::Type
            } else if i < len && chars[i] == '(' {
                SyntaxTokenKind::Function
            } else {
                SyntaxTokenKind::Plain
            };

            tokens.push(SyntaxToken { kind, text: word });
            continue;
        }

        // Operatori o punteggiatura
        if "=+-*/<>!&|:;,.(){}[]".contains(ch) {
            tokens.push(SyntaxToken {
                kind: SyntaxTokenKind::Operator,
                text: ch.to_string(),
            });
            i += 1;
            continue;
        }

        // Spazi o altri caratteri
        tokens.push(SyntaxToken {
            kind: SyntaxTokenKind::Plain,
            text: ch.to_string(),
        });
        i += 1;
    }

    tokens
}

/// Blocco di visualizzazione codice sorgente con syntax highlighting multilinguaggio,
/// numerazione delle righe, badge del linguaggio e pulsante di copia negli appunti.
#[component]
pub fn CodeBlock(
    /// Codice sorgente da visualizzare ed evidenziare.
    code: String,
    /// Linguaggio di programmazione per il tokenizer (es. "rust", "rsx", "javascript", "python", "json", "bash").
    #[props(default = "rust".to_string())]
    language: String,
    /// Nome del file opzionale visualizzato nella barra superiore (es. "main.rs").
    #[props(default)]
    filename: Option<String>,
    /// Se mostrare la numerazione delle righe sul margine sinistro. Defaults to true.
    #[props(default = true)]
    with_line_numbers: bool,
    /// Se includere il pulsante rapido di copia negli appunti. Defaults to true.
    #[props(default = true)]
    with_copy_button: bool,
    /// Numeri di riga (1-indexed) da evidenziare visivamente con accento luminoso.
    #[props(default)]
    highlight_lines: Vec<usize>,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    let lines: Vec<&str> = code.lines().collect();

    #[cfg(feature = "theme")]
    let _theme = use_theme();

    let display_title = filename.unwrap_or_else(|| language.to_uppercase());

    rsx! {
        div { class: "sd-code-block {class}",
            // Barra superiore del blocco codice
            div { class: "sd-code-block-header",
                div { class: "sd-code-block-title-group",
                    span { class: "sd-code-block-lang-badge", "{display_title}" }
                }
                if with_copy_button {
                    div { class: "sd-code-block-actions",
                        CopyButton {
                            value: code.clone(),
                            label: Some("Copia".to_string()),
                            copied_label: "Copiato!".to_string(),
                        }
                    }
                }
            }

            // Area codice preformattata con righe
            div { class: "sd-code-block-body",
                pre { class: "sd-code-pre",
                    code { class: "sd-code-content",
                        for (idx, line_str) in lines.iter().enumerate() {
                            {
                                let line_num = idx + 1;
                                let is_hl = highlight_lines.contains(&line_num);
                                let line_class = if is_hl { "sd-code-line sd-code-line--highlighted" } else { "sd-code-line" };
                                let tokens = tokenize_line(line_str, &language);

                                rsx! {
                                    div { key: "{line_num}", class: "{line_class}",
                                        if with_line_numbers {
                                            span { class: "sd-code-line-number", "{line_num}" }
                                        }
                                        span { class: "sd-code-line-text",
                                            if tokens.is_empty() {
                                                " "
                                            } else {
                                                for (t_idx, token) in tokens.iter().enumerate() {
                                                    span {
                                                        key: "{t_idx}",
                                                        class: "{token.kind.class_name()}",
                                                        "{token.text}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Componente equivalente a [`CodeBlock`].
#[component]
pub fn CodeHighlight(
    /// Codice sorgente da visualizzare ed evidenziare.
    code: String,
    /// Linguaggio di programmazione per il tokenizer. Defaults to "rust".
    #[props(default = "rust".to_string())]
    language: String,
    /// Nome del file opzionale visualizzato nella barra superiore.
    #[props(default)]
    filename: Option<String>,
    /// Se mostrare la numerazione delle righe sul margine sinistro. Defaults to true.
    #[props(default = true)]
    with_line_numbers: bool,
    /// Se includere il pulsante rapido di copia negli appunti. Defaults to true.
    #[props(default = true)]
    with_copy_button: bool,
    /// Numeri di riga (1-indexed) da evidenziare visivamente con accento luminoso.
    #[props(default)]
    highlight_lines: Vec<usize>,
    /// Classi CSS aggiuntive.
    #[props(default)]
    class: String,
) -> Element {
    rsx! {
        CodeBlock {
            code,
            language,
            filename,
            with_line_numbers,
            with_copy_button,
            highlight_lines,
            class,
        }
    }
}
