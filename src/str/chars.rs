pub struct Chars<'a> {
    s: &'a str,
}

pub fn chars(s: &str) -> Chars<'_> {
    Chars::new(s)
}

impl<'a> Chars<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { s }
    }

    pub fn as_str(&self) -> &'a str {
        self.s
    }
}

impl<'a> Iterator for Chars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.s.is_empty() {
            return None;
        }

        // return UTF 8 bytes that we have to decode in order to form a valid unicode.

        let bytes = self.s.as_bytes();

        // `self.s` is valid UTF-8, so its first byte is always a leading byte:
        // the number of leading `1` bits gives the char length in bytes, and
        // the mask keeps the payload bits that follow that prefix.
        let (len, mask): (usize, u8) = match bytes[0] {
            0x00..=0x7F => (1, 0x7F), // 0xxxxxxx, mask 01111111 keeps the 7 payload bits
            0xC0..=0xDF => (2, 0x1F), // 110xxxxx, mask 00011111 keeps the 5 payload bits
            0xE0..=0xEF => (3, 0x0F), // 1110xxxx, mask 00001111 keeps the 4 payload bits
            0xF0..=0xF7 => (4, 0x07), // 11110xxx, mask 00000111 keeps the 3 payload bits
            _ => unreachable!("continuation byte at char start in valid UTF-8"),
        };

        // codepoint which represent unicode,
        let mut cp: u32 = (bytes[0] & mask) as u32;

        for i in 1..len {
            cp = (cp << 6) | (bytes[i] & 0x3F) as u32; // 0x3F drops the 2-bit `10` continuation prefix
        }

        self.s = &self.s[len..];
        char::from_u32(cp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_yields_nothing() {
        assert_eq!(chars("").next(), None);
    }

    #[test]
    fn ascii_is_one_char_per_byte() {
        assert_eq!(
            chars("hello").collect::<Vec<_>>(),
            ['h', 'e', 'l', 'l', 'o']
        );
    }

    #[test]
    fn decodes_every_multibyte_length() {
        assert_eq!(chars("é").next(), Some('é')); // 2 bytes
        assert_eq!(chars("€").next(), Some('€')); // 3 bytes
        assert_eq!(chars("😀").next(), Some('😀')); // 4 bytes
        assert_eq!(chars("aé€😀").collect::<Vec<_>>(), ['a', 'é', '€', '😀']);
    }

    #[test]
    fn decodes_the_edges_of_each_encoding_range() {
        // last and first code points of the 1, 2, 3 and 4 byte encodings.
        let edges = "\u{7F}\u{80}\u{7FF}\u{800}\u{FFFF}\u{10000}\u{10FFFF}";
        assert_eq!(
            chars(edges).collect::<Vec<_>>(),
            edges.chars().collect::<Vec<_>>()
        );
    }

    #[test]
    fn matches_std_on_mixed_input() {
        let s = "Grüße, 世界! 🦀🦀 \0\n";
        assert!(chars(s).eq(s.chars()));
    }

    #[test]
    fn as_str_is_the_undecoded_remainder() {
        let mut it = chars("aé€");
        assert_eq!(it.as_str(), "aé€");
        it.next();
        assert_eq!(it.as_str(), "é€");
        it.next();
        assert_eq!(it.as_str(), "€");
        it.next();
        assert_eq!(it.as_str(), "");
    }

    #[test]
    fn stays_exhausted_after_the_end() {
        let mut it = chars("a");
        assert_eq!(it.next(), Some('a'));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }
}
