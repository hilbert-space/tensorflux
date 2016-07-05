use ffi::TF_Status;

use Result;

pub struct Status {
    raw: *mut TF_Status,
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

#[inline]
pub fn as_raw(status: &Status) -> *mut TF_Status {
    status.raw
}
