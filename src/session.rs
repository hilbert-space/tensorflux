use ffi;
use libc::size_t;
use std::collections::HashMap;
use std::ffi::CString;

use Result;
use options::{self, Options};
use status::{self, Status};
use tensor::Tensor;

/// A session.
pub struct Session {
    #[allow(dead_code)]
    options: Options,
    status: Status,
    inputs: HashMap<CString, Box<Flexor>>,
    raw: *mut ffi::TF_Session,
}

trait Flexor {}

impl Session {
    /// Create a session.
    pub fn new(options: Options) -> Result<Self> {
        let status = try!(Status::new());
        let raw = nonnull!(ffi!(TF_NewSession(options::raw(&options), status::raw(&status))),
                           &status);
        Ok(Session {
            options: options,
            status: status,
            inputs: HashMap::new(),
            raw: raw,
        })
    }

    /// Extend the graph.
    pub fn extend<T>(&mut self, definition: T) -> Result<()> where T: AsRef<[u8]> {
        let data = definition.as_ref();
        ok!(ffi!(TF_ExtendGraph(self.raw, data.as_ptr() as *const _, data.len() as size_t,
                                status::raw(&self.status))),
            &self.status);
        Ok(())
    }

    /// Set an input.
    pub fn input<N, T>(&mut self, name: N, tensor: Tensor<T>) -> Result<()>
        where N: Into<Vec<u8>>, T: 'static
    {
        self.inputs.insert(ok!(CString::new(name)), Box::new(tensor));
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

impl<T> Flexor for Tensor<T> {}
