use ffi::TF_Tensor;
use libc::{c_int, c_void, int64_t, size_t};
use std::ptr;

use Result;
use memory::Memory;
use value::Value;

/// A tensor.
pub struct Tensor<T> {
    dimensions: Vec<int64_t>,
    memory: Memory<T>,
    raw: *mut TF_Tensor,
}

impl<T> Tensor<T> where T: Value {
    /// Create a tensor.
    pub fn new(data: Vec<T>, dimensions: &[usize]) -> Result<Self> {
        let (given, needed) = (data.len(), dimensions.iter().fold(1, |p, &d| p * d));
        if needed > given {
            raise!("there should be at least {} data point(s)", needed);
        }
        let dimensions = dimensions.iter().map(|&d| d as int64_t).collect::<Vec<_>>();
        let memory = Memory::new(data);
        let raw = nonnull!(ffi!(TF_NewTensor(T::kind(), dimensions.as_ptr(),
                                dimensions.len() as c_int, memory.as_ptr() as *mut _,
                                needed as size_t, Some(noop), ptr::null_mut())));
        Ok(Tensor { dimensions: dimensions, memory: memory, raw: raw })
    }

    /// Return the dimensions.
    pub fn dimensions(&self) -> Vec<usize> {
        self.dimensions.iter().map(|&d| d as usize).collect()
    }

    #[doc(hidden)]
    pub fn copy_raw(&self) -> Result<*mut TF_Tensor> {
        Ok(nonnull!(ffi!(TF_NewTensor(T::kind(), self.dimensions.as_ptr(),
                         self.dimensions.len() as c_int, self.as_ptr() as *mut _,
                         self.len() as size_t, Some(noop), ptr::null_mut()))))
    }

    #[doc(hidden)]
    pub fn from_raw(raw: *mut TF_Tensor) -> Result<Self> {
        if ffi!(TF_TensorType(raw)) != T::kind() {
            raise!("the data types do not match");
        }
        let pointer = nonnull!(ffi!(TF_TensorData(raw))) as *mut _;
        let dimensions = (0..ffi!(TF_NumDims(raw))).map(|i| ffi!(TF_Dim(raw, i)))
                                                   .collect::<Vec<_>>();
        let length = if dimensions.is_empty() { 0 } else {
            dimensions.iter().fold(1, |p, &d| p * d as usize)
        };
        let memory = unsafe { Memory::from_raw_parts(pointer, length) };
        Ok(Tensor { dimensions: dimensions, memory: memory, raw: raw })
    }
}

memory!(Tensor<T>);

impl<T> Drop for Tensor<T> {
    #[inline]
    fn drop(&mut self) {
        ffi!(TF_DeleteTensor(self.raw));
    }
}

unsafe extern "C" fn noop(_: *mut c_void, _: size_t, _: *mut c_void) {}
