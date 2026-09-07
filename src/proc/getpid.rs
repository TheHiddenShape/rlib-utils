/// returns the process ID of the calling process.
///
/// wraps `getpid(2)`. the call crosses the FFI boundary, so it is `unsafe`,
/// but the wrapper is sound: nothing is dereferenced and the call cannot
/// fail, so no invariant is left to the caller.
pub fn getpid() -> libc::pid_t {
    todo!()
}
