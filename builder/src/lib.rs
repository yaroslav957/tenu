#![no_std]
#![forbid(unused)]
#![forbid(unstable_features)]
#![forbid(clippy::undocumented_unsafe_blocks)]

#[macro_export]
macro_rules! entry {
    (
        $(#[$meta:meta])*
        pub fn main($args:ident: Args) -> i32 $body:block
    ) => {
        $(#[$meta])*
        pub fn main($args: Args) -> i32 {
            $body
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn entry(stack: *const u8) -> i32 {
            //
            //SAFETY:
            let argc = unsafe { *(stack) as usize };
            //SAFETY:
            let argv = unsafe { stack.offset(8) as *const Arg };
            //SAFETY:
            let args = unsafe { Args::from_raw(argc, argv) };

            main(args)
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn _start() {
            #[cfg(target_arch = "x86_64")]
            unsafe {
                ::core::arch::asm!(
                    "lea rdi, [rsp + 8]",
                    "call entry",
                    "mov rdi, rax",
                    "mov rax, 60",
                    "syscall",
                    options(noreturn)
                )
            }

            #[cfg(target_arch = "aarch64")]
            unsafe {
                ::core::arch::asm!(
                    "mov x0, sp",
                    "bl entry",
                    "mov x8, 93",
                    "svc #0",
                    options(noreturn)
                )
            }
        }
    }
}

mod arg;
mod args;

pub use crate::arg::*;
pub use crate::args::*;
