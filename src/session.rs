use ffi;
use libc::{c_int, size_t};
use std::ffi::CString;
use std::{mem, ptr};

use Result;
use kind::Value;
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
pub struct Input {
    name: CString,
    tensor: Box<Flexor>,
}

/// An output.
pub struct Output {
    name: CString,
    tensor: Option<*mut ffi::TF_Tensor>,
}

/// A target.
pub struct Target {
    name: CString,
}

trait Flexor {
    fn into_raw(&mut self) -> *mut ffi::TF_Tensor;
}

impl Session {
    /// Create a session.
    pub fn new(options: Options) -> Result<Self> {
        let status = try!(Status::new());
        let raw = nonnull!(ffi!(TF_NewSession(options::as_raw(&options), status::as_raw(&status))),
                           &status);
        Ok(Session { options: options, status: status, raw: raw })
    }

    /// Extend the graph.
    pub fn extend<T>(&mut self, definition: T) -> Result<()> where T: AsRef<[u8]> {
        let data = definition.as_ref();
        ok!(ffi!(TF_ExtendGraph(self.raw, data.as_ptr() as *const _, data.len() as size_t,
                                status::as_raw(&self.status))),
            &self.status);
        Ok(())
    }

    /// Run the graph.
    pub fn run<'l>(&mut self, mut inputs: Vec<Input>, mut outputs: Vec<Output>,
                   targets: Vec<Target>) -> Result<Vec<Output>>
    {
        let ni = inputs.len();
        let mut input_names = vec![ptr::null(); ni];
        let mut input_tensors = vec![ptr::null_mut(); ni];
        for i in 0..ni {
            input_names[i] = inputs[i].name.as_ptr();
            input_tensors[i] = inputs[i].tensor.into_raw();
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
                        nt as c_int, ptr::null_mut(), status::as_raw(&self.status))),
            &self.status);

        for i in 0..no {
            outputs[i].set(output_tensors[i]);
        }

        Ok(outputs)
    }
}

impl Drop for Session {
    #[inline]
    fn drop(&mut self) {
        ffi!(TF_CloseSession(self.raw, status::as_raw(&self.status)));
        ffi!(TF_DeleteSession(self.raw, status::as_raw(&self.status)));
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

impl Output {
    /// Create an output.
    #[inline]
    pub fn new<T>(name: T) -> Self where T: Into<String> {
        Output {
            name: unsafe { CString::from_vec_unchecked(name.into().into()) },
            tensor: None,
        }
    }

    /// Convert into a tensor.
    pub fn into<T>(mut self) -> Result<Tensor<T>> where T: Value {
        match self.tensor.take() {
            Some(tensor) => tensor::from_raw(tensor),
            _ => raise!("the output has not been processed"),
        }
    }

    #[inline]
    fn set(&mut self, tensor: *mut ffi::TF_Tensor) {
        if let Some(tensor) = mem::replace(&mut self.tensor, Some(tensor)) {
            ffi!(TF_DeleteTensor(tensor));
        }
    }
}

impl Drop for Output {
    #[inline]
    fn drop(&mut self) {
        if let Some(tensor) = self.tensor.take() {
            ffi!(TF_DeleteTensor(tensor));
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
    fn into_raw(&mut self) -> *mut ffi::TF_Tensor {
        tensor::into_raw(self)
    }
}
