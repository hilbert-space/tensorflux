use ffi;

use result::Result;

pub struct Status {
    raw: *mut ffi::TF_Status,
}

impl Status {
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
