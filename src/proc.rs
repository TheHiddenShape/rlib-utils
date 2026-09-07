//! process control, thin wrappers over `libc`.

mod getpid;

pub use getpid::getpid;
