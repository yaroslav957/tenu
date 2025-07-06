#![no_std]
#![forbid(unstable_features)]
#![forbid(clippy::undocumented_unsafe_blocks)]

extern crate alloc;

mod cursor;
pub mod error;
pub mod lex;
pub mod parser;
