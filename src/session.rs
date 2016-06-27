use ffi;
use libc::{c_int, size_t};
use std::ffi::CString;
use std::ptr;

use Result;
use options::{self, Options};
use status::{self, Status};
use tensor::{self, Tensor};

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
pub struct Output<'l> {
    name: CString,
    tensor: &'l mut Flexor,
}

/// A target.
#[allow(dead_code)]
pub struct Target {
    name: CString,
}

trait Flexor {
    fn unwrap(&mut self) -> *mut ffi::TF_Tensor;
}

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
    pub fn run<'l>(&mut self, mut inputs: Vec<Input>, outputs: Vec<Output<'l>>,
                   targets: Vec<Target>) -> Result<()>
    {
        let ni = inputs.len();
        let mut input_names = vec![ptr::null(); ni];
        let mut input_tensors = vec![ptr::null_mut(); ni];
        for i in 0..ni {
            input_names[i] = inputs[i].name.as_ptr();
            input_tensors[i] = inputs[i].tensor.unwrap();
        }

        let no = outputs.len();
        let mut output_names = vec![ptr::null(); no];
        let mut output_tensors = vec![ptr::null_mut(); no];
        for i in 0..no {
            output_names[i] = outputs[i].name.as_ptr();
        }

        let nt = targets.len();
        let mut target_names = vec![ptr::null(); nt];
        for i in 0..nt {
            target_names[i] = targets[i].name.as_ptr();
        }

        ok!(ffi!(TF_Run(self.raw, ptr::null(), input_names.as_mut_ptr(),
                        input_tensors.as_mut_ptr(), ni as c_int, output_names.as_mut_ptr(),
                        output_tensors.as_mut_ptr(), no as c_int, target_names.as_mut_ptr(),
                        nt as c_int, ptr::null_mut(), status::raw(&self.status))),
            &self.status);

        for i in 0..no {
            ffi!(TF_DeleteTensor(output_tensors[i]));
        }

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
    pub fn new<T, U>(name: T, tensor: Tensor<U>) -> Self
        where T: Into<String>, U: 'static
    {
        Input {
            name: unsafe { CString::from_vec_unchecked(name.into().into()) },
            tensor: Box::new(tensor),
        }
    }
}

impl<'l> Output<'l> {
    /// Create an output.
    #[inline]
    pub fn new<T, U>(name: T, tensor: &'l mut Tensor<U>) -> Self
        where T: Into<String>, U: 'static
    {
        Output {
            name: unsafe { CString::from_vec_unchecked(name.into().into()) },
            tensor: tensor,
        }
    }
}

impl Target {
    /// Create a target.
    #[inline]
    pub fn new<T>(name: T) -> Self where T: Into<String> {
        Target {
            name: unsafe { CString::from_vec_unchecked(name.into().into()) },
        }
    }
}

impl<T> Flexor for Tensor<T> {
    #[inline(always)]
    fn unwrap(&mut self) -> *mut ffi::TF_Tensor {
        tensor::unwrap(self)
    }
}
