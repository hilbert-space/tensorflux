//! Interface to [TensorFlow][1].
//!
//! [1]: https://www.tensorflow.org

extern crate libc;
extern crate tensorflow_sys as ffi;

#[macro_use]
mod macros;

mod graph;
mod options;
mod result;
mod session;
mod status;

#[path = "type.rs"]
mod typo;

pub use graph::Graph;
pub use options::Options;
pub use result::{Error, Result};
pub use session::Session;
pub use typo::Type;
