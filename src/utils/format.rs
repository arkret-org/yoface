//! Numeric / size formatting.

/// Format a byte count as a human-readable size (B/KB/MB/GB/TB) using base 1024.
///
/// Previously yougen `file_transfer::format_size` and sodmin
/// `pages::media::format_bytes` each carried an equivalent copy; consolidated
/// here. Rules:
/// - Below 1 KB: `{n} B` (integer).
/// - Otherwise: scale to the largest fitting unit; values >= 10 use 0 decimal
///   places, otherwise 1 decimal place.
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit = 0usize;
    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else if value >= 10.0 {
        format!("{value:.0} {}", UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_bytes_units() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(10 * 1024), "10 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(format_bytes(1024_u64.pow(4)), "1.0 TB");
    }
}
