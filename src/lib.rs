//! a UNIX-like system library written in Rust.
//!
//! `no_std` by default; the `not(test)` guard keeps the unit tests working,
//! since their harness links `std`.
#![cfg_attr(not(test), no_std)]

pub mod mem;
pub mod proc;
pub mod str;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
