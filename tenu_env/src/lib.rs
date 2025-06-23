#![no_std]
#![deny(unused)]
#![forbid(unstable_features)]
#![forbid(clippy::undocumented_unsafe_blocks)]

mod arg;
mod env;

pub use crate::arg::*;
pub use crate::env::*;
