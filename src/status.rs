use ffi;

use Result;

pub struct Status {
    raw: *mut ffi::TF_Status,
}

impl Status {
    pub fn new() -> Result<Self> {
        Ok(Status { raw: nonnull!(ffi!(TF_NewStatus())) })
    }
}

impl Drop for Status {
    #[inline]
    fn drop(&mut self) {
        ffi!(TF_DeleteStatus(self.raw));
    }
}

#[inline(always)]
pub fn as_raw(status: &Status) -> *mut ffi::TF_Status {
    status.raw
}
