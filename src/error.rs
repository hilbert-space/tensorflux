use ffi::TF_Code;
use std::ffi::CStr;
use std::{error, fmt};

use status::{self, Status};

/// An error.
#[derive(Clone, Debug)]
pub struct Error {
    code: TF_Code,
    message: String,
}

impl Error {
    /// Return the current error if any.
    pub fn current(status: &Status) -> Option<Self> {
        let code = ffi!(TF_GetCode(status::as_raw(status)));
        if code == TF_Code::TF_OK {
            return None;
        }
        let message = ffi!(TF_Message(status::as_raw(status)));
        if message.is_null() {
            return None;
        }
        let message = match unsafe { CStr::from_ptr(message).to_str() } {
            Ok(message) => message.into(),
            _ => String::new(),
        };
        Some(Error { code: code, message: message })
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.message.fmt(formatter)
    }
}

impl error::Error for Error {
    #[inline]
    fn description(&self) -> &str {
        &self.message
    }
}

impl<T> From<T> for Error where T: Into<String> {
    #[inline]
    fn from(message: T) -> Error {
        Error { code: TF_Code::TF_UNKNOWN, message: message.into() }
    }
}
