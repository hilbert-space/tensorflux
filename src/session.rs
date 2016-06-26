use ffi;
use libc::size_t;
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
    raw: *mut ffi::TF_Session,
}

/// An input.
#[allow(dead_code)]
pub struct Input {
    name: CString,
    tensor: Box<Flexor>,
}

/// An output.
#[allow(dead_code)]
pub struct Output {
    name: CString,
}

trait Flexor {}

impl Session {
    /// Create a session.
    pub fn new(options: Options) -> Result<Self> {
        let status = try!(Status::new());
        let raw = nonnull!(ffi!(TF_NewSession(options::raw(&options), status::raw(&status))),
                           &status);
        Ok(Session { options: options, status: status, raw: raw })
    }

    /// Extend the graph.
    pub fn extend<T>(&mut self, definition: T) -> Result<()> where T: AsRef<[u8]> {
        let data = definition.as_ref();
        ok!(ffi!(TF_ExtendGraph(self.raw, data.as_ptr() as *const _, data.len() as size_t,
                                status::raw(&self.status))),
            &self.status);
        Ok(())
    }

    /// Run the graph.
    pub fn run(&mut self, _: Vec<Input>, _: Vec<Output>) -> Result<()> {
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

impl Input {
    /// Create an input.
    #[inline]
    pub fn new<T, U>(name: T, tensor: Tensor<U>) -> Self where T: Into<String>, U: 'static {
        Input {
            name: unsafe { CString::from_vec_unchecked(name.into().into()) },
            tensor: Box::new(tensor),
        }
    }
}

impl Output {
    /// Create an output.
    #[inline]
    pub fn new<T>(name: T) -> Self where T: Into<String> {
        Output {
            name: unsafe { CString::from_vec_unchecked(name.into().into()) },
        }
    }
}

impl<T> Flexor for Tensor<T> {}
