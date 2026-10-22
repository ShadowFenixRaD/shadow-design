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
    let haystack = if case_sensitive {
        text.to_string()
    } else {
        text.to_lowercase()
    };
    let needle = if case_sensitive {
        query.to_string()
    } else {
        query.to_lowercase()
    };

    let mut start = 0;
    while let Some(pos) = haystack[start..].find(&needle) {
        let match_start = start + pos;
        let match_end = match_start + needle.len();

        if match_start > start {
            chunks.push(Chunk {
                text: text[start..match_start].to_string(),
                is_match: false,
            });
        }

        chunks.push(Chunk {
            text: text[match_start..match_end].to_string(),
            is_match: true,
        });

        start = match_end;
    }

    if start < text.len() {
        chunks.push(Chunk {
            text: text[start..].to_string(),
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
