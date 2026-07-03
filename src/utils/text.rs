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

/// Length above which [`short_protocol_id`] starts eliding.
const SHORT_PROTOCOL_ID_THRESHOLD: usize = 32;

/// Return a compact, display-only label for long protocol identifiers
/// (DIDs / `ck:*` ids). Storage, inputs, copy buttons, routes, and API
/// payloads must keep the canonical value — this helper is intentionally for
/// read-only UI text such as menus, badges, rows, and status labels.
///
/// Moved here from yougen `views::helpers` (single source; sodmin can share).
pub fn short_protocol_id(value: impl AsRef<str>) -> String {
    let value = value.as_ref().trim();
    if value.len() <= SHORT_PROTOCOL_ID_THRESHOLD {
        return value.to_owned();
    }

    if let Some((prefix, tail)) = value.rsplit_once(':')
        && tail.len() >= 20
        && prefix.len() <= 20
    {
        return format!("{prefix}:{}", truncate_middle(tail, 8, 6));
    }

    truncate_middle(value, 16, 8)
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
