use core::ffi::{c_char, c_int};

use crate::Arg;

pub struct Args {
    inner: &'static [Arg],
}
impl Args {
    pub(crate) fn from_raw(argc: c_int, argv: *const *const c_char) -> Self {
        let argc = argc as usize;
        let argv = argv as *const Arg;

        let inner = unsafe { core::slice::from_raw_parts(argv, argc) };

        Self { inner }
    }
}

impl Iterator for Args {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let [curr, rest @ ..] = self.inner else {
            return None;
        };

        self.inner = rest;

        Some(curr.as_str().unwrap_or_default())
    }
}
