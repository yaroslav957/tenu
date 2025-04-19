#![no_std]

extern crate alloc;

use crate::error::Result;
use alloc::vec::Vec;
use core::ffi::{CStr, c_char, c_int};

pub mod builder;
pub mod error;
pub mod parser;

#[derive(Debug)]
pub struct Arg<'a>(&'a str);

impl Arg<'_> {
    // `From` and 'TryFrom' from core::convert doesn't support type aliases (`c_char`)
    ///
    pub fn from_argv(arg: *const c_char) -> Result<Self> {
        // SAFETY: arg ptr has a valid nul terminator, so it's safe enough
        let arg = unsafe { CStr::from_ptr(arg).to_str()? };
        Ok(Arg(Arg::validate(arg)))
    }

    pub(crate) fn validate(arg: &str) -> &str {
        // TODO
        arg
    }
}

/// Raw CLI arguments.
///
/// After starting, it lives until the end of program execution
/// so the lifetime is static.
#[derive(Debug)]
pub struct Args<'a: 'static>(pub Vec<Arg<'a>>);

impl Args<'_> {
    /// Creates Args from raw environment arguments.
    ///
    /// SAFETY:
    /// - `argc` must be the number of arguments.
    /// - `argv` must be a pointer to an array of `argc` pointers to C strings,
    ///   each of which must be valid.
    pub unsafe fn from_raw_env(argc: c_int, argv: *const *const c_char) -> Result<Self> {
        let mut buf: Vec<Arg> = Vec::with_capacity(argc as usize);

        // SAFETY: TODO!()
        let args = unsafe { core::slice::from_raw_parts(argv, argc as usize) };
        for arg in args {
            let arg = Arg::from_argv(*arg)?;
            buf.push(arg);
        }

        Ok(Args(buf))
    }
}
