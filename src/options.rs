use ffi;
use libc::size_t;

use Result;
use status::{self, Status};

/// Options.
pub struct Options {
    status: Status,
    raw: *mut ffi::TF_SessionOptions,
}

impl Options {
    /// Create options.
    pub fn new() -> Result<Self> {
        Ok(Options {
            status: try!(Status::new()),
            raw: nonnull!(ffi!(TF_NewSessionOptions())),
        })
    }

    /// Configure using a protocol buffer.
    pub fn configure<T>(&mut self, buffer: T) -> Result<()> where T: AsRef<[u8]> {
        let buffer = buffer.as_ref();
        ok!(ffi!(TF_SetConfig(self.raw, buffer.as_ptr() as *const _, buffer.len() as size_t,
                              status::as_raw(&self.status))),
            &self.status);
        Ok(())
    }
}

impl Drop for Options {
    #[inline]
    fn drop(&mut self) {
        ffi!(TF_DeleteSessionOptions(self.raw));
    }
}

#[inline(always)]
pub fn as_raw(options: &Options) -> *mut ffi::TF_SessionOptions {
    options.raw
}
