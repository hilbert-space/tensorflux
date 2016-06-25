use ffi;

use result::Result;

/// A status.
pub struct Status {
    raw: *mut ffi::TF_Status,
}

impl Status {
    /// Create a status.
    pub fn new() -> Result<Status> {
        Ok(Status { raw: nonnull!(unsafe { ffi::TF_NewStatus() }) })
    }
}

impl Drop for Status {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::TF_DeleteStatus(self.raw) };
    }
}

#[inline(always)]
pub fn raw(status: &Status) -> *mut ffi::TF_Status {
    status.raw
}
