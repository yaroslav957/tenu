use core::ptr;

unsafe extern "C" fn capture(argc: i32, argv: *const *const u8) {
    unsafe {
        ARGC = argc;
        ARGV = argv;
    }
}

static mut ARGC: i32 = 0;
static mut ARGV: *const *const u8 = ptr::null();

#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]
#[cfg_attr(not(target_os = "macos"), unsafe(link_section = ".init_array"))]
#[used]
static CAPTURE: unsafe extern "C" fn(i32, *const *const u8) = capture;

fn main() {
    println!(
        "timofeika archipov,
        Moscow city,
        rayon lyublino,
        ylutsya Kadyrova,
        dom 13A,
        podyezd 2,
        kvartira 221"
    );
}
