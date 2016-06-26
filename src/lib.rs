//! Interface to [TensorFlow][1].
//!
//! [1]: https://www.tensorflow.org

extern crate libc;
extern crate tensorflow_sys as ffi;

#[macro_use]
mod macros;

mod definition;
mod error;
mod options;
mod session;
mod status;

#[path = "type.rs"]
mod typo;

pub use definition::Definition;
pub use error::Error;
pub use options::Options;
pub use session::Session;
pub use typo::Type;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;
