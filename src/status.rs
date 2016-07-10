use ffi::TF_Status;

use Result;

pub struct Status {
    raw: *mut TF_Status,
}

impl Status {
    pub fn new() -> Result<Self> {
        Ok(Status { raw: nonnull!(ffi!(TF_NewStatus())) })
    }

    #[inline]
    pub fn as_raw(&self) -> *mut TF_Status {
        self.raw
    }
}

impl Drop for Status {
    #[inline]
    fn drop(&mut self) {
        ffi!(TF_DeleteStatus(self.raw));
    }
}
