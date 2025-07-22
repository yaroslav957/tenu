#![no_std]
#![deny(unused)]
#![allow(unused_macros)]
#![forbid(unstable_features)]
#![forbid(clippy::undocumented_unsafe_blocks)]

//macro_rules! exit {}
//macro_rules! entry {}

mod arg;
mod args;

pub use crate::arg::*;
pub use crate::args::*;
