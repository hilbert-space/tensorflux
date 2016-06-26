use ffi;
use libc::{c_int, c_longlong, c_void, size_t};
use std::ptr;

use Result;
use kind::Value;

/// A tensor.
pub struct Tensor<T> {
    #[allow(dead_code)]
    data: Vec<T>,
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
        Ok(Tensor { data: data, raw: raw })
    }
}

impl<T> Drop for Tensor<T> {
    #[inline]
    fn drop(&mut self) {
        ffi!(TF_DeleteTensor(self.raw));
    }
}

unsafe extern "C" fn noop(_: *mut c_void, _: size_t, _: *mut c_void) {}
