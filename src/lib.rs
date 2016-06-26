//! Interface to [TensorFlow][1].
//!
//! [1]: https://www.tensorflow.org

extern crate libc;
extern crate tensorflow_sys as ffi;

#[macro_use]
mod macros;

mod definition;
mod error;
mod kind;
mod options;
mod session;
mod status;
mod tensor;

pub use definition::Definition;
pub use error::Error;
pub use kind::{Type, Value};
pub use options::Options;
pub use session::{Input, Output, Session, Target};
pub use tensor::Tensor;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;
