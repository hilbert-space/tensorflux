//! Interface to [TensorFlow][1].
//!
//! [1]: https://www.tensorflow.org
//!
//! ## Example
//!
//! Create a graph in Python:
//!
//! ```python
//! import tensorflow as tf
//!
//! with tf.Session() as session:
//!     a = tf.Variable(0.0, name='a')
//!     b = tf.Variable(0.0, name='b')
//!     c = tf.mul(a, b, name='c')
//!     tf.train.write_graph(session.graph_def, '', 'graph.pb', as_text=False)
//! ```
//!
//! Evaluate the graph in Rust:
//!
//! ```
//! # const GRAPH_PATH: &'static str = "examples/fixtures/graph.pb";
//! use tensorflux::{Definition, Input, Options, Output, Session, Tensor};
//!
//! let mut session = Session::new(Options::new().unwrap()).unwrap();
//! session.extend(&Definition::load(GRAPH_PATH).unwrap()).unwrap(); // c = a * b
//!
//! let mut inputs = vec![Input::new("a"), Input::new("b")];
//! inputs[0].set(Tensor::new(vec![1f32, 2.0, 3.0], &[3]).unwrap());
//! inputs[1].set(Tensor::new(vec![4f32, 5.0, 6.0], &[3]).unwrap());
//!
//! let mut outputs = vec![Output::new("c")];
//!
//! session.run(&mut inputs, &mut outputs, &vec![]).unwrap();
//!
//! let result = outputs[0].get::<f32>().unwrap();
//! assert_eq!(&result[..], &[1.0 * 4.0, 2.0 * 5.0, 3.0 * 6.0]);
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
