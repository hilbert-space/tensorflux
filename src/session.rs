use ffi;

use graph::Graph;
use options::{self, Options};
use result::Result;
use status::{self, Status};

/// A session.
pub struct Session {
    #[allow(dead_code)]
    options: Options,
    status: Status,
    raw: *mut ffi::TF_Session,
}

impl Session {
    /// Create a session.
    pub fn new(options: Options) -> Result<Self> {
        let status = try!(Status::new());
        let raw = unsafe { ffi::TF_NewSession(options::raw(&options),
                                              status::raw(&status)) };
        let raw = nonnull!(raw, &status);
        Ok(Session { options: options, status: status, raw: raw })
    }

    /// Append a graph.
    pub fn append(&mut self, _: Graph) -> Result<()> {
        Ok(())
    }
}

impl Drop for Session {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::TF_CloseSession(self.raw, status::raw(&self.status));
            ffi::TF_DeleteSession(self.raw, status::raw(&self.status));
        }
    }
}
