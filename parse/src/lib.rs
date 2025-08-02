#![no_std]
#![forbid(unstable_features)]
#![forbid(clippy::undocumented_unsafe_blocks)]

extern crate alloc;

pub mod error;
pub mod lex;
pub mod parser;
