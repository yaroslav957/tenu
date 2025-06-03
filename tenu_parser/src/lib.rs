// ебись тут сам теперь, свои линты ставь
// я бы на твоем месте фулл без ансейфа делал

#![no_std]
#![forbid(unstable_features)]
#![forbid(clippy::undocumented_unsafe_blocks)]

extern crate alloc;

pub mod error;
pub mod lex;
pub mod parser;
