use crate::error::Result;
use alloc::vec::Vec;
use core::ffi::{CStr, c_char, c_int};

/// Raw CLI arguments.
///
/// After starting, it lives until the end of program execution.
#[derive(Debug)]
pub struct Args(pub Vec<&'static str>);

impl Args {
    /// Creates Args from raw stack pointer.
    ///
    /// SAFETY:
    /// - `stack_top` MUST be a valid pointer to the top of stack
    pub unsafe fn from_raw_stack_top(stack_top: *const u8) -> Result<Self> {
        // SAFETY:
        // ,-------------------,
        // |       argc        | <-- stack top
        // !-------------------!
        // |       argv        | <-- *stack_top.offset(8)
        // !-------------------!
        // |       envp        |
        // !-------------------!
        // |   auxiliary vec   |
        // !-------------------!
        // |        ...        |
        // '-------------------'
        let (argc, argv) = unsafe {
            (
                *(stack_top as *const c_int),
                stack_top.offset(8) as *const *const c_char,
            )
        };
        unsafe { Args::from_raw_env(argc, argv) }
    }

    /// Creates Args from raw environment arguments.
    ///
    /// SAFETY:
    /// - `argc` MUST be the number of arguments.
    /// - `argv` MUST be a pointer to an array of `argc` pointers to CStr,
    ///   Each pointer in the array MUST be a null-terminated CStr.
    ///
    /// Given these preconditions, this function safely converts arguments into `Vec<&str>`.
    /// If any CStr is not valid UTF-8, the function safely returns an error.
    pub unsafe fn from_raw_env(argc: c_int, argv: *const *const c_char) -> Result<Self> {
        let mut buf: Vec<&str> = Vec::with_capacity(argc as usize);

        // SAFETY: preconditions guarantee the validity of `argc` (with correct length conversion) and `argv`
        let args = unsafe { core::slice::from_raw_parts(argv, argc as usize) };
        for arg in args {
            // SAFETY: arg ptr has a valid null terminator, so it's safe enough
            let arg = unsafe { CStr::from_ptr(*arg).to_str()? };
            buf.push(arg);
        }

        Ok(Args(buf))
    }

    pub fn from_str(string: &'static str) -> Self {
        Args(string.split_whitespace().collect())
    }

    pub fn from_vec(vec: Vec<&'static str>) -> Self {
        Args(vec)
    }
}
