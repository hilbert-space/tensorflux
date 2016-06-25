//! Interface to [TensorFlow][1].
//!
//! [1]: https://www.tensorflow.org

extern crate tensorflow_sys as ffi;

#[macro_use]
mod macros;

#[path = "type.rs"]
mod typo;

mod options;
mod result;
mod session;
mod status;

pub use typo::Type;
pub use options::Options;
pub use result::{Error, Result};
pub use session::Session;
