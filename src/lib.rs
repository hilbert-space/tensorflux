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
//! a = tf.Variable(0.0, name='a')
//! b = tf.Variable(0.0, name='b')
//! c = tf.mul(a, b, name='c')
//!
//! tf.train.write_graph(tf.Session().graph_def, '', 'graph.pb', as_text=False)
//! ```
//!
//! Evaluate the graph in Rust:
//!
//! ```
//! use tensorflux::{Buffer, Input, Options, Output, Session, Tensor};
//!
//! let graph = "graph.pb"; // c = a * b
//! # let graph = "examples/fixtures/multiplication.pb";
//! let mut session = Session::new(&Options::new().unwrap()).unwrap();
//! session.extend(&Buffer::load(graph).unwrap()).unwrap();
//!
//! let mut inputs = vec![Input::new("a"), Input::new("b")];
//! inputs[0].set(Tensor::new(vec![1f32, 2.0, 3.0], &[3]).unwrap());
//! inputs[1].set(Tensor::new(vec![4f32, 5.0, 6.0], &[3]).unwrap());
//!
//! let mut outputs = vec![Output::new("c")];
//!
//! session.run(&mut inputs, &mut outputs, &[], None, None).unwrap();
//!
//! let result = outputs[0].get::<f32>().unwrap();
//! assert_eq!(&result[..], &[1.0 * 4.0, 2.0 * 5.0, 3.0 * 6.0]);
//! ```

extern crate libc;
extern crate tensorflow_sys as ffi;

#[cfg(feature = "complex")]
extern crate num_complex as num;

#[macro_use]
mod macros;

mod buffer;
mod error;
mod kind;
mod memory;
mod options;
mod session;
mod status;
mod tensor;

pub use buffer::Buffer;
pub use error::Error;
pub use kind::{Type, Value};
pub use options::Options;
pub use session::{Input, Output, Session, Target};
pub use tensor::Tensor;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

/// A complex number with 32-bit parts.
#[cfg(feature = "complex")]
#[allow(non_camel_case_types)]
pub type c32 = num::Complex<f32>;

/// A complex number with 64-bit parts.
#[cfg(feature = "complex")]
#[allow(non_camel_case_types)]
pub type c64 = num::Complex<f64>;
