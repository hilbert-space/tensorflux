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
    status: Status,
    raw: *mut ffi::TF_Session,
}

/// An input.
pub struct Input {
    name: CString,
    tensor: Option<Box<Flexor>>,
}

/// An output.
pub struct Output {
    name: CString,
    tensor: Option<*mut ffi::TF_Tensor>,
}

trait Flexor {
    fn into_raw(&mut self) -> *mut ffi::TF_Tensor;
}

impl Session {
    /// Create a session.
    pub fn new(options: &Options) -> Result<Self> {
        let status = try!(Status::new());
        let raw = nonnull!(ffi!(TF_NewSession(options::as_raw(options), status::as_raw(&status))),
                           &status);
        Ok(Session { status: status, raw: raw })
    }

    /// Extend the graph using a protocol buffer.
    ///
    /// The schema of the protocol buffer is called GraphDef, and it can be
    /// found in TensorFlow’s [repository][1]. An example of creating a graph
    /// definition is given in the [main description][2] of this package.
    ///
    /// [1]: https://github.com/tensorflow/tensorflow/blob/master/tensorflow/core/framework/graph.proto
    /// [2]: index.html#example
    pub fn extend<T>(&mut self, buffer: T) -> Result<()> where T: AsRef<[u8]> {
        let buffer = buffer.as_ref();
        ok!(ffi!(TF_ExtendGraph(self.raw, buffer.as_ptr() as *const _, buffer.len() as size_t,
                                status::as_raw(&self.status))),
            &self.status);
        Ok(())
    }

    /// Run the graph.
    pub fn run<'l>(&mut self, inputs: &mut [Input], outputs: &mut [Output]) -> Result<()> {
        let ni = inputs.len();
        let mut input_names = vec![ptr::null(); ni];
        let mut input_tensors = vec![ptr::null_mut(); ni];
        let mut input_garbage = Vec::with_capacity(ni);
        for i in 0..ni {
            input_names[i] = inputs[i].name.as_ptr();
            match inputs[i].tensor.take() {
                Some(mut tensor) => {
                    input_tensors[i] = tensor.into_raw();
                    input_garbage.push(tensor);
                },
                _ => raise!("some of the inputs have not been set"),
            }
        }

        let no = outputs.len();
        let mut output_names = vec![ptr::null(); no];
        let mut output_tensors = vec![ptr::null_mut(); no];
        for i in 0..no {
            output_names[i] = outputs[i].name.as_ptr();
        }

        let mut target_names = vec![];

        ok!(ffi!(TF_Run(self.raw, ptr::null(), input_names.as_mut_ptr(),
                        input_tensors.as_mut_ptr(), ni as c_int, output_names.as_mut_ptr(),
                        output_tensors.as_mut_ptr(), no as c_int, target_names.as_mut_ptr(),
                        0, ptr::null_mut(), status::as_raw(&self.status))),
            &self.status);

        for i in 0..no {
            outputs[i].set(output_tensors[i]);
        }

        Ok(())
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
    pub fn new<T>(name: T) -> Self where T: Into<String> {
        Input { name: into_cstring!(name), tensor: None }
    }

    /// Assign a tensor.
    pub fn set<T>(&mut self, tensor: Tensor<T>) where T: Value {
        self.tensor = Some(Box::new(tensor));
    }
}

impl Output {
    /// Create an output.
    #[inline]
    pub fn new<T>(name: T) -> Self where T: Into<String> {
        Output { name: into_cstring!(name), tensor: None }
    }

    /// Extract the tensor.
    pub fn get<T>(&mut self) -> Result<Tensor<T>> where T: Value {
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

impl<T> Flexor for Tensor<T> {
    #[inline(always)]
    fn into_raw(&mut self) -> *mut ffi::TF_Tensor {
        tensor::into_raw(self)
    }
}
