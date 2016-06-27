use ffi;
use libc::{c_int, c_longlong, c_void, size_t};
use std::ops::{Deref, DerefMut};
use std::{mem, ptr};

use Result;
use kind::{Type, Value};

/// A tensor.
pub struct Tensor<T> {
    data: Vec<T>,
    drop: bool,
    raw: Option<*mut ffi::TF_Tensor>,
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
        Ok(Tensor { data: data, drop: true, raw: Some(raw) })
    }
}

impl<T> Deref for Tensor<T> {
    type Target = [T];

    #[inline(always)]
    fn deref(&self) -> &[T] {
        &self.data
    }
}

impl<T> DerefMut for Tensor<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

impl<T> Drop for Tensor<T> {
    fn drop(&mut self) {
        if !self.drop {
            mem::forget(mem::replace(&mut self.data, vec![]));
        }
        if let Some(raw) = self.raw.take() {
            ffi!(TF_DeleteTensor(raw));
        }
    }
}

impl<T> Into<Vec<T>> for Tensor<T> where T: Clone {
    fn into(mut self) -> Vec<T> {
        if self.drop {
            mem::replace(&mut self.data, vec![])
        } else {
            self.data.clone()
        }
    }
}

pub fn from_raw<T>(raw: *mut ffi::TF_Tensor) -> Result<Tensor<T>> where T: Value {
    if Type::from(ffi!(TF_TensorType(raw))) != T::kind() {
        raise!("the data types do not match");
    }

    let count = ffi!(TF_NumDims(raw)) as usize;
    let mut size = 1;
    let mut dimensions = vec![0; count];
    for i in 0..count {
        dimensions[i] = ffi!(TF_Dim(raw, i as c_int)) as usize;
        size *= dimensions[i];
    }

    let pointer = nonnull!(ffi!(TF_TensorData(raw)));
    let data = unsafe { Vec::from_raw_parts(pointer as *mut _, size, size) };

    Ok(Tensor { data: data, drop: false, raw: Some(raw) })
}

#[inline(always)]
pub fn into_raw<T>(tensor: &mut Tensor<T>) -> *mut ffi::TF_Tensor {
    tensor.raw.take().unwrap()
}

unsafe extern "C" fn noop(_: *mut c_void, _: size_t, _: *mut c_void) {}
