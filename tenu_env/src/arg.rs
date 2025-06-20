use core::str::Utf8Error;

#[derive(Debug)]
#[repr(transparent)]
pub struct Arg {
    pub(crate) inner: *const u8,
}

impl Arg {
    /// SAFETY:
    pub(crate) unsafe fn from_ptr(ptr: *const u8) -> Self {
        Self { inner: ptr }
    }

    ///
    pub fn as_bytes(&self) -> &[u8] {
        // SAFETY:
        unsafe { core::slice::from_raw_parts(self.inner, self.len()) }
    }

    ///
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        core::str::from_utf8(self.as_bytes())
    }

    ///
    pub fn len(&self) -> usize {
        let ptr = self.inner;
        let mut idx = 0;

        // SAFETY:
        while unsafe { *ptr.add(idx) } != 0 {
            idx += 1;
        }

        idx
    }
}
