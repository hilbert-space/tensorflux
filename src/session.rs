use ffi::{TF_DataType, TF_Session, TF_Tensor};
use libc::{c_int, size_t};
use std::ffi::CString;
use std::{mem, ptr};

use Result;
use buffer::{self, Buffer};
use options::{self, Options};
use status::{self, Status};
use tensor::{self, Tensor};
use value::Value;

/// A session.
pub struct Session {
    status: Status,
    raw: *mut TF_Session,
}

/// An input.
pub struct Input {
    name: CString,
    tensor: Option<Box<Flexor>>,
}

/// An output.
pub struct Output {
    name: CString,
    tensor: Option<*mut TF_Tensor>,
}

/// A target.
pub struct Target {
    name: CString,
}

trait Flexor {
    fn copy_raw(&self) -> Result<*mut TF_Tensor>;
    fn kind(&self) -> TF_DataType;
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
    /// The schema of the `definition` protocol buffer is called `GraphDef`, and
    /// it can be found in TensorFlow’s [repository][1]. An example of creating
    /// a graph definition is given in the [main description][2] of this
    /// package.
    ///
    /// [1]: https://github.com/tensorflow/tensorflow/blob/master/tensorflow/core/framework/graph.proto
    /// [2]: index.html#example
    pub fn extend(&mut self, definition: &Buffer) -> Result<()> {
        let definition = definition.as_ref();
        ok!(ffi!(TF_ExtendGraph(self.raw, definition.as_ptr() as *const _,
                                definition.len() as size_t, status::as_raw(&self.status))),
            &self.status);
        Ok(())
    }

    /// Run the graph.
    ///
    /// The schemas of the `options` and `metadata` protocol buffers are called
    /// `RunOptions` and `RunMetadata`, respectively, and they can be found in
    /// TensorFlow’s [repository][1].
    ///
    /// [1]: https://github.com/tensorflow/tensorflow/blob/master/tensorflow/core/protobuf/config.proto
    pub fn run(&mut self, inputs: &[Input], outputs: &mut [Output], targets: &[Target],
               options: Option<&Buffer>, metadata: Option<&mut Buffer>) -> Result<()> {

        let ni = inputs.len();
        let mut input_names = vec![ptr::null(); ni];
        let mut input_tensors = vec![ptr::null_mut(); ni];

        macro_rules! cleanup(() => ({
            for tensor in input_tensors.drain(..) {
                ffi!(TF_DeleteTensor(tensor));
            }
        }));

        for i in 0..ni {
            input_names[i] = inputs[i].name.as_ptr();
            input_tensors[i] = match inputs[i].tensor.as_ref().map(|tensor| tensor.copy_raw()) {
                Some(Ok(tensor)) => tensor,
                Some(Err(error)) => {
                    cleanup!();
                    return Err(error);
                },
                _ => {
                    cleanup!();
                    raise!("some of the inputs have not been set");
                },
            };
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

        let options_buffer = if let Some(buffer) = options {
            buffer::as_raw(buffer)
        } else {
            ptr::null_mut()
        };

        let metadata_buffer = if let Some(ref buffer) = metadata {
            buffer::as_raw(buffer)
        } else {
            ptr::null_mut()
        };

        ok!(ffi!(TF_Run(self.raw, options_buffer, input_names.as_mut_ptr(),
                        input_tensors.as_mut_ptr(), ni as c_int, output_names.as_mut_ptr(),
                        output_tensors.as_mut_ptr(), no as c_int, target_names.as_mut_ptr(),
                        nt as c_int, metadata_buffer, status::as_raw(&self.status))),
            &self.status);

        for i in 0..no {
            outputs[i].set(output_tensors[i]);
        }

        if let Some(buffer) = metadata {
            buffer::reset(buffer);
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
    pub fn new<T, U>(name: T, tensor: Tensor<U>) -> Self where T: Into<String>, U: Value {
        Input { name: into_cstring!(name), tensor: Some(Box::new(tensor)) }
    }

    /// Extract the tensor.
    pub fn get<T>(&mut self) -> Result<Tensor<T>> where T: Value {
        if self.tensor.is_none() {
            raise!("the tensor has not been set");
        }
        if self.tensor.as_ref().unwrap().kind() != T::kind() {
            raise!("the data types do not match");
        }
        let tensor = self.tensor.take().unwrap();
        Ok(*unsafe { Box::from_raw(Box::into_raw(tensor) as *mut _) })
    }

    /// Assign a tensor.
    #[inline]
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
            _ => raise!("the tensor has not been set"),
        }
    }

    #[inline]
    fn set(&mut self, tensor: *mut TF_Tensor) {
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
        Target { name: into_cstring!(name) }
    }
}

impl<T> Flexor for Tensor<T> where T: Value {
    #[inline(always)]
    fn copy_raw(&self) -> Result<*mut TF_Tensor> {
        tensor::copy_raw(self)
    }

    #[inline(always)]
    fn kind(&self) -> TF_DataType {
        T::kind()
    }
}

#[cfg(test)]
mod tests {
    use session::Input;
    use tensor::Tensor;

    #[test]
    fn input_get() {
        let a = Tensor::new(vec![42.0, 69.0], &[2]).unwrap();
        let mut a = Input::new("a", a);
        let a = a.get::<f64>().unwrap();
        assert_eq!(&a[..], &[42.0, 69.0]);
    }
}
