use crate::Arg;

///
pub struct Args {
    inner: &'static [Arg],
}

impl Args {
    ///
    pub fn from_raw(argc: usize, argv: *const Arg) -> Self {
        //SAFETY:
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
