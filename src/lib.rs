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
//! macro_rules! ok(($result:expr) => ($result.unwrap()));
//!
//! let mut session = ok!(Session::new(ok!(Options::new())));
//! ok!(session.extend(&ok!(Definition::load(GRAPH_PATH)))); // c = a * b
//!
//! let a = ok!(Tensor::new(vec![1f32, 2.0, 3.0], &[3]));
//! let b = ok!(Tensor::new(vec![4f32, 5.0, 6.0], &[3]));
//!
//! let inputs = vec![Input::new("a:0", a), Input::new("b:0", b)];
//! let mut outputs = vec![Output::new("c:0")];
//!
//! ok!(session.run(inputs, &mut outputs, vec![]));
//!
//! let c: Tensor<f32> = outputs.pop().unwrap().into().unwrap();
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
