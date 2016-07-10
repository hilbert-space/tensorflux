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
//! a = tf.placeholder(tf.float32, name='a')
//! b = tf.placeholder(tf.float32, name='b')
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
//! macro_rules! ok(($result:expr) => ($result.unwrap()));
//!
//! let graph = "graph.pb"; // c = a * b
//! # let graph = "examples/assets/multiplication.pb";
//! let mut session = ok!(Session::new(&ok!(Options::new())));
//! ok!(session.extend(&ok!(Buffer::load(graph))));
//!
//! let a = ok!(Tensor::new(vec![1f32, 2.0, 3.0], &[3]));
//! let b = ok!(Tensor::new(vec![4f32, 5.0, 6.0], &[3]));
//!
//! let inputs = vec![Input::new("a", a), Input::new("b", b)];
//! let mut outputs = vec![Output::new("c")];
//! ok!(session.run(&inputs, &mut outputs, &[], None, None));
//!
//! let c = ok!(outputs[0].get::<f32>());
//! assert_eq!(&c[..], &[1.0 * 4.0, 2.0 * 5.0, 3.0 * 6.0]);
//! ```

extern crate libc;
extern crate tensorflux_sys as ffi;

#[cfg(feature = "complex")]
extern crate num_complex as num;

#[macro_use]
mod macros;

mod buffer;
mod error;
mod memory;
mod options;
mod session;
mod status;
mod library;
mod tensor;
mod value;

pub use buffer::Buffer;
pub use error::Error;
pub use library::Library;
pub use options::Options;
pub use session::{Input, Output, Session, Target};
pub use tensor::Tensor;
pub use value::Value;

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
