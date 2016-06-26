use ffi;

use Result;

/// Options.
pub struct Options {
    raw: *mut ffi::TF_SessionOptions,
}

impl Options {
    /// Create options.
    pub fn new() -> Result<Self> {
        Ok(Options { raw: nonnull!(ffi!(TF_NewSessionOptions())) })
    }
}

impl Drop for Options {
    #[inline]
    fn drop(&mut self) {
        ffi!(TF_DeleteSessionOptions(self.raw));
    }
}

#[inline(always)]
pub fn raw(options: &Options) -> *mut ffi::TF_SessionOptions {
    options.raw
}
