use ffi;
use libc::{c_int, c_longlong, c_void, size_t};
use std::{mem, ptr};

use Result;
use kind::{Type, Value};
use memory::Memory;

/// A tensor.
pub struct Tensor<T> {
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
        let mut dimensions = dimensions.iter().map(|&d| d as c_longlong).collect::<Vec<_>>();
        let raw = nonnull!(ffi!(TF_NewTensor(T::kind().into(), dimensions.as_mut_ptr(),
                                dimensions.len() as c_int, data.as_mut_ptr() as *mut _,
                                needed as size_t, Some(noop), ptr::null_mut())));
        Ok(Tensor { memory: Memory::new(data), raw: raw })
    }

    /// Return the dimensions.
    pub fn dimensions(&self) -> Vec<usize> {
        (0..ffi!(TF_NumDims(self.raw))).map(|i| ffi!(TF_Dim(self.raw, i)) as usize).collect()
    }
}

deref!(Tensor::memory<T>);

impl<T> Drop for Tensor<T> {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            ffi!(TF_DeleteTensor(self.raw));
        }
    }
}

pub fn from_raw<T>(raw: *mut ffi::TF_Tensor) -> Result<Tensor<T>> where T: Value {
    if Type::from(ffi!(TF_TensorType(raw))) != T::kind() {
        raise!("the data types do not match");
    }
    let pointer = nonnull!(ffi!(TF_TensorData(raw))) as *mut _;
    let length = (0..ffi!(TF_NumDims(raw))).fold(1, |p, i| p * ffi!(TF_Dim(raw, i)) as usize);
    Ok(Tensor { memory: Memory::from_raw(pointer, length), raw: raw })
}

#[inline(always)]
pub fn into_raw<T>(tensor: &mut Tensor<T>) -> *mut ffi::TF_Tensor {
    mem::replace(&mut tensor.raw, ptr::null_mut())
}

unsafe extern "C" fn noop(_: *mut c_void, _: size_t, _: *mut c_void) {}
