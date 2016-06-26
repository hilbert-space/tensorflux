use ffi;
use libc::size_t;

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
        let raw = nonnull!(ffi!(TF_NewSession(options::raw(&options), status::raw(&status))),
                           &status);
        Ok(Session { options: options, status: status, raw: raw })
    }

    /// Append the nodes of a graph.
    pub fn extend(&mut self, graph: &Graph) -> Result<()> {
        ok!(ffi!(TF_ExtendGraph(self.raw, graph.as_ptr() as *const _, graph.len() as size_t,
                                status::raw(&self.status))), &self.status);
        Ok(())
    }
}

impl Drop for Session {
    #[inline]
    fn drop(&mut self) {
        ffi!(TF_CloseSession(self.raw, status::raw(&self.status)));
        ffi!(TF_DeleteSession(self.raw, status::raw(&self.status)));
    }
}
