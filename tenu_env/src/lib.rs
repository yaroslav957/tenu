mod arg;
pub use crate::arg::*;

use core::{
    ffi::{c_char, c_int},
    ptr, slice,
};

static mut ARGC: usize = 0;
static mut ARGV: *const Arg = ptr::null_mut();

#[used]
// check some errors on libc::musl
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]
#[cfg_attr(not(target_os = "macos"), unsafe(link_section = ".init_array"))]
static __: unsafe extern "C" fn(c_int, *const *const c_char) = {
    // SAFETY:
    unsafe extern "C" fn capture(argc: c_int, argv: *const *const c_char) {
        unsafe {
            ARGC = argc as usize;
            ARGV = argv as *const Arg;
        }
    }
    capture
};

#[derive(Debug)]
pub struct Env {
    args: &'static [Arg],
}

impl Env {
    pub fn new() -> Self {
        let args = unsafe { slice::from_raw_parts(ARGV, ARGC as usize) };
        Self { args }
    }
}
