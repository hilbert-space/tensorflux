use ffi::TF_Library;

use Result;
use status::Status;

/// A library.
pub struct Library {
    #[allow(dead_code)]
    raw: *mut TF_Library,
}

impl Library {
    /// Load a library.
    pub fn load<T>(name: T) -> Result<Self> where T: Into<String> {
        let name = into_cstring!(name);
        let status = try!(Status::new());
        let raw = nonnull!(ffi!(TF_LoadLibrary(name.as_ptr(), status.as_raw())), &status);
        Ok(Library { raw: raw })
    }
}

impl Drop for Library {
    #[inline]
    fn drop(&mut self) {
        // Not available in the API.
    }
}
