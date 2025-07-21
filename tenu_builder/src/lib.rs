#![no_std]
#![deny(unused)]
#![forbid(unstable_features)]
#![forbid(clippy::undocumented_unsafe_blocks)]

mod arg;
mod args;

pub use crate::arg::*;
pub use crate::args::*;
