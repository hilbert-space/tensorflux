//! Interface to [TensorFlow][1].
//!
//! [1]: https://www.tensorflow.org
//!
//! ## Example
//!
//! ```
//! # const GRAPH_PATH: &'static str = "examples/fixtures/graph.pb";
//! use tensorflux::{Definition, Input, Options, Output, Session, Tensor};
//!
//! let mut session = Session::new(Options::new().unwrap()).unwrap();
//! session.extend(&Definition::load(GRAPH_PATH).unwrap()).unwrap(); // c = a * b
//!
//! let a = Tensor::new(vec![1f32, 2.0, 3.0], vec![3]).unwrap();
//! let b = Tensor::new(vec![4f32, 5.0, 6.0], vec![3]).unwrap();
//!
//! let inputs = vec![Input::new("a", a), Input::new("b", b)];
//! let outputs = vec![Output::new("c")];
//!
//! let mut results = session.run(inputs, outputs, vec![]).unwrap();
//!
//! let c: Tensor<f32> = results.pop().unwrap().into().unwrap();
//!
//! assert_eq!(&c[..], &[1.0 * 4.0, 2.0 * 5.0, 3.0 * 6.0]);
//! ```

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
