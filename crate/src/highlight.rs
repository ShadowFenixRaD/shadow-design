use dioxus::prelude::*;

#[derive(Clone, PartialEq, Debug)]
struct Chunk {
    text: String,
    is_match: bool,
}

fn split_highlight(text: &str, query: &str, case_sensitive: bool) -> Vec<Chunk> {
    if query.is_empty() {
        return vec![Chunk {
            text: text.to_string(),
            is_match: false,
        }];
    }

    let mut chunks = Vec::new();
    let text_chars: Vec<char> = text.chars().collect();
    let query_chars: Vec<char> = query.chars().collect();

    if query_chars.is_empty() || text_chars.len() < query_chars.len() {
        return vec![Chunk {
            text: text.to_string(),
            is_match: false,
        }];
    }

    let matches_at = |start_idx: usize| -> bool {
        if start_idx + query_chars.len() > text_chars.len() {
            return false;
        }
        for (qi, &qc) in query_chars.iter().enumerate() {
            let tc = text_chars[start_idx + qi];
            if case_sensitive {
                if tc != qc {
                    return false;
                }
            } else if tc.to_lowercase().ne(qc.to_lowercase()) {
                return false;
            }
        }
        true
    };

    let mut i = 0;
    let mut last_end = 0;

    while i <= text_chars.len().saturating_sub(query_chars.len()) {
        if matches_at(i) {
            if i > last_end {
                let unmatch: String = text_chars[last_end..i].iter().collect();
                chunks.push(Chunk {
                    text: unmatch,
                    is_match: false,
                });
            }
            let matched: String = text_chars[i..i + query_chars.len()].iter().collect();
            chunks.push(Chunk {
                text: matched,
                is_match: true,
            });
            i += query_chars.len();
            last_end = i;
        } else {
            i += 1;
        }
    }

    if last_end < text_chars.len() {
        let tail: String = text_chars[last_end..].iter().collect();
        chunks.push(Chunk {
            text: tail,
            is_match: false,
        });
    }

    chunks
}

/// Typography component highlighting matched search substrings with contrasting accent background.
#[component]
pub fn Highlight(
    /// Source text content to display.
    text: String,
    /// Substring query to highlight within the text.
    query: String,
    /// Whether query matching should be case-sensitive. Defaults to false.
    #[props(default = false)]
    case_sensitive: bool,
    /// Custom color hex or CSS style for the highlight marker.
    #[props(default)]
    highlight_color: Option<String>,
    /// Additional CSS classes for the highlight `<mark>` element.
    #[props(default)]
    highlight_class: String,
    /// Additional CSS classes for the root container.
    #[props(default)]
    class: String,
) -> Element {
    let chunks = split_highlight(&text, &query, case_sensitive);

    let mark_style = if let Some(ref c) = highlight_color {
        format!("--sd-highlight-color: {c};")
    } else {
        String::new()
    };

    rsx! {
        span { class: "sd-highlight-container {class}",
            for (idx, chunk) in chunks.into_iter().enumerate() {
                if chunk.is_match {
                    mark {
                        key: "{idx}",
                        class: "sd-highlight {highlight_class}",
                        style: "{mark_style}",
                        "{chunk.text}"
                    }
                } else {
                    span {
                        key: "{idx}",
                        "{chunk.text}"
                    }
                }
            }
        }
    }
}
