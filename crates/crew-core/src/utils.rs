//! Shared utility functions.

/// Truncate a string at a UTF-8 safe boundary, appending a suffix.
///
/// Does nothing if `s.len() <= max_len`.
pub fn truncate_utf8(s: &mut String, max_len: usize, suffix: &str) {
    if s.len() <= max_len {
        return;
    }
    let mut limit = max_len;
    while limit > 0 && !s.is_char_boundary(limit) {
        limit -= 1;
    }
    s.truncate(limit);
    s.push_str(suffix);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_no_op() {
        let mut s = "hello".to_string();
        truncate_utf8(&mut s, 10, "...");
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_truncate_ascii() {
        let mut s = "abcdefghij".to_string();
        truncate_utf8(&mut s, 5, "...");
        assert_eq!(s, "abcde...");
    }

    #[test]
    fn test_truncate_utf8_boundary() {
        // 你好世 = 9 bytes, truncate at 7 should back up to byte 6
        let mut s = "\u{4F60}\u{597D}\u{4E16}".to_string();
        truncate_utf8(&mut s, 7, "...");
        assert_eq!(s, "\u{4F60}\u{597D}...");
    }
}
