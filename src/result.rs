use ffi;
use std::ffi::CStr;
use std::{error, fmt};

use status::{self, Status};

macro_rules! nonnull(
    ($pointer:expr, $status:expr) => ({
        let pointer = $pointer;
        if pointer.is_null() {
            if let Some(error) = ::result::Error::current($status) {
                return Err(error);
            }
            raise!("failed to call TensorFlow");
        }
        pointer
    });
    ($pointer:expr) => ({
        let pointer = $pointer;
        if pointer.is_null() {
            raise!("failed to call TensorFlow");
        }
        pointer
    });
);

macro_rules! ok(
    ($result:expr) => (match $result {
        Ok(result) => result,
        Err(error) => return Err(error),
    });
);

macro_rules! raise(
    ($message:expr) => (return Err(::result::Error::from($message)));
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Code {
    OK,
    Cancelled,
    Unknown,
    InvalidArgument,
    DeadlineExceeded,
    NotFound,
    AlreadyExists,
    PermissionDenied,
    Unauthenticated,
    ResourceExhausted,
    FailedPrecondition,
    Aborted,
    OutOfRange,
    Unimplemented,
    Internal,
    Unavailable,
    DataLoss,
}

/// An error.
#[derive(Clone, Debug)]
pub struct Error {
    code: Code,
    message: String,
}

/// A result.
pub type Result<T> = ::std::result::Result<T, Error>;

impl Error {
    /// Return the current error if any.
    pub fn current(status: &Status) -> Option<Error> {
        let code = Code::from(unsafe { ffi::TF_GetCode(status::raw(status)) });
        if code == Code::OK {
            return None;
        }
        let message = unsafe { ffi::TF_Message(status::raw(status)) };
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

impl Code {
    fn from(code: ffi::TF_Code) -> Code {
        match code {
            ffi::TF_OK => Code::OK,
            ffi::TF_CANCELLED => Code::Cancelled,
            ffi::TF_UNKNOWN => Code::Unknown,
            ffi::TF_INVALID_ARGUMENT => Code::InvalidArgument,
            ffi::TF_DEADLINE_EXCEEDED => Code::DeadlineExceeded,
            ffi::TF_NOT_FOUND => Code::NotFound,
            ffi::TF_ALREADY_EXISTS => Code::AlreadyExists,
            ffi::TF_PERMISSION_DENIED => Code::PermissionDenied,
            ffi::TF_UNAUTHENTICATED => Code::Unauthenticated,
            ffi::TF_RESOURCE_EXHAUSTED => Code::ResourceExhausted,
            ffi::TF_FAILED_PRECONDITION => Code::FailedPrecondition,
            ffi::TF_ABORTED => Code::Aborted,
            ffi::TF_OUT_OF_RANGE => Code::OutOfRange,
            ffi::TF_UNIMPLEMENTED => Code::Unimplemented,
            ffi::TF_INTERNAL => Code::Internal,
            ffi::TF_UNAVAILABLE => Code::Unavailable,
            ffi::TF_DATA_LOSS => Code::DataLoss,
        }
    }
}
