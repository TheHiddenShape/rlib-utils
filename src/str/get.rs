use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

pub trait StrIndex {
    fn get(self, s: &str) -> Option<&str>;
}

impl StrIndex for Range<usize> {
    fn get(self, s: &str) -> Option<&str> {
        internal_get(s, self.start, self.end)
    }
}

impl StrIndex for RangeFull {
    fn get(self, s: &str) -> Option<&str> {
        internal_get(s, 0, s.len())
    }
}

impl StrIndex for RangeFrom<usize> {
    fn get(self, s: &str) -> Option<&str> {
        internal_get(s, self.start, s.len())
    }
}

impl StrIndex for RangeTo<usize> {
    fn get(self, s: &str) -> Option<&str> {
        internal_get(s, 0, self.end)
    }
}

impl StrIndex for RangeInclusive<usize> {
    fn get(self, s: &str) -> Option<&str> {
        internal_get(s, *self.start(), self.end().checked_add(1)?)
    }
}

impl StrIndex for RangeToInclusive<usize> {
    fn get(self, s: &str) -> Option<&str> {
        internal_get(s, 0, self.end.checked_add(1)?)
    }
}

fn is_char_boundary(s: &str, i: usize) -> bool {
    match s.as_bytes().get(i) {
        Some(&b) => b & 0xC0 != 0x80,
        None => i == s.len(),
    }
}

pub fn get<I: StrIndex>(s: &str, i: I) -> Option<&str> {
    i.get(s)
}

fn internal_get(s: &str, start: usize, end: usize) -> Option<&str> {
    if !(start <= end) {
        return None;
    }
    if !(end <= s.len()) {
        return None;
    }
    // both ends must sit on a char boundary, otherwise the slice would not be
    // valid UTF-8.
    if !is_char_boundary(s, start) || !is_char_boundary(s, end) {
        return None;
    }
    Some(&s[start..end])
}

#[cfg(test)]
mod tests {
    use super::*;

    const S: &str = "aé€😀";

    #[test]
    fn every_range_form_matches_std() {
        for a in 0..=S.len() + 1 {
            for b in 0..=S.len() + 1 {
                assert_eq!(get(S, a..b), S.get(a..b), "{a}..{b}");
                assert_eq!(get(S, a..=b), S.get(a..=b), "{a}..={b}");
            }
            assert_eq!(get(S, a..), S.get(a..), "{a}..");
            assert_eq!(get(S, ..a), S.get(..a), "..{a}");
            assert_eq!(get(S, ..=a), S.get(..=a), "..={a}");
        }
        assert_eq!(get(S, ..), S.get(..));
    }

    #[test]
    fn borrows_instead_of_copying() {
        let got = get(S, 3..6).unwrap();
        assert!(core::ptr::eq(got.as_ptr(), S[3..].as_ptr()));
    }

    #[test]
    fn an_inclusive_range_at_the_max_is_none_instead_of_overflowing() {
        assert_eq!(get(S, 0..=usize::MAX), None);
        assert_eq!(get(S, ..=usize::MAX), None);
    }
}
