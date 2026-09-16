/// returns number of chars inside str
///
/// Rust libraries may assume that string slices are always valid UTF-8.
/// https://doc.rust-lang.org/std/primitive.str.html#invariant
///
/// counts every byte that is not a UTF-8 continuation byte (`10xxxxxx`), i.e. one byte per char.

pub fn char_count(s: &str) -> usize {
    let bytes = s.as_bytes();
    bytes.iter().filter(|&&b| b & 0xC0 != 0x80).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_is_zero() {
        assert_eq!(char_count(""), 0);
    }

    #[test]
    fn ascii_is_one_byte_per_char() {
        assert_eq!(char_count("hello"), 5);
    }

    #[test]
    fn multibyte_chars_count_once() {
        assert_eq!(char_count("é"), 1); // 2 bytes
        assert_eq!(char_count("€"), 1); // 3 bytes
        assert_eq!(char_count("😀"), 1); // 4 bytes
        assert_eq!(char_count("aé€😀"), 4);
    }
}
