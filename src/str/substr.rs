/// returns the `len` bytes of `s` starting at `start`, or `None` if that range leaves the slice.
///
/// unlike `&s[start..start + len]`, a range outside the slice is a `None` for
/// the caller to handle rather than a panic.
///
/// guards against integer overflow on the slice indices.
pub fn substr(s: &[u8], start: usize, len: usize) -> Option<&[u8]> {
    let end = start.checked_add(len)?;
    s.get(start..end)
}

#[cfg(test)]
mod tests {
    use super::*;

    const S: &[u8] = b"hello, world";

    #[test]
    fn takes_a_range_from_the_middle() {
        assert_eq!(substr(S, 7, 5), Some(&b"world"[..]));
    }

    #[test]
    fn borrows_instead_of_copying() {
        let got = substr(S, 7, 5).unwrap();
        assert!(core::ptr::eq(got.as_ptr(), S[7..].as_ptr()));
    }

    #[test]
    fn an_empty_range_at_the_end_is_not_an_error() {
        assert_eq!(substr(S, S.len(), 0), Some(&b""[..]));
    }

    #[test]
    fn a_range_past_the_end_is_none() {
        assert_eq!(substr(S, 0, S.len() + 1), None);
        assert_eq!(substr(S, S.len() + 1, 0), None);
    }

    #[test]
    fn a_length_that_would_overflow_is_none() {
        assert_eq!(substr(S, 1, usize::MAX), None);
    }
}
