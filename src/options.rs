use ffi;
use libc::size_t;
use std::ffi::CString;
use std::mem;

use Result;
use status::{self, Status};

/// Options.
pub struct Options {
    status: Status,
    target: Option<CString>,
    raw: *mut ffi::TF_SessionOptions,
}

impl Options {
    /// Create options.
    pub fn new() -> Result<Self> {
        Ok(Options {
            status: try!(Status::new()),
            target: None,
            raw: nonnull!(ffi!(TF_NewSessionOptions())),
        })
    }

    /// Configure using a protocol buffer.
    ///
    /// The scheme of the `configuration` protocol buffer is called
    /// `ConfigProto`, and it can be found in TensorFlow’s [repository][1].
    ///
    /// [1]: https://github.com/tensorflow/tensorflow/blob/master/tensorflow/core/protobuf/config.proto
    pub fn configure<T>(&mut self, configuration: T) -> Result<()> where T: AsRef<[u8]> {
        let configuration = configuration.as_ref();
        ok!(ffi!(TF_SetConfig(self.raw, configuration.as_ptr() as *const _,
                              configuration.len() as size_t, status::as_raw(&self.status))),
            &self.status);
        Ok(())
    }

    /// Set the target.
    pub fn target<T>(&mut self, target: T) where T: Into<String> {
        let target = into_cstring!(target);
        ffi!(TF_SetTarget(self.raw, target.as_ptr()));
        mem::replace(&mut self.target, Some(target));
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
