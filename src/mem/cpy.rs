/// copies every byte of `src` into `dst`.
///
/// the `size_t n` of C is gone: the length is carried by the slices, and a
/// mismatch panics instead of overflowing. the regions must not overlap.
pub fn memcpy(dst: &mut [u8], src: &[u8]) {
    todo!()
}
