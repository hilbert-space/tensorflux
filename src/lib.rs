//! Interface to [TensorFlow][1].
//!
//! [1]: https://www.tensorflow.org

extern crate tensorflow_sys as ffi;

#[macro_use]
mod result;

mod options;
mod session;
mod status;

pub use options::Options;
pub use result::{Error, Result};
pub use session::Session;
