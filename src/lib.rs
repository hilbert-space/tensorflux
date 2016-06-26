//! Interface to [TensorFlow][1].
//!
//! [1]: https://www.tensorflow.org

extern crate libc;
extern crate tensorflow_sys as ffi;

#[macro_use]
mod macros;

mod definition;
mod options;
mod result;
mod session;
mod status;

#[path = "type.rs"]
mod typo;

pub use definition::Definition;
pub use options::Options;
pub use result::{Error, Result};
pub use session::Session;
pub use typo::Type;
