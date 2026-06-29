//! Text truncation / display labels.

/// Middle-elide an overly long string as "head...tail"; for read-only UI text.
///
/// Previously yougen `views::helpers::shorten_ascii_middle` and several inline
/// `take(n)+"..."` snippets in sodmin each carried a copy; consolidated here.
/// Returns the value unchanged when `value.len() <= head + tail + 3`; otherwise
/// returns `{head}...{tail}`.
///
/// Note: slices by *byte*, so it is only safe for ASCII identifiers
/// (DID / handle / id, etc.). Callers must handle non-ASCII input themselves.
pub fn truncate_middle(value: &str, head: usize, tail: usize) -> String {
    let len = value.len();
    if len <= head + tail + 3 {
        return value.to_owned();
    }
    format!("{}...{}", &value[..head], &value[len - tail..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_middle_short_passthrough() {
        assert_eq!(truncate_middle("abc", 8, 6), "abc");
    }

    #[test]
    fn truncate_middle_long() {
        let s = "0123456789abcdefghijklmnopqrstuvwxyz";
        assert_eq!(truncate_middle(s, 4, 4), "0123...wxyz");
    }
}
