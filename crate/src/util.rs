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

/// Verifica se una stringa è un colore esadecimale CSS valido (#rgb, #rgba, #rrggbb, #rrggbbaa).
#[must_use]
pub fn is_valid_hex_color(hex: &str) -> bool {
    let s = hex.trim();
    if !s.starts_with('#') {
        return false;
    }
    let body = &s[1..];
    (body.len() == 3 || body.len() == 4 || body.len() == 6 || body.len() == 8)
        && body.chars().all(|c| c.is_ascii_hexdigit())
}

/// Converte un colore hex valido in tuple (r, g, b, alpha_0_to_1).
#[must_use]
pub fn parse_hex_rgba(input: &str) -> Option<(u8, u8, u8, f32)> {
    let clean = input.trim().trim_start_matches('#');
    match clean.len() {
        3 => {
            let r = u8::from_str_radix(&clean[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&clean[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&clean[2..3].repeat(2), 16).ok()?;
            Some((r, g, b, 1.0))
        }
        4 => {
            let r = u8::from_str_radix(&clean[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&clean[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&clean[2..3].repeat(2), 16).ok()?;
            let a_val = u8::from_str_radix(&clean[3..4].repeat(2), 16).ok()?;
            Some((r, g, b, (a_val as f32) / 255.0))
        }
        6 => {
            let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
            let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
            let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
            Some((r, g, b, 1.0))
        }
        8 => {
            let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
            let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
            let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
            let a_val = u8::from_str_radix(&clean[6..8], 16).ok()?;
            Some((r, g, b, (a_val as f32) / 255.0))
        }
        _ => None,
    }
}

/// Converte un colore hex valido in tuple (r, g, b). Se non valido, restituisce None.
#[must_use]
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let (r, g, b, _) = parse_hex_rgba(hex)?;
    Some((r, g, b))
}

/// Asynchronous sleep helper compatible with wasm32 and native architectures.
#[allow(dead_code)]
pub(crate) async fn sleep_ms(ms: u64) {
    #[cfg(target_arch = "wasm32")]
    {
        gloo_timers::future::TimeoutFuture::new(ms as u32).await;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
    }
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

    #[test]
    fn test_hex_helpers() {
        assert!(is_valid_hex_color("#fff"));
        assert!(is_valid_hex_color("#a855f7"));
        assert!(!is_valid_hex_color("not_a_hex"));
        assert_eq!(hex_to_rgb("#ffffff"), Some((255, 255, 255)));
        assert_eq!(hex_to_rgb("#000"), Some((0, 0, 0)));
        assert_eq!(parse_hex_rgba("#ff000080").map(|(r,g,b,_)| (r,g,b)), Some((255, 0, 0)));
    }
}
