use ffi;

use result::Result;

/// Session options.
pub struct Options {
    raw: *mut ffi::TF_SessionOptions,
}

impl Options {
    /// Create session options.
    pub fn new() -> Result<Options> {
        Ok(Options { raw: nonnull!(unsafe { ffi::TF_NewSessionOptions() }) })
    }
}

impl Drop for Options {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::TF_DeleteSessionOptions(self.raw) };
    }
}

#[inline(always)]
pub fn raw(options: &Options) -> *mut ffi::TF_SessionOptions {
    options.raw
}
