use crate::error::Result;
use alloc::vec::Vec;
use core::ffi::{CStr, c_char, c_int};

/// Raw CLI arguments.
///
/// After starting, it lives until the end of program execution
/// so the lifetime is static.
#[derive(Debug)]
pub struct Args<'a: 'static>(pub Vec<&'a str>);

impl Args<'_> {
    /// Creates Args from raw environment arguments.
    ///
    /// SAFETY:
    /// - `argc` must be the number of arguments.
    /// - `argv` must be a pointer to an array of `argc` pointers to C strings,
    ///   each of which must be valid.
    pub unsafe fn from_raw_env(argc: c_int, argv: *const *const c_char) -> Result<Self> {
        let mut buf: Vec<&str> = Vec::with_capacity(argc as usize);

        // SAFETY: TODO!()
        let args = unsafe { core::slice::from_raw_parts(argv, argc as usize) };
        for arg in args {
            // SAFETY: arg ptr has a valid nul terminator, so it's safe enough
            let arg = unsafe { CStr::from_ptr(*arg).to_str()? };
            buf.push(arg);
        }

        Ok(Args(buf))
    }
    
    pub fn from_str(_s: &'static str) -> Result<Self> {
       todo!()
    }
}
