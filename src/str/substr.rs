/// returns the `len` bytes of `s` starting at `start`.
///
/// C has to `malloc` a copy, since a pointer into the middle of a string
/// carries neither its length nor any proof that the source outlives it. the
/// slice carries the first, the lifetime the second, so this borrows instead.
pub fn substr(s: &[u8], start: usize, len: usize) -> &[u8] {
    todo!()
}
