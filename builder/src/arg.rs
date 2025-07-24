use core::str::Utf8Error;

/// Wrapper around ptr to a null-terminated byte sequence.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Arg {
    inner: *const u8,
}

impl Arg {
    /// Calculates the length of the byte sequence
    /// by counting bytes until the null terminator.
    pub fn len(&self) -> usize {
        let ptr = self.inner;
        let mut idx = 0;

        // SAFETY:
        // Dereferencing `ptr.add(idx)` is safe only if `inner` points
        // to a valid, null-terminated sequence. Iteration stops at the null-byte.
        while unsafe { *ptr.add(idx) } != 0 {
            idx += 1;
        }

        idx
    }

    /// Checks the length of the byte sequence
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a byte slice of the underlying data,
    /// until the null-terminator.
    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY:
        // Relies on `inner` being valid and `len()` correctly computing the length
        // to the null-terminator. The slice is constructed only up to the null-byte.
        unsafe { core::slice::from_raw_parts(self.inner, self.len()) }
    }

    /// Try to onvert slice from `as_bytes` into a UTF-8 &str
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        core::str::from_utf8(self.as_bytes())
    }
}
