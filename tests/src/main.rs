use core::ptr;
use rncp::{prelude::RawArgs, *};
use std::os::raw::c_char;

unsafe extern "C" fn capture(argc: i32, argv: *const *const c_char) {
    unsafe {
        ARGC = argc;
        ARGV = argv;
    }
}

static mut ARGC: i32 = 0;
static mut ARGV: *const *const c_char = ptr::null();

#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]
#[cfg_attr(not(target_os = "macos"), unsafe(link_section = ".init_array"))]
#[used]
static CAPTURE: unsafe extern "C" fn(i32, *const *const c_char) = capture;

fn main() {
    let args = unsafe { RawArgs::from_raw_env(ARGC, ARGV).unwrap() };
    let foo = builder::ArgBuilder::new("foo")
        .with_value(&parser::IntParser)
        .short()
        .get(&args);

    // Run with -- foo 52 as an arguments and see what's about to happen (segfault) \j
    println!("{foo:?}");
}
