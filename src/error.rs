use std::ffi::CStr;
use std::{error, fmt};

use status::{self, Status};

translate! {
    pub struct Code => TF_Code,

    OK => TF_OK,
    Aborted => TF_ABORTED,
    AlreadyExists => TF_ALREADY_EXISTS,
    Cancelled => TF_CANCELLED,
    DataLoss => TF_DATA_LOSS,
    DeadlineExceeded => TF_DEADLINE_EXCEEDED,
    FailedPrecondition => TF_FAILED_PRECONDITION,
    Internal => TF_INTERNAL,
    InvalidArgument => TF_INVALID_ARGUMENT,
    NotFound => TF_NOT_FOUND,
    OutOfRange => TF_OUT_OF_RANGE,
    PermissionDenied => TF_PERMISSION_DENIED,
    ResourceExhausted => TF_RESOURCE_EXHAUSTED,
    Unauthenticated => TF_UNAUTHENTICATED,
    Unavailable => TF_UNAVAILABLE,
    Unimplemented => TF_UNIMPLEMENTED,
    Unknown => TF_UNKNOWN,
}

/// An error.
#[derive(Clone, Debug)]
pub struct Error {
    code: Code,
    message: String,
}

impl Error {
    /// Return the current error if any.
    pub fn current(status: &Status) -> Option<Self> {
        let code = Code::from(ffi!(TF_GetCode(status::raw(status))));
        if code == Code::OK {
            return None;
        }
        let message = ffi!(TF_Message(status::raw(status)));
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
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", &self.message)
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
        Error { code: Code::Unknown, message: message.into() }
    }
}
