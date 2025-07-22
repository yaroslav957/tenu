#![no_std]
#![no_main]

use core::panic::PanicInfo;

use tenu::{Arg, Args, entry};

entry! {
    pub fn main(args: Args) -> i32 {
        0
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
