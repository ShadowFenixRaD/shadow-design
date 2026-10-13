//! Utility helpers for shadow-design.

/// Combines multiple class name fragments, splitting on whitespace,
/// removing empty entries, and deduplicating tokens while preserving order.
#[allow(dead_code)]
pub(crate) fn cx<'a>(parts: impl IntoIterator<Item = &'a str>) -> String {
    let mut seen = Vec::new();
    for part in parts {
        for token in part.split_whitespace() {
            if !seen.contains(&token) {
                seen.push(token);
            }
        }
    }
    seen.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cx_empty() {
        assert_eq!(cx([]), "");
        assert_eq!(cx(["", "  ", ""]), "");
    }

    #[test]
    fn test_cx_joins_and_trims() {
        assert_eq!(cx(["sd-btn", "sd-btn--primary"]), "sd-btn sd-btn--primary");
        assert_eq!(cx(["  sd-btn  ", " sd-btn--md "]), "sd-btn sd-btn--md");
    }

    #[test]
    fn test_cx_deduplicates() {
        assert_eq!(
            cx(["sd-btn", "sd-btn--primary", "sd-btn", "rounded-md"]),
            "sd-btn sd-btn--primary rounded-md"
        );
        assert_eq!(
            cx(["p-4 m-2", "p-4 font-bold"]),
            "p-4 m-2 font-bold"
        );
    }
}
