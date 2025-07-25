use crate::Arg;

/// A wrapper around a slice of `Arg` instances.
#[derive(Clone, Copy)]
pub struct Args {
    inner: &'static [Arg],
}

impl Args {
    /// Creates a new `Args` instance from a argc/argv,
    /// ps: cast argv (`*const *const u8`) as `*const Arg`.
    ///
    /// # Safety:
    /// The caller must ensure that `argv` points to a valid array of `argc`
    /// null-terminated byte sequences, and that the memory remains valid
    /// for the `'static` lifetime.
    pub unsafe fn from_raw(argc: usize, argv: *const Arg) -> Self {
        // SAFETY:
        // guarantees above
        let inner = unsafe { core::slice::from_raw_parts(argv, argc) };

        Self { inner }
    }
}

impl Iterator for Args {
    type Item = &'static str;

    /// Advances the iterator and returns the next string in the collection.
    fn next(&mut self) -> Option<Self::Item> {
        let [curr, rest @ ..] = self.inner else {
            return None;
        };

        self.inner = rest;

        Some(curr.as_str().unwrap_or_default())
    }
}
