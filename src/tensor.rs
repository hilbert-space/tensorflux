use ffi;
use libc::{c_int, c_longlong, c_void, size_t};
use std::ops::{Deref, DerefMut};
use std::{mem, ptr};

use Result;
use kind::Value;

/// A tensor.
pub struct Tensor<T> {
    #[allow(dead_code)]
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
    #[inline]
    fn drop(&mut self) {
        if !self.drop {
            mem::forget(mem::replace(&mut self.data, vec![]));
        }
        if let Some(raw) = self.raw.take() {
            ffi!(TF_DeleteTensor(raw));
        }
    }
}

#[inline(always)]
pub fn unwrap<T>(tensor: &mut Tensor<T>) -> *mut ffi::TF_Tensor {
    tensor.raw.take().unwrap_or_else(|| ptr::null_mut())
}

unsafe extern "C" fn noop(_: *mut c_void, _: size_t, _: *mut c_void) {}
