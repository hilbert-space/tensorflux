# TensorFlux [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an interface to [TensorFlow][tensorflow].

## [Documentation][documentation]

## [Example][example]

Create a graph in Python:

```python
import tensorflow as tf

with tf.Session() as session:
    a = tf.Variable(0.0, name='a')
    b = tf.Variable(0.0, name='b')
    c = tf.mul(a, b, name='c')
    tf.train.write_graph(session.graph_def, '', 'graph.pb', as_text=False)
```

Evaluate the graph in Rust:

```rust
use tensorflux::{Buffer, Input, Options, Output, Session, Tensor};

let mut session = Session::new(&Options::new().unwrap()).unwrap();
session.extend(&Buffer::load(GRAPH_PATH).unwrap()).unwrap(); // c = a * b

let mut inputs = vec![Input::new("a"), Input::new("b")];
inputs[0].set(Tensor::new(vec![1f32, 2.0, 3.0], &[3]).unwrap());
inputs[1].set(Tensor::new(vec![4f32, 5.0, 6.0], &[3]).unwrap());

let mut outputs = vec![Output::new("c")];

session.run(&mut inputs, &mut outputs).unwrap();

let result = outputs[0].get::<f32>().unwrap();
assert_eq!(&result[..], &[1.0 * 4.0, 2.0 * 5.0, 3.0 * 6.0]);
```

## [Requirements][requirements]

## [Configuration][configuration]

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[configuration]: https://github.com/stainless-steel/tensorflow-sys#configuration
[documentation]: https://stainless-steel.github.io/tensorflux
[example]: examples/workflow.rs
[requirements]: https://github.com/stainless-steel/tensorflow-sys#requirements
[tensorflow]: https://www.tensorflow.org

[status-img]: https://travis-ci.org/stainless-steel/tensorflux.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/tensorflux
[version-img]: https://img.shields.io/crates/v/tensorflux.svg
[version-url]: https://crates.io/crates/tensorflux
