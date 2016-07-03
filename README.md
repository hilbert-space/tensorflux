# TensorFlux [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides an interface to [TensorFlow][tensorflow].

## [Documentation][documentation]

## [Example][example]

Create a graph in Python:

```python
import tensorflow as tf

a = tf.placeholder(tf.float32, name='a')
b = tf.placeholder(tf.float32, name='b')
c = tf.mul(a, b, name='c')

tf.train.write_graph(tf.Session().graph_def, '', 'graph.pb', as_text=False)
```

Evaluate the graph in Rust:

```rust
use tensorflux::{Buffer, Input, Options, Output, Session, Tensor};

macro_rules! ok(($result:expr) => ($result.unwrap()));

let graph = "graph.pb"; // c = a * b
let mut session = ok!(Session::new(&ok!(Options::new())));
ok!(session.extend(&ok!(Buffer::load(graph))));

let a = ok!(Tensor::new(vec![1f32, 2.0, 3.0], &[3]));
let b = ok!(Tensor::new(vec![4f32, 5.0, 6.0], &[3]));

let inputs = vec![Input::new("a", a), Input::new("b", b)];
let mut outputs = vec![Output::new("c")];
ok!(session.run(&inputs, &mut outputs, &[], None, None));

let c = ok!(outputs[0].get::<f32>());
assert_eq!(&c[..], &[1.0 * 4.0, 2.0 * 5.0, 3.0 * 6.0]);
```

## [Requirements][requirements]

## [Configuration][configuration]

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[configuration]: https://github.com/stainless-steel/tensorflow-sys#configuration
[documentation]: https://stainless-steel.github.io/tensorflux
[example]: examples/multiplication.rs
[requirements]: https://github.com/stainless-steel/tensorflow-sys#requirements
[tensorflow]: https://www.tensorflow.org

[status-img]: https://travis-ci.org/stainless-steel/tensorflux.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/tensorflux
[version-img]: https://img.shields.io/crates/v/tensorflux.svg
[version-url]: https://crates.io/crates/tensorflux
