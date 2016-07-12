use ffi::TF_SessionOptions;
use libc::size_t;
use std::ffi::CString;

use Result;
use buffer::Buffer;
use status::Status;

/// Options.
pub struct Options {
    target: Option<CString>,
    raw: *mut TF_SessionOptions,
}

impl Options {
    /// Create options.
    pub fn new() -> Result<Self> {
        Ok(Options { target: None, raw: nonnull!(ffi!(TF_NewSessionOptions())) })
    }

    /// Set the configuration using a protocol buffer.
    ///
    /// The scheme of the `configuration` protocol buffer is called
    /// `ConfigProto`, and it can be found in TensorFlow’s [repository][1].
    ///
    /// [1]: https://github.com/tensorflow/tensorflow/blob/master/tensorflow/core/protobuf/config.proto
    pub fn configure(&mut self, configuration: &Buffer) -> Result<()> {
        let status = try!(Status::new());
        let configuration = configuration.as_ref();
        ok!(ffi!(TF_SetConfig(self.raw, configuration.as_ptr() as *const _,
                              configuration.len() as size_t, status.as_raw())), &status);
        Ok(())
    }

    /// Set the target.
    pub fn target<T>(&mut self, target: T) where T: Into<String> {
        let target = into_cstring!(target);
        ffi!(TF_SetTarget(self.raw, target.as_ptr()));
        self.target = Some(target);
    }

    #[doc(hidden)]
    #[inline]
    pub fn as_raw(&self) -> *mut TF_SessionOptions {
        self.raw
    }
}

impl Drop for Options {
    #[inline]
    fn drop(&mut self) {
        ffi!(TF_DeleteSessionOptions(self.raw));
    }
}
