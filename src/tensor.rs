use ffi;
use libc::{c_int, c_longlong, c_void, size_t};
use std::ptr;

use Result;
use kind::{Type, Value};
use memory::Memory;

/// A tensor.
pub struct Tensor<T> {
    dimensions: Vec<c_longlong>,
    memory: Memory<T>,
    raw: *mut ffi::TF_Tensor,
}

impl<T> Tensor<T> where T: Value {
    /// Create a tensor.
    pub fn new(mut data: Vec<T>, dimensions: &[usize]) -> Result<Self> {
        let (given, needed) = (data.len(), dimensions.iter().fold(1, |p, &d| p * d));
        if needed > given {
            raise!("there should be at least {} data point(s)", needed);
        }
        let dimensions = dimensions.iter().map(|&d| d as c_longlong).collect::<Vec<_>>();
        let raw = nonnull!(ffi!(TF_NewTensor(T::kind().into(), dimensions.as_ptr() as *mut _,
                                dimensions.len() as c_int, data.as_mut_ptr() as *mut _,
                                needed as size_t, Some(noop), ptr::null_mut())));
        Ok(Tensor { dimensions: dimensions, memory: Memory::new(data), raw: raw })
    }

    /// Return the dimensions.
    pub fn dimensions(&self) -> Vec<usize> {
        self.dimensions.iter().map(|&d| d as usize).collect()
    }
}

memory!(Tensor<T>);

impl<T> Drop for Tensor<T> {
    #[inline]
    fn drop(&mut self) {
        ffi!(TF_DeleteTensor(self.raw));
    }
}

pub fn copy_raw<T>(tensor: &Tensor<T>) -> Result<*mut ffi::TF_Tensor> where T: Value {
    Ok(nonnull!(ffi!(TF_NewTensor(T::kind().into(), tensor.dimensions.as_ptr() as *mut _,
                     tensor.dimensions.len() as c_int, tensor.as_ptr() as *mut _,
                     tensor.len() as size_t, Some(noop), ptr::null_mut()))))
}

pub fn from_raw<T>(raw: *mut ffi::TF_Tensor) -> Result<Tensor<T>> where T: Value {
    if Type::from(ffi!(TF_TensorType(raw))) != T::kind() {
        raise!("the data types do not match");
    }
    let pointer = nonnull!(ffi!(TF_TensorData(raw))) as *mut _;
    let dimensions = (0..ffi!(TF_NumDims(raw))).map(|i| ffi!(TF_Dim(raw, i))).collect::<Vec<_>>();
    let length = if dimensions.is_empty() { 0 } else {
        dimensions.iter().fold(1, |p, &d| p * d as usize)
    };
    Ok(Tensor { dimensions: dimensions, memory: Memory::from_raw(pointer, length), raw: raw })
}

unsafe extern "C" fn noop(_: *mut c_void, _: size_t, _: *mut c_void) {}
